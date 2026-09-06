//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/ingenic,sysost.h
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
// This header provides clock numbers for the Ingenic OST DT binding.
//
pub const OST_CLK_PERCPU_TIMER: c_int = 1;
pub const OST_CLK_GLOBAL_TIMER: c_int = 0;
pub const OST_CLK_PERCPU_TIMER0: c_int = 1;
pub const OST_CLK_PERCPU_TIMER1: c_int = 2;
pub const OST_CLK_PERCPU_TIMER2: c_int = 3;
pub const OST_CLK_PERCPU_TIMER3: c_int = 4;
pub const OST_CLK_EVENT_TIMER: c_int = 1;
pub const OST_CLK_EVENT_TIMER0: c_int = 0;
pub const OST_CLK_EVENT_TIMER1: c_int = 1;
pub const OST_CLK_EVENT_TIMER2: c_int = 2;
pub const OST_CLK_EVENT_TIMER3: c_int = 3;
pub const OST_CLK_EVENT_TIMER4: c_int = 4;
pub const OST_CLK_EVENT_TIMER5: c_int = 5;
pub const OST_CLK_EVENT_TIMER6: c_int = 6;
pub const OST_CLK_EVENT_TIMER7: c_int = 7;
pub const OST_CLK_EVENT_TIMER8: c_int = 8;
pub const OST_CLK_EVENT_TIMER9: c_int = 9;
pub const OST_CLK_EVENT_TIMER10: c_int = 10;
pub const OST_CLK_EVENT_TIMER11: c_int = 11;
pub const OST_CLK_EVENT_TIMER12: c_int = 12;
pub const OST_CLK_EVENT_TIMER13: c_int = 13;
pub const OST_CLK_EVENT_TIMER14: c_int = 14;
pub const OST_CLK_EVENT_TIMER15: c_int = 15;
