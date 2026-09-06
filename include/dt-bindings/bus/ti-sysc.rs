//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/bus/ti-sysc.h
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
// TI sysc interconnect target module defines
// Generic sysc found on omap2 and later, also known as type1

// Generic sysc found on omap4 and later, also known as type2

// SmartReflex sysc found on 36xx and later

// PRUSS sysc found on AM33xx/AM43xx/AM57xx

// SYSCONFIG STANDBYMODE/MIDLEMODE/SIDLEMODE supported by hardware
pub const SYSC_IDLE_FORCE: c_int = 0;
pub const SYSC_IDLE_NO: c_int = 1;
pub const SYSC_IDLE_SMART: c_int = 2;
pub const SYSC_IDLE_SMART_WKUP: c_int = 3;
