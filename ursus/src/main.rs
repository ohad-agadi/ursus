use std::path::PathBuf;

use cairo_air::{verifier::verify_cairo, PreProcessedTraceVariant};
use clap::{Parser, Subcommand};
use stwo_cairo_prover::stwo_prover::core::{pcs::PcsConfig, vcs::blake2_merkle::Blake2sMerkleChannel};
use ursus::execute::execute_and_prove;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a proof for a target file
    Prove {
        /// Path to the target file
        #[arg(short, long)]
        target: PathBuf,
        /// Path to the proof file
        #[arg(short, long)]
        proof: PathBuf,
    },
    /// Verify a proof
    Verify {
        /// Path to the proof JSON file
        #[arg(short, long)]
        proof: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Prove { target, proof } => {
            println!("Generating proof for target: {:?}", target);
            let cairo_proof = execute_and_prove(target.to_str().unwrap());
            // serialize proof to file
            let proof_json = serde_json::to_string(&cairo_proof).unwrap();
            std::fs::write(proof.to_str().unwrap(), proof_json).unwrap();
            println!("Proof saved to: {:?}", proof);
        }
        Commands::Verify { proof } => {
            println!("Verifying proof from: {:?}", proof);
            let cairo_proof =
                serde_json::from_reader(std::fs::File::open(proof.to_str().unwrap()).unwrap())
                    .unwrap();
            let pcs_config = PcsConfig::default();
            let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
            let result =
                verify_cairo::<Blake2sMerkleChannel>(cairo_proof, pcs_config, preprocessed_trace);
            println!("Verification result: {:?}", result);
        }
    }
}
