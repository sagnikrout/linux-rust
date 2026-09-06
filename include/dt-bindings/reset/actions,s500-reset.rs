//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/actions,s500-reset.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Device Tree binding constants for Actions Semi S500 Reset Management Unit
//
// Copyright (c) 2014 Actions Semi Inc.
// Copyright (c) 2020 Cristian Ciocaltea <cristian.ciocaltea@gmail.com>
//
pub const RESET_DMAC: c_int = 0;
pub const RESET_NORIF: c_int = 1;
pub const RESET_DDR: c_int = 2;
pub const RESET_NANDC: c_int = 3;
pub const RESET_SD0: c_int = 4;
pub const RESET_SD1: c_int = 5;
pub const RESET_PCM1: c_int = 6;
pub const RESET_DE: c_int = 7;
pub const RESET_LCD: c_int = 8;
pub const RESET_SD2: c_int = 9;
pub const RESET_DSI: c_int = 10;
pub const RESET_CSI: c_int = 11;
pub const RESET_BISP: c_int = 12;
pub const RESET_KEY: c_int = 13;
pub const RESET_GPIO: c_int = 14;
pub const RESET_AUDIO: c_int = 15;
pub const RESET_PCM0: c_int = 16;
pub const RESET_VDE: c_int = 17;
pub const RESET_VCE: c_int = 18;
pub const RESET_GPU3D: c_int = 19;
pub const RESET_NIC301: c_int = 20;
pub const RESET_LENS: c_int = 21;
pub const RESET_PERIPHRESET: c_int = 22;
pub const RESET_USB2_0: c_int = 23;
pub const RESET_TVOUT: c_int = 24;
pub const RESET_HDMI: c_int = 25;
pub const RESET_HDCP2TX: c_int = 26;
pub const RESET_UART6: c_int = 27;
pub const RESET_UART0: c_int = 28;
pub const RESET_UART1: c_int = 29;
pub const RESET_UART2: c_int = 30;
pub const RESET_SPI0: c_int = 31;
pub const RESET_SPI1: c_int = 32;
pub const RESET_SPI2: c_int = 33;
pub const RESET_SPI3: c_int = 34;
pub const RESET_I2C0: c_int = 35;
pub const RESET_I2C1: c_int = 36;
pub const RESET_USB3: c_int = 37;
pub const RESET_UART3: c_int = 38;
pub const RESET_UART4: c_int = 39;
pub const RESET_UART5: c_int = 40;
pub const RESET_I2C2: c_int = 41;
pub const RESET_I2C3: c_int = 42;
pub const RESET_ETHERNET: c_int = 43;
pub const RESET_CHIPID: c_int = 44;
pub const RESET_USB2_1: c_int = 45;
pub const RESET_WD0RESET: c_int = 46;
pub const RESET_WD1RESET: c_int = 47;
pub const RESET_WD2RESET: c_int = 48;
pub const RESET_WD3RESET: c_int = 49;
pub const RESET_DBG0RESET: c_int = 50;
pub const RESET_DBG1RESET: c_int = 51;
pub const RESET_DBG2RESET: c_int = 52;
pub const RESET_DBG3RESET: c_int = 53;
