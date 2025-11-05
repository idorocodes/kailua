use crate::constants::*;
use crate::error::Errors;
use crate::events::*;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn verify_payments(
    ctx: Context<VerifyPaymentsContext>,
    query_id: String,
    agent_id: String,
) -> Result<()> {
    require!(query_id.len() <= MAX_QUERYID_LEN, Errors::QueryIdMaxLen);
    require!(agent_id.len() <= MAX_AGENTID_LEN, Errors::AgentIdMaxLen);

    let query = &mut ctx.accounts.query;

    require!(
        query.payment_status == PaymentStatus::Pending,
        Errors::PaymentNotPending
    );

    query.payment_status = PaymentStatus::Verified;

    emit!(VerifyPaymentEvent {
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
pub struct VerifyPaymentsContext<'info> {
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"query",agent_id.as_bytes(),query_id.as_bytes()],
        bump
    )]
    pub query: Account<'info, Query>,

    pub system_program: Program<'info, System>,
}
