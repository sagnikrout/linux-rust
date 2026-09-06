//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/t7xx/t7xx_hif_dpmaif_tx.h
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
// Chiranjeevi Rapolu <chiranjeevi.rapolu@intel.com>
// Moises Veleta <moises.veleta@intel.com>
// Sreehari Kancharla <sreehari.kancharla@intel.com>
//

pub const DPMAIF_TX_DEFAULT_QUEUE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_drb {
    pub header: __le32,
    pub data_addr_l: __le32,
    pub data_addr_h: __le32,
    pub pd: },
    pub msg_hdr: __le32,
    pub reserved1: __le32,
    pub msg: },
}

// Header fields

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmaif_drb_skb {
    pub skb: *mut sk_buff,
    pub bus_addr: dma_addr_t,
    pub data_len: c_uint,
    pub index:13: u16,
    pub is_msg:1: u16,
    pub is_frag:1: u16,
    pub is_last:1: u16,
}

extern "C" {
    pub fn t7xx_dpmaif_tx_thread_rel(dpmaif_ctrl: *mut dpmaif_ctrl);
}
extern "C" {
    pub fn t7xx_dpmaif_tx_thread_init(dpmaif_ctrl: *mut dpmaif_ctrl) -> c_int;
}
extern "C" {
    pub fn t7xx_dpmaif_txq_free(txq: *mut dpmaif_tx_queue);
}
extern "C" {
    pub fn t7xx_dpmaif_irq_tx_done(dpmaif_ctrl: *mut dpmaif_ctrl, que_mask: c_uint);
}
extern "C" {
    pub fn t7xx_dpmaif_txq_init(txq: *mut dpmaif_tx_queue) -> c_int;
}
extern "C" {
    pub fn t7xx_dpmaif_tx_stop(dpmaif_ctrl: *mut dpmaif_ctrl);
}
extern "C" {
    pub fn t7xx_dpmaif_tx_clear(dpmaif_ctrl: *mut dpmaif_ctrl);
}
