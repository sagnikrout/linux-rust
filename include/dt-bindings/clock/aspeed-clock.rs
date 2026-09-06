//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/aspeed-clock.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR MIT)
pub const ASPEED_CLK_GATE_ECLK: c_int = 0;
pub const ASPEED_CLK_GATE_GCLK: c_int = 1;
pub const ASPEED_CLK_GATE_MCLK: c_int = 2;
pub const ASPEED_CLK_GATE_VCLK: c_int = 3;
pub const ASPEED_CLK_GATE_BCLK: c_int = 4;
pub const ASPEED_CLK_GATE_DCLK: c_int = 5;
pub const ASPEED_CLK_GATE_REFCLK: c_int = 6;
pub const ASPEED_CLK_GATE_USBPORT2CLK: c_int = 7;
pub const ASPEED_CLK_GATE_LCLK: c_int = 8;
pub const ASPEED_CLK_GATE_USBUHCICLK: c_int = 9;
pub const ASPEED_CLK_GATE_D1CLK: c_int = 10;
pub const ASPEED_CLK_GATE_YCLK: c_int = 11;
pub const ASPEED_CLK_GATE_USBPORT1CLK: c_int = 12;
pub const ASPEED_CLK_GATE_UART1CLK: c_int = 13;
pub const ASPEED_CLK_GATE_UART2CLK: c_int = 14;
pub const ASPEED_CLK_GATE_UART5CLK: c_int = 15;
pub const ASPEED_CLK_GATE_ESPICLK: c_int = 16;
pub const ASPEED_CLK_GATE_MAC1CLK: c_int = 17;
pub const ASPEED_CLK_GATE_MAC2CLK: c_int = 18;
pub const ASPEED_CLK_GATE_RSACLK: c_int = 19;
pub const ASPEED_CLK_GATE_UART3CLK: c_int = 20;
pub const ASPEED_CLK_GATE_UART4CLK: c_int = 21;
pub const ASPEED_CLK_GATE_SDCLK: c_int = 22;
pub const ASPEED_CLK_GATE_LHCCLK: c_int = 23;
pub const ASPEED_CLK_HPLL: c_int = 24;
pub const ASPEED_CLK_AHB: c_int = 25;
pub const ASPEED_CLK_APB: c_int = 26;
pub const ASPEED_CLK_UART: c_int = 27;
pub const ASPEED_CLK_SDIO: c_int = 28;
pub const ASPEED_CLK_ECLK: c_int = 29;
pub const ASPEED_CLK_ECLK_MUX: c_int = 30;
pub const ASPEED_CLK_LHCLK: c_int = 31;
pub const ASPEED_CLK_MAC: c_int = 32;
pub const ASPEED_CLK_BCLK: c_int = 33;
pub const ASPEED_CLK_MPLL: c_int = 34;
pub const ASPEED_CLK_24M: c_int = 35;
pub const ASPEED_CLK_MAC1RCLK: c_int = 36;
pub const ASPEED_CLK_MAC2RCLK: c_int = 37;
pub const ASPEED_RESET_XDMA: c_int = 0;
pub const ASPEED_RESET_MCTP: c_int = 1;
pub const ASPEED_RESET_ADC: c_int = 2;
pub const ASPEED_RESET_JTAG_MASTER: c_int = 3;
pub const ASPEED_RESET_MIC: c_int = 4;
pub const ASPEED_RESET_PWM: c_int = 5;
pub const ASPEED_RESET_PECI: c_int = 6;
pub const ASPEED_RESET_I2C: c_int = 7;
pub const ASPEED_RESET_AHB: c_int = 8;
pub const ASPEED_RESET_CRT1: c_int = 9;
pub const ASPEED_RESET_HACE: c_int = 10;
pub const ASPEED_RESET_VIDEO: c_int = 11;
