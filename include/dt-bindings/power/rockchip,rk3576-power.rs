//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/rockchip,rk3576-power.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
// VD_NPU
pub const RK3576_PD_NPU: c_int = 0;
pub const RK3576_PD_NPUTOP: c_int = 1;
pub const RK3576_PD_NPU0: c_int = 2;
pub const RK3576_PD_NPU1: c_int = 3;
// VD_GPU
pub const RK3576_PD_GPU: c_int = 4;
// VD_LOGIC
pub const RK3576_PD_NVM: c_int = 5;
pub const RK3576_PD_SDGMAC: c_int = 6;
pub const RK3576_PD_USB: c_int = 7;
pub const RK3576_PD_PHP: c_int = 8;
pub const RK3576_PD_SUBPHP: c_int = 9;
pub const RK3576_PD_AUDIO: c_int = 10;
pub const RK3576_PD_VEPU0: c_int = 11;
pub const RK3576_PD_VEPU1: c_int = 12;
pub const RK3576_PD_VPU: c_int = 13;
pub const RK3576_PD_VDEC: c_int = 14;
pub const RK3576_PD_VI: c_int = 15;
pub const RK3576_PD_VO0: c_int = 16;
pub const RK3576_PD_VO1: c_int = 17;
pub const RK3576_PD_VOP: c_int = 18;
