//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/amd-sfh-hid/amd_sfh_pcie.h
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
// AMD MP2 PCIe communication driver
// Copyright 2020-2021 Advanced Micro Devices, Inc.
// Authors: Shyam Sundar S K <Shyam-sundar.S-k@amd.com>
// Sandeep Singh <Sandeep.singh@amd.com>
// Basavaraj Natikar <Basavaraj.Natikar@amd.com>
//

// MP2 C2P Message Registers
pub const AMD_C2P_MSG0: c_uint = 0x10500;
pub const AMD_C2P_MSG1: c_uint = 0x10504;
pub const AMD_C2P_MSG2: c_uint = 0x10508;
// MP2 P2C Message Registers
pub const AMD_P2C_MSG3: c_uint = 0x1068C /* Supported Sensors info */;
pub const V2_STATUS: c_uint = 0x2;
pub const HPD_IDX: c_int = 16;
pub const ACS_IDX: c_int = 22;

pub const SENSOR_DISCOVERY_STATUS_SHIFT: c_int = 3;
// SFH Command register
#[repr(C)]
#[derive(Copy, Clone)]
pub union sfh_cmd_base {
    pub ul: u32,
    pub 8: u32 cmd_id :,
    pub 8: u32 sensor_id :,
    pub 16: u32 period :,
    pub s: },
    pub 4: u32 cmd_id :,
    pub 1: u32 intr_disable :,
    pub 3: u32 rsvd1 :,
    pub 7: u32 length :,
    pub 1: u32 mem_type :,
    pub 8: u32 sensor_id :,
    pub 8: u32 period :,
    pub cmd_v2: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cmd_response {
    pub resp: u32,
    pub 2: u32 status :,
    pub 1: u32 out_in_c2p :,
    pub 1: u32 rsvd1 :,
    pub 4: u32 response :,
    pub 8: u32 sub_cmd :,
    pub 6: u32 sensor_id :,
    pub 10: u32 rsvd2 :,
    pub response_v2: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sfh_cmd_param {
    pub ul: u32,
    pub 2: u32 buf_layout :,
    pub 6: u32 buf_length :,
    pub 24: u32 rsvd :,
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfh_cmd_reg {
    pub cmd_base: sfh_cmd_base,
    pub cmd_param: sfh_cmd_param,
    pub phys_addr: phys_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sensor_idx {
    accel_idx = 0,
    gyro_idx = 1,
    mag_idx = 2,
    op_idx = 15,
    als_idx = 19
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mem_use_type {
    USE_DRAM,
    USE_C2P_REG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpd_status {
    pub 16: u32 object_distance :,
    pub 8: u32 probablity :,
    pub 4: u32 human_presence_actual :,
    pub 4: u32 human_presence_report :,
    pub shpd: },
    pub val: u32,
}

extern "C" {
    pub fn amd_mp2_get_sensor_num(privdata: *mut amd_mp2_dev, sensor_id: *mut u8) -> c_int;
}
extern "C" {
    pub fn amd_sfh_hid_client_init(privdata: *mut amd_mp2_dev) -> c_int;
}
extern "C" {
    pub fn amd_sfh_hid_client_deinit(privdata: *mut amd_mp2_dev) -> c_int;
}
extern "C" {
    pub fn amd_sfh_set_desc_ops(mp2_ops: *mut amd_mp2_ops);
}
