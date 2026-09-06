//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/mc146818rtc.h
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
// Machine dependent access functions for RTC registers.
//

//
// This lock provides nmi access to the CMOS/RTC registers.  It has some
// special properties.  It is owned by a CPU and stores the index register
// currently being accessed (if owned).  The idea here is that it works
// like a normal lock (normally).  However, in an NMI, the NMI code will
// first check to see if its CPU owns the lock, meaning that the NMI
// interrupted during the read/write of the device.  If it does, it goes ahead
// and performs the access and then restores the index register.  If it does
// not, it locks normally.
//
// Note that since we are working with NMIs, we need this lock even in
// a non-SMP machine just to mark that the lock is owned.
//
// This only works with compare-and-swap.  There is no other way to
// atomically claim the lock and set the owner.
//

//
// All of these below must be called with interrupts off, preempt
// disabled, etc.
//

pub const do_i_have_lock_cmos(): c_int = 0;
pub const current_lock_cmos_reg(): c_int = 0;

//
// The yet supported machines all access the RTC index register via
// an ISA port access but the way to access the date register differs ...
//

extern "C" {
    pub fn rtc_cmos_read(addr: c_uchar) -> c_uchar;
}
extern "C" {
    pub fn rtc_cmos_write(val: c_uchar, addr: c_uchar);
}
extern "C" {
    pub fn mach_set_cmos_time(now: *const timespec64) -> c_int;
}
extern "C" {
    pub fn mach_get_cmos_time(now: *mut timespec64);
}
pub const RTC_IRQ: c_int = 8;
