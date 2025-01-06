use chrono::{Datelike, NaiveDate};
use plotters::prelude::*;
use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the .csv file containing the data
    let df = LazyCsvReader::new("datasets/ipc_caba_base2021.csv")
        .with_has_header(true)
        .finish()?;

    let data_2023_10_01 = df
        .clone()
        .filter(col("indice_tiempo").eq(lit("2023-10-01")))
        .collect()?;

    let ipc_2023_10_01: f64 = data_2023_10_01
        .column("nivel_general")?
        .f64()?
        .iter()
        .next()
        .flatten()
        .unwrap();

    let data_2024_10_01 = df
        .clone()
        .filter(col("indice_tiempo").eq(lit("2024-10-01")))
        .collect()?;

    let ipc_2024_10_01: f64 = data_2024_10_01
        .column("nivel_general")?
        .f64()?
        .iter()
        .next()
        .flatten()
        .unwrap();

    println!("{data_2023_10_01:?}");
    println!("{data_2024_10_01:?}");

    let inflation_interanual = (ipc_2024_10_01 - ipc_2023_10_01) / ipc_2023_10_01 * 100.0;
    println!(
        "Interanual Inflation October2023-October2024 based on CABA's IPC: {inflation_interanual:.2}%"
    );
    println!("For more Information: https://en.wikipedia.org/wiki/Inflation#Measures");

    // Convert the dates from string to NaiveDate
    let parsed_dates: Vec<NaiveDate> = df.clone().select([col("indice_tiempo")]).collect()?[0]
        .str()?
        .into_iter()
        .filter_map(|opt_date| {
            let date_str = opt_date.unwrap();
            Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap())
        })
        .collect();

    // Parse the IPCs
    let parsed_ipcs: Vec<f64> = df.select([col("nivel_general")]).collect()?[0]
        .f64()?
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .unwrap();

    // Set up the DrawingArea from plotters
    let drawing_area =
        SVGBackend::new("imgs/ipc_caba_base2021.svg", (1200, 800)).into_drawing_area();
    drawing_area.fill(&WHITE)?;

    // Set up the ChartContext
    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("IPC General - CABA - BASE 2021", ("sans-serif", 40))
        .margin(50)
        .x_label_area_size(70)
        .y_label_area_size(30)
        .build_cartesian_2d(
            parsed_dates.first().unwrap().clone()..parsed_dates.last().unwrap().clone(),
            parsed_ipcs.first().unwrap().floor()..parsed_ipcs.last().unwrap().ceil(),
        )?;

    // Configure the chart with custom x labels and styles
    chart
        .configure_mesh()
        .label_style(("sans-serif", 15))
        .x_label_formatter(&move |d| {
            format!("{:02}/{:02}/{:02}", d.day(), d.month(), d.year() % 100)
        })
        .x_labels(parsed_dates.len() / 2) // Set number of x labels based on actual data points
        .y_labels(15) // Adjust as needed
        .draw()?;

    // Draw the data, using zip "join" both iterators
    chart.draw_series(
        LineSeries::new(
            parsed_dates
                .iter()
                .zip(parsed_ipcs.iter())
                .map(|(date, value)| (*date, *value)),
            &BLUE,
        )
        .point_size(4),
    )?;

    Ok(())
}
