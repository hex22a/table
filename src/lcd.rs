use arduino_hal::{delay_ms, delay_us, port::{Pin, mode::Output}};

pub struct Lcd {
    rs: Pin<Output>,
    e: Pin<Output>,
    d: [Pin<Output>; 4],
}

impl Lcd {
    pub fn new(rs: Pin<Output>, e: Pin<Output>, d: [Pin<Output>; 4]) -> Self {
        let mut lcd = Lcd { rs, e, d };
        delay_ms(100);
        lcd.rs.set_low();
        lcd.nibble(0x3);
        delay_ms(10);
        lcd.nibble(0x3);
        delay_ms(1);
        lcd.nibble(0x3);
        delay_ms(1);
        lcd.nibble(0x2);
        delay_ms(1);
        lcd.cmd(0x28);
        lcd.cmd(0x0C);
        lcd.cmd(0x06);
        lcd.clear();
        lcd
    }

    fn nibble(&mut self, n: u8) {
        for i in 0..4 {
            if (n >> i) & 1 == 1 {
                self.d[i].set_high();
            } else {
                self.d[i].set_low();
            }
        }
        self.e.set_high();
        delay_us(1);
        self.e.set_low();
        delay_us(50);
    }

    fn send(&mut self, byte: u8, data: bool) {
        if data {
            self.rs.set_high();
        } else {
            self.rs.set_low();
        }
        self.nibble(byte >> 4);
        self.nibble(byte & 0x0F);
    }

    pub fn cmd(&mut self, c: u8) {
        self.send(c, false);
    }

    pub fn clear(&mut self) {
        self.cmd(0x01);
        delay_ms(2);
    }

    pub fn set_cursor(&mut self, col: u8, row: u8) {
        let row = if row == 0 { 0x00 } else { 0x40 };
        let addr = col + row;
        self.cmd(0x80 | addr);
    }

    pub fn print(&mut self, s: &str) {
        for b in s.bytes() {
            self.send(b, true);
        }
    }
}
