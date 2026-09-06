//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/vtpm_proxy.h
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
// Definitions for the VTPM proxy driver
// Copyright (c) 2015, 2016, IBM Corporation
// Copyright (C) 2016 Intel Corporation
//
// This program is free software; you can redistribute it and/or modify it
// under the terms and conditions of the GNU General Public License,
// version 2, as published by the Free Software Foundation.
//
// This program is distributed in the hope it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
// more details.
//

//
// enum vtpm_proxy_flags - flags for the proxy TPM
// @VTPM_PROXY_FLAG_TPM2:	the proxy TPM uses TPM 2.0 protocol
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vtpm_proxy_flags {
    VTPM_PROXY_FLAG_TPM2	= 1,
}

//
// struct vtpm_proxy_new_dev - parameter structure for the
// %VTPM_PROXY_IOC_NEW_DEV ioctl
// @flags:	flags for the proxy TPM
// @tpm_num:	index of the TPM device
// @fd:		the file descriptor used by the proxy TPM
// @major:	the major number of the TPM device
// @minor:	the minor number of the TPM device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vtpm_proxy_new_dev {
    pub /: *mut *mut __u32 flags; / input,
    pub /: *mut *mut __u32 tpm_num; / output,
    pub /: *mut *mut __u32 fd; / output,
    pub /: *mut *mut __u32 major; / output,
    pub /: *mut *mut __u32 minor; / output,
}

// vendor specific commands to set locality
pub const TPM2_CC_SET_LOCALITY: c_uint = 0x20001000;
pub const TPM_ORD_SET_LOCALITY: c_uint = 0x20001000;
