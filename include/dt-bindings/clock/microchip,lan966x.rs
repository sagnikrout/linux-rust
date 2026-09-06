//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/microchip,lan966x.h
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
// Copyright (c) 2021 Microchip Inc.
//
// Author: Kavyasree Kotagiri <kavyasree.kotagiri@microchip.com>
//
pub const GCK_ID_QSPI0: c_int = 0;
pub const GCK_ID_QSPI1: c_int = 1;
pub const GCK_ID_QSPI2: c_int = 2;
pub const GCK_ID_SDMMC0: c_int = 3;
pub const GCK_ID_PI: c_int = 4;
pub const GCK_ID_MCAN0: c_int = 5;
pub const GCK_ID_MCAN1: c_int = 6;
pub const GCK_ID_FLEXCOM0: c_int = 7;
pub const GCK_ID_FLEXCOM1: c_int = 8;
pub const GCK_ID_FLEXCOM2: c_int = 9;
pub const GCK_ID_FLEXCOM3: c_int = 10;
pub const GCK_ID_FLEXCOM4: c_int = 11;
pub const GCK_ID_TIMER: c_int = 12;
pub const GCK_ID_USB_REFCLK: c_int = 13;
// Gate clocks
pub const GCK_GATE_UHPHS: c_int = 14;
pub const GCK_GATE_UDPHS: c_int = 15;
pub const GCK_GATE_MCRAMC: c_int = 16;
pub const GCK_GATE_HMATRIX: c_int = 17;
pub const N_CLOCKS: c_int = 18;
