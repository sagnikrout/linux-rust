//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/mfd/stm32f7-rcc.h
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


// SPDX-License-Identifier: GPL-2.0
//
// This header provides constants for the STM32F7 RCC IP
//
// AHB1
pub const STM32F7_RCC_AHB1_GPIOA: c_int = 0;
pub const STM32F7_RCC_AHB1_GPIOB: c_int = 1;
pub const STM32F7_RCC_AHB1_GPIOC: c_int = 2;
pub const STM32F7_RCC_AHB1_GPIOD: c_int = 3;
pub const STM32F7_RCC_AHB1_GPIOE: c_int = 4;
pub const STM32F7_RCC_AHB1_GPIOF: c_int = 5;
pub const STM32F7_RCC_AHB1_GPIOG: c_int = 6;
pub const STM32F7_RCC_AHB1_GPIOH: c_int = 7;
pub const STM32F7_RCC_AHB1_GPIOI: c_int = 8;
pub const STM32F7_RCC_AHB1_GPIOJ: c_int = 9;
pub const STM32F7_RCC_AHB1_GPIOK: c_int = 10;
pub const STM32F7_RCC_AHB1_CRC: c_int = 12;
pub const STM32F7_RCC_AHB1_BKPSRAM: c_int = 18;
pub const STM32F7_RCC_AHB1_DTCMRAM: c_int = 20;
pub const STM32F7_RCC_AHB1_DMA1: c_int = 21;
pub const STM32F7_RCC_AHB1_DMA2: c_int = 22;
pub const STM32F7_RCC_AHB1_DMA2D: c_int = 23;
pub const STM32F7_RCC_AHB1_ETHMAC: c_int = 25;
pub const STM32F7_RCC_AHB1_ETHMACTX: c_int = 26;
pub const STM32F7_RCC_AHB1_ETHMACRX: c_int = 27;
pub const STM32FF_RCC_AHB1_ETHMACPTP: c_int = 28;
pub const STM32F7_RCC_AHB1_OTGHS: c_int = 29;
pub const STM32F7_RCC_AHB1_OTGHSULPI: c_int = 30;

// AHB2
pub const STM32F7_RCC_AHB2_DCMI: c_int = 0;
pub const STM32F7_RCC_AHB2_CRYP: c_int = 4;
pub const STM32F7_RCC_AHB2_HASH: c_int = 5;
pub const STM32F7_RCC_AHB2_RNG: c_int = 6;
pub const STM32F7_RCC_AHB2_OTGFS: c_int = 7;

// AHB3
pub const STM32F7_RCC_AHB3_FMC: c_int = 0;
pub const STM32F7_RCC_AHB3_QSPI: c_int = 1;

// APB1
pub const STM32F7_RCC_APB1_TIM2: c_int = 0;
pub const STM32F7_RCC_APB1_TIM3: c_int = 1;
pub const STM32F7_RCC_APB1_TIM4: c_int = 2;
pub const STM32F7_RCC_APB1_TIM5: c_int = 3;
pub const STM32F7_RCC_APB1_TIM6: c_int = 4;
pub const STM32F7_RCC_APB1_TIM7: c_int = 5;
pub const STM32F7_RCC_APB1_TIM12: c_int = 6;
pub const STM32F7_RCC_APB1_TIM13: c_int = 7;
pub const STM32F7_RCC_APB1_TIM14: c_int = 8;
pub const STM32F7_RCC_APB1_LPTIM1: c_int = 9;
pub const STM32F7_RCC_APB1_WWDG: c_int = 11;
pub const STM32F7_RCC_APB1_CAN3: c_int = 13;
pub const STM32F7_RCC_APB1_SPI2: c_int = 14;
pub const STM32F7_RCC_APB1_SPI3: c_int = 15;
pub const STM32F7_RCC_APB1_SPDIFRX: c_int = 16;
pub const STM32F7_RCC_APB1_UART2: c_int = 17;
pub const STM32F7_RCC_APB1_UART3: c_int = 18;
pub const STM32F7_RCC_APB1_UART4: c_int = 19;
pub const STM32F7_RCC_APB1_UART5: c_int = 20;
pub const STM32F7_RCC_APB1_I2C1: c_int = 21;
pub const STM32F7_RCC_APB1_I2C2: c_int = 22;
pub const STM32F7_RCC_APB1_I2C3: c_int = 23;
pub const STM32F7_RCC_APB1_I2C4: c_int = 24;
pub const STM32F7_RCC_APB1_CAN1: c_int = 25;
pub const STM32F7_RCC_APB1_CAN2: c_int = 26;
pub const STM32F7_RCC_APB1_CEC: c_int = 27;
pub const STM32F7_RCC_APB1_PWR: c_int = 28;
pub const STM32F7_RCC_APB1_DAC: c_int = 29;
pub const STM32F7_RCC_APB1_UART7: c_int = 30;
pub const STM32F7_RCC_APB1_UART8: c_int = 31;

// APB2
pub const STM32F7_RCC_APB2_TIM1: c_int = 0;
pub const STM32F7_RCC_APB2_TIM8: c_int = 1;
pub const STM32F7_RCC_APB2_USART1: c_int = 4;
pub const STM32F7_RCC_APB2_USART6: c_int = 5;
pub const STM32F7_RCC_APB2_SDMMC2: c_int = 7;
pub const STM32F7_RCC_APB2_ADC1: c_int = 8;
pub const STM32F7_RCC_APB2_ADC2: c_int = 9;
pub const STM32F7_RCC_APB2_ADC3: c_int = 10;
pub const STM32F7_RCC_APB2_SDMMC1: c_int = 11;
pub const STM32F7_RCC_APB2_SPI1: c_int = 12;
pub const STM32F7_RCC_APB2_SPI4: c_int = 13;
pub const STM32F7_RCC_APB2_SYSCFG: c_int = 14;
pub const STM32F7_RCC_APB2_TIM9: c_int = 16;
pub const STM32F7_RCC_APB2_TIM10: c_int = 17;
pub const STM32F7_RCC_APB2_TIM11: c_int = 18;
pub const STM32F7_RCC_APB2_SPI5: c_int = 20;
pub const STM32F7_RCC_APB2_SPI6: c_int = 21;
pub const STM32F7_RCC_APB2_SAI1: c_int = 22;
pub const STM32F7_RCC_APB2_SAI2: c_int = 23;
pub const STM32F7_RCC_APB2_LTDC: c_int = 26;
pub const STM32F7_RCC_APB2_DSI: c_int = 27;

