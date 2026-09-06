//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/t7xx/t7xx_hif_dpmaif_rx.h
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
//
// Copyright (c) 2021, MediaTek Inc.
// Copyright (c) 2021-2022, Intel Corporation.
//
// Authors:
// Haijun Liu <haijun.liu@mediatek.com>
// Eliot Lee <eliot.lee@intel.com>
// Ricardo Martinez <ricardo.martinez@linux.intel.com>
//
// Contributors:
// Amir Hanania <amir.hanania@intel.com>
// Moises Veleta <moises.veleta@intel.com>
// Sreehari Kancharla <sreehari.kancharla@intel.com>
//

pub const PKT_TYPE_IP4: c_int = 0;
pub const PKT_TYPE_IP6: c_int = 1;
// Structure of DL PIT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_pit {
    pub header: __le32,
    pub data_addr_l: __le32,
    pub data_addr_h: __le32,
    pub footer: __le32,
    pub pd: },
    pub params_1: __le32,
    pub params_2: __le32,
    pub params_3: __le32,
    pub msg: },
}

// PIT header fields

// PIT footer fields

extern "C" {
    pub fn t7xx_dpmaif_rxq_init(queue: *mut dpmaif_rx_queue) -> c_int;
}
extern "C" {
    pub fn t7xx_dpmaif_rx_clear(dpmaif_ctrl: *mut dpmaif_ctrl);
}
extern "C" {
    pub fn t7xx_dpmaif_bat_rel_wq_alloc(dpmaif_ctrl: *mut dpmaif_ctrl) -> c_int;
}
extern "C" {
    pub fn t7xx_dpmaif_rx_stop(dpmaif_ctrl: *mut dpmaif_ctrl);
}
extern "C" {
    pub fn t7xx_dpmaif_irq_rx_done(dpmaif_ctrl: *mut dpmaif_ctrl, que_mask: c_uint);
}
extern "C" {
    pub fn t7xx_dpmaif_rxq_free(queue: *mut dpmaif_rx_queue);
}
extern "C" {
    pub fn t7xx_dpmaif_bat_wq_rel(dpmaif_ctrl: *mut dpmaif_ctrl);
}
extern "C" {
    pub fn t7xx_dpmaif_napi_rx_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
