//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kernel/process.h
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
// Code shared between 32 and 64 bit

extern "C" {
    pub fn __switch_to_xtra(prev_p: *mut task_struct, next_p: *mut task_struct);
}
//
// This needs to be inline to optimize for the common case where no extra
// work needs to be done.
//
// Avoid __switch_to_xtra() invocation when conditional
// STIBP is disabled and the only different bit is
// TIF_SPEC_IB. For CONFIG_SMP=n TIF_SPEC_IB is not
// in the TIF_WORK_CTXSW masks.
//
// __switch_to_xtra() handles debug registers, i/o bitmaps,
// speculation mitigations etc.
//
