# RISC Zero Rust Starter Template

Welcome to the RISC Zero Rust Starter Template! This template is intended to give you a starting point for building a project using the RISC Zero zkVM. Throughout the code are comments labelled `TODO` in places where we expect projects will need to modify the code.

TODO: Replace this README with a README for your project
TODO: Verify whether the included `.gitignore`, `LICENSE`, and `rust-toolchain` files are appropriate to your project

## Quick Start

# Example Prover Application

This is just an example prover application based on the RISC Zero sha example which can create the proofs that the onchcain logic expects to verify.

## Sync chain metadata
If the onchain extrinsic signatures have been updated, the local metadata will need to be updated. Run `subxt metadata -f bytes > metadata.scale`

## Running
Run `cargo run` to run the prover and send a transaction which uploads the proof