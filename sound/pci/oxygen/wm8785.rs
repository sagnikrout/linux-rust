//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/oxygen/wm8785.h
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

// Macro flag: #define WM8785_H_INCLUDED
pub const WM8785_R0: c_int = 0;
pub const WM8785_R1: c_int = 1;
pub const WM8785_R2: c_int = 2;
pub const WM8785_R7: c_int = 7;
// R0
pub const WM8785_MCR_MASK: c_uint = 0x007;
pub const WM8785_MCR_SLAVE: c_uint = 0x000;
pub const WM8785_MCR_MASTER_128: c_uint = 0x001;
pub const WM8785_MCR_MASTER_192: c_uint = 0x002;
pub const WM8785_MCR_MASTER_256: c_uint = 0x003;
pub const WM8785_MCR_MASTER_384: c_uint = 0x004;
pub const WM8785_MCR_MASTER_512: c_uint = 0x005;
pub const WM8785_MCR_MASTER_768: c_uint = 0x006;
pub const WM8785_OSR_MASK: c_uint = 0x018;
pub const WM8785_OSR_SINGLE: c_uint = 0x000;
pub const WM8785_OSR_DOUBLE: c_uint = 0x008;
pub const WM8785_OSR_QUAD: c_uint = 0x010;
pub const WM8785_FORMAT_MASK: c_uint = 0x060;
pub const WM8785_FORMAT_RJUST: c_uint = 0x000;
pub const WM8785_FORMAT_LJUST: c_uint = 0x020;
pub const WM8785_FORMAT_I2S: c_uint = 0x040;
pub const WM8785_FORMAT_DSP: c_uint = 0x060;
// R1
pub const WM8785_WL_MASK: c_uint = 0x003;
pub const WM8785_WL_16: c_uint = 0x000;
pub const WM8785_WL_20: c_uint = 0x001;
pub const WM8785_WL_24: c_uint = 0x002;
pub const WM8785_WL_32: c_uint = 0x003;
pub const WM8785_LRP: c_uint = 0x004;
pub const WM8785_BCLKINV: c_uint = 0x008;
pub const WM8785_LRSWAP: c_uint = 0x010;
pub const WM8785_DEVNO_MASK: c_uint = 0x0e0;
// R2
pub const WM8785_HPFR: c_uint = 0x001;
pub const WM8785_HPFL: c_uint = 0x002;
pub const WM8785_SDODIS: c_uint = 0x004;
pub const WM8785_PWRDNR: c_uint = 0x008;
pub const WM8785_PWRDNL: c_uint = 0x010;
pub const WM8785_TDM_MASK: c_uint = 0x1c0;
