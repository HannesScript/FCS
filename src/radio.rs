//////////////////////////////////////////////////
/// Radio Stuff
//////////////////////////////////////////////////

use heapless::String;

const RADIO_CHANNEL: u8 = 91; // not a default of the nRF24L01 for extra safety
const RADIO_FREQUENCY: u32 = 2_491_000_000; // Hz (always 2400 MHz + 91 MHz)

pub fn send_data(_data: &str) {
    let _ = RADIO_CHANNEL;
    let _ = RADIO_FREQUENCY;
}

pub fn get_received_data(buffer: &mut String<32>) {
    buffer.clear();
}
