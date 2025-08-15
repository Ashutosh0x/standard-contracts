use anchor_lang::prelude::*;

declare_id!("FXFjiHkZLqR9TWdGRcYAZPvFZLSXNrfKD3rwPTPoB8Xe");

#[program]
pub mod universal_nft {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Initialized!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
