//! Automatically rewritten from C Header to Rust Module
//! Source: fs/afs/afs_cm.h
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
// AFS Cache Manager definitions
//
// Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AFS_CM_Operations {
    CBCallBack		= 204,	/* break callback promises */
    CBInitCallBackState	= 205,	/* initialise callback state */
    CBProbe			= 206,	/* probe client */
    CBGetLock		= 207,	/* get contents of CM lock table */
    CBGetCE			= 208,	/* get cache file description */
    CBGetXStatsVersion	= 209,	/* get version of extended statistics */
    CBGetXStats		= 210,	/* get contents of extended statistics data */
    CBInitCallBackState3	= 213,	/* initialise callback state, version 3 */
    CBProbeUuid		= 214,	/* check the client hasn't rebooted */
    CBTellMeAboutYourself	= 65538, /* get client capabilities */
}

pub const AFS_CAP_ERROR_TRANSLATION: c_uint = 0x1;
