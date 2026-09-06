//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/mediatek,mt6735-infracfg.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
pub const CLK_INFRA_DBG: c_int = 0;
pub const CLK_INFRA_GCE: c_int = 1;
pub const CLK_INFRA_TRBG: c_int = 2;
pub const CLK_INFRA_CPUM: c_int = 3;
pub const CLK_INFRA_DEVAPC: c_int = 4;
pub const CLK_INFRA_AUDIO: c_int = 5;
pub const CLK_INFRA_GCPU: c_int = 6;
pub const CLK_INFRA_L2C_SRAM: c_int = 7;
pub const CLK_INFRA_M4U: c_int = 8;
pub const CLK_INFRA_CLDMA: c_int = 9;
pub const CLK_INFRA_CONNMCU_BUS: c_int = 10;
pub const CLK_INFRA_KP: c_int = 11;
pub const CLK_INFRA_APXGPT: c_int = 12;
pub const CLK_INFRA_SEJ: c_int = 13;
pub const CLK_INFRA_CCIF0_AP: c_int = 14;
pub const CLK_INFRA_CCIF1_AP: c_int = 15;
pub const CLK_INFRA_PMIC_SPI: c_int = 16;
pub const CLK_INFRA_PMIC_WRAP: c_int = 17;
