const GPIO_DATA: u8 = 2;

const THRESHOLD_0_1: i32 = 250;

fn main() {
    loop {
        read();
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}

fn read() {
    let mut bits: Vec<u8> = Vec::with_capacity(64);

    let mut pin = rppal::gpio::Gpio::new()
        .unwrap()
        .get(GPIO_DATA)
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
        // use std::io::Write;
        loop {
            if pin.is_high() {
                break;
            }
        }
        let mut counter = 0;
        while pin.is_high() {
            counter += 1;
            // ????????????????????????????????
            // std::thread::sleep(std::time::Duration::from_micros(1));
        }
        // write!(out, "{} ", counter).unwrap();
        print!("{} ", counter);
        // ??????????????????????????
        if counter > THRESHOLD_0_1 {
            bits.push(1);
        } else {
            bits.push(0);
        }
    }
    print!("\n");

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
        panic!(
            "warn: Failed to check parity. Check: {}, Parity: {}",
            check, bytes[4]
        );
    }

    println!("Temp: {}.{}C, Hum: {}%", bytes[2], bytes[3], bytes[0]);
}
