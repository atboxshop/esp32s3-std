use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys::esp_random;
use smart_leds::{
    brightness,
    gamma,
    SmartLedsWrite,
};
use std::thread::sleep;
use std::time::Duration;
use ws2812_esp32_rmt_driver::driver::color::LedPixelColorGrbw32;
use ws2812_esp32_rmt_driver::{LedPixelEsp32Rmt, RGB8};

const LED_PIN: u32 = 48;
const NUM_PIXELS:usize = 1;

fn main() -> ! {
    let mut ws2812 = LedPixelEsp32Rmt::<RGB8, LedPixelColorGrbw32>::new(0, LED_PIN).unwrap();
    let mut red = unsafe{esp_random()} as u8;
    let mut green = unsafe{esp_random()} as u8;
    let mut blue = unsafe{esp_random()} as u8;
    let mut color = RGB8{r:red,g:green,b:blue};
    let bright = 5u8;
    let mut data: [RGB8; NUM_PIXELS] = [color; NUM_PIXELS];
    loop {
        ws2812.write(brightness(gamma(data.iter().cloned()), bright))
                .unwrap();
        sleep(Duration::from_millis(200));
       
        red = unsafe{esp_random()} as u8;
        println!("{}",red);
        green = unsafe{esp_random()} as u8;
        println!("{}",green);
        blue = unsafe{esp_random()} as u8;
        println!("{}",blue);
        color = RGB8{r:red,g:green,b:blue};
        data = [color; NUM_PIXELS];
    }
}