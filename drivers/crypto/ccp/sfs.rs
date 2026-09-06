//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccp/sfs.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// AMD Platform Security Processor (PSP) Seamless Firmware (SFS) Support.
//
// Copyright (C) 2025 Advanced Micro Devices, Inc.
//
// Author: Ashish Kalra <ashish.kalra@amd.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfs_misc_dev {
    pub refcount: kref,
    pub misc: miscdevice,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfs_command {
    pub hdr: psp_ext_req_buffer_hdr,
    pub psp_ext_req_buffer_hdr)]: u8 buf[PAGE_SIZE - sizeof(struct,
    pub sfs_buffer: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfs_device {
    pub dev: *mut device,
    pub psp: *mut psp_device,
    pub page: *mut page,
    pub command_buf: *mut sfs_command,
    pub misc: *mut sfs_misc_dev,
}

extern "C" {
    pub fn sfs_dev_destroy(psp: *mut psp_device);
}
extern "C" {
    pub fn sfs_dev_init(psp: *mut psp_device) -> c_int;
}
