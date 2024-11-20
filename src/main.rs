use cpal::traits::{DeviceTrait, HostTrait};

fn main() {
    let host = cpal::default_host();
    for device in host.input_devices().unwrap() {
        println!("devices found: {:?}", device.name());
    }
}
