use anchor_lang::prelude::*;

declare_id!("8NWgWiLoXdERkez5eb5zBjDSTu9dSHrbLyTx4tKnJGzy");

#[program]
pub mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter Account Created");
        msg!("Current Count: { }", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("Counter updated");
        msg!("Current count {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // Need a pda to store count
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, CounterData>,
    // signer
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, CounterData>,
    pub user: Signer<'info>,
}

#[account]
pub struct CounterData {
    pub count: u64,
}
