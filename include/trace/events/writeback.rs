//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/writeback.h
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

// enums need to be exported to user space

//
// Now redefine the EM() and EMe() macros to map the enums to the strings
// that will be printed in the output.
//

// may be called for files on pseudo FSes w/ unregistered bdi

extern "C" {
    pub fn cgroup_ino(_arg: wb->memcg_css->cgroup) -> return;
}
extern "C" {
    pub fn __trace_wb_assign_cgroup(_arg: wbc->wb) -> return;
}

//
// Inode writeback list tracking.
//

// This part must be outside protection
