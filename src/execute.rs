use std::collections::HashMap;

use cairo_air::{CairoProof, PreProcessedTraceVariant};
use cairo_lang_casm::hints::Hint;
use cairo_lang_executable::executable::{EntryPointKind, Executable};
use cairo_lang_runner::{build_hints_dict, Arg, CairoHintProcessor};
use cairo_vm::cairo_run::{cairo_run_program, CairoRunConfig};
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::types::program::Program;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::Felt252;
use log::{debug, info};
use stwo_cairo_adapter::builtins::MemorySegmentAddresses;
use stwo_cairo_adapter::memory::{MemoryBuilder, MemoryConfig, MemoryEntry};
use stwo_cairo_adapter::vm_import::{adapt_to_stwo_input, RelocatedTraceEntry};
use stwo_cairo_adapter::PublicSegmentContext;
use stwo_cairo_prover::stwo_prover::core::pcs::PcsConfig;
use stwo_cairo_prover::stwo_prover::core::vcs::blake2_merkle::{
    Blake2sMerkleChannel, Blake2sMerkleHasher,
};

pub fn execute_and_prove(target_path: &str, args: Vec<Arg>) -> CairoProof<Blake2sMerkleHasher> {
    let executable: Executable =
        serde_json::from_reader(std::fs::File::open(target_path).unwrap()).unwrap();

    let (program, string_to_hint) = program_and_hints_from_executable(&executable);

    let mut hint_processor = CairoHintProcessor {
        runner: None,
        user_args: vec![vec![Arg::Array(args)]],
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
        relocate_mem: true,
        layout: LayoutName::all_cairo_stwo,
        secure_run: None,
        allow_missing_builtins: None,
        dynamic_layout_params: None,
        disable_trace_padding: true,
        proof_mode: true,
        ..Default::default()
    };

    info!("Executing program...");
    let runner = cairo_run_program(&program, &cairo_run_config, &mut hint_processor).unwrap();
    info!("Program executed successfully.");
    let public_input = runner.get_air_public_input().unwrap();
    let addresses = public_input
        .public_memory
        .iter()
        .map(|entry| entry.address as u32)
        .collect::<Vec<_>>();
    let segments = public_input
        .memory_segments
        .iter()
        .map(|(&k, v)| {
            (
                k,
                MemorySegmentAddresses {
                    begin_addr: v.begin_addr,
                    stop_ptr: v.stop_ptr,
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let trace = runner
        .relocated_trace
        .as_ref()
        .unwrap()
        .iter()
        .map(|x| RelocatedTraceEntry {
            ap: x.ap,
            fp: x.fp,
            pc: x.pc,
        })
        .collect::<Vec<_>>();
    let mem = runner
        .relocated_memory
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            x.as_ref().map(|value| MemoryEntry {
                address: i as u64,
                value: unsafe { std::mem::transmute::<[u8; 32], [u32; 8]>(value.to_bytes_le()) },
            })
        });
    let mem = MemoryBuilder::from_iter(MemoryConfig::default(), mem);
    let main_args = runner
        .get_program()
        .iter_builtins()
        .copied()
        .collect::<Vec<_>>();
    let public_segment_context = PublicSegmentContext::new(&main_args);

    let preprocessed_trace = match public_segment_context[1] {
        true => PreProcessedTraceVariant::Canonical,
        false => PreProcessedTraceVariant::CanonicalWithoutPedersen,
    };

    info!("Generating input for the prover...");
    let input =
        adapt_to_stwo_input(&trace, mem, addresses, &segments, public_segment_context).unwrap();
    info!("Input for the prover generated successfully.");
    debug!(
        "State transitions: {}",
        input.state_transitions.casm_states_by_opcode
    );
    debug!("Builtins: {:#?}", input.builtins_segments.get_counts());

    info!("Using preprocessed trace: {:?}", preprocessed_trace);
    info!("Proving...");
    let pcs_config = PcsConfig::default();
    stwo_cairo_prover::prover::prove_cairo::<Blake2sMerkleChannel>(
        input,
        pcs_config,
        preprocessed_trace,
    )
    .unwrap()
}

fn program_and_hints_from_executable(executable: &Executable) -> (Program, HashMap<String, Hint>) {
    let data: Vec<MaybeRelocatable> = executable
        .program
        .bytecode
        .iter()
        .map(Felt252::from)
        .map(MaybeRelocatable::from)
        .collect();
    let (hints, string_to_hint) = build_hints_dict(&executable.program.hints);
    let entrypoint = executable
        .entrypoints
        .iter()
        .find(|e| matches!(e.kind, EntryPointKind::Standalone))
        .unwrap();
    let program = Program::new_for_proof(
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
    .unwrap();
    (program, string_to_hint)
}

#[cfg(test)]
mod tests {
    use cairo_air::verifier::verify_cairo;
    use num_bigint::BigInt;

    use super::*;

    #[test]
    fn test_e2e() {
        let target_path = "/home/ohad/ursus/playground/target/release/playground.executable.json";
        let args = vec![Arg::Value(Felt252::from(BigInt::from(100)))];
        let proof = execute_and_prove(target_path, args);
        let pcs_config = PcsConfig::default();
        let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
        let result = verify_cairo::<Blake2sMerkleChannel>(proof, pcs_config, preprocessed_trace);
        assert!(result.is_ok());
    }
}
