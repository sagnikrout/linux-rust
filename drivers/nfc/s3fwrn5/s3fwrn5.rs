//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nfc/s3fwrn5/s3fwrn5.h
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
// NCI based driver for Samsung S3FWRN5 NFC chip
//
// Copyright (C) 2015 Samsung Electronics
// Robert Baldyga <r.baldyga@samsung.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s3fwrn5_mode {
    S3FWRN5_MODE_COLD,
    S3FWRN5_MODE_NCI,
    S3FWRN5_MODE_FW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3fwrn5_phy_ops {
    pub sleep): *mut *mut *mut void (set_wake)(void id, bool,
    pub s3fwrn5_mode): *mut *mut *mut void (set_mode)(void id, enum,
    pub id): *mut *mut s3fwrn5_mode (get_mode)(void,
    pub skb): *mut *mut *mut int (write)(void id, struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3fwrn5_info {
    pub ndev: *mut nci_dev,
    pub phy_id: *mut c_void,
    pub pdev: *mut device,
    pub phy_ops: *const s3fwrn5_phy_ops,
    pub fw_info: s3fwrn5_fw_info,
    pub mutex: mutex,
}

extern "C" {
    pub fn s3fwrn5_remove(ndev: *mut nci_dev);
}
