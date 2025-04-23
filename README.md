🧠 BDK Competency Test: Migration from BDK 0.x to BDK 1.x
=========================================================

This repository demonstrates the migration of a Bitcoin wallet implementation from Bitcoin Dev Kit (BDK) 0.x (specifically version 0.28.0) to BDK 1.x (1.0.0). The project includes two directories:

*   📂 bdk-0x-wallet: A Rust-based wallet implementation using BDK 0.28.0, showcasing wallet creation, address generation, and basic setup using the older API.
    
*   📂 bdk-1x-wallet: The same wallet functionality, migrated to BDK 1.0.0, incorporating updated APIs, cleaner descriptor handling, and modern Rust practices.
    

The goal is to highlight key differences, challenges, and solutions encountered during migration—mirroring real-world efforts like upgrading Galoy's [Bria](https://github.com/GaloyMoney/bria) project to BDK 1.0.

📦 Prerequisites
----------------

*   **Rust**: Installed via [rustup.rs](https://rustup.rs/) (recommended version: rustc >= 1.65)
    
*   **Git**: To clone this repository
    
*   **Electrum Server**: For blockchain synchronization (e.g., electrum.blockstream.info:50002 for testnet)
    
*   **Dependencies**: Managed via Cargo.toml in each project folder
    

🛠️ Setup Instructions
----------------------

### 1\. Clone the Repository

Plain textANTLR4BashCC#CSSCoffeeScriptCMakeDartDjangoDockerEJSErlangGitGoGraphQLGroovyHTMLJavaJavaScriptJSONJSXKotlinLaTeXLessLuaMakefileMarkdownMATLABMarkupObjective-CPerlPHPPowerShell.propertiesProtocol BuffersPythonRRubySass (Sass)Sass (Scss)SchemeSQLShellSwiftSVGTSXTypeScriptWebAssemblyYAMLXML`   git clone https://github.com//bdk-competency-test.git  cd bdk-competency-test   `

### 2\. Build a Wallet

Navigate to the desired wallet directory:

*   For BDK 0.x: cd bdk-0x-wallet
    
*   For BDK 1.x: cd bdk-1x-wallet
    

Install dependencies:

Plain textANTLR4BashCC#CSSCoffeeScriptCMakeDartDjangoDockerEJSErlangGitGoGraphQLGroovyHTMLJavaJavaScriptJSONJSXKotlinLaTeXLessLuaMakefileMarkdownMATLABMarkupObjective-CPerlPHPPowerShell.propertiesProtocol BuffersPythonRRubySass (Sass)Sass (Scss)SchemeSQLShellSwiftSVGTSXTypeScriptWebAssemblyYAMLXML`   cargo build   `

### 3\. Run the Wallet

Execute the wallet (edit src/main.rs for testnet/mainnet):

Plain textANTLR4BashCC#CSSCoffeeScriptCMakeDartDjangoDockerEJSErlangGitGoGraphQLGroovyHTMLJavaJavaScriptJSONJSXKotlinLaTeXLessLuaMakefileMarkdownMATLABMarkupObjective-CPerlPHPPowerShell.propertiesProtocol BuffersPythonRRubySass (Sass)Sass (Scss)SchemeSQLShellSwiftSVGTSXTypeScriptWebAssemblyYAMLXML`   cargo run   `

Expect outputs like generated addresses and wallet balance.

🚀 Migration Highlights
-----------------------

Migrating from BDK 0.x to 1.x involves adapting to new APIs, improved descriptor handling, and streamlined dependencies. Here’s what changed:

### 🔑 Wallet & Descriptors

*   **BDK 0.x**:
    
    *   Manual descriptor strings (e.g., wpkh(\[fingerprint/derivation\]xpub/0/\*)).
        
    *   Requires xprv derivation with bitcoin or bip39 crates.
        
    *   Issue: Descriptor parsing errors (e.g., "Invalid public key format").
        
    *   Fix: Derived xprv using bip32::ExtendedPrivKey.
        
*   **BDK 1.x**:
    
    *   Simplified templates (e.g., wpkh()) via bdk\_wallet::keys::DescriptorKey.
        
    *   Fewer manual steps, better error messages.
        
    *   Update: Switched to built-in key expressions.
        

### 🌐 Blockchain Sync

*   **BDK 0.x**:
    
    *   Uses bdk::blockchain::ElectrumBlockchain with manual wallet.sync().
        
    *   Verbose setup.
        
*   **BDK 1.x**:
    
    *   Streamlined with bdk\_wallet::chain::electrum::ElectrumBlockchain.
        
    *   Uses FullScanRequest for efficient syncing.
        
    *   Update: Adopted new sync APIs.
        

### 💸 Transactions

*   **BDK 0.x**:
    
    *   TxBuilder with explicit input/output config.
        
    *   Limited RBF support.
        
*   **BDK 1.x**:
    
    *   Fluent TxBuilder API (e.g., add\_recipient()).
        
    *   Native RBF via enable\_rbf().
        
    *   Update: Rewrote transaction logic for modern practices.
        

### 📚 Dependencies

*   **BDK 0.x**:
    
    *   bdk crate (0.28.0) with "electrum", "keys-bip39".
        
    *   Extra crates like bitcoin, bip39.
        
*   **BDK 1.x**:
    
    *   bdk\_wallet (1.0.0) with integrated key management.
        
    *   Leaner Cargo.toml.
        
    *   Update: Removed redundant dependencies.
        

⚠️ Migration Challenges
-----------------------

*   **Descriptor Errors**: BDK 0.x failed on raw seeds; fixed in 1.x with proper key derivation.
    
*   **API Overhaul**: Legacy bdk::Wallet methods deprecated, requiring a rewrite.
    
*   **Testing**: Validated both wallets on testnet with matching mnemonics.
    

🎮 Try It Out
-------------

*   cd bdk-0x-walletcargo runSee addresses generated with wpkh descriptors.
    
*   cd bdk-1x-walletcargo runSame functionality, modernized with RBF support.
    

📖 Resources
------------

*   [BDK 1.0.0 Documentation](https://docs.rs/bdk_wallet/1.0.0/bdk_wallet/)
    
*   [BDK Website](https://bitcoindevkit.org/)
    
*   [Bria Repository](https://github.com/GaloyMoney/bria)
    

📜 License
----------

MIT License