use cpal::traits::{DeviceTrait, HostTrait};

fn main() {
    let host = cpal::default_host();
    println!(
        "default input: {:?}",
        host.default_input_device().unwrap().name()
    );
    for device in host.input_devices().unwrap() {
        println!("input devices found: {:?}", device.name());
    }
    println!(
        "default output: {:?}",
        host.default_output_device().unwrap().name()
    );
    for device in host.output_devices().unwrap() {
        println!("output devices found: {:?}", device.name());
    }
}
