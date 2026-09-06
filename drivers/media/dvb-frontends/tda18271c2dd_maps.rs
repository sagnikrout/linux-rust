//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/tda18271c2dd_maps.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HF_S {
    HF_None = 0, HF_B, HF_DK, HF_G, HF_I, HF_L, HF_L1, HF_MN, HF_FM_Radio,
    HF_AnalogMax, HF_DVBT_6MHZ, HF_DVBT_7MHZ, HF_DVBT_8MHZ,
    HF_DVBT, HF_ATSC,  HF_DVBC_6MHZ,  HF_DVBC_7MHZ,
    HF_DVBC_8MHZ, HF_DVBC
}
