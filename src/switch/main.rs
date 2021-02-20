use std::error::Error;
use std::thread;
use std::time::Duration;
use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

const GPIO_SWITCH: u8 = 17;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Printing a switch state on a {}", DeviceInfo::new()?.model());

    let pin = Gpio::new()?.get(GPIO_SWITCH)?.into_input();

    loop {
        thread::sleep(Duration::from_millis(500));
        if pin.is_low() {
            println!("Switch is off.");
            thread::sleep(Duration::from_millis(500));
        }
        if pin.is_high() {
            println!("Switch is on!");
            thread::sleep(Duration::from_millis(500));
        }
    }
}