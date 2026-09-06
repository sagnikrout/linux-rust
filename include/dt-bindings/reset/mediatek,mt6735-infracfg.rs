//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/mediatek,mt6735-infracfg.h
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
pub const MT6735_INFRA_RST0_EMI_REG: c_int = 0;
pub const MT6735_INFRA_RST0_DRAMC0_AO: c_int = 1;
pub const MT6735_INFRA_RST0_AP_CIRQ_EINT: c_int = 2;
pub const MT6735_INFRA_RST0_APXGPT: c_int = 3;
pub const MT6735_INFRA_RST0_SCPSYS: c_int = 4;
pub const MT6735_INFRA_RST0_KP: c_int = 5;
pub const MT6735_INFRA_RST0_PMIC_WRAP: c_int = 6;
pub const MT6735_INFRA_RST0_CLDMA_AO_TOP: c_int = 7;
pub const MT6735_INFRA_RST0_USBSIF_TOP: c_int = 8;
pub const MT6735_INFRA_RST0_EMI: c_int = 9;
pub const MT6735_INFRA_RST0_CCIF: c_int = 10;
pub const MT6735_INFRA_RST0_DRAMC0: c_int = 11;
pub const MT6735_INFRA_RST0_EMI_AO_REG: c_int = 12;
pub const MT6735_INFRA_RST0_CCIF_AO: c_int = 13;
pub const MT6735_INFRA_RST0_TRNG: c_int = 14;
pub const MT6735_INFRA_RST0_SYS_CIRQ: c_int = 15;
pub const MT6735_INFRA_RST0_GCE: c_int = 16;
pub const MT6735_INFRA_RST0_M4U: c_int = 17;
pub const MT6735_INFRA_RST0_CCIF1: c_int = 18;
pub const MT6735_INFRA_RST0_CLDMA_TOP_PD: c_int = 19;
