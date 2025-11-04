use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer, MintTo, Burn},
};
use mpl_token_metadata::{
    instruction as mpl_instruction,
    ID as TOKEN_METADATA_ID,
};

declare_id!("De1FiNexxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");

/// Delfine Network Program
/// Implements SPL Token (DELF), NFT Minting (FINE), and Crowdsale functionality
#[program]
pub mod delfine {
    use super::*;

    /// Initialize the DELF SPL Token
    /// Supply: 1,000,000,000 tokens with 9 decimals
    pub fn initialize_delf_token(ctx: Context<InitializeDelfToken>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.delf_mint = ctx.accounts.delf_mint.key();
        config.token_rate = 1000; // 1000 DELF per 1 SOL
        config.crowdsale_active = false;
        config.total_delf_sold = 0;
        config.bump = *ctx.bumps.get("config").unwrap();

        msg!("DELF Token initialized successfully");
        Ok(())
    }

    /// Mint initial supply of DELF tokens to the authority
    pub fn mint_delf_initial_supply(
        ctx: Context<MintDelfTokens>,
        amount: u64,
    ) -> Result<()> {
        require!(
            ctx.accounts.authority.key() == ctx.accounts.config.authority,
            DelfineError::Unauthorized
        );

        let seeds = &[
            b"config",
            &[ctx.accounts.config.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.delf_mint.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.config.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::mint_to(cpi_ctx, amount)?;

        msg!("Minted {} DELF tokens", amount);
        Ok(())
    }

    /// Initialize FINE NFT Collection
    pub fn initialize_fine_collection(
        ctx: Context<InitializeFineCollection>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        let collection_config = &mut ctx.accounts.collection_config;
        collection_config.authority = ctx.accounts.authority.key();
        collection_config.collection_mint = ctx.accounts.collection_mint.key();
        collection_config.name = name;
        collection_config.symbol = symbol;
        collection_config.total_minted = 0;
        collection_config.paused = false;
        collection_config.bump = *ctx.bumps.get("collection_config").unwrap();

        msg!("FINE NFT Collection initialized");
        Ok(())
    }

    /// Mint a FINE NFT
    pub fn mint_fine_nft(
        ctx: Context<MintFineNft>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        let collection_config = &mut ctx.accounts.collection_config;

        require!(!collection_config.paused, DelfineError::ContractPaused);
        require!(
            ctx.accounts.authority.key() == collection_config.authority,
            DelfineError::Unauthorized
        );

        // Mint the NFT token (1 token with 0 decimals)
        let cpi_accounts = MintTo {
            mint: ctx.accounts.nft_mint.to_account_info(),
            to: ctx.accounts.nft_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;

        collection_config.total_minted += 1;

        msg!("FINE NFT minted: {}", name);
        Ok(())
    }

    /// Start the crowdsale
    pub fn start_crowdsale(ctx: Context<ManageCrowdsale>) -> Result<()> {
        require!(
            ctx.accounts.authority.key() == ctx.accounts.config.authority,
            DelfineError::Unauthorized
        );

        let config = &mut ctx.accounts.config;
        config.crowdsale_active = true;

        msg!("Crowdsale started");
        Ok(())
    }

    /// Stop the crowdsale
    pub fn stop_crowdsale(ctx: Context<ManageCrowdsale>) -> Result<()> {
        require!(
            ctx.accounts.authority.key() == ctx.accounts.config.authority,
            DelfineError::Unauthorized
        );

        let config = &mut ctx.accounts.config;
        config.crowdsale_active = false;

        msg!("Crowdsale stopped");
        Ok(())
    }

    /// Purchase DELF tokens with SOL
    /// Rate: 1000 DELF per 1 SOL
    pub fn buy_delf_tokens(
        ctx: Context<BuyDelfTokens>,
        sol_amount: u64,
    ) -> Result<()> {
        let config = &ctx.accounts.config;

        require!(config.crowdsale_active, DelfineError::CrowdsaleNotActive);
        require!(sol_amount > 0, DelfineError::InvalidAmount);

        // Calculate DELF tokens to send (rate is 1000 DELF per 1 SOL)
        // SOL has 9 decimals, DELF has 9 decimals
        let delf_amount = sol_amount
            .checked_mul(config.token_rate)
            .ok_or(DelfineError::Overflow)?;

        // Transfer SOL from buyer to authority
        let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.buyer.key(),
            &ctx.accounts.authority.key(),
            sol_amount,
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Transfer DELF tokens from crowdsale vault to buyer
        let cpi_accounts = Transfer {
            from: ctx.accounts.crowdsale_vault.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_ctx, delf_amount)?;

        // Update stats
        let config = &mut ctx.accounts.config;
        config.total_delf_sold = config.total_delf_sold
            .checked_add(delf_amount)
            .ok_or(DelfineError::Overflow)?;

        msg!("Purchased {} DELF tokens for {} SOL", delf_amount, sol_amount);
        Ok(())
    }

    /// Burn FINE NFT tokens
    pub fn burn_fine_nft(
        ctx: Context<BurnFineNft>,
        amount: u64,
    ) -> Result<()> {
        let cpi_accounts = Burn {
            mint: ctx.accounts.nft_mint.to_account_info(),
            from: ctx.accounts.nft_token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::burn(cpi_ctx, amount)?;

        msg!("Burned {} FINE NFT token(s)", amount);
        Ok(())
    }

    /// Pause FINE NFT minting
    pub fn pause_fine_minting(ctx: Context<ManageFineCollection>) -> Result<()> {
        require!(
            ctx.accounts.authority.key() == ctx.accounts.collection_config.authority,
            DelfineError::Unauthorized
        );

        let collection_config = &mut ctx.accounts.collection_config;
        collection_config.paused = true;

        msg!("FINE minting paused");
        Ok(())
    }

    /// Unpause FINE NFT minting
    pub fn unpause_fine_minting(ctx: Context<ManageFineCollection>) -> Result<()> {
        require!(
            ctx.accounts.authority.key() == ctx.accounts.collection_config.authority,
            DelfineError::Unauthorized
        );

        let collection_config = &mut ctx.accounts.collection_config;
        collection_config.paused = false;

        msg!("FINE minting unpaused");
        Ok(())
    }

    /// Update token rate for crowdsale
    pub fn update_token_rate(
        ctx: Context<ManageCrowdsale>,
        new_rate: u64,
    ) -> Result<()> {
        require!(
            ctx.accounts.authority.key() == ctx.accounts.config.authority,
            DelfineError::Unauthorized
        );

        let config = &mut ctx.accounts.config;
        config.token_rate = new_rate;

        msg!("Token rate updated to {}", new_rate);
        Ok(())
    }
}

// ============================================================================
// ACCOUNT CONTEXTS
// ============================================================================

#[derive(Accounts)]
pub struct InitializeDelfToken<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Config::LEN,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = config,
    )]
    pub delf_mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintDelfTokens<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub delf_mint: Account<'info, Mint>,

    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct InitializeFineCollection<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + FineCollectionConfig::LEN,
        seeds = [b"fine_collection"],
        bump
    )]
    pub collection_config: Account<'info, FineCollectionConfig>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority,
    )]
    pub collection_mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintFineNft<'info> {
    #[account(
        mut,
        seeds = [b"fine_collection"],
        bump = collection_config.bump,
    )]
    pub collection_config: Account<'info, FineCollectionConfig>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = authority,
    )]
    pub nft_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = nft_mint,
        associated_token::authority = recipient,
    )]
    pub nft_token_account: Account<'info, TokenAccount>,

    /// CHECK: Recipient of the NFT
    pub recipient: AccountInfo<'info>,

    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct BuyDelfTokens<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub delf_mint: Account<'info, Mint>,

    #[account(mut)]
    pub crowdsale_vault: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = delf_mint,
        associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECK: Authority receiving SOL
    #[account(mut)]
    pub authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ManageCrowdsale<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ManageFineCollection<'info> {
    #[account(
        mut,
        seeds = [b"fine_collection"],
        bump = collection_config.bump,
    )]
    pub collection_config: Account<'info, FineCollectionConfig>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct BurnFineNft<'info> {
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,

    #[account(mut)]
    pub nft_token_account: Account<'info, TokenAccount>,

    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

// ============================================================================
// STATE ACCOUNTS
// ============================================================================

#[account]
pub struct Config {
    pub authority: Pubkey,          // 32
    pub delf_mint: Pubkey,          // 32
    pub token_rate: u64,            // 8 (tokens per SOL)
    pub crowdsale_active: bool,     // 1
    pub total_delf_sold: u64,       // 8
    pub bump: u8,                   // 1
}

impl Config {
    pub const LEN: usize = 32 + 32 + 8 + 1 + 8 + 1;
}

#[account]
pub struct FineCollectionConfig {
    pub authority: Pubkey,          // 32
    pub collection_mint: Pubkey,    // 32
    pub name: String,               // 4 + 32
    pub symbol: String,             // 4 + 10
    pub total_minted: u64,          // 8
    pub paused: bool,               // 1
    pub bump: u8,                   // 1
}

impl FineCollectionConfig {
    pub const LEN: usize = 32 + 32 + 36 + 14 + 8 + 1 + 1;
}

// ============================================================================
// ERRORS
// ============================================================================

#[error_code]
pub enum DelfineError {
    #[msg("Unauthorized access")]
    Unauthorized,

    #[msg("Crowdsale is not active")]
    CrowdsaleNotActive,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Arithmetic overflow")]
    Overflow,

    #[msg("Contract is paused")]
    ContractPaused,

    #[msg("Insufficient balance")]
    InsufficientBalance,
}
