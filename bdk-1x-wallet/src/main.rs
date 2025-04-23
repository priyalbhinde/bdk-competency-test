use anyhow::Result;
use bdk_wallet::{
    descriptor::template::Bip84,
    keys::{bip39::{Language, Mnemonic, WordCount}, GeneratableKey},
    Wallet, // Import Wallet directly from the crate root
    KeychainKind,
};
use bitcoin::Network;

fn main() -> Result<()> {
    // Generate a new random mnemonic phrase
    let mnemonic = Mnemonic::generate((WordCount::Words12, Language::English)).unwrap();
    println!("Generated mnemonic: {}", mnemonic.to_string());
    

    // 2. Build BIP84 (native SegWit) descriptor templates
    let external_desc = Bip84(mnemonic.clone(), KeychainKind::External);
    let internal_desc = Bip84(mnemonic, KeychainKind::Internal);

    // 3. Create the wallet in memory
    let mut wallet = Wallet::create(external_desc, internal_desc)
        .network(Network::Testnet)
        .create_wallet_no_persist()?;

    // 4. Reveal (or peek) the next external address
    let address_info = wallet.reveal_next_address(KeychainKind::External);
    println!("Wallet created successfully!");
    println!("Address: {}", address_info.address);

    Ok(())
}