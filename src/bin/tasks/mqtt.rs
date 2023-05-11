use crate::statics::*;
use defmt::error;
use defmt::info;
use defmt::Debug2Format;
use embassy_stm32::peripherals::*;
use embassy_stm32::usart::Uart;
#[embassy_executor::task]

pub async fn uart_task(uart: Uart<'static, USART6, DMA2_CH6, DMA2_CH1>) {
    let mut uart = uart;
    if uart.blocking_flush().is_err() {
        panic!();
    };
    let (mut tx, mut rx) = uart.split();
    let mut buf = [0_u8; 512];
    loop {
        let len = match rx.read_until_idle(&mut buf).await {
            Ok(l) => l,
            Err(e) => {
                error!("MQTT Rx error: {}", e);
                continue;
            }
        };

        let mut config = READINGS.lock().await;
        if let Err(e) = config.update_from_json(&buf[..len]) {
            error!("UART deserialise MQTT bytes error {}", Debug2Format(&e));
            let _ = tx
                .write("UART deserialise MQTT bytes error".as_bytes())
                .await;
        } else {
            info!("Payloads updated from MQTT");
            let _ = tx.write("Payloads updated from MQTT".as_bytes()).await;
            FIRSTMQTT.signal(true)
        };
        buf = [0_u8; 512];
    }
}
