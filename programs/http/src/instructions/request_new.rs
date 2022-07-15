use {
    crate::state::{
        Api, Config, HttpMethod, Request, RequestAccount, SEED_API, SEED_CONFIG, SEED_REQUEST,
    },
    anchor_lang::{
        prelude::*,
        solana_program::{system_program, sysvar},
        system_program::{transfer, Transfer},
    },
    cronos_pool::state::Pool,
    std::{collections::HashMap, mem::size_of},
};

#[derive(Accounts)]
#[instruction(
    id: String, 
    method: HttpMethod, 
    route: String
)]
pub struct RequestNew<'info> {
    #[account(
        seeds = [
            SEED_API,
            api.base_url.as_bytes().as_ref(),
            api.owner.as_ref()
        ],
        bump,
    )]
    pub api: Account<'info, Api>,

    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        seeds = [
            SEED_REQUEST,
            api.key().as_ref(),
            id.as_bytes().as_ref(),
            payer.key().as_ref(),
        ],
        bump,
        space = 8 + size_of::<Request>(),
        payer = payer
    )]
    pub request: Account<'info, Request>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(
    ctx: Context<RequestNew>,
    id: String,
    method: HttpMethod,
    route: String,
) -> Result<()> {
    // Fetch accounts
    let api = &ctx.accounts.api;
    let clock = &ctx.accounts.clock;
    let config = &ctx.accounts.config;
    let payer = &mut ctx.accounts.payer;
    let pool = &ctx.accounts.pool;
    let request = &mut ctx.accounts.request;
    let system_program = &ctx.accounts.system_program;

    // TODO Validate route is a relative path

    // Initialize the request account
    let created_at = clock.slot;
    let fee_amount = config.request_fee;
    let timeout_fee_amount = config.timeout_fee;
    let headers = HashMap::new(); // TODO Get headers from ix data
    let workers = pool
        .clone()
        .into_inner()
        .delegates
        .iter()
        .map(|k| *k)
        .collect::<Vec<Pubkey>>();
    request.new(
        api,
        created_at,
        fee_amount,
        headers,
        id,
        method,
        payer.key(),
        route,
        timeout_fee_amount,
        workers,
    )?;

    // Transfer fees into request account to hold in escrow
    // let net_fee_amount = fee_amount.checked_add(timeout_fee_amount).unwrap();
    let net_fee_amount = fee_amount;
    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: payer.to_account_info(),
                to: request.to_account_info(),
            },
        ),
        net_fee_amount,
    )?;

    Ok(())
}
