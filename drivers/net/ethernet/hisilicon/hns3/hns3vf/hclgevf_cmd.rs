//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns3/hns3vf/hclgevf_cmd.h
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
// Copyright (c) 2016-2017 Hisilicon Limited.

pub const HCLGEVF_CMDQ_RX_INVLD_B: c_int = 0;
pub const HCLGEVF_CMDQ_RX_OUTVLD_B: c_int = 1;
pub const HCLGEVF_SYNC_RX_RING_HEAD_EN_B: c_int = 4;
pub const HCLGEVF_TQP_REG_OFFSET: c_uint = 0x80000;
pub const HCLGEVF_TQP_REG_SIZE: c_uint = 0x200;
pub const HCLGEVF_TQP_MAX_SIZE_DEV_V2: c_int = 1024;
pub const HCLGEVF_TQP_EXT_REG_OFFSET: c_uint = 0x100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_tqp_map {
    pub /: *mut *mut __le16 tqp_id; / Absolute tqp id for in this pf,
    pub /: *mut *mut u8 tqp_vf; / VF id,
pub const HCLGEVF_TQP_MAP_TYPE_PF: c_int = 0;
pub const HCLGEVF_TQP_MAP_TYPE_VF: c_int = 1;
pub const HCLGEVF_TQP_MAP_TYPE_B: c_int = 0;
pub const HCLGEVF_TQP_MAP_EN_B: c_int = 1;
    pub /: *mut *mut u8 tqp_flag; / Indicate it's pf or vf tqp,
    pub /: *mut *mut __le16 tqp_vid; / Virtual id in this pf/vf,
    pub rsv: [u8; 18],
}

pub const HCLGEVF_VECTOR_ELEMENTS_PER_CMD: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclgevf_int_type {
    HCLGEVF_INT_TX = 0,
    HCLGEVF_INT_RX,
    HCLGEVF_INT_EVENT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_ctrl_vector_chain {
    pub int_vector_id: u8,
    pub int_cause_num: u8,
pub const HCLGEVF_INT_TYPE_S: c_int = 0;
pub const HCLGEVF_INT_TYPE_M: c_uint = 0x3;
pub const HCLGEVF_TQP_ID_S: c_int = 2;
    pub tqp_type_and_id: [__le16; HCLGEVF_VECTOR_ELEMENTS_PER_CMD],
    pub vfid: u8,
    pub resv: u8,
}

pub const HCLGEVF_MSIX_OFT_ROCEE_S: c_int = 0;

pub const HCLGEVF_VEC_NUM_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_query_res_cmd {
    pub tqp_num: __le16,
    pub reserved: __le16,
    pub msixcap_localid_ba_nic: __le16,
    pub msixcap_localid_ba_rocee: __le16,
    pub vf_intr_vector_number: __le16,
    pub rsv: [__le16; 7],
}

pub const HCLGEVF_GRO_EN_B: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_cfg_gro_status_cmd {
    pub gro_en: u8,
    pub rsv: [u8; 23],
}

pub const HCLGEVF_LINK_STS_B: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_link_status_cmd {
    pub status: u8,
    pub rsv: [u8; 23],
}

pub const HCLGEVF_RING_ID_MASK: c_uint = 0x3ff;
pub const HCLGEVF_TQP_ENABLE_B: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_cfg_com_tqp_queue_cmd {
    pub tqp_id: __le16,
    pub stream_id: __le16,
    pub enable: u8,
    pub rsv: [u8; 19],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_cfg_tx_queue_pointer_cmd {
    pub tqp_id: __le16,
    pub tx_tail: __le16,
    pub tx_head: __le16,
    pub fbd_num: __le16,
    pub ring_offset: __le16,
    pub rsv: [u8; 14],
}

// this bit indicates that the driver is ready for hardware reset
pub const HCLGEVF_NIC_SW_RST_RDY_B: c_int = 16;

pub const HCLGEVF_NIC_CMQ_DESC_NUM: c_int = 1024;
pub const HCLGEVF_NIC_CMQ_DESC_NUM_S: c_int = 3;
pub const HCLGEVF_QUERY_DEV_SPECS_BD_NUM: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_dev_specs_0_cmd {
    pub rsv0: __le32,
    pub mac_entry_num: __le32,
    pub mng_entry_num: __le32,
    pub rss_ind_tbl_size: __le16,
    pub rss_key_size: __le16,
    pub int_ql_max: __le16,
    pub max_non_tso_bd_num: u8,
    pub rsv1: [u8; 5],
}

pub const HCLGEVF_DEF_MAX_INT_GL: c_uint = 0x1FE0U;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_dev_specs_1_cmd {
    pub max_frm_size: __le16,
    pub rsv0: __le16,
    pub max_int_gl: __le16,
    pub rsv1: [u8; 18],
}

extern "C" {
    pub fn hclgevf_cmd_send(hw: *mut hclgevf_hw, desc: *mut hclge_desc, num: c_int) -> c_int;
}
extern "C" {
    pub fn hclgevf_arq_init(hdev: *mut hclgevf_dev);
}
