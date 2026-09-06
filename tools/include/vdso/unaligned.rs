//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/vdso/unaligned.h
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
// __get_unaligned_t - read an unaligned value from memory.
// @type:	the type to load from the pointer.
// @ptr:	the pointer to load from.
//
// Use memcpy to affect an unaligned type sized load avoiding undefined behavior
// from approaches like type punning that require -fno-strict-aliasing in order
// to be correct. As type may be const, use __unqual_scalar_typeof to map to a
// non-const type - you can't memcpy into a const type. The
// __get_unaligned_ctrl_type gives __unqual_scalar_typeof its required
// expression rather than type, a pointer is used to avoid warnings about mixing
// the use of 0 and NULL. The void* cast silences ubsan warnings.
//

//
// __put_unaligned_t - write an unaligned value to memory.
// @type:	the type of the value to store.
// @val:	the value to store.
// @ptr:	the pointer to store to.
//
// Use memcpy to affect an unaligned type sized store avoiding undefined
// behavior from approaches like type punning that require -fno-strict-aliasing
// in order to be correct. The void* cast silences ubsan warnings.
//

