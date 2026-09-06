//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_hwdev.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_event_service_type {
    HINIC3_EVENT_SRV_COMM = 0,
    HINIC3_EVENT_SRV_NIC  = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_comm_event_type {
    HINIC3_COMM_EVENT_PCIE_LINK_DOWN = 0,
    HINIC3_COMM_EVENT_HEART_LOST = 1,
    HINIC3_COMM_EVENT_FAULT = 2,
    HINIC3_COMM_EVENT_SRIOV_STATE_CHANGE = 3,
    HINIC3_COMM_EVENT_CARD_REMOVE = 4,
    HINIC3_COMM_EVENT_MGMT_WATCHDOG = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_fault_err_level {
    HINIC3_FAULT_LEVEL_SERIOUS_FLR = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_fault_source_type {
    HINIC3_FAULT_SRC_HW_PHY_FAULT = 9,
    HINIC3_FAULT_SRC_TX_TIMEOUT   = 22,
}

// driver-specific data of pci_dev
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_pcidev {
    pub pdev: *mut pci_dev,
    pub hwdev: *mut hinic3_hwdev,
// Auxiliary devices
    pub hadev: [*mut hinic3_adev; HINIC3_SERVICE_T_MAX],
    pub cfg_reg_base: *mut void __iomem,
    pub intr_reg_base: *mut void __iomem,
    pub mgmt_reg_base: *mut void __iomem,
    pub db_base: *mut void __iomem,
    pub db_dwqe_len: u64,
    pub db_base_phy: u64,
// lock for attach/detach uld
    pub pdev_mutex: mutex,
    pub state: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_hwdev {
    pub adapter: *mut hinic3_pcidev,
    pub pdev: *mut pci_dev,
    pub dev: *mut device,
    pub dev_id: c_int,
    pub hwif: *mut hinic3_hwif,
    pub cfg_mgmt: *mut hinic3_cfg_mgmt_info,
    pub aeqs: *mut hinic3_aeqs,
    pub ceqs: *mut hinic3_ceqs,
    pub mbox: *mut hinic3_mbox,
    pub cmdqs: *mut hinic3_cmdqs,
    pub sync_time_task: delayed_work,
    pub workq: *mut workqueue_struct,
    pub pf_to_mgmt: *mut hinic3_msg_pf_to_mgmt,
// protect channel init and uninit
    pub channel_lock: spinlock_t,
    pub features: [u64; COMM_MAX_FEATURE_QWORD],
    pub wq_page_size: u32,
    pub max_cmdq: u8,
    pub func_state: c_ulong,
    pub chip_present_flag: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_event_info {
// enum hinic3_event_service_type
    pub service: u16,
    pub type: u16,
    pub event_data: [u8; 104],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_adev {
    pub adev: auxiliary_device,
    pub hwdev: *mut hinic3_hwdev,
    pub svc_type: hinic3_service_type,
    pub event): *mut hinic3_event_info,
}

extern "C" {
    pub fn hinic3_init_hwdev(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn hinic3_free_hwdev(hwdev: *mut hinic3_hwdev);
}
extern "C" {
    pub fn hinic3_set_api_stop(hwdev: *mut hinic3_hwdev);
}
