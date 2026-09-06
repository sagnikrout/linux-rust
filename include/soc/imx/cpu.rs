//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/imx/cpu.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
pub const MXC_CPU_MX1: c_int = 1;
pub const MXC_CPU_MX21: c_int = 21;
pub const MXC_CPU_MX25: c_int = 25;
pub const MXC_CPU_MX27: c_int = 27;
pub const MXC_CPU_MX31: c_int = 31;
pub const MXC_CPU_MX35: c_int = 35;
pub const MXC_CPU_MX50: c_int = 50;
pub const MXC_CPU_MX51: c_int = 51;
pub const MXC_CPU_MX53: c_int = 53;
pub const MXC_CPU_IMX6SL: c_uint = 0x60;
pub const MXC_CPU_IMX6DL: c_uint = 0x61;
pub const MXC_CPU_IMX6SX: c_uint = 0x62;
pub const MXC_CPU_IMX6Q: c_uint = 0x63;
pub const MXC_CPU_IMX6UL: c_uint = 0x64;
pub const MXC_CPU_IMX6ULL: c_uint = 0x65;
// virtual cpu id for i.mx6ulz
pub const MXC_CPU_IMX6ULZ: c_uint = 0x6b;
pub const MXC_CPU_IMX6SLL: c_uint = 0x67;
pub const MXC_CPU_IMX7D: c_uint = 0x72;
pub const MXC_CPU_IMX7ULP: c_uint = 0xff;
pub const MXC_CPU_VFx10: c_uint = 0x010;
pub const MXC_CPU_VF500: c_uint = 0x500;

pub const MXC_CPU_VF600: c_uint = 0x600;

