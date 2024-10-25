use yahoo_finance_api::{self as yahoo, Quote};
use tokio_test;
use chrono::*;
mod chart;

pub fn get_by_symbol(name: &String) -> Vec<Quote> {
    // Handle invalid input
    if name.chars().all(|c| !c.is_alphabetic()) {
        println!("Invalid input");
        return vec![];
    }

    let provider = yahoo::YahooConnector::new().unwrap();

    // Handle missing symbol
    let resp = tokio_test::block_on(provider.search_ticker(name));
    match resp {
        Ok(quotes) => {
            if quotes.quotes.len() == 0 {
                println!("Symbol not found");
                return vec![];
            }
        }
        Err(e) => {
            eprintln!("Failed to retrieve quotes: {:?}", e);
            return vec![];
        }
    }

    // Returns historic quotes covered 6 months with daily interval
    let resp = tokio_test::block_on(provider.get_quote_range(name, "1d", "6mo"));
    match resp {
        Ok(quotes) => quotes.quotes().unwrap(),
        Err(e) => {
            eprintln!("Failed to retrieve quotes: {:?}", e);
            vec![]
        }
    }
}

fn vec_quotes_to_closing_prices(quotes: &Vec<Quote>) -> Vec<(chrono::NaiveDateTime, f64)> {
    let closing_prices: Vec<(chrono::NaiveDateTime, f64)> = quotes
        .iter()
        .map(|quote| {
            let datetime = chrono::NaiveDateTime::from_timestamp(quote.timestamp as i64, 0);
            (datetime, quote.close)
        })
        .collect();

    closing_prices
}

pub fn find_volatile_days(quotes: &Vec<Quote>) -> Vec<String> {
    let closing_prices = vec_quotes_to_closing_prices(quotes);

    let mut volatile_days = Vec::new();

    for window in closing_prices.windows(2) {
        let (_, price1) = window[0];
        let (date2, price2) = window[1];
        let variation = ((price2 - price1) / price1).abs();

        if variation > 0.02 {
            volatile_days.push(date2.format("%Y-%m-%d %H:%M:%S").to_string());
        }
    }

    // println!("Volatile days: {:?}", volatile_days);

    volatile_days
}

pub fn find_min_max_closing_prices(quotes: &Vec<Quote>) -> (f64, NaiveDateTime, f64, NaiveDateTime) {
    let closing_prices = vec_quotes_to_closing_prices(quotes);

    let (mut minp, mut mind, mut maxp, mut maxd) = (0.0, NaiveDateTime::from_timestamp(0, 0), 0.0, NaiveDateTime::from_timestamp(0, 0));
    if let Some((min_date, min_price)) = closing_prices.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()) {
        minp = min_price.clone();
        mind = min_date.clone();
        println!("Minimum price: {} on {}", min_price, min_date);
    }

    if let Some((max_date, max_price)) = closing_prices.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()) {
        maxp = max_price.clone();
        maxd = max_date.clone();
        println!("Maximum price: {} on {}", max_price, max_date);
    }

    (minp, mind, maxp, maxd)
}

pub fn functional_test(symbol: &String) {
    let quotes = get_by_symbol(&symbol.to_string());
    
    if quotes.is_empty() {
        println!("Stock symbol not exist: {}, please input a correct stock symbol", symbol);
        return; // no data
    }

    for q in &quotes {
        let local_datetime: DateTime<Local> = DateTime::from_timestamp(q.timestamp as i64, 0).unwrap().into();

        // println!("{:?}", local_datetime.format("%Y-%m-%d %H:%M:%S"));
        // println!("{:?}", q);
    }

    let volatile_days = find_volatile_days(&quotes);
    find_min_max_closing_prices(&quotes);
    chart::print_closing_prices_and_dates(&quotes, &volatile_days, symbol);
}
