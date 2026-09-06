//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/instrumented.h
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
// This header provides generic wrappers for memory access instrumentation that
// the compiler cannot emit for: KASAN, KCSAN, KMSAN.
//

//
// instrument_read - instrument regular read access
// @v: address of access
// @size: size of access
//
// Instrument a regular read access. The instrumentation should be inserted
// before the actual read happens.
//
// instrument_write - instrument regular write access
// @v: address of access
// @size: size of access
//
// Instrument a regular write access. The instrumentation should be inserted
// before the actual write happens.
//
// instrument_read_write - instrument regular read-write access
// @v: address of access
// @size: size of access
//
// Instrument a regular write access. The instrumentation should be inserted
// before the actual write happens.
//

//
// instrument_atomic_read - instrument atomic read access
// @v: address of access
// @size: size of access
//
// Instrument an atomic read access. The instrumentation should be inserted
// before the actual read happens.
//
// instrument_atomic_write - instrument atomic write access
// @v: address of access
// @size: size of access
//
// Instrument an atomic write access. The instrumentation should be inserted
// before the actual write happens.
//
// instrument_atomic_read_write - instrument atomic read-write access
// @v: address of access
// @size: size of access
//
// Instrument an atomic read-write access. The instrumentation should be
// inserted before the actual write happens.
//
// instrument_copy_to_user - instrument reads of copy_to_user
// @to: destination address
// @from: source address
// @n: number of bytes to copy
//
// Instrument reads from kernel memory, that are due to copy_to_user (and
// variants). The instrumentation must be inserted before the accesses.
//
// instrument_copy_from_user_before - add instrumentation before copy_from_user
// @to: destination address
// @from: source address
// @n: number of bytes to copy
//
// Instrument writes to kernel memory, that are due to copy_from_user (and
// variants). The instrumentation should be inserted before the accesses.
//
// instrument_copy_from_user_after - add instrumentation after copy_from_user
// @to: destination address
// @from: source address
// @n: number of bytes to copy
// @left: number of bytes not copied (as returned by copy_from_user)
//
// Instrument writes to kernel memory, that are due to copy_from_user (and
// variants). The instrumentation should be inserted after the accesses.
//
// instrument_memcpy_before - add instrumentation before non-instrumented memcpy
// @to: destination address
// @from: source address
// @n: number of bytes to copy
//
// Instrument memory accesses that happen in custom memcpy implementations. The
// instrumentation should be inserted before the memcpy call.
//
// instrument_memcpy_after - add instrumentation after non-instrumented memcpy
// @to: destination address
// @from: source address
// @n: number of bytes to copy
// @left: number of bytes not copied (if known)
//
// Instrument memory accesses that happen in custom memcpy implementations. The
// instrumentation should be inserted after the memcpy call.
//
// instrument_get_user() - add instrumentation to get_user()-like macros
// @to: destination variable, may not be address-taken
//
// get_user() and friends are fragile, so it may depend on the implementation
// whether the instrumentation happens before or after the data is copied from
// the userspace.
//

//
// instrument_put_user() - add instrumentation to put_user()-like macros
// @from: source address
// @ptr: userspace pointer to copy to
// @size: number of bytes to copy
//
// put_user() and friends are fragile, so it may depend on the implementation
// whether the instrumentation happens before or after the data is copied from
// the userspace.
//

