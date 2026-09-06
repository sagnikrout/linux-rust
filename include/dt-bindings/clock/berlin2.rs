//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/berlin2.h
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
//
// Berlin2 BG2/BG2CD clock tree IDs
//
pub const CLKID_SYS: c_int = 0;
pub const CLKID_CPU: c_int = 1;
pub const CLKID_DRMFIGO: c_int = 2;
pub const CLKID_CFG: c_int = 3;
pub const CLKID_GFX: c_int = 4;
pub const CLKID_ZSP: c_int = 5;
pub const CLKID_PERIF: c_int = 6;
pub const CLKID_PCUBE: c_int = 7;
pub const CLKID_VSCOPE: c_int = 8;
pub const CLKID_NFC_ECC: c_int = 9;
pub const CLKID_VPP: c_int = 10;
pub const CLKID_APP: c_int = 11;
pub const CLKID_AUDIO0: c_int = 12;
pub const CLKID_AUDIO2: c_int = 13;
pub const CLKID_AUDIO3: c_int = 14;
pub const CLKID_AUDIO1: c_int = 15;
pub const CLKID_GFX3D_CORE: c_int = 16;
pub const CLKID_GFX3D_SYS: c_int = 17;
pub const CLKID_ARC: c_int = 18;
pub const CLKID_VIP: c_int = 19;
pub const CLKID_SDIO0XIN: c_int = 20;
pub const CLKID_SDIO1XIN: c_int = 21;
pub const CLKID_GFX3D_EXTRA: c_int = 22;
pub const CLKID_GC360: c_int = 23;
pub const CLKID_SDIO_DLLMST: c_int = 24;
pub const CLKID_GETH0: c_int = 25;
pub const CLKID_GETH1: c_int = 26;
pub const CLKID_SATA: c_int = 27;
pub const CLKID_AHBAPB: c_int = 28;
pub const CLKID_USB0: c_int = 29;
pub const CLKID_USB1: c_int = 30;
pub const CLKID_PBRIDGE: c_int = 31;
pub const CLKID_SDIO0: c_int = 32;
pub const CLKID_SDIO1: c_int = 33;
pub const CLKID_NFC: c_int = 34;
pub const CLKID_SMEMC: c_int = 35;
pub const CLKID_AUDIOHD: c_int = 36;
pub const CLKID_VIDEO0: c_int = 37;
pub const CLKID_VIDEO1: c_int = 38;
pub const CLKID_VIDEO2: c_int = 39;
pub const CLKID_TWD: c_int = 40;
