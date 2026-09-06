//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns3/hns3_debugfs.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2021 Hisilicon Limited.

pub const HNS3_DBG_ITEM_NAME_LEN: c_int = 32;
pub const HNS3_DBG_FILE_NAME_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_dbg_item {
    pub name: [c_char; HNS3_DBG_ITEM_NAME_LEN],
    pub /: *mut *mut u16 interval; / blank numbers after the item,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_dbg_data {
    pub handle: *mut hnae3_handle,
    pub cmd: hnae3_dbg_cmd,
    pub qid: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns3_dbg_dentry_type {
    HNS3_DBG_DENTRY_TM,
    HNS3_DBG_DENTRY_TX_BD,
    HNS3_DBG_DENTRY_RX_BD,
    HNS3_DBG_DENTRY_MAC,
    HNS3_DBG_DENTRY_REG,
    HNS3_DBG_DENTRY_QUEUE,
    HNS3_DBG_DENTRY_FD,
    HNS3_DBG_DENTRY_COMMON,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_dbg_dentry_info {
    pub name: *const c_char,
    pub dentry: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_dbg_cmd_info {
    pub name: *const c_char,
    pub cmd: hnae3_dbg_cmd,
    pub dentry: hns3_dbg_dentry_type,
    pub cmd): *mut *mut *mut int (init)(struct hnae3_handle handle, unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_dbg_cap_info {
    pub name: *const c_char,
    pub cap_bit: HNAE3_DEV_CAP_BITS,
}
