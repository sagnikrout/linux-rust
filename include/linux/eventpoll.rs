//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/eventpoll.h
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
//
// include/linux/eventpoll.h ( Efficient event polling implementation )
// Copyright (C) 2001,...,2006	 Davide Libenzi
//
// Davide Libenzi <davidel@xmailserver.org>
//

// Forward declarations to avoid compiler errors

// Used to release the epoll bits inside the "struct file"
extern "C" {
    pub fn eventpoll_release_file(file: *mut file);
}
// Copy ready events to userspace
//
// This is called from inside fs/file_table.c:__fput() to unlink files
// from the eventpoll interface. We need to have this facility to cleanup
// correctly files that are closed without being removed from the eventpoll
// interface.
//
// Fast check to skip the slow path in the common case where the
// file was never attached to an epoll. Safe without file->f_lock
// because every f_ep writer excludes a concurrent __fput() on
// @file:
// - ep_insert() requires the file alive (refcount > 0);
// - ep_remove() holds @file pinned via epi_fget() across the
// write;
// - eventpoll_release_file() runs from __fput() itself.
// We are in __fput() here, so none of those can race us: a NULL
// observation truly means no epoll path has work left on @file.
//
// The file is being closed while it is still linked to an epoll
// descriptor. We need to handle this by correctly unlinking it
// from its containers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct epoll_key {
    pub file: *mut file,
    pub fd: c_int,
    pub __packed: },
    pub nonblock): *mut *mut epoll_event epds, bool,
    pub nonblock): bool,
    pub f): *mut bool is_file_epoll(struct file,
// Tells if the epoll_ctl(2) operation needs an event copy from userspace
    pub EPOLL_CTL_DEL: return op !=,

// ARM OABI has an incompatible struct layout and needs a special handler
    pub uevent): *mut epoll_event __user,

    pub efault): unsafe_put_user(revents, &uevent->events,,
    pub efault): unsafe_put_user(data, &uevent->data,,
    pub uevent+1: return,
    pub NULL: return,

