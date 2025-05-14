# Ursus

Ursus is a Cairo program prover and verifier that enables generating and verifying proofs for Cairo programs.


## Installation

```bash
# Clone the repository
git clone <repository-url>
cd ursus

# Build the project with native CPU optimizations
./build.sh

# Optional: Add the binary to your PATH
# For example, to add it to /usr/local/bin:
sudo cp target/release/ursus /usr/local/bin/
```

## Usage

Ursus provides two main commands: `prove` and `verify`.

### Generating a Proof

To generate a proof for a Cairo program:

```bash
ursus prove --target <path-to-cairo-program> --proof <output-proof-path>
```

Example:
```bash
ursus prove --target ./programs/example.cairo --proof ./proofs/example_proof.json
```

### Verifying a Proof

To verify an existing proof:

```bash
ursus verify --proof <path-to-proof-file>
```

Example:
```bash
ursus verify --proof ./proofs/example_proof.json
```

## Command Line Arguments

### Prove Command
- `--target` or `-t`: Path to the Cairo program file to prove
- `--proof` or `-p`: Path where the generated proof will be saved (JSON format)

### Verify Command
- `--proof` or `-p`: Path to the proof file to verify

## Output

- When generating a proof, the proof will be saved as a JSON file at the specified path
- When verifying a proof, the result of the verification will be printed to the console

## License

[Add your license information here] 