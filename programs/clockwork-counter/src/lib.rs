use anchor_lang::prelude::*;

declare_id!("Fit6msXwWsyuNncCpPt93snET1eEGh1AY9ZcZvAyi2Jk");

#[program]
pub mod clockwork_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
