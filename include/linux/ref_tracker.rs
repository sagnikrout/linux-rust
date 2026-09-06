//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ref_tracker.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ref_tracker_dir {

    pub lock: spinlock_t,
    pub quarantine_avail: c_uint,
    pub untracked: refcount_t,
    pub no_tracker: refcount_t,
    pub dead: bool,
    pub /: *mut *mut list_head list; / List of active trackers,
    pub /: *mut *mut list_head quarantine; / List of dead trackers,
    pub /: *const *const *const char class; / object classname,

}

extern "C" {
    pub fn ref_tracker_dir_debugfs(dir: *mut ref_tracker_dir);
}
extern "C" {
    pub fn ref_tracker_dir_symlink(dir: *mut ref_tracker_dir, fmt: *const c_char, ...);
}

//
// ref_tracker_dir_init - initialize a ref_tracker dir
// @dir: ref_tracker_dir to be initialized
// @quarantine_count: max number of entries to be tracked
// @class: pointer to static string that describes object type
//
// Initialize a ref_tracker_dir. If debugfs is configured, then a file
// will also be created for it under the top-level ref_tracker debugfs
// directory.
//
// Note that @class must point to a static string.
//
extern "C" {
    pub fn ref_tracker_dir_exit(dir: *mut ref_tracker_dir);
}
extern "C" {
    pub fn ref_tracker_dir_snprint(dir: *mut ref_tracker_dir, buf: *mut c_char, size: usize) -> c_int;
}

