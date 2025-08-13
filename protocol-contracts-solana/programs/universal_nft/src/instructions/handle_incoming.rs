﻿use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::nft_origin::{NftOrigin, CrossChainNftPayload};
use crate::state::gateway::GatewayConfig;
use crate::state::replay::ReplayMarker;
use crate::utils::{derive_nft_origin_pda, derive_replay_marker_pda};

#[derive(Accounts)]
pub struct HandleIncoming<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub recipient: SystemAccount<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer,
        mint::freeze_authority = payer,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = recipient
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,
    /// CHECK: gateway program config PDA
    pub gateway_config: UncheckedAccount<'info>,
    /// CHECK: replay marker account
    #[account(mut)]
    pub replay_marker: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<HandleIncoming>, payload: Vec<u8>) -> Result<()> {
    let clock = Clock::get()?;

    // Load gateway config from PDA and verify signer program id from payload origin (out-of-band)
    let (cfg_pda, _bump) = Pubkey::find_program_address(&[GatewayConfig::SEED], &crate::ID);
    require_keys_eq!(ctx.accounts.gateway_config.key(), cfg_pda, ErrorCode::UnauthorizedGateway);
    let data = ctx.accounts.gateway_config.try_borrow_data()?;
    let cfg = GatewayConfig::try_from_slice(&data[8..]).map_err(|_| ErrorCode::UnauthorizedGateway)?;

    // Deserialize payload
    let p: CrossChainNftPayload = CrossChainNftPayload::try_from_slice(&payload)
        .map_err(|_| ErrorCode::InvalidPayload)?;

    // Replay protection: derive and ensure empty
    let (replay_pda, bump) = derive_replay_marker_pda(&p.token_id, p.nonce);
    require_keys_eq!(ctx.accounts.replay_marker.key(), replay_pda, ErrorCode::ReplayPdaMismatch);
    if !ctx.accounts.replay_marker.data_is_empty() {
        return Err(ErrorCode::ReplayAttack.into());
    }
    let space = 8 + ReplayMarker::LEN;
    let lamports = Rent::get()?.minimum_balance(space);
    anchor_lang::solana_program::program::invoke_signed(
        &anchor_lang::solana_program::system_instruction::create_account(
            &ctx.accounts.payer.key(),
            &replay_pda,
            lamports,
            space as u64,
            &crate::ID,
        ),
        &[
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.replay_marker.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[&[ReplayMarker::SEED, &p.token_id, &p.nonce.to_le_bytes(), &[bump]]],
    )?;

    // Write replay marker
    let marker = ReplayMarker {
        token_id: p.token_id,
        nonce: p.nonce,
        created_at: clock.unix_timestamp,
        bump,
    };
    let mut data = ctx.accounts.replay_marker.try_borrow_mut_data()?;
    marker.try_serialize(&mut &mut data[..])?;

    // Mint 1 token to recipient
    anchor_spl::token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        ),
        1,
    )?;

    // Create or update nft_origin PDA
    let (nft_origin_pda, nft_origin_bump) = derive_nft_origin_pda(&p.token_id);
    
    let nft_origin = NftOrigin {
        origin_chain: p.origin_chain_id,
        origin_token_id: p.token_id,
        origin_mint: p.origin_mint,
        metadata_uri: p.metadata_uri,
        created_at: clock.unix_timestamp,
        bump: nft_origin_bump,
    };

    // Create the nft_origin account if it doesn't exist
    if ctx.accounts.payer.key() != &nft_origin_pda {
        anchor_lang::solana_program::program::invoke_signed(
            &anchor_lang::solana_program::system_instruction::create_account(
                &ctx.accounts.payer.key(),
                &nft_origin_pda,
                Rent::get()?.minimum_balance(NftOrigin::LEN),
                NftOrigin::LEN as u64,
                &crate::ID,
            ),
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&[NftOrigin::SEED, &p.token_id, &[nft_origin_bump]]],
        )?;
    }

    // Initialize the nft_origin account with data
    let mut nft_origin_account = anchor_lang::solana_program::account_info::AccountInfo::try_from(&nft_origin_pda)?;
    let mut data = nft_origin_account.try_borrow_mut_data()?;
    nft_origin.try_serialize(&mut &mut data[..])?;

    // Emit cross-chain mint event
    emit!(CrossChainMintEvent {
        token_id: p.token_id,
        origin_chain: p.origin_chain_id,
        recipient: p.recipient,
        nonce: p.nonce,
        timestamp: clock.unix_timestamp,
    });

    msg!("Minted Universal NFT from cross-chain transfer");
    msg!("Token ID: {}", hex::encode(&p.token_id[..8]));
    msg!("Origin Chain: {}", p.origin_chain_id);
    msg!("Recipient: {}", p.recipient);

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct CrossChainMintEvent {
    pub token_id: [u8; 32],
    pub origin_chain: u16,
    pub recipient: Pubkey,
    pub nonce: u64,
    pub timestamp: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized gateway")]
    UnauthorizedGateway,
    #[msg("Invalid payload")]
    InvalidPayload,
    #[msg("Replay attack detected")]
    ReplayAttack,
    #[msg("Replay PDA mismatch")]
    ReplayPdaMismatch,
}
