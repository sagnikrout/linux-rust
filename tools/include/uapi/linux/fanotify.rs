//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/fanotify.h
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

// the following events that user-space can register for
pub const FAN_ACCESS: c_uint = 0x00000001	/* File was accessed */;
pub const FAN_MODIFY: c_uint = 0x00000002	/* File was modified */;
pub const FAN_ATTRIB: c_uint = 0x00000004	/* Metadata changed */;
pub const FAN_CLOSE_WRITE: c_uint = 0x00000008	/* Writable file closed */;
pub const FAN_CLOSE_NOWRITE: c_uint = 0x00000010	/* Unwritable file closed */;
pub const FAN_OPEN: c_uint = 0x00000020	/* File was opened */;
pub const FAN_MOVED_FROM: c_uint = 0x00000040	/* File was moved from X */;
pub const FAN_MOVED_TO: c_uint = 0x00000080	/* File was moved to Y */;
pub const FAN_CREATE: c_uint = 0x00000100	/* Subfile was created */;
pub const FAN_DELETE: c_uint = 0x00000200	/* Subfile was deleted */;
pub const FAN_DELETE_SELF: c_uint = 0x00000400	/* Self was deleted */;
pub const FAN_MOVE_SELF: c_uint = 0x00000800	/* Self was moved */;
pub const FAN_OPEN_EXEC: c_uint = 0x00001000	/* File was opened for exec */;
pub const FAN_Q_OVERFLOW: c_uint = 0x00004000	/* Event queued overflowed */;
pub const FAN_FS_ERROR: c_uint = 0x00008000	/* Filesystem error */;
pub const FAN_OPEN_PERM: c_uint = 0x00010000	/* File open in perm check */;
pub const FAN_ACCESS_PERM: c_uint = 0x00020000	/* File accessed in perm check */;
pub const FAN_OPEN_EXEC_PERM: c_uint = 0x00040000	/* File open/exec in perm check */;
// #define FAN_DIR_MODIFY	0x00080000 */	/* Deprecated (reserved)
pub const FAN_PRE_ACCESS: c_uint = 0x00100000	/* Pre-content access hook */;
pub const FAN_MNT_ATTACH: c_uint = 0x01000000	/* Mount was attached */;
pub const FAN_MNT_DETACH: c_uint = 0x02000000	/* Mount was detached */;
pub const FAN_EVENT_ON_CHILD: c_uint = 0x08000000	/* Interested in child events */;
pub const FAN_RENAME: c_uint = 0x10000000	/* File was renamed */;
pub const FAN_ONDIR: c_uint = 0x40000000	/* Event occurred against dir */;
// helper events

// flags used for fanotify_init()
pub const FAN_CLOEXEC: c_uint = 0x00000001;
pub const FAN_NONBLOCK: c_uint = 0x00000002;
// These are NOT bitwise flags.  Both bits are used together.
pub const FAN_CLASS_NOTIF: c_uint = 0x00000000;
pub const FAN_CLASS_CONTENT: c_uint = 0x00000004;
pub const FAN_CLASS_PRE_CONTENT: c_uint = 0x00000008;
// Deprecated - do not use this in programs and do not add new flags here!

pub const FAN_UNLIMITED_QUEUE: c_uint = 0x00000010;
pub const FAN_UNLIMITED_MARKS: c_uint = 0x00000020;
pub const FAN_ENABLE_AUDIT: c_uint = 0x00000040;
// Flags to determine fanotify event format
pub const FAN_REPORT_PIDFD: c_uint = 0x00000080	/* Report pidfd for event->pid */;
pub const FAN_REPORT_TID: c_uint = 0x00000100	/* event->pid is thread id */;
pub const FAN_REPORT_FID: c_uint = 0x00000200	/* Report unique file id */;
pub const FAN_REPORT_DIR_FID: c_uint = 0x00000400	/* Report unique directory id */;
pub const FAN_REPORT_NAME: c_uint = 0x00000800	/* Report events with name */;
pub const FAN_REPORT_TARGET_FID: c_uint = 0x00001000	/* Report dirent target id  */;
pub const FAN_REPORT_FD_ERROR: c_uint = 0x00002000	/* event->fd can report error */;
pub const FAN_REPORT_MNT: c_uint = 0x00004000	/* Report mount events */;
// Convenience macro - FAN_REPORT_NAME requires FAN_REPORT_DIR_FID

// Convenience macro - FAN_REPORT_TARGET_FID requires all other FID flags

// Deprecated - do not use this in programs and do not add new flags here!

