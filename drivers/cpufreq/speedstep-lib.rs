//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/cpufreq/speedstep-lib.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// (C) 2002 - 2003 Dominik Brodowski <linux@brodo.de>
//
// Library for common functions for Intel SpeedStep v.1 and v.2 support
//
// BIG FAT DISCLAIMER: Work in progress code. Possibly *dangerous
//
// processors
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum speedstep_processor {
    SPEEDSTEP_CPU_PIII_C_EARLY = 0x00000001,  /* Coppermine core */
    SPEEDSTEP_CPU_PIII_C	   = 0x00000002,  /* Coppermine core */
    SPEEDSTEP_CPU_PIII_T	   = 0x00000003,  /* Tualatin core */
    SPEEDSTEP_CPU_P4M	   = 0x00000004,  /* P4-M  */
// the following processors are not speedstep-capable and are not auto-detected
// in speedstep_detect_processor(). However, their speed can be detected using
// the speedstep_get_frequency() call.
    SPEEDSTEP_CPU_PM	   = 0xFFFFFF03,  /* Pentium M  */
    SPEEDSTEP_CPU_P4D	   = 0xFFFFFF04,  /* desktop P4  */
    SPEEDSTEP_CPU_PCORE	   = 0xFFFFFF05,  /* Core */
}

// speedstep states -- only two of them
pub const SPEEDSTEP_HIGH: c_uint = 0x00000000;
pub const SPEEDSTEP_LOW: c_uint = 0x00000001;
// detect a speedstep-capable processor
extern "C" {
    pub fn speedstep_detect_processor() -> speedstep_processor;
}
// detect the current speed (in khz) of the processor
extern "C" {
    pub fn speedstep_get_frequency(processor: speedstep_processor) -> c_uint;
}
// detect the low and high speeds of the processor. The callback
// set_state"'s first argument is either SPEEDSTEP_HIGH or
// SPEEDSTEP_LOW; the second argument is zero so that no
// cpufreq_notify_transition calls are initiated.
//
