use yahoo_finance_api::Quote;
use chrono::NaiveDateTime;
use plotters::prelude::*;

pub fn print_closing_prices_and_dates(quotes: &Vec<Quote>, volatile_days: &Vec<String>, stock_symbol:&str) -> Result<(), Box<dyn std::error::Error>> {
    let closing_prices: Vec<f64> = quotes.iter().map(|q| q.close).collect();
    let high: Vec<f64> = quotes.iter().map(|q| q.high).collect();
    let low: Vec<f64> = quotes.iter().map(|q| q.low).collect();
    let dates: Vec<String> = quotes.iter()
        .map(|q| NaiveDateTime::from_timestamp(q.timestamp as i64, 0).to_string())
        .collect();
    
    let positions: Vec<usize> = volatile_days.iter()
        .filter_map(|day| dates.iter().position(|d| d == day))
        .collect();
    
    // println!("{:?}", positions);
    
    let min_price = closing_prices.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_price = closing_prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    // Convert Vec<String> to Vec<&str>
    let date_strs: Vec<&str> = dates.iter().map(|s| s.as_str()).collect();

    // println!("Closing prices: {:?}", closing_prices);
    // println!("Dates: {:?}", dates);
    // println!("Volatile days: {:?}", volatile_days);
    // println!("High: {:?}", high);
    // println!("Low: {:?}", low);

    // let ymax = closing_prices.iter().cloned().fold(f64::NAN, f64::max);

    plot(&date_strs, &closing_prices, &high, &low, &positions, min_price, max_price, stock_symbol)?;
    Ok(())
}

fn plot(dates: &[&str], closing_prices: &[f64], high: &[f64], low: &[f64], positions: &[usize], min_price:f64, max_price:f64, stock_symbol:&str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plot.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let ymin = closing_prices.iter().cloned().fold(f64::INFINITY, f64::min);
    let ymax = closing_prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("{} Close Price", stock_symbol), ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..dates.len(), ymin..ymax)?;

    chart.configure_mesh()
        .x_labels(10)
        .x_label_formatter(&|x| dates[*x as usize].to_string())
        .draw()?;

    chart.draw_series(LineSeries::new(
        (0..).zip(closing_prices.iter()).map(|(x, y)| (x, *y)),
        &RED,
    ))?;

    //Annotate minimum
    if let Some(min_index) = closing_prices.iter().position(|&p| p == min_price) {
        chart.draw_series(std::iter::once(Text::new(
            format!("Min: {:.2}", min_price),
            (min_index, min_price),
            ("sans-serif", 15).into_font(),
        )))?;
    }

    // Annotate maximum
    if let Some(max_index) = closing_prices.iter().position(|&p| p == max_price) {
        chart.draw_series(std::iter::once(Text::new(
            format!("Max: {:.2}", max_price),
            (max_index, max_price),
            ("sans-serif", 15).into_font(),
        )))?;
    }

    for &pos in positions {
        // Draw vertical line for error bar
        chart.draw_series(LineSeries::new(
            vec![(pos, high[pos]), (pos, low[pos])],
            &BLUE,
        ))?;

        // Draw horizontal lines at high and low points for error bar caps
        chart.draw_series(LineSeries::new(
            vec![(pos.saturating_sub(1), high[pos]), (pos.saturating_add(1), high[pos])],
            &BLUE,
        ))?;

        chart.draw_series(LineSeries::new(
            vec![(pos.saturating_sub(1), low[pos]), (pos.saturating_add(1), low[pos])],
            &BLUE,
        ))?;

        // Mark closing prices with blue circles
        chart.draw_series(PointSeries::of_element(
            vec![(pos, closing_prices[pos])],
            5,
            &BLUE,
            &|c, s, st| {
                return EmptyElement::at(c)
                + Circle::new((0,0),s,st.filled());
            },
        ))?;
    }

    Ok(())
}


