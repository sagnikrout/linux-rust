//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/pxa/pxa-ssp.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// ASoC PXA SSP port support
//
// SSP clock sources
pub const PXA_SSP_CLK_PLL: c_int = 0;
pub const PXA_SSP_CLK_EXT: c_int = 1;
pub const PXA_SSP_CLK_NET: c_int = 2;
pub const PXA_SSP_CLK_AUDIO: c_int = 3;
pub const PXA_SSP_CLK_NET_PLL: c_int = 4;
// SSP audio dividers
pub const PXA_SSP_AUDIO_DIV_ACDS: c_int = 0;
pub const PXA_SSP_AUDIO_DIV_SCDB: c_int = 1;
pub const PXA_SSP_DIV_SCR: c_int = 2;
// SSP ACDS audio dividers values
pub const PXA_SSP_CLK_AUDIO_DIV_1: c_int = 0;
pub const PXA_SSP_CLK_AUDIO_DIV_2: c_int = 1;
pub const PXA_SSP_CLK_AUDIO_DIV_4: c_int = 2;
pub const PXA_SSP_CLK_AUDIO_DIV_8: c_int = 3;
pub const PXA_SSP_CLK_AUDIO_DIV_16: c_int = 4;
pub const PXA_SSP_CLK_AUDIO_DIV_32: c_int = 5;
// SSP divider bypass
pub const PXA_SSP_CLK_SCDB_4: c_int = 0;
pub const PXA_SSP_CLK_SCDB_1: c_int = 1;
pub const PXA_SSP_CLK_SCDB_8: c_int = 2;
pub const PXA_SSP_PLL_OUT: c_int = 0;
