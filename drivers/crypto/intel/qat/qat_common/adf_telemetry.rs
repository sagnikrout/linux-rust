//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_telemetry.h
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
// Copyright (c) 2023 Intel Corporation.

// Interval within device writes data to DMA region. Value in milliseconds.
pub const ADF_TL_DATA_WR_INTERVAL_MS: c_int = 1000;
// Interval within timer interrupt should be handled. Value in milliseconds.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_tl_hw_data {
    pub layout_sz: usize,
    pub slice_reg_sz: usize,
    pub cmdq_reg_sz: usize,
    pub rp_reg_sz: usize,
    pub msg_cnt_off: usize,
    pub dev_counters: *const adf_tl_dbg_counter,
    pub sl_util_counters: *const adf_tl_dbg_counter,
    pub sl_exec_counters: *const adf_tl_dbg_counter,
    pub cmdq_counters: *const adf_tl_dbg_counter,
    pub rp_counters: *const adf_tl_dbg_counter,
    pub num_hbuff: u8,
    pub cpp_ns_per_cycle: u8,
    pub bw_units_to_bytes: u8,
    pub num_dev_counters: u8,
    pub num_rp_counters: u8,
    pub num_cmdq_counters: u8,
    pub max_rp: u8,
    pub max_sl_cnt: u8,
    pub multiplier: icp_qat_fw_init_admin_slice_cnt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_telemetry {
    pub accel_dev: *mut adf_accel_dev,
    pub state: core::sync::atomic::AtomicI32,
    pub hbuffs: u32,
    pub hb_num: c_int,
    pub msg_cnt: u32,
    pub /: *mut *mut dma_addr_t regs_data_p; / bus address for DMA mapping,
    pub /: *mut *mut *mut void regs_data; / virtual address for DMA mapping,
//
// @regs_hist_buff: array of pointers to copies of the last @hbuffs
// values of @regs_data
//
    pub regs_hist_buff: *mut c_void,
    pub dbg_dir: *mut dentry,
    pub rp_num_indexes: *mut u8,
//
// @regs_hist_lock: protects from race conditions between write and read
// to the copies referenced by @regs_hist_buff
//
    pub regs_hist_lock: mutex,
//
// @wr_lock: protects from concurrent writes to debugfs telemetry files
//
    pub wr_lock: mutex,
    pub work_ctx: delayed_work,
    pub slice_cnt: icp_qat_fw_init_admin_slice_cnt,
    pub cmdq_cnt: icp_qat_fw_init_admin_slice_cnt,
}

extern "C" {
    pub fn adf_tl_init(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_tl_start(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_tl_stop(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_tl_shutdown(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_tl_run(accel_dev: *mut adf_accel_dev, state: c_int) -> c_int;
}
extern "C" {
    pub fn adf_tl_halt(accel_dev: *mut adf_accel_dev) -> c_int;
}

