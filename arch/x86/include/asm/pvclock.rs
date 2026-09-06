//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pvclock.h
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

// some helper functions for xen and kvm pv clock sources
extern "C" {
    pub fn pvclock_clocksource_read(src: *mut pvclock_vcpu_time_info) -> u64;
}
extern "C" {
    pub fn pvclock_clocksource_read_nowd(src: *mut pvclock_vcpu_time_info) -> u64;
}
extern "C" {
    pub fn pvclock_read_flags(src: *mut pvclock_vcpu_time_info) -> u8;
}
extern "C" {
    pub fn pvclock_set_flags(flags: u8);
}
extern "C" {
    pub fn pvclock_tsc_khz(src: *mut pvclock_vcpu_time_info) -> c_ulong;
}
extern "C" {
    pub fn pvclock_resume();
}
extern "C" {
    pub fn pvclock_touch_watchdogs();
}
// Make sure that the version is read before the data.
// Make sure that the version is re-read after the data.
extern "C" {
    pub fn unlikely(src->version: version !=) -> return;
}
//
// Scale a 64-bit delta by scaling and multiplying by a 32-bit fraction,
// yielding a 64-bit result.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvclock_vsyscall_time_info {
    pub pvti: pvclock_vcpu_time_info,
    pub __attribute__((__aligned__(SMP_CACHE_BYTES))): },

    pub pvti): *mut void pvclock_set_pvti_cpu0_va(struct pvclock_vsyscall_time_info,
    pub pvclock_get_pvti_cpu0_va(void): *mut pvclock_vsyscall_time_info,

    pub NULL: return,

