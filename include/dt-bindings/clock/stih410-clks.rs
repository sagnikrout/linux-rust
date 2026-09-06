//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/stih410-clks.h
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
//
// This header provides constants clk index STMicroelectronics
// STiH410 SoC.
//

// STiH410 introduces new clock outputs compared to STiH407
// CLOCKGEN C0
pub const CLK_TX_ICN_HADES: c_int = 32;
pub const CLK_RX_ICN_HADES: c_int = 33;
pub const CLK_ICN_REG_16: c_int = 34;
pub const CLK_PP_HADES: c_int = 35;
pub const CLK_CLUST_HADES: c_int = 36;
pub const CLK_HWPE_HADES: c_int = 37;
pub const CLK_FC_HADES: c_int = 38;
// CLOCKGEN D0
pub const CLK_PCMR10_MASTER: c_int = 4;
pub const CLK_USB2_PHY: c_int = 5;
