//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/include/soc.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2010 Broadcom Corporation
//
pub const SI_ENUM_BASE_DEFAULT: c_uint = 0x18000000;
// Common core control flags
pub const SICF_BIST_EN: c_uint = 0x8000;
pub const SICF_PME_EN: c_uint = 0x4000;
pub const SICF_CORE_BITS: c_uint = 0x3ffc;
pub const SICF_FGC: c_uint = 0x0002;
pub const SICF_CLOCK_EN: c_uint = 0x0001;
// Common core status flags
pub const SISF_BIST_DONE: c_uint = 0x8000;
pub const SISF_BIST_ERROR: c_uint = 0x4000;
pub const SISF_GATED_CLK: c_uint = 0x2000;
pub const SISF_DMA64: c_uint = 0x1000;
pub const SISF_CORE_BITS: c_uint = 0x0fff;
