//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/lpc32xx-clock.h
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
// Copyright (c) 2015 Vladimir Zapolskiy <vz@mleia.com>
//
// This code is released using a dual license strategy: BSD/GPL
// You can choose the licence that better fits your requirements.
//
// Released under the terms of 3-clause BSD License
// Released under the terms of GNU General Public License Version 2.0
//
// LPC32XX System Control Block clocks
pub const LPC32XX_CLK_RTC: c_int = 1;
pub const LPC32XX_CLK_DMA: c_int = 2;
pub const LPC32XX_CLK_MLC: c_int = 3;
pub const LPC32XX_CLK_SLC: c_int = 4;
pub const LPC32XX_CLK_LCD: c_int = 5;
pub const LPC32XX_CLK_MAC: c_int = 6;
pub const LPC32XX_CLK_SD: c_int = 7;
pub const LPC32XX_CLK_DDRAM: c_int = 8;
pub const LPC32XX_CLK_SSP0: c_int = 9;
pub const LPC32XX_CLK_SSP1: c_int = 10;
pub const LPC32XX_CLK_UART3: c_int = 11;
pub const LPC32XX_CLK_UART4: c_int = 12;
pub const LPC32XX_CLK_UART5: c_int = 13;
pub const LPC32XX_CLK_UART6: c_int = 14;
pub const LPC32XX_CLK_IRDA: c_int = 15;
pub const LPC32XX_CLK_I2C1: c_int = 16;
pub const LPC32XX_CLK_I2C2: c_int = 17;
pub const LPC32XX_CLK_TIMER0: c_int = 18;
pub const LPC32XX_CLK_TIMER1: c_int = 19;
pub const LPC32XX_CLK_TIMER2: c_int = 20;
pub const LPC32XX_CLK_TIMER3: c_int = 21;
pub const LPC32XX_CLK_TIMER4: c_int = 22;
pub const LPC32XX_CLK_TIMER5: c_int = 23;
pub const LPC32XX_CLK_WDOG: c_int = 24;
pub const LPC32XX_CLK_I2S0: c_int = 25;
pub const LPC32XX_CLK_I2S1: c_int = 26;
pub const LPC32XX_CLK_SPI1: c_int = 27;
pub const LPC32XX_CLK_SPI2: c_int = 28;
pub const LPC32XX_CLK_MCPWM: c_int = 29;
pub const LPC32XX_CLK_HSTIMER: c_int = 30;
pub const LPC32XX_CLK_KEY: c_int = 31;
pub const LPC32XX_CLK_PWM1: c_int = 32;
pub const LPC32XX_CLK_PWM2: c_int = 33;
pub const LPC32XX_CLK_ADC: c_int = 34;
pub const LPC32XX_CLK_HCLK_PLL: c_int = 35;
pub const LPC32XX_CLK_PERIPH: c_int = 36;
// LPC32XX USB clocks
pub const LPC32XX_USB_CLK_I2C: c_int = 1;
pub const LPC32XX_USB_CLK_DEVICE: c_int = 2;
pub const LPC32XX_USB_CLK_HOST: c_int = 3;
