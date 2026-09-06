//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/mt7621-clk.h
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
// Author: Sergio Paracuellos <sergio.paracuellos@gmail.com>
//
pub const MT7621_CLK_XTAL: c_int = 0;
pub const MT7621_CLK_CPU: c_int = 1;
pub const MT7621_CLK_BUS: c_int = 2;
pub const MT7621_CLK_50M: c_int = 3;
pub const MT7621_CLK_125M: c_int = 4;
pub const MT7621_CLK_150M: c_int = 5;
pub const MT7621_CLK_250M: c_int = 6;
pub const MT7621_CLK_270M: c_int = 7;
pub const MT7621_CLK_HSDMA: c_int = 8;
pub const MT7621_CLK_FE: c_int = 9;
pub const MT7621_CLK_SP_DIVTX: c_int = 10;
pub const MT7621_CLK_TIMER: c_int = 11;
pub const MT7621_CLK_PCM: c_int = 12;
pub const MT7621_CLK_PIO: c_int = 13;
pub const MT7621_CLK_GDMA: c_int = 14;
pub const MT7621_CLK_NAND: c_int = 15;
pub const MT7621_CLK_I2C: c_int = 16;
pub const MT7621_CLK_I2S: c_int = 17;
pub const MT7621_CLK_SPI: c_int = 18;
pub const MT7621_CLK_UART1: c_int = 19;
pub const MT7621_CLK_UART2: c_int = 20;
pub const MT7621_CLK_UART3: c_int = 21;
pub const MT7621_CLK_ETH: c_int = 22;
pub const MT7621_CLK_PCIE0: c_int = 23;
pub const MT7621_CLK_PCIE1: c_int = 24;
pub const MT7621_CLK_PCIE2: c_int = 25;
pub const MT7621_CLK_CRYPTO: c_int = 26;
pub const MT7621_CLK_SHXC: c_int = 27;
pub const MT7621_CLK_MAX: c_int = 28;
