# Migration Guide: Ethereum → Solana

This document explains the migration from Ethereum/Polygon to Solana for the Delfine Network.

## Overview

The Delfine Network was successfully migrated from Ethereum-based smart contracts (Solidity/Vyper) to Solana programs (Rust/Anchor), improving performance and reducing costs.

## Key Changes

### 1. Smart Contract → Solana Program

| Aspect | Before (Ethereum) | After (Solana) |
|--------|-------------------|----------------|
| Language | Solidity/Vyper | Rust (Anchor framework) |
| Token Standard | ERC20 | SPL Token |
| NFT Standard | ERC1155 | Metaplex |
| Account Model | Account-based | Account-based with PDAs |
| Gas Model | Variable gas fees | Fixed transaction fees |

### 2. Token Standards

#### DELF Token (ERC20 → SPL Token)

**Before (ERC20):**
```solidity
contract DELF {
    mapping(address => uint256) balances;
    function transfer(address to, uint256 amount) {...}
}
```

**After (SPL Token):**
- Native Solana token using SPL Token Program
- Managed through Anchor CPI calls
- Token accounts created automatically via Associated Token Program

#### FINE NFT (ERC1155 → Metaplex)

**Before (ERC1155):**
```solidity
contract FINE is ERC1155 {
    function mint(address to, uint256 id, uint256 amount) {...}
}
```

**After (Metaplex NFT):**
- One mint per NFT (0 decimals)
- Rich metadata support via Metaplex
- Collection-based organization

### 3. Crowdsale Contract

**Before (Vyper):**
```python
@external
@payable
def buyTokens(_beneficiary: address):
    weiAmount: uint256 = msg.value
    tokens: uint256 = weiAmount * self.rate
    self.token.transferFrom(self.wallet, _beneficiary, tokens)
```

**After (Anchor/Rust):**
```rust
pub fn buy_delf_tokens(ctx: Context<BuyDelfTokens>, sol_amount: u64) -> Result<()> {
    let delf_amount = sol_amount.checked_mul(ctx.accounts.config.token_rate)?;
    // Transfer SOL
    // Transfer DELF tokens
    Ok(())
}
```

### 4. Frontend Integration

#### Wallet Connection

**Before (MetaMask):**
```javascript
import { defaultEvmStores } from 'svelte-web3';
defaultEvmStores.setBrowserProvider();
```

**After (Phantom):**
```javascript
import { PhantomWalletAdapter } from '@solana/wallet-adapter-wallets';
import { workSpace } from './stores/solanaStore';

const walletAdapter = new PhantomWalletAdapter();
await walletAdapter.connect();
```

#### Contract Interaction

**Before (Web3.js/Ethers):**
```javascript
const contract = new web3.eth.Contract(ABI, address);
await contract.methods.buyTokens(account)
    .send({ value: amount, from: account });
```

**After (Anchor):**
```javascript
const tx = await program.methods
    .buyDelfTokens(new BN(amount))
    .accounts({...})
    .rpc();
```

### 5. Deployment

**Before (Brownie):**
```python
from brownie import DELF, accounts

def deploy():
    account = accounts.load('deployer')
    delf = DELF.deploy({'from': account})
    return delf
```

**After (Anchor):**
```bash
anchor build
anchor deploy
ts-node scripts/deploy-solana.ts
```

## Architecture Comparison

### Account Structure

**Ethereum:**
- Single contract address
- Internal state storage
- Mapping-based data structures

**Solana:**
- Multiple accounts (program, data accounts, token accounts)
- PDA (Program Derived Addresses) for deterministic accounts
- Explicit account passing

### Example: Token Purchase

**Ethereum Flow:**
1. User calls `buyTokens()` on crowdsale contract
2. Contract transfers tokens from internal mapping
3. Single transaction

**Solana Flow:**
1. User calls `buy_delf_tokens` instruction
2. Explicitly pass all accounts:
   - Config account (PDA)
   - DELF mint
   - Crowdsale vault (token account)
   - Buyer's token account
   - System programs
