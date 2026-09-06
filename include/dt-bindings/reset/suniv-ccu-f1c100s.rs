//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/suniv-ccu-f1c100s.h
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
//
// Copyright (C) 2018 Icenowy Zheng <icenowy@aosc.xyz>
//
pub const RST_USB_PHY0: c_int = 0;
pub const RST_BUS_DMA: c_int = 1;
pub const RST_BUS_MMC0: c_int = 2;
pub const RST_BUS_MMC1: c_int = 3;
pub const RST_BUS_DRAM: c_int = 4;
pub const RST_BUS_SPI0: c_int = 5;
pub const RST_BUS_SPI1: c_int = 6;
pub const RST_BUS_OTG: c_int = 7;
pub const RST_BUS_VE: c_int = 8;
pub const RST_BUS_LCD: c_int = 9;
pub const RST_BUS_DEINTERLACE: c_int = 10;
pub const RST_BUS_CSI: c_int = 11;
pub const RST_BUS_TVD: c_int = 12;
pub const RST_BUS_TVE: c_int = 13;
pub const RST_BUS_DE_BE: c_int = 14;
pub const RST_BUS_DE_FE: c_int = 15;
pub const RST_BUS_CODEC: c_int = 16;
pub const RST_BUS_SPDIF: c_int = 17;
pub const RST_BUS_IR: c_int = 18;
pub const RST_BUS_RSB: c_int = 19;
pub const RST_BUS_I2S0: c_int = 20;
pub const RST_BUS_I2C0: c_int = 21;
pub const RST_BUS_I2C1: c_int = 22;
pub const RST_BUS_I2C2: c_int = 23;
pub const RST_BUS_UART0: c_int = 24;
pub const RST_BUS_UART1: c_int = 25;
pub const RST_BUS_UART2: c_int = 26;
