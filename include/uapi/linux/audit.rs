//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/audit.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
// audit.h -- Auditing support
//
// Copyright 2003-2004 Red Hat Inc., Durham, North Carolina.
// All Rights Reserved.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
//
// Written by Rickard E. (Rik) Faith <faith@redhat.com>
//

// The netlink messages for the audit system is divided into blocks:
// 1000 - 1099 are for commanding the audit system
// 1100 - 1199 user space trusted application messages
// 1200 - 1299 messages internal to the audit daemon
// 1300 - 1399 audit event messages
// 1400 - 1499 access control messages
// 1500 - 1599 kernel LSPP events
// 1600 - 1699 kernel crypto events
// 1700 - 1799 kernel anomaly records
// 1800 - 1899 kernel integrity events
// 1900 - 1999 future kernel use
// 2000 is for otherwise unclassified kernel audit messages (legacy)
// 2001 - 2099 unused (kernel)
// 2100 - 2199 user space anomaly records
// 2200 - 2299 user space actions taken in response to anomalies
// 2300 - 2399 user space generated LSPP events
// 2400 - 2499 user space crypto events
// 2500 - 2999 future user space (maybe integrity labels and related events)
//
// Messages from 1000-1199 are bi-directional. 1200-1299 & 2100 - 2999 are
// exclusively user space. 1300-2099 is kernel --> user space
// communication.
//

pub const AUDIT_LAST_USER_MSG: c_int = 1199;

pub const AUDIT_LAST_USER_MSG2: c_int = 2999;

// #define AUDIT_FS_WATCH	1301	 * Deprecated

pub const AUDIT_FIRST_KERN_ANOM_MSG: c_int = 1700;
pub const AUDIT_LAST_KERN_ANOM_MSG: c_int = 1799;

// Rule flags
pub const AUDIT_FILTER_USER: c_uint = 0x00	/* Apply rule to user-generated messages */;
pub const AUDIT_FILTER_TASK: c_uint = 0x01	/* Apply rule at task creation (not syscall) */;
pub const AUDIT_FILTER_ENTRY: c_uint = 0x02	/* Apply rule at syscall entry */;
pub const AUDIT_FILTER_WATCH: c_uint = 0x03	/* Apply rule to file system watches */;
pub const AUDIT_FILTER_EXIT: c_uint = 0x04	/* Apply rule at syscall exit */;
pub const AUDIT_FILTER_EXCLUDE: c_uint = 0x05	/* Apply rule before record creation */;

pub const AUDIT_FILTER_FS: c_uint = 0x06	/* Apply rule at __audit_inode_child */;
pub const AUDIT_FILTER_URING_EXIT: c_uint = 0x07	/* Apply rule at io_uring op exit */;
pub const AUDIT_NR_FILTERS: c_int = 8;
pub const AUDIT_FILTER_PREPEND: c_uint = 0x10	/* Prepend to front of list */;
// Rule actions

// Rule structure sizes -- if these change, different AUDIT_ADD and
// AUDIT_LIST commands must be implemented.
pub const AUDIT_MAX_FIELDS: c_int = 64;
pub const AUDIT_MAX_KEY_LEN: c_int = 256;
pub const AUDIT_BITMASK_SIZE: c_int = 64;

