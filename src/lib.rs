#![no_std]

extern crate volatile_register;

pub mod afio;
pub mod btim;
pub mod gpio;
pub mod rcc;
pub mod usart;

use afio::Afio;
use btim::BTim;
use gpio::Gpio;
use rcc::Rcc;
use usart::Usart;

// const FSMC: usize = 0xa0000000;
// const PWR: usize = 0x40007000;
const RCC: usize = 0x40021000;
const GPIOA: usize = 0x40010800;
const GPIOB: usize = 0x40010c00;
const GPIOC: usize = 0x40011000;
const GPIOD: usize = 0x40011400;
const GPIOE: usize = 0x40011800;
const GPIOF: usize = 0x40011c00;
const GPIOG: usize = 0x40012000;
const AFIO: usize = 0x40010000;
// const EXTI: usize = 0x40010400;
// const DMA1: usize = 0x40020000;
// const DMA2: usize = 0x40020400;
// const RTC: usize = 0x40002800;
// const BKP: usize = 0x40006c04;
// const IWDG: usize = 0x40003000;
// const WWDG: usize = 0x40002c00;
// const TIM1: usize = 0x40012c00;
// const TIM2: usize = 0x40000000;
// const TIM3: usize = 0x40000400;
// const TIM4: usize = 0x40000800;
// const TIM5: usize = 0x40000c00;
// const TIM12: usize = 0x40001800;
// const TIM13: usize = 0x40001c00;
// const TIM14: usize = 0x40002000;
const TIM6: usize = 0x40001000;
const TIM7: usize = 0x40001400;
// const I2C1: usize = 0x40005400;
// const I2C2: usize = 0x40005800;
// const SPI1: usize = 0x40013000;
// const SPI2: usize = 0x40003800;
// const SPI3: usize = 0x40003c00;
const USART1: usize = 0x40013800;
// const USART2: usize = 0x40004400;
// const USART3: usize = 0x40004800;
// const ADC1: usize = 0x40012400;
// const DAC: usize = 0x40007400;
// const DBG: usize = 0xe0042000;
// const UART4: usize = 0x40004c00;
// const UART5: usize = 0x40005000;
// const CRC: usize = 0x40023000;
// const FLASH: usize = 0x40022000;
// const TIM15: usize = 0x40014000;
// const TIM16: usize = 0x40014400;
// const TIM17: usize = 0x40014800;
// const CEC: usize = 0x40007800;
// const NVIC: usize = 0xe000e000;

pub fn afio() -> &'static Afio {
    unsafe { deref(AFIO) }
}

pub unsafe fn afio_mut() -> &'static mut Afio {
    deref_mut(AFIO)
}

pub fn gpioa() -> &'static Gpio {
    unsafe { deref(GPIOA) }
}

pub unsafe fn gpioa_mut() -> &'static mut Gpio {
    deref_mut(GPIOA)
}

pub fn gpiob() -> &'static Gpio {
    unsafe { deref(GPIOB) }
}

pub unsafe fn gpiob_mut() -> &'static mut Gpio {
    deref_mut(GPIOB)
}

pub fn gpioc() -> &'static Gpio {
    unsafe { deref(GPIOC) }
}

pub unsafe fn gpioc_mut() -> &'static mut Gpio {
    deref_mut(GPIOC)
}

pub fn gpiod() -> &'static Gpio {
    unsafe { deref(GPIOD) }
}

pub unsafe fn gpiod_mut() -> &'static mut Gpio {
    deref_mut(GPIOD)
}

pub fn gpioe() -> &'static Gpio {
    unsafe { deref(GPIOE) }
}

pub unsafe fn gpioe_mut() -> &'static mut Gpio {
    deref_mut(GPIOE)
}

pub fn gpiof() -> &'static Gpio {
    unsafe { deref(GPIOF) }
}

pub unsafe fn gpiof_mut() -> &'static mut Gpio {
    deref_mut(GPIOF)
}

pub fn gpiog() -> &'static Gpio {
    unsafe { deref(GPIOG) }
}

pub unsafe fn gpiog_mut() -> &'static mut Gpio {
    deref_mut(GPIOG)
}

pub fn rcc() -> &'static Rcc {
    unsafe { deref(RCC) }
}

pub unsafe fn rcc_mut() -> &'static mut Rcc {
    deref_mut(RCC)
}

pub fn tim6() -> &'static BTim {
    unsafe { deref(TIM6) }
}

pub unsafe fn tim6_mut() -> &'static mut BTim {
    deref_mut(TIM6)
}

pub fn tim7() -> &'static BTim {
    unsafe { deref(TIM7) }
}

pub unsafe fn tim7_mut() -> &'static mut BTim {
    deref_mut(TIM7)
}

pub fn usart1() -> &'static Usart {
    unsafe { deref(USART1) }
}

pub unsafe fn usart1_mut() -> &'static mut Usart {
    deref_mut(USART1)
}

unsafe fn deref<T>(address: usize) -> &'static T {
    &*(address as *const T)
}

unsafe fn deref_mut<T>(address: usize) -> &'static mut T {
    &mut *(address as *mut T)
}
