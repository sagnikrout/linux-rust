//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_heartbeat.h
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
// Copyright(c) 2023 Intel Corporation

pub const ADF_CFG_HB_TIMER_MIN_MS: c_int = 200;
pub const ADF_CFG_HB_TIMER_DEFAULT_MS: c_int = 500;
pub const ADF_CFG_HB_COUNT_THRESHOLD: c_int = 3;
pub const ADF_CFG_HB_RESET_MS: c_int = 5000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adf_device_heartbeat_status {
    HB_DEV_UNRESPONSIVE = 0,
    HB_DEV_ALIVE,
    HB_DEV_UNSUPPORTED,
}

// Heartbeat counter pair
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hb_cnt_pair {
    pub resp_heartbeat_cnt: __u16,
    pub req_heartbeat_cnt: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_heartbeat {
    pub hb_sent_counter: c_uint,
    pub hb_failed_counter: c_uint,
    pub hb_timer: c_uint,
    pub last_hb_check_time: u64,
    pub last_hb_reset_time: u64,
    pub ctrs_cnt_checked: bool,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hb_dma_addr {
    pub phy_addr: dma_addr_t,
    pub virt_addr: *mut c_void,
    pub dma: },
    pub base_dir: *mut dentry,
    pub status: *mut dentry,
    pub cfg: *mut dentry,
    pub sent: *mut dentry,
    pub failed: *mut dentry,

    pub inject_error: *mut dentry,

    pub dbgfs: },
}

extern "C" {
    pub fn adf_heartbeat_init(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_heartbeat_start(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_heartbeat_shutdown(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_heartbeat_check_ctrs(accel_dev: *mut adf_accel_dev);
}

extern "C" {
    pub fn adf_heartbeat_inject_error(accel_dev: *mut adf_accel_dev) -> c_int;
}

