//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rcuref.h
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

pub const RCUREF_ONEREF: c_uint = 0x00000000U;
pub const RCUREF_MAXREF: c_uint = 0x7FFFFFFFU;
pub const RCUREF_SATURATED: c_uint = 0xA0000000U;
pub const RCUREF_RELEASED: c_uint = 0xC0000000U;
pub const RCUREF_DEAD: c_uint = 0xE0000000U;
pub const RCUREF_NOREF: c_uint = 0xFFFFFFFFU;
//
// rcuref_init - Initialize a rcuref reference count with the given reference count
// @ref:	Pointer to the reference count
// @cnt:	The initial reference count typically '1'
//
// rcuref_read - Read the number of held reference counts of a rcuref
// @ref:	Pointer to the reference count
//
// Return: The number of held references (0 ... N). The value 0 does not
// indicate that it is safe to schedule the object, protected by this reference
// counter, for deconstruction.
// If you want to know if the reference counter has been marked DEAD (as
// signaled by rcuref_put()) please use rcuread_is_dead().
//
// Return 0 if within the DEAD zone.
//
// rcuref_is_dead -	Check if the rcuref has been already marked dead
// @ref:		Pointer to the reference count
//
// Return: True if the object has been marked DEAD. This signals that a previous
// invocation of rcuref_put() returned true on this reference counter meaning
// the protected object can safely be scheduled for deconstruction.
// Otherwise, returns false.
//
extern "C" {
    pub fn rcuref_get_slowpath(ref: *mut rcuref_t) -> __must_check bool;
}
//
// rcuref_get - Acquire one reference on a rcuref reference count
// @ref:	Pointer to the reference count
//
// Similar to atomic_inc_not_zero() but saturates at RCUREF_MAXREF.
//
// Provides no memory ordering, it is assumed the caller has guaranteed the
// object memory to be stable (RCU, etc.). It does provide a control dependency
// and thereby orders future stores. See documentation in lib/rcuref.c
//
// Return:
// False if the attempt to acquire a reference failed. This happens
// when the last reference has been put already
//
// True if a reference was successfully acquired
//
// Unconditionally increase the reference count. The saturation and
// dead zones provide enough tolerance for this.
//
// Handle the cases inside the saturation and dead zones
extern "C" {
    pub fn rcuref_get_slowpath(_arg: ref) -> return;
}
extern "C" {
    pub fn rcuref_put_slowpath(ref: *mut rcuref_t, cnt: c_uint) -> __must_check bool;
}
//
// Internal helper. Do not invoke directly.
//
// Unconditionally decrease the reference count. The saturation and
// dead zones provide enough tolerance for this.
//
// Handle the last reference drop and cases inside the saturation
// and dead zones.
//
extern "C" {
    pub fn rcuref_put_slowpath(_arg: ref, _arg: cnt) -> return;
}
//
// rcuref_put_rcusafe -- Release one reference for a rcuref reference count RCU safe
// @ref:	Pointer to the reference count
//
// Provides release memory ordering, such that prior loads and stores are done
// before, and provides an acquire ordering on success such that free()
// must come after.
//
// Can be invoked from contexts, which guarantee that no grace period can
// happen which would free the object concurrently if the decrement drops
// the last reference and the slowpath races against a concurrent get() and
// put() pair. rcu_read_lock()'ed and atomic contexts qualify.
//
// Return:
// True if this was the last reference with no future references
// possible. This signals the caller that it can safely release the
// object which is protected by the reference counter.
//
// False if there are still active references or the put() raced
// with a concurrent get()/put() pair. Caller is not allowed to
// release the protected object.
//
extern "C" {
    pub fn __rcuref_put(_arg: ref) -> return;
}
//
// rcuref_put -- Release one reference for a rcuref reference count
// @ref:	Pointer to the reference count
//
// Can be invoked from any context.
//
// Provides release memory ordering, such that prior loads and stores are done
// before, and provides an acquire ordering on success such that free()
// must come after.
//
// Return:
//
// True if this was the last reference with no future references
// possible. This signals the caller that it can safely schedule the
// object, which is protected by the reference counter, for
// deconstruction.
//
// False if there are still active references or the put() raced
// with a concurrent get()/put() pair. Caller is not allowed to
// deconstruct the protected object.
//
