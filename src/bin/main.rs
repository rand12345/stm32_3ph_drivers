#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(error_in_core)]

use defmt::debug;
use defmt::error;
use embassy_executor::Spawner;
use embassy_stm32::interrupt;
use embassy_stm32::usart;
use embassy_stm32::usart::Uart;
use embassy_time::Duration;
use embedded_alloc::Heap;

use crate::statics::FIRSTMQTT;

use {defmt_rtt as _, panic_probe as _};
pub mod config;
mod errors;
mod statics;
mod tasks;
mod types;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024 * 2;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    let mut config = embassy_stm32::Config::default();
    config.rcc.pll48 = true; //Some(embassy_stm32::time::Hertz(1_000_000));
    let p = embassy_stm32::init(config);
    debug!("3ph driver test - SDM230 meter");

    let usart1 = {
        let mut config = usart::Config::default();
        config.baudrate = 9600;
        let irq = interrupt::take!(USART1);
        Uart::new(p.USART1, p.PA10, p.PA9, irq, p.DMA2_CH7, p.DMA2_CH5, config)
    };

    let usart2 = {
        let mut config = usart::Config::default();
        config.baudrate = 9600;
        let irq = interrupt::take!(USART2);
        Uart::new(p.USART2, p.PA3, p.PA2, irq, p.DMA1_CH6, p.DMA1_CH5, config)
    };
    let usart3 = {
        let mut config = usart::Config::default();
        config.baudrate = 9600;
        let irq = interrupt::take!(USART3);
        Uart::new(
            p.USART3, p.PC11, p.PC10, irq, p.DMA1_CH3, p.DMA1_CH1, config,
        )
    };
    let usart6 = {
        let mut config = usart::Config::default();
        config.baudrate = 115200;
        let irq = interrupt::take!(USART6);
        Uart::new(p.USART6, p.PC7, p.PC6, irq, p.DMA2_CH6, p.DMA2_CH1, config)
    };

    // defmt::unwrap!(spawner.spawn(crate::tasks::mqtt::uart_task(usart6)));
    defmt::unwrap!(spawner.spawn(crate::tasks::led_task(p.PA5)));
    debug!("Testing");
    {
        // LOOPBACK TESTS
        debug!("Loop");
        let mut uart1 = usart1;
        let mut uart2 = usart2;

        let mut de1 = embassy_stm32::gpio::Output::new(
            p.PB3,
            embassy_stm32::gpio::Level::Low,
            embassy_stm32::gpio::Speed::High,
        );
        let mut re1 = embassy_stm32::gpio::Output::new(
            p.PB5,
            embassy_stm32::gpio::Level::Low,
            embassy_stm32::gpio::Speed::High,
        );
        let mut de2 = embassy_stm32::gpio::Output::new(
            p.PC4,
            embassy_stm32::gpio::Level::Low,
            embassy_stm32::gpio::Speed::High,
        );
        let mut re2 = embassy_stm32::gpio::Output::new(
            p.PB13,
            embassy_stm32::gpio::Level::Low,
            embassy_stm32::gpio::Speed::High,
        );
        let mut buffer = [0; 5];

        // uart1.blocking_flush().unwrap();
        // uart2.blocking_flush().unwrap();

        loop {
            debug!("1");
            de1.set_high();
            de2.set_low();
            re1.set_high();
            re2.set_low();

            embassy_time::Timer::after(Duration::from_millis(300)).await;
            uart1.write(&"UART1 TX".as_bytes()).await.unwrap();
            embassy_time::Timer::after(Duration::from_millis(300)).await;
            debug!("2");
            // if let Err(e) = uart2.blocking_read(&mut buffer) {
            //     error!("Error: {}", e)
            // };

            debug!("3");
            re2.set_high();
            de1.set_low();
            debug!("UART2 RX: {}", buffer);

            embassy_time::Timer::after(Duration::from_millis(300)).await;
            buffer = [0; 5];
        }
        {
            de2.set_high();
            re1.set_low();
            uart2.blocking_write(&"UART2 TX".as_bytes()).unwrap();

            embassy_time::Timer::after(Duration::from_secs(1)).await;
            uart1.read_until_idle(&mut buffer).await.unwrap();
            re1.set_high();
            de2.set_low();
            debug!("UART1 RX: {}", buffer);
            buffer = [0; 5];
            debug!("4");
            embassy_time::Timer::after(Duration::from_secs(1)).await
        }
    }
    FIRSTMQTT.wait().await;
    defmt::info!("Readings aquired from MQTT, starting meters");
    defmt::unwrap!(spawner.spawn(crate::tasks::process::meter1(usart1, p.PB3, p.PB5)));
    defmt::unwrap!(spawner.spawn(crate::tasks::process::meter2(usart2, p.PC4, p.PB13)));
    defmt::unwrap!(spawner.spawn(crate::tasks::process::meter3(usart3, p.PC12, p.PD2)));

    // static global value per phase/inverter giving frequency and power apparant (fake)
    // function for handling rx input from inverter (req)
    // monitor usart6 for json payload
    // {meter1: -1, meter2: 23, meter3: 31}
    //
}