pub const AUDIT_SYSCALL_CLASSES: c_int = 16;
pub const AUDIT_CLASS_DIR_WRITE: c_int = 0;
pub const AUDIT_CLASS_DIR_WRITE_32: c_int = 1;
pub const AUDIT_CLASS_CHATTR: c_int = 2;
pub const AUDIT_CLASS_CHATTR_32: c_int = 3;
pub const AUDIT_CLASS_READ: c_int = 4;
pub const AUDIT_CLASS_READ_32: c_int = 5;
pub const AUDIT_CLASS_WRITE: c_int = 6;
pub const AUDIT_CLASS_WRITE_32: c_int = 7;
pub const AUDIT_CLASS_SIGNAL: c_int = 8;
pub const AUDIT_CLASS_SIGNAL_32: c_int = 9;
// This bitmask is used to validate user input.  It represents all bits that
// are currently used in an audit field constant understood by the kernel.
// If you are adding a new #define AUDIT_<whatever>, please ensure that
// AUDIT_UNUSED_BITS is updated if need be.
pub const AUDIT_UNUSED_BITS: c_uint = 0x07FFFC00;
// AUDIT_FIELD_COMPARE rule list
pub const AUDIT_COMPARE_UID_TO_OBJ_UID: c_int = 1;
pub const AUDIT_COMPARE_GID_TO_OBJ_GID: c_int = 2;
pub const AUDIT_COMPARE_EUID_TO_OBJ_UID: c_int = 3;
pub const AUDIT_COMPARE_EGID_TO_OBJ_GID: c_int = 4;
pub const AUDIT_COMPARE_AUID_TO_OBJ_UID: c_int = 5;
pub const AUDIT_COMPARE_SUID_TO_OBJ_UID: c_int = 6;
pub const AUDIT_COMPARE_SGID_TO_OBJ_GID: c_int = 7;
pub const AUDIT_COMPARE_FSUID_TO_OBJ_UID: c_int = 8;
pub const AUDIT_COMPARE_FSGID_TO_OBJ_GID: c_int = 9;
pub const AUDIT_COMPARE_UID_TO_AUID: c_int = 10;
pub const AUDIT_COMPARE_UID_TO_EUID: c_int = 11;
pub const AUDIT_COMPARE_UID_TO_FSUID: c_int = 12;
pub const AUDIT_COMPARE_UID_TO_SUID: c_int = 13;
pub const AUDIT_COMPARE_AUID_TO_FSUID: c_int = 14;
pub const AUDIT_COMPARE_AUID_TO_SUID: c_int = 15;
pub const AUDIT_COMPARE_AUID_TO_EUID: c_int = 16;
pub const AUDIT_COMPARE_EUID_TO_SUID: c_int = 17;
pub const AUDIT_COMPARE_EUID_TO_FSUID: c_int = 18;
pub const AUDIT_COMPARE_SUID_TO_FSUID: c_int = 19;
pub const AUDIT_COMPARE_GID_TO_EGID: c_int = 20;
pub const AUDIT_COMPARE_GID_TO_FSGID: c_int = 21;
pub const AUDIT_COMPARE_GID_TO_SGID: c_int = 22;
pub const AUDIT_COMPARE_EGID_TO_FSGID: c_int = 23;
pub const AUDIT_COMPARE_EGID_TO_SGID: c_int = 24;
pub const AUDIT_COMPARE_SGID_TO_FSGID: c_int = 25;

// Rule fields
// These are useful when checking the
// task structure at task creation time
// (AUDIT_PER_TASK).
pub const AUDIT_PID: c_int = 0;
pub const AUDIT_UID: c_int = 1;
pub const AUDIT_EUID: c_int = 2;
pub const AUDIT_SUID: c_int = 3;
pub const AUDIT_FSUID: c_int = 4;
pub const AUDIT_GID: c_int = 5;
pub const AUDIT_EGID: c_int = 6;
pub const AUDIT_SGID: c_int = 7;
pub const AUDIT_FSGID: c_int = 8;
pub const AUDIT_LOGINUID: c_int = 9;
pub const AUDIT_PERS: c_int = 10;
pub const AUDIT_ARCH: c_int = 11;
pub const AUDIT_MSGTYPE: c_int = 12;

pub const AUDIT_PPID: c_int = 18;
pub const AUDIT_OBJ_USER: c_int = 19;
pub const AUDIT_OBJ_ROLE: c_int = 20;
pub const AUDIT_OBJ_TYPE: c_int = 21;
pub const AUDIT_OBJ_LEV_LOW: c_int = 22;
pub const AUDIT_OBJ_LEV_HIGH: c_int = 23;
pub const AUDIT_LOGINUID_SET: c_int = 24;

