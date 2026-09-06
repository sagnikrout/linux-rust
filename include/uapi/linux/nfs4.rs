//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/nfs4.h
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
// include/linux/nfs4.h
//
// NFSv4 protocol definitions.
//
// Copyright (c) 2002 The Regents of the University of Michigan.
// All rights reserved.
//
// Kendrick Smith <kmsmith@umich.edu>
// Andy Adamson   <andros@umich.edu>
//

pub const NFS4_BITMAP_SIZE: c_int = 3;
pub const NFS4_STATEID_SEQID_SIZE: c_int = 4;
pub const NFS4_STATEID_OTHER_SIZE: c_int = 12;

pub const NFS4_OPAQUE_LIMIT: c_int = 1024;
pub const NFS4_MAX_SESSIONID_LEN: c_int = 16;
pub const NFS4_ACCESS_READ: c_uint = 0x0001;
pub const NFS4_ACCESS_LOOKUP: c_uint = 0x0002;
pub const NFS4_ACCESS_MODIFY: c_uint = 0x0004;
pub const NFS4_ACCESS_EXTEND: c_uint = 0x0008;
pub const NFS4_ACCESS_DELETE: c_uint = 0x0010;
pub const NFS4_ACCESS_EXECUTE: c_uint = 0x0020;
pub const NFS4_ACCESS_XAREAD: c_uint = 0x0040;
pub const NFS4_ACCESS_XAWRITE: c_uint = 0x0080;
pub const NFS4_ACCESS_XALIST: c_uint = 0x0100;
pub const NFS4_FH_PERSISTENT: c_uint = 0x0000;
pub const NFS4_FH_NOEXPIRE_WITH_OPEN: c_uint = 0x0001;
pub const NFS4_FH_VOLATILE_ANY: c_uint = 0x0002;
pub const NFS4_FH_VOL_MIGRATION: c_uint = 0x0004;
pub const NFS4_FH_VOL_RENAME: c_uint = 0x0008;
pub const NFS4_OPEN_RESULT_CONFIRM: c_uint = 0x0002;
pub const NFS4_OPEN_RESULT_LOCKTYPE_POSIX: c_uint = 0x0004;
pub const NFS4_OPEN_RESULT_PRESERVE_UNLINKED: c_uint = 0x0008;
pub const NFS4_OPEN_RESULT_NO_OPEN_STATEID: c_uint = 0x0010;
pub const NFS4_OPEN_RESULT_MAY_NOTIFY_LOCK: c_uint = 0x0020;
pub const NFS4_SHARE_ACCESS_MASK: c_uint = 0x000F;
pub const NFS4_SHARE_ACCESS_READ: c_uint = 0x0001;
pub const NFS4_SHARE_ACCESS_WRITE: c_uint = 0x0002;
pub const NFS4_SHARE_ACCESS_BOTH: c_uint = 0x0003;
pub const NFS4_SHARE_DENY_READ: c_uint = 0x0001;
pub const NFS4_SHARE_DENY_WRITE: c_uint = 0x0002;
pub const NFS4_SHARE_DENY_BOTH: c_uint = 0x0003;
// nfs41
pub const NFS4_SHARE_WANT_TYPE_MASK: c_uint = 0xFF00;
pub const NFS4_SHARE_WANT_NO_PREFERENCE: c_uint = 0x0000;
pub const NFS4_SHARE_WANT_READ_DELEG: c_uint = 0x0100;
pub const NFS4_SHARE_WANT_WRITE_DELEG: c_uint = 0x0200;
pub const NFS4_SHARE_WANT_ANY_DELEG: c_uint = 0x0300;
pub const NFS4_SHARE_WANT_NO_DELEG: c_uint = 0x0400;
pub const NFS4_SHARE_WANT_CANCEL: c_uint = 0x0500;
pub const NFS4_SHARE_WHEN_MASK: c_uint = 0xF0000;
pub const NFS4_SHARE_SIGNAL_DELEG_WHEN_RESRC_AVAIL: c_uint = 0x10000;
pub const NFS4_SHARE_PUSH_DELEG_WHEN_UNCONTENDED: c_uint = 0x20000;
pub const NFS4_SHARE_WANT_MOD_MASK: c_uint = 0xF00000;
pub const NFS4_SHARE_WANT_DELEG_TIMESTAMPS: c_uint = 0x100000;
pub const NFS4_SHARE_WANT_OPEN_XOR_DELEGATION: c_uint = 0x200000;

