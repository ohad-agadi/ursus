use std::path::Path;

use cairo_air::{CairoProof, PreProcessedTraceVariant};
// use cairo_lang_utils::bigint::BigUintAsHex;
use cairo_vm::cairo_run::{cairo_run, cairo_run_program, CairoRunConfig};
use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::types::layout_name::LayoutName;
// use cario_lang_runner::{Arg, CairoHintProcessor};
use stwo_cairo_adapter::adapter::adapter;
use stwo_cairo_adapter::ProverInput;
use stwo_cairo_prover::stwo_prover::core::pcs::PcsConfig;
use stwo_cairo_prover::stwo_prover::core::vcs::blake2_merkle::{
    Blake2sMerkleChannel, Blake2sMerkleHasher,
};

fn load_prebuilt_executable(path: &Path, filename: String) -> Executable {
    let file_path = path.join(&filename);
    ensure!(
        file_path.exists(),
        formatdoc! {r#"
            package has not been compiled, file does not exist: `{filename}`
            help: run `scarb build` to compile the package
        "#}
    );
    let file = fs::File::open(&file_path)
        .with_context(|| format!("failed to open executable program: `{file_path}`"))?;
    serde_json::from_reader(file)
        .with_context(|| format!("failed to deserialize executable program: `{file_path}`"))
}

pub fn execute(scarb_build_dir: &str, build_target: &str) {
    let executable = load_prebuilt_executable(
        &scarb_build_dir,
        format!("{}.executable.json", build_target.name),
    )?;

    let data = executable
        .program
        .bytecode
        .iter()
        .map(Felt252::from)
        .map(MaybeRelocatable::from)
        .collect();

    let (hints, string_to_hint) = build_hints_dict(&executable.program.hints);

    let program = if args.run.target.is_standalone() {
        let entrypoint = executable
            .entrypoints
            .iter()
            .find(|e| matches!(e.kind, EntryPointKind::Standalone))
            .with_context(|| "no `Standalone` entrypoint found")?;
        Program::new_for_proof(
            entrypoint.builtins.clone(),
            data,
            entrypoint.offset,
            entrypoint.offset + 4,
            hints,
            Default::default(),
            Default::default(),
            vec![],
            None,
        )
    } else {
        let entrypoint = executable
            .entrypoints
            .iter()
            .find(|e| matches!(e.kind, EntryPointKind::Bootloader))
            .with_context(|| "no `Bootloader` entrypoint found")?;
        Program::new(
            entrypoint.builtins.clone(),
            data,
            Some(entrypoint.offset),
            hints,
            Default::default(),
            Default::default(),
            vec![],
            None,
        )
    }
    .with_context(|| "failed setting up program")?;

    let mut hint_processor = CairoHintProcessor {
        runner: None,
        user_args: vec![vec![Arg::Array(
            args.run.arguments.clone().read_arguments()?,
        )]],
        string_to_hint,
        starknet_state: Default::default(),
        run_resources: Default::default(),
        syscalls_used_resources: Default::default(),
        no_temporary_segments: false,
        markers: Default::default(),
        panic_traceback: Default::default(),
    };

    let proof_mode = args.run.target.is_standalone();

    let cairo_run_config = CairoRunConfig {
        allow_missing_builtins: Some(true),
        layout: LayoutName::all_cairo,
        proof_mode,
        secure_run: None,
        relocate_mem: output.is_standard(),
        trace_enabled: output.is_standard(),
        disable_trace_padding: proof_mode,
        ..Default::default()
    };

    let mut runner =
        cairo_run_program(&program, &cairo_run_config, &mut hint_processor).map_err(|err| {
            if let Some(panic_data) = hint_processor.markers.last() {
                anyhow!(format_for_panic(panic_data.iter().copied()))
            } else {
                anyhow::Error::from(err).context("Cairo program run failed")
            }
        })?;
}

pub fn prover_input_from_compiled_cairo_program(compiled_program: &[u8]) -> ProverInput {
    let cairo_run_config = CairoRunConfig {
        entrypoint: "main",
        trace_enabled: true,
        relocate_mem: false,
        layout: LayoutName::all_cairo_stwo,
        proof_mode: true,
        secure_run: None,
        allow_missing_builtins: Some(true),
        dynamic_layout_params: None,
        disable_trace_padding: true,
    };

    let runner = cairo_run(
        compiled_program,
        &cairo_run_config,
        &mut BuiltinHintProcessor::new_empty(),
    )
    .expect("Failed to run cairo program");

    adapter(&mut runner.get_prover_input_info().unwrap()).unwrap()
}

pub fn prove_program(compiled_program: &[u8]) -> CairoProof<Blake2sMerkleHasher> {
    let prover_input = prover_input_from_compiled_cairo_program(compiled_program);
    let pcs_config = PcsConfig::default();
    let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
    stwo_cairo_prover::prover::prove_cairo::<Blake2sMerkleChannel>(
        prover_input,
        pcs_config,
        preprocessed_trace,
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use cairo_air::verifier::verify_cairo;

    use super::*;

    #[test]
    fn test_prover_input_from_compiled_cairo_program() {
        let compiled_program_path =
            Path::new("/home/ohad/ursus/playground/target/release/playground.executable.json");
        let compiled_program =
            std::fs::read(compiled_program_path).expect("Failed to read compiled program");
        let _ = prover_input_from_compiled_cairo_program(&compiled_program);
    }

    #[test]
    fn test_prove_program() {
        let compiled_program_path = Path::new("src/artifacts/all_opcodes.json");
        let compiled_program =
            std::fs::read(compiled_program_path).expect("Failed to read compiled program");
        let proof = prove_program(&compiled_program);

        let pcs_config = PcsConfig::default();
        let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
        let result = verify_cairo::<Blake2sMerkleChannel>(proof, pcs_config, preprocessed_trace);
        assert!(result.is_ok());
    }
}