// These are ONLY useful when checking
// at syscall exit time (AUDIT_AT_EXIT).
pub const AUDIT_DEVMAJOR: c_int = 100;
pub const AUDIT_DEVMINOR: c_int = 101;
pub const AUDIT_INODE: c_int = 102;
pub const AUDIT_EXIT: c_int = 103;

pub const AUDIT_WATCH: c_int = 105;
pub const AUDIT_PERM: c_int = 106;
pub const AUDIT_DIR: c_int = 107;
pub const AUDIT_FILETYPE: c_int = 108;
pub const AUDIT_OBJ_UID: c_int = 109;
pub const AUDIT_OBJ_GID: c_int = 110;
pub const AUDIT_FIELD_COMPARE: c_int = 111;
pub const AUDIT_EXE: c_int = 112;
pub const AUDIT_SADDR_FAM: c_int = 113;
pub const AUDIT_ARG0: c_int = 200;

pub const AUDIT_FILTERKEY: c_int = 210;
pub const AUDIT_NEGATE: c_uint = 0x80000000;
// These are the supported operators.
// 4  2  1  8
// =  >  <  ?
// ----------
// 0  0  0	 0	00	nonsense
// 0  0  0	 1	08	&  bit mask
// 0  0  1	 0	10	<
// 0  1  0	 0	20	>
// 0  1  1	 0	30	!=
// 1  0  0	 0	40	=
// 1  0  0	 1	48	&=  bit test
// 1  0  1	 0	50	<=
// 1  1  0	 0	60	>=
// 1  1  1	 1	78	all operators
//
pub const AUDIT_BIT_MASK: c_uint = 0x08000000;
pub const AUDIT_LESS_THAN: c_uint = 0x10000000;
pub const AUDIT_GREATER_THAN: c_uint = 0x20000000;
pub const AUDIT_NOT_EQUAL: c_uint = 0x30000000;
pub const AUDIT_EQUAL: c_uint = 0x40000000;

// Status symbols
// Mask values
pub const AUDIT_STATUS_ENABLED: c_uint = 0x0001;
pub const AUDIT_STATUS_FAILURE: c_uint = 0x0002;
pub const AUDIT_STATUS_PID: c_uint = 0x0004;
pub const AUDIT_STATUS_RATE_LIMIT: c_uint = 0x0008;
pub const AUDIT_STATUS_BACKLOG_LIMIT: c_uint = 0x0010;
pub const AUDIT_STATUS_BACKLOG_WAIT_TIME: c_uint = 0x0020;
pub const AUDIT_STATUS_LOST: c_uint = 0x0040;
pub const AUDIT_STATUS_BACKLOG_WAIT_TIME_ACTUAL: c_uint = 0x0080;
pub const AUDIT_FEATURE_BITMAP_BACKLOG_LIMIT: c_uint = 0x00000001;
pub const AUDIT_FEATURE_BITMAP_BACKLOG_WAIT_TIME: c_uint = 0x00000002;
pub const AUDIT_FEATURE_BITMAP_EXECUTABLE_PATH: c_uint = 0x00000004;
pub const AUDIT_FEATURE_BITMAP_EXCLUDE_EXTEND: c_uint = 0x00000008;
pub const AUDIT_FEATURE_BITMAP_SESSIONID_FILTER: c_uint = 0x00000010;
pub const AUDIT_FEATURE_BITMAP_LOST_RESET: c_uint = 0x00000020;
pub const AUDIT_FEATURE_BITMAP_FILTER_FS: c_uint = 0x00000040;

// deprecated: AUDIT_VERSION_*

