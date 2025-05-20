use std::time::Instant;

use cairo_air::verifier::verify_cairo;
use cairo_air::PreProcessedTraceVariant;
use clap::Parser;
use log::{info, warn};
use stwo_cairo_prover::stwo_prover::core::pcs::PcsConfig;
use stwo_cairo_prover::stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;
use ursus::args::{Cli, Commands, ProgramArguments};
use ursus::execute::execute_and_prove;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Prove {
            target,
            proof,
            arguments,
            arguments_file,
        } => {
            info!("Generating proof for target: {:?}", target);
            let start = Instant::now();
            let args = ProgramArguments {
                arguments: arguments.unwrap_or_default(),
                arguments_file,
            };
            let cairo_proof = execute_and_prove(target.to_str().unwrap(), args.read_arguments());
            let elapsed = start.elapsed();

            // Serialize proof to file.
            let proof_json = serde_json::to_string(&cairo_proof).unwrap();
            std::fs::write(proof.to_str().unwrap(), proof_json).unwrap();
            info!("Proof saved to: {:?}", proof);
            info!("Proof generation completed in {:.2?}", elapsed);
        }
        Commands::Verify {
            proof,
            with_pedersen,
        } => {
            info!("Verifying proof from: {:?}", proof);
            let cairo_proof =
                serde_json::from_reader(std::fs::File::open(proof.to_str().unwrap()).unwrap())
                    .unwrap();
            let pcs_config = PcsConfig::default();
            let preprocessed_trace = match with_pedersen {
                true => PreProcessedTraceVariant::Canonical,
                false => PreProcessedTraceVariant::CanonicalWithoutPedersen,
            };
            let result =
                verify_cairo::<Blake2sMerkleChannel>(cairo_proof, pcs_config, preprocessed_trace);
            match result {
                Ok(_) => info!("Verification successful"),
                Err(e) => warn!("Verification failed: {:?}", e),
            }
        }
    }
}
