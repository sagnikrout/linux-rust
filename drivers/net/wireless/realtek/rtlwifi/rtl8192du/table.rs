//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192du/table.h
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
// Copyright(c) 2024  Realtek Corporation.
pub const PHY_REG_2T_ARRAYLENGTH: c_int = 372;
pub const PHY_REG_ARRAY_PG_LENGTH: c_int = 624;
pub const RADIOA_2T_ARRAYLENGTH: c_int = 378;
pub const RADIOB_2T_ARRAYLENGTH: c_int = 384;
pub const RADIOA_2T_INT_PA_ARRAYLENGTH: c_int = 378;
pub const RADIOB_2T_INT_PA_ARRAYLENGTH: c_int = 384;
pub const MAC_2T_ARRAYLENGTH: c_int = 192;
pub const AGCTAB_ARRAYLENGTH: c_int = 386;
pub const AGCTAB_5G_ARRAYLENGTH: c_int = 194;
pub const AGCTAB_2G_ARRAYLENGTH: c_int = 194;
