//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/fcoe_sysfs.h
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
// Copyright (c) 2011-2012 Intel Corporation.  All rights reserved.
//
// Maintained at www.Open-FCoE.org
//

// Macro flag: #define FCOE_SYSFS

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_sysfs_function_template {
    pub ): *mut *mut void (get_fcoe_ctlr_link_fail)(struct fcoe_ctlr_device,
    pub ): *mut *mut void (get_fcoe_ctlr_vlink_fail)(struct fcoe_ctlr_device,
    pub ): *mut *mut void (get_fcoe_ctlr_miss_fka)(struct fcoe_ctlr_device,
    pub ): *mut *mut void (get_fcoe_ctlr_symb_err)(struct fcoe_ctlr_device,
    pub ): *mut *mut void (get_fcoe_ctlr_err_block)(struct fcoe_ctlr_device,
    pub ): *mut *mut void (get_fcoe_ctlr_fcs_error)(struct fcoe_ctlr_device,
    pub ): *mut *mut void (set_fcoe_ctlr_mode)(struct fcoe_ctlr_device,
    pub ): *mut *mut int (set_fcoe_ctlr_enabled)(struct fcoe_ctlr_device,
    pub ): *mut *mut void (get_fcoe_fcf_selected)(struct fcoe_fcf_device,
    pub ): *mut *mut void (get_fcoe_fcf_vlan_id)(struct fcoe_fcf_device,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fip_conn_type {
    FIP_CONN_TYPE_UNKNOWN,
    FIP_CONN_TYPE_FABRIC,
    FIP_CONN_TYPE_VN2VN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctlr_enabled_state {
    FCOE_CTLR_ENABLED,
    FCOE_CTLR_DISABLED,
    FCOE_CTLR_UNUSED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_ctlr_device {
    pub id: u32,
    pub dev: device,
    pub f: *mut fcoe_sysfs_function_template,
    pub fcfs: list_head,
    pub work_q: *mut workqueue_struct,
    pub devloss_work_q: *mut workqueue_struct,
    pub lock: mutex,
    pub fcf_dev_loss_tmo: c_int,
    pub mode: fip_conn_type,
    pub enabled: ctlr_enabled_state,
// expected in host order for displaying
    pub lesb: fcoe_fc_els_lesb,
}

// fcf states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcf_state {
    FCOE_FCF_STATE_UNKNOWN,
    FCOE_FCF_STATE_DISCONNECTED,
    FCOE_FCF_STATE_CONNECTED,
    FCOE_FCF_STATE_DELETED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_fcf_device {
    pub id: u32,
    pub dev: device,
    pub peers: list_head,
    pub delete_work: work_struct,
    pub dev_loss_work: delayed_work,
    pub dev_loss_tmo: u32,
    pub priv: *mut c_void,
    pub state: fcf_state,
    pub fabric_name: u64,
    pub switch_name: u64,
    pub fc_map: u32,
    pub vfid: u16,
    pub mac: [u8; ETH_ALEN],
    pub priority: u8,
    pub fka_period: u32,
    pub selected: u8,
    pub vlan_id: u16,
}

// parentage should never be missing

extern "C" {
    pub fn fcoe_ctlr_device_delete(: *mut fcoe_ctlr_device);
}
extern "C" {
    pub fn fcoe_fcf_device_delete(: *mut fcoe_fcf_device);
}
extern "C" {
    pub fn fcoe_sysfs_setup() -> int __init;
}
extern "C" {
    pub fn fcoe_sysfs_teardown() -> void __exit;
}
