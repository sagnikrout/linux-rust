//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/sound/qcom,q6asm.h
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
pub const MSM_FRONTEND_DAI_MULTIMEDIA1: c_int = 0;
pub const MSM_FRONTEND_DAI_MULTIMEDIA2: c_int = 1;
pub const MSM_FRONTEND_DAI_MULTIMEDIA3: c_int = 2;
pub const MSM_FRONTEND_DAI_MULTIMEDIA4: c_int = 3;
pub const MSM_FRONTEND_DAI_MULTIMEDIA5: c_int = 4;
pub const MSM_FRONTEND_DAI_MULTIMEDIA6: c_int = 5;
pub const MSM_FRONTEND_DAI_MULTIMEDIA7: c_int = 6;
pub const MSM_FRONTEND_DAI_MULTIMEDIA8: c_int = 7;
pub const MSM_FRONTEND_DAI_MULTIMEDIA9: c_int = 8;
pub const MSM_FRONTEND_DAI_MULTIMEDIA10: c_int = 9;
pub const MSM_FRONTEND_DAI_MULTIMEDIA11: c_int = 10;
pub const MSM_FRONTEND_DAI_MULTIMEDIA12: c_int = 11;
pub const MSM_FRONTEND_DAI_MULTIMEDIA13: c_int = 12;
pub const MSM_FRONTEND_DAI_MULTIMEDIA14: c_int = 13;
pub const MSM_FRONTEND_DAI_MULTIMEDIA15: c_int = 14;
pub const MSM_FRONTEND_DAI_MULTIMEDIA16: c_int = 15;
pub const Q6ASM_DAI_TX_RX: c_int = 0;
pub const Q6ASM_DAI_TX: c_int = 1;
pub const Q6ASM_DAI_RX: c_int = 2;
