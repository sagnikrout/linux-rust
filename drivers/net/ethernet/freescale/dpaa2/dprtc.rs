//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/dpaa2/dprtc.h
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
// Copyright 2013-2016 Freescale Semiconductor Inc.
// Copyright 2016-2018 NXP
//
// Data Path Real Time Counter API
// Contains initialization APIs and runtime control APIs for RTC
//
pub const DPRTC_MAX_IRQ_NUM: c_int = 1;
pub const DPRTC_IRQ_INDEX: c_int = 0;
pub const DPRTC_EVENT_PPS: c_uint = 0x08000000;
pub const DPRTC_EVENT_ETS1: c_uint = 0x00800000;
pub const DPRTC_EVENT_ETS2: c_uint = 0x00400000;
