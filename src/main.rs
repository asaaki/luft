#[allow(unused)]
use tokio::prelude::*;
use {
    co2mon::{Result as Co2Result, Sensor},
    influxdb::{Client, Query, Timestamp},
    std::{error::Error, thread, time::Duration},
    tokio::runtime::Runtime,
};

#[derive(structopt::StructOpt)]
struct Args {
    /// influxdb port
    #[structopt(
        short = "P",
        long = "influxdb-port",
        env = "INFLUXDB_PORT",
        default_value = "8086"
    )]
    influxdb_port: u16,

    /// influxdb host
    #[structopt(
        short = "H",
        long = "influxdb-host",
        env = "INFLUXDB_HOST",
        default_value = "127.0.0.1"
    )]
    influxdb_host: String,
}

#[paw::main]
fn main(args: Args) -> Result<(), Box<dyn Error>> {
    privdrop();
    let mut rt = Runtime::new()?;
    rt.block_on(run_sensor_test(&args))?;
    Ok(())
}

fn privdrop() {
    if !nix::unistd::geteuid().is_root() {
        return;
    }
    privdrop::PrivDrop::default()
        .chroot("/var/empty")
        .user("nobody")
        .apply()
        .unwrap_or_else(|e| eprintln!("Failed to drop privileges: {}", e));
}

async fn run_sensor_test(args: &Args) -> Co2Result<()> {
    let sensor = Sensor::open_default()?;
    let influxdb_uri = format!("http://{}:{}", args.influxdb_host, args.influxdb_port);
    let client = Client::new(influxdb_uri, "test_co2");

    loop {
        match sensor.read() {
            Ok(reading) => {
                dbg!("{:.4} °C, {} ppm CO₂", reading.temperature(), reading.co2());
                let write_query = Query::write_query(Timestamp::Now, "measurements")
                    .add_field("co2", reading.co2())
                    .add_field("temperature", reading.temperature())
                    .add_tag("loc", "home");

                client.query(&write_query).await.unwrap_or_else(|e| {
                    eprintln!("influxdb client write error: {}", e);
                    "influx client error".into()
                });
            }
            Err(e) => eprintln!("{}", e),
        }
        thread::sleep(Duration::from_secs(5));
    }
}
