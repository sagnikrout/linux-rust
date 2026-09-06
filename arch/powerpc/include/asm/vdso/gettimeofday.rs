//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/vdso/gettimeofday.h
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

pub const VDSO_HAS_CLOCK_GETRES: c_int = 1;
pub const VDSO_HAS_TIME: c_int = 1;
//
// powerpc specific delta calculation.
//
// This variant removes the masking of the subtraction because the
// clocksource mask of all VDSO capable clocksources on powerpc is U64_MAX
// which would result in a pointless operation. The compiler cannot
// optimize it away as the mask comes from the vdso data and is not compile
// time constant.
//
pub const VDSO_DELTA_NOMASK: c_int = 1;
extern "C" {
    pub fn asm(_arg: "r3") -> register int ret;
}
extern "C" {
    pub fn do_syscall_2(_arg: __NR_gettimeofday, long)_tv: (unsigned, long)_tz: (unsigned) -> return;
}

extern "C" {
    pub fn do_syscall_2(_arg: __NR_clock_gettime, _arg: _clkid, long)_ts: (unsigned) -> return;
}
extern "C" {
    pub fn do_syscall_2(_arg: __NR_clock_getres, _arg: _clkid, long)_ts: (unsigned) -> return;
}

pub const BUILD_VDSO32: c_int = 1;
extern "C" {
    pub fn do_syscall_2(_arg: __NR_clock_gettime64, _arg: _clkid, long)_ts: (unsigned) -> return;
}
extern "C" {
    pub fn do_syscall_2(_arg: __NR_clock_getres_time64, _arg: _clkid, long)_ts: (unsigned) -> return;
}
extern "C" {
    pub fn do_syscall_2(_arg: __NR_clock_gettime, _arg: _clkid, long)_ts: (unsigned) -> return;
}
extern "C" {
    pub fn do_syscall_2(_arg: __NR_clock_getres, _arg: _clkid, long)_ts: (unsigned) -> return;
}

extern "C" {
    pub fn get_tb() -> return;
}

