//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/boot/dts/freescale/imx95-power.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright 2024 NXP
//
pub const IMX95_PD_ANA: c_int = 0;
pub const IMX95_PD_AON: c_int = 1;
pub const IMX95_PD_BBSM: c_int = 2;
pub const IMX95_PD_CAMERA: c_int = 3;
pub const IMX95_PD_CCMSRCGPC: c_int = 4;
pub const IMX95_PD_A55C0: c_int = 5;
pub const IMX95_PD_A55C1: c_int = 6;
pub const IMX95_PD_A55C2: c_int = 7;
pub const IMX95_PD_A55C3: c_int = 8;
pub const IMX95_PD_A55C4: c_int = 9;
pub const IMX95_PD_A55C5: c_int = 10;
pub const IMX95_PD_A55P: c_int = 11;
pub const IMX95_PD_DDR: c_int = 12;
pub const IMX95_PD_DISPLAY: c_int = 13;
pub const IMX95_PD_GPU: c_int = 14;
pub const IMX95_PD_HSIO_TOP: c_int = 15;
pub const IMX95_PD_HSIO_WAON: c_int = 16;
pub const IMX95_PD_M7: c_int = 17;
pub const IMX95_PD_NETC: c_int = 18;
pub const IMX95_PD_NOC: c_int = 19;
pub const IMX95_PD_NPU: c_int = 20;
pub const IMX95_PD_VPU: c_int = 21;
pub const IMX95_PD_WAKEUP: c_int = 22;
pub const IMX95_PERF_ELE: c_int = 0;
pub const IMX95_PERF_M33: c_int = 1;
pub const IMX95_PERF_WAKEUP: c_int = 2;
pub const IMX95_PERF_M7: c_int = 3;
pub const IMX95_PERF_DRAM: c_int = 4;
pub const IMX95_PERF_HSIO: c_int = 5;
pub const IMX95_PERF_NPU: c_int = 6;
pub const IMX95_PERF_NOC: c_int = 7;
pub const IMX95_PERF_A55: c_int = 8;
pub const IMX95_PERF_GPU: c_int = 9;
pub const IMX95_PERF_VPU: c_int = 10;
pub const IMX95_PERF_CAM: c_int = 11;
pub const IMX95_PERF_DISP: c_int = 12;
