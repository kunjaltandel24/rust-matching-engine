mod matching_engine;
mod orderbook;

use matching_engine::MatchingEngine;
use orderbook::OrderType;

fn main() {
    // Create a new matching engine instance
    let mut matching_engine = MatchingEngine::new();

    // Add some buy and sell orders
    matching_engine.add_order(103.0, 10, OrderType::Buy);
    matching_engine.add_order(101.0, 15, OrderType::Sell);
    matching_engine.add_order(100.0, 20, OrderType::Buy);
    matching_engine.add_order(102.0, 10, OrderType::Sell);

    // Match orders and execute trades
    matching_engine.match_orders();
}
