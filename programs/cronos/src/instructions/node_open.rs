
use {
    crate::state::*, 
    anchor_lang::prelude::*, 
    solana_program::system_program, 
    std::mem::size_of
};

#[derive(Accounts)]
#[instruction(bot_bump: u8)]
pub struct BotRegister<'info> {
    #[account(
        init,
        seeds = [
            SEED_DAEMON, 
            owner.key().as_ref()
        ],
        bump,
        payer = owner,
        space = 8 + size_of::<Daemon>(),
    )]
    pub node: Account<'info, Daemon>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<BotRegister>, bot_bump: u8) -> Result<()> {
    let daemon = &mut ctx.accounts.daemon;
    let fee = &mut ctx.accounts.fee;
    let owner = &ctx.accounts.owner;

    daemon.open(owner.key(), bot_bump)?;
    fee.open(daemon.key(), fee_bump)
}