3. Multiple account interactions in single transaction

## Benefits of Migration

### Performance
- **Transaction Speed**: <1s finality vs 2-15s on Ethereum
- **TPS**: 50,000+ vs 15-30 on Ethereum
- **Cost**: $0.00025 per tx vs $0.10-$50 on Ethereum

### Developer Experience
- **Type Safety**: Rust's type system prevents many bugs
- **Testing**: Better testing framework with Anchor
- **Composability**: CPI (Cross-Program Invocation) enables powerful composability

### User Experience
- **Faster confirmations**: Near-instant transactions
- **Lower costs**: Negligible fees
- **Better wallets**: Phantom, Solflare provide superior UX

## Breaking Changes

### For Users
1. **Different wallet**: Need Phantom or Solflare instead of MetaMask
2. **New token addresses**: DELF and FINE have new addresses on Solana
3. **Migration required**: Old tokens on Polygon need to be migrated (contact team)

### For Developers
1. **Complete rewrite**: Solidity/Vyper code doesn't port directly
2. **New paradigms**: Learn Solana's account model and Anchor framework
3. **Different tools**: Replace Hardhat/Brownie with Anchor CLI

## Migration Checklist

- [x] Rewrite smart contracts in Rust/Anchor
- [x] Implement SPL Token for DELF
- [x] Implement Metaplex NFT for FINE
- [x] Create crowdsale program
- [x] Update frontend to Solana wallet adapter
- [x] Create deployment scripts
- [x] Write comprehensive tests
- [x] Update documentation
- [ ] Deploy to devnet for testing
- [ ] Audit smart contracts
- [ ] Deploy to mainnet
- [ ] Set up token migration bridge
- [ ] Update website
- [ ] Announce migration to community

## Common Pitfalls

### 1. Account Size
Solana accounts must specify size at creation. Calculate carefully:
```rust
impl Config {
    pub const LEN: usize = 32 + 32 + 8 + 1 + 8 + 1;
}
```

### 2. PDA Seeds
Use consistent seeds for PDA derivation:
```rust
let (config_pda, bump) = PublicKey::findProgramAddress(
    &[b"config"],
    program_id
);
```

### 3. Token Account Creation
Always check if associated token account exists before transfer:
```rust
#[account(
    init_if_needed,
    payer = buyer,
    associated_token::mint = delf_mint,
    associated_token::authority = buyer,
)]
pub buyer_token_account: Account<'info, TokenAccount>,
```

### 4. Arithmetic Overflow
Use checked math operations:
```rust
let delf_amount = sol_amount
    .checked_mul(config.token_rate)
    .ok_or(DelfineError::Overflow)?;
```

## Testing

### Before (Hardhat/Brownie)
```python
def test_buy_tokens(delf, crowdsale, accounts):
    crowdsale.buyTokens(accounts[1], {'from': accounts[1], 'value': 1e18})
    assert delf.balanceOf(accounts[1]) == 1000e18
```

### After (Anchor)
```typescript
it("Buys DELF tokens", async () => {
    await program.methods
        .buyDelfTokens(new BN(LAMPORTS_PER_SOL))
        .accounts({...})
        .rpc();

    const balance = await getAccount(connection, buyerTokenAccount);
    assert.equal(balance.amount, 1000 * 1e9);
});
```

## Resources

- [Solana Documentation](https://docs.solana.com)
- [Anchor Book](https://book.anchor-lang.com)
- [SPL Token Docs](https://spl.solana.com/token)
- [Metaplex Docs](https://docs.metaplex.com)

## Support

If you encounter issues during migration or have questions:
- GitHub Issues: [Create an issue](https://github.com/your-repo/issues)
- Discord: [Join our server](#)
- Email: support@delfine.global

---

**Migration completed:** 2024
**Migrated by:** Delfine Network Team
