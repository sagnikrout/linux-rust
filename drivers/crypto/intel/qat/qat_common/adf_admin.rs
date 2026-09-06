//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_admin.h
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

// Macro flag: #define ADF_ADMIN

extern "C" {
    pub fn adf_init_admin_comms(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_exit_admin_comms(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_send_admin_init(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_get_ae_fw_counters(accel_dev: *mut adf_accel_dev, ae: u16, reqs: *mut u64, resps: *mut u64) -> c_int;
}
extern "C" {
    pub fn adf_init_admin_pm(accel_dev: *mut adf_accel_dev, idle_delay: u32) -> c_int;
}
extern "C" {
    pub fn adf_send_admin_tim_sync(accel_dev: *mut adf_accel_dev, cnt: u32) -> c_int;
}
extern "C" {
    pub fn adf_send_admin_hb_timer(accel_dev: *mut adf_accel_dev, ticks: u32) -> c_int;
}
extern "C" {
    pub fn adf_get_fw_timestamp(accel_dev: *mut adf_accel_dev, timestamp: *mut u64) -> c_int;
}
extern "C" {
    pub fn adf_get_pm_info(accel_dev: *mut adf_accel_dev, p_state_addr: dma_addr_t, buff_size: usize) -> c_int;
}
extern "C" {
    pub fn adf_get_cnv_stats(accel_dev: *mut adf_accel_dev, ae: u16, err_cnt: *mut u16, latest_err: *mut u16) -> c_int;
}
extern "C" {
    pub fn adf_send_admin_tl_stop(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_send_admin_arb_query(accel_dev: *mut adf_accel_dev, cmd: c_int, svn: *mut u8) -> c_int;
}
extern "C" {
    pub fn adf_send_admin_arb_commit(accel_dev: *mut adf_accel_dev) -> c_int;
}
