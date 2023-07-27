#![allow(dead_code, unused)]
use bdk::bitcoin::util::bip32::{ExtendedPubKey, Fingerprint};
use bdk::bitcoin::Network;
use bdk::template::{Bip84Public, DescriptorTemplate};
use bdk::database::memory::MemoryDatabase;
use bdk::Wallet;
use bdk::wallet::{AddressIndex, signer::SignOptions};
use slip132::FromSlip132;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    xpub: String,

    #[arg(long, default_value_t = 0)]
    position: u32,

    #[arg(long, default_value_t = false)]
    testnet: bool,
}

fn main() {
    
    let args = Cli::parse(); 
    let mut network:Network = Network::Bitcoin;

    if args.testnet {
        network = Network::Testnet
    }

    let slip132_xpub = ExtendedPubKey::from_slip132_str(args.xpub.as_str()).unwrap();
    let fingerprint = slip132_xpub.parent_fingerprint;
    let descriptor_bip84_public = Bip84Public(slip132_xpub.clone(), fingerprint, bdk::KeychainKind::External).build(network).unwrap();
    let descriptor_bip84_public_internal = Bip84Public(slip132_xpub.clone(), fingerprint, bdk::KeychainKind::Internal).build(network).unwrap();
     
    let wallet = Wallet::new_offline(
        descriptor_bip84_public,
        Some(descriptor_bip84_public_internal),
        network,
        MemoryDatabase::default()
    ).unwrap();

    println!("{}", wallet.get_address(AddressIndex::Peek(args.position)).unwrap().address);
}