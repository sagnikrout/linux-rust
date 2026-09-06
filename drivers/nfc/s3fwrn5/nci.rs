//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nfc/s3fwrn5/nci.h
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

pub const NCI_PROP_SET_RFREG: c_uint = 0x22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_prop_set_rfreg_cmd {
    pub index: __u8,
    pub data: [__u8; 252],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_prop_set_rfreg_rsp {
    pub status: __u8,
}

pub const NCI_PROP_START_RFREG: c_uint = 0x26;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_prop_start_rfreg_rsp {
    pub status: __u8,
}

pub const NCI_PROP_STOP_RFREG: c_uint = 0x27;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_prop_stop_rfreg_cmd {
    pub checksum: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_prop_stop_rfreg_rsp {
    pub status: __u8,
}

pub const NCI_PROP_FW_CFG: c_uint = 0x28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_prop_fw_cfg_cmd {
    pub clk_type: __u8,
    pub clk_speed: __u8,
    pub clk_req: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_prop_fw_cfg_rsp {
    pub status: __u8,
}

extern "C" {
    pub fn s3fwrn5_nci_rf_configure(info: *mut s3fwrn5_info, fw_name: *const c_char) -> c_int;
}
