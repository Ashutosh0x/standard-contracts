use anchor_lang::prelude::*;

declare_id!("7bwYem3NvksZcsmgBLtNbQLkS1p35ahHa7JsssKeQ8UT");

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
