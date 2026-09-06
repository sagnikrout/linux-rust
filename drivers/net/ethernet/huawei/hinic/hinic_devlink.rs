//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_devlink.h
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
// Huawei HiNIC PCI Express Linux driver
// Copyright(c) 2017 Huawei Technologies Co., Ltd
//

pub const MAX_FW_TYPE_NUM: c_int = 30;
pub const HINIC_MAGIC_NUM: c_uint = 0x18221100;
pub const UPDATEFW_IMAGE_HEAD_SIZE: c_int = 1024;
pub const FW_UPDATE_COLD: c_int = 0;
pub const FW_UPDATE_HOT: c_int = 1;
pub const UP_TYPE_A: c_uint = 0x0;
pub const UP_TYPE_B: c_uint = 0x1;
pub const MAX_FW_FRAGMENT_LEN: c_int = 1536;
pub const HINIC_FW_DISMATCH_ERROR: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_fw_type {
    UP_FW_UPDATE_UP_TEXT_A = 0x0,
    UP_FW_UPDATE_UP_DATA_A,
    UP_FW_UPDATE_UP_TEXT_B,
    UP_FW_UPDATE_UP_DATA_B,
    UP_FW_UPDATE_UP_DICT,

    UP_FW_UPDATE_HLINK_ONE = 0x5,
    UP_FW_UPDATE_HLINK_TWO,
    UP_FW_UPDATE_HLINK_THR,
    UP_FW_UPDATE_PHY,
    UP_FW_UPDATE_TILE_TEXT,

    UP_FW_UPDATE_TILE_DATA = 0xa,
    UP_FW_UPDATE_TILE_DICT,
    UP_FW_UPDATE_PPE_STATE,
    UP_FW_UPDATE_PPE_BRANCH,
    UP_FW_UPDATE_PPE_EXTACT,

    UP_FW_UPDATE_CLP_LEGACY = 0xf,
    UP_FW_UPDATE_PXE_LEGACY,
    UP_FW_UPDATE_ISCSI_LEGACY,
    UP_FW_UPDATE_CLP_EFI,
    UP_FW_UPDATE_PXE_EFI,

    UP_FW_UPDATE_ISCSI_EFI = 0x14,
    UP_FW_UPDATE_CFG,
    UP_FW_UPDATE_BOOT,
    UP_FW_UPDATE_VPD,
    FILE_TYPE_TOTAL_NUM
}

pub const UP_FW_UPDATE_UP_TEXT: c_uint = 0x0;
pub const UP_FW_UPDATE_UP_DATA: c_uint = 0x1;
pub const UP_FW_UPDATE_VPD_B: c_uint = 0x15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_section_info_st {
    pub fw_section_len: u32,
    pub fw_section_offset: u32,
    pub fw_section_version: u32,
    pub fw_section_type: u32,
    pub fw_section_crc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_image_st {
    pub fw_version: u32,
    pub fw_len: u32,
    pub fw_magic: u32,
    pub fw_section_cnt:16: u32,
    pub resd:16: u32,
    pub fw_info: },
    pub fw_section_info: [fw_section_info_st; MAX_FW_TYPE_NUM],
    pub device_id: u32,
    pub res: [u32; 101],
    pub bin_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_image_st {
    pub image_section_info: [fw_section_info_st; MAX_FW_TYPE_NUM],
    pub up_total_len: u32,
    pub fw_version: u32,
    pub image_info: },
    pub section_type_num: u32,
    pub device_id: u32,
}

extern "C" {
    pub fn hinic_devlink_free(devlink: *mut devlink);
}
extern "C" {
    pub fn hinic_devlink_register(priv: *mut hinic_devlink_priv);
}
extern "C" {
    pub fn hinic_devlink_unregister(priv: *mut hinic_devlink_priv);
}
extern "C" {
    pub fn hinic_health_reporters_create(priv: *mut hinic_devlink_priv) -> c_int;
}
extern "C" {
    pub fn hinic_health_reporters_destroy(priv: *mut hinic_devlink_priv);
}
