//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kref.h
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
// kref.h - library routines for handling generic reference counted objects
//
// Copyright (C) 2004 Greg Kroah-Hartman <greg@kroah.com>
// Copyright (C) 2004 IBM Corp.
//
// based on kobject.h which was:
// Copyright (C) 2002-2003 Patrick Mochel <mochel@osdl.org>
// Copyright (C) 2002-2003 Open Source Development Labs
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kref {
    pub refcount: refcount_t,
}

//
// kref_init - initialize object.
// @kref: object in question.
//
extern "C" {
    pub fn refcount_read(_arg: &kref->refcount) -> return;
}
//
// kref_get - increment refcount for object.
// @kref: object.
//
// kref_put - Decrement refcount for object
// @kref: Object
// @release: Pointer to the function that will clean up the object when the
// last reference to the object is released.
//
// Decrement the refcount, and if 0, call @release.  The caller may not
// pass NULL or kfree() as the release function.
//
// Return: 1 if this call removed the object, otherwise return 0.  Beware,
// if this function returns 0, another caller may have removed the object
// by the time this function returns.  The return value is only certain
// if you want to see if the object is definitely released.
//
// kref_put_mutex - Decrement refcount for object
// @kref: Object
// @release: Pointer to the function that will clean up the object when the
// last reference to the object is released.
// @mutex: Mutex which protects the release function.
//
// This variant of kref_lock() calls the @release function with the @mutex
// held.  The @release function will release the mutex.
//
// kref_put_lock - Decrement refcount for object
// @kref: Object
// @release: Pointer to the function that will clean up the object when the
// last reference to the object is released.
// @lock: Spinlock which protects the release function.
//
// This variant of kref_lock() calls the @release function with the @lock
// held.  The @release function will release the lock.
//
// kref_get_unless_zero - Increment refcount for object unless it is zero.
// @kref: object.
//
// This function is intended to simplify locking around refcounting for
// objects that can be looked up from a lookup structure, and which are
// removed from that lookup structure in the object destructor.
// Operations on such objects require at least a read lock around
// lookup + kref_get, and a write lock around kref_put + remove from lookup
// structure. Furthermore, RCU implementations become extremely tricky.
// With a lookup followed by a kref_get_unless_zero *with return value check
// locking in the kref_put path can be deferred to the actual removal from
// the lookup structure and RCU lookups become trivial.
//
// Return: non-zero if the increment succeeded. Otherwise return 0.
//
extern "C" {
    pub fn refcount_inc_not_zero(_arg: &kref->refcount) -> return;
}
