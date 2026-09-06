//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/net/renesas,r9a09g077-pcs-miic.h
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
//
// Copyright (C) 2025 Renesas Electronics Corporation.
//
// Media Interface Connection Matrix
// ===========================================================
//
// Selects the function of the Media interface of the MAC to be used
//
// SW_MODE[2:0] | Port 0      | Port 1      | Port 2      | Port 3
// -------------|-------------|-------------|-------------|-------------
// 000b         | ETHSW Port0 | ETHSW Port1 | ETHSW Port2 | GMAC1
// 001b         | ESC Port0   | ESC Port1   | GMAC2       | GMAC1
// 010b         | ESC Port0   | ESC Port1   | ETHSW Port2 | GMAC1
// 011b         | ESC Port0   | ESC Port1   | ESC Port2   | GMAC1
// 100b         | ETHSW Port0 | ESC Port1   | ESC Port2   | GMAC1
// 101b         | ETHSW Port0 | ESC Port1   | ETHSW Port2 | GMAC1
// 110b         | ETHSW Port0 | ETHSW Port1 | GMAC2       | GMAC1
// 111b         | GMAC0       | GMAC1       | GMAC2       | -
//
pub const ETHSS_GMAC0_PORT: c_int = 0;
pub const ETHSS_GMAC1_PORT: c_int = 1;
pub const ETHSS_GMAC2_PORT: c_int = 2;
pub const ETHSS_ESC_PORT0: c_int = 3;
pub const ETHSS_ESC_PORT1: c_int = 4;
pub const ETHSS_ESC_PORT2: c_int = 5;
pub const ETHSS_ETHSW_PORT0: c_int = 6;
pub const ETHSS_ETHSW_PORT1: c_int = 7;
pub const ETHSS_ETHSW_PORT2: c_int = 8;
