use {
    crate::structs,
    co2mon::{Result as Co2Result, Sensor},
    futures_timer::Delay,
    influxdb::{Client, InfluxDbWriteable},
    std::time::Duration,
};

static LOOP_INTERVAL_SECONDS: u64 = 5;

pub(crate) async fn run_forever(args: &structs::Args) -> Co2Result<()> {
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
        // alternative for std::thread
        // thread::sleep(Duration::from_secs(LOOP_INTERVAL_SECONDS));
        let _ = Delay::new(Duration::from_secs(LOOP_INTERVAL_SECONDS)).await;
    }
}
