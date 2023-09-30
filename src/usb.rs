use atsamd_hal::{
    clock::GenericClockController,
    gpio::{AnyPin, PA24, PA25},
    pac::{self, interrupt, NVIC},
    usb::UsbBus,
};
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

pub fn init_usb(
    nvic: &mut NVIC,
    power_managmenet: &mut pac::PM,
    usb_peripheral: pac::USB,
    clocks: &mut GenericClockController,
    dm: impl AnyPin<Id = PA24>,
    dp: impl AnyPin<Id = PA25>,
) {
    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(usb_allocator(
            usb_peripheral,
            clocks,
            power_managmenet,
            dm,
            dp,
        ));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_SERIAL = Some(SerialPort::new(bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("Fake company")
                .product("Serial port")
                .serial_number("TEST")
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }

    unsafe {
        nvic.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }
}

fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: impl AnyPin<Id = PA24>,
    dp: impl AnyPin<Id = PA25>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.usb(&gclk0).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(clock, pm, dm, dp, usb))
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

pub fn write(data: &[u8]) {
    unsafe {
        if let Some(serial) = USB_SERIAL.as_mut() {
            serial.write(data).ok();
        }
    }
}

fn poll_usb() {
    unsafe {
        if let Some(usb_dev) = USB_BUS.as_mut() {
            if let Some(serial) = USB_SERIAL.as_mut() {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    serial.write("got data\n".as_bytes()).ok();
                    for (i, c) in buf.iter().enumerate() {
                        if i >= count {
                            break;
                        }
                        serial.write(&[*c]).ok();
                    }
                };
            };
        };
    };
}

#[interrupt]
fn USB() {
    poll_usb();
}
