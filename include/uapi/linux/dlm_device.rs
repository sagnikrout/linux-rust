//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dlm_device.h
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
// This is the device interface for dlm, most users will use a library
// interface.
//

pub const DLM_USER_LVB_LEN: c_int = 32;
// Version of the device interface
pub const DLM_DEVICE_VERSION_MAJOR: c_int = 6;
pub const DLM_DEVICE_VERSION_MINOR: c_int = 0;
pub const DLM_DEVICE_VERSION_PATCH: c_int = 2;
// struct passed to the lock write
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_lock_params {
    pub mode: __u8,
    pub namelen: __u8,
    pub unused: __u16,
    pub flags: __u32,
    pub lkid: __u32,
    pub parent: __u32,
    pub xid: __u64,
    pub timeout: __u64,
    pub castparam: *mut void __user,
    pub castaddr: *mut void __user,
    pub bastparam: *mut void __user,
    pub bastaddr: *mut void __user,
    pub lksb: *mut dlm_lksb __user,
    pub lvb: [c_char; DLM_USER_LVB_LEN],
    pub name: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_lspace_params {
    pub flags: __u32,
    pub minor: __u32,
    pub name: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_purge_params {
    pub nodeid: __u32,
    pub pid: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_write_request {
    pub version: [__u32; 3],
    pub cmd: __u8,
    pub is64bit: __u8,
    pub unused: [__u8; 2],
    pub lock: dlm_lock_params,
    pub lspace: dlm_lspace_params,
    pub purge: dlm_purge_params,
    pub i: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_device_version {
    pub version: [__u32; 3],
}

// struct read from the "device" fd,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_lock_result {
    pub version: [__u32; 3],
    pub length: __u32,
    pub user_astaddr: *mut *mut void __user,
    pub user_astparam: *mut *mut void __user,
    pub user_lksb: *mut *mut dlm_lksb __user,
    pub lksb: dlm_lksb,
    pub bast_mode: __u8,
    pub unused: [__u8; 3],
// Offsets may be zero if no data is present
    pub lvb_offset: __u32,
}

// Commands passed to the device
pub const DLM_USER_LOCK: c_int = 1;
pub const DLM_USER_UNLOCK: c_int = 2;
pub const DLM_USER_QUERY: c_int = 3;
pub const DLM_USER_CREATE_LOCKSPACE: c_int = 4;
pub const DLM_USER_REMOVE_LOCKSPACE: c_int = 5;
pub const DLM_USER_PURGE: c_int = 6;
pub const DLM_USER_DEADLOCK: c_int = 7;
// Lockspace flags
pub const DLM_USER_LSFLG_AUTOFREE: c_int = 1;
pub const DLM_USER_LSFLG_FORCEFREE: c_int = 2;
