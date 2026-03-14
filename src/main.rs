#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    main,
    time::{Duration, Instant},
    delay::Delay,
    prelude::*,
};

pub use heapless::*;

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
    let mut curr_height_meters: i64 = 0;
    let mut max_height_meters: i64 = 0;

    let _peripherals = esp_hal::init(esp_hal::Config::default());
    esp_println::logger::init_logger_from_env();

    loop {
        curr_height_meters = get_height_bmp();
        if curr_height_meters > max_height_meters { max_height_meters = curr_height_meters; }

        receive(&mut is_flying);
        read_send_and_store_data();

        if is_flying {
            control_fins();
            
            // Deploy chute when falling again
            if curr_height_meters - 3 < max_height_meters {
                parachute_deploy();
            }
        }
    }
}

fn receive(is_flying: &mut bool) {
    // TODO: receive commands
    let mut command: string = "";
    get_received_data(&mut command);

    // if it receives "LAUNCH" it starts a countdown and fire engine
    if "LAUNCH" == command {
        let delay = Delay::new();

        for i in 10..=1 {
            send_data(format!("{}...", i));
            delay.delay_millis(1000);
        }

        send_data("LAUNCH!");
        fire_engine();
    }
}

fn read_send_and_store_data() {

}

fn control_fins() {

}

fn parachute_deploy() {

}
