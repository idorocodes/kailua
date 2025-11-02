use anchor_lang::prelude::*;
pub mod events;
pub use events::*;
pub mod states;
pub use states::*;
pub mod instructions;
pub use instructions::*;
pub mod constants;
pub use constants::*;
pub mod error;
pub use error::*;


declare_id!("2vQRNXYKgupCSi5448ray5LAkMTwSP6a2D9cwn5hyGGB");

#[program]
pub mod kailua {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
