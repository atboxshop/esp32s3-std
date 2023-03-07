use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::thread::sleep;
use std::time::Duration;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;



fn main()
{
    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio9).unwrap();
    let mut btn = PinDriver::input(peripherals.pins.gpio12).unwrap();
    let mut btn_src = PinDriver::output(peripherals.pins.gpio13).unwrap();
    btn.set_pull(Pull::Down).unwrap();
    btn_src.set_high().unwrap();
   
    loop {
        sleep(Duration::from_millis(10));
        if btn.is_low() {
            led.set_high().unwrap();
            println!("{}", btn.is_low());
        }
        else {
            led.set_low().unwrap();
            println!("{}",btn.is_low());
        }
   }
}