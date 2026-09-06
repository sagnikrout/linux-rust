//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pata_arasan_cf_data.h
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


//
// include/linux/pata_arasan_cf_data.h
//
// Arasan Compact Flash host controller platform data header file
//
// Copyright (C) 2011 ST Microelectronics
// Viresh Kumar <vireshk@kernel.org>
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arasan_cf_pdata {
    pub cf_if_clk: u8,

//
// Platform specific incapabilities of CF controller is handled via
// quirks
//
    pub quirk: u32,

}
