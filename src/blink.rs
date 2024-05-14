#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use mcx_pac::pac;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, 48_000_000u32);

    pac::instance::SYSCON0.ahbclkctrl0().modify(|r| {
        r.set_gpio0(pac::syscon_nx4x::enumm::Gpio0::ENABLE);
        r.set_port0(pac::syscon_nx4x::enumm::Port0::ENABLE);
    });
    pac::instance::GPIO0.pddr().modify(|r| {
        r.set_pdd10(pac::gpio::enumm::Pdd10::PDD1);
    });

    loop {
        delay.delay_ms(1000u32);
        pac::instance::GPIO0.ptor().modify(|r| {
            r.set_ptto10(pac::gpio::enumm::Ptto10::PTTO1);
        })
    }
}
