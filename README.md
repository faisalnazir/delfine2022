# Delfine Network - The Future of Art on Solana

Delfine Network is a physical NFT network to trade passion assets, now powered by Solana blockchain.

🌐 **Website:** [http://delfine.global](http://delfine.global)

## 🚀 Overview

Delfine Network has been migrated from Ethereum/Polygon to Solana, bringing:

- **Faster transactions**: Sub-second finality with Solana
- **Lower fees**: Minimal transaction costs (~$0.00025 per transaction)
- **Scalability**: Handle thousands of transactions per second
- **Better UX**: Seamless wallet integration with Phantom and other Solana wallets

## 📦 Project Structure

```
delfine2022/
├── programs/
│   └── delfine/          # Solana program (Rust/Anchor)
├── scripts/              # Deployment scripts
├── ws/                   # SvelteKit frontend
└── tests/                # Program tests
```

## 🏗️ Architecture

### Smart Contracts (Solana Programs)

1. **DELF Token** - SPL Token implementation
   - Supply: 1,000,000,000 DELF
   - Decimals: 9
   - Standard: SPL Token

2. **FINE NFT Collection** - Metaplex NFT Standard
   - Multi-token support
   - Metadata integration
   - Mint/Burn functionality
   - Pausable minting

3. **Crowdsale Program** - Token Sale functionality
   - Exchange rate: 1000 DELF per 1 SOL
   - Configurable by authority
   - Start/Stop controls

### Frontend

- **Framework**: SvelteKit
- **Styling**: TailwindCSS + DaisyUI
- **Wallet**: Solana Wallet Adapter (Phantom, Solflare, etc.)
- **Web3**: @solana/web3.js + Anchor

## 🛠️ Installation

### Prerequisites

1. **Node.js** (v16 or higher)
   ```bash
   node --version
   ```

2. **Rust** (latest stable)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Solana CLI** (v1.16 or higher)
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
   solana --version
   ```

4. **Anchor** (v0.29.0)
   ```bash
   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
   avm install 0.29.0
   avm use 0.29.0
   anchor --version
   ```

### Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd delfine2022
   ```

2. **Install dependencies**
   ```bash
   npm install
   cd ws && npm install && cd ..
   ```

3. **Configure Solana CLI**
   ```bash
   # For devnet (testing)
   solana config set --url devnet

   # Generate a wallet if you don't have one
   solana-keygen new

   # Check your wallet address
   solana address

   # Request airdrop (devnet only)
   solana airdrop 2
   ```

## 🚀 Deployment

### Deploy to Devnet

```bash
# Build and deploy the program
npm run deploy:devnet

# Or use the script directly
./scripts/deploy-solana.sh devnet
```

This will:
1. Build the Anchor program
2. Deploy to Solana devnet
3. Initialize DELF token with 1B supply
4. Create FINE NFT collection
5. Start the crowdsale
6. Save deployment info to `deployment-info.json`

### Deploy to Mainnet

⚠️ **Warning**: Make sure you have sufficient SOL for deployment costs

```bash
npm run deploy:mainnet
```

### Update Frontend Configuration

After deployment, update the frontend with the deployed addresses:

1. Open `ws/src/routes/index.svelte`
2. Update these constants:
   ```javascript
   const PROGRAM_ID = new PublicKey('YOUR_PROGRAM_ID');
   const DELF_MINT = new PublicKey('YOUR_DELF_MINT_ADDRESS');
   ```

3. Open `ws/src/stores/solanaStore.js`
4. Update the PROGRAM_ID constant

You can find these addresses in `scripts/deployment-info.json` after deployment.

## 🧪 Testing

Run the test suite:

```bash
anchor test
```

## 💻 Development

### Start the Frontend

```bash
cd ws
npm run dev
```

The app will be available at `http://localhost:3000`

### Build the Frontend

```bash
cd ws
npm run build
```

## 🔑 Program Instructions

### Initialize DELF Token
```rust
initialize_delf_token()
```
Creates the DELF SPL token and configuration account.

### Mint Initial Supply
```rust
mint_delf_initial_supply(amount: u64)
```
Mints the initial supply of DELF tokens to the crowdsale vault.

### Buy DELF Tokens
```rust
buy_delf_tokens(sol_amount: u64)
```
Purchase DELF tokens with SOL at the configured exchange rate.

### Initialize FINE Collection
```rust
initialize_fine_collection(name: String, symbol: String, uri: String)
```
Creates the FINE NFT collection.

### Mint FINE NFT
```rust
mint_fine_nft(name: String, symbol: String, uri: String)
```
Mints a new FINE NFT to a recipient.

### Start/Stop Crowdsale
```rust
start_crowdsale()
stop_crowdsale()
```
Control the crowdsale state (authority only).

### Pause/Unpause FINE Minting
```rust
pause_fine_minting()
unpause_fine_minting()
```
Control FINE NFT minting (authority only).

## 🌐 Frontend Features

### Token Purchase Page
- Connect Solana wallet (Phantom, Solflare, etc.)
- View SOL and DELF balances
- Purchase DELF tokens with SOL
- Real-time exchange rate calculation

### DApp Interface
- View your wallet address
- Display your FINE NFTs
- Check token balances

### Art Gallery
- Browse curated artwork
- Integration with Cleveland Art Museum API
- Filter by department

## 🔐 Security

- All privileged functions require authority signature
- Pausable mechanisms for emergency stops
- PDA-based account derivation for security
- Rate limiting on the crowdsale

## 📝 Configuration

### Network Configuration

Edit `Anchor.toml` to change network settings:

```toml
[provider]
cluster = "devnet"  # or "mainnet-beta"
wallet = "~/.config/solana/id.json"
```

### Token Rate

Default: 1000 DELF per 1 SOL

Change with the `update_token_rate` instruction (authority only).

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

MIT License - see LICENSE file for details

## 🔗 Links

- Website: [http://delfine.global](http://delfine.global)
- Solana Docs: [https://docs.solana.com](https://docs.solana.com)
- Anchor Docs: [https://www.anchor-lang.com](https://www.anchor-lang.com)

## 💡 Migration Notes

This project was migrated from Ethereum/Polygon to Solana. Key changes:

| Component | Ethereum | Solana |
|-----------|----------|--------|
| Token Standard | ERC20 | SPL Token |
| NFT Standard | ERC1155 | Metaplex |
| Language | Solidity/Vyper | Rust (Anchor) |
| Framework | Brownie | Anchor |
| Wallet | MetaMask | Phantom/Solflare |
| Gas Token | MATIC/ETH | SOL |

## ❓ Troubleshooting

### Build Errors

If you encounter build errors:
```bash
# Clean build artifacts
anchor clean
rm -rf target/

# Rebuild
anchor build
```

### Deployment Fails

1. Check your SOL balance: `solana balance`
2. Request airdrop (devnet): `solana airdrop 2`
3. Verify network: `solana config get`

### Frontend Issues

1. Clear browser cache
2. Reinstall dependencies:
   ```bash
   cd ws
   rm -rf node_modules package-lock.json
   npm install
   ```

3. Check wallet connection (Phantom must be installed)

## 📞 Support

For issues and questions:
- GitHub Issues: Create an issue in this repository
- Website: [http://delfine.global](http://delfine.global)

---

**Built with ❤️ on Solana**
