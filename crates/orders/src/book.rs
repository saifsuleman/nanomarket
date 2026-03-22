use events::types::{Price, Quantity};
use crate::{order::Order, trade::MatchResult};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct PriceLevel {
    orders: Vec<Order>
}

impl PriceLevel {
    fn new() -> Self {
        Self { orders: Vec::new() }
    }

    fn push(&mut self, order: Order) {
        self.orders.push(order);
    }

    fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    fn total_quantity(&self) -> Quantity {
        self.orders.iter().map(|o| o.remaining).sum()
    }

    fn order_count(&self) -> usize {
        self.orders.len()
    }
}

#[derive(Debug, Clone)]
pub struct LevelSummary {
    pub price: Price,
    pub total_quantity: Quantity,
    pub order_count: usize,
}

#[derive(Debug, Clone)]
pub struct OrderBook {
    bids: BTreeMap<Price, PriceLevel>,
    asks: BTreeMap<Price, PriceLevel>,
}


impl OrderBook {
    pub fn best_bid(&self) -> Option<Price> {
        self.bids.keys().next_back().copied()
    }

    pub fn best_ask(&self) -> Option<Price> {
        self.asks.keys().next().copied()
    }

    pub fn spread(&self) -> Option<i64> {
        match (self.best_ask(), self.best_bid()) {
            (Some(a), Some(b)) => Some(a as i64 - b as i64),
            _ => None
        }
    }

    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_ask(), self.best_bid()) {
            (Some(a), Some(b)) => Some((a as f64 + b as f64) / 2.0),
            _ => None
        }
    }

    pub fn submit(&mut self, mut order: Order) -> MatchResult {
        todo!()
    }
}
