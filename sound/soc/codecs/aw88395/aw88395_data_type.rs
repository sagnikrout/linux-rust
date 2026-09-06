//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/aw88395/aw88395_data_type.h
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
// aw883_data_type.h --  The data type of the AW88395 chip
//
// Copyright (c) 2022-2023 AWINIC Technology CO., LTD
//
// Author: Bruce zhao <zhaolei@awinic.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aw_cfg_hdr_version {
    AW88395_CFG_HDR_VER	= 0x00000001,
    AW88395_CFG_HDR_VER_V1	= 0x01000000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aw_cfg_dde_type {
    AW88395_DEV_NONE_TYPE_ID	= 0xFFFFFFFF,
    AW88395_DEV_TYPE_ID		= 0x00000000,
    AW88395_SKT_TYPE_ID		= 0x00000001,
    AW88395_DEV_DEFAULT_TYPE_ID	= 0x00000002,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aw_sec_type {
    ACF_SEC_TYPE_REG = 0,
    ACF_SEC_TYPE_DSP,
    ACF_SEC_TYPE_DSP_CFG,
    ACF_SEC_TYPE_DSP_FW,
    ACF_SEC_TYPE_HDR_REG,
    ACF_SEC_TYPE_HDR_DSP_CFG,
    ACF_SEC_TYPE_HDR_DSP_FW,
    ACF_SEC_TYPE_MULTIPLE_BIN,
    ACF_SEC_TYPE_SKT_PROJECT,
    ACF_SEC_TYPE_DSP_PROJECT,
    ACF_SEC_TYPE_MONITOR,
    ACF_SEC_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum profile_data_type {
    AW88395_DATA_TYPE_REG = 0,
    AW88395_DATA_TYPE_DSP_CFG,
    AW88395_DATA_TYPE_DSP_FW,
    AW88395_DATA_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aw_prof_type {
    AW88395_PROFILE_MUSIC = 0,
    AW88395_PROFILE_VOICE,
    AW88395_PROFILE_VOIP,
    AW88395_PROFILE_RINGTONE,
    AW88395_PROFILE_RINGTONE_HS,
    AW88395_PROFILE_LOWPOWER,
    AW88395_PROFILE_BYPASS,
    AW88395_PROFILE_MMI,
    AW88395_PROFILE_FM,
    AW88395_PROFILE_NOTIFICATION,
    AW88395_PROFILE_RECEIVER,
    AW88395_PROFILE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aw_profile_status {
    AW88395_PROFILE_WAIT = 0,
    AW88395_PROFILE_OK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_cfg_hdr {
    pub id: u32,
    pub project: [c_char; PROJECT_NAME_MAX],
    pub custom: [c_char; CUSTOMER_NAME_MAX],
    pub version: [c_char; CFG_VERSION_MAX],
    pub author_id: u32,
    pub ddt_size: u32,
    pub ddt_num: u32,
    pub hdr_offset: u32,
    pub hdr_version: u32,
    pub reserved: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_cfg_dde {
    pub type: u32,
    pub dev_name: [c_char; DEV_NAME_MAX],
    pub dev_index: u16,
    pub dev_bus: u16,
    pub dev_addr: u16,
    pub dev_profile: u16,
    pub data_type: u32,
    pub data_size: u32,
    pub data_offset: u32,
    pub data_crc: u32,
    pub reserved: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_cfg_dde_v1 {
    pub type: u32,
    pub dev_name: [c_char; DEV_NAME_MAX],
    pub dev_index: u16,
    pub dev_bus: u16,
    pub dev_addr: u16,
    pub dev_profile: u16,
    pub data_type: u32,
    pub data_size: u32,
    pub data_offset: u32,
    pub data_crc: u32,
    pub dev_profile_str: [c_char; PROFILE_STR_MAX],
    pub chip_id: u32,
    pub reserved: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_sec_data_desc {
    pub len: u32,
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_prof_desc {
    pub id: u32,
    pub prof_st: u32,
    pub prf_str: *mut c_char,
    pub fw_ver: u32,
    pub sec_desc: [aw_sec_data_desc; AW88395_DATA_TYPE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_all_prof_info {
    pub prof_desc: [aw_prof_desc; AW88395_PROFILE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_prof_info {
    pub count: c_int,
    pub prof_type: c_int,
    pub prof_name_list: *mut c_char,
    pub prof_desc: *mut aw_prof_desc,
}
