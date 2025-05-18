use std::path::Path;

use cairo_air::{CairoProof, PreProcessedTraceVariant};
use cairo_lang_executable::executable::Executable;
use cairo_lang_execute_utils::{program_and_hints_from_executable, user_args_from_flags};
use cairo_lang_runner::{Arg, CairoHintProcessor};
use cairo_vm::cairo_run::{cairo_run_program, CairoRunConfig};
use cairo_vm::types::layout_name::LayoutName;
use stwo_cairo_adapter::adapter::adapter;
use stwo_cairo_adapter::vm_import::adapt_vm_output;
use stwo_cairo_prover::stwo_prover::core::pcs::PcsConfig;
use stwo_cairo_prover::stwo_prover::core::vcs::blake2_merkle::{
    Blake2sMerkleChannel, Blake2sMerkleHasher,
};

pub fn run_program(target_path: &str) -> CairoProof<Blake2sMerkleHasher> {
    let executable: Executable =
        serde_json::from_reader(std::fs::File::open(target_path).unwrap()).unwrap();
    println!(
        "Bytecode size (felt252 count): {}",
        executable.program.bytecode.len()
    );

    let (program, string_to_hint) = program_and_hints_from_executable(&executable, true).unwrap();

    let user_args = vec![];

    let mut hint_processor = CairoHintProcessor {
        runner: None,
        user_args: vec![vec![Arg::Array(user_args)]],
        string_to_hint,
        starknet_state: Default::default(),
        run_resources: Default::default(),
        syscalls_used_resources: Default::default(),
        no_temporary_segments: false,
        markers: Default::default(),
        panic_traceback: Default::default(),
    };

    let cairo_run_config = CairoRunConfig {
        trace_enabled: true,
        relocate_mem: false,
        layout: LayoutName::all_cairo_stwo,
        secure_run: None,
        allow_missing_builtins: None,
        dynamic_layout_params: None,
        disable_trace_padding: true,
        proof_mode: true,
        ..Default::default()
    };

    let cairo_run_config2 = CairoRunConfig {
        trace_enabled: true,
        relocate_mem: true,
        layout: LayoutName::all_cairo_stwo,
        secure_run: None,
        allow_missing_builtins: None,
        dynamic_layout_params: None,
        disable_trace_padding: true,
        proof_mode: true,
        ..Default::default()
    };

    let runner = cairo_run_program(&program, &cairo_run_config, &mut hint_processor).unwrap();

    let runner2 = cairo_run_program(&program, &cairo_run_config2, &mut hint_processor).unwrap();
    let pub_input2 = runner2.get_air_public_input().unwrap();
    println!("pub_input2: {:?}", pub_input2.memory_segments);

    let mut prover_info = runner.get_prover_input_info().unwrap();
    println!("prover_info: {:?}", prover_info.builtins_segments);
    let input = adapter(&mut prover_info).unwrap();
    let pcs_config = PcsConfig::default();
    let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
    stwo_cairo_prover::prover::prove_cairo::<Blake2sMerkleChannel>(
        input,
        pcs_config,
        preprocessed_trace,
    )
    .unwrap()
}

pub fn prove_program(target_path: &str) -> CairoProof<Blake2sMerkleHasher> {
    let public_input_path = &Path::new(target_path).join("apui.json");
    let private_input_path = &Path::new(target_path).join("apriv.json");

    let prover_input = adapt_vm_output(public_input_path, private_input_path).unwrap();
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
    fn test_prove_program() {
        let target_path = "/home/ohad/raito/target/execute/light_client/execution1";
        let proof = prove_program(target_path);

        let pcs_config = PcsConfig::default();
        let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
        let result = verify_cairo::<Blake2sMerkleChannel>(proof, pcs_config, preprocessed_trace);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_program() {
        let target_path = "/home/ohad/ursus/playground/target/dev/playground.executable.json";
        let proof = run_program(target_path);
    }
}
