//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt766-sdca-sdw.h
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
// rt766-sdca-sdw.h -- RT766 SDCA ALSA SoC audio driver header
//
// Copyright(c) 2026 Realtek Semiconductor Corp.
//

// 0x40400289 - 0x4040028a
// 0x40400291 - 0x40400292
// 0x40400789 - 0x4040078a
// 0x40400791 - 0x40400792
// 0x40600259 - 0x4060025a
// 0x40801809 - 0x4080180c
// 0x40801811 - 0x40801814
// 0x41000189 - 0x4100018a
// 0x41000191 - 0x41000192
