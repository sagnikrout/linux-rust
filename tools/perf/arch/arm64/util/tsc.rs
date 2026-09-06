//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/arm64/util/tsc.c
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

#[no_mangle]
pub unsafe extern "C" fn rdtsc() -> u64 {
    u64 rdtsc(void)
    {
    u64 val;
//
// According to ARM DDI 0487F.c, from Armv8.0 to Armv8.5 inclusive, the
// system counter is at least 56 bits wide; from Armv8.6, the counter
// must be 64 bits wide.  So the system counter could be less than 64
// bits wide and it is attributed with the flag 'cap_user_time_short'
// is true.
//
    asm volatile("mrs %0, cntvct_el0" : "=r" (val));
    return val;
    }
