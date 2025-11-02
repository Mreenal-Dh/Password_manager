🔐 Password Manager on Blockchain
A decentralized password management system built on Stellar blockchain using Soroban smart contracts. Store your encrypted passwords on-chain where only you hold the decryption key.

Show Image
Show Image
Show Image

🌟 Why Blockchain for Passwords?
No Central Database - Eliminate single points of failure
User Sovereignty - You own your data through cryptographic keys
Zero-Knowledge - Passwords encrypted client-side before storage
Censorship Resistant - No entity can block your access
Immutable Audit Trail - Complete transparency of all operations
🚀 Quick Start
Prerequisites
bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features opt
Build & Deploy
bash
# Clone repository
git clone https://github.com/yourusername/password-manager-blockchain.git
cd password-manager-blockchain/password-manager

# Build contract
cargo build --target wasm32-unknown-unknown --release

# Configure testnet
stellar network add --global testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"

# Create and fund account
stellar keys generate --global alice --network testnet
stellar keys fund alice --network testnet

# Deploy
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/password_manager.wasm \
  --source alice \
  --network testnet
📖 Usage Example
bash
# Store encrypted password
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- store_password \
  --user <YOUR_ADDRESS> \
  --service_name "GitHub" \
  --username "user@example.com" \
  --encrypted_password "<ENCRYPTED_DATA>"

# Retrieve password
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- get_password \
  --user <YOUR_ADDRESS> \
  --entry_id 1
🔑 Core Functions
Function	Description
store_password()	Store new encrypted password entry
get_password()	Retrieve password by entry ID
update_password()	Update existing password
get_entry_count()	Get total number of stored entries
🛡️ Security
⚠️ Critical: Always encrypt passwords client-side before storing on-chain!

javascript
// Example: Client-side encryption with crypto-js
const CryptoJS = require("crypto-js");
const encrypted = CryptoJS.AES.encrypt(password, masterPassword).toString();
Best Practices:

Use AES-256-GCM encryption
Never store master password or private keys
Keep wallet keys secure (use hardware wallets)
Audit smart contract before mainnet deployment
🏗️ Project Structure
password-manager-blockchain/
├── password-manager/
│   ├── src/
│   │   └── lib.rs          # Smart contract code
│   ├── Cargo.toml           # Dependencies
│   └── contract_id.txt      # Deployed contract ID
├── README.md
└── LICENSE
🎯 Roadmap
 Core password CRUD operations
 Testnet deployment
 Browser extension
 Mobile apps (iOS/Android)
 Password sharing with encryption
 Multi-factor authentication
 Cross-chain support
 Mainnet deployment
🤝 Contributing
Contributions welcome! Please read our Contributing Guide first.

bash
# Fork and clone
git clone https://github.com/yourusername/password-manager-blockchain.git

# Create feature branch
git checkout -b feature/amazing-feature

# Make changes and test
cargo test

# Submit PR
📝 License
This project is licensed under the MIT License - see LICENSE file for details.

⚠️ Disclaimer
This is a proof-of-concept for educational purposes. Use at your own risk.

Not audited for production use
No warranty provided
Users responsible for key management
Loss of keys = permanent loss of access
🔗 Resources
Stellar Documentation
Soroban Docs
Full Documentation
Deployment Guide
📧 Contact
GitHub Issues: Report bugs
Discord: Join community
Twitter: @YourHandle
Built with ❤️ on Stellar | Empowering digital sovereignty through decentralization

⭐ Star this repo if you find it useful!

<img width="1920" height="1080" alt="Screenshot 2025-11-02 163408" src="https://github.com/user-attachments/assets/d45d6471-043d-4693-b37b-885049df8950" />
