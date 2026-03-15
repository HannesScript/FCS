#![no_std]
#![no_main]

use core::fmt::Write;

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    main,
};

use heapless::String;

mod sensors;
mod radio;
mod sd_card;
mod mosfets;

use sensors::*;
use radio::*;
use sd_card::*;
use mosfets::{parachute_deploy, fire_engine};

#[main]
fn main() -> ! {
    let mut is_flying: bool = false;
    let mut curr_height_meters: f64 = 0.0;
    let mut max_height_meters: f64 = 0.0;
    let starting_height: f64 = get_height_bmp_m().unwrap_or_default();
    let time: u32 = 0;

    let log_file_path = make_log_file_path(3);

    let _peripherals = esp_hal::init(esp_hal::Config::default());
    esp_println::logger::init_logger_from_env();

    loop {
        curr_height_meters = get_height_bmp_m().unwrap_or_default() - starting_height;
        if curr_height_meters > max_height_meters { max_height_meters = curr_height_meters; }

        receive(&mut is_flying);
        read_send_and_store_data(&starting_height, &time, &log_file_path);

        if is_flying {
            control_fins();
            
            // Deploy chute when falling again
            if curr_height_meters - 3.0 < max_height_meters {
                parachute_deploy();
            }
        }
    }
}

fn receive(is_flying: &mut bool) {
    // TODO: receive commands
    let mut command = String::<32>::new();
    get_received_data(&mut command);

    // if it receives "LAUNCH" it starts a countdown and fire engine
    match command.as_str() {
        "LAUNCH" => {
            *is_flying = true;
            let delay = Delay::new();

            for i in (1..=10).rev() {
                let mut countdown = String::<16>::new();
                let _ = write!(&mut countdown, "{}...", i);
                send_data(&countdown);
                delay.delay_millis(1000);
            }

            send_data("LAUNCH!");
            fire_engine();
        }
        _ => {}
    }
}

fn read_send_and_store_data(starting_height: &f64, time: &u32, log_file_path: &str) {
    let height: f64 = get_height_bmp_m().unwrap_or_default() - starting_height;
    let gps_lat_and_gps_lon: [f64; 2] = get_gps_lat_and_lon().unwrap();
    let temp_c: f32 = get_dht_temp_c().unwrap();
    let humidity_percent: f32 = get_dht_humidity_percent().unwrap();
    let gyro_p_y_r: [f64; 3] = get_gyro_orientation_p_y_r().unwrap();

    let mut data_string = String::<192>::new();
    let _ = write!(
        &mut data_string,
        "time={};height={};gps_lat={};gps_lon={};temp={};humidity={};pitch={};yaw={};roll={}",
        time,
        height,
        gps_lat_and_gps_lon[0],
        gps_lat_and_gps_lon[1],
        temp_c,
        humidity_percent,
        gyro_p_y_r[0],
        gyro_p_y_r[1],
        gyro_p_y_r[2]
    );

    // send via radio
    send_data(&data_string);

    // store to sd
    sd_append_to_file(log_file_path, &data_string);
}

fn control_fins() {

}

fn make_log_file_path(log_index: u32) -> String<32> {
    let mut path = String::<32>::new();
    let _ = write!(&mut path, "logs/{}.txt", log_index);
    path
}
