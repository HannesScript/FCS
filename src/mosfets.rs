pub fn fire_engine() {
    // TODO: turn on mosfet, wait 0.5 secs, turn off and return
}

pub fn parachute_deploy() {
    for _ in 0..10 {
        // TODO: turn on MOSFET
        Delay::new().delay_millis(75);
        // TODO: turn off
    }

    return;
}
