use bdk::{Wallet, database::MemoryDatabase};
use bdk::bitcoin::Network;
use bdk::wallet::AddressIndex;
use bdk::keys::{DerivableKey, ExtendedKey, GeneratableKey};
use bdk::keys::bip39::{Mnemonic, WordCount, Language};

fn main() {
    // Generate a new random mnemonic phrase
    let mnemonic = Mnemonic::generate((WordCount::Words12, Language::English)).unwrap();
    println!("Generated mnemonic: {}", mnemonic.to_string());
    
    // Convert the mnemonic to a BDK compatible key
    let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
    let xprv = xkey.into_xprv(Network::Testnet).unwrap();
    
    // Create a descriptor using the xprv
    let descriptor = format!("wpkh({}/*)", xprv);
    
    // Create the wallet
    let wallet = Wallet::new(
        &descriptor,
        None,
        Network::Testnet,
        MemoryDatabase::default(),
    ).unwrap();
    
    // Generate an address
    let address = wallet.get_address(AddressIndex::New).unwrap();
    
    println!("Wallet created successfully!");
    println!("Address: {}", address);
}