//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqflags.h
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
// include/linux/irqflags.h
//
// IRQ flags tracing: follow the state of the hardirq and softirq flags and
// provide callbacks for transitions between ON and OFF states.
//
// This file gets included from lowlevel asm headers too, to provide
// wrapped versions of the local_irq_*() APIs, based on the
// raw_local_irq_*() macros from the lowlevel headers.
//

// Currently lockdep_softirqs_on/off is used only by lockdep

extern "C" {
    pub fn lockdep_softirqs_on(ip: c_ulong);
}
extern "C" {
    pub fn lockdep_softirqs_off(ip: c_ulong);
}
extern "C" {
    pub fn lockdep_hardirqs_on_prepare();
}
extern "C" {
    pub fn lockdep_hardirqs_on(ip: c_ulong);
}
extern "C" {
    pub fn lockdep_hardirqs_off(ip: c_ulong);
}

extern "C" {
    pub fn trace_hardirqs_on_prepare();
}
extern "C" {
    pub fn trace_hardirqs_off_finish();
}
extern "C" {
    pub fn trace_hardirqs_on();
}
extern "C" {
    pub fn trace_hardirqs_off();
}

extern "C" {
    pub fn stop_critical_timings();
}
extern "C" {
    pub fn start_critical_timings();
}

extern "C" {
    pub fn warn_bogus_irq_restore();
}

//
// Wrap the arch provided IRQ routines to provide appropriate checks.
//

//
// The local_irq_*() APIs are equal to the raw_local_irq*()
// if !TRACE_IRQFLAGS.
//

//
// Some architectures don't define arch_irqs_disabled(), so even if either
// definition would be fine we need to use different ones for the time being
// to avoid build issues.
//

