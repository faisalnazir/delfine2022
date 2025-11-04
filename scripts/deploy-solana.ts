/**
 * Delfine Network Solana Deployment Script
 *
 * This script deploys the Delfine program and initializes:
 * 1. DELF SPL Token (1 billion supply)
 * 2. FINE NFT Collection
 * 3. Crowdsale configuration
 */

import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import {
  PublicKey,
  Keypair,
  SystemProgram,
  LAMPORTS_PER_SOL
} from '@solana/web3.js';
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  createAssociatedTokenAccountInstruction,
} from '@solana/spl-token';
import fs from 'fs';

// Load the program IDL
const idl = JSON.parse(
  fs.readFileSync('../target/idl/delfine.json', 'utf8')
);

async function main() {
  console.log('🚀 Starting Delfine Network Solana Deployment...\n');

  // Configure the client to use the devnet cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const programId = new PublicKey(idl.metadata.address);
  const program = new Program(idl, programId, provider);

  console.log('Program ID:', programId.toString());
  console.log('Deployer:', provider.wallet.publicKey.toString());
  console.log('Network:', provider.connection.rpcEndpoint);
  console.log('');

  // Check deployer balance
  const balance = await provider.connection.getBalance(provider.wallet.publicKey);
  console.log(`Deployer balance: ${balance / LAMPORTS_PER_SOL} SOL`);

  if (balance < 0.5 * LAMPORTS_PER_SOL) {
    console.error('❌ Insufficient balance. Need at least 0.5 SOL for deployment.');
    console.log('Request airdrop with: solana airdrop 2');
    return;
  }
  console.log('');

  try {
    // ===========================================================================
    // Step 1: Initialize DELF Token
    // ===========================================================================
    console.log('📝 Step 1: Initializing DELF Token...');

    const [configPda, configBump] = await PublicKey.findProgramAddress(
      [Buffer.from('config')],
      programId
    );

    // Create DELF mint keypair
    const delfMint = Keypair.generate();

    console.log('Config PDA:', configPda.toString());
    console.log('DELF Mint:', delfMint.publicKey.toString());

    // Initialize DELF token
    const initDelfTx = await program.methods
      .initializeDelfToken()
      .accounts({
        config: configPda,
        delfMint: delfMint.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([delfMint])
      .rpc();

    console.log('✅ DELF Token initialized. Tx:', initDelfTx);
    console.log('');

    // ===========================================================================
    // Step 2: Mint Initial DELF Supply (1 billion tokens)
    // ===========================================================================
    console.log('📝 Step 2: Minting initial DELF supply...');

    // Create crowdsale vault (associated token account for config PDA)
    const crowdsaleVault = await getAssociatedTokenAddress(
      delfMint.publicKey,
      configPda,
      true
    );

    console.log('Crowdsale Vault:', crowdsaleVault.toString());

    // Create the vault account
    const createVaultIx = createAssociatedTokenAccountInstruction(
      provider.wallet.publicKey,
      crowdsaleVault,
      configPda,
      delfMint.publicKey
    );

    const createVaultTx = new anchor.web3.Transaction().add(createVaultIx);
    await provider.sendAndConfirm(createVaultTx);

    // Mint 1 billion DELF tokens (with 9 decimals)
    const initialSupply = new anchor.BN(1_000_000_000).mul(
      new anchor.BN(10).pow(new anchor.BN(9))
    );

    const mintTx = await program.methods
      .mintDelfInitialSupply(initialSupply)
      .accounts({
        config: configPda,
        delfMint: delfMint.publicKey,
        destination: crowdsaleVault,
        authority: provider.wallet.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log('✅ Minted 1,000,000,000 DELF tokens. Tx:', mintTx);
    console.log('');

    // ===========================================================================
    // Step 3: Initialize FINE NFT Collection
    // ===========================================================================
    console.log('📝 Step 3: Initializing FINE NFT Collection...');

    const [collectionConfigPda, collectionBump] = await PublicKey.findProgramAddress(
      [Buffer.from('fine_collection')],
      programId
    );

    const collectionMint = Keypair.generate();

    const initCollectionTx = await program.methods
      .initializeFineCollection(
        'DELFINE FINE',
        'FINE',
        'https://delfine.global/metadata/collection.json'
      )
      .accounts({
        collectionConfig: collectionConfigPda,
        collectionMint: collectionMint.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([collectionMint])
      .rpc();

    console.log('✅ FINE Collection initialized. Tx:', initCollectionTx);
    console.log('Collection Config PDA:', collectionConfigPda.toString());
    console.log('Collection Mint:', collectionMint.publicKey.toString());
    console.log('');

    // ===========================================================================
    // Step 4: Start Crowdsale
    // ===========================================================================
    console.log('📝 Step 4: Starting crowdsale...');

    const startCrowdsaleTx = await program.methods
      .startCrowdsale()
      .accounts({
        config: configPda,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    console.log('✅ Crowdsale started. Tx:', startCrowdsaleTx);
    console.log('');

    // ===========================================================================
    // Summary
    // ===========================================================================
    console.log('🎉 Deployment Complete!\n');
    console.log('='.repeat(70));
    console.log('DEPLOYMENT SUMMARY');
    console.log('='.repeat(70));
    console.log('Network:', provider.connection.rpcEndpoint);
    console.log('Program ID:', programId.toString());
    console.log('');
    console.log('DELF Token:');
    console.log('  Mint Address:', delfMint.publicKey.toString());
    console.log('  Config PDA:', configPda.toString());
    console.log('  Crowdsale Vault:', crowdsaleVault.toString());
    console.log('  Initial Supply: 1,000,000,000 DELF');
    console.log('  Token Rate: 1000 DELF per 1 SOL');
    console.log('');
    console.log('FINE NFT Collection:');
    console.log('  Collection Mint:', collectionMint.publicKey.toString());
    console.log('  Collection Config:', collectionConfigPda.toString());
    console.log('');
    console.log('Status:');
    console.log('  Crowdsale Active: YES');
    console.log('='.repeat(70));
    console.log('');
    console.log('⚠️  IMPORTANT: Save these addresses!');
    console.log('');
    console.log('Update the following in your frontend:');
    console.log(`  PROGRAM_ID: "${programId.toString()}"`);
    console.log(`  DELF_MINT: "${delfMint.publicKey.toString()}"`);
    console.log('');

    // Save deployment info to file
    const deploymentInfo = {
      network: provider.connection.rpcEndpoint,
      programId: programId.toString(),
      delfMint: delfMint.publicKey.toString(),
      configPda: configPda.toString(),
      crowdsaleVault: crowdsaleVault.toString(),
      collectionMint: collectionMint.publicKey.toString(),
      collectionConfig: collectionConfigPda.toString(),
      deployer: provider.wallet.publicKey.toString(),
      timestamp: new Date().toISOString(),
    };

    fs.writeFileSync(
      './deployment-info.json',
      JSON.stringify(deploymentInfo, null, 2)
    );

    console.log('✅ Deployment info saved to: ./deployment-info.json');

  } catch (error) {
    console.error('❌ Deployment failed:', error);
    throw error;
  }
}

main()
  .then(() => {
    console.log('\n✅ Script completed successfully');
    process.exit(0);
  })
  .catch((error) => {
    console.error('\n❌ Script failed:', error);
    process.exit(1);
  });
