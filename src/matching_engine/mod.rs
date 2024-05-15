use std::collections::BinaryHeap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::orderbook::Order;
use crate::orderbook::OrderType;

// Define a struct for the matching engine
pub struct MatchingEngine {
    buy_orders: BinaryHeap<Order>,  // Max heap for buy orders
    sell_orders: BinaryHeap<Order>, // Min heap for sell orders
    next_order_id: u64,
}

impl MatchingEngine {
    // Create a new matching engine instance
    pub fn new() -> Self {
        MatchingEngine {
            buy_orders: BinaryHeap::new(),
            sell_orders: BinaryHeap::new(),
            next_order_id: 1,
        }
    }

    // Add a new order to the matching engine
    pub fn add_order(&mut self, price: f64, quantity: u64, order_type: OrderType) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let order = Order {
            id: self.next_order_id,
            price,
            quantity,
            order_type,
            timestamp,
        };
        self.next_order_id += 1;

        match order.order_type {
            OrderType::Buy => {
                self.buy_orders.push(order);
            },
            OrderType::Sell => {
                self.sell_orders.push(order);
            },
        }
    }

    // Match buy and sell orders and execute trades
    pub fn match_orders(&mut self) {
        if (self.buy_orders.is_empty() || self.sell_orders.is_empty()) {
            println!("no opposit order exists!");
            return;
        }
        while let (Some(mut buy_order), Some(mut sell_order)) = (self.buy_orders.pop(), self.sell_orders.pop()) {
            if buy_order.price >= sell_order.price {
                let matched_quantity = buy_order.quantity.min(sell_order.quantity);
                println!("Trade: Buy Order {:?} <-> Sell Order {:?}", buy_order.id, sell_order.id);
                buy_order.quantity -= matched_quantity;
                sell_order.quantity -= matched_quantity;

                if buy_order.quantity > 0 {
                    self.buy_orders.push(buy_order);
                }
                if sell_order.quantity > 0 {
                    self.sell_orders.push(sell_order);
                }
            } else {
                self.buy_orders.push(buy_order);
                self.sell_orders.push(sell_order);
                break; // No more matching orders
            }
        }
    }

    // Print the order book
    pub fn print_order_book(&self) {
        println!("{}", "-".to_string().repeat(100));
        println!("Buy Orders: {:?}", self.buy_orders);
        // for order in &self.buy_orders.clone().into_sorted_vec() {
        //     println!("Price: {:.2}, Size: {:?}", order.price, order.quantity);
        // }
        println!("{}", "-".to_string().repeat(100));
        println!("Sell Orders: {:?}", self.sell_orders);
        // for order in &self.sell_orders.clone().into_sorted_vec() {
        //     println!("Price: {:.2}, Size: {:?}", order.price, order.quantity);
        // }
        println!("{}", "-".to_string().repeat(100));
    }
}
