//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/marvell,pxa910.h
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
// fixed clocks and plls
pub const PXA910_CLK_CLK32: c_int = 1;
pub const PXA910_CLK_VCTCXO: c_int = 2;
pub const PXA910_CLK_PLL1: c_int = 3;
pub const PXA910_CLK_PLL1_2: c_int = 8;
pub const PXA910_CLK_PLL1_4: c_int = 9;
pub const PXA910_CLK_PLL1_8: c_int = 10;
pub const PXA910_CLK_PLL1_16: c_int = 11;
pub const PXA910_CLK_PLL1_6: c_int = 12;
pub const PXA910_CLK_PLL1_12: c_int = 13;
pub const PXA910_CLK_PLL1_24: c_int = 14;
pub const PXA910_CLK_PLL1_48: c_int = 15;
pub const PXA910_CLK_PLL1_96: c_int = 16;
pub const PXA910_CLK_PLL1_13: c_int = 17;
pub const PXA910_CLK_PLL1_13_1_5: c_int = 18;
pub const PXA910_CLK_PLL1_2_1_5: c_int = 19;
pub const PXA910_CLK_PLL1_3_16: c_int = 20;
pub const PXA910_CLK_PLL1_192: c_int = 21;
pub const PXA910_CLK_UART_PLL: c_int = 27;
pub const PXA910_CLK_USB_PLL: c_int = 28;
// apb peripherals
pub const PXA910_CLK_TWSI0: c_int = 60;
pub const PXA910_CLK_TWSI1: c_int = 61;
pub const PXA910_CLK_TWSI2: c_int = 62;
pub const PXA910_CLK_TWSI3: c_int = 63;
pub const PXA910_CLK_GPIO: c_int = 64;
pub const PXA910_CLK_KPC: c_int = 65;
pub const PXA910_CLK_RTC: c_int = 66;
pub const PXA910_CLK_PWM0: c_int = 67;
pub const PXA910_CLK_PWM1: c_int = 68;
pub const PXA910_CLK_PWM2: c_int = 69;
pub const PXA910_CLK_PWM3: c_int = 70;
pub const PXA910_CLK_UART0: c_int = 71;
pub const PXA910_CLK_UART1: c_int = 72;
pub const PXA910_CLK_UART2: c_int = 73;
pub const PXA910_CLK_SSP0: c_int = 74;
pub const PXA910_CLK_SSP1: c_int = 75;
pub const PXA910_CLK_TIMER0: c_int = 76;
pub const PXA910_CLK_TIMER1: c_int = 77;
// axi peripherals
pub const PXA910_CLK_DFC: c_int = 100;
pub const PXA910_CLK_SDH0: c_int = 101;
pub const PXA910_CLK_SDH1: c_int = 102;
pub const PXA910_CLK_SDH2: c_int = 103;
pub const PXA910_CLK_USB: c_int = 104;
pub const PXA910_CLK_SPH: c_int = 105;
pub const PXA910_CLK_DISP0: c_int = 106;
pub const PXA910_CLK_CCIC0: c_int = 107;
pub const PXA910_CLK_CCIC0_PHY: c_int = 108;
pub const PXA910_CLK_CCIC0_SPHY: c_int = 109;
