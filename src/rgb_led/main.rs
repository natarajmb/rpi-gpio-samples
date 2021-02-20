use std::error::Error;
use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio, Level};
use rppal::system::DeviceInfo;

const GPIO_RED: u8 = 17;
const GPIO_GREEN: u8 = 27;
const GPIO_BLUE: u8 = 22;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking LED with Red, Green & Blue on a {}", DeviceInfo::new()?.model());

    let mut red = Gpio::new()?.get(GPIO_RED)?.into_output();
    let mut green = Gpio::new()?.get(GPIO_GREEN)?.into_output();
    let mut blue = Gpio::new()?.get(GPIO_BLUE)?.into_output();

    loop {
        red.write(Level::High);
        green.write(Level::Low);
        blue.write(Level::Low);
        thread::sleep(Duration::from_millis(500));

        red.write(Level::Low);
        green.write(Level::High);
        blue.write(Level::Low);
        thread::sleep(Duration::from_millis(500));

        red.write(Level::Low);
        green.write(Level::Low);
        blue.write(Level::High);
        thread::sleep(Duration::from_millis(500));
    }
}