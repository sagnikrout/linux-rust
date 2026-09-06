//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/watch_queue.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum watch_notification_type {
    WATCH_TYPE_META		= 0,	/* Special record */
    WATCH_TYPE_KEY_NOTIFY	= 1,	/* Key change event notification */
    WATCH_TYPE__NR		= 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum watch_meta_notification_subtype {
    WATCH_META_REMOVAL_NOTIFICATION	= 0,	/* Watched object was removed */
    WATCH_META_LOSS_NOTIFICATION	= 1,	/* Data loss occurred */
}

//
// Notification record header.  This is aligned to 64-bits so that subclasses
// can contain __u64 fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct watch_notification {
    pub /: *mut *mut __u32 type:24; / enum watch_notification_type,
    pub /: *mut *mut __u32 subtype:8; / Type-specific subtype (filterable),
    pub info: __u32,
pub const WATCH_INFO_LENGTH: c_uint = 0x0000007f	/* Length of record */;
pub const WATCH_INFO_LENGTH__SHIFT: c_int = 0;
pub const WATCH_INFO_ID: c_uint = 0x0000ff00	/* ID of watchpoint */;
pub const WATCH_INFO_ID__SHIFT: c_int = 8;
pub const WATCH_INFO_TYPE_INFO: c_uint = 0xffff0000	/* Type-specific info */;
pub const WATCH_INFO_TYPE_INFO__SHIFT: c_int = 16;
pub const WATCH_INFO_FLAG_0: c_uint = 0x00010000	/* Type-specific info, flag bit 0 */;
pub const WATCH_INFO_FLAG_1: c_uint = 0x00020000	/* ... */;
pub const WATCH_INFO_FLAG_2: c_uint = 0x00040000;
pub const WATCH_INFO_FLAG_3: c_uint = 0x00080000;
pub const WATCH_INFO_FLAG_4: c_uint = 0x00100000;
pub const WATCH_INFO_FLAG_5: c_uint = 0x00200000;
pub const WATCH_INFO_FLAG_6: c_uint = 0x00400000;
pub const WATCH_INFO_FLAG_7: c_uint = 0x00800000;
}

//
// Notification filtering rules (IOC_WATCH_QUEUE_SET_FILTER).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct watch_notification_type_filter {
    pub /: *mut *mut __u32 type; / Type to apply filter to,
    pub /: *mut *mut __u32 info_filter; / Filter on watch_notification::info,
    pub /: *mut *mut __u32 info_mask; / Mask of relevant bits in info_filter,
    pub /: *mut *mut __u32 subtype_filter[8]; / Bitmask of subtypes to filter on,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct watch_notification_filter {
    pub /: *mut *mut __u32 nr_filters; / Number of filters,
    pub /: *mut *mut __u32 __reserved; / Must be 0,
    pub filters: [watch_notification_type_filter; ],
}

//
// Extended watch removal notification.  This is used optionally if the type
// wants to indicate an identifier for the object being watched, if there is
// such.  This can be distinguished by the length.
//
// type -> WATCH_TYPE_META
// subtype -> WATCH_META_REMOVAL_NOTIFICATION
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct watch_notification_removal {
    pub watch: watch_notification,
    pub /: *mut *mut __u64 id; / Type-dependent identifier,
}

//
// Type of key/keyring change notification.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum key_notification_subtype {
    NOTIFY_KEY_INSTANTIATED	= 0, /* Key was instantiated (aux is error code) */
    NOTIFY_KEY_UPDATED	= 1, /* Key was updated */
    NOTIFY_KEY_LINKED	= 2, /* Key (aux) was added to watched keyring */
    NOTIFY_KEY_UNLINKED	= 3, /* Key (aux) was removed from watched keyring */
    NOTIFY_KEY_CLEARED	= 4, /* Keyring was cleared */
    NOTIFY_KEY_REVOKED	= 5, /* Key was revoked */
    NOTIFY_KEY_INVALIDATED	= 6, /* Key was invalidated */
    NOTIFY_KEY_SETATTR	= 7, /* Key's attributes got changed */
}

//
// Key/keyring notification record.
// - watch.type = WATCH_TYPE_KEY_NOTIFY
// - watch.subtype = enum key_notification_type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_notification {
    pub watch: watch_notification,
    pub /: *mut *mut __u32 key_id; / The key/keyring affected,
    pub /: *mut *mut __u32 aux; / Per-type auxiliary data,
}
