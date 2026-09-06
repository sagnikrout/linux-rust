//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dlmconstants.h
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
// Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
// Copyright (C) 2004-2007 Red Hat, Inc.  All rights reserved.
//
// This copyrighted material is made available to anyone wishing to use,
// modify, copy, or redistribute it subject to the terms and conditions
// of the GNU General Public License v.2.
//
// Constants used by DLM interface.
//
pub const DLM_LOCKSPACE_LEN: c_int = 64;
pub const DLM_RESNAME_MAXLEN: c_int = 64;
//
// Lock Modes
//

//
// Flags to dlm_lock
//
// DLM_LKF_NOQUEUE
//
// Do not queue the lock request on the wait queue if it cannot be granted
// immediately.  If the lock cannot be granted because of this flag, DLM will
// either return -EAGAIN from the dlm_lock call or will return 0 from
// dlm_lock and -EAGAIN in the lock status block when the AST is executed.
//
// DLM_LKF_CANCEL
//
// Used to cancel a pending lock request or conversion.  A converting lock is
// returned to its previously granted mode.
//
// DLM_LKF_CONVERT
//
// Indicates a lock conversion request.  For conversions the name and namelen
// are ignored and the lock ID in the LKSB is used to identify the lock.
//
// DLM_LKF_VALBLK
//
// Requests DLM to return the current contents of the lock value block in the
// lock status block.  When this flag is set in a lock conversion from PW or EX
// modes, DLM assigns the value specified in the lock status block to the lock
// value block of the lock resource.  The LVB is a DLM_LVB_LEN size array
// containing application-specific information.
//
// DLM_LKF_QUECVT
//
// Force a conversion request to be queued, even if it is compatible with
// the granted modes of other locks on the same resource.
//
// DLM_LKF_IVVALBLK
//
// Invalidate the lock value block.
//
// DLM_LKF_CONVDEADLK
//
// Allows the dlm to resolve conversion deadlocks internally by demoting the
// granted mode of a converting lock to NL.  The DLM_SBF_DEMOTED flag is
// returned for a conversion that's been effected by this.
//
// DLM_LKF_PERSISTENT
//
// Only relevant to locks originating in userspace.  A persistent lock will not
// be removed if the process holding the lock exits.
//
// DLM_LKF_NODLCKWT
//
// Do not cancel the lock if it gets into conversion deadlock.
//
// DLM_LKF_NODLCKBLK
//
// net yet implemented
//
// DLM_LKF_EXPEDITE
//
// Used only with new requests for NL mode locks.  Tells the lock manager
// to grant the lock, ignoring other locks in convert and wait queues.
//
// DLM_LKF_NOQUEUEBAST
//
// Send blocking AST's before returning -EAGAIN to the caller.  It is only
// used along with the NOQUEUE flag.  Blocking AST's are not sent for failed
// NOQUEUE requests otherwise.
//
// DLM_LKF_HEADQUE
//
// Add a lock to the head of the convert or wait queue rather than the tail.
//
// DLM_LKF_NOORDER
//
// Disregard the standard grant order rules and grant a lock as soon as it
// is compatible with other granted locks.
//
// DLM_LKF_ORPHAN
//
// Acquire an orphan lock.
//
// DLM_LKF_ALTPR
//
// If the requested mode cannot be granted immediately, try to grant the lock
// in PR mode instead.  If this alternate mode is granted instead of the
// requested mode, DLM_SBF_ALTMODE is returned in the lksb.
//
// DLM_LKF_ALTCW
//
// The same as ALTPR, but the alternate mode is CW.
//
// DLM_LKF_FORCEUNLOCK
//
// Unlock the lock even if it is converting or waiting or has sublocks.
// Only really for use by the userland device.c code.
//
// DLM_LKF_TIMEOUT
//
// This value is deprecated and reserved. DO NOT USE!
//
pub const DLM_LKF_NOQUEUE: c_uint = 0x00000001;
pub const DLM_LKF_CANCEL: c_uint = 0x00000002;
pub const DLM_LKF_CONVERT: c_uint = 0x00000004;
pub const DLM_LKF_VALBLK: c_uint = 0x00000008;
pub const DLM_LKF_QUECVT: c_uint = 0x00000010;
pub const DLM_LKF_IVVALBLK: c_uint = 0x00000020;
pub const DLM_LKF_CONVDEADLK: c_uint = 0x00000040;
pub const DLM_LKF_PERSISTENT: c_uint = 0x00000080;
pub const DLM_LKF_NODLCKWT: c_uint = 0x00000100;
pub const DLM_LKF_NODLCKBLK: c_uint = 0x00000200;
pub const DLM_LKF_EXPEDITE: c_uint = 0x00000400;
pub const DLM_LKF_NOQUEUEBAST: c_uint = 0x00000800;
pub const DLM_LKF_HEADQUE: c_uint = 0x00001000;
pub const DLM_LKF_NOORDER: c_uint = 0x00002000;
pub const DLM_LKF_ORPHAN: c_uint = 0x00004000;
pub const DLM_LKF_ALTPR: c_uint = 0x00008000;
pub const DLM_LKF_ALTCW: c_uint = 0x00010000;
pub const DLM_LKF_FORCEUNLOCK: c_uint = 0x00020000;
pub const DLM_LKF_TIMEOUT: c_uint = 0x00040000;
//
// Some return codes that are not in errno.h
//
pub const DLM_ECANCEL: c_uint = 0x10001;
pub const DLM_EUNLOCK: c_uint = 0x10002;
