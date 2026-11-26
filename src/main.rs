#![no_std]
#![no_main]

mod lcd;

use panic_halt as _;
use lcd::Lcd;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut lcd = Lcd::new(
        pins.d1.into_output().downgrade(),
        pins.d2.into_output().downgrade(),
        [
            pins.d4.into_output().downgrade(),
            pins.d5.into_output().downgrade(),
            pins.d6.into_output().downgrade(),
            pins.d7.into_output().downgrade(),
        ],
    );
    lcd.print("Hello");
    lcd.set_cursor(0, 1);
    lcd.print("Rust on AVR");

    let mut led = pins.d13.into_output();

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
