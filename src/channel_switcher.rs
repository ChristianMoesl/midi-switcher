use atsamd_hal::ehal::digital::v2::StatefulOutputPin;
use atsamd_hal::sleeping_delay::SleepingDelay;
use atsamd_hal::timer_traits::InterruptDrivenTimer;
use crate::_embedded_hal_blocking_delay_DelayMs;

pub enum Channel {
    ONE,
    TWO
}

pub struct ChannelSwitcher<Set: StatefulOutputPin, Unset: StatefulOutputPin, Timer: InterruptDrivenTimer> {
    set_pin: Set,
    unset_pin: Unset,
    delay: SleepingDelay<Timer>,
}

impl<Set: StatefulOutputPin, Unset: StatefulOutputPin, Timer: InterruptDrivenTimer> ChannelSwitcher<Set, Unset, Timer> {
    pub fn new(
        mut set_pin: Set,
        mut unset_pin: Unset,
        mut delay: SleepingDelay<Timer>
    ) -> Self {
        let mut switcher = Self {
            set_pin,
            unset_pin,
            delay
        };

        switcher.reset_pins();
        switcher
    }

    pub fn switch(&mut self, channel: Channel) {
        match channel {
           Channel::ONE => {
               self.turn_relais_on();
           },
           Channel::TWO => {
                self.turn_relais_off();
           }
        }

    }

    fn turn_relais_on(&mut self) {
        self.unset_pin.set_low().unwrap();
        self.set_pin.set_high().unwrap();
        self.switch_delay();
        self.reset_pins();
    }

    fn turn_relais_off(&mut self) {
        self.set_pin.set_low().unwrap();
        self.unset_pin.set_high().unwrap();
        self.switch_delay();
        self.reset_pins();
    }

    fn switch_delay(&mut self) {
        self.delay.delay_ms(20);
    }

    fn reset_pins(&mut self) {
        set_pin.set_low().unwrap();
        unset_pin.set_low().unwrap();
    }

    pub fn is_busy(&self) -> bool {
        self.set_pin.is_set_high().unwrap() || self.unset_pin.is_set_high().unwrap()
    }
}

