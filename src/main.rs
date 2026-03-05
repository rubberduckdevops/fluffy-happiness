mod am2302;
mod binutils;
mod cdev;

use am2302::Reading;
use cdev::push_pull;
use std::{thread, time};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn try_read(gpio_number: u32) -> Option<Reading> {
    let mut final_result = None;
    let all_data = push_pull(gpio_number);
    if all_data.len() < 40 {
        tracing::warn!("Saad, read not enough data");
        return final_result;
    }
    for data in all_data.windows(40) {
        let result = Reading::from_binary_vector(&data);
        match result {
            Ok(reading) => {
                final_result = Some(reading);
                break;
            }
            Err(e) => {
                tracing::error!("Error: {:?}", e)
            }
        }
    }
    final_result
}

fn setup_logging() {
    let file_appender = tracing_subscriber::fmt::layer()
        .with_writer(std::fs::File::create("sensor.log").expect("Failed to create log file"))
        .with_ansi(false);

    let stdout_layer = tracing_subscriber::fmt::layer().with_writer(std::io::stdout);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with(file_appender)
        .with(stdout_layer)
        .init();
}



fn main() {
    setup_logging();

    tracing::info!("Starting Sensor Data Collection");

    let gpio_number = 4; // GPIO4  (7)
    let sleep_time = time::Duration::from_secs(5);
    tracing::info!("GPIO: {}", &gpio_number);
    loop {
        tracing::debug!("Sleeping for another {:?}, to be sure device is ready", sleep_time);
        thread::sleep(sleep_time);
        match try_read(gpio_number) {
            Some(reading) => {
                tracing::info!("Reading: {:?}", reading);
            },
            None => tracing::warn!("Unable to get the data"),
        }
    }

    // for _ in 1..30 {
    //     println!(
    //         "Sleeping for another {:?}, to be sure that device is ready",
    //         sleep_time
    //     );
    //     thread::sleep(sleep_time);
    //     match try_read(gpio_number) {
    //         Some(reading) => {
    //             tracing::info!("Reading: {:?}", reading);
    //         },
    //         None => tracing::warn!("Unable to get the data"),
    //     }
    // }
}
