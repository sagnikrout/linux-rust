//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/dlm/dlmapi.h
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
// dlmapi.h
//
// externally exported dlm interfaces
//
// Copyright (C) 2004 Oracle.  All rights reserved.
//
// NOTE: changes made to this enum should be reflected in dlmdebug.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dlm_status {
    DLM_NORMAL = 0,           /*  0: request in progress */
    DLM_GRANTED,              /*  1: request granted */
    DLM_DENIED,               /*  2: request denied */
    DLM_DENIED_NOLOCKS,       /*  3: request denied, out of system resources */
    DLM_WORKING,              /*  4: async request in progress */
    DLM_BLOCKED,              /*  5: lock request blocked */
    DLM_BLOCKED_ORPHAN,       /*  6: lock request blocked by a orphan lock*/
    DLM_DENIED_GRACE_PERIOD,  /*  7: topological change in progress */
    DLM_SYSERR,               /*  8: system error */
    DLM_NOSUPPORT,            /*  9: unsupported */
    DLM_CANCELGRANT,          /* 10: can't cancel convert: already granted */
    DLM_IVLOCKID,             /* 11: bad lockid */
    DLM_SYNC,                 /* 12: synchronous request granted */
    DLM_BADTYPE,              /* 13: bad resource type */
    DLM_BADRESOURCE,          /* 14: bad resource handle */
    DLM_MAXHANDLES,           /* 15: no more resource handles */
    DLM_NOCLINFO,             /* 16: can't contact cluster manager */
    DLM_NOLOCKMGR,            /* 17: can't contact lock manager */
    DLM_NOPURGED,             /* 18: can't contact purge daemon */
    DLM_BADARGS,              /* 19: bad api args */
    DLM_VOID,                 /* 20: no status */
    DLM_NOTQUEUED,            /* 21: NOQUEUE was specified and request failed */
    DLM_IVBUFLEN,             /* 22: invalid resource name length */
    DLM_CVTUNGRANT,           /* 23: attempted to convert ungranted lock */
    DLM_BADPARAM,             /* 24: invalid lock mode specified */
    DLM_VALNOTVALID,          /* 25: value block has been invalidated */
    DLM_REJECTED,             /* 26: request rejected, unrecognized client */
    DLM_ABORT,                /* 27: blocked lock request cancelled */
    DLM_CANCEL,               /* 28: conversion request cancelled */
    DLM_IVRESHANDLE,          /* 29: invalid resource handle */
    DLM_DEADLOCK,             /* 30: deadlock recovery refused this request */
    DLM_DENIED_NOASTS,        /* 31: failed to allocate AST */
    DLM_FORWARD,              /* 32: request must wait for primary's response */
    DLM_TIMEOUT,              /* 33: timeout value for lock has expired */
    DLM_IVGROUPID,            /* 34: invalid group specification */
    DLM_VERS_CONFLICT,        /* 35: version conflicts prevent request handling */
    DLM_BAD_DEVICE_PATH,      /* 36: Locks device does not exist or path wrong */
    DLM_NO_DEVICE_PERMISSION, /* 37: Client has insufficient pers for device */
    DLM_NO_CONTROL_DEVICE,    /* 38: Cannot set options on opened device */

    DLM_RECOVERING,           /* 39: extension, allows caller to fail a lock
    request if it is being recovered */
    DLM_MIGRATING,            /* 40: extension, allows caller to fail a lock
    request if it is being migrated */
    DLM_MAXSTATS,             /* 41: upper limit for return code validation */
}

// for pretty-printing dlm_status error names
// Eventually the DLM will use standard errno values, but in the
// meantime this lets us track dlm errors as they bubble up. When we
// bring its error reporting into line with the rest of the stack,
// these can just be replaced with calls to mlog_errno.

pub const DLM_LKSB_UNUSED1: c_uint = 0x01;
pub const DLM_LKSB_PUT_LVB: c_uint = 0x02;
pub const DLM_LKSB_GET_LVB: c_uint = 0x04;
pub const DLM_LKSB_UNUSED2: c_uint = 0x08;
pub const DLM_LKSB_UNUSED3: c_uint = 0x10;
pub const DLM_LKSB_UNUSED4: c_uint = 0x20;
pub const DLM_LKSB_UNUSED5: c_uint = 0x40;
pub const DLM_LKSB_UNUSED6: c_uint = 0x80;
pub const DLM_LVB_LEN: c_int = 64;
// Callers are only allowed access to the lvb and status members of
// this struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_lockstatus {
    pub status: dlm_status,
    pub flags: u32,
    pub lockid: *mut dlm_lock,
    pub lvb: [c_char; DLM_LVB_LEN],
}

