#[allow(unused)]
use tokio::prelude::*;
use {
    co2mon::{Result as Co2Result, Sensor},
    influxdb::{Client, InfluxDbWriteable},
    std::{error::Error, thread, time::Duration},
    tokio::runtime::Runtime,
};

mod structs;
mod utils;

#[paw::main]
fn main(args: structs::Args) -> Result<(), Box<dyn Error>> {
    utils::privdrop();
    let mut rt = Runtime::new()?;
    rt.block_on(run_sensor_test(&args))?;
    Ok(())
}

async fn run_sensor_test(args: &structs::Args) -> Co2Result<()> {
    let sensor = Sensor::open_default()?;
    let client = Client::new(args.influxdb_uri(), &args.influxdb_database);

    loop {
        let reading_result = dbg!(structs::AirQualityReading::from_sensor_now(&sensor));
        match reading_result {
            Ok(reading) => {
                let query = &reading.into_query(&args.influxdb_measurement);
                let write_result = client.query(query).await;
                if let Err(error) = write_result {
                    eprintln!("writing to influxdb might have failed: {}", error)
                };
            }
            Err(_e) => { /* noop, skip cycle */ }
        }
        thread::sleep(Duration::from_secs(5));
    }
}
