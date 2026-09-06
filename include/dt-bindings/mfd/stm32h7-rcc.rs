//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/mfd/stm32h7-rcc.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


//
// This header provides constants for the STM32H7 RCC IP
//
// AHB3
pub const STM32H7_RCC_AHB3_MDMA: c_int = 0;
pub const STM32H7_RCC_AHB3_DMA2D: c_int = 4;
pub const STM32H7_RCC_AHB3_JPGDEC: c_int = 5;
pub const STM32H7_RCC_AHB3_FMC: c_int = 12;
pub const STM32H7_RCC_AHB3_QUADSPI: c_int = 14;
pub const STM32H7_RCC_AHB3_SDMMC1: c_int = 16;
pub const STM32H7_RCC_AHB3_CPU: c_int = 31;

// AHB1
pub const STM32H7_RCC_AHB1_DMA1: c_int = 0;
pub const STM32H7_RCC_AHB1_DMA2: c_int = 1;
pub const STM32H7_RCC_AHB1_ADC12: c_int = 5;
pub const STM32H7_RCC_AHB1_ART: c_int = 14;
pub const STM32H7_RCC_AHB1_ETH1MAC: c_int = 15;
pub const STM32H7_RCC_AHB1_USB1OTG: c_int = 25;
pub const STM32H7_RCC_AHB1_USB2OTG: c_int = 27;

// AHB2
pub const STM32H7_RCC_AHB2_CAMITF: c_int = 0;
pub const STM32H7_RCC_AHB2_CRYPT: c_int = 4;
pub const STM32H7_RCC_AHB2_HASH: c_int = 5;
pub const STM32H7_RCC_AHB2_RNG: c_int = 6;
pub const STM32H7_RCC_AHB2_SDMMC2: c_int = 9;

// AHB4
pub const STM32H7_RCC_AHB4_GPIOA: c_int = 0;
pub const STM32H7_RCC_AHB4_GPIOB: c_int = 1;
pub const STM32H7_RCC_AHB4_GPIOC: c_int = 2;
pub const STM32H7_RCC_AHB4_GPIOD: c_int = 3;
pub const STM32H7_RCC_AHB4_GPIOE: c_int = 4;
pub const STM32H7_RCC_AHB4_GPIOF: c_int = 5;
pub const STM32H7_RCC_AHB4_GPIOG: c_int = 6;
pub const STM32H7_RCC_AHB4_GPIOH: c_int = 7;
pub const STM32H7_RCC_AHB4_GPIOI: c_int = 8;
pub const STM32H7_RCC_AHB4_GPIOJ: c_int = 9;
pub const STM32H7_RCC_AHB4_GPIOK: c_int = 10;
pub const STM32H7_RCC_AHB4_CRC: c_int = 19;
pub const STM32H7_RCC_AHB4_BDMA: c_int = 21;
pub const STM32H7_RCC_AHB4_ADC3: c_int = 24;
pub const STM32H7_RCC_AHB4_HSEM: c_int = 25;

// APB3
pub const STM32H7_RCC_APB3_LTDC: c_int = 3;
pub const STM32H7_RCC_APB3_DSI: c_int = 4;

// APB1L
pub const STM32H7_RCC_APB1L_TIM2: c_int = 0;
pub const STM32H7_RCC_APB1L_TIM3: c_int = 1;
pub const STM32H7_RCC_APB1L_TIM4: c_int = 2;
pub const STM32H7_RCC_APB1L_TIM5: c_int = 3;
pub const STM32H7_RCC_APB1L_TIM6: c_int = 4;
pub const STM32H7_RCC_APB1L_TIM7: c_int = 5;
pub const STM32H7_RCC_APB1L_TIM12: c_int = 6;
pub const STM32H7_RCC_APB1L_TIM13: c_int = 7;
pub const STM32H7_RCC_APB1L_TIM14: c_int = 8;
pub const STM32H7_RCC_APB1L_LPTIM1: c_int = 9;
pub const STM32H7_RCC_APB1L_SPI2: c_int = 14;
pub const STM32H7_RCC_APB1L_SPI3: c_int = 15;
pub const STM32H7_RCC_APB1L_SPDIF_RX: c_int = 16;
pub const STM32H7_RCC_APB1L_USART2: c_int = 17;
pub const STM32H7_RCC_APB1L_USART3: c_int = 18;
pub const STM32H7_RCC_APB1L_UART4: c_int = 19;
pub const STM32H7_RCC_APB1L_UART5: c_int = 20;
pub const STM32H7_RCC_APB1L_I2C1: c_int = 21;
pub const STM32H7_RCC_APB1L_I2C2: c_int = 22;
pub const STM32H7_RCC_APB1L_I2C3: c_int = 23;
pub const STM32H7_RCC_APB1L_HDMICEC: c_int = 27;
pub const STM32H7_RCC_APB1L_DAC12: c_int = 29;
pub const STM32H7_RCC_APB1L_USART7: c_int = 30;
pub const STM32H7_RCC_APB1L_USART8: c_int = 31;

// APB1H
pub const STM32H7_RCC_APB1H_CRS: c_int = 1;
pub const STM32H7_RCC_APB1H_SWP: c_int = 2;
pub const STM32H7_RCC_APB1H_OPAMP: c_int = 4;
pub const STM32H7_RCC_APB1H_MDIOS: c_int = 5;
pub const STM32H7_RCC_APB1H_FDCAN: c_int = 8;

// APB2
pub const STM32H7_RCC_APB2_TIM1: c_int = 0;
pub const STM32H7_RCC_APB2_TIM8: c_int = 1;
pub const STM32H7_RCC_APB2_USART1: c_int = 4;
pub const STM32H7_RCC_APB2_USART6: c_int = 5;
pub const STM32H7_RCC_APB2_SPI1: c_int = 12;
pub const STM32H7_RCC_APB2_SPI4: c_int = 13;
pub const STM32H7_RCC_APB2_TIM15: c_int = 16;
pub const STM32H7_RCC_APB2_TIM16: c_int = 17;
pub const STM32H7_RCC_APB2_TIM17: c_int = 18;
pub const STM32H7_RCC_APB2_SPI5: c_int = 20;
pub const STM32H7_RCC_APB2_SAI1: c_int = 22;
pub const STM32H7_RCC_APB2_SAI2: c_int = 23;
pub const STM32H7_RCC_APB2_SAI3: c_int = 24;
pub const STM32H7_RCC_APB2_DFSDM1: c_int = 28;
pub const STM32H7_RCC_APB2_HRTIM: c_int = 29;

// APB4
pub const STM32H7_RCC_APB4_SYSCFG: c_int = 1;
pub const STM32H7_RCC_APB4_LPUART1: c_int = 3;
pub const STM32H7_RCC_APB4_SPI6: c_int = 5;
pub const STM32H7_RCC_APB4_I2C4: c_int = 7;
pub const STM32H7_RCC_APB4_LPTIM2: c_int = 9;
pub const STM32H7_RCC_APB4_LPTIM3: c_int = 10;
pub const STM32H7_RCC_APB4_LPTIM4: c_int = 11;
pub const STM32H7_RCC_APB4_LPTIM5: c_int = 12;
pub const STM32H7_RCC_APB4_COMP12: c_int = 14;
pub const STM32H7_RCC_APB4_VREF: c_int = 15;
pub const STM32H7_RCC_APB4_SAI4: c_int = 21;
pub const STM32H7_RCC_APB4_TMPSENS: c_int = 26;

