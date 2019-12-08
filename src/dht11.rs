const THRESHOLD_0_1: i32 = 250;

pub struct DHT11 {
    gpio_data_port: u8,
}

pub struct Metric {
    pub temperature: String, // :(
    pub humidity: u8,
    pub parity: u8,
}

#[derive(failure::Fail, std::fmt::Debug)]
pub enum DTH11Error {
    #[fail(display = "The expected parity is {}, however it received {}", _0, _1)]
    ParityCheckError(u8, u8),
}

impl DHT11 {
    pub fn new(gpio_data_port: u8) -> Self {
        DHT11 {
            gpio_data_port: gpio_data_port,
        }
    }

    pub fn read(&self) -> Result<Metric, failure::Error> {
        let mut bits: Vec<u8> = Vec::with_capacity(64);

        let mut pin = rppal::gpio::Gpio::new()
            .unwrap()
            .get(self.gpio_data_port)
            .unwrap()
            .into_io(rppal::gpio::Mode::Output);

        // handshake (?)
        pin.set_high();
        std::thread::sleep(std::time::Duration::from_micros(5));
        pin.set_low();
        std::thread::sleep(std::time::Duration::from_millis(25));
        pin.set_high();
        pin.set_mode(rppal::gpio::Mode::Input);
        loop {
            if pin.is_low() {
                break;
            }
        }
        loop {
            if pin.is_high() {
                break;
            }
        }
        loop {
            if pin.is_low() {
                break;
            }
        }

        // read serial data
        for _ in 0..40 {
            loop {
                if pin.is_high() {
                    break;
                }
            }

            let mut counter = 0;
            while pin.is_high() {
                counter += 1;
            }
            if counter > THRESHOLD_0_1 {
                bits.push(1);
            } else {
                bits.push(0);
            }
        }

        let mut bytes = Vec::with_capacity(5);
        for chunk in bits.chunks(8) {
            let mut byte = 0;
            for (i, bit) in chunk.iter().enumerate() {
                let digit = 7 - i as u32;
                byte += bit * (2_u8.pow(digit));
            }
            bytes.push(byte);
        }

        let check = bytes[0] + bytes[1] + bytes[2] + bytes[3];
        if check != bytes[4] {
            Err(failure::Error::from(DTH11Error::ParityCheckError(
                bytes[4], check,
            )))
        } else {
            Ok(Metric {
                temperature: format!("{}.{}", bytes[2], bytes[3]),
                humidity: bytes[0],
                parity: bytes[4],
            })
        }
    }
}
