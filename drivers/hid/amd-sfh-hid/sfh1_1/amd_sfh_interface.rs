//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/amd-sfh-hid/sfh1_1/amd_sfh_interface.h
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
// AMD MP2 1.1 communication interfaces
//
// Copyright (c) 2022, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Author: Basavaraj Natikar <Basavaraj.Natikar@amd.com>
//

pub const SENSOR_DATA_MEM_SIZE_DEFAULT: c_int = 256;
pub const TOTAL_STATIC_MEM_DEFAULT: c_int = 1024;
pub const OFFSET_SFH_INFO_BASE_DEFAULT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sensor_index {
    ACCEL_IDX,
    GYRO_IDX,
    MAG_IDX,
    SRA_IDX,
    ALS_IDX,
    HPD_IDX,
    MAX_IDX = 15,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfh_cmd_base {
    pub ul: u32,
    pub 4: u32 sensor_id :,
    pub 4: u32 cmd_id :,
    pub 8: u32 sub_cmd_id :,
    pub 12: u32 sub_cmd_value :,
    pub 3: u32 rsvd :,
    pub 1: u32 intr_disable :,
    pub cmd: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfh_cmd_response {
    pub resp: u32,
    pub 8: u32 response :,
    pub 4: u32 sensor_id :,
    pub 4: u32 cmd_id :,
    pub 6: u32 sub_cmd :,
    pub 10: u32 rsvd2 :,
    pub response: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfh_platform_info {
    pub pi: u32,
    pub 16: u32 cust_id :,
    pub 6: u32 plat_id :,
    pub 4: u32 interface_id :,
    pub 6: u32 rsvd :,
    pub pinfo: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfh_firmware_info {
    pub fw_ver: u32,
    pub 8: u32 minor_rev :,
    pub 8: u32 major_rev :,
    pub 8: u32 minor_ver :,
    pub 8: u32 major_ver :,
    pub fver: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfh_sensor_list {
    pub slist: u32,
    pub 16: u32 sensors :,
    pub 16: u32 rsvd :,
    pub sl: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfh_sensor_prop {
    pub sprop: u32,
    pub 16: u32 elist :,
    pub 16: u32 feat :,
    pub sf: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfh_base_info {
    pub sfh_base: [u32; 24],
    pub plat_info: sfh_platform_info,
    pub fw_info: sfh_firmware_info,
    pub s_list: sfh_sensor_list,
    pub rsvd: u32,
    pub s_prop: [sfh_sensor_prop; 16],
    pub sbase: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfh_common_data {
    pub timestamp: u64,
    pub intr_cnt: u32,
    pub 16: u32 featvalid :,
    pub 13: u32 rsvd :,
    pub 3: u32 sensor_state :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfh_float32 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfh_accel_data {
    pub commondata: sfh_common_data,
    pub acceldata: sfh_float32,
    pub accelstatus: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfh_gyro_data {
    pub commondata: sfh_common_data,
    pub gyrodata: sfh_float32,
    pub result: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfh_mag_data {
    pub commondata: sfh_common_data,
    pub magdata: sfh_float32,
    pub accuracy: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfh_als_data {
    pub commondata: sfh_common_data,
    pub lux: u32,
    pub light_color_temp: u32,
    pub chromaticity_x: u32,
    pub chromaticity_y: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpd_status {
    pub 16: u32 distance :,
    pub 8: u32 probablity :,
    pub 2: u32 presence :,
    pub 5: u32 rsvd :,
    pub 1: u32 state :,
    pub shpd: },
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sfh_op_mode {
    pub val: u32,
    pub 3: u32 mode :,
    pub 1: u32 lidstatus :,
    pub 10: u32 angle :,
    pub 2: u32 inbagstatedbg :,
    pub 2: u32 ontablestate :,
    pub 2: u32 inbagstate :,
    pub 2: u32 outbagstate :,
    pub 1: u32 inbagmlcstate :,
    pub 2: u32 powerstate :,
    pub 3: u32 data :,
    pub 4: u32 devicemode :,
    pub op_mode: },
}

extern "C" {
    pub fn sfh_interface_init(mp2: *mut amd_mp2_dev);
}
extern "C" {
    pub fn amd_sfh1_1_set_desc_ops(mp2_ops: *mut amd_mp2_ops);
}
extern "C" {
    pub fn amd_sfh_float_to_int(flt32_val: u32) -> c_int;
}
