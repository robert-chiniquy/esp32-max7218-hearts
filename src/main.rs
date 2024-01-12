use esp_idf_svc::hal::spi::{config::*, *};
use esp_idf_svc::hal::{delay::*, prelude::*, *};

use max7219::*;

use font8x8::{UnicodeFonts, BASIC_FONTS};

fn main() -> ! {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let sclk = peripherals.pins.gpio0;
    let mosi = peripherals.pins.gpio2;
    let cs = peripherals.pins.gpio3;

    let spi_drv = SpiDriver::new(
        peripherals.spi2,
        sclk,
        mosi,
        None::<gpio::AnyIOPin>,
        &SpiDriverConfig::new(),
    )
    .unwrap();

    let config = Config::new().baudrate(10.MHz().into()).data_mode(Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    });

    let spi = SpiDeviceDriver::new(spi_drv, Some(cs), &config).unwrap();

    let mut display = MAX7219::from_spi(4, spi).unwrap();

    display.power_on().unwrap();
    display.set_intensity(0, 0x2).unwrap();

    let msg: Vec<char> = "EMMA".chars().rev().collect();

    let mut heart: [u8; 8] = [
        0b00000000, 0b01100110, 0b11111111, 0b11111111, 0b01111110, 0b00111100, 0b00011000,
        0b00000000,
    ];

    loop {
        for addr in [3, 2, 1, 0] {
            let letter = BASIC_FONTS.get(msg[(addr) % 4]).unwrap();
            display.write_raw(addr, &flip_90(&letter)).unwrap();
        }
        FreeRtos::delay_ms(1800_u32);

        for addr in [2, 1, 0, 3] {
            display.write_raw(addr, &heart).unwrap();
            if addr % 2 == 1 {
                heart = flip_180(&heart);
            } else {
                heart = flip_90(&heart);
            }
            FreeRtos::delay_ms(124_u32);
        }
        FreeRtos::delay_ms(1275_u32);

        log::info!("❤️");
    }
}

fn flip_90(bitmap: &[u8; 8]) -> [u8; 8] {
    let mut result = [0; 8];

    for i in 0..8 {
        result[i] |= (bitmap[0] >> i) & 0b01
            | ((bitmap[1] >> i) & 0b01) << 1
            | ((bitmap[2] >> i) & 0b01) << 2
            | ((bitmap[3] >> i) & 0b01) << 3
            | ((bitmap[4] >> i) & 0b01) << 4
            | ((bitmap[5] >> i) & 0b01) << 5
            | ((bitmap[6] >> i) & 0b01) << 6
            | ((bitmap[7] >> i) & 0b01) << 7;
    }

    result
}

fn flip_180(bitmap: &[u8; 8]) -> [u8; 8] {
    let mut result = [0; 8];

    for i in 0..8 {
        let src_byte = bitmap[i];
        let mut dest_byte = 0;

        dest_byte |= ((src_byte >> 0) & 0b01) << 7;
        dest_byte |= ((src_byte >> 1) & 0b01) << 6;
        dest_byte |= ((src_byte >> 2) & 0b01) << 5;
        dest_byte |= ((src_byte >> 3) & 0b01) << 4;
        dest_byte |= ((src_byte >> 4) & 0b01) << 3;
        dest_byte |= ((src_byte >> 5) & 0b01) << 2;
        dest_byte |= ((src_byte >> 6) & 0b01) << 1;
        dest_byte |= ((src_byte >> 7) & 0b01) << 0;

        result[i] = dest_byte;
    }

    result
}
