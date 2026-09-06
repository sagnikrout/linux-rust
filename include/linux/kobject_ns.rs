//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kobject_ns.h
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
// Kernel object name space definitions
//
// Copyright (c) 2002-2003 Patrick Mochel
// Copyright (c) 2002-2003 Open Source Development Labs
// Copyright (c) 2006-2008 Greg Kroah-Hartman <greg@kroah.com>
// Copyright (c) 2006-2008 Novell Inc.
//
// Split from kobject.h by David Howells (dhowells@redhat.com)
//
// Please read Documentation/core-api/kobject.rst before using the kobject
// interface, ESPECIALLY the parts about reference counts and object
// destructors.
//
// Namespace types which are used to tag kobjects and sysfs entries.
// Network namespace will likely be the first.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kobj_ns_type {
    KOBJ_NS_TYPE_NONE = 0,
    KOBJ_NS_TYPE_NET,
    KOBJ_NS_TYPES
}

//
// Callbacks so sysfs can determine namespaces
// @grab_current_ns: return a new reference to calling task's namespace
// @netlink_ns: return namespace to which a sock belongs (right?)
// @initial_ns: return the initial namespace (i.e. init_net_ns)
// @drop_ns: drops a reference to namespace
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kobj_ns_type_operations {
    pub type: kobj_ns_type,
    pub (*current_may_mount)(void): *mut bool,
    pub (*grab_current_ns)(void): *mut ns_common,
    pub sk): *const *const *const ns_common (netlink_ns)(sock,
    pub (*initial_ns)(void): *const ns_common,
    pub ): *mut *mut void (drop_ns)(struct ns_common,
}

extern "C" {
    pub fn kobj_ns_type_register(ops: *const kobj_ns_type_operations) -> c_int;
}
extern "C" {
    pub fn kobj_ns_type_registered(type: kobj_ns_type) -> c_int;
}
extern "C" {
    pub fn kobj_ns_current_may_mount(type: kobj_ns_type) -> bool;
}
extern "C" {
    pub fn kobj_ns_drop(type: kobj_ns_type, ns: *mut ns_common);
}
