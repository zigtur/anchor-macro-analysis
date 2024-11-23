use anchor_lang::prelude::*;

declare_id!("3t3f2Hp6Mw2sFqFxJcMuHj3RSWL6X4J9cBosSydfttKn");

#[program]
pub mod anchor_macros_analysis {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
