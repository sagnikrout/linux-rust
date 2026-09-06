//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/delay.h
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

// Undefined functions to get compile-time errors
extern "C" {
    pub fn __bad_udelay();
}
extern "C" {
    pub fn __bad_ndelay();
}
extern "C" {
    pub fn __udelay(usecs: c_ulong);
}
extern "C" {
    pub fn __ndelay(nsecs: c_ulong);
}
extern "C" {
    pub fn __const_udelay(xloops: c_ulong);
}
extern "C" {
    pub fn __delay(loops: c_ulong);
}
//
// The microseconds/nanosecond delay multiplicators are used to convert a
// constant microseconds/nanoseconds value to a value which can be used by the
// architectures specific implementation to transform it into loops.
//

//
// The maximum constant udelay/ndelay value picked out of thin air to prevent
// too long constant udelays/ndelays.
//
pub const DELAY_CONST_MAX: c_int = 20000;
//
// udelay - Inserting a delay based on microseconds with busy waiting
// @usec:	requested delay in microseconds
//
// When delaying in an atomic context ndelay(), udelay() and mdelay() are the
// only valid variants of delaying/sleeping to go with.
//
// When inserting delays in non atomic context which are shorter than the time
// which is required to queue e.g. an hrtimer and to enter then the scheduler,
// it is also valuable to use udelay(). But it is not simple to specify a
// generic threshold for this which will fit for all systems. An approximation
// is a threshold for all delays up to 10 microseconds.
//
// When having a delay which is larger than the architecture specific
// %MAX_UDELAY_MS value, please make sure mdelay() is used. Otherwise a overflow
// risk is given.
//
// Please note that ndelay(), udelay() and mdelay() may return early for several
// reasons (https://lists.openwall.net/linux-kernel/2011/01/09/56):
//
// #. computed loops_per_jiffy too low (due to the time taken to execute the
// timer interrupt.)
// #. cache behaviour affecting the time it takes to execute the loop function.
// #. CPU clock rate changes.
//
// ndelay - Inserting a delay based on nanoseconds with busy waiting
// @nsec:	requested delay in nanoseconds
//
// See udelay() for basic information about ndelay() and it's variants.
//