// Failure-to-log actions
pub const AUDIT_FAIL_SILENT: c_int = 0;
pub const AUDIT_FAIL_PRINTK: c_int = 1;
pub const AUDIT_FAIL_PANIC: c_int = 2;
//
// These bits disambiguate different calling conventions that share an
// ELF machine type, bitness, and endianness
//
pub const __AUDIT_ARCH_CONVENTION_MASK: c_uint = 0x30000000;
pub const __AUDIT_ARCH_CONVENTION_MIPS64_N32: c_uint = 0x20000000;
// distinguish syscall tables
pub const __AUDIT_ARCH_64BIT: c_uint = 0x80000000;
pub const __AUDIT_ARCH_LE: c_uint = 0x40000000;

// do not define AUDIT_ARCH_PPCLE since it is not supported by audit

pub const AUDIT_PERM_EXEC: c_int = 1;
pub const AUDIT_PERM_WRITE: c_int = 2;
pub const AUDIT_PERM_READ: c_int = 4;
pub const AUDIT_PERM_ATTR: c_int = 8;
// MAX_AUDIT_MESSAGE_LENGTH is set in audit:lib/libaudit.h as:
// 8970 // PATH_MAX*2+CONTEXT_SIZE*2+11+256+1
// max header+body+tailer: 44 + 29 + 32 + 262 + 7 + pad
//
pub const AUDIT_MESSAGE_TEXT_MAX: c_int = 8560;
// Multicast Netlink socket groups (default up to 32)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum audit_nlgrps {
    AUDIT_NLGRP_NONE,	/* Group 0 not used */
    AUDIT_NLGRP_READLOG,	/* "best effort" read only socket */
    __AUDIT_NLGRP_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_status {
    pub /: *mut *mut __u32 mask; / Bit mask for valid entries,
    pub /: *mut *mut __u32 enabled; / 1 = enabled, 0 = disabled,
    pub /: *mut *mut __u32 failure; / Failure-to-log action,
    pub /: *mut *mut __u32 pid; / pid of auditd process,
    pub /: *mut *mut __u32 rate_limit; / messages rate limit (per second),
    pub /: *mut *mut __u32 backlog_limit; / waiting messages limit,
    pub /: *mut *mut __u32 lost; / messages lost,
    pub /: *mut *mut __u32 backlog; / messages waiting in queue,
    pub /: *mut *mut __u32 version; / deprecated: audit api version num,
    pub /: *mut *mut __u32 feature_bitmap; / bitmap of kernel audit features,
}

// message limit exceeded
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_features {
pub const AUDIT_FEATURE_VERSION: c_int = 1;
    pub vers: __u32,
    pub /: *mut *mut __u32 mask; / which bits we are dealing with,
    pub /: *mut *mut __u32 features; / which feature to enable/disable,
    pub /: *mut *mut __u32 lock; / which features to lock,
}

pub const AUDIT_FEATURE_ONLY_UNSET_LOGINUID: c_int = 0;
pub const AUDIT_FEATURE_LOGINUID_IMMUTABLE: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_tty_status {
    pub /: *mut *mut __u32 enabled; / 1 = enabled, 0 = disabled,
    pub /: *mut *mut __u32 log_passwd; / 1 = enabled, 0 = disabled,
}

// audit_rule_data supports filter rules with both integer and string
// fields.  It corresponds with AUDIT_ADD_RULE, AUDIT_DEL_RULE and
// AUDIT_LIST_RULES requests.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_rule_data {
    pub /: *mut *mut __u32 flags; / AUDIT_PER_{TASK,CALL}, AUDIT_PREPEND,
    pub /: *mut *mut __u32 action; / AUDIT_NEVER, AUDIT_POSSIBLE, AUDIT_ALWAYS,
    pub field_count: __u32,
    pub /: *mut *mut __u32 mask[AUDIT_BITMASK_SIZE]; / syscall(s) affected,
    pub fields: [__u32; AUDIT_MAX_FIELDS],
    pub values: [__u32; AUDIT_MAX_FIELDS],
    pub fieldflags: [__u32; AUDIT_MAX_FIELDS],
    pub /: *mut *mut __u32 buflen; / total length of string fields,
    pub /: *mut *mut char buf[]; / string fields buffer,
}
