//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/alphascale,asm9260.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2014 Oleksij Rempel <linux@rempel-privat.de>
//
// ahb gate
pub const CLKID_AHB_ROM: c_int = 0;
pub const CLKID_AHB_RAM: c_int = 1;
pub const CLKID_AHB_GPIO: c_int = 2;
pub const CLKID_AHB_MAC: c_int = 3;
pub const CLKID_AHB_EMI: c_int = 4;
pub const CLKID_AHB_USB0: c_int = 5;
pub const CLKID_AHB_USB1: c_int = 6;
pub const CLKID_AHB_DMA0: c_int = 7;
pub const CLKID_AHB_DMA1: c_int = 8;
pub const CLKID_AHB_UART0: c_int = 9;
pub const CLKID_AHB_UART1: c_int = 10;
pub const CLKID_AHB_UART2: c_int = 11;
pub const CLKID_AHB_UART3: c_int = 12;
pub const CLKID_AHB_UART4: c_int = 13;
pub const CLKID_AHB_UART5: c_int = 14;
pub const CLKID_AHB_UART6: c_int = 15;
pub const CLKID_AHB_UART7: c_int = 16;
pub const CLKID_AHB_UART8: c_int = 17;
pub const CLKID_AHB_UART9: c_int = 18;
pub const CLKID_AHB_I2S0: c_int = 19;
pub const CLKID_AHB_I2C0: c_int = 20;
pub const CLKID_AHB_I2C1: c_int = 21;
pub const CLKID_AHB_SSP0: c_int = 22;
pub const CLKID_AHB_IOCONFIG: c_int = 23;
pub const CLKID_AHB_WDT: c_int = 24;
pub const CLKID_AHB_CAN0: c_int = 25;
pub const CLKID_AHB_CAN1: c_int = 26;
pub const CLKID_AHB_MPWM: c_int = 27;
pub const CLKID_AHB_SPI0: c_int = 28;
pub const CLKID_AHB_SPI1: c_int = 29;
pub const CLKID_AHB_QEI: c_int = 30;
pub const CLKID_AHB_QUADSPI0: c_int = 31;
pub const CLKID_AHB_CAMIF: c_int = 32;
pub const CLKID_AHB_LCDIF: c_int = 33;
pub const CLKID_AHB_TIMER0: c_int = 34;
pub const CLKID_AHB_TIMER1: c_int = 35;
pub const CLKID_AHB_TIMER2: c_int = 36;
pub const CLKID_AHB_TIMER3: c_int = 37;
pub const CLKID_AHB_IRQ: c_int = 38;
pub const CLKID_AHB_RTC: c_int = 39;
pub const CLKID_AHB_NAND: c_int = 40;
pub const CLKID_AHB_ADC0: c_int = 41;
pub const CLKID_AHB_LED: c_int = 42;
pub const CLKID_AHB_DAC0: c_int = 43;
pub const CLKID_AHB_LCD: c_int = 44;
pub const CLKID_AHB_I2S1: c_int = 45;
pub const CLKID_AHB_MAC1: c_int = 46;
// divider
pub const CLKID_SYS_CPU: c_int = 47;
pub const CLKID_SYS_AHB: c_int = 48;
pub const CLKID_SYS_I2S0M: c_int = 49;
pub const CLKID_SYS_I2S0S: c_int = 50;
pub const CLKID_SYS_I2S1M: c_int = 51;
pub const CLKID_SYS_I2S1S: c_int = 52;
pub const CLKID_SYS_UART0: c_int = 53;
pub const CLKID_SYS_UART1: c_int = 54;
pub const CLKID_SYS_UART2: c_int = 55;
pub const CLKID_SYS_UART3: c_int = 56;
pub const CLKID_SYS_UART4: c_int = 56;
pub const CLKID_SYS_UART5: c_int = 57;
pub const CLKID_SYS_UART6: c_int = 58;
pub const CLKID_SYS_UART7: c_int = 59;
pub const CLKID_SYS_UART8: c_int = 60;
pub const CLKID_SYS_UART9: c_int = 61;
pub const CLKID_SYS_SPI0: c_int = 62;
pub const CLKID_SYS_SPI1: c_int = 63;
pub const CLKID_SYS_QUADSPI: c_int = 64;
pub const CLKID_SYS_SSP0: c_int = 65;
pub const CLKID_SYS_NAND: c_int = 66;
pub const CLKID_SYS_TRACE: c_int = 67;
pub const CLKID_SYS_CAMM: c_int = 68;
pub const CLKID_SYS_WDT: c_int = 69;
pub const CLKID_SYS_CLKOUT: c_int = 70;
pub const CLKID_SYS_MAC: c_int = 71;
pub const CLKID_SYS_LCD: c_int = 72;
pub const CLKID_SYS_ADCANA: c_int = 73;
pub const MAX_CLKS: c_int = 74;
