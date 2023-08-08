# Example prover application
Gets some value onchain, proves it, verifies onchain and executes some other extrinsic if verified.

Value stored is just a hash commitment. Offchain prover can prove knowledge of it.

## Project Structure
```
├── node: contains all code for Substrate chain
│   ├── pallets
│   │   ├── example: pallet which stores some value the prover wants to retrieve. In this case the thing stored is a hash commitment. Also  verifies proofs.
│   ├── runtime
│   ├── node
├── examples: Contains code for example program
│   ├── factors: CLI for building and uploading "Factors" example program
│   ├── methods: Core logic for example program
├── prover: Prover application for proving one program execution and uploading its proof onchain
```

## Installation
Please follow the setup instructions located on the [Substrate docs site](https://docs.substrate.io/install/). Check the setup by running `cargo check` in the directory root.


## Usage instructions

### Start the chain


### Prover
