#![no_std]
#![no_main]

mod midi;
mod usb;
mod channel_switcher;

use atsamd_hal as hal;
use hal::gpio::{Pins, PA15, PA17, PA20};
use hal::sercom::{uart, Sercom0};
use midi::Midi;
use midly::live::LiveEvent;
use midly::MidiMessage;
use usb::init_usb;

use crate::hal::delay::Delay;
use crate::hal::gpio::{Pin, PushPullOutput};
use crate::pac::CorePeripherals;
use cortex_m_rt::entry;
use hal::clock::GenericClockController;
use hal::pac;
use hal::prelude::*;
use pac::Peripherals;
use panic_halt as _;
use crate::channel_switcher::ChannelSwitcher;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let pins = Pins::new(peripherals.PORT);
    let mut red_led: Pin<PA17, PushPullOutput> = pins.pa17.into();
    let mut set: Pin<PA15, PushPullOutput> = pins.pa15.into();
    let mut unset: Pin<PA20, PushPullOutput> = pins.pa20.into();

    let mut channel_switcher = ChannelSwitcher::new(set, unset);

    // Take peripheral and pins
    let mut pm = peripherals.PM;

    if cfg!(debug) {
        init_usb(
            &mut core.NVIC,
            &mut pm,
            peripherals.USB,
            &mut clocks,
            pins.pa24,
            pins.pa25,
        );
    }

    let mut midi = Midi::new(
        &mut clocks,
        &pm,
        &mut core.NVIC,
        peripherals.SERCOM0,
        uart::Pads::<Sercom0>::default().rx(pins.pa11).tx(pins.pa10),
    );

    delay.delay_ms(250u16);
    red_led.set_high().unwrap();
    delay.delay_ms(250u16);
    red_led.set_low().unwrap();
    delay.delay_ms(250u16);
    red_led.set_high().unwrap();
    delay.delay_ms(250u16);
    red_led.set_low().unwrap();

    loop {
        const BANK_SELECT: u8 = 0;

        midi.poll(|event| {
            if let LiveEvent::Midi {
                channel,
                message: MidiMessage::Controller { controller, value },
            } = event
            {
                if channel == 0 && controller == BANK_SELECT {
                    if value == 0 {
                        red_led.set_low().unwrap();
                        unset.set_high().unwrap();
                        delay.delay_ms(10u8);
                        unset.set_low().unwrap();
                    } else if value == 1 {
                        red_led.set_high().unwrap();
                        set.set_high().unwrap();
                        delay.delay_ms(10u8);
                        set.set_low().unwrap();
                    }
                }
            }
        });
    }
}