pub const NFS4_CDFC4_FORE: c_uint = 0x1;
pub const NFS4_CDFC4_BACK: c_uint = 0x2;
pub const NFS4_CDFC4_BOTH: c_uint = 0x3;
pub const NFS4_CDFC4_FORE_OR_BOTH: c_uint = 0x3;
pub const NFS4_CDFC4_BACK_OR_BOTH: c_uint = 0x7;
pub const NFS4_CDFS4_FORE: c_uint = 0x1;
pub const NFS4_CDFS4_BACK: c_uint = 0x2;
pub const NFS4_CDFS4_BOTH: c_uint = 0x3;
pub const NFS4_SET_TO_SERVER_TIME: c_int = 0;
pub const NFS4_SET_TO_CLIENT_TIME: c_int = 1;
pub const NFS4_ACE_ACCESS_ALLOWED_ACE_TYPE: c_int = 0;
pub const NFS4_ACE_ACCESS_DENIED_ACE_TYPE: c_int = 1;
pub const NFS4_ACE_SYSTEM_AUDIT_ACE_TYPE: c_int = 2;
pub const NFS4_ACE_SYSTEM_ALARM_ACE_TYPE: c_int = 3;
pub const ACL4_SUPPORT_ALLOW_ACL: c_uint = 0x01;
pub const ACL4_SUPPORT_DENY_ACL: c_uint = 0x02;
pub const ACL4_SUPPORT_AUDIT_ACL: c_uint = 0x04;
pub const ACL4_SUPPORT_ALARM_ACL: c_uint = 0x08;
pub const NFS4_ACL_AUTO_INHERIT: c_uint = 0x00000001;
pub const NFS4_ACL_PROTECTED: c_uint = 0x00000002;
pub const NFS4_ACL_DEFAULTED: c_uint = 0x00000004;
pub const NFS4_ACE_FILE_INHERIT_ACE: c_uint = 0x00000001;
pub const NFS4_ACE_DIRECTORY_INHERIT_ACE: c_uint = 0x00000002;
pub const NFS4_ACE_NO_PROPAGATE_INHERIT_ACE: c_uint = 0x00000004;
pub const NFS4_ACE_INHERIT_ONLY_ACE: c_uint = 0x00000008;
pub const NFS4_ACE_SUCCESSFUL_ACCESS_ACE_FLAG: c_uint = 0x00000010;
pub const NFS4_ACE_FAILED_ACCESS_ACE_FLAG: c_uint = 0x00000020;
pub const NFS4_ACE_IDENTIFIER_GROUP: c_uint = 0x00000040;
pub const NFS4_ACE_INHERITED_ACE: c_uint = 0x00000080;
pub const NFS4_ACE_READ_DATA: c_uint = 0x00000001;
pub const NFS4_ACE_LIST_DIRECTORY: c_uint = 0x00000001;
pub const NFS4_ACE_WRITE_DATA: c_uint = 0x00000002;
pub const NFS4_ACE_ADD_FILE: c_uint = 0x00000002;
pub const NFS4_ACE_APPEND_DATA: c_uint = 0x00000004;
pub const NFS4_ACE_ADD_SUBDIRECTORY: c_uint = 0x00000004;
pub const NFS4_ACE_READ_NAMED_ATTRS: c_uint = 0x00000008;
pub const NFS4_ACE_WRITE_NAMED_ATTRS: c_uint = 0x00000010;
pub const NFS4_ACE_EXECUTE: c_uint = 0x00000020;
pub const NFS4_ACE_DELETE_CHILD: c_uint = 0x00000040;
pub const NFS4_ACE_READ_ATTRIBUTES: c_uint = 0x00000080;
pub const NFS4_ACE_WRITE_ATTRIBUTES: c_uint = 0x00000100;
pub const NFS4_ACE_WRITE_RETENTION: c_uint = 0x00000200;
pub const NFS4_ACE_WRITE_RETENTION_HOLD: c_uint = 0x00000400;
pub const NFS4_ACE_DELETE: c_uint = 0x00010000;
pub const NFS4_ACE_READ_ACL: c_uint = 0x00020000;
pub const NFS4_ACE_WRITE_ACL: c_uint = 0x00040000;
pub const NFS4_ACE_WRITE_OWNER: c_uint = 0x00080000;
pub const NFS4_ACE_SYNCHRONIZE: c_uint = 0x00100000;
pub const NFS4_ACE_GENERIC_READ: c_uint = 0x00120081;
pub const NFS4_ACE_GENERIC_WRITE: c_uint = 0x00160106;
pub const NFS4_ACE_GENERIC_EXECUTE: c_uint = 0x001200A0;
pub const NFS4_ACE_MASK_ALL: c_uint = 0x001F01FF;
pub const EXCHGID4_FLAG_SUPP_MOVED_REFER: c_uint = 0x00000001;
pub const EXCHGID4_FLAG_SUPP_MOVED_MIGR: c_uint = 0x00000002;
pub const EXCHGID4_FLAG_BIND_PRINC_STATEID: c_uint = 0x00000100;
pub const EXCHGID4_FLAG_USE_NON_PNFS: c_uint = 0x00010000;
pub const EXCHGID4_FLAG_USE_PNFS_MDS: c_uint = 0x00020000;
pub const EXCHGID4_FLAG_USE_PNFS_DS: c_uint = 0x00040000;
pub const EXCHGID4_FLAG_MASK_PNFS: c_uint = 0x00070000;
pub const EXCHGID4_FLAG_UPD_CONFIRMED_REC_A: c_uint = 0x40000000;
pub const EXCHGID4_FLAG_CONFIRMED_R: c_uint = 0x80000000;
pub const EXCHGID4_FLAG_SUPP_FENCE_OPS: c_uint = 0x00000004;
//
// Since the validity of these bits depends on whether
// they're set in the argument or response, have separate
// invalid flag masks for arg (_A) and resp (_R).
//
pub const EXCHGID4_FLAG_MASK_A: c_uint = 0x40070103;
pub const EXCHGID4_FLAG_MASK_R: c_uint = 0x80070103;
pub const EXCHGID4_2_FLAG_MASK_R: c_uint = 0x80070107;
pub const SEQ4_STATUS_CB_PATH_DOWN: c_uint = 0x00000001;
pub const SEQ4_STATUS_CB_GSS_CONTEXTS_EXPIRING: c_uint = 0x00000002;
pub const SEQ4_STATUS_CB_GSS_CONTEXTS_EXPIRED: c_uint = 0x00000004;
pub const SEQ4_STATUS_EXPIRED_ALL_STATE_REVOKED: c_uint = 0x00000008;
pub const SEQ4_STATUS_EXPIRED_SOME_STATE_REVOKED: c_uint = 0x00000010;
pub const SEQ4_STATUS_ADMIN_STATE_REVOKED: c_uint = 0x00000020;
pub const SEQ4_STATUS_RECALLABLE_STATE_REVOKED: c_uint = 0x00000040;
pub const SEQ4_STATUS_LEASE_MOVED: c_uint = 0x00000080;
pub const SEQ4_STATUS_RESTART_RECLAIM_NEEDED: c_uint = 0x00000100;
pub const SEQ4_STATUS_CB_PATH_DOWN_SESSION: c_uint = 0x00000200;
pub const SEQ4_STATUS_BACKCHANNEL_FAULT: c_uint = 0x00000400;
pub const NFS4_SECINFO_STYLE4_CURRENT_FH: c_int = 0;
pub const NFS4_SECINFO_STYLE4_PARENT: c_int = 1;

// An NFS4 sessions server must support at least NFS4_MAX_OPS operations.
// If a compound requires more operations, adjust NFS4_MAX_OPS accordingly.
//
pub const NFS4_MAX_OPS: c_int = 8;
// Our NFS4 client back channel server only wants the cb_sequene and the
// actual operation per compound
//
pub const NFS4_MAX_BACK_CHANNEL_OPS: c_int = 2;
