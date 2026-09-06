//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/rk3588-power.h
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
// VD_LITDSU
pub const RK3588_PD_CPU_0: c_int = 0;
pub const RK3588_PD_CPU_1: c_int = 1;
pub const RK3588_PD_CPU_2: c_int = 2;
pub const RK3588_PD_CPU_3: c_int = 3;
// VD_BIGCORE0
pub const RK3588_PD_CPU_4: c_int = 4;
pub const RK3588_PD_CPU_5: c_int = 5;
// VD_BIGCORE1
pub const RK3588_PD_CPU_6: c_int = 6;
pub const RK3588_PD_CPU_7: c_int = 7;
// VD_NPU
pub const RK3588_PD_NPU: c_int = 8;
pub const RK3588_PD_NPUTOP: c_int = 9;
pub const RK3588_PD_NPU1: c_int = 10;
pub const RK3588_PD_NPU2: c_int = 11;
// VD_GPU
pub const RK3588_PD_GPU: c_int = 12;
// VD_VCODEC
pub const RK3588_PD_VCODEC: c_int = 13;
pub const RK3588_PD_RKVDEC0: c_int = 14;
pub const RK3588_PD_RKVDEC1: c_int = 15;
pub const RK3588_PD_VENC0: c_int = 16;
pub const RK3588_PD_VENC1: c_int = 17;
// VD_DD01
pub const RK3588_PD_DDR01: c_int = 18;
// VD_DD23
pub const RK3588_PD_DDR23: c_int = 19;
// VD_LOGIC
pub const RK3588_PD_CENTER: c_int = 20;
pub const RK3588_PD_VDPU: c_int = 21;
pub const RK3588_PD_RGA30: c_int = 22;
pub const RK3588_PD_AV1: c_int = 23;
pub const RK3588_PD_VOP: c_int = 24;
pub const RK3588_PD_VO0: c_int = 25;
pub const RK3588_PD_VO1: c_int = 26;
pub const RK3588_PD_VI: c_int = 27;
pub const RK3588_PD_ISP1: c_int = 28;
pub const RK3588_PD_FEC: c_int = 29;
pub const RK3588_PD_RGA31: c_int = 30;
pub const RK3588_PD_USB: c_int = 31;
pub const RK3588_PD_PHP: c_int = 32;
pub const RK3588_PD_GMAC: c_int = 33;
pub const RK3588_PD_PCIE: c_int = 34;
pub const RK3588_PD_NVM: c_int = 35;
pub const RK3588_PD_NVM0: c_int = 36;
pub const RK3588_PD_SDIO: c_int = 37;
pub const RK3588_PD_AUDIO: c_int = 38;
pub const RK3588_PD_SECURE: c_int = 39;
pub const RK3588_PD_SDMMC: c_int = 40;
pub const RK3588_PD_CRYPTO: c_int = 41;
pub const RK3588_PD_BUS: c_int = 42;
// VD_PMU
pub const RK3588_PD_PMU1: c_int = 43;
