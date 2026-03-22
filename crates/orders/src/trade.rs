use events::{lamport::LamportTimestamp, types::{OrderId, Price, Quantity, TradeId}};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fill {
    pub bid_order_id: OrderId,
    pub ask_order_id: OrderId,
    pub price: Price,
    pub quantity: Quantity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trade {
    pub id: TradeId,
    pub bid_order_id: OrderId,
    pub ask_order_id: OrderId,
    pub price: Price,
    pub quantity: Quantity,
    pub timestamp: LamportTimestamp,
}

impl fmt::Display for Trade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Trade#{} [bid={} ask={} px={} qty={} {}]",
            self.id, self.bid_order_id, self.ask_order_id, self.price, self.quantity, self.timestamp
        )
    }
}

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub fills: Vec<Fill>,
    pub resting: Option<OrderId>,
}
