//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/t7xx/t7xx_modem_ops.h
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
// Moises Veleta <moises.veleta@intel.com>
// Ricardo Martinez <ricardo.martinez@linux.intel.com>
//
// Contributors:
// Amir Hanania <amir.hanania@intel.com>
// Chiranjeevi Rapolu <chiranjeevi.rapolu@intel.com>
// Sreehari Kancharla <sreehari.kancharla@intel.com>
//

pub const FEATURE_COUNT: c_int = 64;
//
// enum hif_ex_stage -	HIF exception handshake stages with the HW.
// @HIF_EX_INIT:        Disable and clear TXQ.
// @HIF_EX_INIT_DONE:   Polling for initialization to be done.
// @HIF_EX_CLEARQ_DONE: Disable RX, flush TX/RX workqueues and clear RX.
// @HIF_EX_ALLQ_RESET:  HW is back in safe mode for re-initialization and restart.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hif_ex_stage {
    HIF_EX_INIT,
    HIF_EX_INIT_DONE,
    HIF_EX_CLEARQ_DONE,
    HIF_EX_ALLQ_RESET,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_runtime_feature {
    pub feature_id: u8,
    pub support_info: u8,
    pub reserved: [u8; 2],
    pub data_len: __le32,
    pub data: [__le32; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum md_event_id {
    FSM_PRE_START,
    FSM_START,
    FSM_READY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t7xx_sys_info {
    pub ready: bool,
    pub handshake_ongoing: bool,
    pub feature_set: [u8; FEATURE_COUNT],
    pub ctl_port: *mut t7xx_port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t7xx_modem {
    pub md_ctrl: [*mut cldma_ctrl; CLDMA_NUM],
    pub t7xx_dev: *mut t7xx_pci_dev,
    pub core_md: t7xx_sys_info,
    pub core_ap: t7xx_sys_info,
    pub md_init_finish: bool,
    pub rgu_irq_asserted: bool,
    pub handshake_wq: *mut workqueue_struct,
    pub handshake_work: work_struct,
    pub ap_handshake_work: work_struct,
    pub fsm_ctl: *mut t7xx_fsm_ctl,
    pub port_prox: *mut port_proxy,
    pub exp_id: c_uint,
    pub /: *mut *mut spinlock_t exp_lock; / Protects exception events,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reset_type {
    FLDR,
    PLDR,
    FASTBOOT,
}

extern "C" {
    pub fn t7xx_md_exception_handshake(md: *mut t7xx_modem);
}
extern "C" {
    pub fn t7xx_md_event_notify(md: *mut t7xx_modem, evt_id: md_event_id);
}
extern "C" {
    pub fn t7xx_md_reset(t7xx_dev: *mut t7xx_pci_dev) -> c_int;
}
extern "C" {
    pub fn t7xx_md_init(t7xx_dev: *mut t7xx_pci_dev) -> c_int;
}
extern "C" {
    pub fn t7xx_md_exit(t7xx_dev: *mut t7xx_pci_dev);
}
extern "C" {
    pub fn t7xx_clear_rgu_irq(t7xx_dev: *mut t7xx_pci_dev);
}
extern "C" {
    pub fn t7xx_reset_device(t7xx_dev: *mut t7xx_pci_dev, type: reset_type) -> c_int;
}
extern "C" {
    pub fn t7xx_pci_mhccif_isr(t7xx_dev: *mut t7xx_pci_dev) -> c_int;
}
