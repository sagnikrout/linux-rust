//! Automatically rewritten from C Header to Rust Module
//! Source: security/landlock/object.h
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
// Landlock LSM - Object management
//
// Copyright © 2016-2020 Mickaël Salaün <mic@digikod.net>
// Copyright © 2018-2020 ANSSI
//

//
// struct landlock_object_underops - Operations on an underlying object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_object_underops {
//
// @release: Releases the underlying object (e.g. iput() for an inode).
//
}

//
// struct landlock_object - Security blob tied to a kernel object
//
// The goal of this structure is to enable to tie a set of ephemeral access
// rights (pertaining to different domains) to a kernel object (e.g an inode)
// in a safe way.  This implies to handle concurrent use and modification.
//
// The lifetime of a &struct landlock_object depends on the rules referring to
// it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_object {
//
// @usage: This counter is used to tie an object to the rules matching
// it or to keep it alive while adding a new rule.  If this counter
// reaches zero, this struct must not be modified, but this counter can
// still be read from within an RCU read-side critical section.  When
// adding a new rule to an object with a usage counter of zero, we must
// wait until the pointer to this object is set to NULL (or recycled).
//
    pub usage: refcount_t,
//
// @lock: Protects against concurrent modifications.  This lock must be
// held from the time @usage drops to zero until any weak references
// from @underobj to this object have been cleaned up.
//
// Lock ordering: inode->i_lock nests inside this.
//
    pub lock: spinlock_t,
//
// @underobj: Used when cleaning up an object and to mark an object as
// tied to its underlying kernel structure.  This pointer is protected
// by @lock.  Cf. landlock_release_inodes() and release_inode().
//
    pub underobj: *mut c_void,
//
// @rcu_free: Enables lockless use of @usage, @lock and
// @underobj from within an RCU read-side critical section.
// @rcu_free and @underops are only used by
// landlock_put_object().
//
    pub rcu_free: rcu_head,
//
// @underops: Enables landlock_put_object() to release the
// underlying object (e.g. inode).
//
    pub underops: *const landlock_object_underops,
}

extern "C" {
    pub fn landlock_put_object(object: *const *const landlock_object);
}
