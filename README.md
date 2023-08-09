# Example prover application
Gets some value onchain, proves it, verifies onchain and executes some other extrinsic if verified.

Value stored is just a hash commitment. Offchain prover can prove knowledge of it.

## Installation
Please follow the setup instructions located on the [Substrate docs site](https://docs.substrate.io/install/). Check the setup by running `cargo check` in the directory root.

### Prover
Generates a proof of knowledge of the hash preimage, then calls the pallet extrinsic with the proof

## Usage instructions

### Start the chain
First, build the chain with `cargo build --release`
Run the local version of the chain with `./target/release/node-template --dev`

### Run the prover
In `./prover`, run `cargo run`
