//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnxt/bnxt_devlink.h
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


// Broadcom NetXtreme-C/E network driver.
//
// Copyright (c) 2017 Broadcom Limited
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// Struct to hold housekeeping info needed by devlink interface
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_dl {
    pub /: *mut *mut *mut bnxt bp; / back ptr to the controlling dev,
    pub remote_reset: bool,
}

pub const NVM_OFF_MSIX_VEC_PER_PF_MAX: c_int = 108;
pub const NVM_OFF_MSIX_VEC_PER_PF_MIN: c_int = 114;
pub const NVM_OFF_IGNORE_ARI: c_int = 164;
pub const NVM_OFF_RDMA_CAPABLE: c_int = 161;
pub const NVM_OFF_DIS_GRE_VER_CHECK: c_int = 171;
pub const NVM_OFF_ENABLE_SRIOV: c_int = 401;
pub const NVM_OFF_SUPPORT_RDMA: c_int = 506;
pub const NVM_OFF_NVM_CFG_VER: c_int = 602;
pub const BNXT_NVM_CFG_VER_BITS: c_int = 8;
pub const BNXT_NVM_CFG_VER_BYTES: c_int = 1;
pub const BNXT_MSIX_VEC_MAX: c_int = 512;
pub const BNXT_MSIX_VEC_MIN_MAX: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_nvm_dir_type {
    BNXT_NVM_SHARED_CFG = 40,
    BNXT_NVM_PORT_CFG,
    BNXT_NVM_FUNC_CFG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_dl_nvm_param {
    pub id: u16,
    pub offset: u16,
    pub dir_type: u16,
    pub nvm_num_bits: u16,
    pub dl_num_bytes: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_dl_version_type {
    BNXT_VERSION_FIXED,
    BNXT_VERSION_RUNNING,
    BNXT_VERSION_STORED,
}

extern "C" {
    pub fn bnxt_devlink_health_fw_report(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_dl_health_fw_status_update(bp: *mut bnxt, healthy: bool);
}
extern "C" {
    pub fn bnxt_dl_health_fw_recovery_done(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_dl_fw_reporters_create(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_dl_fw_reporters_destroy(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_dl_register(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_dl_unregister(bp: *mut bnxt);
}
