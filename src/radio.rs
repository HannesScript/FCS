//////////////////////////////////////////////////
/// Radio Stuff
//////////////////////////////////////////////////

use heapless::string;

static RADIO_CHANNEL: u32 = 91; // not a default of the nRF24L01 for extra safety
static RADIO_FREQUENCY: u32 = 2491_000_000; // Hz ( always 2400 MHz + 91 MHz (channel) )

pub fn send_data(data: string) {
    todo!("Implement");
}

pub fn get_received_data(buffer: &mut string) {
    buffer = string::String("LAUNCH");
    todo!("Implement");
}
