//! Automatically rewritten from C Header to Rust Module
//! Source: include/vdso/helpers.h
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
// Variant of vdso_read_begin() to handle VDSO_CLOCKMODE_TIMENS.
//
// Time namespace enabled tasks have a special VVAR page installed which has
// vc->seq set to 1 and vc->clock_mode set to VDSO_CLOCKMODE_TIMENS. For non
// time namespace affected tasks this does not affect performance because if
// vc->seq is odd, i.e. a concurrent update is in progress the extra check for
// vc->clock_mode is just a few extra instructions while spin waiting for
// vc->seq to become even again.
//
extern "C" {
    pub fn unlikely(start: seq !=) -> return;
}
//
// WRITE_ONCE() is required otherwise the compiler can validly tear
// updates to vc->seq and it is possible that the value seen by the
// reader is inconsistent.
//
// WRITE_ONCE() is required otherwise the compiler can validly tear
// updates to vc->seq and it is possible that the value seen by the
// reader is inconsistent.
//
// Ensure the sequence invalidation is visible before data is modified
// Ensure the data update is visible before the sequence is set valid again
// Ensure the sequence invalidation is visible before data is modified
// Ensure the data update is visible before the sequence is set valid again

