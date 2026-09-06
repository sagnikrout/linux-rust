//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/t7xx/t7xx_netdev.h
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
// Moises Veleta <moises.veleta@intel.com>
//
// Contributors:
// Amir Hanania <amir.hanania@intel.com>
// Chiranjeevi Rapolu <chiranjeevi.rapolu@intel.com>
// Ricardo Martinez <ricardo.martinez@linux.intel.com>
//

pub const NIC_DEV_MAX: c_int = 21;
pub const NIC_DEV_DEFAULT: c_int = 2;

pub const CCMNI_MTU_MAX: c_int = 3000;
pub const NIC_NAPI_POLL_BUDGET: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t7xx_ccmni {
    pub index: u8,
    pub usage: core::sync::atomic::AtomicI32,
    pub dev: *mut net_device,
    pub ctlb: *mut t7xx_ccmni_ctrl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t7xx_ccmni_ctrl {
    pub t7xx_dev: *mut t7xx_pci_dev,
    pub hif_ctrl: *mut dpmaif_ctrl,
    pub ccmni_inst: [*mut t7xx_ccmni; NIC_DEV_MAX],
    pub callbacks: dpmaif_callbacks,
    pub nic_dev_num: c_uint,
    pub md_sta: c_uint,
    pub md_status_notify: t7xx_fsm_notifier,
    pub wwan_is_registered: bool,
    pub dummy_dev: *mut net_device,
    pub napi: [*mut napi_struct; RXQ_NUM],
    pub napi_usr_refcnt: core::sync::atomic::AtomicI32,
    pub is_napi_en: bool,
}

extern "C" {
    pub fn t7xx_ccmni_init(t7xx_dev: *mut t7xx_pci_dev) -> c_int;
}
extern "C" {
    pub fn t7xx_ccmni_exit(t7xx_dev: *mut t7xx_pci_dev);
}
