# Ursus

<p align="center">
  <img src="logo.png" alt="Project Logo" width="300" />
</p>



Execute with sw development, prove with stwo-cairo-prover.

## Prerequisites

- Rust, version mentioned in `rust-toolchain.toml`.
- Scarb (Optional/Recommended). To run the example it's recommended to compile with `scarb nightly-2025-04-12`.

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
    cd example
    scarb  --profile release build 
```

### Generating a Proof

To generate a proof for a Cairo program:

```bash
ursus prove <path-to-cairo-program> <output-proof-path> --arguments <args> 
```

Example:
```bash
ursus prove target/release/example.executable.json ./example_proof.json --arguments 10000
```

### Verifying a Proof

To verify an existing proof:

```bash
ursus verify <path-to-proof-file>
```

Example:
```bash
ursus verify ./example_proof.json
```

## Pre-processed trace:
When pedersen is used in the proof, more pre-processed columns are needed. The variant is automatically deduced during `prove`. 
Verify with `--with-pedersen`.
