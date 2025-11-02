use anchor_lang::prelude::*;


#[error_code]
pub enum  Errors{
    #[msg("Query id has exceeded required length")]
    QueryIdMaxLen,
    #[msg("Agent id has exceeded required length")]
    AgentIdMaxLen,
    #[msg("Payment amount not supplied")]
    PayementNotSupplied,
    #[msg("Payment payload is not supplied")]
    PaymentPayloadNotSupplied,
}