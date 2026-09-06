//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fanotify.h
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
// Flags allowed to be passed from/to userspace.
//
// We intentionally do not add new bits to the old FAN_ALL_* constants, because
// they are uapi exposed constants. If there are programs out there using
// these constant, the programs may break if re-compiled with new uapi headers
// and then run on an old kernel.
//
// Group classes where permission events are allowed

//
// fanotify_init() flags that require CAP_SYS_ADMIN.
// We do not allow unprivileged groups to request permission events.
// We do not allow unprivileged groups to get other process pid in events.
// We do not allow unprivileged groups to use unlimited resources.
//

//
// fanotify_init() flags that are allowed for user without CAP_SYS_ADMIN.
// FAN_CLASS_NOTIF is the only class we allow for unprivileged group.
// We do not allow unprivileged groups to get file descriptors in events,
// so one of the flags for reporting file handles is required.
//

// Internal group flags
pub const FANOTIFY_UNPRIV: c_uint = 0x80000000;

//
// Events that can be reported with data type FSNOTIFY_EVENT_PATH.
// Note that FAN_MODIFY can also be reported with data type
// FSNOTIFY_EVENT_INODE.
//

//
// Directory entry modification events - reported only to directory
// where entry is modified and not to a watching parent.
//

// Content events can be used to inspect file content

// Pre-content events can be used to fill file content

// Events that require a permission response from user

// Events that can be reported with event->fd

// Events that can only be reported with data type FSNOTIFY_EVENT_INODE

// Events that can only be reported with data type FSNOTIFY_EVENT_ERROR

// Events that user can request to be notified on

// Extra flags that may be reported with event or control handling of events

// Events that may be reported to user

// Events and flags relevant only for directories

// These masks check for invalid bits in permission responses.

// Do not use these old uapi constants internally

