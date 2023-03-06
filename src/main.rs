use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::adc::config::Config;
use esp_idf_hal::adc::*;
use esp_idf_hal::peripherals::Peripherals;

fn main()
{
    let peripherals = Peripherals::take().unwrap();
    //let mut led = PinDriver::output(peripherals.pins.gpio12).unwrap();
    let mut adc = AdcDriver::new(peripherals.adc2, &Config::new().calibration(true)).unwrap();
    let mut adc_pin: esp_idf_hal::adc::AdcChannelDriver<'_, Gpio13, Atten11dB<_>> =
        AdcChannelDriver::new(peripherals.pins.gpio13).unwrap();
    loop {
        //led.set_high().unwrap();
        //we are sleeping here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(50);
        let value1 = adc.read(&mut adc_pin).unwrap();
        let value2:f32 = value1.into();
        println!("ADC value: {}", value2/1000.00);
        //FreeRtos::delay_ms(500);
        //led.set_low().unwrap();
        //FreeRtos::delay_ms(100);
        //println!("ADC value: {}", adc.read(&mut adc_pin).unwrap());
        //FreeRtos::delay_ms(500);
        //println!("Hello World!");
   }
}