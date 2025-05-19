use std::path::PathBuf;
use std::time::Instant;

use cairo_air::verifier::verify_cairo;
use cairo_air::PreProcessedTraceVariant;
use clap::{Parser, Subcommand};
use stwo_cairo_prover::stwo_prover::core::pcs::PcsConfig;
use stwo_cairo_prover::stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;
use ursus::args::ProgramArguments;
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
        target: PathBuf,
        /// Path to the proof file
        proof: PathBuf,
        /// Program arguments as comma-separated values
        #[arg(
            long,
            value_delimiter = ',',
            help = "Program arguments as comma-separated values"
        )]
        arguments: Option<Vec<num_bigint::BigInt>>,
        /// Path to a file containing program arguments
        #[arg(
            long,
            conflicts_with = "arguments",
            help = "Path to a file containing program arguments"
        )]
        arguments_file: Option<camino::Utf8PathBuf>,
    },
    /// Verify a proof
    Verify {
        /// Path to the proof JSON file
        proof: PathBuf,
        /// Canonical trace, if Pedersen is included in the program.
        #[arg(short, long)]
        with_pedersen: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Prove {
            target,
            proof,
            arguments,
            arguments_file,
        } => {
            println!("Generating proof for target: {:?}", target);
            let start = Instant::now();
            let args = ProgramArguments {
                arguments: arguments.unwrap_or_default(),
                arguments_file,
            };
            let cairo_proof = execute_and_prove(target.to_str().unwrap(), args.read_arguments());
            let elapsed = start.elapsed();
            // serialize proof to file
            let proof_json = serde_json::to_string(&cairo_proof).unwrap();
            std::fs::write(proof.to_str().unwrap(), proof_json).unwrap();
            println!("Proof saved to: {:?}", proof);
            println!("Proof generation completed in {:.2?}", elapsed);
        }
        Commands::Verify {
            proof,
            with_pedersen,
        } => {
            println!("Verifying proof from: {:?}", proof);
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
            println!("Verification result: {:?}", result);
        }
    }
}
