use esp_hal::delay::Delay;

pub fn fire_engine() {
    // TODO: turn on mosfet
    // wait 0.5 secs
    let delay = Delay::new();
    delay.delay_millis(500);
    // turn off and return
}

pub fn parachute_deploy() {
    // takes around 1.2 second in total (with 1000 ms of waiting)
    let delay = Delay::new();

    for _ in 0..10 {
        // TODO: turn on MOSFET
        delay.delay_millis(50);
        // TODO: turn off
        delay.delay_millis(50);
    }

    return;
}
