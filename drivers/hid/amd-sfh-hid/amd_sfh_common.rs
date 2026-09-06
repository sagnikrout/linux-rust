//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/amd-sfh-hid/amd_sfh_common.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// AMD MP2 common macros and structures
//
// Copyright (c) 2022, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Author: Basavaraj Natikar <Basavaraj.Natikar@amd.com>
//

pub const PCI_DEVICE_ID_AMD_MP2: c_uint = 0x15E4;
pub const PCI_DEVICE_ID_AMD_MP2_1_1: c_uint = 0x164A;

pub const SENSOR_ENABLED: c_int = 4;
pub const SENSOR_DISABLED: c_int = 5;
pub const AMD_SFH_IDLE_LOOP: c_int = 200;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_id {
    NO_OP,
    ENABLE_SENSOR,
    DISABLE_SENSOR,
    STOP_ALL_SENSORS = 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_mp2_version {
    MP2_VER_V2 = 1,
    MP2_VER_1_1 = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_mp2_sensor_info {
    pub sensor_idx: u8,
    pub period: u32,
    pub dma_address: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfh_dev_status {
    pub is_hpd_present: bool,
    pub is_hpd_enabled: bool,
    pub is_als_present: bool,
    pub is_sra_present: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_mp2_dev {
    pub pdev: *mut pci_dev,
    pub cl_data: *mut amdtp_cl_data,
    pub mmio: *mut void __iomem,
    pub vsbase: *mut void __iomem,
    pub sfh1_1_ops: *const amd_sfh1_1_ops,
    pub mp2_ops: *mut amd_mp2_ops,
    pub in_data: amd_input_data,
// mp2 active control status
    pub mp2_acs: u32,
    pub dev_en: sfh_dev_status,
    pub work: work_struct,
// mp2 to protect data
    pub lock: mutex,
    pub init_done: u8,
    pub rver: u8,
    pub mp2_ver: u8,
    pub tm_auxdev: *mut auxiliary_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_mp2_ops {
    pub info): *mut *mut *mut void (start)(struct amd_mp2_dev privdata, struct amd_mp2_sensor_info,
    pub sensor_idx): *mut *mut *mut void (stop)(struct amd_mp2_dev privdata, u16,
    pub privdata): *mut *mut void (stop_all)(struct amd_mp2_dev,
    pub sensor_sts): *mut *mut *mut int (response)(struct amd_mp2_dev mp2, u8 sid, u32,
    pub privdata): *mut *mut void (clear_intr)(struct amd_mp2_dev,
    pub privdata): *mut *mut int (init_intr)(struct amd_mp2_dev,
    pub privdata): *mut *mut int (discovery_status)(struct amd_mp2_dev,
    pub mp2): *mut *mut void (suspend)(struct amd_mp2_dev,
    pub mp2): *mut *mut void (resume)(struct amd_mp2_dev,
    pub privdata): *mut *mut void (remove)(void,
    pub rep_desc[]): *mut *mut int (get_rep_desc)(int sensor_idx, u8,
    pub descriptor_name): *mut *mut u32 (get_desc_sz)(int sensor_idx, int,
    pub feature_report): *mut *mut u8 (get_feat_rep)(int sensor_idx, int report_id, u8,
    pub in_data): *mut amd_input_data,
}

extern "C" {
    pub fn amd_sfh_work(work: *mut work_struct);
}
extern "C" {
    pub fn amd_sfh_work_buffer(work: *mut work_struct);
}
extern "C" {
    pub fn amd_sfh_clear_intr_v2(privdata: *mut amd_mp2_dev);
}
extern "C" {
    pub fn amd_sfh_irq_init_v2(privdata: *mut amd_mp2_dev) -> c_int;
}
extern "C" {
    pub fn amd_sfh_clear_intr(privdata: *mut amd_mp2_dev);
}
extern "C" {
    pub fn amd_sfh_irq_init(privdata: *mut amd_mp2_dev) -> c_int;
}
extern "C" {
    pub fn amd_sfh_op_idx_enabled(mp2: *mut amd_mp2_dev) -> bool;
}
extern "C" {
    pub fn sfh_set_emp2(mp2: *mut amd_mp2_dev);
}
extern "C" {
    pub fn sfh_deinit_emp2();
}
