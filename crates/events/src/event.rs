use crate::{
    lamport::LamportTimestamp,
    types::{OrderId, OrderType, Price, Quantity, Side, TradeId},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    OrderPlaced {
        order_id: OrderId,
        side: Side,
        order_type: OrderType,
        price: Price,
        quantity: Quantity,
        timestamp: LamportTimestamp,
    },
    OrderFilled {
        trade_id: TradeId,
        bid_order_id: OrderId,
        ask_order_id: OrderId,
        price: Price,
        quantity: Quantity,
        timestamp: LamportTimestamp,
    },
    OrderCancelled {
        order_id: OrderId,
        timestamp: LamportTimestamp,
    },
}

impl Event {
    pub fn timestamp(&self) -> LamportTimestamp {
        match self {
            Event::OrderPlaced { timestamp, .. }
            | Event::OrderFilled { timestamp, .. }
            | Event::OrderCancelled { timestamp, .. } => *timestamp,
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::OrderPlaced {
                order_id,
                side,
                order_type,
                price,
                quantity,
                timestamp,
            } => write!(
                f,
                "[{}] OrderPlaced  id={} {} {} px={} qty={}",
                timestamp, order_id, side, order_type, price, quantity
            ),
            Event::OrderFilled {
                trade_id,
                bid_order_id,
                ask_order_id,
                price,
                quantity,
                timestamp,
            } => write!(
                f,
                "[{}] OrderFilled  trade={} bid={} ask={} px={} qty={}",
                timestamp, trade_id, bid_order_id, ask_order_id, price, quantity
            ),
            Event::OrderCancelled {
                order_id,
                timestamp,
            } => write!(f, "[{}] OrderCancelled id={}", timestamp, order_id),
        }
    }
}
