//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/mt7621-reset.h
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
// Copyright (c) 2021 Sergio Paracuellos
// Author: Sergio Paracuellos <sergio.paracuellos@gmail.com>
//
pub const MT7621_RST_SYS: c_int = 0;
pub const MT7621_RST_MCM: c_int = 2;
pub const MT7621_RST_HSDMA: c_int = 5;
pub const MT7621_RST_FE: c_int = 6;
pub const MT7621_RST_SPDIFTX: c_int = 7;
pub const MT7621_RST_TIMER: c_int = 8;
pub const MT7621_RST_INT: c_int = 9;
pub const MT7621_RST_MC: c_int = 10;
pub const MT7621_RST_PCM: c_int = 11;
pub const MT7621_RST_PIO: c_int = 13;
pub const MT7621_RST_GDMA: c_int = 14;
pub const MT7621_RST_NFI: c_int = 15;
pub const MT7621_RST_I2C: c_int = 16;
pub const MT7621_RST_I2S: c_int = 17;
pub const MT7621_RST_SPI: c_int = 18;
pub const MT7621_RST_UART1: c_int = 19;
pub const MT7621_RST_UART2: c_int = 20;
pub const MT7621_RST_UART3: c_int = 21;
pub const MT7621_RST_ETH: c_int = 23;
pub const MT7621_RST_PCIE0: c_int = 24;
pub const MT7621_RST_PCIE1: c_int = 25;
pub const MT7621_RST_PCIE2: c_int = 26;
pub const MT7621_RST_AUX_STCK: c_int = 28;
pub const MT7621_RST_CRYPTO: c_int = 29;
pub const MT7621_RST_SDXC: c_int = 30;
pub const MT7621_RST_PPE: c_int = 31;
