//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns3/hns3pf/hclge_fd.h
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
// Copyright (c) 2026 Hisilicon Limited.
extern "C" {
    pub fn hclge_init_fd_config(hdev: *mut hclge_dev) -> c_int;
}
extern "C" {
    pub fn hclge_add_fd_entry(handle: *mut hnae3_handle, cmd: *mut ethtool_rxnfc) -> c_int;
}
extern "C" {
    pub fn hclge_del_fd_entry(handle: *mut hnae3_handle, cmd: *mut ethtool_rxnfc) -> c_int;
}
extern "C" {
    pub fn hclge_del_all_fd_entries(hdev: *mut hclge_dev);
}
extern "C" {
    pub fn hclge_restore_fd_entries(handle: *mut hnae3_handle) -> c_int;
}
extern "C" {
    pub fn hclge_enable_fd(handle: *mut hnae3_handle, enable: bool);
}
extern "C" {
    pub fn hclge_is_cls_flower_active(handle: *mut hnae3_handle) -> bool;
}
extern "C" {
    pub fn hclge_clear_arfs_rules(hdev: *mut hclge_dev) -> c_int;
}
extern "C" {
    pub fn hclge_sync_fd_table(hdev: *mut hclge_dev);
}
extern "C" {
    pub fn hclge_rfs_filter_expire(hdev: *mut hclge_dev);
}
