use crate::statics::READINGS;

use defmt::{error, info};
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    peripherals::*,
    usart::Uart,
};
use embassy_time::{Duration, Timer};

fn calculate_crc(payload: &[u8]) -> [u8; 2] {
    use crc16::*;
    let mut crc: State<MODBUS> = State::new();
    crc.update(&payload);
    crc.get().to_le_bytes()
}

fn process_power_request(req: [u8; 8], pow: f32) -> [u8; 9] {
    let mut payload = [0; 9];
    payload[0..=1].copy_from_slice(&req[0..2]);
    payload[2] = 0x4; // f32 fc4
    payload[3..8].copy_from_slice(&pow.to_be_bytes());
    let crc = calculate_crc(&payload[0..8]);
    payload[8..=9].copy_from_slice(&crc); //crc calc here
    return payload;
}

#[embassy_executor::task]
pub async fn meter1(uart: Uart<'static, USART1, DMA2_CH7, DMA2_CH5>, de_pin: PB3, re_pin: PB5) {
    let mut de = Output::new(de_pin, Level::Low, Speed::VeryHigh);
    let mut re = Output::new(re_pin, Level::Low, Speed::VeryHigh);
    let mut uart = uart;
    const NUM: u8 = 1;
    info!("Starting meter{}", NUM);
    loop {
        re.set_low();
        de.set_low();
        let mut req = [0; 8];
        let _ = uart.read_until_idle(&mut req).await;

        // sync
        if let Some(bytes) = req[0..2].try_into().ok() {
            if ![[0x1, 0x4], [0x2, 0x4]].contains(&bytes) {
                continue;
            }
        } else {
            error!("Invalid meter{} req {:x}", NUM, req);
            continue;
        }
        re.set_high();
        de.set_high();
        // get mutex reading for meter1
        let pow = { READINGS.lock().await.meter1 };
        let payload = process_power_request(req, pow);
        // send payload
        Timer::after(Duration::from_millis(10)).await;
        match uart.write(&payload).await {
            Ok(_) => info!("Meter{}: Sent reading {}W", NUM, pow),
            Err(e) => error!("Meter{}: {}", NUM, e),
        }
    }
}
#[embassy_executor::task]
pub async fn meter2(uart: Uart<'static, USART2, DMA1_CH6, DMA1_CH5>, de_pin: PC4, re_pin: PB13) {
    let mut de = Output::new(de_pin, Level::Low, Speed::VeryHigh);
    let mut re = Output::new(re_pin, Level::Low, Speed::VeryHigh);
    let mut uart = uart;
    const NUM: u8 = 2;
    info!("Starting meter{}", NUM);
    loop {
        re.set_low();
        de.set_low();
        let mut req = [0; 8];
        let _ = uart.read_until_idle(&mut req).await;

        // sync
        if let Some(bytes) = req[0..2].try_into().ok() {
            if ![[0x1, 0x4], [0x2, 0x4]].contains(&bytes) {
                continue;
            }
        } else {
            error!("Invalid meter{} req {:x}", NUM, req);
            continue;
        }
        re.set_high();
        de.set_high();
        // get mutex reading for meter2
        let pow = { READINGS.lock().await.meter1 };
        let payload = process_power_request(req, pow);
        // send payload
        Timer::after(Duration::from_millis(10)).await;
        match uart.write(&payload).await {
            Ok(_) => info!("Meter{}: Sent reading {}W", NUM, pow),
            Err(e) => error!("Meter{}: {}", NUM, e),
        }
    }
}
#[embassy_executor::task]
pub async fn meter3(uart: Uart<'static, USART3, DMA1_CH3, DMA1_CH1>, de_pin: PC12, re_pin: PD2) {
    let mut de = Output::new(de_pin, Level::Low, Speed::VeryHigh);
    let mut re = Output::new(re_pin, Level::Low, Speed::VeryHigh);
    let mut uart = uart;
    const NUM: u8 = 3;
    info!("Starting meter{}", NUM);
    loop {
        re.set_low();
        de.set_low();
        let mut req = [0; 8];
        let _ = uart.read_until_idle(&mut req).await;

        // sync
        if let Some(bytes) = req[0..2].try_into().ok() {
            if ![[0x1, 0x4], [0x2, 0x4]].contains(&bytes) {
                continue;
            }
        } else {
            error!("Invalid meter{} req {:x}", NUM, req);
            continue;
        }

        re.set_high();
        de.set_high();
        // get mutex reading for meter3
        let pow = { READINGS.lock().await.meter1 };
        let payload = process_power_request(req, pow);
        // send payload
        Timer::after(Duration::from_millis(10)).await;
        match uart.write(&payload).await {
            Ok(_) => info!("Meter{}: Sent reading {}W", NUM, pow),
            Err(e) => error!("Meter{}: {}", NUM, e),
        }
    }
}
