//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/rk3399-power.h
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
// VD_CORE_L
pub const RK3399_PD_A53_L0: c_int = 0;
pub const RK3399_PD_A53_L1: c_int = 1;
pub const RK3399_PD_A53_L2: c_int = 2;
pub const RK3399_PD_A53_L3: c_int = 3;
pub const RK3399_PD_SCU_L: c_int = 4;
// VD_CORE_B
pub const RK3399_PD_A72_B0: c_int = 5;
pub const RK3399_PD_A72_B1: c_int = 6;
pub const RK3399_PD_SCU_B: c_int = 7;
// VD_LOGIC
pub const RK3399_PD_TCPD0: c_int = 8;
pub const RK3399_PD_TCPD1: c_int = 9;
pub const RK3399_PD_CCI: c_int = 10;
pub const RK3399_PD_CCI0: c_int = 11;
pub const RK3399_PD_CCI1: c_int = 12;
pub const RK3399_PD_PERILP: c_int = 13;
pub const RK3399_PD_PERIHP: c_int = 14;
pub const RK3399_PD_VIO: c_int = 15;
pub const RK3399_PD_VO: c_int = 16;
pub const RK3399_PD_VOPB: c_int = 17;
pub const RK3399_PD_VOPL: c_int = 18;
pub const RK3399_PD_ISP0: c_int = 19;
pub const RK3399_PD_ISP1: c_int = 20;
pub const RK3399_PD_HDCP: c_int = 21;
pub const RK3399_PD_GMAC: c_int = 22;
pub const RK3399_PD_EMMC: c_int = 23;
pub const RK3399_PD_USB3: c_int = 24;
pub const RK3399_PD_EDP: c_int = 25;
pub const RK3399_PD_GIC: c_int = 26;
pub const RK3399_PD_SD: c_int = 27;
pub const RK3399_PD_SDIOAUDIO: c_int = 28;
pub const RK3399_PD_ALIVE: c_int = 29;
// VD_CENTER
pub const RK3399_PD_CENTER: c_int = 30;
pub const RK3399_PD_VCODEC: c_int = 31;
pub const RK3399_PD_VDU: c_int = 32;
pub const RK3399_PD_RGA: c_int = 33;
pub const RK3399_PD_IEP: c_int = 34;
// VD_GPU
pub const RK3399_PD_GPU: c_int = 35;
// VD_PMU
pub const RK3399_PD_PMU: c_int = 36;
