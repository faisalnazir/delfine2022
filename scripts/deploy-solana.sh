#!/bin/bash

# Delfine Network Solana Deployment Script
# This script builds and deploys the Delfine program to Solana

set -e

echo "🚀 Delfine Network - Solana Deployment"
echo "======================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo -e "${RED}❌ Solana CLI not found. Please install it first.${NC}"
    echo "Visit: https://docs.solana.com/cli/install-solana-cli-tools"
    exit 1
fi

# Check if Anchor is installed
if ! command -v anchor &> /dev/null; then
    echo -e "${RED}❌ Anchor CLI not found. Please install it first.${NC}"
    echo "Visit: https://www.anchor-lang.com/docs/installation"
    exit 1
fi

echo -e "${GREEN}✅ Prerequisites installed${NC}"
echo ""

# Set network (default to devnet)
NETWORK=${1:-devnet}
echo "Network: $NETWORK"

# Configure Solana CLI
echo -e "${YELLOW}📝 Configuring Solana CLI...${NC}"
solana config set --url $NETWORK

# Check wallet balance
BALANCE=$(solana balance)
echo "Wallet balance: $BALANCE"
echo ""

# Request airdrop if on devnet and balance is low
if [[ "$NETWORK" == "devnet" ]]; then
    echo -e "${YELLOW}💰 Requesting airdrop (if needed)...${NC}"
    solana airdrop 2 || echo "Airdrop failed (might have reached limit)"
    echo ""
fi

# Build the program
echo -e "${YELLOW}🔨 Building Anchor program...${NC}"
anchor build
echo -e "${GREEN}✅ Build complete${NC}"
echo ""

# Deploy the program
echo -e "${YELLOW}🚀 Deploying program...${NC}"
anchor deploy
echo -e "${GREEN}✅ Program deployed${NC}"
echo ""

# Update program ID in Anchor.toml and lib.rs
PROGRAM_ID=$(solana address -k target/deploy/delfine-keypair.json)
echo "Program ID: $PROGRAM_ID"
echo ""

# Update Anchor.toml
echo -e "${YELLOW}📝 Updating Anchor.toml...${NC}"
sed -i.bak "s/De1FiNexxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx/$PROGRAM_ID/g" Anchor.toml
echo -e "${GREEN}✅ Updated${NC}"
echo ""

# Update lib.rs
echo -e "${YELLOW}📝 Updating lib.rs...${NC}"
sed -i.bak "s/De1FiNexxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx/$PROGRAM_ID/g" programs/delfine/src/lib.rs
echo -e "${GREEN}✅ Updated${NC}"
echo ""

# Rebuild with updated program ID
echo -e "${YELLOW}🔨 Rebuilding with updated program ID...${NC}"
anchor build
echo -e "${GREEN}✅ Rebuild complete${NC}"
echo ""

# Deploy again
echo -e "${YELLOW}🚀 Deploying updated program...${NC}"
anchor deploy
echo -e "${GREEN}✅ Program redeployed${NC}"
echo ""

# Run deployment script
echo -e "${YELLOW}📝 Initializing program accounts...${NC}"
cd scripts
ts-node deploy-solana.ts
cd ..
echo ""

echo -e "${GREEN}🎉 Deployment complete!${NC}"
echo ""
echo "Next steps:"
echo "1. Check deployment-info.json for contract addresses"
echo "2. Update frontend configuration with the new addresses"
echo "3. Test the application"
echo ""
