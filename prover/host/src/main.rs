// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::{Arg, Command};
use subxt::{
	ext::sp_core::{sr25519::Pair as SubxtPair, Pair as SubxtPairT},
	tx::PairSigner,
	OnlineClient, PolkadotConfig,
};

// Runtime types, etc
#[subxt::subxt(runtime_metadata_path = "./metadata.scale")]
pub mod substrate_node {}

use risc0_zkvm::{
    SegmentReceipt,
    serde::to_vec,
    // sha::Digest,
    Executor, ExecutorEnv, SessionReceipt,
};
use methods::{HASH_ELF, HASH_ID};

/// Hash the given bytes, returning the digest and a [SessionReceipt] that can
/// be used to verify that the that the hash was computed correctly (i.e. that
/// the Prover knows a preimage for the given SHA-256 hash)
// fn provably_hash(input: &str) -> (Digest, SessionReceipt) {
fn provably_hash(input: &str) -> SessionReceipt {
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(input).unwrap())
        .build();

    let elf = HASH_ELF;
    let mut exec = Executor::from_elf(env.clone(), elf).unwrap();

    let session = exec.run().unwrap();
    let receipt = session.prove().unwrap();

    // TODO: Get digest here
    // let digest = from_slice::<Vec<u8>, _>(&receipt.journal)
    //     .unwrap()
    //     .try_into()
    //     .unwrap();
    // (digest, receipt)
    receipt
}

#[tokio::main]
async fn main() {
    // Parse command line
    let matches = Command::new("hash")
        .arg(Arg::new("message").default_value(""))
        .get_matches();
    let message = matches.get_one::<String>("message").unwrap();

    // Prove hash the message.
    // let (digest, receipt) = provably_hash(message);
    let receipt = provably_hash(message);

    println!("Current image id is: {:?}", HASH_ID);

    // Verify the receipt, ensuring the prover knows a valid SHA-256 preimage.
    receipt
        .verify(HASH_ID)
        .expect("receipt verification failed");

    let substrate_receipt = receipt
        .segments
        .into_iter()
        .map(|SegmentReceipt { seal, index }| (seal, index))
        .collect();

    // This is the well-known //Alice key. Don't use in a real application
	let restored_key = SubxtPair::from_string(
		"0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a",
		None,
	)
	.unwrap();
	let signer = PairSigner::new(restored_key);

    let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();

    api.tx()
    .sign_and_submit_then_watch_default(
        &substrate_node::tx()
            // Name of the pallet in chain metadata: example
            .example()
            // Specify the extrinsic and arguments
            .verify_preimage_proof(substrate_receipt, receipt.journal,),
        &signer,
    )
    .await
    .unwrap()
    .wait_for_finalized()
    .await
    .unwrap();
println!("Sent tx");

    // println!("I provably know data whose SHA-256 hash is {}", digest);
}
