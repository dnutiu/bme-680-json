use bme680::i2c::Address;
use bme680::{Bme680, IIRFilterSize, OversamplingSetting, PowerMode, SettingsBuilder};
use core::time::Duration;
use clap::Parser;
use embedded_hal::delay::DelayNs;
use linux_embedded_hal as hal;
use linux_embedded_hal::Delay;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct JsonData {
    temperature: f32,
    pressure: f32,
    humidity: f32,
    gas_resistance: u32
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = "A CLI to grab the measurements of the Bme680 sensor and output them in the console in a JSON format.")]
struct Args {
    /// I2C address to use
    #[arg(long, default_value_t = String::from("/dev/i2c-1"))]
    i2c_address: String,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let i2c = hal::I2cdev::new(args.i2c_address)?;
    let mut delay = Delay {};
    let mut dev = Bme680::init(i2c, &mut delay, Address::Primary)?;

    let settings = SettingsBuilder::new()
        .with_humidity_oversampling(OversamplingSetting::OS2x)
        .with_pressure_oversampling(OversamplingSetting::OS4x)
        .with_temperature_oversampling(OversamplingSetting::OS8x)
        .with_temperature_filter(IIRFilterSize::Size3)
        .with_gas_measurement(Duration::from_millis(1500), 320, 23)
        .with_run_gas(true)
        .build();

    dev.set_sensor_settings(&mut delay, &settings)?;
    dev.set_sensor_mode(&mut delay, PowerMode::ForcedMode)?;

    Delay {}.delay_ms(5000u32);
    let (data, _) = dev.get_measurement(&mut delay)?;
    let serialized = serde_json::to_string(&JsonData{
        temperature: data.temperature_celsius(),
        pressure: data.pressure_hpa(),
        humidity: data.humidity_percent(),
        gas_resistance: data.gas_resistance_ohm(),
    })?;

    println!("{}", serialized);

    Ok(())
}