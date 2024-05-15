use inquire::{Select, Text};
use std::{process, str::FromStr};

mod matching_engine;
mod orderbook;

use matching_engine::MatchingEngine;
use orderbook::OrderType;

fn user_input<T: FromStr>(msg: &str, choices: Option<Vec<String>>) -> Result<T, <T as FromStr>::Err> {
    let ans = match choices.clone() {
        Some(ch) => {
            Select::new(
                msg,
                ch,
            ).prompt()
        },
        None => {
            Text::new(msg).prompt()
        },
    };

    // Match on the result of parsing
    match ans {
        Ok(input) => {
            let casted_input: &str = &input;
            casted_input.parse()
        },
        Err(_) => {
            println!("Please enter a valid input.");
            user_input::<T>(msg, choices)
        },
    }
}

fn ask_for_action(orderbook: &mut MatchingEngine) {
    let create_order = "create order".to_string();
    let remove_order = "remove order".to_string();
    let print_orderbook = "Print Orderbook".to_string();
    let exit = "Exit".to_string();
    let action_choices: Vec<String> = vec![
        create_order.clone(),
        remove_order.clone(),
        print_orderbook.clone(),
        exit.clone(),
    ];

    let ans = user_input::<String>(
        "Hey, Please let us know what do you want to do?",
        Some(action_choices),
    );

    let choice = match ans {
        Ok(ch) => Some(ch),
        Err(_) => {
            println!("Invalid choice!");
            return;
        },
    };

    match choice {
        Some(ch) if ch == create_order => {
            let buy = "Buy".to_string();
            let sell = "Sell".to_string();
            let price = user_input::<f64>(
                "Enter Order Price:",
                None,
            ).unwrap();

            let quantity = user_input::<u64>(
                "Enter Order Quantity:",
                None,
            ).unwrap();

            let ot = user_input::<String>(
                "Select Order Type:",
                Some(vec![buy.clone(), sell.clone()]),
            ).unwrap();

            let order_type = match ot == sell {
                true => OrderType::Sell,
                _ => OrderType::Buy
            };
            orderbook.add_order(price, quantity, order_type);
            orderbook.match_orders();
            orderbook.print_order_book();
            ask_for_action(orderbook)
        },
        Some(ch) if ch == remove_order => {
            println!("Not implemented yet!");
            ask_for_action(orderbook)
        },
        Some(ch) if ch == print_orderbook => {
            orderbook.print_order_book();
            ask_for_action(orderbook)
        },
        Some(ch) if ch == exit => process::exit(1),
        _ => println!("invalid choice!"),
    }
}

fn main() {
    // Create a new matching engine instance
    let mut matching_engine = MatchingEngine::new();
    ask_for_action(&mut matching_engine);

    // // Add some buy and sell orders
    // matching_engine.add_order(103.0, 10, OrderType::Buy);
    // matching_engine.add_order(101.0, 15, OrderType::Sell);
    // matching_engine.add_order(100.0, 20, OrderType::Buy);
    // matching_engine.add_order(102.0, 10, OrderType::Sell);

    // // Match orders and execute trades
    // matching_engine.match_orders();
}
