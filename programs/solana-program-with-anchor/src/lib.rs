use anchor_lang::prelude::*;

declare_id!("7MtMDbYQxhjkqWPoZT1szVSeWwaGUMUJzYnHMs1jD8Wq");

#[program]
pub mod solana_program_with_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
