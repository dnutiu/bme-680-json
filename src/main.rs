use bme680::i2c::Address;
use bme680::{Bme680, IIRFilterSize, OversamplingSetting, PowerMode, SettingsBuilder};
use core::time::Duration;
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

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let i2c = hal::I2cdev::new("/dev/i2c-1")?;
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