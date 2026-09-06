//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/timer.h
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

extern "C" {
    pub fn native_sched_clock() -> c_ulonglong;
}
extern "C" {
    pub fn recalibrate_cpu_khz();
}
extern "C" {
    pub fn using_native_sched_clock() -> bool;
}
extern "C" {
    pub fn paravirt_set_sched_clock((*func)(void): *mut u64);
}
//
// We use the full linear equation: f(x) = a + b*x, in order to allow
// a continuous function in the face of dynamic freq changes.
//
// Continuity means that when our frequency changes our slope (b); we want to
// ensure that: f(t) == f'(t), which gives: a + b*t == a' + b'*t.
//
// Without an offset (a) the above would not be possible.
//
// See the comment near cycles_2_ns() for details on how we compute (b).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cyc2ns_data {
    pub cyc2ns_mul: u32,
    pub cyc2ns_shift: u32,
    pub cyc2ns_offset: u64,
}

extern "C" {
    pub fn cyc2ns_read_begin(: *mut cyc2ns_data);
}
extern "C" {
    pub fn cyc2ns_read_end();
}
