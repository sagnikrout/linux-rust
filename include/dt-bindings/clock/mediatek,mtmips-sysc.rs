//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/mediatek,mtmips-sysc.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Author: Sergio Paracuellos <sergio.paracuellos@gmail.com>
//
// Ralink RT-2880 clocks
pub const RT2880_CLK_XTAL: c_int = 0;
pub const RT2880_CLK_CPU: c_int = 1;
pub const RT2880_CLK_BUS: c_int = 2;
pub const RT2880_CLK_TIMER: c_int = 3;
pub const RT2880_CLK_WATCHDOG: c_int = 4;
pub const RT2880_CLK_UART: c_int = 5;
pub const RT2880_CLK_I2C: c_int = 6;
pub const RT2880_CLK_UARTLITE: c_int = 7;
pub const RT2880_CLK_ETHERNET: c_int = 8;
pub const RT2880_CLK_WMAC: c_int = 9;
// Ralink RT-305X clocks
pub const RT305X_CLK_XTAL: c_int = 0;
pub const RT305X_CLK_CPU: c_int = 1;
pub const RT305X_CLK_BUS: c_int = 2;
pub const RT305X_CLK_TIMER: c_int = 3;
pub const RT305X_CLK_WATCHDOG: c_int = 4;
pub const RT305X_CLK_UART: c_int = 5;
pub const RT305X_CLK_I2C: c_int = 6;
pub const RT305X_CLK_I2S: c_int = 7;
pub const RT305X_CLK_SPI1: c_int = 8;
pub const RT305X_CLK_SPI2: c_int = 9;
pub const RT305X_CLK_UARTLITE: c_int = 10;
pub const RT305X_CLK_ETHERNET: c_int = 11;
pub const RT305X_CLK_WMAC: c_int = 12;
// Ralink RT-3352 clocks
pub const RT3352_CLK_XTAL: c_int = 0;
pub const RT3352_CLK_CPU: c_int = 1;
pub const RT3352_CLK_PERIPH: c_int = 2;
pub const RT3352_CLK_BUS: c_int = 3;
pub const RT3352_CLK_TIMER: c_int = 4;
pub const RT3352_CLK_WATCHDOG: c_int = 5;
pub const RT3352_CLK_UART: c_int = 6;
pub const RT3352_CLK_I2C: c_int = 7;
pub const RT3352_CLK_I2S: c_int = 8;
pub const RT3352_CLK_SPI1: c_int = 9;
pub const RT3352_CLK_SPI2: c_int = 10;
pub const RT3352_CLK_UARTLITE: c_int = 11;
pub const RT3352_CLK_ETHERNET: c_int = 12;
pub const RT3352_CLK_WMAC: c_int = 13;
// Ralink RT-3883 clocks
pub const RT3883_CLK_XTAL: c_int = 0;
pub const RT3883_CLK_CPU: c_int = 1;
pub const RT3883_CLK_BUS: c_int = 2;
pub const RT3883_CLK_PERIPH: c_int = 3;
pub const RT3883_CLK_TIMER: c_int = 4;
pub const RT3883_CLK_WATCHDOG: c_int = 5;
pub const RT3883_CLK_UART: c_int = 6;
pub const RT3883_CLK_I2C: c_int = 7;
pub const RT3883_CLK_I2S: c_int = 8;
pub const RT3883_CLK_SPI1: c_int = 9;
pub const RT3883_CLK_SPI2: c_int = 10;
pub const RT3883_CLK_UARTLITE: c_int = 11;
pub const RT3883_CLK_ETHERNET: c_int = 12;
pub const RT3883_CLK_WMAC: c_int = 13;
// Ralink RT-5350 clocks
pub const RT5350_CLK_XTAL: c_int = 0;
pub const RT5350_CLK_CPU: c_int = 1;
pub const RT5350_CLK_BUS: c_int = 2;
pub const RT5350_CLK_PERIPH: c_int = 3;
pub const RT5350_CLK_TIMER: c_int = 4;
pub const RT5350_CLK_WATCHDOG: c_int = 5;
pub const RT5350_CLK_UART: c_int = 6;
pub const RT5350_CLK_I2C: c_int = 7;
pub const RT5350_CLK_I2S: c_int = 8;
pub const RT5350_CLK_SPI1: c_int = 9;
pub const RT5350_CLK_SPI2: c_int = 10;
pub const RT5350_CLK_UARTLITE: c_int = 11;
pub const RT5350_CLK_ETHERNET: c_int = 12;
pub const RT5350_CLK_WMAC: c_int = 13;
// Ralink MT-7620 clocks
pub const MT7620_CLK_XTAL: c_int = 0;
pub const MT7620_CLK_PLL: c_int = 1;
pub const MT7620_CLK_CPU: c_int = 2;
pub const MT7620_CLK_PERIPH: c_int = 3;
pub const MT7620_CLK_BUS: c_int = 4;
pub const MT7620_CLK_BBPPLL: c_int = 5;
pub const MT7620_CLK_SDHC: c_int = 6;
pub const MT7620_CLK_TIMER: c_int = 7;
pub const MT7620_CLK_WATCHDOG: c_int = 8;
pub const MT7620_CLK_UART: c_int = 9;
pub const MT7620_CLK_I2C: c_int = 10;
pub const MT7620_CLK_I2S: c_int = 11;
pub const MT7620_CLK_SPI1: c_int = 12;
pub const MT7620_CLK_SPI2: c_int = 13;
pub const MT7620_CLK_UARTLITE: c_int = 14;
pub const MT7620_CLK_MMC: c_int = 15;
pub const MT7620_CLK_WMAC: c_int = 16;
// Ralink MT-76X8 clocks
pub const MT76X8_CLK_XTAL: c_int = 0;
pub const MT76X8_CLK_CPU: c_int = 1;
pub const MT76X8_CLK_BBPPLL: c_int = 2;
pub const MT76X8_CLK_PCMI2S: c_int = 3;
pub const MT76X8_CLK_PERIPH: c_int = 4;
pub const MT76X8_CLK_BUS: c_int = 5;
pub const MT76X8_CLK_SDHC: c_int = 6;
pub const MT76X8_CLK_TIMER: c_int = 7;
pub const MT76X8_CLK_WATCHDOG: c_int = 8;
pub const MT76X8_CLK_I2C: c_int = 9;
pub const MT76X8_CLK_I2S: c_int = 10;
pub const MT76X8_CLK_SPI1: c_int = 11;
pub const MT76X8_CLK_SPI2: c_int = 12;
pub const MT76X8_CLK_UART0: c_int = 13;
pub const MT76X8_CLK_UART1: c_int = 14;
pub const MT76X8_CLK_UART2: c_int = 15;
pub const MT76X8_CLK_MMC: c_int = 16;
pub const MT76X8_CLK_WMAC: c_int = 17;
