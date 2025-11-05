
use anchor_lang::prelude::*;
use crate::states::*;


#[event]
pub struct QuerySubmittedEvent {
    pub query_id: String,
    pub agent_id: String,
    pub query_payload: String,
    pub payment : u64,
    pub payment_status : PaymentStatus,
    pub time_stamp : u64,
}



#[event]
pub struct QueryStatusEvent{
    pub query_id: String,
    pub agent_id: String,
    pub query_payload: String,
    pub payment : u64,
    pub payment_status : PaymentStatus,
    pub time_stamp : u64,
}


#[event]
pub struct VerifyPaymentEvent{
    pub query_id: String,
    pub agent_id: String,
    pub query_payload: String,
    pub payment : u64,
    pub payment_status : PaymentStatus,
    pub time_stamp : u64,
}



