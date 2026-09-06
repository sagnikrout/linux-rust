//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nfc/nxp-nci/nxp-nci.h
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
// Copyright (C) 2014  NXP Semiconductors  All rights reserved.
//
// Authors: Clément Perrochaud <clement.perrochaud@nxp.com>
//
// Derived from PN544 device driver:
// Copyright (C) 2012  Intel Corporation. All rights reserved.
//

pub const NXP_NCI_FW_HDR_LEN: c_int = 2;
pub const NXP_NCI_FW_CRC_LEN: c_int = 2;
pub const NXP_NCI_FW_FRAME_LEN_MASK: c_uint = 0x03FF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nxp_nci_mode {
    NXP_NCI_MODE_COLD,
    NXP_NCI_MODE_NCI,
    NXP_NCI_MODE_FW
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxp_nci_phy_ops {
    pub mode): *mut *mut *mut int (set_mode)(void id, enum nxp_nci_mode,
    pub skb): *mut *mut *mut int (write)(void id, struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxp_nci_fw_info {
    pub 1]: char name[NFC_FIRMWARE_NAME_MAXSIZE +,
    pub fw: *const firmware,
    pub size: usize,
    pub written: usize,
    pub data: *const u8,
    pub frame_size: usize,
    pub work: work_struct,
    pub cmd_completion: completion,
    pub cmd_result: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxp_nci_info {
    pub ndev: *mut nci_dev,
    pub phy_id: *mut c_void,
    pub pdev: *mut device,
    pub mode: nxp_nci_mode,
    pub phy_ops: *const nxp_nci_phy_ops,
    pub max_payload: c_uint,
    pub info_lock: mutex,
    pub fw_info: nxp_nci_fw_info,
}

extern "C" {
    pub fn nxp_nci_fw_download(ndev: *mut nci_dev, firmware_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn nxp_nci_fw_work(work: *mut work_struct);
}
extern "C" {
    pub fn nxp_nci_fw_recv_frame(ndev: *mut nci_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn nxp_nci_fw_work_complete(info: *mut nxp_nci_info, result: c_int);
}
extern "C" {
    pub fn nxp_nci_remove(ndev: *mut nci_dev);
}
