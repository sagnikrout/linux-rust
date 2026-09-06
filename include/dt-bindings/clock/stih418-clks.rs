//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/stih418-clks.h
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
// STiH418 SoC.
//

// STiH418 introduces new clock outputs compared to STiH410
// CLOCKGEN C0
pub const CLK_PROC_BDISP_0: c_int = 14;
pub const CLK_PROC_BDISP_1: c_int = 15;
pub const CLK_TX_ICN_1: c_int = 23;
pub const CLK_ETH_PHYREF: c_int = 27;
pub const CLK_PP_HEVC: c_int = 35;
pub const CLK_CLUST_HEVC: c_int = 36;
pub const CLK_HWPE_HEVC: c_int = 37;
pub const CLK_FC_HEVC: c_int = 38;
pub const CLK_PROC_MIXER: c_int = 39;
pub const CLK_PROC_SC: c_int = 40;
pub const CLK_AVSP_HEVC: c_int = 41;
// CLOCKGEN D2

pub const CLK_TMDS_HDMI_DIV2: c_int = 5;
pub const CLK_VP9: c_int = 47;
