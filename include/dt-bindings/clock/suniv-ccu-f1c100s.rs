//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/suniv-ccu-f1c100s.h
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
// Copyright (c) 2018 Icenowy Zheng <icenowy@aosc.xyz>
//
pub const CLK_CPU: c_int = 11;
pub const CLK_BUS_DMA: c_int = 14;
pub const CLK_BUS_MMC0: c_int = 15;
pub const CLK_BUS_MMC1: c_int = 16;
pub const CLK_BUS_DRAM: c_int = 17;
pub const CLK_BUS_SPI0: c_int = 18;
pub const CLK_BUS_SPI1: c_int = 19;
pub const CLK_BUS_OTG: c_int = 20;
pub const CLK_BUS_VE: c_int = 21;
pub const CLK_BUS_LCD: c_int = 22;
pub const CLK_BUS_DEINTERLACE: c_int = 23;
pub const CLK_BUS_CSI: c_int = 24;
pub const CLK_BUS_TVD: c_int = 25;
pub const CLK_BUS_TVE: c_int = 26;
pub const CLK_BUS_DE_BE: c_int = 27;
pub const CLK_BUS_DE_FE: c_int = 28;
pub const CLK_BUS_CODEC: c_int = 29;
pub const CLK_BUS_SPDIF: c_int = 30;
pub const CLK_BUS_IR: c_int = 31;
pub const CLK_BUS_RSB: c_int = 32;
pub const CLK_BUS_I2S0: c_int = 33;
pub const CLK_BUS_I2C0: c_int = 34;
pub const CLK_BUS_I2C1: c_int = 35;
pub const CLK_BUS_I2C2: c_int = 36;
pub const CLK_BUS_PIO: c_int = 37;
pub const CLK_BUS_UART0: c_int = 38;
pub const CLK_BUS_UART1: c_int = 39;
pub const CLK_BUS_UART2: c_int = 40;
pub const CLK_MMC0: c_int = 41;
pub const CLK_MMC0_SAMPLE: c_int = 42;
pub const CLK_MMC0_OUTPUT: c_int = 43;
pub const CLK_MMC1: c_int = 44;
pub const CLK_MMC1_SAMPLE: c_int = 45;
pub const CLK_MMC1_OUTPUT: c_int = 46;
pub const CLK_I2S: c_int = 47;
pub const CLK_SPDIF: c_int = 48;
pub const CLK_USB_PHY0: c_int = 49;
pub const CLK_DRAM_VE: c_int = 50;
pub const CLK_DRAM_CSI: c_int = 51;
pub const CLK_DRAM_DEINTERLACE: c_int = 52;
pub const CLK_DRAM_TVD: c_int = 53;
pub const CLK_DRAM_DE_FE: c_int = 54;
pub const CLK_DRAM_DE_BE: c_int = 55;
pub const CLK_DE_BE: c_int = 56;
pub const CLK_DE_FE: c_int = 57;
pub const CLK_TCON: c_int = 58;
pub const CLK_DEINTERLACE: c_int = 59;
pub const CLK_TVE2_CLK: c_int = 60;
pub const CLK_TVE1_CLK: c_int = 61;
pub const CLK_TVD: c_int = 62;
pub const CLK_CSI: c_int = 63;
pub const CLK_VE: c_int = 64;
pub const CLK_CODEC: c_int = 65;
pub const CLK_AVS: c_int = 66;
pub const CLK_IR: c_int = 67;
