# Ursus

Execute with sw development, prove with stwo-cairo-prover.

## Prerequisites

- Scarb 2.11.4
- Rust, version mentioned in `rust-toolchain.toml`.

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd ursus

# Build the project,
./build.sh

# Optional: Add the binary to your PATH
# For example, to add it to /usr/local/bin:
sudo cp target/release/ursus /usr/local/bin/
```

## Usage

### Compiling a project.

Use scarb to compile an executable:
```bash
    scarb  --profile release build 
```

### Generating a Proof

To generate a proof for a Cairo program:

```bash
ursus prove <path-to-cairo-program> <output-proof-path>
```

Example:
```bash
ursus prove <project_dir>/target/release/example.executable.json ./proofs/example_proof.json
```

### Verifying a Proof

To verify an existing proof:

```bash
ursus verify <path-to-proof-file>
```

Example:
```bash
ursus verify ./proofs/example_proof.json
```

## Pre-processed trace:
When pedersen is used in the proof, more pre-processed columns are needed. The variant is automatically deduced during `prove`. 
Verify with `--with-pedersen`.

## Output

- When generating a proof, the proof will be saved as a JSON file at the specified path
- When verifying a proof, the result of the verification will be printed to the console
