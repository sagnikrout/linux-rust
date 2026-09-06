//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hpet.h
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
pub const __HPET__: c_int = 1;

//
// Offsets into HPET Registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpet {
    pub /: *mut *mut u64 hpet_cap; / capabilities,
    pub /: *mut *mut u64 res0; / reserved,
    pub /: *mut *mut u64 hpet_config; / configuration,
    pub /: *mut *mut u64 res1; / reserved,
    pub /: *mut *mut u64 hpet_isr; / interrupt status reg,
    pub /: *mut *mut u64 res2[25]; / reserved,
    pub _hpet_mc64: u64,
    pub _hpet_mc32: u32,
    pub _hpet_mc: c_ulong,
    pub _u0: },
    pub /: *mut *mut u64 res3; / reserved,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpet_timer {
    pub /: *mut *mut u64 hpet_config; / configuration/cap,
    pub _hpet_hc64: u64,
    pub _hpet_hc32: u32,
    pub _hpet_compare: c_ulong,
    pub _u1: },
    pub /: *mut *mut u64 hpet_fsb[2]; / FSB route,
    pub hpet_timers: [}; ],
}

//
// HPET general capabilities register
//

//
// HPET general configuration register
//

//
// Timer configuration register
//

//
// Timer FSB Interrupt Route Register
//

//
// exported interfaces
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpet_data {
    pub hd_phys_address: c_ulong,
    pub hd_address: *mut void __iomem,
    pub hd_nirqs: c_ushort,
    pub /: *mut *mut unsigned int hd_state; / timer allocated,
    pub hd_irq: [c_uint; HPET_MAX_TIMERS],
}

extern "C" {
    pub fn hpet_alloc(: *mut hpet_data) -> c_int;
}
