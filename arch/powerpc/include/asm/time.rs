//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/time.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Common time prototypes and such for all ppc machines.
//
// Written by Cort Dougan (cort@cs.nmt.edu) to merge
// Paul Mackerras' version and mine for PReP and Pmac.
//

// time.c
extern "C" {
    pub fn generic_calibrate_decr();
}

extern "C" {
    pub fn get_boot_tb() -> u64;
}

// Some sane defaults: 125 MHz timebase, 1GHz processor

#[repr(C)]
#[derive(Copy, Clone)]
pub struct div_result {
    pub result_high: u64,
    pub result_low: u64,
}

extern "C" {
    pub fn mfspr(_arg: SPRN_VTB) -> return;
}
// Accessor functions for the decrementer register.
// The 4xx doesn't even have a decrementer.  I tried to use the
// generic timer interrupt code, which seems OK, with the 4xx PIT
// in auto-reload mode.  The problem is PIT stops counting when it
// hits zero.  If it would wrap, we could use it just like a decrementer.
//
extern "C" {
    pub fn mfspr(_arg: SPRN_DEC) -> return;
}
//
// Note: Book E and 4xx processors differ from other PowerPC processors
// in when the decrementer generates its interrupt: on the 1 to 0
// transition for Book E/4xx, but on the 0 to -1 transition for others.
//

extern "C" {
    pub fn secondary_cpu_time_init();
}
extern "C" {
    pub fn time_init() -> void __init;
}
extern "C" {
    pub fn __this_cpu_read(_arg: decrementers_next_tb) -> return;
}

extern "C" {
    pub fn timer_rearm_host_dec(now: u64);
}

// Convert timebase ticks to nanoseconds
extern "C" {
    pub fn tb_to_ns(tb_ticks: c_ulonglong) -> c_ulonglong;
}
extern "C" {
    pub fn timer_broadcast_interrupt();
}
// SPLPAR and VIRT_CPU_ACCOUNTING_NATIVE
extern "C" {
    pub fn pseries_accumulate_stolen_time();
}
extern "C" {
    pub fn pseries_calculate_stolen_time(stop_tb: u64) -> u64;
}

