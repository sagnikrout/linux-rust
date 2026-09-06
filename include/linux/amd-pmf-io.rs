//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/amd-pmf-io.h
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
//
// AMD Platform Management Framework Interface
//
// Copyright (c) 2023, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Authors: Shyam Sundar S K <Shyam-sundar.S-k@amd.com>
// Basavaraj Natikar <Basavaraj.Natikar@amd.com>
//

//
// enum sfh_message_type - Query the SFH message type
// @MT_HPD: Message ID to know the Human presence info from MP2 FW
// @MT_ALS: Message ID to know the Ambient light info from MP2 FW
// @MT_SRA: Message ID to know the SRA data from MP2 FW
// @MT_OP_MODE: Message ID to know the operating-mode (tablet/laptop) info
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sfh_message_type {
    MT_HPD,
    MT_ALS,
    MT_SRA,
    MT_OP_MODE,
}

//
// enum sfh_hpd_info - Query the Human presence information
// @SFH_NOT_DETECTED: Check the HPD connection information from MP2 FW
// @SFH_USER_PRESENT: Check if the user is present from HPD sensor
// @SFH_USER_AWAY: Check if the user is away from HPD sensor
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sfh_hpd_info {
    SFH_NOT_DETECTED,
    SFH_USER_PRESENT,
    SFH_USER_AWAY,
}

//
// struct amd_sfh_info - get HPD sensor info from MP2 FW
// @ambient_light: Populates the ambient light information
// @user_present: Populates the user presence information
// @platform_type: Operating modes (clamshell, flat, tent, etc.)
// @laptop_placement: Device states (ontable, onlap, outbag)
// @op_mode: Operating-mode field (see enum sfh_dev_mode); used for tablet detection
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_sfh_info {
    pub ambient_light: u32,
    pub user_present: u8,
    pub platform_type: u32,
    pub laptop_placement: u32,
    pub op_mode: u32,
}

//
// enum sfh_dev_mode - SFH operating-mode field (sfh_op_mode.mode, bits 0-2)
// @SFH_MODE_LAPTOP: Device is in laptop/clamshell posture
// @SFH_MODE_TABLET: Device is in tablet posture
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sfh_dev_mode {
    SFH_MODE_LAPTOP	= 1,
    SFH_MODE_TABLET	= 3,
}

//
// struct amd_pmf_npu_metrics: Get NPU metrics data from PMF driver
// @npuclk_freq: NPU clock frequency [MHz]
// @npu_busy: NPU busy % [0-100]
// @npu_power: NPU power [mW]
// @mpnpuclk_freq: MPNPU [MHz]
// @npu_reads: NPU read bandwidth [MB/sec]
// @npu_writes: NPU write bandwidth [MB/sec]
// @npu_temp: NPU temperature [C]
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmf_npu_metrics {
    pub npuclk_freq: u16,
    pub npu_busy: [u16; 8],
    pub npu_power: u16,
    pub mpnpuclk_freq: u16,
    pub npu_reads: u16,
    pub npu_writes: u16,
    pub npu_temp: u16,
}

extern "C" {
    pub fn amd_get_sfh_info(sfh_info: *mut amd_sfh_info, op: sfh_message_type) -> c_int;
}
// AMD PMF and NPU interface
extern "C" {
    pub fn amd_pmf_get_npu_data(info: *mut amd_pmf_npu_metrics) -> c_int;
}
