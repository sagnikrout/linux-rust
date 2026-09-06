//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/inotify.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Inode based directory notification for Linux
//
// Copyright (C) 2005 John McCutchan
//
// For O_CLOEXEC and O_NONBLOCK

//
// struct inotify_event - structure read from the inotify device for each event
//
// When you are watching a directory, you will receive the filename for events
// such as IN_CREATE, IN_DELETE, IN_OPEN, IN_CLOSE, ..., relative to the wd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inotify_event {
    pub /: *mut *mut __s32 wd; / watch descriptor,
    pub /: *mut *mut __u32 mask; / watch mask,
    pub /: *mut *mut __u32 cookie; / cookie to synchronize two events,
    pub /: *mut *mut __u32 len; / length (including nulls) of name,
    pub /: *mut *mut char name[]; / stub for possible name,
}

// the following are legal, implemented events that user-space can watch for
pub const IN_ACCESS: c_uint = 0x00000001	/* File was accessed */;
pub const IN_MODIFY: c_uint = 0x00000002	/* File was modified */;
pub const IN_ATTRIB: c_uint = 0x00000004	/* Metadata changed */;
pub const IN_CLOSE_WRITE: c_uint = 0x00000008	/* Writable file was closed */;
pub const IN_CLOSE_NOWRITE: c_uint = 0x00000010	/* Unwritable file closed */;
pub const IN_OPEN: c_uint = 0x00000020	/* File was opened */;
pub const IN_MOVED_FROM: c_uint = 0x00000040	/* File was moved from X */;
pub const IN_MOVED_TO: c_uint = 0x00000080	/* File was moved to Y */;
pub const IN_CREATE: c_uint = 0x00000100	/* Subfile was created */;
pub const IN_DELETE: c_uint = 0x00000200	/* Subfile was deleted */;
pub const IN_DELETE_SELF: c_uint = 0x00000400	/* Self was deleted */;
pub const IN_MOVE_SELF: c_uint = 0x00000800	/* Self was moved */;
// the following are legal events.  they are sent as needed to any watch
pub const IN_UNMOUNT: c_uint = 0x00002000	/* Backing fs was unmounted */;
pub const IN_Q_OVERFLOW: c_uint = 0x00004000	/* Event queued overflowed */;
pub const IN_IGNORED: c_uint = 0x00008000	/* File was ignored */;
// helper events

// special flags
pub const IN_ONLYDIR: c_uint = 0x01000000	/* only watch the path if it is a directory */;
pub const IN_DONT_FOLLOW: c_uint = 0x02000000	/* don't follow a sym link */;
pub const IN_EXCL_UNLINK: c_uint = 0x04000000	/* exclude events on unlinked objects */;
pub const IN_MASK_CREATE: c_uint = 0x10000000	/* only create watches */;
pub const IN_MASK_ADD: c_uint = 0x20000000	/* add to the mask of an already existing watch */;
pub const IN_ISDIR: c_uint = 0x40000000	/* event occurred against dir */;
pub const IN_ONESHOT: c_uint = 0x80000000	/* only send event once */;
//
// All of the events - we build the list by hand so that we can add flags in
// the future and not break backward compatibility.  Apps will get only the
// events that they originally wanted.  Be sure to add new events here!
//

// Flags for sys_inotify_init1.

//
// ioctl numbers: inotify uses 'I' prefix for all ioctls,
// except historical FIONREAD, which is based on 'T'.
//
// INOTIFY_IOC_SETNEXTWD: set desired number of next created
// watch descriptor.
//

