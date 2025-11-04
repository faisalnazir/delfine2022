# 🚀 Quick Start Guide

Get Delfine Network running on Solana in 5 minutes!

## Prerequisites Check

```bash
# Check if you have the required tools
node --version    # Should be v16+
solana --version  # Should be v1.16+
anchor --version  # Should be v0.29.0
```

If any are missing, see [README.md](README.md#installation) for installation instructions.

## Step 1: Install Dependencies (2 min)

```bash
# Clone and enter directory
cd delfine2022

# Install root dependencies
npm install

# Install frontend dependencies
cd ws && npm install && cd ..
```

## Step 2: Configure Solana (1 min)

```bash
# Set network to devnet
solana config set --url devnet

# Create a wallet (or use existing)
solana-keygen new --outfile ~/.config/solana/id.json

# Get some SOL for testing
solana airdrop 2
```

## Step 3: Build & Deploy (2 min)

```bash
# Build the Solana program
anchor build

# Deploy to devnet
anchor deploy

# Initialize the program
cd scripts
ts-node deploy-solana.ts
cd ..
```

**Save the addresses** shown in the output - you'll need them!

## Step 4: Update Frontend Config (30 sec)

Edit `ws/src/routes/index.svelte` and `ws/src/stores/solanaStore.js`:

```javascript
// Replace these placeholders with your deployed addresses
const PROGRAM_ID = new PublicKey('YOUR_PROGRAM_ID_FROM_STEP_3');
const DELF_MINT = new PublicKey('YOUR_DELF_MINT_FROM_STEP_3');
```

## Step 5: Run the App (30 sec)

```bash
cd ws
npm run dev
```

Open http://localhost:3000 in your browser!

## 🎯 What You Can Do Now

1. **Connect Wallet**: Click "Connect Wallet" (install Phantom first)
2. **Buy DELF Tokens**: Enter SOL amount and click "BUY DELF Tokens"
3. **Check Balance**: See your DELF balance update in real-time

## Common Issues

### "Insufficient funds"
```bash
solana airdrop 2
```

### "Program not deployed"
Make sure you ran `anchor deploy` and updated the PROGRAM_ID in frontend.

### "Wallet not found"
Install Phantom wallet extension: https://phantom.app

## Next Steps

- Read the [full documentation](README.md)
- Check [migration guide](MIGRATION_GUIDE.md) for architecture details
- Run tests: `anchor test`
- Deploy to mainnet: `npm run deploy:mainnet` (only when ready!)

## 🆘 Need Help?

- Check [README.md](README.md#troubleshooting) troubleshooting section
- Open a GitHub issue
- Visit http://delfine.global

---

**Happy building! 🏗️**
