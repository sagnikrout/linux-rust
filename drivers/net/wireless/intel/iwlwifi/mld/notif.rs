//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/notif.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2024-2025 Intel Corporation
//

// Macro flag: #define __iwl_mld_notif_h__
extern "C" {
    pub fn iwl_mld_async_handlers_wk(wiphy: *mut wiphy, wk: *mut wiphy_work);
}
extern "C" {
    pub fn iwl_mld_cancel_async_notifications(mld: *mut iwl_mld);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mld_object_type {
    IWL_MLD_OBJECT_TYPE_NONE,
    IWL_MLD_OBJECT_TYPE_LINK,
    IWL_MLD_OBJECT_TYPE_STA,
    IWL_MLD_OBJECT_TYPE_VIF,
    IWL_MLD_OBJECT_TYPE_ROC,
    IWL_MLD_OBJECT_TYPE_SCAN,
    IWL_MLD_OBJECT_TYPE_FTM_REQ,
    IWL_MLD_OBJECT_TYPE_NAN,
}

extern "C" {
    pub fn iwl_mld_delete_handlers(mld: *mut iwl_mld, cmds: *const u16, n_cmds: c_int);
}
