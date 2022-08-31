use {
    anchor_lang::{prelude::*, solana_program::program::invoke, system_program},
    anchor_spl::{associated_token, token},
    mpl_token_metadata::{instruction as token_instruction, ID as TOKEN_METADATA_ID},
};

declare_id!("55ibTnYbkZZPhuSxPtsG2v1t76gAjHxDxLbj3q6EikPo");

#[program]
pub mod mint_nft {
    use super::*;

    pub fn mint(
        _ctx: Context<MintNft>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        msg!("Creating mint account...");
        msg!("Mint: {}", &_ctx.accounts.mint.key());
        system_program::create_account(
            CpiContext::new(
                _ctx.accounts.token_program.to_account_info(),
                system_program::CreateAccount {
                    from: _ctx.accounts.mint_authority.to_account_info(),
                    to: _ctx.accounts.mint.to_account_info(),
                },
            ),
            10000000,
            82,
            &_ctx.accounts.token_program.key(),
        )?;

        msg!("Initializing mint account...");
        msg!("Mint: {}", &_ctx.accounts.mint.key());
        token::initialize_mint(
            CpiContext::new(
                _ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: _ctx.accounts.mint.to_account_info(),
                    rent: _ctx.accounts.rent.to_account_info(),
                },
            ),
            0,
            &_ctx.accounts.mint_authority.key(),
            Some(&_ctx.accounts.mint_authority.key()),
        )?;

        msg!("Creating token account...");
        msg!("Token Address: {}", &_ctx.accounts.token_account.key());
        associated_token::create(CpiContext::new(
            _ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: _ctx.accounts.mint_authority.to_account_info(),
                associated_token: _ctx.accounts.token_account.to_account_info(),
                authority: _ctx.accounts.mint_authority.to_account_info(),
                mint: _ctx.accounts.mint.to_account_info(),
                system_program: _ctx.accounts.system_program.to_account_info(),
                token_program: _ctx.accounts.token_program.to_account_info(),
                rent: _ctx.accounts.rent.to_account_info(),
            },
        ))?;

        msg!("Minting token to token account...");
        msg!("Mint: {}", &_ctx.accounts.mint.to_account_info().key());
        msg!("Token Address: {}", &_ctx.accounts.token_account.key());
        token::mint_to(
            CpiContext::new(
                _ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: _ctx.accounts.mint.to_account_info(),
                    to: _ctx.accounts.token_account.to_account_info(),
                    authority: _ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            1,
        )?;

        msg!("Creating metadata account...");
        msg!(
            "Metadata account address: {}",
            &_ctx.accounts.metadata.to_account_info().key()
        );
        invoke(
            &token_instruction::create_metadata_accounts_v2(
                TOKEN_METADATA_ID,
                _ctx.accounts.metadata.key(),
                _ctx.accounts.mint.key(),
                _ctx.accounts.mint_authority.key(),
                _ctx.accounts.mint_authority.key(),
                _ctx.accounts.mint_authority.key(),
                metadata_title,
                metadata_symbol,
                metadata_uri,
                None,
                1,
                true,
                false,
                None,
                None,
            ),
            &[
                _ctx.accounts.metadata.to_account_info(),
                _ctx.accounts.mint.to_account_info(),
                _ctx.accounts.token_account.to_account_info(),
                _ctx.accounts.mint_authority.to_account_info(),
                _ctx.accounts.rent.to_account_info(),
            ],
        )?;

        msg!("Creating master edition metadata account...");
        msg!(
            "Master edition metadata account address: {}",
            &_ctx.accounts.master_edition.to_account_info().key()
        );
        invoke(
            &token_instruction::create_master_edition_v3(
                TOKEN_METADATA_ID,
                _ctx.accounts.master_edition.key(),
                _ctx.accounts.mint.key(),
                _ctx.accounts.mint_authority.key(),
                _ctx.accounts.mint_authority.key(),
                _ctx.accounts.metadata.key(),
                _ctx.accounts.mint_authority.key(),
                Some(0),
            ),
            &[
                _ctx.accounts.master_edition.to_account_info(),
                _ctx.accounts.metadata.to_account_info(),
                _ctx.accounts.mint.to_account_info(),
                _ctx.accounts.token_account.to_account_info(),
                _ctx.accounts.mint_authority.to_account_info(),
                _ctx.accounts.rent.to_account_info(),
            ],
        )?;

        msg!("Token mint process completed successfully.");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    // metadata with the info about the NFT
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    // owner to edit
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    // Mint pub key
    #[account(mut)]
    pub mint: Signer<'info>,

    //User Token Account
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    // The owner of the NFT
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    // Rent var
    pub rent: Sysvar<'info, Rent>,

    // System Program
    pub system_program: Program<'info, System>,

    // Info with the token program
    pub token_program: Program<'info, token::Token>,

    // Associated account of the token
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    // Metadata program.
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
}
