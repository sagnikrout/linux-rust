//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/file_ref.h
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


// SPDX-License-Identifier: GPL-2.0-only

//
// file_ref is a reference count implementation specifically for use by
// files. It takes inspiration from rcuref but differs in key aspects
// such as support for SLAB_TYPESAFE_BY_RCU type caches.
//
// FILE_REF_ONEREF                FILE_REF_MAXREF
// 0x0000000000000000UL      0x7FFFFFFFFFFFFFFFUL
// <-------------------valid ------------------->
//
// FILE_REF_SATURATED
// 0x8000000000000000UL 0xA000000000000000UL 0xBFFFFFFFFFFFFFFFUL
// <-----------------------saturation zone---------------------->
//
// FILE_REF_RELEASED                   FILE_REF_DEAD
// 0xC000000000000000UL         0xE000000000000000UL
// <-------------------dead zone------------------->
//
// FILE_REF_NOREF
// 0xFFFFFFFFFFFFFFFFUL
//

pub const FILE_REF_ONEREF: c_uint = 0x0000000000000000UL;
pub const FILE_REF_MAXREF: c_uint = 0x7FFFFFFFFFFFFFFFUL;
pub const FILE_REF_SATURATED: c_uint = 0xA000000000000000UL;
pub const FILE_REF_RELEASED: c_uint = 0xC000000000000000UL;
pub const FILE_REF_DEAD: c_uint = 0xE000000000000000UL;
pub const FILE_REF_NOREF: c_uint = 0xFFFFFFFFFFFFFFFFUL;

pub const FILE_REF_ONEREF: c_uint = 0x00000000U;
pub const FILE_REF_MAXREF: c_uint = 0x7FFFFFFFU;
pub const FILE_REF_SATURATED: c_uint = 0xA0000000U;
pub const FILE_REF_RELEASED: c_uint = 0xC0000000U;
pub const FILE_REF_DEAD: c_uint = 0xE0000000U;
pub const FILE_REF_NOREF: c_uint = 0xFFFFFFFFU;

//
// file_ref_init - Initialize a file reference count
// @ref: Pointer to the reference count
// @cnt: The initial reference count typically '1'
//
extern "C" {
    pub fn __file_ref_put(ref: *mut file_ref_t, cnt: c_ulong) -> bool;
}
//
// file_ref_get - Acquire one reference on a file
// @ref: Pointer to the reference count
//
// Similar to atomic_inc_not_zero() but saturates at FILE_REF_MAXREF.
//
// Provides full memory ordering.
//
// Return: False if the attempt to acquire a reference failed. This happens
// when the last reference has been put already. True if a reference
// was successfully acquired
//
// Unconditionally increase the reference count with full
// ordering. The saturation and dead zones provide enough
// tolerance for this.
//
// If this indicates negative the file in question the fail can
// be freed and immediately reused due to SLAB_TYPSAFE_BY_RCU.
// Hence, unconditionally altering the file reference count to
// e.g., reset the file reference count back to the middle of
// the deadzone risk end up marking someone else's file as dead
// behind their back.
//
// It would be possible to do a careful:
//
// cnt = atomic_long_inc_return();
// if (likely(cnt >= 0))
// return true;
//
// and then something like:
//
// if (cnt >= FILE_REF_RELEASE)
// atomic_long_try_cmpxchg(&ref->refcnt, &cnt, FILE_REF_DEAD),
//
// to set the value back to the middle of the deadzone. But it's
// practically impossible to go from FILE_REF_DEAD to
// FILE_REF_ONEREF. It would need 2305843009213693952/2^61
// file_ref_get()s to resurrect such a dead file.
//
// file_ref_inc - Acquire one reference on a file
// @ref: Pointer to the reference count
//
// Acquire an additional reference on a file. Warns if the caller didn't
// already hold a reference.
//
// file_ref_put -- Release a file reference
// @ref:	Pointer to the reference count
//
// Provides release memory ordering, such that prior loads and stores
// are done before, and provides an acquire ordering on success such
// that free() must come after.
//
// Return: True if this was the last reference with no future references
// possible. This signals the caller that it can safely release
// the object which is protected by the reference counter.
// False if there are still active references or the put() raced
// with a concurrent get()/put() pair. Caller is not allowed to
// release the protected object.
//
// While files are SLAB_TYPESAFE_BY_RCU and thus file_ref_put()
// calls don't risk UAFs when a file is recyclyed, it is still
// vulnerable to UAFs caused by freeing the whole slab page once
// it becomes unused. Prevent file_ref_put() from being
// preempted protects against this.
//
// Unconditionally decrease the reference count. The saturation
// and dead zones provide enough tolerance for this. If this
// fails then we need to handle the last reference drop and
// cases inside the saturation and dead zones.
//
extern "C" {
    pub fn __file_ref_put(_arg: ref, _arg: cnt) -> return;
}
//
// file_ref_put_close - drop a reference expecting it would transition to FILE_REF_NOREF
// @ref:	Pointer to the reference count
//
// Semantically it is equivalent to calling file_ref_put(), but it trades lower
// performance in face of other CPUs also modifying the refcount for higher
// performance when this happens to be the last reference.
//
// For the last reference file_ref_put() issues 2 atomics. One to drop the
// reference and another to transition it to FILE_REF_DEAD. This routine does
// the work in one step, but in order to do it has to pre-read the variable which
// decreases scalability.
//
// Use with close() et al, stick to file_ref_put() by default.
//
extern "C" {
    pub fn file_ref_put(_arg: ref) -> return;
}
//
// file_ref_read - Read the number of file references
// @ref: Pointer to the reference count
//
// Return: The number of held references (0 ... N)
//
// Return 0 if within the DEAD zone.
//
// __file_ref_read_raw - Return the value stored in ref->refcnt
// @ref: Pointer to the reference count
//
// Return: The raw value found in the counter
//
// A hack for file_needs_f_pos_lock(), you probably want to use
// file_ref_read() instead.
//
extern "C" {
    pub fn atomic_long_read(_arg: &ref->refcnt) -> return;
}
