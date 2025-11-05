use crate::constants::*;
use crate::error::Errors;
use crate::events::*;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn getquery_status(
    ctx: Context<GetQueryStatusContext>,
    query_id: String,
    agent_id: String,
) -> Result<()> {
    require!(query_id.len() <= MAX_QUERYID_LEN, Errors::QueryIdMaxLen);
    require!(agent_id.len() <= MAX_AGENTID_LEN, Errors::AgentIdMaxLen);

    let query = &ctx.accounts.query;

    emit!(QueryStatusEvent {
        query_id: query.query_id.clone(),
        agent_id: query.agent_id.clone(),
        query_payload: query.query_payload.clone(),
        payment: query.payment,
        payment_status: query.payment_status.clone(),
        time_stamp: query.time_stamp,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(agent_id:String,query_id:String)]
pub struct GetQueryStatusContext<'info> {
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"query",agent_id.as_bytes(),query_id.as_bytes()],
        bump
    )]
    pub query: Account<'info, Query>,

    pub system_program: Program<'info, System>,
}
