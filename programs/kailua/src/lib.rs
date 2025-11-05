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

    pub fn new_query(ctx:Context<SubmitQueryContext>, query_id:String, agent_id:String,query_payload:String,payment:u64) -> Result<()>{
        submit_query(ctx, query_id, agent_id, query_payload, payment)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
