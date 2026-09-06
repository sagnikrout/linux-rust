//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns3/hns3pf/hclge_debugfs.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2018-2019 Hisilicon Limited.

pub const HCLGE_DBG_MNG_TBL_MAX: c_int = 64;

pub const HCLGE_DBG_MNG_VLAN_TAG: c_uint = 0x0FFF;
pub const HCLGE_DBG_MNG_PF_ID: c_uint = 0x0007;
pub const HCLGE_DBG_MNG_VF_ID: c_uint = 0x00FF;
// Get DFX BD number offset
pub const HCLGE_DBG_DFX_BIOS_OFFSET: c_int = 1;
pub const HCLGE_DBG_DFX_SSU_0_OFFSET: c_int = 2;
pub const HCLGE_DBG_DFX_SSU_1_OFFSET: c_int = 3;
pub const HCLGE_DBG_DFX_IGU_OFFSET: c_int = 4;
pub const HCLGE_DBG_DFX_RPU_0_OFFSET: c_int = 5;
pub const HCLGE_DBG_DFX_RPU_1_OFFSET: c_int = 6;
pub const HCLGE_DBG_DFX_NCSI_OFFSET: c_int = 7;
pub const HCLGE_DBG_DFX_RTC_OFFSET: c_int = 8;
pub const HCLGE_DBG_DFX_PPP_OFFSET: c_int = 9;
pub const HCLGE_DBG_DFX_RCB_OFFSET: c_int = 10;
pub const HCLGE_DBG_DFX_TQP_OFFSET: c_int = 11;
pub const HCLGE_DBG_DFX_SSU_2_OFFSET: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_qos_pri_map_cmd {
    pub 4: pri1_tc :,
    pub 4: pri3_tc :,
    pub 4: pri5_tc :,
    pub 4: pri7_tc :,
    pub 4: rev :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_dbg_bitmap_cmd {
    pub bitmap: u8,
    pub 1: bit7 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_dbg_reg_common_msg {
    pub msg_num: c_int,
    pub offset: c_int,
    pub cmd: hclge_opcode_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_dbg_tcam_msg {
    pub stage: u8,
    pub loc: u32,
}

pub const HCLGE_DBG_MAX_DFX_MSG_LEN: c_int = 60;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_dbg_dfx_message {
    pub flag: c_int,
    pub message: [c_char; HCLGE_DBG_MAX_DFX_MSG_LEN],
}

pub const HCLGE_DBG_MAC_REG_TYPE_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_dbg_reg_type_info {
    pub cmd: hnae3_dbg_cmd,
    pub dfx_msg: *const hclge_dbg_dfx_message,
    pub reg_msg: hclge_dbg_reg_common_msg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_dbg_func {
    pub cmd: hnae3_dbg_cmd,
    pub len): *mut *mut *mut *mut int (dbg_dump)(struct hclge_dev hdev, char buf, int,
    pub len): *mut *mut char buf, int,
    pub dbg_read_func: read_func,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_dbg_status_dfx_info {
    pub offset: u32,
    pub message: [c_char; HCLGE_DBG_MAX_DFX_MSG_LEN],
}

pub const HCLGE_DBG_INFO_LEN: c_int = 256;
pub const HCLGE_DBG_VLAN_FLTR_INFO_LEN: c_int = 256;
pub const HCLGE_DBG_VLAN_OFFLOAD_INFO_LEN: c_int = 512;
pub const HCLGE_DBG_ID_LEN: c_int = 16;
pub const HCLGE_DBG_ITEM_NAME_LEN: c_int = 32;
pub const HCLGE_DBG_DATA_STR_LEN: c_int = 32;
pub const HCLGE_DBG_TM_INFO_LEN: c_int = 256;
pub const HCLGE_BILLION_NANO_SECONDS: c_int = 1000000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_dbg_item {
    pub name: [c_char; HCLGE_DBG_ITEM_NAME_LEN],
    pub /: *mut *mut u16 interval; / blank numbers after the item,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_dbg_vlan_cfg {
    pub pvid: u16,
    pub accept_tag1: u8,
    pub accept_tag2: u8,
    pub accept_untag1: u8,
    pub accept_untag2: u8,
    pub insert_tag1: u8,
    pub insert_tag2: u8,
    pub shift_tag: u8,
    pub strip_tag1: u8,
    pub strip_tag2: u8,
    pub drop_tag1: u8,
    pub drop_tag2: u8,
    pub pri_only1: u8,
    pub pri_only2: u8,
}
