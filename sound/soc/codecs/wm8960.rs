//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8960.h
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
// wm8960.h  --  WM8960 Soc Audio driver
//
// WM8960 register space
pub const WM8960_CACHEREGNUM: c_int = 56;
pub const WM8960_LINVOL: c_uint = 0x0;
pub const WM8960_RINVOL: c_uint = 0x1;
pub const WM8960_LOUT1: c_uint = 0x2;
pub const WM8960_ROUT1: c_uint = 0x3;
pub const WM8960_CLOCK1: c_uint = 0x4;
pub const WM8960_DACCTL1: c_uint = 0x5;
pub const WM8960_DACCTL2: c_uint = 0x6;
pub const WM8960_IFACE1: c_uint = 0x7;
pub const WM8960_CLOCK2: c_uint = 0x8;
pub const WM8960_IFACE2: c_uint = 0x9;
pub const WM8960_LDAC: c_uint = 0xa;
pub const WM8960_RDAC: c_uint = 0xb;
pub const WM8960_RESET: c_uint = 0xf;
pub const WM8960_3D: c_uint = 0x10;
pub const WM8960_ALC1: c_uint = 0x11;
pub const WM8960_ALC2: c_uint = 0x12;
pub const WM8960_ALC3: c_uint = 0x13;
pub const WM8960_NOISEG: c_uint = 0x14;
pub const WM8960_LADC: c_uint = 0x15;
pub const WM8960_RADC: c_uint = 0x16;
pub const WM8960_ADDCTL1: c_uint = 0x17;
pub const WM8960_ADDCTL2: c_uint = 0x18;
pub const WM8960_POWER1: c_uint = 0x19;
pub const WM8960_POWER2: c_uint = 0x1a;
pub const WM8960_ADDCTL3: c_uint = 0x1b;
pub const WM8960_APOP1: c_uint = 0x1c;
pub const WM8960_APOP2: c_uint = 0x1d;
pub const WM8960_LINPATH: c_uint = 0x20;
pub const WM8960_RINPATH: c_uint = 0x21;
pub const WM8960_LOUTMIX: c_uint = 0x22;
pub const WM8960_ROUTMIX: c_uint = 0x25;
pub const WM8960_MONOMIX1: c_uint = 0x26;
pub const WM8960_MONOMIX2: c_uint = 0x27;
pub const WM8960_LOUT2: c_uint = 0x28;
pub const WM8960_ROUT2: c_uint = 0x29;
pub const WM8960_MONO: c_uint = 0x2a;
pub const WM8960_INBMIX1: c_uint = 0x2b;
pub const WM8960_INBMIX2: c_uint = 0x2c;
pub const WM8960_BYPASS1: c_uint = 0x2d;
pub const WM8960_BYPASS2: c_uint = 0x2e;
pub const WM8960_POWER3: c_uint = 0x2f;
pub const WM8960_ADDCTL4: c_uint = 0x30;
pub const WM8960_CLASSD1: c_uint = 0x31;
pub const WM8960_CLASSD3: c_uint = 0x33;
pub const WM8960_PLL1: c_uint = 0x34;
pub const WM8960_PLL2: c_uint = 0x35;
pub const WM8960_PLL3: c_uint = 0x36;
pub const WM8960_PLL4: c_uint = 0x37;
//
// WM8960 Clock dividers
//
pub const WM8960_SYSCLKDIV: c_int = 0;
pub const WM8960_DACDIV: c_int = 1;
pub const WM8960_OPCLKDIV: c_int = 2;
pub const WM8960_DCLKDIV: c_int = 3;
pub const WM8960_TOCLKSEL: c_int = 4;

