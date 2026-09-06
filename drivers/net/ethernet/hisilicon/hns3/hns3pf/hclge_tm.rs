//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns3/hns3pf/hclge_tm.h
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

// MAC Pause

pub const HCLGE_DEFAULT_PAUSE_TRANS_GAP: c_uint = 0x7F;
pub const HCLGE_DEFAULT_PAUSE_TRANS_TIME: c_uint = 0xFFFF;
// SP or DWRR

pub const HCLGE_TM_TX_SCHD_SP_MSK: c_uint = 0xFE;
pub const HCLGE_ETHER_MAX_RATE: c_int = 100000;
pub const HCLGE_TM_PF_MAX_PRI_NUM: c_int = 8;
pub const HCLGE_TM_PF_MAX_QSET_NUM: c_int = 8;
pub const HCLGE_DSCP_MAP_TC_BD_NUM: c_int = 2;

pub const HCLGE_TM_FLUSH_TIME_MS: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_pg_to_pri_link_cmd {
    pub pg_id: u8,
    pub rsvd1: [u8; 3],
    pub pri_bit_map: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_qs_to_pri_link_cmd {
    pub qs_id: __le16,
    pub rsvd: __le16,
    pub priority: u8,

    pub link_vld: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_nq_to_qs_link_cmd {
    pub nq_id: __le16,
    pub rsvd: __le16,

pub const HCLGE_TM_QS_ID_L_S: c_int = 0;

pub const HCLGE_TM_QS_ID_H_S: c_int = 10;
pub const HCLGE_TM_QS_ID_H_EXT_S: c_int = 11;

    pub qset_id: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_tqp_tx_queue_tc_cmd {
    pub queue_id: __le16,
    pub rsvd: __le16,
    pub tc_id: u8,
    pub rev: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_pg_weight_cmd {
    pub pg_id: u8,
    pub dwrr: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_priority_weight_cmd {
    pub pri_id: u8,
    pub dwrr: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_pri_sch_mode_cfg_cmd {
    pub pri_id: u8,
    pub rsvd: [u8; 3],
    pub sch_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_qs_sch_mode_cfg_cmd {
    pub qs_id: __le16,
    pub rsvd: [u8; 2],
    pub sch_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_qs_weight_cmd {
    pub qs_id: __le16,
    pub dwrr: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_ets_tc_weight_cmd {
    pub tc_weight: [u8; HNAE3_MAX_TC],
    pub weight_offset: u8,
    pub rsvd: [u8; 15],
}

pub const HCLGE_TM_SHAP_IR_B_LSH: c_int = 0;

pub const HCLGE_TM_SHAP_IR_U_LSH: c_int = 8;

pub const HCLGE_TM_SHAP_IR_S_LSH: c_int = 12;

pub const HCLGE_TM_SHAP_BS_B_LSH: c_int = 16;

pub const HCLGE_TM_SHAP_BS_S_LSH: c_int = 21;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_shap_bucket {
    HCLGE_TM_SHAP_C_BUCKET = 0,
    HCLGE_TM_SHAP_P_BUCKET,
}

// set bit HCLGE_TM_RATE_VLD to 1 means use 'rate' to config shaping
pub const HCLGE_TM_RATE_VLD: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_pri_shapping_cmd {
    pub pri_id: u8,
    pub rsvd: [u8; 3],
    pub pri_shapping_para: __le32,
    pub flag: u8,
    pub rsvd1: [u8; 3],
    pub pri_rate: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_pg_shapping_cmd {
    pub pg_id: u8,
    pub rsvd: [u8; 3],
    pub pg_shapping_para: __le32,
    pub flag: u8,
    pub rsvd1: [u8; 3],
    pub pg_rate: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_qs_shapping_cmd {
    pub qs_id: __le16,
    pub rsvd: [u8; 2],
    pub qs_shapping_para: __le32,
    pub flag: u8,
    pub rsvd1: [u8; 3],
    pub qs_rate: __le32,
}

pub const HCLGE_BP_GRP_NUM: c_int = 32;
pub const HCLGE_BP_SUB_GRP_ID_S: c_int = 0;

pub const HCLGE_BP_GRP_ID_S: c_int = 5;

pub const HCLGE_BP_EXT_GRP_NUM: c_int = 40;
pub const HCLGE_BP_EXT_GRP_ID_S: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_bp_to_qs_map_cmd {
    pub tc_id: u8,
    pub rsvd: [u8; 2],
    pub qs_group_id: u8,
    pub qs_bit_map: __le32,
    pub rsvd1: u32,
}

pub const HCLGE_PFC_DISABLE: c_int = 0;
pub const HCLGE_PFC_TX_RX_DISABLE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_pfc_en_cmd {
    pub tx_rx_en_bitmap: u8,
    pub pri_en_bitmap: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_cfg_pause_param_cmd {
    pub mac_addr: [u8; ETH_ALEN],
    pub pause_trans_gap: u8,
    pub rsvd: u8,
    pub pause_trans_time: __le16,
    pub rsvd1: [u8; 6],
// extra mac address to do double check for pause frame
    pub mac_addr_extra: [u8; ETH_ALEN],
    pub rsvd2: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_pfc_stats_cmd {
    pub pkt_num: [__le64; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_port_shapping_cmd {
    pub port_shapping_para: __le32,
    pub flag: u8,
    pub rsvd: [u8; 3],
    pub port_rate: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_shaper_ir_para {
    pub /: *mut *mut u8 ir_b; / IR_B parameter of IR shaper,
    pub /: *mut *mut u8 ir_u; / IR_U parameter of IR shaper,
    pub /: *mut *mut u8 ir_s; / IR_S parameter of IR shaper,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_tm_nodes_cmd {
    pub pg_base_id: u8,
    pub pri_base_id: u8,
    pub qset_base_id: __le16,
    pub queue_base_id: __le16,
    pub pg_num: u8,
    pub pri_num: u8,
    pub qset_num: __le16,
    pub queue_num: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_tm_shaper_para {
    pub rate: u32,
    pub ir_b: u8,
    pub ir_u: u8,
    pub ir_s: u8,
    pub bs_b: u8,
    pub bs_s: u8,
    pub flag: u8,
}

extern "C" {
    pub fn hclge_tm_schd_init(hdev: *mut hclge_dev) -> c_int;
}
extern "C" {
    pub fn hclge_tm_vport_map_update(hdev: *mut hclge_dev) -> c_int;
}
extern "C" {
    pub fn hclge_pause_setup_hw(hdev: *mut hclge_dev, init: bool) -> c_int;
}
extern "C" {
    pub fn hclge_tm_schd_setup_hw(hdev: *mut hclge_dev) -> c_int;
}
extern "C" {
    pub fn hclge_tm_prio_tc_info_update(hdev: *mut hclge_dev, prio_tc: *mut u8);
}
extern "C" {
    pub fn hclge_tm_schd_info_update(hdev: *mut hclge_dev, num_tc: u8);
}
extern "C" {
    pub fn hclge_tm_pfc_info_update(hdev: *mut hclge_dev);
}
extern "C" {
    pub fn hclge_tm_dwrr_cfg(hdev: *mut hclge_dev) -> c_int;
}
extern "C" {
    pub fn hclge_tm_init_hw(hdev: *mut hclge_dev, init: bool) -> c_int;
}
extern "C" {
    pub fn hclge_mac_pause_en_cfg(hdev: *mut hclge_dev, tx: bool, rx: bool) -> c_int;
}
extern "C" {
    pub fn hclge_pause_addr_cfg(hdev: *mut hclge_dev, mac_addr: *const u8) -> c_int;
}
extern "C" {
    pub fn hclge_mac_pause_setup_hw(hdev: *mut hclge_dev) -> c_int;
}
extern "C" {
    pub fn hclge_pfc_rx_stats_get(hdev: *mut hclge_dev, stats: *mut u64);
}
extern "C" {
    pub fn hclge_pfc_tx_stats_get(hdev: *mut hclge_dev, stats: *mut u64);
}
extern "C" {
    pub fn hclge_tm_qs_shaper_cfg(vport: *mut hclge_vport, max_tx_rate: c_int) -> c_int;
}
extern "C" {
    pub fn hclge_tm_port_shaper_cfg(hdev: *mut hclge_dev) -> c_int;
}
extern "C" {
    pub fn hclge_tm_get_qset_num(hdev: *mut hclge_dev, qset_num: *mut u16) -> c_int;
}
extern "C" {
    pub fn hclge_tm_get_pri_num(hdev: *mut hclge_dev, pri_num: *mut u8) -> c_int;
}
extern "C" {
    pub fn hclge_tm_get_qset_sch_mode(hdev: *mut hclge_dev, qset_id: u16, mode: *mut u8) -> c_int;
}
extern "C" {
    pub fn hclge_tm_get_qset_weight(hdev: *mut hclge_dev, qset_id: u16, weight: *mut u8) -> c_int;
}
extern "C" {
    pub fn hclge_tm_get_pri_sch_mode(hdev: *mut hclge_dev, pri_id: u8, mode: *mut u8) -> c_int;
}
extern "C" {
    pub fn hclge_tm_get_pri_weight(hdev: *mut hclge_dev, pri_id: u8, weight: *mut u8) -> c_int;
}
extern "C" {
    pub fn hclge_tm_get_q_to_qs_map(hdev: *mut hclge_dev, q_id: u16, qset_id: *mut u16) -> c_int;
}
extern "C" {
    pub fn hclge_tm_get_q_to_tc(hdev: *mut hclge_dev, q_id: u16, tc_id: *mut u8) -> c_int;
}
extern "C" {
    pub fn hclge_tm_get_pg_weight(hdev: *mut hclge_dev, pg_id: u8, weight: *mut u8) -> c_int;
}
extern "C" {
    pub fn hclge_tm_get_pg_sch_mode(hdev: *mut hclge_dev, pg_id: u8, mode: *mut u8) -> c_int;
}
extern "C" {
    pub fn hclge_up_to_tc_map(hdev: *mut hclge_dev) -> c_int;
}
extern "C" {
    pub fn hclge_dscp_to_tc_map(hdev: *mut hclge_dev) -> c_int;
}
extern "C" {
    pub fn hclge_tm_flush_cfg(hdev: *mut hclge_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn hclge_reset_tc_config(hdev: *mut hclge_dev);
}
