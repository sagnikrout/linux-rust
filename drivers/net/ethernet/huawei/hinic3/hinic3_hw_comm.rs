//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_hw_comm.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) Huawei Technologies Co., Ltd. 2025. All rights reserved.

pub const HINIC3_WQ_PAGE_SIZE_ORDER: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_interrupt_info {
    pub lli_set: u32,
    pub interrupt_coalesc_set: u32,
    pub msix_index: u16,
    pub lli_credit_limit: u8,
    pub lli_timer_cfg: u8,
    pub pending_limit: u8,
    pub coalesc_timer_cfg: u8,
    pub resend_timer_cfg: u8,
}

extern "C" {
    pub fn hinic3_func_reset(hwdev: *mut hinic3_hwdev, func_id: u16, reset_flag: u64) -> c_int;
}
extern "C" {
    pub fn hinic3_set_cmdq_depth(hwdev: *mut hinic3_hwdev, cmdq_depth: u16) -> c_int;
}
extern "C" {
    pub fn hinic3_func_rx_tx_flush(hwdev: *mut hinic3_hwdev) -> c_int;
}
extern "C" {
    pub fn hinic3_sync_time_to_fw(hwdev: *mut hinic3_hwdev);
}
extern "C" {
    pub fn hinic3_clean_root_ctxt(hwdev: *mut hinic3_hwdev) -> c_int;
}
