use core::sync::atomic::{self, AtomicBool, AtomicU8, Ordering};

use atsamd_hal::{
    clock::GenericClockController,
    pac::{interrupt, NVIC, SERCOM0},
    sercom::uart::{
        self, BaudMode, BitOrder, Clock, EightBit, Flags, Oversampling, Parity, StopBits, ValidPads,
    },
    time::U32Ext,
};
use midly::{live::LiveEvent, stream::MidiStream};

midly::stack_buffer! {
     #[repr(C)]
     pub struct StaticBuffer([u8; 16]);
}

/// Shared atomic between SERCOM0 interrupt and midi module
static INTERRUPT_FIRED: AtomicBool = AtomicBool::new(false);
static RECEIVED_BYTE: AtomicU8 = AtomicU8::new(0);

pub struct Midi {
    midi_stream: MidiStream<StaticBuffer>,
}

impl Midi {
    pub fn new<P: ValidPads>(
        clocks: &mut GenericClockController,
        pm: &Clock,
        nvic: &mut NVIC,
        sercom: P::Sercom,
        pads: P,
    ) -> Self {
        let gclk0 = clocks.gclk0();
        let clock = &clocks.sercom0_core(&gclk0).unwrap();
        let mut uart = uart::Config::new(pm, sercom, pads, clock.freq())
            .baud(31250_u32.hz(), BaudMode::Fractional(Oversampling::Bits16))
            .char_size::<EightBit>()
            .bit_order(BitOrder::LsbFirst)
            .stop_bits(StopBits::OneBit)
            .parity(Parity::None)
            .enable();

        // enable interrupts
        uart.enable_interrupts(Flags::RXC);

        unsafe {
            nvic.set_priority(interrupt::SERCOM0, 2);
            NVIC::unmask(interrupt::SERCOM0);
        }

        Self {
            midi_stream: MidiStream::default(),
        }
    }

    pub fn poll(&mut self, mut handle_event: impl FnMut(LiveEvent)) {
        if INTERRUPT_FIRED.load(Ordering::Relaxed) {
            let byte = RECEIVED_BYTE.load(Ordering::Relaxed);

            INTERRUPT_FIRED.store(false, Ordering::Relaxed);

            self.midi_stream.feed(&[byte], handle_event);
        }
    }
}

#[interrupt]
fn SERCOM0() {
    let data = unsafe { SERCOM0::ptr().as_ref().unwrap().usart().data.read().bits() };

    INTERRUPT_FIRED.store(true, atomic::Ordering::Relaxed);
    RECEIVED_BYTE.store(data as u8, Ordering::Relaxed);
}
