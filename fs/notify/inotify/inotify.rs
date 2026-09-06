//! Automatically rewritten from C Header to Rust Module
//! Source: fs/notify/inotify/inotify.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inotify_event_info {
    pub fse: fsnotify_event,
    pub mask: u32,
    pub wd: c_int,
    pub sync_cookie: u32,
    pub name_len: c_int,
    pub name: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inotify_inode_mark {
    pub fsn_mark: fsnotify_mark,
    pub wd: c_int,
}

extern "C" {
    pub fn container_of(_arg: fse, inotify_event_info: struct, _arg: fse) -> return;
}
//
// INOTIFY_USER_FLAGS represents all of the mask bits that we expose to
// userspace.  There is at least one bit (FS_EVENT_ON_CHILD) which is
// used only internally to the kernel.
//

extern "C" {
    pub fn inc_ucount(_arg: ucounts->ns, _arg: ucounts->uid, _arg: UCOUNT_INOTIFY_WATCHES) -> return;
}
