
use anchor_lang::prelude::*;




#[account]
#[derive(InitSpace)]
pub struct Query{
    #[max_len(50)]
    pub query_id: String,
    #[max_len(50)]
    pub agent_id: String,
    #[max_len(50)]
    pub query_payload: String,
    pub payment: u64,
    pub payment_status : PaymentStatus,
    pub time_stamp : u64,
}


#[derive(AnchorDeserialize,AnchorSerialize,Clone,PartialEq,Eq,InitSpace)]
pub enum PaymentStatus {
    Pending,
    Failed,
    Verified,
}