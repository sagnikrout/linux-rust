//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_cfg.h
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_cfg_key_val {
    pub key: [c_char; ADF_CFG_MAX_KEY_LEN_IN_BYTES],
    pub val: [c_char; ADF_CFG_MAX_VAL_LEN_IN_BYTES],
    pub type: adf_cfg_val_type,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_cfg_section {
    pub name: [c_char; ADF_CFG_MAX_SECTION_LEN_IN_BYTES],
    pub list: list_head,
    pub param_head: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_cfg_device_data {
    pub sec_list: list_head,
    pub debug: *mut dentry,
    pub lock: rw_semaphore,
}

extern "C" {
    pub fn adf_cfg_dev_add(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_cfg_dev_remove(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_cfg_dev_dbgfs_add(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_cfg_dev_dbgfs_rm(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_cfg_section_add(accel_dev: *mut adf_accel_dev, name: *const c_char) -> c_int;
}
