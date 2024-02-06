#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;


use cortex_m_rt::entry;
use stm32f4xx_hal::{
    pac,
    prelude::*,
    spi,
    gpio::PinState,
};
// use hal::{gpio::NoPin, pac, prelude::*};
// use display_interface_spi::SPIInterfaceNoCS;
use st7735_lcd;
use st7735_lcd::Orientation;

/// SPI mode
pub const MODE: spi::Mode = spi::Mode {
    phase: spi::Phase::CaptureOnFirstTransition,
    polarity: spi::Polarity::IdleLow,
};


use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;

#[entry]
fn main() -> ! {
    #[allow(unused_variables)]
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // create SPI interface
        // let spi = spim::Spim::new(p.SPIM0, pins, spim::Frequency::M8, spim::MODE_3, 122);

        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(48.MHz()).freeze();
        let mut delay = dp.TIM1.delay_us(&clocks);
        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();
        let mut cs = gpioa.pa4.into_push_pull_output_in_state(PinState::High);
        let sck = gpioa.pa5.into_alternate::<5>();
        // let miso = gpioa.pa6.into_alternate();
        let mosi = gpioa.pa7.into_alternate();
        let rst = gpiob.pb0.into_push_pull_output_in_state(PinState::High);
        let dc = gpiob.pb1.into_push_pull_output_in_state(PinState::High);
        let spi = dp.SPI1.spi_bidi(
            (sck, mosi),
            MODE,
            3.MHz(),
            &clocks,
        );
        delay.delay_ms(100);

        let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, false, true, 160, 80);

        cs.set_low();
        disp.init(&mut delay).unwrap();
        disp.set_orientation(&Orientation::Landscape).unwrap();
        disp.set_offset(1, 26);
        disp.clear(Rgb565::BLACK).unwrap();
        cs.set_high();

        delay.delay_ms(100);
    
        // draw
        let image_raw: ImageRawLE<Rgb565> = ImageRaw::new(include_bytes!("rgb565-car.raw"), 160);
        let image = Image::new(&image_raw, Point::new(0, 0));
        cs.set_low();
        image.draw(&mut disp).unwrap();
        cs.set_high();
    }

    loop {}
}

// #[exception]
// fn HardFault(ef: &ExceptionFrame) -> ! {
//     panic!("{:#?}", ef);
// }