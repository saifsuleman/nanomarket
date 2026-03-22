use events::{lamport::LamportTimestamp, types::{OrderId, OrderType, Price, Quantity, Side}};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderId,
    pub side: Side,
    pub order_type: OrderType,
    pub price: Price,
    pub quantity: Quantity,
    pub remaining: Quantity,
    pub timestamp: LamportTimestamp
}

impl Order {
    pub fn new(
        id: OrderId,
        side: Side,
        order_type: OrderType,
        price: Price,
        quantity: Quantity,
        timestamp: LamportTimestamp,
    ) -> Self {
        Self {
            id,
            side,
            order_type,
            price,
            quantity,
            remaining: quantity,
            timestamp,
        }
    }

    pub fn is_filled(&self) -> bool {
        self.remaining == 0
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Order#{} {} {} px={} qty={}/{} {}",
            self.id, self.side, self.order_type, self.price, self.remaining, self.quantity, self.timestamp
        )
    }
}
