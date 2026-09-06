//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-path-selector.h
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
// Copyright (C) 2003 Sistina Software.
// Copyright (C) 2004 Red Hat, Inc. All rights reserved.
//
// Module Author: Heinz Mauelshagen
//
// This file is released under the GPL.
//
// Path-Selector registration.
//

//
// We provide an abstraction for the code that chooses which path
// to send some io down.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct path_selector {
    pub type: *mut path_selector_type,
    pub context: *mut c_void,
}

//
// If a path selector uses this flag, a high resolution timer is used
// (via ktime_get_ns) to account for IO start time in BIO-based mpath.
// This improves performance of some path selectors (i.e. HST), in
// exchange for slightly higher overhead when submitting the BIO.
// The extra cost is usually offset by improved path selection for
// some benchmarks.
//
// This has no effect for request-based mpath, since it already uses a
// higher precision timer by default.
//
pub const DM_PS_USE_HR_TIMER: c_uint = 0x00000001;

// Information about a path selector type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct path_selector_type {
    pub name: *mut c_char,
    pub module: *mut module,
    pub features: c_uint,
    pub table_args: c_uint,
    pub info_args: c_uint,
//
// Constructs a path selector object, takes custom arguments
//
    pub argv): *mut *mut *mut int (create)(struct path_selector ps, unsigned int argc, char,
    pub ps): *mut *mut void (destroy)(struct path_selector,
//
// Add an opaque path object, along with some selector specific
// path args (eg, path priority).
//
    pub error): *mut *mut *mut int argc, char argv, char,
//
// Chooses a path for this io, if no paths are available then
// NULL will be returned.
//
    pub nr_bytes): *mut *mut *mut *mut dm_path (select_path)(path_selector ps, size_t,
//
// Notify the selector that a path has failed.
//
    pub p): *mut *mut *mut void (fail_path)(struct path_selector ps, struct dm_path,
//
// Ask selector to reinstate a path.
//
    pub p): *mut *mut *mut int (reinstate_path)(struct path_selector ps, struct dm_path,
//
// Table content based on parameters added in ps_add_path_fn
// or path selector status
//
    pub maxlen): *mut *mut status_type_t type, char result, unsigned int,
    pub nr_bytes): usize,
    pub start_time): size_t nr_bytes, u64,
}

// Register a path selector
extern "C" {
    pub fn dm_register_path_selector(type: *mut path_selector_type) -> c_int;
}
// Unregister a path selector
extern "C" {
    pub fn dm_unregister_path_selector(type: *mut path_selector_type);
}
// Returns a registered path selector type
// Releases a path selector
extern "C" {
    pub fn dm_put_path_selector(pst: *mut path_selector_type);
}
