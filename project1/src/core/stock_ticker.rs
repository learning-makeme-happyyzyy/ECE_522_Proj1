// Task1: Parse input and handle the stock ticker
use yahoo_finance_api::{self as yahoo, Quote};
use tokio_test;
use chrono::*;

pub fn get_by_name(name: &String) -> Vec<Quote>{
    // Handle invalid input
    if name.chars().all(|c| !c.is_alphabetic()) {
        println!("Invalid input");
        return vec![];
    }

    let provider = yahoo::YahooConnector::new().unwrap();

    // Handle missing symbol
    let resp = tokio_test::block_on(provider.search_ticker(name)).unwrap();
    if resp.quotes.len() == 0 {
        println!("Symbol not found");
        return vec![];
    }

    // returns historic quotes coverd 6 months with daily interval
    let resp = tokio_test::block_on(provider.get_quote_range(name, "1d", "6mo")).unwrap();
    
    resp.quotes().unwrap()
}

pub fn functional_test() {
    let quotes = get_by_name(&"AAPL".to_string());

    for q in quotes {
        let local_datetime: DateTime<Local> = DateTime::from_timestamp(q.timestamp as i64, 0).unwrap().into(); // 使用 into() 转为 Local

        println!("{:?}", local_datetime.format("%Y-%m-%d %H:%M:%S"));
    }
}