use {
    chrono::{DateTime, Utc},
    co2mon::{Reading, Sensor},
    influxdb::InfluxDbWriteable,
    std::error::Error,
};

#[derive(structopt::StructOpt)]
pub(crate) struct Args {
    /// influxdb port
    #[structopt(
        short = "P",
        long = "influxdb-port",
        env = "INFLUXDB_PORT",
        default_value = "8086"
    )]
    pub(crate) influxdb_port: u16,

    /// influxdb host
    #[structopt(
        short = "H",
        long = "influxdb-host",
        env = "INFLUXDB_HOST",
        default_value = "127.0.0.1"
    )]
    pub(crate) influxdb_host: String,

    /// influxdb database
    #[structopt(
        short = "D",
        long = "influxdb-database",
        env = "INFLUXDB_DATABASE",
        default_value = "test_co2"
    )]
    pub(crate) influxdb_database: String,

    /// influxdb measurement (like a collection/table)
    #[structopt(
        short = "M",
        long = "influxdb-measurement",
        env = "INFLUXDB_DATABASE",
        default_value = "measurements2"
    )]
    pub(crate) influxdb_measurement: String,
}

impl Args {
    pub(crate) fn influxdb_uri(&self) -> String {
        format!("http://{}:{}", self.influxdb_host, self.influxdb_port)
    }
}

#[derive(InfluxDbWriteable, Debug)]
pub(crate) struct AirQualityReading {
    time: DateTime<Utc>,
    co2: u16,         // aaa ... bbbb (low: 400ish, high: 1200+)
    temperature: f32, // °C
    #[tag]
    location: String,
}

impl AirQualityReading {
    pub(crate) fn from_sensor_now(sensor: &Sensor) -> Result<Self, Box<dyn Error>> {
        match sensor.read() {
            Ok(reading) => Ok(Self::from_sensor_reading_now(reading)),
            Err(e) => {
                eprintln!("{}", e);
                Err(Box::new(e))
            }
        }
    }

    fn from_sensor_reading_now(reading: Reading) -> Self {
        let now: DateTime<Utc> = Utc::now();
        Self {
            time: now,
            co2: reading.co2(),
            temperature: reading.temperature(),
            location: String::from("home"),
        }
    }
}
