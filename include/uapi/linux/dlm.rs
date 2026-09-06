//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dlm.h
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
// Copyright (C) 2004-2011 Red Hat, Inc.  All rights reserved.
//
// This copyrighted material is made available to anyone wishing to use,
// modify, copy, or redistribute it subject to the terms and conditions
// of the GNU General Public License v.2.
//
// Interface to Distributed Lock Manager (DLM)
// routines and structures to use DLM lockspaces
//
// Lock levels and flags are here

pub type dlm_lockspace_t = c_void;
//
// Lock status block
//
// Use this structure to specify the contents of the lock value block.  For a
// conversion request, this structure is used to specify the lock ID of the
// lock.  DLM writes the status of the lock request and the lock ID assigned
// to the request in the lock status block.
//
// sb_lkid: the returned lock ID.  It is set on new (non-conversion) requests.
// It is available when dlm_lock returns.
//
// sb_lvbptr: saves or returns the contents of the lock's LVB according to rules
// shown for the DLM_LKF_VALBLK flag.
//
// sb_flags: DLM_SBF_DEMOTED is returned if in the process of promoting a lock,
// it was first demoted to NL to avoid conversion deadlock.
// DLM_SBF_VALNOTVALID is returned if the resource's LVB is marked invalid.
//
// sb_status: the returned status of the lock request set prior to AST
// execution.  Possible return values:
//
// 0 if lock request was successful
// -EAGAIN if request would block and is flagged DLM_LKF_NOQUEUE
// -DLM_EUNLOCK if unlock request was successful
// -DLM_ECANCEL if a cancel completed successfully
// -EDEADLK if a deadlock was detected
// -ETIMEDOUT if the lock request was canceled due to a timeout
//
pub const DLM_SBF_DEMOTED: c_uint = 0x01;
pub const DLM_SBF_VALNOTVALID: c_uint = 0x02;
pub const DLM_SBF_ALTMODE: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_lksb {
    pub sb_status: c_int,
    pub sb_lkid: __u32,
    pub sb_flags: c_char,
    pub sb_lvbptr: *mut *mut c_char,
}

// dlm_new_lockspace() flags
// DLM_LSFL_TIMEWARN is deprecated and reserved. DO NOT USE!
pub const DLM_LSFL_TIMEWARN: c_uint = 0x00000002;
pub const DLM_LSFL_NEWEXCL: c_uint = 0x00000008;
// currently reserved due in-kernel use
pub const __DLM_LSFL_RESERVED0: c_uint = 0x00000010;
