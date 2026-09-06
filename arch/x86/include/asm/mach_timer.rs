//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/mach_timer.h
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
// Machine specific calibrate_tsc() for generic.
// Split out from timer_tsc.c by Osamu Tomita <tomita@cinet.co.jp>
//
// ------ Calibrate the TSC -------
// Return 2^32 * (1 / (TSC clocks per usec)) for do_fast_gettimeoffset().
// Too much 64-bit arithmetic here to do this cleanly in C, and for
// accuracy's sake we want to keep the overhead on the CTC speaker (channel 2)
// output busy loop as low as possible. We avoid reading the CTC registers
// directly because of the awkward 8-bit access mechanism of the 82C54
// device.
//

// Set the Gate high, disable speaker
//
// Now let's take care of CTC channel 2
//
// Set the Gate high, program CTC channel 2 for mode 0,
// (interrupt on terminal count mode), binary count,
// load 5 * LATCH count, (LSB and MSB) to begin countdown.
//
// Some devices need a delay here.
//
// count_p = count;
