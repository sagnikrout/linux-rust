//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/rockchip,rk3528-power.h
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
pub const RK3528_PD_PMU: c_int = 0;
pub const RK3528_PD_BUS: c_int = 1;
pub const RK3528_PD_DDR: c_int = 2;
pub const RK3528_PD_MSCH: c_int = 3;
// VD_GPU
pub const RK3528_PD_GPU: c_int = 4;
// VD_LOGIC
pub const RK3528_PD_RKVDEC: c_int = 5;
pub const RK3528_PD_RKVENC: c_int = 6;
pub const RK3528_PD_VO: c_int = 7;
pub const RK3528_PD_VPU: c_int = 8;
