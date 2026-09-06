//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_debug.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015 QLogic Corporation
// Copyright (c) 2019-2021 Marvell International Ltd.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_dbg_features {
    DBG_FEATURE_GRC,
    DBG_FEATURE_IDLE_CHK,
    DBG_FEATURE_MCP_TRACE,
    DBG_FEATURE_REG_FIFO,
    DBG_FEATURE_IGU_FIFO,
    DBG_FEATURE_PROTECTION_OVERRIDE,
    DBG_FEATURE_FW_ASSERTS,
    DBG_FEATURE_ILT,
    DBG_FEATURE_NUM
}

// Forward Declaration
extern "C" {
    pub fn qed_dbg_grc(cdev: *mut qed_dev, buffer: *mut c_void, num_dumped_bytes: *mut u32) -> c_int;
}
extern "C" {
    pub fn qed_dbg_grc_size(cdev: *mut qed_dev) -> c_int;
}
extern "C" {
    pub fn qed_dbg_idle_chk_size(cdev: *mut qed_dev) -> c_int;
}
extern "C" {
    pub fn qed_dbg_reg_fifo_size(cdev: *mut qed_dev) -> c_int;
}
extern "C" {
    pub fn qed_dbg_igu_fifo_size(cdev: *mut qed_dev) -> c_int;
}
extern "C" {
    pub fn qed_dbg_protection_override_size(cdev: *mut qed_dev) -> c_int;
}
extern "C" {
    pub fn qed_dbg_fw_asserts_size(cdev: *mut qed_dev) -> c_int;
}
extern "C" {
    pub fn qed_dbg_ilt(cdev: *mut qed_dev, buffer: *mut c_void, num_dumped_bytes: *mut u32) -> c_int;
}
extern "C" {
    pub fn qed_dbg_ilt_size(cdev: *mut qed_dev) -> c_int;
}
extern "C" {
    pub fn qed_dbg_mcp_trace_size(cdev: *mut qed_dev) -> c_int;
}
extern "C" {
    pub fn qed_dbg_phy_size(cdev: *mut qed_dev) -> c_int;
}
extern "C" {
    pub fn qed_dbg_all_data(cdev: *mut qed_dev, buffer: *mut c_void) -> c_int;
}
extern "C" {
    pub fn qed_dbg_all_data_size(cdev: *mut qed_dev) -> c_int;
}
extern "C" {
    pub fn qed_get_debug_engine(cdev: *mut qed_dev) -> u8;
}
extern "C" {
    pub fn qed_set_debug_engine(cdev: *mut qed_dev, engine_number: c_int);
}
extern "C" {
    pub fn qed_dbg_feature_size(cdev: *mut qed_dev, feature: qed_dbg_features) -> c_int;
}
extern "C" {
    pub fn qed_dbg_pf_init(cdev: *mut qed_dev);
}
extern "C" {
    pub fn qed_dbg_pf_exit(cdev: *mut qed_dev);
}
