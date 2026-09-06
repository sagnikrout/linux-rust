//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/hirschmann-hellcreek.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Hirschmann Hellcreek TSN switch platform data.
//
// Copyright (C) 2020 Linutronix GmbH
// Author Kurt Kanzenbach <kurt@linutronix.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hellcreek_platform_data {
    pub /: *const *const *const char name; / Switch name,
    pub /: *mut *mut int num_ports; / Amount of switch ports,
    pub /: *mut *mut int is_100_mbits; / Is it configured to 100 or 1000 mbit/s,
    pub /: *mut *mut int qbv_support; / Qbv support on front TSN ports,
    pub /: *mut *mut int qbv_on_cpu_port; / Qbv support on the CPU port,
    pub /: *mut *mut int qbu_support; / Qbu support on front TSN ports,
    pub /: *mut *mut u16 module_id; / Module identificaton,
}