// flags used for fanotify_modify_mark()
pub const FAN_MARK_ADD: c_uint = 0x00000001;
pub const FAN_MARK_REMOVE: c_uint = 0x00000002;
pub const FAN_MARK_DONT_FOLLOW: c_uint = 0x00000004;
pub const FAN_MARK_ONLYDIR: c_uint = 0x00000008;
// FAN_MARK_MOUNT is		0x00000010
pub const FAN_MARK_IGNORED_MASK: c_uint = 0x00000020;
pub const FAN_MARK_IGNORED_SURV_MODIFY: c_uint = 0x00000040;
pub const FAN_MARK_FLUSH: c_uint = 0x00000080;
// FAN_MARK_FILESYSTEM is	0x00000100
pub const FAN_MARK_EVICTABLE: c_uint = 0x00000200;
// This bit is mutually exclusive with FAN_MARK_IGNORED_MASK bit
pub const FAN_MARK_IGNORE: c_uint = 0x00000400;
// These are NOT bitwise flags.  Both bits can be used togther.
pub const FAN_MARK_INODE: c_uint = 0x00000000;
pub const FAN_MARK_MOUNT: c_uint = 0x00000010;
pub const FAN_MARK_FILESYSTEM: c_uint = 0x00000100;
pub const FAN_MARK_MNTNS: c_uint = 0x00000110;
//
// Convenience macro - FAN_MARK_IGNORE requires FAN_MARK_IGNORED_SURV_MODIFY
// for non-inode mark types.
//

// Deprecated - do not use this in programs and do not add new flags here!

// Deprecated - do not use this in programs and do not add new flags here!

//
// All events which require a permission response from userspace
//
// Deprecated - do not use this in programs and do not add new flags here!

// Deprecated - do not use this in programs and do not add new flags here!

pub const FANOTIFY_METADATA_VERSION: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_event_metadata {
    pub event_len: __u32,
    pub vers: __u8,
    pub reserved: __u8,
    pub metadata_len: __u16,
    pub mask: __aligned_u64,
    pub fd: __s32,
    pub pid: __s32,
}

pub const FAN_EVENT_INFO_TYPE_FID: c_int = 1;
pub const FAN_EVENT_INFO_TYPE_DFID_NAME: c_int = 2;
pub const FAN_EVENT_INFO_TYPE_DFID: c_int = 3;
pub const FAN_EVENT_INFO_TYPE_PIDFD: c_int = 4;
pub const FAN_EVENT_INFO_TYPE_ERROR: c_int = 5;
pub const FAN_EVENT_INFO_TYPE_RANGE: c_int = 6;
pub const FAN_EVENT_INFO_TYPE_MNT: c_int = 7;
// Special info types for FAN_RENAME
pub const FAN_EVENT_INFO_TYPE_OLD_DFID_NAME: c_int = 10;
// Reserved for FAN_EVENT_INFO_TYPE_OLD_DFID	11
pub const FAN_EVENT_INFO_TYPE_NEW_DFID_NAME: c_int = 12;
// Reserved for FAN_EVENT_INFO_TYPE_NEW_DFID	13
// Variable length info record following event metadata
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_event_info_header {
    pub info_type: __u8,
    pub pad: __u8,
    pub len: __u16,
}

//
// Unique file identifier info record.
// This structure is used for records of types FAN_EVENT_INFO_TYPE_FID,
// FAN_EVENT_INFO_TYPE_DFID and FAN_EVENT_INFO_TYPE_DFID_NAME.
// For FAN_EVENT_INFO_TYPE_DFID_NAME there is additionally a null terminated
// name immediately after the file handle.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_event_info_fid {
    pub hdr: fanotify_event_info_header,
    pub fsid: __kernel_fsid_t,
//
// Following is an opaque struct file_handle that can be passed as
// an argument to open_by_handle_at(2).
//
    pub handle: [c_uchar; ],
}

//
// This structure is used for info records of type FAN_EVENT_INFO_TYPE_PIDFD.
// It holds a pidfd for the pid that was responsible for generating an event.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_event_info_pidfd {
    pub hdr: fanotify_event_info_header,
    pub pidfd: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_event_info_error {
    pub hdr: fanotify_event_info_header,
    pub error: __s32,
    pub error_count: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_event_info_range {
    pub hdr: fanotify_event_info_header,
    pub pad: __u32,
    pub offset: __u64,
    pub count: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_event_info_mnt {
    pub hdr: fanotify_event_info_header,
    pub mnt_id: __u64,
}

//
// User space may need to record additional information about its decision.
// The extra information type records what kind of information is included.
// The default is none. We also define an extra information buffer whose
// size is determined by the extra information type.
//
// If the information type is Audit Rule, then the information following
// is the rule number that triggered the user space decision that
// requires auditing.
//
pub const FAN_RESPONSE_INFO_NONE: c_int = 0;
pub const FAN_RESPONSE_INFO_AUDIT_RULE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_response {
    pub fd: __s32,
    pub response: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_response_info_header {
    pub type: __u8,
    pub pad: __u8,
    pub len: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanotify_response_info_audit_rule {
    pub hdr: fanotify_response_info_header,
    pub rule_number: __u32,
    pub subj_trust: __u32,
    pub obj_trust: __u32,
}

// Legit userspace responses to a _PERM event
pub const FAN_ALLOW: c_uint = 0x01;
pub const FAN_DENY: c_uint = 0x02;
// errno other than EPERM can specified in upper byte of deny response
pub const FAN_ERRNO_BITS: c_int = 8;

pub const FAN_AUDIT: c_uint = 0x10	/* Bitmask to create audit record for result */;
pub const FAN_INFO: c_uint = 0x20	/* Bitmask to indicate additional information */;
// No fd set in event

// Helper functions to deal with fanotify_event_metadata buffers

