use cairo_air::{CairoProof, PreProcessedTraceVariant};
use cairo_vm::cairo_run::{cairo_run, CairoRunConfig};
use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::types::layout_name::LayoutName;
use stwo_cairo_adapter::adapter::adapter;
use stwo_cairo_adapter::ProverInput;
use stwo_cairo_prover::stwo_prover::core::pcs::PcsConfig;
use stwo_cairo_prover::stwo_prover::core::vcs::blake2_merkle::{
    Blake2sMerkleChannel, Blake2sMerkleHasher,
};

pub fn prover_input_from_compiled_cairo_program(compiled_program: &[u8]) -> ProverInput {
    let cairo_run_config = CairoRunConfig {
        entrypoint: "main",
        trace_enabled: true,
        relocate_mem: false,
        layout: LayoutName::all_cairo_stwo,
        proof_mode: true,
        secure_run: None,
        allow_missing_builtins: None,
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
        let compiled_program_path = Path::new("src/artifacts/all_opcodes.json");
        // print pwd:
        println!(
            "Current working directory: {:?}",
            std::env::current_dir().unwrap()
        );
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
