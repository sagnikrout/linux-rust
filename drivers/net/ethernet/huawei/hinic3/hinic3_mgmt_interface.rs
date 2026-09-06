//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_mgmt_interface.h
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
#[derive(Copy, Clone)]
pub struct l2nic_cmd_feature_nego {
    pub msg_head: mgmt_msg_head,
    pub func_id: u16,
    pub opcode: u8,
    pub rsvd: u8,
    pub s_feature: [u64; 4],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum l2nic_func_tbl_cfg_bitmap {
    L2NIC_FUNC_TBL_CFG_INIT        = 0,
    L2NIC_FUNC_TBL_CFG_RX_BUF_SIZE = 1,
    L2NIC_FUNC_TBL_CFG_MTU         = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_func_tbl_cfg {
    pub rx_wqe_buf_size: u16,
    pub mtu: u16,
    pub rsvd: [u32; 9],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_set_func_tbl {
    pub msg_head: mgmt_msg_head,
    pub func_id: u16,
    pub rsvd: u16,
    pub cfg_bitmap: u32,
    pub tbl_cfg: l2nic_func_tbl_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_set_mac {
    pub msg_head: mgmt_msg_head,
    pub func_id: u16,
    pub vlan_id: u16,
    pub rsvd1: u16,
    pub mac: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_update_mac {
    pub msg_head: mgmt_msg_head,
    pub func_id: u16,
    pub vlan_id: u16,
    pub rsvd1: u16,
    pub old_mac: [u8; ETH_ALEN],
    pub rsvd2: u16,
    pub new_mac: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_vlan_config {
    pub msg_head: mgmt_msg_head,
    pub func_id: u16,
    pub opcode: u8,
    pub rsvd1: u8,
    pub vlan_id: u16,
    pub rsvd2: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_vlan_offload {
    pub msg_head: mgmt_msg_head,
    pub func_id: u16,
    pub vlan_offload: u8,
    pub rsvd1: [u8; 5],
}

// set vlan filter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_set_vlan_filter {
    pub msg_head: mgmt_msg_head,
    pub func_id: u16,
    pub rsvd: [u8; 2],
// bit0:vlan filter en; bit1:broadcast_filter_en
    pub vlan_filter_ctrl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_set_ci_attr {
    pub msg_head: mgmt_msg_head,
    pub func_idx: u16,
    pub dma_attr_off: u8,
    pub pending_limit: u8,
    pub coalescing_time: u8,
    pub intr_en: u8,
    pub intr_idx: u16,
    pub l2nic_sqn: u32,
    pub rsvd: u32,
    pub ci_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_clear_qp_resource {
    pub msg_head: mgmt_msg_head,
    pub func_id: u16,
    pub rsvd1: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_force_pkt_drop {
    pub msg_head: mgmt_msg_head,
    pub port: u8,
    pub rsvd1: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_set_vport_state {
    pub msg_head: mgmt_msg_head,
    pub func_id: u16,
    pub rsvd1: u16,
// 0--disable, 1--enable
    pub state: u8,
    pub rsvd2: [u8; 3],
}

//
// Definition of the NIC receiving mode
//
pub const L2NIC_RX_MODE_UC: c_uint = 0x01;
pub const L2NIC_RX_MODE_MC: c_uint = 0x02;
pub const L2NIC_RX_MODE_BC: c_uint = 0x04;
pub const L2NIC_RX_MODE_MC_ALL: c_uint = 0x08;
pub const L2NIC_RX_MODE_PROMISC: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_rx_mode_config {
    pub msg_head: mgmt_msg_head,
    pub func_id: u16,
    pub rsvd1: u16,
    pub rx_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_set_dcb_state {
    pub head: mgmt_msg_head,
    pub func_id: u16,
// 0 - get dcb state, 1 - set dcb state
    pub op_code: u8,
// 0 - disable, 1 - enable dcb
    pub state: u8,
// 0 - disable, 1 - enable dcb
    pub port_state: u8,
    pub rsvd: [u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_lro_config {
    pub msg_head: mgmt_msg_head,
    pub func_id: u16,
    pub opcode: u8,
    pub rsvd1: u8,
    pub lro_ipv4_en: u8,
    pub lro_ipv6_en: u8,
// unit is 1K
    pub lro_max_pkt_len: u8,
    pub resv2: [u8; 13],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_lro_timer {
    pub msg_head: mgmt_msg_head,
// 1: set timer value, 0: get timer value
    pub opcode: u8,
    pub rsvd: [u8; 3],
    pub timer: u32,
}

pub const L2NIC_RSS_INDIR_SIZE: c_int = 256;
pub const L2NIC_RSS_KEY_SIZE: c_int = 40;
// IEEE 802.1Qaz std
pub const L2NIC_DCB_COS_MAX: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_pause_config {
    pub msg_head: mgmt_msg_head,
    pub port_id: u8,
    pub opcode: u8,
    pub rsvd1: u16,
    pub auto_neg: u8,
    pub rx_pause: u8,
    pub tx_pause: u8,
    pub rsvd2: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_rss_ctx_tbl {
    pub msg_head: mgmt_msg_head,
    pub func_id: u16,
    pub rsvd1: u16,
    pub context: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_cfg_rss_engine {
    pub msg_head: mgmt_msg_head,
    pub func_id: u16,
    pub opcode: u8,
    pub hash_engine: u8,
    pub rsvd1: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_cfg_rss_hash_key {
    pub msg_head: mgmt_msg_head,
    pub func_id: u16,
    pub opcode: u8,
    pub rsvd1: u8,
    pub key: [u8; L2NIC_RSS_KEY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_cfg_rss {
    pub msg_head: mgmt_msg_head,
    pub func_id: u16,
    pub rss_en: u8,
    pub rq_priority_number: u8,
    pub prio_tc: [u8; L2NIC_DCB_COS_MAX],
    pub num_qps: u16,
    pub rsvd1: u16,
}

// Commands between NIC to fw
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum l2nic_cmd {
// FUNC CFG
    L2NIC_CMD_SET_FUNC_TBL        = 5,
    L2NIC_CMD_SET_VPORT_ENABLE    = 6,
    L2NIC_CMD_SET_RX_MODE         = 7,
    L2NIC_CMD_SET_SQ_CI_ATTR      = 8,
    L2NIC_CMD_CLEAR_QP_RESOURCE   = 11,
    L2NIC_CMD_CFG_RX_LRO          = 13,
    L2NIC_CMD_CFG_LRO_TIMER       = 14,
    L2NIC_CMD_FEATURE_NEGO        = 15,
    L2NIC_CMD_GET_MAC             = 20,
    L2NIC_CMD_SET_MAC             = 21,
    L2NIC_CMD_DEL_MAC             = 22,
    L2NIC_CMD_UPDATE_MAC          = 23,
    L2NIC_CMD_CFG_FUNC_VLAN       = 25,
    L2NIC_CMD_SET_VLAN_FILTER_EN  = 26,
    L2NIC_CMD_SET_RX_VLAN_OFFLOAD = 27,
    L2NIC_CMD_CFG_RSS             = 60,
    L2NIC_CMD_CFG_RSS_HASH_KEY    = 63,
    L2NIC_CMD_CFG_RSS_HASH_ENGINE = 64,
    L2NIC_CMD_SET_RSS_CTX_TBL     = 65,
    L2NIC_CMD_CFG_PAUSE_INFO      = 101,
    L2NIC_CMD_QOS_DCB_STATE       = 110,
    L2NIC_CMD_FORCE_PKT_DROP      = 113,
    L2NIC_CMD_MAX                 = 256,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2nic_cmd_rss_set_indir_tbl {
    pub rsvd: [__le32; 4],
    pub entry: [__le16; L2NIC_RSS_INDIR_SIZE],
}

// NIC CMDQ MODE
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum l2nic_ucode_cmd {
    L2NIC_UCODE_CMD_MODIFY_QUEUE_CTX  = 0,
    L2NIC_UCODE_CMD_CLEAN_QUEUE_CTX   = 1,
    L2NIC_UCODE_CMD_SET_RSS_INDIR_TBL = 4,
}

// hilink mac group command
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mag_cmd {
    MAG_CMD_SET_PORT_ENABLE = 6,
    MAG_CMD_GET_LINK_STATUS = 7,

    MAG_CMD_GET_PORT_INFO   = 153,
}

// firmware also use this cmd report link event to driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mag_cmd_get_link_status {
    pub head: mgmt_msg_head,
    pub port_id: u8,
// 0:link down  1:link up
    pub status: u8,
    pub rsvd0: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_nic_feature_cap {
    HINIC3_NIC_F_CSUM           = BIT(0),
    HINIC3_NIC_F_SCTP_CRC       = BIT(1),
    HINIC3_NIC_F_TSO            = BIT(2),
    HINIC3_NIC_F_LRO            = BIT(3),
    HINIC3_NIC_F_UFO            = BIT(4),
    HINIC3_NIC_F_RSS            = BIT(5),
    HINIC3_NIC_F_RX_VLAN_FILTER = BIT(6),
    HINIC3_NIC_F_RX_VLAN_STRIP  = BIT(7),
    HINIC3_NIC_F_TX_VLAN_INSERT = BIT(8),
    HINIC3_NIC_F_VXLAN_OFFLOAD  = BIT(9),
    HINIC3_NIC_F_FDIR           = BIT(11),
    HINIC3_NIC_F_PROMISC        = BIT(12),
    HINIC3_NIC_F_ALLMULTI       = BIT(13),
    HINIC3_NIC_F_RATE_LIMIT     = BIT(16),
}

pub const HINIC3_NIC_F_ALL_MASK: c_uint = 0x33bff;
pub const HINIC3_NIC_DRV_DEFAULT_FEATURE: c_uint = 0x3f03f;
