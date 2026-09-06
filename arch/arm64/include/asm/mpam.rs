//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/mpam.h
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
// Copyright (C) 2025 Arm Ltd.

//
// The value of the MPAM0_EL1 sysreg when a task is in resctrl's default group.
// This is used by the context switch code to use the resctrl CPU property
// instead. The value is modified when CDP is enabled/disabled by mounting
// the resctrl filesystem.
//

//
// The resctrl filesystem writes to the partid/pmg values for threads and CPUs,
// which may race with reads in mpam_thread_switch(). Ensure only one of the old
// or new values are used. Particular care should be taken with the pmg field as
// mpam_thread_switch() may read a partid and pmg that don't match, causing this
// value to be stored with cache allocations, despite being considered 'free' by
// resctrl.
//
extern "C" {
    pub fn READ_ONCE(_arg: task_thread_info(tsk)->mpam_partid_pmg) -> return;
}
// Synchronising the EL0 write is left until the ERET to EL0

