use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::Errors;
use crate::events::*;
use crate::states::*;

pub fn submit_query(
    ctx: Context<SubmitQueryContext>,
    query_id: String,
    agent_id: String,
    query_payload: String,
    payment: u64,
) -> Result<()> {
    require!(query_id.len() <= MAX_QUERYID_LEN, Errors::QueryIdMaxLen);
    require!(agent_id.len() <= MAX_AGENTID_LEN, Errors::AgentIdMaxLen);

    let query_account = &mut ctx.accounts.query;

    query_account.agent_id = agent_id;
    query_account.query_id = query_id;
    query_account.query_payload = query_payload;
    query_account.time_stamp = Clock::get()?.unix_timestamp as u64;
    query_account.payment = payment;
    query_account.payment_status = PaymentStatus::Pending;

    emit!(QuerySubmittedEvent {
        query_id: query_account.query_id.clone(),
        query_payload: query_account.query_payload.clone(),
        agent_id: query_account.agent_id.clone(),
        payment: query_account.payment,
        payment_status: query_account.payment_status.clone(),
        time_stamp: query_account.time_stamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct SubmitQueryContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init,
       space = ANCHOR_DISCRIMINATOR + Query::INIT_SPACE,
       payer = signer
        )]
    pub query: Account<'info, Query>,

    pub system_program: Program<'info, System>,
}