// Valid lock modes.

pub const LKM_MAXMODE: c_int = 5;
pub const LKM_MODEMASK: c_uint = 0xff;
// Flags passed to dlmlock and dlmunlock:
// reserved: flags used by the "real" dlm
// only a few are supported by this dlm
// (U) = unsupported by ocfs2 dlm
pub const LKM_ORPHAN: c_uint = 0x00000010  /* this lock is orphanable (U) */;
pub const LKM_PARENTABLE: c_uint = 0x00000020  /* this lock was orphaned (U) */;
pub const LKM_BLOCK: c_uint = 0x00000040  /* blocking lock request (U) */;
pub const LKM_LOCAL: c_uint = 0x00000080  /* local lock request */;
pub const LKM_VALBLK: c_uint = 0x00000100  /* lock value block request */;
pub const LKM_NOQUEUE: c_uint = 0x00000200  /* non blocking request */;
pub const LKM_CONVERT: c_uint = 0x00000400  /* conversion request */;
pub const LKM_NODLCKWT: c_uint = 0x00000800  /* this lock won't deadlock (U) */;
pub const LKM_UNLOCK: c_uint = 0x00001000  /* deallocate this lock */;
pub const LKM_CANCEL: c_uint = 0x00002000  /* cancel conversion request */;
pub const LKM_DEQALL: c_uint = 0x00004000  /* remove all locks held by proc (U) */;
pub const LKM_INVVALBLK: c_uint = 0x00008000  /* invalidate lock value block */;
pub const LKM_SYNCSTS: c_uint = 0x00010000  /* return synchronous status if poss (U) */;
pub const LKM_TIMEOUT: c_uint = 0x00020000  /* lock request contains timeout (U) */;
pub const LKM_SNGLDLCK: c_uint = 0x00040000  /* request can self-deadlock (U) */;
pub const LKM_FINDLOCAL: c_uint = 0x00080000  /* find local lock request (U) */;
pub const LKM_PROC_OWNED: c_uint = 0x00100000  /* owned by process, not group (U) */;
pub const LKM_XID: c_uint = 0x00200000  /* use transaction id for deadlock (U) */;
pub const LKM_XID_CONFLICT: c_uint = 0x00400000  /* do not allow lock inheritance (U) */;
pub const LKM_FORCE: c_uint = 0x00800000  /* force unlock flag */;
pub const LKM_REVVALBLK: c_uint = 0x01000000  /* temporary solution: re-validate;
// unused
pub const LKM_UNUSED1: c_uint = 0x00000001  /* unused */;
pub const LKM_UNUSED2: c_uint = 0x00000002  /* unused */;
pub const LKM_UNUSED3: c_uint = 0x00000004  /* unused */;
pub const LKM_UNUSED4: c_uint = 0x00000008  /* unused */;
pub const LKM_UNUSED5: c_uint = 0x02000000  /* unused */;
pub const LKM_UNUSED6: c_uint = 0x04000000  /* unused */;
pub const LKM_UNUSED7: c_uint = 0x08000000  /* unused */;
// ocfs2 extensions: internal only
// should never be used by caller
pub const LKM_MIGRATION: c_uint = 0x10000000  /* extension: lockres is to be migrated;
pub const LKM_PUT_LVB: c_uint = 0x20000000  /* extension: lvb is being passed;
pub const LKM_GET_LVB: c_uint = 0x40000000  /* extension: lvb should be copied;
pub const LKM_RECOVERY: c_uint = 0x80000000  /* extension: flag for recovery lock;
extern "C" {
    pub fn void(: *mut dlm_astlockfunc_t)(void) -> typedef;
}
extern "C" {
    pub fn void(: *mut dlm_bastlockfunc_t)(void, _arg: c_int) -> typedef;
}
extern "C" {
    pub fn void(: *mut dlm_astunlockfunc_t)(void, dlm_status: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_protocol_version {
    pub pv_major: u8,
    pub pv_minor: u8,
}

extern "C" {
    pub fn dlm_unregister_domain(dlm: *mut dlm_ctxt);
}
extern "C" {
    pub fn dlm_print_one_lock(lockid: *mut dlm_lock);
}
extern "C" {
    pub fn void(_arg: dlm_eviction_func)(int, : *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_eviction_cb {
    pub ec_item: list_head,
    pub ec_func: *mut dlm_eviction_func,
    pub ec_data: *mut c_void,
}

extern "C" {
    pub fn dlm_unregister_eviction_cb(cb: *mut dlm_eviction_cb);
}
