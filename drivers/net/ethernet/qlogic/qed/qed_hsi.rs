//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_hsi.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2021 Marvell International Ltd.
//

// Opcodes for the event ring
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum common_event_opcode {
    COMMON_EVENT_PF_START,
    COMMON_EVENT_PF_STOP,
    COMMON_EVENT_VF_START,
    COMMON_EVENT_VF_STOP,
    COMMON_EVENT_VF_PF_CHANNEL,
    COMMON_EVENT_VF_FLR,
    COMMON_EVENT_PF_UPDATE,
    COMMON_EVENT_FW_ERROR,
    COMMON_EVENT_RL_UPDATE,
    COMMON_EVENT_EMPTY,
    MAX_COMMON_EVENT_OPCODE
}

// Common Ramrod Command IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum common_ramrod_cmd_id {
    COMMON_RAMROD_UNUSED,
    COMMON_RAMROD_PF_START,
    COMMON_RAMROD_PF_STOP,
    COMMON_RAMROD_VF_START,
    COMMON_RAMROD_VF_STOP,
    COMMON_RAMROD_PF_UPDATE,
    COMMON_RAMROD_RL_UPDATE,
    COMMON_RAMROD_EMPTY,
    MAX_COMMON_RAMROD_CMD_ID
}

// How ll2 should deal with packet upon errors
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum core_error_handle {
    LL2_DROP_PACKET,
    LL2_DO_NOTHING,
    LL2_ASSERT,
    MAX_CORE_ERROR_HANDLE
}

// Opcodes for the event ring
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum core_event_opcode {
    CORE_EVENT_TX_QUEUE_START,
    CORE_EVENT_TX_QUEUE_STOP,
    CORE_EVENT_RX_QUEUE_START,
    CORE_EVENT_RX_QUEUE_STOP,
    CORE_EVENT_RX_QUEUE_FLUSH,
    CORE_EVENT_TX_QUEUE_UPDATE,
    CORE_EVENT_QUEUE_STATS_QUERY,
    MAX_CORE_EVENT_OPCODE
}

// The L4 pseudo checksum mode for Core
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum core_l4_pseudo_checksum_mode {
    CORE_L4_PSEUDO_CSUM_CORRECT_LENGTH,
    CORE_L4_PSEUDO_CSUM_ZERO_LENGTH,
    MAX_CORE_L4_PSEUDO_CHECKSUM_MODE
}

// LL2 SP error code
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum core_ll2_error_code {
    LL2_OK = 0,
    LL2_ERROR,
    MAX_CORE_LL2_ERROR_CODE
}

// Light-L2 RX Producers in Tstorm RAM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_ll2_port_stats {
    pub gsi_invalid_hdr: regpair,
    pub gsi_invalid_pkt_length: regpair,
    pub gsi_unsupported_pkt_typ: regpair,
    pub gsi_crcchksm_error: regpair,
}

// LL2 TX Per Queue Stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_ll2_pstorm_per_queue_stat {
    pub sent_ucast_bytes: regpair,
    pub sent_mcast_bytes: regpair,
    pub sent_bcast_bytes: regpair,
    pub sent_ucast_pkts: regpair,
    pub sent_mcast_pkts: regpair,
    pub sent_bcast_pkts: regpair,
    pub error_drop_pkts: regpair,
}

// Light-L2 RX Producers in Tstorm RAM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_ll2_rx_prod {
    pub bd_prod: __le16,
    pub cqe_prod: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_ll2_tstorm_per_queue_stat {
    pub packet_too_big_discard: regpair,
    pub no_buff_discard: regpair,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_ll2_ustorm_per_queue_stat {
    pub rcv_ucast_bytes: regpair,
    pub rcv_mcast_bytes: regpair,
    pub rcv_bcast_bytes: regpair,
    pub rcv_ucast_pkts: regpair,
    pub rcv_mcast_pkts: regpair,
    pub rcv_bcast_pkts: regpair,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_ll2_rx_per_queue_stat {
    pub tstorm_stat: core_ll2_tstorm_per_queue_stat,
    pub ustorm_stat: core_ll2_ustorm_per_queue_stat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_ll2_tx_per_queue_stat {
    pub pstorm_stat: core_ll2_pstorm_per_queue_stat,
}

// Structure for doorbell data, in PWM mode, for RX producers update.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_pwm_prod_update_data {
    pub /: *mut *mut __le16 icid; / internal CID,
    pub reserved0: u8,
    pub params: u8,
pub const CORE_PWM_PROD_UPDATE_DATA_AGG_CMD_MASK: c_uint = 0x3;
pub const CORE_PWM_PROD_UPDATE_DATA_AGG_CMD_SHIFT: c_int = 0;
pub const CORE_PWM_PROD_UPDATE_DATA_RESERVED1_MASK: c_uint = 0x3F	/* Set 0 */;
pub const CORE_PWM_PROD_UPDATE_DATA_RESERVED1_SHIFT: c_int = 2;
    pub /: *mut *mut core_ll2_rx_prod prod; / Producers,
}

// Ramrod data for rx/tx queue statistics query ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_queue_stats_query_ramrod_data {
    pub rx_stat: u8,
    pub tx_stat: u8,
    pub reserved: [__le16; 3],
    pub rx_stat_addr: regpair,
    pub tx_stat_addr: regpair,
}

// Core Ramrod Command IDs (light L2)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum core_ramrod_cmd_id {
    CORE_RAMROD_UNUSED,
    CORE_RAMROD_RX_QUEUE_START,
    CORE_RAMROD_TX_QUEUE_START,
    CORE_RAMROD_RX_QUEUE_STOP,
    CORE_RAMROD_TX_QUEUE_STOP,
    CORE_RAMROD_RX_QUEUE_FLUSH,
    CORE_RAMROD_TX_QUEUE_UPDATE,
    CORE_RAMROD_QUEUE_STATS_QUERY,
    MAX_CORE_RAMROD_CMD_ID
}

// Core RX CQE Type for Light L2
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum core_roce_flavor_type {
    CORE_ROCE,
    CORE_RROCE,
    MAX_CORE_ROCE_FLAVOR_TYPE
}

// Specifies how ll2 should deal with packets errors: packet_too_big and
// no_buff.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_rx_action_on_error {
    pub error_type: u8,
pub const CORE_RX_ACTION_ON_ERROR_PACKET_TOO_BIG_MASK: c_uint = 0x3;
pub const CORE_RX_ACTION_ON_ERROR_PACKET_TOO_BIG_SHIFT: c_int = 0;
pub const CORE_RX_ACTION_ON_ERROR_NO_BUFF_MASK: c_uint = 0x3;
pub const CORE_RX_ACTION_ON_ERROR_NO_BUFF_SHIFT: c_int = 2;
pub const CORE_RX_ACTION_ON_ERROR_RESERVED_MASK: c_uint = 0xF;
pub const CORE_RX_ACTION_ON_ERROR_RESERVED_SHIFT: c_int = 4;
}

// Core RX BD for Light L2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_rx_bd {
    pub addr: regpair,
    pub reserved: [__le16; 4],
}

// Core RX CM offload BD for Light L2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_rx_bd_with_buff_len {
    pub addr: regpair,
    pub buff_length: __le16,
    pub reserved: [__le16; 3],
}

// Core RX CM offload BD for Light L2
#[repr(C)]
#[derive(Copy, Clone)]
pub union core_rx_bd_union {
    pub rx_bd: core_rx_bd,
    pub rx_bd_with_len: core_rx_bd_with_buff_len,
}

// Opaque Data for Light L2 RX CQE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_rx_cqe_opaque_data {
    pub data: [__le32; 2],
}

// Core RX CQE Type for Light L2
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum core_rx_cqe_type {
    CORE_RX_CQE_ILLEGAL_TYPE,
    CORE_RX_CQE_TYPE_REGULAR,
    CORE_RX_CQE_TYPE_GSI_OFFLOAD,
    CORE_RX_CQE_TYPE_SLOW_PATH,
    MAX_CORE_RX_CQE_TYPE
}

// Core RX CQE for Light L2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_rx_fast_path_cqe {
    pub type: u8,
    pub placement_offset: u8,
    pub parse_flags: parsing_and_err_flags,
    pub packet_length: __le16,
    pub vlan: __le16,
    pub opaque_data: core_rx_cqe_opaque_data,
    pub err_flags: parsing_err_flags,
    pub packet_source: u8,
    pub reserved0: u8,
    pub reserved1: [__le32; 3],
}

// Core Rx CM offload CQE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_rx_gsi_offload_cqe {
    pub type: u8,
    pub data_length_error: u8,
    pub parse_flags: parsing_and_err_flags,
    pub data_length: __le16,
    pub vlan: __le16,
    pub src_mac_addrhi: __le32,
    pub src_mac_addrlo: __le16,
    pub qp_id: __le16,
    pub src_qp: __le32,
    pub opaque_data: core_rx_cqe_opaque_data,
    pub packet_source: u8,
    pub reserved: [u8; 3],
}

// Core RX CQE for Light L2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_rx_slow_path_cqe {
    pub type: u8,
    pub ramrod_cmd_id: u8,
    pub echo: __le16,
    pub opaque_data: core_rx_cqe_opaque_data,
    pub reserved1: [__le32; 5],
}

// Core RX CM offload BD for Light L2
#[repr(C)]
#[derive(Copy, Clone)]
pub union core_rx_cqe_union {
    pub rx_cqe_fp: core_rx_fast_path_cqe,
    pub rx_cqe_gsi: core_rx_gsi_offload_cqe,
    pub rx_cqe_sp: core_rx_slow_path_cqe,
}

// RX packet source.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum core_rx_pkt_source {
    CORE_RX_PKT_SOURCE_NETWORK = 0,
    CORE_RX_PKT_SOURCE_LB,
    CORE_RX_PKT_SOURCE_TX,
    CORE_RX_PKT_SOURCE_LL2_TX,
    MAX_CORE_RX_PKT_SOURCE
}

// Ramrod data for rx queue start ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_rx_start_ramrod_data {
    pub bd_base: regpair,
    pub cqe_pbl_addr: regpair,
    pub mtu: __le16,
    pub sb_id: __le16,
    pub sb_index: u8,
    pub complete_cqe_flg: u8,
    pub complete_event_flg: u8,
    pub drop_ttl0_flg: u8,
    pub num_of_pbl_pages: __le16,
    pub inner_vlan_stripping_en: u8,
    pub report_outer_vlan: u8,
    pub queue_id: u8,
    pub main_func_queue: u8,
    pub mf_si_bcast_accept_all: u8,
    pub mf_si_mcast_accept_all: u8,
    pub action_on_error: core_rx_action_on_error,
    pub gsi_offload_flag: u8,
    pub vport_id_valid: u8,
    pub vport_id: u8,
    pub zero_prod_flg: u8,
    pub wipe_inner_vlan_pri_en: u8,
    pub reserved: [u8; 2],
}

// Ramrod data for rx queue stop ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_rx_stop_ramrod_data {
    pub complete_cqe_flg: u8,
    pub complete_event_flg: u8,
    pub queue_id: u8,
    pub reserved1: u8,
    pub reserved2: [__le16; 2],
}

// Flags for Core TX BD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_tx_bd_data {
    pub as_bitfield: __le16,
pub const CORE_TX_BD_DATA_FORCE_VLAN_MODE_MASK: c_uint = 0x1;
pub const CORE_TX_BD_DATA_FORCE_VLAN_MODE_SHIFT: c_int = 0;
pub const CORE_TX_BD_DATA_VLAN_INSERTION_MASK: c_uint = 0x1;
pub const CORE_TX_BD_DATA_VLAN_INSERTION_SHIFT: c_int = 1;
pub const CORE_TX_BD_DATA_START_BD_MASK: c_uint = 0x1;
pub const CORE_TX_BD_DATA_START_BD_SHIFT: c_int = 2;
pub const CORE_TX_BD_DATA_IP_CSUM_MASK: c_uint = 0x1;
pub const CORE_TX_BD_DATA_IP_CSUM_SHIFT: c_int = 3;
pub const CORE_TX_BD_DATA_L4_CSUM_MASK: c_uint = 0x1;
pub const CORE_TX_BD_DATA_L4_CSUM_SHIFT: c_int = 4;
pub const CORE_TX_BD_DATA_IPV6_EXT_MASK: c_uint = 0x1;
pub const CORE_TX_BD_DATA_IPV6_EXT_SHIFT: c_int = 5;
pub const CORE_TX_BD_DATA_L4_PROTOCOL_MASK: c_uint = 0x1;
pub const CORE_TX_BD_DATA_L4_PROTOCOL_SHIFT: c_int = 6;
pub const CORE_TX_BD_DATA_L4_PSEUDO_CSUM_MODE_MASK: c_uint = 0x1;
pub const CORE_TX_BD_DATA_L4_PSEUDO_CSUM_MODE_SHIFT: c_int = 7;
pub const CORE_TX_BD_DATA_NBDS_MASK: c_uint = 0xF;
pub const CORE_TX_BD_DATA_NBDS_SHIFT: c_int = 8;
pub const CORE_TX_BD_DATA_ROCE_FLAV_MASK: c_uint = 0x1;
pub const CORE_TX_BD_DATA_ROCE_FLAV_SHIFT: c_int = 12;
pub const CORE_TX_BD_DATA_IP_LEN_MASK: c_uint = 0x1;
pub const CORE_TX_BD_DATA_IP_LEN_SHIFT: c_int = 13;
pub const CORE_TX_BD_DATA_DISABLE_STAG_INSERTION_MASK: c_uint = 0x1;
pub const CORE_TX_BD_DATA_DISABLE_STAG_INSERTION_SHIFT: c_int = 14;
pub const CORE_TX_BD_DATA_RESERVED0_MASK: c_uint = 0x1;
pub const CORE_TX_BD_DATA_RESERVED0_SHIFT: c_int = 15;
}

// Core TX BD for Light L2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_tx_bd {
    pub addr: regpair,
    pub nbytes: __le16,
    pub nw_vlan_or_lb_echo: __le16,
    pub bd_data: core_tx_bd_data,
    pub bitfield1: __le16,
pub const CORE_TX_BD_L4_HDR_OFFSET_W_MASK: c_uint = 0x3FFF;
pub const CORE_TX_BD_L4_HDR_OFFSET_W_SHIFT: c_int = 0;
pub const CORE_TX_BD_TX_DST_MASK: c_uint = 0x3;
pub const CORE_TX_BD_TX_DST_SHIFT: c_int = 14;
}

// Light L2 TX Destination
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum core_tx_dest {
    CORE_TX_DEST_NW,
    CORE_TX_DEST_LB,
    CORE_TX_DEST_RESERVED,
    CORE_TX_DEST_DROP,
    MAX_CORE_TX_DEST
}

// Ramrod data for tx queue start ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_tx_start_ramrod_data {
    pub pbl_base_addr: regpair,
    pub mtu: __le16,
    pub sb_id: __le16,
    pub sb_index: u8,
    pub stats_en: u8,
    pub stats_id: u8,
    pub conn_type: u8,
    pub pbl_size: __le16,
    pub qm_pq_id: __le16,
    pub gsi_offload_flag: u8,
    pub ctx_stats_en: u8,
    pub vport_id_valid: u8,
    pub vport_id: u8,
    pub enforce_security_flag: u8,
    pub reserved: [u8; 7],
}

// Ramrod data for tx queue stop ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_tx_stop_ramrod_data {
    pub reserved0: [__le32; 2],
}

// Ramrod data for tx queue update ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_tx_update_ramrod_data {
    pub update_qm_pq_id_flg: u8,
    pub reserved0: u8,
    pub qm_pq_id: __le16,
    pub reserved1: [__le32; 1],
}

// Enum flag for what type of dcb data to update
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcb_dscp_update_mode {
    DONT_UPDATE_DCB_DSCP,
    UPDATE_DCB,
    UPDATE_DSCP,
    UPDATE_DCB_DSCP,
    MAX_DCB_DSCP_UPDATE_MODE
}

// The core storm context for the Ystorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_core_conn_st_ctx {
    pub reserved: [__le32; 4],
}

// The core storm context for the Pstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstorm_core_conn_st_ctx {
    pub reserved: [__le32; 20],
}

// Core Slowpath Connection storm context of Xstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_core_conn_st_ctx {
    pub spq_base_addr: regpair,
    pub reserved0: [__le32; 2],
    pub spq_cons: __le16,
    pub reserved1: [__le16; 111],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_core_conn_ag_ctx {
    pub reserved0: u8,
    pub state: u8,
    pub flags0: u8,
pub const XSTORM_CORE_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED1_SHIFT: c_int = 1;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED2_SHIFT: c_int = 2;
pub const XSTORM_CORE_CONN_AG_CTX_EXIST_IN_QM3_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_EXIST_IN_QM3_SHIFT: c_int = 3;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED3_SHIFT: c_int = 4;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED4_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED4_SHIFT: c_int = 5;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED5_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED5_SHIFT: c_int = 6;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED6_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED6_SHIFT: c_int = 7;
    pub flags1: u8,
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED7_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED7_SHIFT: c_int = 0;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED8_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED8_SHIFT: c_int = 1;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED9_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED9_SHIFT: c_int = 2;
pub const XSTORM_CORE_CONN_AG_CTX_BIT11_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_BIT11_SHIFT: c_int = 3;
pub const XSTORM_CORE_CONN_AG_CTX_BIT12_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_BIT12_SHIFT: c_int = 4;
pub const XSTORM_CORE_CONN_AG_CTX_BIT13_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_BIT13_SHIFT: c_int = 5;
pub const XSTORM_CORE_CONN_AG_CTX_TX_RULE_ACTIVE_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_TX_RULE_ACTIVE_SHIFT: c_int = 6;
pub const XSTORM_CORE_CONN_AG_CTX_DQ_CF_ACTIVE_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_DQ_CF_ACTIVE_SHIFT: c_int = 7;
    pub flags2: u8,
pub const XSTORM_CORE_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF0_SHIFT: c_int = 0;
pub const XSTORM_CORE_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF1_SHIFT: c_int = 2;
pub const XSTORM_CORE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF2_SHIFT: c_int = 4;
pub const XSTORM_CORE_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF3_SHIFT: c_int = 6;
    pub flags3: u8,
pub const XSTORM_CORE_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF4_SHIFT: c_int = 0;
pub const XSTORM_CORE_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF5_SHIFT: c_int = 2;
pub const XSTORM_CORE_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF6_SHIFT: c_int = 4;
pub const XSTORM_CORE_CONN_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF7_SHIFT: c_int = 6;
    pub flags4: u8,
pub const XSTORM_CORE_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF8_SHIFT: c_int = 0;
pub const XSTORM_CORE_CONN_AG_CTX_CF9_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF9_SHIFT: c_int = 2;
pub const XSTORM_CORE_CONN_AG_CTX_CF10_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF10_SHIFT: c_int = 4;
pub const XSTORM_CORE_CONN_AG_CTX_CF11_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF11_SHIFT: c_int = 6;
    pub flags5: u8,
pub const XSTORM_CORE_CONN_AG_CTX_CF12_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF12_SHIFT: c_int = 0;
pub const XSTORM_CORE_CONN_AG_CTX_CF13_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF13_SHIFT: c_int = 2;
pub const XSTORM_CORE_CONN_AG_CTX_CF14_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF14_SHIFT: c_int = 4;
pub const XSTORM_CORE_CONN_AG_CTX_CF15_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF15_SHIFT: c_int = 6;
    pub flags6: u8,
pub const XSTORM_CORE_CONN_AG_CTX_CONSOLID_PROD_CF_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CONSOLID_PROD_CF_SHIFT: c_int = 0;
pub const XSTORM_CORE_CONN_AG_CTX_CF17_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF17_SHIFT: c_int = 2;
pub const XSTORM_CORE_CONN_AG_CTX_DQ_CF_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_DQ_CF_SHIFT: c_int = 4;
pub const XSTORM_CORE_CONN_AG_CTX_TERMINATE_CF_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_TERMINATE_CF_SHIFT: c_int = 6;
    pub flags7: u8,
pub const XSTORM_CORE_CONN_AG_CTX_FLUSH_Q0_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_FLUSH_Q0_SHIFT: c_int = 0;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED10_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED10_SHIFT: c_int = 2;
pub const XSTORM_CORE_CONN_AG_CTX_SLOW_PATH_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_SLOW_PATH_SHIFT: c_int = 4;
pub const XSTORM_CORE_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF0EN_SHIFT: c_int = 6;
pub const XSTORM_CORE_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags8: u8,
pub const XSTORM_CORE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 0;
pub const XSTORM_CORE_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF3EN_SHIFT: c_int = 1;
pub const XSTORM_CORE_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF4EN_SHIFT: c_int = 2;
pub const XSTORM_CORE_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF5EN_SHIFT: c_int = 3;
pub const XSTORM_CORE_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF6EN_SHIFT: c_int = 4;
pub const XSTORM_CORE_CONN_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF7EN_SHIFT: c_int = 5;
pub const XSTORM_CORE_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF8EN_SHIFT: c_int = 6;
pub const XSTORM_CORE_CONN_AG_CTX_CF9EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF9EN_SHIFT: c_int = 7;
    pub flags9: u8,
pub const XSTORM_CORE_CONN_AG_CTX_CF10EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF10EN_SHIFT: c_int = 0;
pub const XSTORM_CORE_CONN_AG_CTX_CF11EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF11EN_SHIFT: c_int = 1;
pub const XSTORM_CORE_CONN_AG_CTX_CF12EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF12EN_SHIFT: c_int = 2;
pub const XSTORM_CORE_CONN_AG_CTX_CF13EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF13EN_SHIFT: c_int = 3;
pub const XSTORM_CORE_CONN_AG_CTX_CF14EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF14EN_SHIFT: c_int = 4;
pub const XSTORM_CORE_CONN_AG_CTX_CF15EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF15EN_SHIFT: c_int = 5;
pub const XSTORM_CORE_CONN_AG_CTX_CONSOLID_PROD_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CONSOLID_PROD_CF_EN_SHIFT: c_int = 6;
pub const XSTORM_CORE_CONN_AG_CTX_CF17EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF17EN_SHIFT: c_int = 7;
    pub flags10: u8,
pub const XSTORM_CORE_CONN_AG_CTX_DQ_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_DQ_CF_EN_SHIFT: c_int = 0;
pub const XSTORM_CORE_CONN_AG_CTX_TERMINATE_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_TERMINATE_CF_EN_SHIFT: c_int = 1;
pub const XSTORM_CORE_CONN_AG_CTX_FLUSH_Q0_EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_FLUSH_Q0_EN_SHIFT: c_int = 2;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED11_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED11_SHIFT: c_int = 3;
pub const XSTORM_CORE_CONN_AG_CTX_SLOW_PATH_EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_SLOW_PATH_EN_SHIFT: c_int = 4;
pub const XSTORM_CORE_CONN_AG_CTX_CF23EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_CF23EN_SHIFT: c_int = 5;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED12_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED12_SHIFT: c_int = 6;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED13_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED13_SHIFT: c_int = 7;
    pub flags11: u8,
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED14_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED14_SHIFT: c_int = 0;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED15_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RESERVED15_SHIFT: c_int = 1;
pub const XSTORM_CORE_CONN_AG_CTX_TX_DEC_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_TX_DEC_RULE_EN_SHIFT: c_int = 2;
pub const XSTORM_CORE_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 3;
pub const XSTORM_CORE_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 4;
pub const XSTORM_CORE_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 5;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED1_SHIFT: c_int = 6;
pub const XSTORM_CORE_CONN_AG_CTX_RULE9EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RULE9EN_SHIFT: c_int = 7;
    pub flags12: u8,
pub const XSTORM_CORE_CONN_AG_CTX_RULE10EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RULE10EN_SHIFT: c_int = 0;
pub const XSTORM_CORE_CONN_AG_CTX_RULE11EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RULE11EN_SHIFT: c_int = 1;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED2_SHIFT: c_int = 2;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED3_SHIFT: c_int = 3;
pub const XSTORM_CORE_CONN_AG_CTX_RULE14EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RULE14EN_SHIFT: c_int = 4;
pub const XSTORM_CORE_CONN_AG_CTX_RULE15EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RULE15EN_SHIFT: c_int = 5;
pub const XSTORM_CORE_CONN_AG_CTX_RULE16EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RULE16EN_SHIFT: c_int = 6;
pub const XSTORM_CORE_CONN_AG_CTX_RULE17EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RULE17EN_SHIFT: c_int = 7;
    pub flags13: u8,
pub const XSTORM_CORE_CONN_AG_CTX_RULE18EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RULE18EN_SHIFT: c_int = 0;
pub const XSTORM_CORE_CONN_AG_CTX_RULE19EN_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_RULE19EN_SHIFT: c_int = 1;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED4_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED4_SHIFT: c_int = 2;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED5_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED5_SHIFT: c_int = 3;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED6_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED6_SHIFT: c_int = 4;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED7_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED7_SHIFT: c_int = 5;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED8_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED8_SHIFT: c_int = 6;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED9_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_A0_RESERVED9_SHIFT: c_int = 7;
    pub flags14: u8,
pub const XSTORM_CORE_CONN_AG_CTX_BIT16_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_BIT16_SHIFT: c_int = 0;
pub const XSTORM_CORE_CONN_AG_CTX_BIT17_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_BIT17_SHIFT: c_int = 1;
pub const XSTORM_CORE_CONN_AG_CTX_BIT18_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_BIT18_SHIFT: c_int = 2;
pub const XSTORM_CORE_CONN_AG_CTX_BIT19_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_BIT19_SHIFT: c_int = 3;
pub const XSTORM_CORE_CONN_AG_CTX_BIT20_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_BIT20_SHIFT: c_int = 4;
pub const XSTORM_CORE_CONN_AG_CTX_BIT21_MASK: c_uint = 0x1;
pub const XSTORM_CORE_CONN_AG_CTX_BIT21_SHIFT: c_int = 5;
pub const XSTORM_CORE_CONN_AG_CTX_CF23_MASK: c_uint = 0x3;
pub const XSTORM_CORE_CONN_AG_CTX_CF23_SHIFT: c_int = 6;
    pub byte2: u8,
    pub physical_q0: __le16,
    pub consolid_prod: __le16,
    pub reserved16: __le16,
    pub tx_bd_cons: __le16,
    pub tx_bd_or_spq_prod: __le16,
    pub updated_qm_pq_id: __le16,
    pub conn_dpi: __le16,
    pub byte3: u8,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub reg3: __le32,
    pub reg4: __le32,
    pub reg5: __le32,
    pub reg6: __le32,
    pub word7: __le16,
    pub word8: __le16,
    pub word9: __le16,
    pub word10: __le16,
    pub reg7: __le32,
    pub reg8: __le32,
    pub reg9: __le32,
    pub byte7: u8,
    pub byte8: u8,
    pub byte9: u8,
    pub byte10: u8,
    pub byte11: u8,
    pub byte12: u8,
    pub byte13: u8,
    pub byte14: u8,
    pub byte15: u8,
    pub e5_reserved: u8,
    pub word11: __le16,
    pub reg10: __le32,
    pub reg11: __le32,
    pub reg12: __le32,
    pub reg13: __le32,
    pub reg14: __le32,
    pub reg15: __le32,
    pub reg16: __le32,
    pub reg17: __le32,
    pub reg18: __le32,
    pub reg19: __le32,
    pub word12: __le16,
    pub word13: __le16,
    pub word14: __le16,
    pub word15: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_core_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const TSTORM_CORE_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const TSTORM_CORE_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const TSTORM_CORE_CONN_AG_CTX_BIT2_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_BIT2_SHIFT: c_int = 2;
pub const TSTORM_CORE_CONN_AG_CTX_BIT3_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_BIT3_SHIFT: c_int = 3;
pub const TSTORM_CORE_CONN_AG_CTX_BIT4_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_BIT4_SHIFT: c_int = 4;
pub const TSTORM_CORE_CONN_AG_CTX_BIT5_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_BIT5_SHIFT: c_int = 5;
pub const TSTORM_CORE_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const TSTORM_CORE_CONN_AG_CTX_CF0_SHIFT: c_int = 6;
    pub flags1: u8,
pub const TSTORM_CORE_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const TSTORM_CORE_CONN_AG_CTX_CF1_SHIFT: c_int = 0;
pub const TSTORM_CORE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const TSTORM_CORE_CONN_AG_CTX_CF2_SHIFT: c_int = 2;
pub const TSTORM_CORE_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const TSTORM_CORE_CONN_AG_CTX_CF3_SHIFT: c_int = 4;
pub const TSTORM_CORE_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const TSTORM_CORE_CONN_AG_CTX_CF4_SHIFT: c_int = 6;
    pub flags2: u8,
pub const TSTORM_CORE_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const TSTORM_CORE_CONN_AG_CTX_CF5_SHIFT: c_int = 0;
pub const TSTORM_CORE_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const TSTORM_CORE_CONN_AG_CTX_CF6_SHIFT: c_int = 2;
pub const TSTORM_CORE_CONN_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const TSTORM_CORE_CONN_AG_CTX_CF7_SHIFT: c_int = 4;
pub const TSTORM_CORE_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const TSTORM_CORE_CONN_AG_CTX_CF8_SHIFT: c_int = 6;
    pub flags3: u8,
pub const TSTORM_CORE_CONN_AG_CTX_CF9_MASK: c_uint = 0x3;
pub const TSTORM_CORE_CONN_AG_CTX_CF9_SHIFT: c_int = 0;
pub const TSTORM_CORE_CONN_AG_CTX_CF10_MASK: c_uint = 0x3;
pub const TSTORM_CORE_CONN_AG_CTX_CF10_SHIFT: c_int = 2;
pub const TSTORM_CORE_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_CF0EN_SHIFT: c_int = 4;
pub const TSTORM_CORE_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_CF1EN_SHIFT: c_int = 5;
pub const TSTORM_CORE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 6;
pub const TSTORM_CORE_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_CF3EN_SHIFT: c_int = 7;
    pub flags4: u8,
pub const TSTORM_CORE_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_CF4EN_SHIFT: c_int = 0;
pub const TSTORM_CORE_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_CF5EN_SHIFT: c_int = 1;
pub const TSTORM_CORE_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_CF6EN_SHIFT: c_int = 2;
pub const TSTORM_CORE_CONN_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_CF7EN_SHIFT: c_int = 3;
pub const TSTORM_CORE_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_CF8EN_SHIFT: c_int = 4;
pub const TSTORM_CORE_CONN_AG_CTX_CF9EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_CF9EN_SHIFT: c_int = 5;
pub const TSTORM_CORE_CONN_AG_CTX_CF10EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_CF10EN_SHIFT: c_int = 6;
pub const TSTORM_CORE_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 7;
    pub flags5: u8,
pub const TSTORM_CORE_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 0;
pub const TSTORM_CORE_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const TSTORM_CORE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const TSTORM_CORE_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const TSTORM_CORE_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const TSTORM_CORE_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 5;
pub const TSTORM_CORE_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const TSTORM_CORE_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const TSTORM_CORE_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub reg3: __le32,
    pub reg4: __le32,
    pub reg5: __le32,
    pub reg6: __le32,
    pub reg7: __le32,
    pub reg8: __le32,
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub byte4: u8,
    pub byte5: u8,
    pub word1: __le16,
    pub word2: __le16,
    pub word3: __le16,
    pub ll2_rx_prod: __le32,
    pub reg10: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_core_conn_ag_ctx {
    pub reserved: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const USTORM_CORE_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const USTORM_CORE_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const USTORM_CORE_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const USTORM_CORE_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const USTORM_CORE_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const USTORM_CORE_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const USTORM_CORE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const USTORM_CORE_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const USTORM_CORE_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const USTORM_CORE_CONN_AG_CTX_CF3_SHIFT: c_int = 0;
pub const USTORM_CORE_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const USTORM_CORE_CONN_AG_CTX_CF4_SHIFT: c_int = 2;
pub const USTORM_CORE_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const USTORM_CORE_CONN_AG_CTX_CF5_SHIFT: c_int = 4;
pub const USTORM_CORE_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const USTORM_CORE_CONN_AG_CTX_CF6_SHIFT: c_int = 6;
    pub flags2: u8,
pub const USTORM_CORE_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const USTORM_CORE_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const USTORM_CORE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const USTORM_CORE_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_CF3EN_SHIFT: c_int = 3;
pub const USTORM_CORE_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_CF4EN_SHIFT: c_int = 4;
pub const USTORM_CORE_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_CF5EN_SHIFT: c_int = 5;
pub const USTORM_CORE_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_CF6EN_SHIFT: c_int = 6;
pub const USTORM_CORE_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 7;
    pub flags3: u8,
pub const USTORM_CORE_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 0;
pub const USTORM_CORE_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const USTORM_CORE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const USTORM_CORE_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const USTORM_CORE_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const USTORM_CORE_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 5;
pub const USTORM_CORE_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const USTORM_CORE_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const USTORM_CORE_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub word1: __le16,
    pub rx_producers: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub reg3: __le32,
    pub word2: __le16,
    pub word3: __le16,
}

// The core storm context for the Mstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_core_conn_st_ctx {
    pub reserved: [__le32; 40],
}

// The core storm context for the Ustorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_core_conn_st_ctx {
    pub reserved: [__le32; 20],
}

// The core storm context for the Tstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_core_conn_st_ctx {
    pub reserved: [__le32; 4],
}

// core connection context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_conn_context {
    pub ystorm_st_context: ystorm_core_conn_st_ctx,
    pub ystorm_st_padding: [regpair; 2],
    pub pstorm_st_context: pstorm_core_conn_st_ctx,
    pub pstorm_st_padding: [regpair; 2],
    pub xstorm_st_context: xstorm_core_conn_st_ctx,
    pub xstorm_ag_context: xstorm_core_conn_ag_ctx,
    pub tstorm_ag_context: tstorm_core_conn_ag_ctx,
    pub ustorm_ag_context: ustorm_core_conn_ag_ctx,
    pub mstorm_st_context: mstorm_core_conn_st_ctx,
    pub ustorm_st_context: ustorm_core_conn_st_ctx,
    pub ustorm_st_padding: [regpair; 2],
    pub tstorm_st_context: tstorm_core_conn_st_ctx,
    pub tstorm_st_padding: [regpair; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_mstorm_per_pf_stat {
    pub gre_discard_pkts: regpair,
    pub vxlan_discard_pkts: regpair,
    pub geneve_discard_pkts: regpair,
    pub lb_discard_pkts: regpair,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_mstorm_per_queue_stat {
    pub ttl0_discard: regpair,
    pub packet_too_big_discard: regpair,
    pub no_buff_discard: regpair,
    pub not_active_discard: regpair,
    pub tpa_coalesced_pkts: regpair,
    pub tpa_coalesced_events: regpair,
    pub tpa_aborts_num: regpair,
    pub tpa_coalesced_bytes: regpair,
}

// Ethernet TX Per PF
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_pstorm_per_pf_stat {
    pub sent_lb_ucast_bytes: regpair,
    pub sent_lb_mcast_bytes: regpair,
    pub sent_lb_bcast_bytes: regpair,
    pub sent_lb_ucast_pkts: regpair,
    pub sent_lb_mcast_pkts: regpair,
    pub sent_lb_bcast_pkts: regpair,
    pub sent_gre_bytes: regpair,
    pub sent_vxlan_bytes: regpair,
    pub sent_geneve_bytes: regpair,
    pub sent_mpls_bytes: regpair,
    pub sent_gre_mpls_bytes: regpair,
    pub sent_udp_mpls_bytes: regpair,
    pub sent_gre_pkts: regpair,
    pub sent_vxlan_pkts: regpair,
    pub sent_geneve_pkts: regpair,
    pub sent_mpls_pkts: regpair,
    pub sent_gre_mpls_pkts: regpair,
    pub sent_udp_mpls_pkts: regpair,
    pub gre_drop_pkts: regpair,
    pub vxlan_drop_pkts: regpair,
    pub geneve_drop_pkts: regpair,
    pub mpls_drop_pkts: regpair,
    pub gre_mpls_drop_pkts: regpair,
    pub udp_mpls_drop_pkts: regpair,
}

// Ethernet TX Per Queue Stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_pstorm_per_queue_stat {
    pub sent_ucast_bytes: regpair,
    pub sent_mcast_bytes: regpair,
    pub sent_bcast_bytes: regpair,
    pub sent_ucast_pkts: regpair,
    pub sent_mcast_pkts: regpair,
    pub sent_bcast_pkts: regpair,
    pub error_drop_pkts: regpair,
}

// ETH Rx producers data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_rx_rate_limit {
    pub mult: __le16,
    pub cnst: __le16,
    pub add_sub_cnst: u8,
    pub reserved0: u8,
    pub reserved1: __le16,
}

// Update RSS indirection table entry command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tstorm_rss_update_data {
    pub vport_id: u8,
    pub ind_table_index: u8,
    pub ind_table_value: __le16,
    pub reserved1: __le16,
    pub reserved: u8,
    pub valid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_ustorm_per_pf_stat {
    pub rcv_lb_ucast_bytes: regpair,
    pub rcv_lb_mcast_bytes: regpair,
    pub rcv_lb_bcast_bytes: regpair,
    pub rcv_lb_ucast_pkts: regpair,
    pub rcv_lb_mcast_pkts: regpair,
    pub rcv_lb_bcast_pkts: regpair,
    pub rcv_gre_bytes: regpair,
    pub rcv_vxlan_bytes: regpair,
    pub rcv_geneve_bytes: regpair,
    pub rcv_gre_pkts: regpair,
    pub rcv_vxlan_pkts: regpair,
    pub rcv_geneve_pkts: regpair,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_ustorm_per_queue_stat {
    pub rcv_ucast_bytes: regpair,
    pub rcv_mcast_bytes: regpair,
    pub rcv_bcast_bytes: regpair,
    pub rcv_ucast_pkts: regpair,
    pub rcv_mcast_pkts: regpair,
    pub rcv_bcast_pkts: regpair,
}

// Event Ring VF-PF Channel data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_pf_channel_eqe_data {
    pub msg_addr: regpair,
}

// Event Ring initial cleanup data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct initial_cleanup_eqe_data {
    pub vf_id: u8,
    pub reserved: [u8; 7],
}

// FW error data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_err_data {
    pub recovery_scope: u8,
    pub err_id: u8,
    pub entity_id: __le16,
    pub reserved: [u8; 4],
}

// Event Data Union
#[repr(C)]
#[derive(Copy, Clone)]
pub union event_ring_data {
    pub bytes: [u8; 8],
    pub vf_pf_channel: vf_pf_channel_eqe_data,
    pub iscsi_info: iscsi_eqe_data,
    pub iscsi_conn_done_info: iscsi_connect_done_results,
    pub rdma_data: rdma_eqe_data,
    pub vf_init_cleanup: initial_cleanup_eqe_data,
    pub err_data: fw_err_data,
}

// Event Ring Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_ring_entry {
    pub protocol_id: u8,
    pub opcode: u8,
    pub reserved0: u8,
    pub vf_id: u8,
    pub echo: __le16,
    pub fw_return_code: u8,
    pub flags: u8,
pub const EVENT_RING_ENTRY_ASYNC_MASK: c_uint = 0x1;
pub const EVENT_RING_ENTRY_ASYNC_SHIFT: c_int = 0;
pub const EVENT_RING_ENTRY_RESERVED1_MASK: c_uint = 0x7F;
pub const EVENT_RING_ENTRY_RESERVED1_SHIFT: c_int = 1;
    pub data: event_ring_data,
}

// Event Ring Next Page Address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_ring_next_addr {
    pub addr: regpair,
    pub reserved: [__le32; 2],
}

// Event Ring Element
#[repr(C)]
#[derive(Copy, Clone)]
pub union event_ring_element {
    pub entry: event_ring_entry,
    pub next_addr: event_ring_next_addr,
}

// Ports mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_flow_ctrl_mode {
    flow_ctrl_pause,
    flow_ctrl_pfc,
    MAX_FW_FLOW_CTRL_MODE
}

// GFT profile type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gft_profile_type {
    GFT_PROFILE_TYPE_4_TUPLE,
    GFT_PROFILE_TYPE_L4_DST_PORT,
    GFT_PROFILE_TYPE_IP_DST_ADDR,
    GFT_PROFILE_TYPE_IP_SRC_ADDR,
    GFT_PROFILE_TYPE_TUNNEL_TYPE,
    MAX_GFT_PROFILE_TYPE
}

// Major and Minor hsi Versions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsi_fp_ver_struct {
    pub minor_ver_arr: [u8; 2],
    pub major_ver_arr: [u8; 2],
}

// Integration Phase
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum integ_phase {
    INTEG_PHASE_BB_A0_LATEST = 3,
    INTEG_PHASE_BB_B0_NO_MCP = 10,
    INTEG_PHASE_BB_B0_WITH_MCP = 11,
    MAX_INTEG_PHASE
}

// Ports mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwarp_ll2_tx_queues {
    IWARP_LL2_IN_ORDER_TX_QUEUE = 1,
    IWARP_LL2_ALIGNED_TX_QUEUE,
    IWARP_LL2_ALIGNED_RIGHT_TRIMMED_TX_QUEUE,
    IWARP_LL2_ERROR,
    MAX_IWARP_LL2_TX_QUEUES
}

// Function error ID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum func_err_id {
    FUNC_NO_ERROR,
    VF_PF_CHANNEL_NOT_READY,
    VF_ZONE_MSG_NOT_VALID,
    VF_ZONE_FUNC_NOT_ENABLED,
    ETH_PACKET_TOO_SMALL,
    ETH_ILLEGAL_VLAN_MODE,
    ETH_MTU_VIOLATION,
    ETH_ILLEGAL_INBAND_TAGS,
    ETH_VLAN_INSERT_AND_INBAND_VLAN,
    ETH_ILLEGAL_NBDS,
    ETH_FIRST_BD_WO_SOP,
    ETH_INSUFFICIENT_BDS,
    ETH_ILLEGAL_LSO_HDR_NBDS,
    ETH_ILLEGAL_LSO_MSS,
    ETH_ZERO_SIZE_BD,
    ETH_ILLEGAL_LSO_HDR_LEN,
    ETH_INSUFFICIENT_PAYLOAD,
    ETH_EDPM_OUT_OF_SYNC,
    ETH_TUNN_IPV6_EXT_NBD_ERR,
    ETH_CONTROL_PACKET_VIOLATION,
    ETH_ANTI_SPOOFING_ERR,
    ETH_PACKET_SIZE_TOO_LARGE,
    CORE_ILLEGAL_VLAN_MODE,
    CORE_ILLEGAL_NBDS,
    CORE_FIRST_BD_WO_SOP,
    CORE_INSUFFICIENT_BDS,
    CORE_PACKET_TOO_SMALL,
    CORE_ILLEGAL_INBAND_TAGS,
    CORE_VLAN_INSERT_AND_INBAND_VLAN,
    CORE_MTU_VIOLATION,
    CORE_CONTROL_PACKET_VIOLATION,
    CORE_ANTI_SPOOFING_ERR,
    CORE_PACKET_SIZE_TOO_LARGE,
    CORE_ILLEGAL_BD_FLAGS,
    CORE_GSI_PACKET_VIOLATION,
    MAX_FUNC_ERR_ID
}

// FW error handling mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_err_mode {
    FW_ERR_FATAL_ASSERT,
    FW_ERR_DRV_REPORT,
    MAX_FW_ERR_MODE
}

// FW error recovery scope
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_err_recovery_scope {
    ERR_SCOPE_INVALID,
    ERR_SCOPE_TX_Q,
    ERR_SCOPE_RX_Q,
    ERR_SCOPE_QP,
    ERR_SCOPE_VPORT,
    ERR_SCOPE_FUNC,
    ERR_SCOPE_PORT,
    ERR_SCOPE_ENGINE,
    MAX_FW_ERR_RECOVERY_SCOPE
}

// Mstorm non-triggering VF zone
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_non_trigger_vf_zone {
    pub eth_queue_stat: eth_mstorm_per_queue_stat,
    pub eth_rx_queue_producers: [eth_rx_prod_data; ETH_MAX_RXQ_VF_QUAD],
}

// Mstorm VF zone
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_vf_zone {
    pub non_trigger: mstorm_non_trigger_vf_zone,
}

// vlan header including TPID and TCI fields
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_header {
    pub tpid: __le16,
    pub tci: __le16,
}

// outer tag configurations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct outer_tag_config_struct {
    pub enable_stag_pri_change: u8,
    pub pri_map_valid: u8,
    pub reserved: [u8; 2],
    pub outer_tag: vlan_header,
    pub inner_to_outer_pri_map: [u8; 8],
}

// personality per PF
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum personality_type {
    BAD_PERSONALITY_TYP,
    PERSONALITY_TCP_ULP,
    PERSONALITY_FCOE,
    PERSONALITY_RDMA_AND_ETH,
    PERSONALITY_RDMA,
    PERSONALITY_CORE,
    PERSONALITY_ETH,
    PERSONALITY_RESERVED,
    MAX_PERSONALITY_TYPE
}

// tunnel configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pf_start_tunnel_config {
    pub set_vxlan_udp_port_flg: u8,
    pub set_geneve_udp_port_flg: u8,
    pub set_no_inner_l2_vxlan_udp_port_flg: u8,
    pub tunnel_clss_vxlan: u8,
    pub tunnel_clss_l2geneve: u8,
    pub tunnel_clss_ipgeneve: u8,
    pub tunnel_clss_l2gre: u8,
    pub tunnel_clss_ipgre: u8,
    pub vxlan_udp_port: __le16,
    pub geneve_udp_port: __le16,
    pub no_inner_l2_vxlan_udp_port: __le16,
    pub reserved: [__le16; 3],
}

// Ramrod data for PF start ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pf_start_ramrod_data {
    pub event_ring_pbl_addr: regpair,
    pub consolid_q_pbl_base_addr: regpair,
    pub tunnel_config: pf_start_tunnel_config,
    pub event_ring_sb_id: __le16,
    pub base_vf_id: u8,
    pub num_vfs: u8,
    pub event_ring_num_pages: u8,
    pub event_ring_sb_index: u8,
    pub path_id: u8,
    pub warning_as_error: u8,
    pub dont_log_ramrods: u8,
    pub personality: u8,
    pub log_type_mask: __le16,
    pub mf_mode: u8,
    pub integ_phase: u8,
    pub allow_npar_tx_switching: u8,
    pub reserved0: u8,
    pub hsi_fp_ver: hsi_fp_ver_struct,
    pub outer_tag_config: outer_tag_config_struct,
    pub pf_fp_err_mode: u8,
    pub consolid_q_num_pages: u8,
    pub reserved: [u8; 6],
}

// Data for port update ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct protocol_dcb_data {
    pub dcb_enable_flag: u8,
    pub dscp_enable_flag: u8,
    pub dcb_priority: u8,
    pub dcb_tc: u8,
    pub dscp_val: u8,
    pub dcb_dont_add_vlan0: u8,
}

// Update tunnel configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pf_update_tunnel_config {
    pub update_rx_pf_clss: u8,
    pub update_rx_def_ucast_clss: u8,
    pub update_rx_def_non_ucast_clss: u8,
    pub set_vxlan_udp_port_flg: u8,
    pub set_geneve_udp_port_flg: u8,
    pub set_no_inner_l2_vxlan_udp_port_flg: u8,
    pub tunnel_clss_vxlan: u8,
    pub tunnel_clss_l2geneve: u8,
    pub tunnel_clss_ipgeneve: u8,
    pub tunnel_clss_l2gre: u8,
    pub tunnel_clss_ipgre: u8,
    pub reserved: u8,
    pub vxlan_udp_port: __le16,
    pub geneve_udp_port: __le16,
    pub no_inner_l2_vxlan_udp_port: __le16,
    pub reserved1: [__le16; 3],
}

// Data for port update ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pf_update_ramrod_data {
    pub update_eth_dcb_data_mode: u8,
    pub update_fcoe_dcb_data_mode: u8,
    pub update_iscsi_dcb_data_mode: u8,
    pub update_roce_dcb_data_mode: u8,
    pub update_rroce_dcb_data_mode: u8,
    pub update_iwarp_dcb_data_mode: u8,
    pub update_mf_vlan_flag: u8,
    pub update_enable_stag_pri_change: u8,
    pub eth_dcb_data: protocol_dcb_data,
    pub fcoe_dcb_data: protocol_dcb_data,
    pub iscsi_dcb_data: protocol_dcb_data,
    pub roce_dcb_data: protocol_dcb_data,
    pub rroce_dcb_data: protocol_dcb_data,
    pub iwarp_dcb_data: protocol_dcb_data,
    pub mf_vlan: __le16,
    pub enable_stag_pri_change: u8,
    pub reserved: u8,
    pub tunnel_config: pf_update_tunnel_config,
}

// Ports mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ports_mode {
    ENGX2_PORTX1,
    ENGX2_PORTX2,
    ENGX1_PORTX1,
    ENGX1_PORTX2,
    ENGX1_PORTX4,
    MAX_PORTS_MODE
}

// Protocol-common error code
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum protocol_common_error_code {
    COMMON_ERR_CODE_OK = 0,
    COMMON_ERR_CODE_ERROR,
    MAX_PROTOCOL_COMMON_ERROR_CODE
}

// use to index in hsi_fp_[major|minor]_ver_arr per protocol
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum protocol_version_array_key {
    ETH_VER_KEY = 0,
    ROCE_VER_KEY,
    MAX_PROTOCOL_VERSION_ARRAY_KEY
}

// RDMA TX Stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_sent_stats {
    pub sent_bytes: regpair,
    pub sent_pkts: regpair,
}

// Pstorm non-triggering VF zone
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstorm_non_trigger_vf_zone {
    pub eth_queue_stat: eth_pstorm_per_queue_stat,
    pub rdma_stats: rdma_sent_stats,
}

// Pstorm VF zone
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstorm_vf_zone {
    pub non_trigger: pstorm_non_trigger_vf_zone,
    pub reserved: [regpair; 7],
}

// Ramrod Header of SPQE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ramrod_header {
    pub cid: __le32,
    pub cmd_id: u8,
    pub protocol_id: u8,
    pub echo: __le16,
}

// RDMA RX Stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_rcv_stats {
    pub rcv_bytes: regpair,
    pub rcv_pkts: regpair,
}

// Data for update QCN/DCQCN RL ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rl_update_ramrod_data {
    pub qcn_update_param_flg: u8,
    pub dcqcn_update_param_flg: u8,
    pub rl_init_flg: u8,
    pub rl_start_flg: u8,
    pub rl_stop_flg: u8,
    pub rl_id_first: u8,
    pub rl_id_last: u8,
    pub rl_dc_qcn_flg: u8,
    pub dcqcn_reset_alpha_on_idle: u8,
    pub rl_bc_stage_th: u8,
    pub rl_timer_stage_th: u8,
    pub reserved1: u8,
    pub rl_bc_rate: __le32,
    pub rl_max_rate: __le16,
    pub rl_r_ai: __le16,
    pub rl_r_hai: __le16,
    pub dcqcn_g: __le16,
    pub dcqcn_k_us: __le32,
    pub dcqcn_timeuot_us: __le32,
    pub qcn_timeuot_us: __le32,
    pub reserved2: __le32,
}

// Slowpath Element (SPQE)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slow_path_element {
    pub hdr: ramrod_header,
    pub data_ptr: regpair,
}

// Tstorm non-triggering VF zone
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_non_trigger_vf_zone {
    pub rdma_stats: rdma_rcv_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_per_port_stat {
    pub trunc_error_discard: regpair,
    pub mac_error_discard: regpair,
    pub mftag_filter_discard: regpair,
    pub eth_mac_filter_discard: regpair,
    pub ll2_mac_filter_discard: regpair,
    pub ll2_conn_disabled_discard: regpair,
    pub iscsi_irregular_pkt: regpair,
    pub fcoe_irregular_pkt: regpair,
    pub roce_irregular_pkt: regpair,
    pub iwarp_irregular_pkt: regpair,
    pub eth_irregular_pkt: regpair,
    pub toe_irregular_pkt: regpair,
    pub preroce_irregular_pkt: regpair,
    pub eth_gre_tunn_filter_discard: regpair,
    pub eth_vxlan_tunn_filter_discard: regpair,
    pub eth_geneve_tunn_filter_discard: regpair,
    pub eth_gft_drop_pkt: regpair,
}

// Tstorm VF zone
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_vf_zone {
    pub non_trigger: tstorm_non_trigger_vf_zone,
}

// Tunnel classification scheme
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tunnel_clss {
    TUNNEL_CLSS_MAC_VLAN = 0,
    TUNNEL_CLSS_MAC_VNI,
    TUNNEL_CLSS_INNER_MAC_VLAN,
    TUNNEL_CLSS_INNER_MAC_VNI,
    TUNNEL_CLSS_MAC_VLAN_DUAL_STAGE,
    MAX_TUNNEL_CLSS
}

// Ustorm non-triggering VF zone
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_non_trigger_vf_zone {
    pub eth_queue_stat: eth_ustorm_per_queue_stat,
    pub vf_pf_msg_addr: regpair,
}

// Ustorm triggering VF zone
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_trigger_vf_zone {
    pub vf_pf_msg_valid: u8,
    pub reserved: [u8; 7],
}

// Ustorm VF zone
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_vf_zone {
    pub non_trigger: ustorm_non_trigger_vf_zone,
    pub trigger: ustorm_trigger_vf_zone,
}

// VF-PF channel data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_pf_channel_data {
    pub ready: __le32,
    pub valid: u8,
    pub reserved0: u8,
    pub reserved1: __le16,
}

// Ramrod data for VF start ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_start_ramrod_data {
    pub vf_id: u8,
    pub enable_flr_ack: u8,
    pub opaque_fid: __le16,
    pub personality: u8,
    pub reserved: [u8; 7],
    pub hsi_fp_ver: hsi_fp_ver_struct,
}

// Ramrod data for VF start ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_stop_ramrod_data {
    pub vf_id: u8,
    pub reserved0: u8,
    pub reserved1: __le16,
    pub reserved2: __le32,
}

// VF zone size mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vf_zone_size_mode {
    VF_ZONE_SIZE_MODE_DEFAULT,
    VF_ZONE_SIZE_MODE_DOUBLE,
    VF_ZONE_SIZE_MODE_QUAD,
    MAX_VF_ZONE_SIZE_MODE
}

// Xstorm non-triggering VF zone
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_non_trigger_vf_zone {
    pub non_edpm_ack_pkts: regpair,
}

// Tstorm VF zone
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_vf_zone {
    pub non_trigger: xstorm_non_trigger_vf_zone,
}

// Attentions status block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atten_status_block {
    pub atten_bits: __le32,
    pub atten_ack: __le32,
    pub reserved0: __le16,
    pub sb_index: __le16,
    pub reserved1: __le32,
}

// DMAE command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmae_cmd {
    pub opcode: __le32,
pub const DMAE_CMD_SRC_MASK: c_uint = 0x1;
pub const DMAE_CMD_SRC_SHIFT: c_int = 0;
pub const DMAE_CMD_DST_MASK: c_uint = 0x3;
pub const DMAE_CMD_DST_SHIFT: c_int = 1;
pub const DMAE_CMD_C_DST_MASK: c_uint = 0x1;
pub const DMAE_CMD_C_DST_SHIFT: c_int = 3;
pub const DMAE_CMD_CRC_RESET_MASK: c_uint = 0x1;
pub const DMAE_CMD_CRC_RESET_SHIFT: c_int = 4;
pub const DMAE_CMD_SRC_ADDR_RESET_MASK: c_uint = 0x1;
pub const DMAE_CMD_SRC_ADDR_RESET_SHIFT: c_int = 5;
pub const DMAE_CMD_DST_ADDR_RESET_MASK: c_uint = 0x1;
pub const DMAE_CMD_DST_ADDR_RESET_SHIFT: c_int = 6;
pub const DMAE_CMD_COMP_FUNC_MASK: c_uint = 0x1;
pub const DMAE_CMD_COMP_FUNC_SHIFT: c_int = 7;
pub const DMAE_CMD_COMP_WORD_EN_MASK: c_uint = 0x1;
pub const DMAE_CMD_COMP_WORD_EN_SHIFT: c_int = 8;
pub const DMAE_CMD_COMP_CRC_EN_MASK: c_uint = 0x1;
pub const DMAE_CMD_COMP_CRC_EN_SHIFT: c_int = 9;
pub const DMAE_CMD_COMP_CRC_OFFSET_MASK: c_uint = 0x7;
pub const DMAE_CMD_COMP_CRC_OFFSET_SHIFT: c_int = 10;
pub const DMAE_CMD_RESERVED1_MASK: c_uint = 0x1;
pub const DMAE_CMD_RESERVED1_SHIFT: c_int = 13;
pub const DMAE_CMD_ENDIANITY_MODE_MASK: c_uint = 0x3;
pub const DMAE_CMD_ENDIANITY_MODE_SHIFT: c_int = 14;
pub const DMAE_CMD_ERR_HANDLING_MASK: c_uint = 0x3;
pub const DMAE_CMD_ERR_HANDLING_SHIFT: c_int = 16;
pub const DMAE_CMD_PORT_ID_MASK: c_uint = 0x3;
pub const DMAE_CMD_PORT_ID_SHIFT: c_int = 18;
pub const DMAE_CMD_SRC_PF_ID_MASK: c_uint = 0xF;
pub const DMAE_CMD_SRC_PF_ID_SHIFT: c_int = 20;
pub const DMAE_CMD_DST_PF_ID_MASK: c_uint = 0xF;
pub const DMAE_CMD_DST_PF_ID_SHIFT: c_int = 24;
pub const DMAE_CMD_SRC_VF_ID_VALID_MASK: c_uint = 0x1;
pub const DMAE_CMD_SRC_VF_ID_VALID_SHIFT: c_int = 28;
pub const DMAE_CMD_DST_VF_ID_VALID_MASK: c_uint = 0x1;
pub const DMAE_CMD_DST_VF_ID_VALID_SHIFT: c_int = 29;
pub const DMAE_CMD_RESERVED2_MASK: c_uint = 0x3;
pub const DMAE_CMD_RESERVED2_SHIFT: c_int = 30;
    pub src_addr_lo: __le32,
    pub src_addr_hi: __le32,
    pub dst_addr_lo: __le32,
    pub dst_addr_hi: __le32,
    pub length_dw: __le16,
    pub opcode_b: __le16,
pub const DMAE_CMD_SRC_VF_ID_MASK: c_uint = 0xFF;
pub const DMAE_CMD_SRC_VF_ID_SHIFT: c_int = 0;
pub const DMAE_CMD_DST_VF_ID_MASK: c_uint = 0xFF;
pub const DMAE_CMD_DST_VF_ID_SHIFT: c_int = 8;
    pub comp_addr_lo: __le32,
    pub comp_addr_hi: __le32,
    pub comp_val: __le32,
    pub crc32: __le32,
    pub crc_32_c: __le32,
    pub crc16: __le16,
    pub crc16_c: __le16,
    pub crc10: __le16,
    pub error_bit_reserved: __le16,
pub const DMAE_CMD_ERROR_BIT_MASK: c_uint = 0x1;
pub const DMAE_CMD_ERROR_BIT_SHIFT: c_int = 0;
pub const DMAE_CMD_RESERVED_MASK: c_uint = 0x7FFF;
pub const DMAE_CMD_RESERVED_SHIFT: c_int = 1;
    pub xsum16: __le16,
    pub xsum8: __le16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmae_cmd_comp_crc_en_enum {
    dmae_cmd_comp_crc_disabled,
    dmae_cmd_comp_crc_enabled,
    MAX_DMAE_CMD_COMP_CRC_EN_ENUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmae_cmd_comp_func_enum {
    dmae_cmd_comp_func_to_src,
    dmae_cmd_comp_func_to_dst,
    MAX_DMAE_CMD_COMP_FUNC_ENUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmae_cmd_comp_word_en_enum {
    dmae_cmd_comp_word_disabled,
    dmae_cmd_comp_word_enabled,
    MAX_DMAE_CMD_COMP_WORD_EN_ENUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmae_cmd_c_dst_enum {
    dmae_cmd_c_dst_pcie,
    dmae_cmd_c_dst_grc,
    MAX_DMAE_CMD_C_DST_ENUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmae_cmd_dst_enum {
    dmae_cmd_dst_none_0,
    dmae_cmd_dst_pcie,
    dmae_cmd_dst_grc,
    dmae_cmd_dst_none_3,
    MAX_DMAE_CMD_DST_ENUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmae_cmd_error_handling_enum {
    dmae_cmd_error_handling_send_regular_comp,
    dmae_cmd_error_handling_send_comp_with_err,
    dmae_cmd_error_handling_dont_send_comp,
    MAX_DMAE_CMD_ERROR_HANDLING_ENUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmae_cmd_src_enum {
    dmae_cmd_src_pcie,
    dmae_cmd_src_grc,
    MAX_DMAE_CMD_SRC_ENUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_core_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const MSTORM_CORE_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const MSTORM_CORE_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const MSTORM_CORE_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const MSTORM_CORE_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const MSTORM_CORE_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const MSTORM_CORE_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const MSTORM_CORE_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const MSTORM_CORE_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const MSTORM_CORE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const MSTORM_CORE_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const MSTORM_CORE_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const MSTORM_CORE_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const MSTORM_CORE_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const MSTORM_CORE_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const MSTORM_CORE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const MSTORM_CORE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const MSTORM_CORE_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const MSTORM_CORE_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const MSTORM_CORE_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const MSTORM_CORE_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const MSTORM_CORE_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const MSTORM_CORE_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const MSTORM_CORE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const MSTORM_CORE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const MSTORM_CORE_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const MSTORM_CORE_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub word0: __le16,
    pub word1: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_core_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const YSTORM_CORE_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const YSTORM_CORE_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const YSTORM_CORE_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const YSTORM_CORE_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const YSTORM_CORE_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const YSTORM_CORE_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const YSTORM_CORE_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const YSTORM_CORE_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const YSTORM_CORE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const YSTORM_CORE_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const YSTORM_CORE_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const YSTORM_CORE_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const YSTORM_CORE_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const YSTORM_CORE_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const YSTORM_CORE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const YSTORM_CORE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const YSTORM_CORE_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const YSTORM_CORE_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const YSTORM_CORE_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const YSTORM_CORE_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const YSTORM_CORE_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const YSTORM_CORE_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const YSTORM_CORE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const YSTORM_CORE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const YSTORM_CORE_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const YSTORM_CORE_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
    pub word1: __le16,
    pub word2: __le16,
    pub word3: __le16,
    pub word4: __le16,
    pub reg2: __le32,
    pub reg3: __le32,
}

// DMAE parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dmae_params {
    pub flags: u32,
// If QED_DMAE_PARAMS_RW_REPL_SRC flag is set and the
// source is a block of length DMAE_MAX_RW_SIZE and the
// destination is larger, the source block will be duplicated as
// many times as required to fill the destination block. This is
// used mostly to write a zeroed buffer to destination address
// using DMA
//
pub const QED_DMAE_PARAMS_RW_REPL_SRC_MASK: c_uint = 0x1;
pub const QED_DMAE_PARAMS_RW_REPL_SRC_SHIFT: c_int = 0;
pub const QED_DMAE_PARAMS_SRC_VF_VALID_MASK: c_uint = 0x1;
pub const QED_DMAE_PARAMS_SRC_VF_VALID_SHIFT: c_int = 1;
pub const QED_DMAE_PARAMS_DST_VF_VALID_MASK: c_uint = 0x1;
pub const QED_DMAE_PARAMS_DST_VF_VALID_SHIFT: c_int = 2;
pub const QED_DMAE_PARAMS_COMPLETION_DST_MASK: c_uint = 0x1;
pub const QED_DMAE_PARAMS_COMPLETION_DST_SHIFT: c_int = 3;
pub const QED_DMAE_PARAMS_PORT_VALID_MASK: c_uint = 0x1;
pub const QED_DMAE_PARAMS_PORT_VALID_SHIFT: c_int = 4;
pub const QED_DMAE_PARAMS_SRC_PF_VALID_MASK: c_uint = 0x1;
pub const QED_DMAE_PARAMS_SRC_PF_VALID_SHIFT: c_int = 5;
pub const QED_DMAE_PARAMS_DST_PF_VALID_MASK: c_uint = 0x1;
pub const QED_DMAE_PARAMS_DST_PF_VALID_SHIFT: c_int = 6;
pub const QED_DMAE_PARAMS_RESERVED_MASK: c_uint = 0x1FFFFFF;
pub const QED_DMAE_PARAMS_RESERVED_SHIFT: c_int = 7;
    pub src_vfid: u8,
    pub dst_vfid: u8,
    pub port_id: u8,
    pub src_pfid: u8,
    pub dst_pfid: u8,
    pub reserved1: u8,
    pub reserved2: __le16,
}

// IGU cleanup command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igu_cleanup {
    pub sb_id_and_flags: __le32,
pub const IGU_CLEANUP_RESERVED0_MASK: c_uint = 0x7FFFFFF;
pub const IGU_CLEANUP_RESERVED0_SHIFT: c_int = 0;
pub const IGU_CLEANUP_CLEANUP_SET_MASK: c_uint = 0x1;
pub const IGU_CLEANUP_CLEANUP_SET_SHIFT: c_int = 27;
pub const IGU_CLEANUP_CLEANUP_TYPE_MASK: c_uint = 0x7;
pub const IGU_CLEANUP_CLEANUP_TYPE_SHIFT: c_int = 28;
pub const IGU_CLEANUP_COMMAND_TYPE_MASK: c_uint = 0x1;
pub const IGU_CLEANUP_COMMAND_TYPE_SHIFT: c_int = 31;
    pub reserved1: __le32,
}

// IGU firmware driver command
#[repr(C)]
#[derive(Copy, Clone)]
pub union igu_command {
    pub prod_cons_update: igu_prod_cons_update,
    pub cleanup: igu_cleanup,
}

// IGU firmware driver command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igu_command_reg_ctrl {
    pub opaque_fid: __le16,
    pub igu_command_reg_ctrl_fields: __le16,
pub const IGU_COMMAND_REG_CTRL_PXP_BAR_ADDR_MASK: c_uint = 0xFFF;
pub const IGU_COMMAND_REG_CTRL_PXP_BAR_ADDR_SHIFT: c_int = 0;
pub const IGU_COMMAND_REG_CTRL_RESERVED_MASK: c_uint = 0x7;
pub const IGU_COMMAND_REG_CTRL_RESERVED_SHIFT: c_int = 12;
pub const IGU_COMMAND_REG_CTRL_COMMAND_TYPE_MASK: c_uint = 0x1;
pub const IGU_COMMAND_REG_CTRL_COMMAND_TYPE_SHIFT: c_int = 15;
}

// IGU mapping line structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igu_mapping_line {
    pub igu_mapping_line_fields: __le32,
pub const IGU_MAPPING_LINE_VALID_MASK: c_uint = 0x1;
pub const IGU_MAPPING_LINE_VALID_SHIFT: c_int = 0;
pub const IGU_MAPPING_LINE_VECTOR_NUMBER_MASK: c_uint = 0xFF;
pub const IGU_MAPPING_LINE_VECTOR_NUMBER_SHIFT: c_int = 1;
pub const IGU_MAPPING_LINE_FUNCTION_NUMBER_MASK: c_uint = 0xFF;
pub const IGU_MAPPING_LINE_FUNCTION_NUMBER_SHIFT: c_int = 9;
pub const IGU_MAPPING_LINE_PF_VALID_MASK: c_uint = 0x1;
pub const IGU_MAPPING_LINE_PF_VALID_SHIFT: c_int = 17;
pub const IGU_MAPPING_LINE_IPS_GROUP_MASK: c_uint = 0x3F;
pub const IGU_MAPPING_LINE_IPS_GROUP_SHIFT: c_int = 18;
pub const IGU_MAPPING_LINE_RESERVED_MASK: c_uint = 0xFF;
pub const IGU_MAPPING_LINE_RESERVED_SHIFT: c_int = 24;
}

// IGU MSIX line structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igu_msix_vector {
    pub address: regpair,
    pub data: __le32,
    pub msix_vector_fields: __le32,
pub const IGU_MSIX_VECTOR_MASK_BIT_MASK: c_uint = 0x1;
pub const IGU_MSIX_VECTOR_MASK_BIT_SHIFT: c_int = 0;
pub const IGU_MSIX_VECTOR_RESERVED0_MASK: c_uint = 0x7FFF;
pub const IGU_MSIX_VECTOR_RESERVED0_SHIFT: c_int = 1;
pub const IGU_MSIX_VECTOR_STEERING_TAG_MASK: c_uint = 0xFF;
pub const IGU_MSIX_VECTOR_STEERING_TAG_SHIFT: c_int = 16;
pub const IGU_MSIX_VECTOR_RESERVED1_MASK: c_uint = 0xFF;
pub const IGU_MSIX_VECTOR_RESERVED1_SHIFT: c_int = 24;
}

// per encapsulation type enabling flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prs_reg_encapsulation_type_en {
    pub flags: u8,
pub const PRS_REG_ENCAPSULATION_TYPE_EN_ETH_OVER_GRE_ENABLE_MASK: c_uint = 0x1;
pub const PRS_REG_ENCAPSULATION_TYPE_EN_ETH_OVER_GRE_ENABLE_SHIFT: c_int = 0;
pub const PRS_REG_ENCAPSULATION_TYPE_EN_IP_OVER_GRE_ENABLE_MASK: c_uint = 0x1;
pub const PRS_REG_ENCAPSULATION_TYPE_EN_IP_OVER_GRE_ENABLE_SHIFT: c_int = 1;
pub const PRS_REG_ENCAPSULATION_TYPE_EN_VXLAN_ENABLE_MASK: c_uint = 0x1;
pub const PRS_REG_ENCAPSULATION_TYPE_EN_VXLAN_ENABLE_SHIFT: c_int = 2;
pub const PRS_REG_ENCAPSULATION_TYPE_EN_T_TAG_ENABLE_MASK: c_uint = 0x1;
pub const PRS_REG_ENCAPSULATION_TYPE_EN_T_TAG_ENABLE_SHIFT: c_int = 3;
pub const PRS_REG_ENCAPSULATION_TYPE_EN_ETH_OVER_GENEVE_ENABLE_MASK: c_uint = 0x1;
pub const PRS_REG_ENCAPSULATION_TYPE_EN_ETH_OVER_GENEVE_ENABLE_SHIFT: c_int = 4;
pub const PRS_REG_ENCAPSULATION_TYPE_EN_IP_OVER_GENEVE_ENABLE_MASK: c_uint = 0x1;
pub const PRS_REG_ENCAPSULATION_TYPE_EN_IP_OVER_GENEVE_ENABLE_SHIFT: c_int = 5;
pub const PRS_REG_ENCAPSULATION_TYPE_EN_RESERVED_MASK: c_uint = 0x3;
pub const PRS_REG_ENCAPSULATION_TYPE_EN_RESERVED_SHIFT: c_int = 6;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pxp_tph_st_hint {
    TPH_ST_HINT_BIDIR,
    TPH_ST_HINT_REQUESTER,
    TPH_ST_HINT_TARGET,
    TPH_ST_HINT_TARGET_PRIO,
    MAX_PXP_TPH_ST_HINT
}

// QM hardware structure of enable bypass credit mask
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_rf_bypass_mask {
    pub flags: u8,
pub const QM_RF_BYPASS_MASK_LINEVOQ_MASK: c_uint = 0x1;
pub const QM_RF_BYPASS_MASK_LINEVOQ_SHIFT: c_int = 0;
pub const QM_RF_BYPASS_MASK_RESERVED0_MASK: c_uint = 0x1;
pub const QM_RF_BYPASS_MASK_RESERVED0_SHIFT: c_int = 1;
pub const QM_RF_BYPASS_MASK_PFWFQ_MASK: c_uint = 0x1;
pub const QM_RF_BYPASS_MASK_PFWFQ_SHIFT: c_int = 2;
pub const QM_RF_BYPASS_MASK_VPWFQ_MASK: c_uint = 0x1;
pub const QM_RF_BYPASS_MASK_VPWFQ_SHIFT: c_int = 3;
pub const QM_RF_BYPASS_MASK_PFRL_MASK: c_uint = 0x1;
pub const QM_RF_BYPASS_MASK_PFRL_SHIFT: c_int = 4;
pub const QM_RF_BYPASS_MASK_VPQCNRL_MASK: c_uint = 0x1;
pub const QM_RF_BYPASS_MASK_VPQCNRL_SHIFT: c_int = 5;
pub const QM_RF_BYPASS_MASK_FWPAUSE_MASK: c_uint = 0x1;
pub const QM_RF_BYPASS_MASK_FWPAUSE_SHIFT: c_int = 6;
pub const QM_RF_BYPASS_MASK_RESERVED1_MASK: c_uint = 0x1;
pub const QM_RF_BYPASS_MASK_RESERVED1_SHIFT: c_int = 7;
}

// QM hardware structure of opportunistic credit mask
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_rf_opportunistic_mask {
    pub flags: __le16,
pub const QM_RF_OPPORTUNISTIC_MASK_LINEVOQ_MASK: c_uint = 0x1;
pub const QM_RF_OPPORTUNISTIC_MASK_LINEVOQ_SHIFT: c_int = 0;
pub const QM_RF_OPPORTUNISTIC_MASK_BYTEVOQ_MASK: c_uint = 0x1;
pub const QM_RF_OPPORTUNISTIC_MASK_BYTEVOQ_SHIFT: c_int = 1;
pub const QM_RF_OPPORTUNISTIC_MASK_PFWFQ_MASK: c_uint = 0x1;
pub const QM_RF_OPPORTUNISTIC_MASK_PFWFQ_SHIFT: c_int = 2;
pub const QM_RF_OPPORTUNISTIC_MASK_VPWFQ_MASK: c_uint = 0x1;
pub const QM_RF_OPPORTUNISTIC_MASK_VPWFQ_SHIFT: c_int = 3;
pub const QM_RF_OPPORTUNISTIC_MASK_PFRL_MASK: c_uint = 0x1;
pub const QM_RF_OPPORTUNISTIC_MASK_PFRL_SHIFT: c_int = 4;
pub const QM_RF_OPPORTUNISTIC_MASK_VPQCNRL_MASK: c_uint = 0x1;
pub const QM_RF_OPPORTUNISTIC_MASK_VPQCNRL_SHIFT: c_int = 5;
pub const QM_RF_OPPORTUNISTIC_MASK_FWPAUSE_MASK: c_uint = 0x1;
pub const QM_RF_OPPORTUNISTIC_MASK_FWPAUSE_SHIFT: c_int = 6;
pub const QM_RF_OPPORTUNISTIC_MASK_RESERVED0_MASK: c_uint = 0x1;
pub const QM_RF_OPPORTUNISTIC_MASK_RESERVED0_SHIFT: c_int = 7;
pub const QM_RF_OPPORTUNISTIC_MASK_QUEUEEMPTY_MASK: c_uint = 0x1;
pub const QM_RF_OPPORTUNISTIC_MASK_QUEUEEMPTY_SHIFT: c_int = 8;
pub const QM_RF_OPPORTUNISTIC_MASK_RESERVED1_MASK: c_uint = 0x7F;
pub const QM_RF_OPPORTUNISTIC_MASK_RESERVED1_SHIFT: c_int = 9;
}

// QM hardware structure of QM map memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_rf_pq_map {
    pub reg: __le32,
pub const QM_RF_PQ_MAP_PQ_VALID_MASK: c_uint = 0x1;
pub const QM_RF_PQ_MAP_PQ_VALID_SHIFT: c_int = 0;
pub const QM_RF_PQ_MAP_RL_ID_MASK: c_uint = 0xFF;
pub const QM_RF_PQ_MAP_RL_ID_SHIFT: c_int = 1;
pub const QM_RF_PQ_MAP_VP_PQ_ID_MASK: c_uint = 0x1FF;
pub const QM_RF_PQ_MAP_VP_PQ_ID_SHIFT: c_int = 9;
pub const QM_RF_PQ_MAP_VOQ_MASK: c_uint = 0x1F;
pub const QM_RF_PQ_MAP_VOQ_SHIFT: c_int = 18;
pub const QM_RF_PQ_MAP_WRR_WEIGHT_GROUP_MASK: c_uint = 0x3;
pub const QM_RF_PQ_MAP_WRR_WEIGHT_GROUP_SHIFT: c_int = 23;
pub const QM_RF_PQ_MAP_RL_VALID_MASK: c_uint = 0x1;
pub const QM_RF_PQ_MAP_RL_VALID_SHIFT: c_int = 25;
pub const QM_RF_PQ_MAP_RESERVED_MASK: c_uint = 0x3F;
pub const QM_RF_PQ_MAP_RESERVED_SHIFT: c_int = 26;
}

// Completion params for aggregated interrupt completion
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdm_agg_int_comp_params {
    pub params: __le16,
pub const SDM_AGG_INT_COMP_PARAMS_AGG_INT_INDEX_MASK: c_uint = 0x3F;
pub const SDM_AGG_INT_COMP_PARAMS_AGG_INT_INDEX_SHIFT: c_int = 0;
pub const SDM_AGG_INT_COMP_PARAMS_AGG_VECTOR_ENABLE_MASK: c_uint = 0x1;
pub const SDM_AGG_INT_COMP_PARAMS_AGG_VECTOR_ENABLE_SHIFT: c_int = 6;
pub const SDM_AGG_INT_COMP_PARAMS_AGG_VECTOR_BIT_MASK: c_uint = 0x1FF;
pub const SDM_AGG_INT_COMP_PARAMS_AGG_VECTOR_BIT_SHIFT: c_int = 7;
}

// SDM operation gen command (generate aggregative interrupt)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdm_op_gen {
    pub command: __le32,
pub const SDM_OP_GEN_COMP_PARAM_MASK: c_uint = 0xFFFF;
pub const SDM_OP_GEN_COMP_PARAM_SHIFT: c_int = 0;
pub const SDM_OP_GEN_COMP_TYPE_MASK: c_uint = 0xF;
pub const SDM_OP_GEN_COMP_TYPE_SHIFT: c_int = 16;
pub const SDM_OP_GEN_RESERVED_MASK: c_uint = 0xFFF;
pub const SDM_OP_GEN_RESERVED_SHIFT: c_int = 20;
}

// Physical memory descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phys_mem_desc {
    pub phys_addr: dma_addr_t,
    pub virt_addr: *mut c_void,
    pub /: *mut *mut u32 size; / In bytes,
}

// Virtual memory descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virt_mem_desc {
    pub ptr: *mut c_void,
    pub /: *mut *mut u32 size; / In bytes,
}

//
// HSI Init Functions constants
//
// Number of VLAN priorities
pub const NUM_OF_VLAN_PRIORITIES: c_int = 8;
// BRB RAM init requirements
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_brb_ram_req {
    pub guranteed_per_tc: u32,
    pub headroom_per_tc: u32,
    pub min_pkt_size: u32,
    pub max_ports_per_engine: u32,
    pub num_active_tcs: [u8; MAX_NUM_PORTS],
}

// ETS per-TC init requirements
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_ets_tc_req {
    pub use_sp: u8,
    pub use_wfq: u8,
    pub weight: u16,
}

// ETS init requirements
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_ets_req {
    pub mtu: u32,
    pub tc_req: [init_ets_tc_req; NUM_OF_TCS],
}

// NIG LB RL init requirements
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_nig_lb_rl_req {
    pub lb_mac_rate: u16,
    pub lb_rate: u16,
    pub mtu: u32,
    pub tc_rate: [u16; NUM_OF_PHYS_TCS],
}

// NIG TC mapping for each priority
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_nig_pri_tc_map_entry {
    pub tc_id: u8,
    pub valid: u8,
}

// NIG priority to TC map init requirements
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_nig_pri_tc_map_req {
    pub pri: [init_nig_pri_tc_map_entry; NUM_OF_VLAN_PRIORITIES],
}

// QM per global RL init parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_qm_global_rl_params {
    pub type: u8,
    pub reserved0: u8,
    pub reserved1: u16,
    pub rate_limit: u32,
}

// QM per-port init parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_qm_port_params {
    pub active_phys_tcs: u16,
    pub num_pbf_cmd_lines: u16,
    pub num_btb_blocks: u16,
    pub active: u8,
    pub reserved: u8,
}

// QM per-PQ init parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_qm_pq_params {
    pub vport_id: u16,
    pub rl_id: u16,
    pub rl_valid: u8,
    pub tc_id: u8,
    pub wrr_group: u8,
    pub port_id: u8,
}

// QM per RL init parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_qm_rl_params {
    pub vport_rl: u32,
    pub vport_rl_type: u8,
    pub reserved: [u8; 3],
}

// QM Rate Limiter types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum init_qm_rl_type {
    QM_RL_TYPE_NORMAL,
    QM_RL_TYPE_QCN,
    MAX_INIT_QM_RL_TYPE
}

// QM per-vport init parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_qm_vport_params {
    pub wfq: u16,
    pub reserved: u16,
    pub tc_wfq: [u16; NUM_OF_TCS],
    pub first_tx_pq_id: [u16; NUM_OF_TCS],
}

//
// Init Tool HSI constants and macros
//
// Width of GRC address in bits (addresses are specified in dwords)
pub const GRC_ADDR_BITS: c_int = 23;

// indicates an init that should be applied to any phase ID
pub const ANY_PHASE_ID: c_uint = 0xffff;
// Max size in dwords of a zipped array
pub const MAX_ZIPPED_SIZE: c_int = 8192;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chip_ids {
    CHIP_BB,
    CHIP_K2,
    MAX_CHIP_IDS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_asserts_ram_section {
    pub section_ram_line_offset: __le16,
    pub section_ram_line_size: __le16,
    pub list_dword_offset: u8,
    pub list_element_dword_size: u8,
    pub list_num_elements: u8,
    pub list_next_index_dword_offset: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ver_num {
    pub major: u8,
    pub minor: u8,
    pub rev: u8,
    pub eng: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_ver_info {
    pub tools_ver: __le16,
    pub image_id: u8,
    pub reserved1: u8,
    pub num: fw_ver_num,
    pub timestamp: __le32,
    pub reserved2: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_info {
    pub ver: fw_ver_info,
    pub fw_asserts_section: fw_asserts_ram_section,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_info_location {
    pub grc_addr: __le32,
    pub size: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum init_modes {
    MODE_BB_A0_DEPRECATED,
    MODE_BB,
    MODE_K2,
    MODE_ASIC,
    MODE_EMUL_REDUCED,
    MODE_EMUL_FULL,
    MODE_FPGA,
    MODE_CHIPSIM,
    MODE_SF,
    MODE_MF_SD,
    MODE_MF_SI,
    MODE_PORTS_PER_ENG_1,
    MODE_PORTS_PER_ENG_2,
    MODE_PORTS_PER_ENG_4,
    MODE_100G,
    MODE_SKIP_PRAM_INIT,
    MODE_EMUL_MAC,
    MAX_INIT_MODES
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum init_phases {
    PHASE_ENGINE,
    PHASE_PORT,
    PHASE_PF,
    PHASE_VF,
    PHASE_QM_PF,
    MAX_INIT_PHASES
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum init_split_types {
    SPLIT_TYPE_NONE,
    SPLIT_TYPE_PORT,
    SPLIT_TYPE_PF,
    SPLIT_TYPE_PORT_PF,
    SPLIT_TYPE_VF,
    MAX_INIT_SPLIT_TYPES
}

// Binary buffer header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bin_buffer_hdr {
    pub offset: u32,
    pub length: u32,
}

// Binary init buffer types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bin_init_buffer_type {
    BIN_BUF_INIT_FW_VER_INFO,
    BIN_BUF_INIT_CMD,
    BIN_BUF_INIT_VAL,
    BIN_BUF_INIT_MODE_TREE,
    BIN_BUF_INIT_IRO,
    BIN_BUF_INIT_OVERLAYS,
    MAX_BIN_INIT_BUFFER_TYPE
}

// FW overlay buffer header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_overlay_buf_hdr {
    pub data: u32,
pub const FW_OVERLAY_BUF_HDR_STORM_ID_MASK: c_uint = 0xFF;
pub const FW_OVERLAY_BUF_HDR_STORM_ID_SHIFT: c_int = 0;
pub const FW_OVERLAY_BUF_HDR_BUF_SIZE_MASK: c_uint = 0xFFFFFF;
pub const FW_OVERLAY_BUF_HDR_BUF_SIZE_SHIFT: c_int = 8;
}

// init array header: raw
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_array_raw_hdr {
    pub data: __le32,
pub const INIT_ARRAY_RAW_HDR_TYPE_MASK: c_uint = 0xF;
pub const INIT_ARRAY_RAW_HDR_TYPE_SHIFT: c_int = 0;
pub const INIT_ARRAY_RAW_HDR_PARAMS_MASK: c_uint = 0xFFFFFFF;
pub const INIT_ARRAY_RAW_HDR_PARAMS_SHIFT: c_int = 4;
}

// init array header: standard
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_array_standard_hdr {
    pub data: __le32,
pub const INIT_ARRAY_STANDARD_HDR_TYPE_MASK: c_uint = 0xF;
pub const INIT_ARRAY_STANDARD_HDR_TYPE_SHIFT: c_int = 0;
pub const INIT_ARRAY_STANDARD_HDR_SIZE_MASK: c_uint = 0xFFFFFFF;
pub const INIT_ARRAY_STANDARD_HDR_SIZE_SHIFT: c_int = 4;
}

// init array header: zipped
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_array_zipped_hdr {
    pub data: __le32,
pub const INIT_ARRAY_ZIPPED_HDR_TYPE_MASK: c_uint = 0xF;
pub const INIT_ARRAY_ZIPPED_HDR_TYPE_SHIFT: c_int = 0;
pub const INIT_ARRAY_ZIPPED_HDR_ZIPPED_SIZE_MASK: c_uint = 0xFFFFFFF;
pub const INIT_ARRAY_ZIPPED_HDR_ZIPPED_SIZE_SHIFT: c_int = 4;
}

// init array header: pattern
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_array_pattern_hdr {
    pub data: __le32,
pub const INIT_ARRAY_PATTERN_HDR_TYPE_MASK: c_uint = 0xF;
pub const INIT_ARRAY_PATTERN_HDR_TYPE_SHIFT: c_int = 0;
pub const INIT_ARRAY_PATTERN_HDR_PATTERN_SIZE_MASK: c_uint = 0xF;
pub const INIT_ARRAY_PATTERN_HDR_PATTERN_SIZE_SHIFT: c_int = 4;
pub const INIT_ARRAY_PATTERN_HDR_REPETITIONS_MASK: c_uint = 0xFFFFFF;
pub const INIT_ARRAY_PATTERN_HDR_REPETITIONS_SHIFT: c_int = 8;
}

// init array header union
#[repr(C)]
#[derive(Copy, Clone)]
pub union init_array_hdr {
    pub raw: init_array_raw_hdr,
    pub standard: init_array_standard_hdr,
    pub zipped: init_array_zipped_hdr,
    pub pattern: init_array_pattern_hdr,
}

// init array types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum init_array_types {
    INIT_ARR_STANDARD,
    INIT_ARR_ZIPPED,
    INIT_ARR_PATTERN,
    MAX_INIT_ARRAY_TYPES
}

// init operation: callback
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_callback_op {
    pub op_data: __le32,
pub const INIT_CALLBACK_OP_OP_MASK: c_uint = 0xF;
pub const INIT_CALLBACK_OP_OP_SHIFT: c_int = 0;
pub const INIT_CALLBACK_OP_RESERVED_MASK: c_uint = 0xFFFFFFF;
pub const INIT_CALLBACK_OP_RESERVED_SHIFT: c_int = 4;
    pub callback_id: __le16,
    pub block_id: __le16,
}

// init operation: delay
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_delay_op {
    pub op_data: __le32,
pub const INIT_DELAY_OP_OP_MASK: c_uint = 0xF;
pub const INIT_DELAY_OP_OP_SHIFT: c_int = 0;
pub const INIT_DELAY_OP_RESERVED_MASK: c_uint = 0xFFFFFFF;
pub const INIT_DELAY_OP_RESERVED_SHIFT: c_int = 4;
    pub delay: __le32,
}

// init operation: if_mode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_if_mode_op {
    pub op_data: __le32,
pub const INIT_IF_MODE_OP_OP_MASK: c_uint = 0xF;
pub const INIT_IF_MODE_OP_OP_SHIFT: c_int = 0;
pub const INIT_IF_MODE_OP_RESERVED1_MASK: c_uint = 0xFFF;
pub const INIT_IF_MODE_OP_RESERVED1_SHIFT: c_int = 4;
pub const INIT_IF_MODE_OP_CMD_OFFSET_MASK: c_uint = 0xFFFF;
pub const INIT_IF_MODE_OP_CMD_OFFSET_SHIFT: c_int = 16;
    pub reserved2: __le16,
    pub modes_buf_offset: __le16,
}

// init operation: if_phase
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_if_phase_op {
    pub op_data: __le32,
pub const INIT_IF_PHASE_OP_OP_MASK: c_uint = 0xF;
pub const INIT_IF_PHASE_OP_OP_SHIFT: c_int = 0;
pub const INIT_IF_PHASE_OP_RESERVED1_MASK: c_uint = 0xFFF;
pub const INIT_IF_PHASE_OP_RESERVED1_SHIFT: c_int = 4;
pub const INIT_IF_PHASE_OP_CMD_OFFSET_MASK: c_uint = 0xFFFF;
pub const INIT_IF_PHASE_OP_CMD_OFFSET_SHIFT: c_int = 16;
    pub phase_data: __le32,
pub const INIT_IF_PHASE_OP_PHASE_MASK: c_uint = 0xFF;
pub const INIT_IF_PHASE_OP_PHASE_SHIFT: c_int = 0;
pub const INIT_IF_PHASE_OP_RESERVED2_MASK: c_uint = 0xFF;
pub const INIT_IF_PHASE_OP_RESERVED2_SHIFT: c_int = 8;
pub const INIT_IF_PHASE_OP_PHASE_ID_MASK: c_uint = 0xFFFF;
pub const INIT_IF_PHASE_OP_PHASE_ID_SHIFT: c_int = 16;
}

// init mode operators
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum init_mode_ops {
    INIT_MODE_OP_NOT,
    INIT_MODE_OP_OR,
    INIT_MODE_OP_AND,
    MAX_INIT_MODE_OPS
}

// init operation: raw
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_raw_op {
    pub op_data: __le32,
pub const INIT_RAW_OP_OP_MASK: c_uint = 0xF;
pub const INIT_RAW_OP_OP_SHIFT: c_int = 0;
pub const INIT_RAW_OP_PARAM1_MASK: c_uint = 0xFFFFFFF;
pub const INIT_RAW_OP_PARAM1_SHIFT: c_int = 4;
    pub param2: __le32,
}

// init array params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_op_array_params {
    pub size: __le16,
    pub offset: __le16,
}

// Write init operation arguments
#[repr(C)]
#[derive(Copy, Clone)]
pub union init_write_args {
    pub inline_val: __le32,
    pub zeros_count: __le32,
    pub array_offset: __le32,
    pub runtime: init_op_array_params,
}

// init operation: write
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_write_op {
    pub data: __le32,
pub const INIT_WRITE_OP_OP_MASK: c_uint = 0xF;
pub const INIT_WRITE_OP_OP_SHIFT: c_int = 0;
pub const INIT_WRITE_OP_SOURCE_MASK: c_uint = 0x7;
pub const INIT_WRITE_OP_SOURCE_SHIFT: c_int = 4;
pub const INIT_WRITE_OP_RESERVED_MASK: c_uint = 0x1;
pub const INIT_WRITE_OP_RESERVED_SHIFT: c_int = 7;
pub const INIT_WRITE_OP_WIDE_BUS_MASK: c_uint = 0x1;
pub const INIT_WRITE_OP_WIDE_BUS_SHIFT: c_int = 8;
pub const INIT_WRITE_OP_ADDRESS_MASK: c_uint = 0x7FFFFF;
pub const INIT_WRITE_OP_ADDRESS_SHIFT: c_int = 9;
    pub args: init_write_args,
}

// init operation: read
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_read_op {
    pub op_data: __le32,
pub const INIT_READ_OP_OP_MASK: c_uint = 0xF;
pub const INIT_READ_OP_OP_SHIFT: c_int = 0;
pub const INIT_READ_OP_POLL_TYPE_MASK: c_uint = 0xF;
pub const INIT_READ_OP_POLL_TYPE_SHIFT: c_int = 4;
pub const INIT_READ_OP_RESERVED_MASK: c_uint = 0x1;
pub const INIT_READ_OP_RESERVED_SHIFT: c_int = 8;
pub const INIT_READ_OP_ADDRESS_MASK: c_uint = 0x7FFFFF;
pub const INIT_READ_OP_ADDRESS_SHIFT: c_int = 9;
    pub expected_val: __le32,
}

// Init operations union
#[repr(C)]
#[derive(Copy, Clone)]
pub union init_op {
    pub raw: init_raw_op,
    pub write: init_write_op,
    pub read: init_read_op,
    pub if_mode: init_if_mode_op,
    pub if_phase: init_if_phase_op,
    pub callback: init_callback_op,
    pub delay: init_delay_op,
}

// Init command operation types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum init_op_types {
    INIT_OP_READ,
    INIT_OP_WRITE,
    INIT_OP_IF_MODE,
    INIT_OP_IF_PHASE,
    INIT_OP_DELAY,
    INIT_OP_CALLBACK,
    MAX_INIT_OP_TYPES
}

// init polling types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum init_poll_types {
    INIT_POLL_NONE,
    INIT_POLL_EQ,
    INIT_POLL_OR,
    INIT_POLL_AND,
    MAX_INIT_POLL_TYPES
}

// init source types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum init_source_types {
    INIT_SRC_INLINE,
    INIT_SRC_ZEROS,
    INIT_SRC_ARRAY,
    INIT_SRC_RUNTIME,
    MAX_INIT_SOURCE_TYPES
}

// Internal RAM Offsets macro data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iro {
    pub base: u32,
    pub m1: u16,
    pub m2: u16,
    pub m3: u16,
    pub size: u16,
}

// Win 2
pub const GTT_BAR0_MAP_REG_IGU_CMD: c_uint = 0x00f000UL;
// Win 3
pub const GTT_BAR0_MAP_REG_TSDM_RAM: c_uint = 0x010000UL;
// Win 4
pub const GTT_BAR0_MAP_REG_MSDM_RAM: c_uint = 0x011000UL;
// Win 5
pub const GTT_BAR0_MAP_REG_MSDM_RAM_1024: c_uint = 0x012000UL;
// Win 6
pub const GTT_BAR0_MAP_REG_MSDM_RAM_2048: c_uint = 0x013000UL;
// Win 7
pub const GTT_BAR0_MAP_REG_USDM_RAM: c_uint = 0x014000UL;
// Win 8
pub const GTT_BAR0_MAP_REG_USDM_RAM_1024: c_uint = 0x015000UL;
// Win 9
pub const GTT_BAR0_MAP_REG_USDM_RAM_2048: c_uint = 0x016000UL;
// Win 10
pub const GTT_BAR0_MAP_REG_XSDM_RAM: c_uint = 0x017000UL;
// Win 11
pub const GTT_BAR0_MAP_REG_XSDM_RAM_1024: c_uint = 0x018000UL;
// Win 12
pub const GTT_BAR0_MAP_REG_YSDM_RAM: c_uint = 0x019000UL;
// Win 13
pub const GTT_BAR0_MAP_REG_PSDM_RAM: c_uint = 0x01a000UL;
// Returns the VOQ based on port and TC

//
// qed_qm_pf_mem_size(): Prepare QM ILT sizes.
//
// @num_pf_cids: Number of connections used by this PF.
// @num_vf_cids: Number of connections used by VFs of this PF.
// @num_tids: Number of tasks used by this PF.
// @num_pf_pqs: Number of PQs used by this PF.
// @num_vf_pqs: Number of PQs used by VFs of this PF.
//
// Return: The required host memory size in 4KB units.
//
// Returns the required host memory size in 4KB units.
// Must be called before all QM init HSI functions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_qm_common_rt_init_params {
    pub max_ports_per_engine: u8,
    pub max_phys_tcs_per_port: u8,
    pub pf_rl_en: bool,
    pub pf_wfq_en: bool,
    pub global_rl_en: bool,
    pub vport_wfq_en: bool,
    pub port_params: *mut init_qm_port_params,
}

//
// qed_qm_common_rt_init(): Prepare QM runtime init values for the
// engine phase.
//
// @p_hwfn: HW device data.
// @p_params: Parameters.
//
// Return: 0 on success, -1 on error.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_qm_pf_rt_init_params {
    pub port_id: u8,
    pub pf_id: u8,
    pub max_phys_tcs_per_port: u8,
    pub is_pf_loading: bool,
    pub num_pf_cids: u32,
    pub num_vf_cids: u32,
    pub num_tids: u32,
    pub start_pq: u16,
    pub num_pf_pqs: u16,
    pub num_vf_pqs: u16,
    pub start_vport: u16,
    pub num_vports: u16,
    pub start_rl: u16,
    pub num_rls: u16,
    pub pf_wfq: u16,
    pub pf_rl: u32,
    pub link_speed: u32,
    pub pq_params: *mut init_qm_pq_params,
    pub vport_params: *mut init_qm_vport_params,
    pub rl_params: *mut init_qm_rl_params,
}

//
// qed_qm_pf_rt_init(): Prepare QM runtime init values for the PF phase.
//
// @p_hwfn:  HW device data.
// @p_ptt: Ptt window used for writing the registers
// @p_params: Parameters.
//
// Return: 0 on success, -1 on error.
//
// qed_init_pf_wfq(): Initializes the WFQ weight of the specified PF.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers
// @pf_id: PF ID
// @pf_wfq: WFQ weight. Must be non-zero.
//
// Return: 0 on success, -1 on error.
//
// qed_init_pf_rl(): Initializes the rate limit of the specified PF
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @pf_id: PF ID.
// @pf_rl: rate limit in Mb/sec units
//
// Return: 0 on success, -1 on error.
//
// qed_init_vport_wfq(): Initializes the WFQ weight of the specified VPORT
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers
// @first_tx_pq_id: An array containing the first Tx PQ ID associated
// with the VPORT for each TC. This array is filled by
// qed_qm_pf_rt_init
// @wfq: WFQ weight. Must be non-zero.
//
// Return: 0 on success, -1 on error.
//
// qed_init_vport_tc_wfq(): Initializes the WFQ weight of the specified
// VPORT and TC.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @first_tx_pq_id: The first Tx PQ ID associated with the VPORT and TC.
// (filled by qed_qm_pf_rt_init).
// @weight: VPORT+TC WFQ weight.
//
// Return: 0 on success, -1 on error.
//
// qed_init_global_rl():  Initializes the rate limit of the specified
// rate limiter.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @rl_id: RL ID.
// @rate_limit: Rate limit in Mb/sec units
// @vport_rl_type: Vport RL type.
//
// Return: 0 on success, -1 on error.
//
// qed_send_qm_stop_cmd(): Sends a stop command to the QM.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @is_release_cmd: true for release, false for stop.
// @is_tx_pq: true for Tx PQs, false for Other PQs.
// @start_pq: first PQ ID to stop
// @num_pqs: Number of PQs to stop, starting from start_pq.
//
// Return: Bool, true if successful, false if timeout occurred while waiting
// for QM command done.
//
// qed_set_vxlan_dest_port(): Initializes vxlan tunnel destination udp port.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @dest_port: vxlan destination udp port.
//
// Return: Void.
//
// qed_set_vxlan_enable(): Enable or disable VXLAN tunnel in HW.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @vxlan_enable: vxlan enable flag.
//
// Return: Void.
//
// qed_set_gre_enable(): Enable or disable GRE tunnel in HW.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @eth_gre_enable: Eth GRE enable flag.
// @ip_gre_enable: IP GRE enable flag.
//
// Return: Void.
//
// qed_set_geneve_dest_port(): Initializes geneve tunnel destination udp port
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @dest_port: Geneve destination udp port.
//
// Retur: Void.
//
// qed_set_geneve_enable(): Enable or disable GRE tunnel in HW.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @eth_geneve_enable: Eth GENEVE enable flag.
// @ip_geneve_enable: IP GENEVE enable flag.
//
// Return: Void.
//
// qed_gft_disable(): Disable GFT.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @pf_id: PF on which to disable GFT.
//
// Return: Void.
//
extern "C" {
    pub fn qed_gft_disable(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt, pf_id: u16);
}
//
// qed_gft_config(): Enable and configure HW for GFT.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @pf_id: PF on which to enable GFT.
// @tcp: Set profile tcp packets.
// @udp: Set profile udp  packet.
// @ipv4: Set profile ipv4 packet.
// @ipv6: Set profile ipv6 packet.
// @profile_type: Define packet same fields. Use enum gft_profile_type.
//
// Return: Void.
//
// qed_enable_context_validation(): Enable and configure context
// validation.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
//
// Return: Void.
//
pub const NUM_STORMS: c_int = 6;
//
// qed_get_protocol_type_str(): Get a string for Protocol type.
//
// @protocol_type: Protocol type (using enum protocol_type).
//
// Return: String.
//
// qed_get_ramrod_cmd_id_str(): Get a string for Ramrod command ID.
//
// @protocol_type: Protocol type (using enum protocol_type).
// @ramrod_cmd_id: Ramrod command ID (using per-protocol enum <protocol>_ramrod_cmd_id).
//
// Return: String.
//
// qed_set_rdma_error_level(): Sets the RDMA assert level.
// If the severity of the error will be
// above the level, the FW will assert.
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @assert_level: An array of assert levels for each storm.
//
// Return: Void.
//
// qed_fw_overlay_mem_alloc(): Allocates and fills the FW overlay memory.
//
// @p_hwfn: HW device data.
// @fw_overlay_in_buf: The input FW overlay buffer.
// @buf_size_in_bytes: The size of the input FW overlay buffer in bytes.
// must be aligned to dwords.
//
// Return: A pointer to the allocated overlays memory,
// or NULL in case of failures.
//
// qed_fw_overlay_init_ram(): Initializes the FW overlay RAM.
//
// @p_hwfn: HW device data.
// @p_ptt: Ptt window used for writing the registers.
// @fw_overlay_mem: the allocated FW overlay memory.
//
// Return: Void.
//
// qed_fw_overlay_mem_free(): Frees the FW overlay memory.
//
// @p_hwfn: HW device data.
// @fw_overlay_mem: The allocated FW overlay memory to free.
//
// Return: Void.
//
pub const PCICFG_OFFSET: c_uint = 0x2000;
pub const GRC_CONFIG_REG_PF_INIT_VF: c_uint = 0x624;
// First VF_NUM for PF is encoded in this register.
// The number of VFs assigned to a PF is assumed to be a multiple of 8.
// Software should program these bits based on Total Number of VFs programmed
// for each PF.
// Since registers from 0x000-0x7ff are spilt across functions, each PF will
// have the same location for the same 4 bits
//
pub const GRC_CR_PF_INIT_VF_PF_FIRST_VF_NUM_MASK: c_uint = 0xff;
// Runtime array offsets
pub const DORQ_REG_PF_MAX_ICID_0_RT_OFFSET: c_int = 0;
pub const DORQ_REG_PF_MAX_ICID_1_RT_OFFSET: c_int = 1;
pub const DORQ_REG_PF_MAX_ICID_2_RT_OFFSET: c_int = 2;
pub const DORQ_REG_PF_MAX_ICID_3_RT_OFFSET: c_int = 3;
pub const DORQ_REG_PF_MAX_ICID_4_RT_OFFSET: c_int = 4;
pub const DORQ_REG_PF_MAX_ICID_5_RT_OFFSET: c_int = 5;
pub const DORQ_REG_PF_MAX_ICID_6_RT_OFFSET: c_int = 6;
pub const DORQ_REG_PF_MAX_ICID_7_RT_OFFSET: c_int = 7;
pub const DORQ_REG_VF_MAX_ICID_0_RT_OFFSET: c_int = 8;
pub const DORQ_REG_VF_MAX_ICID_1_RT_OFFSET: c_int = 9;
pub const DORQ_REG_VF_MAX_ICID_2_RT_OFFSET: c_int = 10;
pub const DORQ_REG_VF_MAX_ICID_3_RT_OFFSET: c_int = 11;
pub const DORQ_REG_VF_MAX_ICID_4_RT_OFFSET: c_int = 12;
pub const DORQ_REG_VF_MAX_ICID_5_RT_OFFSET: c_int = 13;
pub const DORQ_REG_VF_MAX_ICID_6_RT_OFFSET: c_int = 14;
pub const DORQ_REG_VF_MAX_ICID_7_RT_OFFSET: c_int = 15;
pub const DORQ_REG_VF_ICID_BIT_SHIFT_NORM_RT_OFFSET: c_int = 16;
pub const DORQ_REG_PF_WAKE_ALL_RT_OFFSET: c_int = 17;
pub const DORQ_REG_TAG1_ETHERTYPE_RT_OFFSET: c_int = 18;
pub const IGU_REG_PF_CONFIGURATION_RT_OFFSET: c_int = 19;
pub const IGU_REG_VF_CONFIGURATION_RT_OFFSET: c_int = 20;
pub const IGU_REG_ATTN_MSG_ADDR_L_RT_OFFSET: c_int = 21;
pub const IGU_REG_ATTN_MSG_ADDR_H_RT_OFFSET: c_int = 22;
pub const IGU_REG_LEADING_EDGE_LATCH_RT_OFFSET: c_int = 23;
pub const IGU_REG_TRAILING_EDGE_LATCH_RT_OFFSET: c_int = 24;
pub const CAU_REG_CQE_AGG_UNIT_SIZE_RT_OFFSET: c_int = 25;
pub const CAU_REG_SB_VAR_MEMORY_RT_OFFSET: c_int = 26;
pub const CAU_REG_SB_VAR_MEMORY_RT_SIZE: c_int = 736;
pub const CAU_REG_SB_ADDR_MEMORY_RT_OFFSET: c_int = 762;
pub const CAU_REG_SB_ADDR_MEMORY_RT_SIZE: c_int = 736;
pub const CAU_REG_PI_MEMORY_RT_OFFSET: c_int = 1498;
pub const CAU_REG_PI_MEMORY_RT_SIZE: c_int = 4416;
pub const PRS_REG_SEARCH_RESP_INITIATOR_TYPE_RT_OFFSET: c_int = 5914;
pub const PRS_REG_TASK_ID_MAX_INITIATOR_PF_RT_OFFSET: c_int = 5915;
pub const PRS_REG_TASK_ID_MAX_INITIATOR_VF_RT_OFFSET: c_int = 5916;
pub const PRS_REG_TASK_ID_MAX_TARGET_PF_RT_OFFSET: c_int = 5917;
pub const PRS_REG_TASK_ID_MAX_TARGET_VF_RT_OFFSET: c_int = 5918;
pub const PRS_REG_SEARCH_TCP_RT_OFFSET: c_int = 5919;
pub const PRS_REG_SEARCH_FCOE_RT_OFFSET: c_int = 5920;
pub const PRS_REG_SEARCH_ROCE_RT_OFFSET: c_int = 5921;
pub const PRS_REG_ROCE_DEST_QP_MAX_VF_RT_OFFSET: c_int = 5922;
pub const PRS_REG_ROCE_DEST_QP_MAX_PF_RT_OFFSET: c_int = 5923;
pub const PRS_REG_SEARCH_OPENFLOW_RT_OFFSET: c_int = 5924;
pub const PRS_REG_SEARCH_NON_IP_AS_OPENFLOW_RT_OFFSET: c_int = 5925;
pub const PRS_REG_OPENFLOW_SUPPORT_ONLY_KNOWN_OVER_IP_RT_OFFSET: c_int = 5926;
pub const PRS_REG_OPENFLOW_SEARCH_KEY_MASK_RT_OFFSET: c_int = 5927;
pub const PRS_REG_TAG_ETHERTYPE_0_RT_OFFSET: c_int = 5928;
pub const PRS_REG_LIGHT_L2_ETHERTYPE_EN_RT_OFFSET: c_int = 5929;
pub const SRC_REG_FIRSTFREE_RT_OFFSET: c_int = 5930;
pub const SRC_REG_FIRSTFREE_RT_SIZE: c_int = 2;
pub const SRC_REG_LASTFREE_RT_OFFSET: c_int = 5932;
pub const SRC_REG_LASTFREE_RT_SIZE: c_int = 2;
pub const SRC_REG_COUNTFREE_RT_OFFSET: c_int = 5934;
pub const SRC_REG_NUMBER_HASH_BITS_RT_OFFSET: c_int = 5935;
pub const PSWRQ2_REG_CDUT_P_SIZE_RT_OFFSET: c_int = 5936;
pub const PSWRQ2_REG_CDUC_P_SIZE_RT_OFFSET: c_int = 5937;
pub const PSWRQ2_REG_TM_P_SIZE_RT_OFFSET: c_int = 5938;
pub const PSWRQ2_REG_QM_P_SIZE_RT_OFFSET: c_int = 5939;
pub const PSWRQ2_REG_SRC_P_SIZE_RT_OFFSET: c_int = 5940;
pub const PSWRQ2_REG_TSDM_P_SIZE_RT_OFFSET: c_int = 5941;
pub const PSWRQ2_REG_TM_FIRST_ILT_RT_OFFSET: c_int = 5942;
pub const PSWRQ2_REG_TM_LAST_ILT_RT_OFFSET: c_int = 5943;
pub const PSWRQ2_REG_QM_FIRST_ILT_RT_OFFSET: c_int = 5944;
pub const PSWRQ2_REG_QM_LAST_ILT_RT_OFFSET: c_int = 5945;
pub const PSWRQ2_REG_SRC_FIRST_ILT_RT_OFFSET: c_int = 5946;
pub const PSWRQ2_REG_SRC_LAST_ILT_RT_OFFSET: c_int = 5947;
pub const PSWRQ2_REG_CDUC_FIRST_ILT_RT_OFFSET: c_int = 5948;
pub const PSWRQ2_REG_CDUC_LAST_ILT_RT_OFFSET: c_int = 5949;
pub const PSWRQ2_REG_CDUT_FIRST_ILT_RT_OFFSET: c_int = 5950;
pub const PSWRQ2_REG_CDUT_LAST_ILT_RT_OFFSET: c_int = 5951;
pub const PSWRQ2_REG_TSDM_FIRST_ILT_RT_OFFSET: c_int = 5952;
pub const PSWRQ2_REG_TSDM_LAST_ILT_RT_OFFSET: c_int = 5953;
pub const PSWRQ2_REG_TM_NUMBER_OF_PF_BLOCKS_RT_OFFSET: c_int = 5954;
pub const PSWRQ2_REG_CDUT_NUMBER_OF_PF_BLOCKS_RT_OFFSET: c_int = 5955;
pub const PSWRQ2_REG_CDUC_NUMBER_OF_PF_BLOCKS_RT_OFFSET: c_int = 5956;
pub const PSWRQ2_REG_TM_VF_BLOCKS_RT_OFFSET: c_int = 5957;
pub const PSWRQ2_REG_CDUT_VF_BLOCKS_RT_OFFSET: c_int = 5958;
pub const PSWRQ2_REG_CDUC_VF_BLOCKS_RT_OFFSET: c_int = 5959;
pub const PSWRQ2_REG_TM_BLOCKS_FACTOR_RT_OFFSET: c_int = 5960;
pub const PSWRQ2_REG_CDUT_BLOCKS_FACTOR_RT_OFFSET: c_int = 5961;
pub const PSWRQ2_REG_CDUC_BLOCKS_FACTOR_RT_OFFSET: c_int = 5962;
pub const PSWRQ2_REG_VF_BASE_RT_OFFSET: c_int = 5963;
pub const PSWRQ2_REG_VF_LAST_ILT_RT_OFFSET: c_int = 5964;
pub const PSWRQ2_REG_DRAM_ALIGN_WR_RT_OFFSET: c_int = 5965;
pub const PSWRQ2_REG_DRAM_ALIGN_RD_RT_OFFSET: c_int = 5966;
pub const PSWRQ2_REG_ILT_MEMORY_RT_OFFSET: c_int = 5967;
pub const PSWRQ2_REG_ILT_MEMORY_RT_SIZE: c_int = 22000;
pub const PGLUE_REG_B_VF_BASE_RT_OFFSET: c_int = 27967;
pub const PGLUE_REG_B_MSDM_OFFSET_MASK_B_RT_OFFSET: c_int = 27968;
pub const PGLUE_REG_B_MSDM_VF_SHIFT_B_RT_OFFSET: c_int = 27969;
pub const PGLUE_REG_B_CACHE_LINE_SIZE_RT_OFFSET: c_int = 27970;
pub const PGLUE_REG_B_PF_BAR0_SIZE_RT_OFFSET: c_int = 27971;
pub const PGLUE_REG_B_PF_BAR1_SIZE_RT_OFFSET: c_int = 27972;
pub const PGLUE_REG_B_VF_BAR1_SIZE_RT_OFFSET: c_int = 27973;
pub const TM_REG_VF_ENABLE_CONN_RT_OFFSET: c_int = 27974;
pub const TM_REG_PF_ENABLE_CONN_RT_OFFSET: c_int = 27975;
pub const TM_REG_PF_ENABLE_TASK_RT_OFFSET: c_int = 27976;
pub const TM_REG_GROUP_SIZE_RESOLUTION_CONN_RT_OFFSET: c_int = 27977;
pub const TM_REG_GROUP_SIZE_RESOLUTION_TASK_RT_OFFSET: c_int = 27978;
pub const TM_REG_CONFIG_CONN_MEM_RT_OFFSET: c_int = 27979;
pub const TM_REG_CONFIG_CONN_MEM_RT_SIZE: c_int = 416;
pub const TM_REG_CONFIG_TASK_MEM_RT_OFFSET: c_int = 28395;
pub const TM_REG_CONFIG_TASK_MEM_RT_SIZE: c_int = 512;
pub const QM_REG_MAXPQSIZE_0_RT_OFFSET: c_int = 28907;
pub const QM_REG_MAXPQSIZE_1_RT_OFFSET: c_int = 28908;
pub const QM_REG_MAXPQSIZE_2_RT_OFFSET: c_int = 28909;
pub const QM_REG_MAXPQSIZETXSEL_0_RT_OFFSET: c_int = 28910;
pub const QM_REG_MAXPQSIZETXSEL_1_RT_OFFSET: c_int = 28911;
pub const QM_REG_MAXPQSIZETXSEL_2_RT_OFFSET: c_int = 28912;
pub const QM_REG_MAXPQSIZETXSEL_3_RT_OFFSET: c_int = 28913;
pub const QM_REG_MAXPQSIZETXSEL_4_RT_OFFSET: c_int = 28914;
pub const QM_REG_MAXPQSIZETXSEL_5_RT_OFFSET: c_int = 28915;
pub const QM_REG_MAXPQSIZETXSEL_6_RT_OFFSET: c_int = 28916;
pub const QM_REG_MAXPQSIZETXSEL_7_RT_OFFSET: c_int = 28917;
pub const QM_REG_MAXPQSIZETXSEL_8_RT_OFFSET: c_int = 28918;
pub const QM_REG_MAXPQSIZETXSEL_9_RT_OFFSET: c_int = 28919;
pub const QM_REG_MAXPQSIZETXSEL_10_RT_OFFSET: c_int = 28920;
pub const QM_REG_MAXPQSIZETXSEL_11_RT_OFFSET: c_int = 28921;
pub const QM_REG_MAXPQSIZETXSEL_12_RT_OFFSET: c_int = 28922;
pub const QM_REG_MAXPQSIZETXSEL_13_RT_OFFSET: c_int = 28923;
pub const QM_REG_MAXPQSIZETXSEL_14_RT_OFFSET: c_int = 28924;
pub const QM_REG_MAXPQSIZETXSEL_15_RT_OFFSET: c_int = 28925;
pub const QM_REG_MAXPQSIZETXSEL_16_RT_OFFSET: c_int = 28926;
pub const QM_REG_MAXPQSIZETXSEL_17_RT_OFFSET: c_int = 28927;
pub const QM_REG_MAXPQSIZETXSEL_18_RT_OFFSET: c_int = 28928;
pub const QM_REG_MAXPQSIZETXSEL_19_RT_OFFSET: c_int = 28929;
pub const QM_REG_MAXPQSIZETXSEL_20_RT_OFFSET: c_int = 28930;
pub const QM_REG_MAXPQSIZETXSEL_21_RT_OFFSET: c_int = 28931;
pub const QM_REG_MAXPQSIZETXSEL_22_RT_OFFSET: c_int = 28932;
pub const QM_REG_MAXPQSIZETXSEL_23_RT_OFFSET: c_int = 28933;
pub const QM_REG_MAXPQSIZETXSEL_24_RT_OFFSET: c_int = 28934;
pub const QM_REG_MAXPQSIZETXSEL_25_RT_OFFSET: c_int = 28935;
pub const QM_REG_MAXPQSIZETXSEL_26_RT_OFFSET: c_int = 28936;
pub const QM_REG_MAXPQSIZETXSEL_27_RT_OFFSET: c_int = 28937;
pub const QM_REG_MAXPQSIZETXSEL_28_RT_OFFSET: c_int = 28938;
pub const QM_REG_MAXPQSIZETXSEL_29_RT_OFFSET: c_int = 28939;
pub const QM_REG_MAXPQSIZETXSEL_30_RT_OFFSET: c_int = 28940;
pub const QM_REG_MAXPQSIZETXSEL_31_RT_OFFSET: c_int = 28941;
pub const QM_REG_MAXPQSIZETXSEL_32_RT_OFFSET: c_int = 28942;
pub const QM_REG_MAXPQSIZETXSEL_33_RT_OFFSET: c_int = 28943;
pub const QM_REG_MAXPQSIZETXSEL_34_RT_OFFSET: c_int = 28944;
pub const QM_REG_MAXPQSIZETXSEL_35_RT_OFFSET: c_int = 28945;
pub const QM_REG_MAXPQSIZETXSEL_36_RT_OFFSET: c_int = 28946;
pub const QM_REG_MAXPQSIZETXSEL_37_RT_OFFSET: c_int = 28947;
pub const QM_REG_MAXPQSIZETXSEL_38_RT_OFFSET: c_int = 28948;
pub const QM_REG_MAXPQSIZETXSEL_39_RT_OFFSET: c_int = 28949;
pub const QM_REG_MAXPQSIZETXSEL_40_RT_OFFSET: c_int = 28950;
pub const QM_REG_MAXPQSIZETXSEL_41_RT_OFFSET: c_int = 28951;
pub const QM_REG_MAXPQSIZETXSEL_42_RT_OFFSET: c_int = 28952;
pub const QM_REG_MAXPQSIZETXSEL_43_RT_OFFSET: c_int = 28953;
pub const QM_REG_MAXPQSIZETXSEL_44_RT_OFFSET: c_int = 28954;
pub const QM_REG_MAXPQSIZETXSEL_45_RT_OFFSET: c_int = 28955;
pub const QM_REG_MAXPQSIZETXSEL_46_RT_OFFSET: c_int = 28956;
pub const QM_REG_MAXPQSIZETXSEL_47_RT_OFFSET: c_int = 28957;
pub const QM_REG_MAXPQSIZETXSEL_48_RT_OFFSET: c_int = 28958;
pub const QM_REG_MAXPQSIZETXSEL_49_RT_OFFSET: c_int = 28959;
pub const QM_REG_MAXPQSIZETXSEL_50_RT_OFFSET: c_int = 28960;
pub const QM_REG_MAXPQSIZETXSEL_51_RT_OFFSET: c_int = 28961;
pub const QM_REG_MAXPQSIZETXSEL_52_RT_OFFSET: c_int = 28962;
pub const QM_REG_MAXPQSIZETXSEL_53_RT_OFFSET: c_int = 28963;
pub const QM_REG_MAXPQSIZETXSEL_54_RT_OFFSET: c_int = 28964;
pub const QM_REG_MAXPQSIZETXSEL_55_RT_OFFSET: c_int = 28965;
pub const QM_REG_MAXPQSIZETXSEL_56_RT_OFFSET: c_int = 28966;
pub const QM_REG_MAXPQSIZETXSEL_57_RT_OFFSET: c_int = 28967;
pub const QM_REG_MAXPQSIZETXSEL_58_RT_OFFSET: c_int = 28968;
pub const QM_REG_MAXPQSIZETXSEL_59_RT_OFFSET: c_int = 28969;
pub const QM_REG_MAXPQSIZETXSEL_60_RT_OFFSET: c_int = 28970;
pub const QM_REG_MAXPQSIZETXSEL_61_RT_OFFSET: c_int = 28971;
pub const QM_REG_MAXPQSIZETXSEL_62_RT_OFFSET: c_int = 28972;
pub const QM_REG_MAXPQSIZETXSEL_63_RT_OFFSET: c_int = 28973;
pub const QM_REG_BASEADDROTHERPQ_RT_OFFSET: c_int = 28974;
pub const QM_REG_BASEADDROTHERPQ_RT_SIZE: c_int = 128;
pub const QM_REG_PTRTBLOTHER_RT_OFFSET: c_int = 29102;
pub const QM_REG_PTRTBLOTHER_RT_SIZE: c_int = 256;
pub const QM_REG_VOQCRDLINE_RT_OFFSET: c_int = 29358;
pub const QM_REG_VOQCRDLINE_RT_SIZE: c_int = 20;
pub const QM_REG_VOQINITCRDLINE_RT_OFFSET: c_int = 29378;
pub const QM_REG_VOQINITCRDLINE_RT_SIZE: c_int = 20;
pub const QM_REG_AFULLQMBYPTHRPFWFQ_RT_OFFSET: c_int = 29398;
pub const QM_REG_AFULLQMBYPTHRVPWFQ_RT_OFFSET: c_int = 29399;
pub const QM_REG_AFULLQMBYPTHRPFRL_RT_OFFSET: c_int = 29400;
pub const QM_REG_AFULLQMBYPTHRGLBLRL_RT_OFFSET: c_int = 29401;
pub const QM_REG_AFULLOPRTNSTCCRDMASK_RT_OFFSET: c_int = 29402;
pub const QM_REG_WRROTHERPQGRP_0_RT_OFFSET: c_int = 29403;
pub const QM_REG_WRROTHERPQGRP_1_RT_OFFSET: c_int = 29404;
pub const QM_REG_WRROTHERPQGRP_2_RT_OFFSET: c_int = 29405;
pub const QM_REG_WRROTHERPQGRP_3_RT_OFFSET: c_int = 29406;
pub const QM_REG_WRROTHERPQGRP_4_RT_OFFSET: c_int = 29407;
pub const QM_REG_WRROTHERPQGRP_5_RT_OFFSET: c_int = 29408;
pub const QM_REG_WRROTHERPQGRP_6_RT_OFFSET: c_int = 29409;
pub const QM_REG_WRROTHERPQGRP_7_RT_OFFSET: c_int = 29410;
pub const QM_REG_WRROTHERPQGRP_8_RT_OFFSET: c_int = 29411;
pub const QM_REG_WRROTHERPQGRP_9_RT_OFFSET: c_int = 29412;
pub const QM_REG_WRROTHERPQGRP_10_RT_OFFSET: c_int = 29413;
pub const QM_REG_WRROTHERPQGRP_11_RT_OFFSET: c_int = 29414;
pub const QM_REG_WRROTHERPQGRP_12_RT_OFFSET: c_int = 29415;
pub const QM_REG_WRROTHERPQGRP_13_RT_OFFSET: c_int = 29416;
pub const QM_REG_WRROTHERPQGRP_14_RT_OFFSET: c_int = 29417;
pub const QM_REG_WRROTHERPQGRP_15_RT_OFFSET: c_int = 29418;
pub const QM_REG_WRROTHERGRPWEIGHT_0_RT_OFFSET: c_int = 29419;
pub const QM_REG_WRROTHERGRPWEIGHT_1_RT_OFFSET: c_int = 29420;
pub const QM_REG_WRROTHERGRPWEIGHT_2_RT_OFFSET: c_int = 29421;
pub const QM_REG_WRROTHERGRPWEIGHT_3_RT_OFFSET: c_int = 29422;
pub const QM_REG_WRRTXGRPWEIGHT_0_RT_OFFSET: c_int = 29423;
pub const QM_REG_WRRTXGRPWEIGHT_1_RT_OFFSET: c_int = 29424;
pub const QM_REG_PQTX2PF_0_RT_OFFSET: c_int = 29425;
pub const QM_REG_PQTX2PF_1_RT_OFFSET: c_int = 29426;
pub const QM_REG_PQTX2PF_2_RT_OFFSET: c_int = 29427;
pub const QM_REG_PQTX2PF_3_RT_OFFSET: c_int = 29428;
pub const QM_REG_PQTX2PF_4_RT_OFFSET: c_int = 29429;
pub const QM_REG_PQTX2PF_5_RT_OFFSET: c_int = 29430;
pub const QM_REG_PQTX2PF_6_RT_OFFSET: c_int = 29431;
pub const QM_REG_PQTX2PF_7_RT_OFFSET: c_int = 29432;
pub const QM_REG_PQTX2PF_8_RT_OFFSET: c_int = 29433;
pub const QM_REG_PQTX2PF_9_RT_OFFSET: c_int = 29434;
pub const QM_REG_PQTX2PF_10_RT_OFFSET: c_int = 29435;
pub const QM_REG_PQTX2PF_11_RT_OFFSET: c_int = 29436;
pub const QM_REG_PQTX2PF_12_RT_OFFSET: c_int = 29437;
pub const QM_REG_PQTX2PF_13_RT_OFFSET: c_int = 29438;
pub const QM_REG_PQTX2PF_14_RT_OFFSET: c_int = 29439;
pub const QM_REG_PQTX2PF_15_RT_OFFSET: c_int = 29440;
pub const QM_REG_PQTX2PF_16_RT_OFFSET: c_int = 29441;
pub const QM_REG_PQTX2PF_17_RT_OFFSET: c_int = 29442;
pub const QM_REG_PQTX2PF_18_RT_OFFSET: c_int = 29443;
pub const QM_REG_PQTX2PF_19_RT_OFFSET: c_int = 29444;
pub const QM_REG_PQTX2PF_20_RT_OFFSET: c_int = 29445;
pub const QM_REG_PQTX2PF_21_RT_OFFSET: c_int = 29446;
pub const QM_REG_PQTX2PF_22_RT_OFFSET: c_int = 29447;
pub const QM_REG_PQTX2PF_23_RT_OFFSET: c_int = 29448;
pub const QM_REG_PQTX2PF_24_RT_OFFSET: c_int = 29449;
pub const QM_REG_PQTX2PF_25_RT_OFFSET: c_int = 29450;
pub const QM_REG_PQTX2PF_26_RT_OFFSET: c_int = 29451;
pub const QM_REG_PQTX2PF_27_RT_OFFSET: c_int = 29452;
pub const QM_REG_PQTX2PF_28_RT_OFFSET: c_int = 29453;
pub const QM_REG_PQTX2PF_29_RT_OFFSET: c_int = 29454;
pub const QM_REG_PQTX2PF_30_RT_OFFSET: c_int = 29455;
pub const QM_REG_PQTX2PF_31_RT_OFFSET: c_int = 29456;
pub const QM_REG_PQTX2PF_32_RT_OFFSET: c_int = 29457;
pub const QM_REG_PQTX2PF_33_RT_OFFSET: c_int = 29458;
pub const QM_REG_PQTX2PF_34_RT_OFFSET: c_int = 29459;
pub const QM_REG_PQTX2PF_35_RT_OFFSET: c_int = 29460;
pub const QM_REG_PQTX2PF_36_RT_OFFSET: c_int = 29461;
pub const QM_REG_PQTX2PF_37_RT_OFFSET: c_int = 29462;
pub const QM_REG_PQTX2PF_38_RT_OFFSET: c_int = 29463;
pub const QM_REG_PQTX2PF_39_RT_OFFSET: c_int = 29464;
pub const QM_REG_PQTX2PF_40_RT_OFFSET: c_int = 29465;
pub const QM_REG_PQTX2PF_41_RT_OFFSET: c_int = 29466;
pub const QM_REG_PQTX2PF_42_RT_OFFSET: c_int = 29467;
pub const QM_REG_PQTX2PF_43_RT_OFFSET: c_int = 29468;
pub const QM_REG_PQTX2PF_44_RT_OFFSET: c_int = 29469;
pub const QM_REG_PQTX2PF_45_RT_OFFSET: c_int = 29470;
pub const QM_REG_PQTX2PF_46_RT_OFFSET: c_int = 29471;
pub const QM_REG_PQTX2PF_47_RT_OFFSET: c_int = 29472;
pub const QM_REG_PQTX2PF_48_RT_OFFSET: c_int = 29473;
pub const QM_REG_PQTX2PF_49_RT_OFFSET: c_int = 29474;
pub const QM_REG_PQTX2PF_50_RT_OFFSET: c_int = 29475;
pub const QM_REG_PQTX2PF_51_RT_OFFSET: c_int = 29476;
pub const QM_REG_PQTX2PF_52_RT_OFFSET: c_int = 29477;
pub const QM_REG_PQTX2PF_53_RT_OFFSET: c_int = 29478;
pub const QM_REG_PQTX2PF_54_RT_OFFSET: c_int = 29479;
pub const QM_REG_PQTX2PF_55_RT_OFFSET: c_int = 29480;
pub const QM_REG_PQTX2PF_56_RT_OFFSET: c_int = 29481;
pub const QM_REG_PQTX2PF_57_RT_OFFSET: c_int = 29482;
pub const QM_REG_PQTX2PF_58_RT_OFFSET: c_int = 29483;
pub const QM_REG_PQTX2PF_59_RT_OFFSET: c_int = 29484;
pub const QM_REG_PQTX2PF_60_RT_OFFSET: c_int = 29485;
pub const QM_REG_PQTX2PF_61_RT_OFFSET: c_int = 29486;
pub const QM_REG_PQTX2PF_62_RT_OFFSET: c_int = 29487;
pub const QM_REG_PQTX2PF_63_RT_OFFSET: c_int = 29488;
pub const QM_REG_PQOTHER2PF_0_RT_OFFSET: c_int = 29489;
pub const QM_REG_PQOTHER2PF_1_RT_OFFSET: c_int = 29490;
pub const QM_REG_PQOTHER2PF_2_RT_OFFSET: c_int = 29491;
pub const QM_REG_PQOTHER2PF_3_RT_OFFSET: c_int = 29492;
pub const QM_REG_PQOTHER2PF_4_RT_OFFSET: c_int = 29493;
pub const QM_REG_PQOTHER2PF_5_RT_OFFSET: c_int = 29494;
pub const QM_REG_PQOTHER2PF_6_RT_OFFSET: c_int = 29495;
pub const QM_REG_PQOTHER2PF_7_RT_OFFSET: c_int = 29496;
pub const QM_REG_PQOTHER2PF_8_RT_OFFSET: c_int = 29497;
pub const QM_REG_PQOTHER2PF_9_RT_OFFSET: c_int = 29498;
pub const QM_REG_PQOTHER2PF_10_RT_OFFSET: c_int = 29499;
pub const QM_REG_PQOTHER2PF_11_RT_OFFSET: c_int = 29500;
pub const QM_REG_PQOTHER2PF_12_RT_OFFSET: c_int = 29501;
pub const QM_REG_PQOTHER2PF_13_RT_OFFSET: c_int = 29502;
pub const QM_REG_PQOTHER2PF_14_RT_OFFSET: c_int = 29503;
pub const QM_REG_PQOTHER2PF_15_RT_OFFSET: c_int = 29504;
pub const QM_REG_RLGLBLPERIOD_0_RT_OFFSET: c_int = 29505;
pub const QM_REG_RLGLBLPERIOD_1_RT_OFFSET: c_int = 29506;
pub const QM_REG_RLGLBLPERIODTIMER_0_RT_OFFSET: c_int = 29507;
pub const QM_REG_RLGLBLPERIODTIMER_1_RT_OFFSET: c_int = 29508;
pub const QM_REG_RLGLBLPERIODSEL_0_RT_OFFSET: c_int = 29509;
pub const QM_REG_RLGLBLPERIODSEL_1_RT_OFFSET: c_int = 29510;
pub const QM_REG_RLGLBLPERIODSEL_2_RT_OFFSET: c_int = 29511;
pub const QM_REG_RLGLBLPERIODSEL_3_RT_OFFSET: c_int = 29512;
pub const QM_REG_RLGLBLPERIODSEL_4_RT_OFFSET: c_int = 29513;
pub const QM_REG_RLGLBLPERIODSEL_5_RT_OFFSET: c_int = 29514;
pub const QM_REG_RLGLBLPERIODSEL_6_RT_OFFSET: c_int = 29515;
pub const QM_REG_RLGLBLPERIODSEL_7_RT_OFFSET: c_int = 29516;
pub const QM_REG_RLGLBLINCVAL_RT_OFFSET: c_int = 29517;
pub const QM_REG_RLGLBLINCVAL_RT_SIZE: c_int = 256;
pub const QM_REG_RLGLBLUPPERBOUND_RT_OFFSET: c_int = 29773;
pub const QM_REG_RLGLBLUPPERBOUND_RT_SIZE: c_int = 256;
pub const QM_REG_RLGLBLCRD_RT_OFFSET: c_int = 30029;
pub const QM_REG_RLGLBLCRD_RT_SIZE: c_int = 256;
pub const QM_REG_RLGLBLENABLE_RT_OFFSET: c_int = 30285;
pub const QM_REG_RLPFPERIOD_RT_OFFSET: c_int = 30286;
pub const QM_REG_RLPFPERIODTIMER_RT_OFFSET: c_int = 30287;
pub const QM_REG_RLPFINCVAL_RT_OFFSET: c_int = 30288;
pub const QM_REG_RLPFINCVAL_RT_SIZE: c_int = 16;
pub const QM_REG_RLPFUPPERBOUND_RT_OFFSET: c_int = 30304;
pub const QM_REG_RLPFUPPERBOUND_RT_SIZE: c_int = 16;
pub const QM_REG_RLPFCRD_RT_OFFSET: c_int = 30320;
pub const QM_REG_RLPFCRD_RT_SIZE: c_int = 16;
pub const QM_REG_RLPFENABLE_RT_OFFSET: c_int = 30336;
pub const QM_REG_RLPFVOQENABLE_RT_OFFSET: c_int = 30337;
pub const QM_REG_WFQPFWEIGHT_RT_OFFSET: c_int = 30338;
pub const QM_REG_WFQPFWEIGHT_RT_SIZE: c_int = 16;
pub const QM_REG_WFQPFUPPERBOUND_RT_OFFSET: c_int = 30354;
pub const QM_REG_WFQPFUPPERBOUND_RT_SIZE: c_int = 16;
pub const QM_REG_WFQPFCRD_RT_OFFSET: c_int = 30370;
pub const QM_REG_WFQPFCRD_RT_SIZE: c_int = 160;
pub const QM_REG_WFQPFENABLE_RT_OFFSET: c_int = 30530;
pub const QM_REG_WFQVPENABLE_RT_OFFSET: c_int = 30531;
pub const QM_REG_BASEADDRTXPQ_RT_OFFSET: c_int = 30532;
pub const QM_REG_BASEADDRTXPQ_RT_SIZE: c_int = 512;
pub const QM_REG_TXPQMAP_RT_OFFSET: c_int = 31044;
pub const QM_REG_TXPQMAP_RT_SIZE: c_int = 512;
pub const QM_REG_WFQVPWEIGHT_RT_OFFSET: c_int = 31556;
pub const QM_REG_WFQVPWEIGHT_RT_SIZE: c_int = 512;
pub const QM_REG_WFQVPUPPERBOUND_RT_OFFSET: c_int = 32068;
pub const QM_REG_WFQVPUPPERBOUND_RT_SIZE: c_int = 512;
pub const QM_REG_WFQVPCRD_RT_OFFSET: c_int = 32580;
pub const QM_REG_WFQVPCRD_RT_SIZE: c_int = 512;
pub const QM_REG_WFQVPMAP_RT_OFFSET: c_int = 33092;
pub const QM_REG_WFQVPMAP_RT_SIZE: c_int = 512;
pub const QM_REG_PTRTBLTX_RT_OFFSET: c_int = 33604;
pub const QM_REG_PTRTBLTX_RT_SIZE: c_int = 1024;
pub const QM_REG_WFQPFCRD_MSB_RT_OFFSET: c_int = 34628;
pub const QM_REG_WFQPFCRD_MSB_RT_SIZE: c_int = 160;
pub const NIG_REG_TAG_ETHERTYPE_0_RT_OFFSET: c_int = 34788;
pub const NIG_REG_BRB_GATE_DNTFWD_PORT_RT_OFFSET: c_int = 34789;
pub const NIG_REG_OUTER_TAG_VALUE_LIST0_RT_OFFSET: c_int = 34790;
pub const NIG_REG_OUTER_TAG_VALUE_LIST1_RT_OFFSET: c_int = 34791;
pub const NIG_REG_OUTER_TAG_VALUE_LIST2_RT_OFFSET: c_int = 34792;
pub const NIG_REG_OUTER_TAG_VALUE_LIST3_RT_OFFSET: c_int = 34793;
pub const NIG_REG_LLH_FUNC_TAGMAC_CLS_TYPE_RT_OFFSET: c_int = 34794;
pub const NIG_REG_LLH_FUNC_TAG_EN_RT_OFFSET: c_int = 34795;
pub const NIG_REG_LLH_FUNC_TAG_EN_RT_SIZE: c_int = 4;
pub const NIG_REG_LLH_FUNC_TAG_VALUE_RT_OFFSET: c_int = 34799;
pub const NIG_REG_LLH_FUNC_TAG_VALUE_RT_SIZE: c_int = 4;
pub const NIG_REG_LLH_FUNC_FILTER_VALUE_RT_OFFSET: c_int = 34803;
pub const NIG_REG_LLH_FUNC_FILTER_VALUE_RT_SIZE: c_int = 32;
pub const NIG_REG_LLH_FUNC_FILTER_EN_RT_OFFSET: c_int = 34835;
pub const NIG_REG_LLH_FUNC_FILTER_EN_RT_SIZE: c_int = 16;
pub const NIG_REG_LLH_FUNC_FILTER_MODE_RT_OFFSET: c_int = 34851;
pub const NIG_REG_LLH_FUNC_FILTER_MODE_RT_SIZE: c_int = 16;
pub const NIG_REG_LLH_FUNC_FILTER_PROTOCOL_TYPE_RT_OFFSET: c_int = 34867;
pub const NIG_REG_LLH_FUNC_FILTER_PROTOCOL_TYPE_RT_SIZE: c_int = 16;
pub const NIG_REG_LLH_FUNC_FILTER_HDR_SEL_RT_OFFSET: c_int = 34883;
pub const NIG_REG_LLH_FUNC_FILTER_HDR_SEL_RT_SIZE: c_int = 16;
pub const NIG_REG_TX_EDPM_CTRL_RT_OFFSET: c_int = 34899;
pub const NIG_REG_PPF_TO_ENGINE_SEL_RT_OFFSET: c_int = 34900;
pub const NIG_REG_PPF_TO_ENGINE_SEL_RT_SIZE: c_int = 8;
pub const CDU_REG_CID_ADDR_PARAMS_RT_OFFSET: c_int = 34908;
pub const CDU_REG_SEGMENT0_PARAMS_RT_OFFSET: c_int = 34909;
pub const CDU_REG_SEGMENT1_PARAMS_RT_OFFSET: c_int = 34910;
pub const CDU_REG_PF_SEG0_TYPE_OFFSET_RT_OFFSET: c_int = 34911;
pub const CDU_REG_PF_SEG1_TYPE_OFFSET_RT_OFFSET: c_int = 34912;
pub const CDU_REG_PF_SEG2_TYPE_OFFSET_RT_OFFSET: c_int = 34913;
pub const CDU_REG_PF_SEG3_TYPE_OFFSET_RT_OFFSET: c_int = 34914;
pub const CDU_REG_PF_FL_SEG0_TYPE_OFFSET_RT_OFFSET: c_int = 34915;
pub const CDU_REG_PF_FL_SEG1_TYPE_OFFSET_RT_OFFSET: c_int = 34916;
pub const CDU_REG_PF_FL_SEG2_TYPE_OFFSET_RT_OFFSET: c_int = 34917;
pub const CDU_REG_PF_FL_SEG3_TYPE_OFFSET_RT_OFFSET: c_int = 34918;
pub const CDU_REG_VF_SEG_TYPE_OFFSET_RT_OFFSET: c_int = 34919;
pub const CDU_REG_VF_FL_SEG_TYPE_OFFSET_RT_OFFSET: c_int = 34920;
pub const PBF_REG_TAG_ETHERTYPE_0_RT_OFFSET: c_int = 34921;
pub const PBF_REG_BTB_SHARED_AREA_SIZE_RT_OFFSET: c_int = 34922;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ0_RT_OFFSET: c_int = 34923;
pub const PBF_REG_BTB_GUARANTEED_VOQ0_RT_OFFSET: c_int = 34924;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ0_RT_OFFSET: c_int = 34925;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ1_RT_OFFSET: c_int = 34926;
pub const PBF_REG_BTB_GUARANTEED_VOQ1_RT_OFFSET: c_int = 34927;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ1_RT_OFFSET: c_int = 34928;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ2_RT_OFFSET: c_int = 34929;
pub const PBF_REG_BTB_GUARANTEED_VOQ2_RT_OFFSET: c_int = 34930;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ2_RT_OFFSET: c_int = 34931;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ3_RT_OFFSET: c_int = 34932;
pub const PBF_REG_BTB_GUARANTEED_VOQ3_RT_OFFSET: c_int = 34933;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ3_RT_OFFSET: c_int = 34934;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ4_RT_OFFSET: c_int = 34935;
pub const PBF_REG_BTB_GUARANTEED_VOQ4_RT_OFFSET: c_int = 34936;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ4_RT_OFFSET: c_int = 34937;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ5_RT_OFFSET: c_int = 34938;
pub const PBF_REG_BTB_GUARANTEED_VOQ5_RT_OFFSET: c_int = 34939;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ5_RT_OFFSET: c_int = 34940;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ6_RT_OFFSET: c_int = 34941;
pub const PBF_REG_BTB_GUARANTEED_VOQ6_RT_OFFSET: c_int = 34942;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ6_RT_OFFSET: c_int = 34943;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ7_RT_OFFSET: c_int = 34944;
pub const PBF_REG_BTB_GUARANTEED_VOQ7_RT_OFFSET: c_int = 34945;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ7_RT_OFFSET: c_int = 34946;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ8_RT_OFFSET: c_int = 34947;
pub const PBF_REG_BTB_GUARANTEED_VOQ8_RT_OFFSET: c_int = 34948;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ8_RT_OFFSET: c_int = 34949;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ9_RT_OFFSET: c_int = 34950;
pub const PBF_REG_BTB_GUARANTEED_VOQ9_RT_OFFSET: c_int = 34951;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ9_RT_OFFSET: c_int = 34952;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ10_RT_OFFSET: c_int = 34953;
pub const PBF_REG_BTB_GUARANTEED_VOQ10_RT_OFFSET: c_int = 34954;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ10_RT_OFFSET: c_int = 34955;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ11_RT_OFFSET: c_int = 34956;
pub const PBF_REG_BTB_GUARANTEED_VOQ11_RT_OFFSET: c_int = 34957;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ11_RT_OFFSET: c_int = 34958;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ12_RT_OFFSET: c_int = 34959;
pub const PBF_REG_BTB_GUARANTEED_VOQ12_RT_OFFSET: c_int = 34960;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ12_RT_OFFSET: c_int = 34961;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ13_RT_OFFSET: c_int = 34962;
pub const PBF_REG_BTB_GUARANTEED_VOQ13_RT_OFFSET: c_int = 34963;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ13_RT_OFFSET: c_int = 34964;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ14_RT_OFFSET: c_int = 34965;
pub const PBF_REG_BTB_GUARANTEED_VOQ14_RT_OFFSET: c_int = 34966;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ14_RT_OFFSET: c_int = 34967;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ15_RT_OFFSET: c_int = 34968;
pub const PBF_REG_BTB_GUARANTEED_VOQ15_RT_OFFSET: c_int = 34969;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ15_RT_OFFSET: c_int = 34970;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ16_RT_OFFSET: c_int = 34971;
pub const PBF_REG_BTB_GUARANTEED_VOQ16_RT_OFFSET: c_int = 34972;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ16_RT_OFFSET: c_int = 34973;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ17_RT_OFFSET: c_int = 34974;
pub const PBF_REG_BTB_GUARANTEED_VOQ17_RT_OFFSET: c_int = 34975;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ17_RT_OFFSET: c_int = 34976;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ18_RT_OFFSET: c_int = 34977;
pub const PBF_REG_BTB_GUARANTEED_VOQ18_RT_OFFSET: c_int = 34978;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ18_RT_OFFSET: c_int = 34979;
pub const PBF_REG_YCMD_QS_NUM_LINES_VOQ19_RT_OFFSET: c_int = 34980;
pub const PBF_REG_BTB_GUARANTEED_VOQ19_RT_OFFSET: c_int = 34981;
pub const PBF_REG_BTB_SHARED_AREA_SETUP_VOQ19_RT_OFFSET: c_int = 34982;
pub const XCM_REG_CON_PHY_Q3_RT_OFFSET: c_int = 34983;
pub const RUNTIME_ARRAY_SIZE: c_int = 34984;
// Init Callbacks
pub const DMAE_READY_CB: c_int = 0;
// The eth storm context for the Tstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_eth_conn_st_ctx {
    pub reserved: [__le32; 4],
}

// The eth storm context for the Pstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstorm_eth_conn_st_ctx {
    pub reserved: [__le32; 8],
}

// The eth storm context for the Xstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_eth_conn_st_ctx {
    pub reserved: [__le32; 60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_eth_conn_ag_ctx {
    pub reserved0: u8,
    pub state: u8,
    pub flags0: u8,
pub const XSTORM_ETH_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED1_SHIFT: c_int = 1;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED2_SHIFT: c_int = 2;
pub const XSTORM_ETH_CONN_AG_CTX_EXIST_IN_QM3_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_EXIST_IN_QM3_SHIFT: c_int = 3;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED3_SHIFT: c_int = 4;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED4_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED4_SHIFT: c_int = 5;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED5_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED5_SHIFT: c_int = 6;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED6_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED6_SHIFT: c_int = 7;
    pub flags1: u8,
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED7_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED7_SHIFT: c_int = 0;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED8_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED8_SHIFT: c_int = 1;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED9_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED9_SHIFT: c_int = 2;
pub const XSTORM_ETH_CONN_AG_CTX_BIT11_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_BIT11_SHIFT: c_int = 3;
pub const XSTORM_ETH_CONN_AG_CTX_E5_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_E5_RESERVED2_SHIFT: c_int = 4;
pub const XSTORM_ETH_CONN_AG_CTX_E5_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_E5_RESERVED3_SHIFT: c_int = 5;
pub const XSTORM_ETH_CONN_AG_CTX_TX_RULE_ACTIVE_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_TX_RULE_ACTIVE_SHIFT: c_int = 6;
pub const XSTORM_ETH_CONN_AG_CTX_DQ_CF_ACTIVE_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_DQ_CF_ACTIVE_SHIFT: c_int = 7;
    pub flags2: u8,
pub const XSTORM_ETH_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_CF0_SHIFT: c_int = 0;
pub const XSTORM_ETH_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_CF1_SHIFT: c_int = 2;
pub const XSTORM_ETH_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_CF2_SHIFT: c_int = 4;
pub const XSTORM_ETH_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_CF3_SHIFT: c_int = 6;
    pub flags3: u8,
pub const XSTORM_ETH_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_CF4_SHIFT: c_int = 0;
pub const XSTORM_ETH_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_CF5_SHIFT: c_int = 2;
pub const XSTORM_ETH_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_CF6_SHIFT: c_int = 4;
pub const XSTORM_ETH_CONN_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_CF7_SHIFT: c_int = 6;
    pub flags4: u8,
pub const XSTORM_ETH_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_CF8_SHIFT: c_int = 0;
pub const XSTORM_ETH_CONN_AG_CTX_CF9_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_CF9_SHIFT: c_int = 2;
pub const XSTORM_ETH_CONN_AG_CTX_CF10_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_CF10_SHIFT: c_int = 4;
pub const XSTORM_ETH_CONN_AG_CTX_CF11_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_CF11_SHIFT: c_int = 6;
    pub flags5: u8,
pub const XSTORM_ETH_CONN_AG_CTX_CF12_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_CF12_SHIFT: c_int = 0;
pub const XSTORM_ETH_CONN_AG_CTX_CF13_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_CF13_SHIFT: c_int = 2;
pub const XSTORM_ETH_CONN_AG_CTX_CF14_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_CF14_SHIFT: c_int = 4;
pub const XSTORM_ETH_CONN_AG_CTX_CF15_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_CF15_SHIFT: c_int = 6;
    pub flags6: u8,
pub const XSTORM_ETH_CONN_AG_CTX_GO_TO_BD_CONS_CF_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_GO_TO_BD_CONS_CF_SHIFT: c_int = 0;
pub const XSTORM_ETH_CONN_AG_CTX_MULTI_UNICAST_CF_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_MULTI_UNICAST_CF_SHIFT: c_int = 2;
pub const XSTORM_ETH_CONN_AG_CTX_DQ_CF_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_DQ_CF_SHIFT: c_int = 4;
pub const XSTORM_ETH_CONN_AG_CTX_TERMINATE_CF_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_TERMINATE_CF_SHIFT: c_int = 6;
    pub flags7: u8,
pub const XSTORM_ETH_CONN_AG_CTX_FLUSH_Q0_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_FLUSH_Q0_SHIFT: c_int = 0;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED10_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED10_SHIFT: c_int = 2;
pub const XSTORM_ETH_CONN_AG_CTX_SLOW_PATH_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_SLOW_PATH_SHIFT: c_int = 4;
pub const XSTORM_ETH_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_CF0EN_SHIFT: c_int = 6;
pub const XSTORM_ETH_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags8: u8,
pub const XSTORM_ETH_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_CF2EN_SHIFT: c_int = 0;
pub const XSTORM_ETH_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_CF3EN_SHIFT: c_int = 1;
pub const XSTORM_ETH_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_CF4EN_SHIFT: c_int = 2;
pub const XSTORM_ETH_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_CF5EN_SHIFT: c_int = 3;
pub const XSTORM_ETH_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_CF6EN_SHIFT: c_int = 4;
pub const XSTORM_ETH_CONN_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_CF7EN_SHIFT: c_int = 5;
pub const XSTORM_ETH_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_CF8EN_SHIFT: c_int = 6;
pub const XSTORM_ETH_CONN_AG_CTX_CF9EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_CF9EN_SHIFT: c_int = 7;
    pub flags9: u8,
pub const XSTORM_ETH_CONN_AG_CTX_CF10EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_CF10EN_SHIFT: c_int = 0;
pub const XSTORM_ETH_CONN_AG_CTX_CF11EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_CF11EN_SHIFT: c_int = 1;
pub const XSTORM_ETH_CONN_AG_CTX_CF12EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_CF12EN_SHIFT: c_int = 2;
pub const XSTORM_ETH_CONN_AG_CTX_CF13EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_CF13EN_SHIFT: c_int = 3;
pub const XSTORM_ETH_CONN_AG_CTX_CF14EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_CF14EN_SHIFT: c_int = 4;
pub const XSTORM_ETH_CONN_AG_CTX_CF15EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_CF15EN_SHIFT: c_int = 5;
pub const XSTORM_ETH_CONN_AG_CTX_GO_TO_BD_CONS_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_GO_TO_BD_CONS_CF_EN_SHIFT: c_int = 6;
pub const XSTORM_ETH_CONN_AG_CTX_MULTI_UNICAST_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_MULTI_UNICAST_CF_EN_SHIFT: c_int = 7;
    pub flags10: u8,
pub const XSTORM_ETH_CONN_AG_CTX_DQ_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_DQ_CF_EN_SHIFT: c_int = 0;
pub const XSTORM_ETH_CONN_AG_CTX_TERMINATE_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_TERMINATE_CF_EN_SHIFT: c_int = 1;
pub const XSTORM_ETH_CONN_AG_CTX_FLUSH_Q0_EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_FLUSH_Q0_EN_SHIFT: c_int = 2;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED11_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED11_SHIFT: c_int = 3;
pub const XSTORM_ETH_CONN_AG_CTX_SLOW_PATH_EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_SLOW_PATH_EN_SHIFT: c_int = 4;
pub const XSTORM_ETH_CONN_AG_CTX_TPH_ENABLE_EN_RESERVED_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_TPH_ENABLE_EN_RESERVED_SHIFT: c_int = 5;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED12_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED12_SHIFT: c_int = 6;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED13_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED13_SHIFT: c_int = 7;
    pub flags11: u8,
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED14_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED14_SHIFT: c_int = 0;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED15_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RESERVED15_SHIFT: c_int = 1;
pub const XSTORM_ETH_CONN_AG_CTX_TX_DEC_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_TX_DEC_RULE_EN_SHIFT: c_int = 2;
pub const XSTORM_ETH_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 3;
pub const XSTORM_ETH_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 4;
pub const XSTORM_ETH_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 5;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED1_SHIFT: c_int = 6;
pub const XSTORM_ETH_CONN_AG_CTX_RULE9EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RULE9EN_SHIFT: c_int = 7;
    pub flags12: u8,
pub const XSTORM_ETH_CONN_AG_CTX_RULE10EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RULE10EN_SHIFT: c_int = 0;
pub const XSTORM_ETH_CONN_AG_CTX_RULE11EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RULE11EN_SHIFT: c_int = 1;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED2_SHIFT: c_int = 2;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED3_SHIFT: c_int = 3;
pub const XSTORM_ETH_CONN_AG_CTX_RULE14EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RULE14EN_SHIFT: c_int = 4;
pub const XSTORM_ETH_CONN_AG_CTX_RULE15EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RULE15EN_SHIFT: c_int = 5;
pub const XSTORM_ETH_CONN_AG_CTX_RULE16EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RULE16EN_SHIFT: c_int = 6;
pub const XSTORM_ETH_CONN_AG_CTX_RULE17EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RULE17EN_SHIFT: c_int = 7;
    pub flags13: u8,
pub const XSTORM_ETH_CONN_AG_CTX_RULE18EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RULE18EN_SHIFT: c_int = 0;
pub const XSTORM_ETH_CONN_AG_CTX_RULE19EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_RULE19EN_SHIFT: c_int = 1;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED4_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED4_SHIFT: c_int = 2;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED5_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED5_SHIFT: c_int = 3;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED6_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED6_SHIFT: c_int = 4;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED7_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED7_SHIFT: c_int = 5;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED8_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED8_SHIFT: c_int = 6;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED9_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_A0_RESERVED9_SHIFT: c_int = 7;
    pub flags14: u8,
pub const XSTORM_ETH_CONN_AG_CTX_EDPM_USE_EXT_HDR_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_EDPM_USE_EXT_HDR_SHIFT: c_int = 0;
pub const XSTORM_ETH_CONN_AG_CTX_EDPM_SEND_RAW_L3L4_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_EDPM_SEND_RAW_L3L4_SHIFT: c_int = 1;
pub const XSTORM_ETH_CONN_AG_CTX_EDPM_INBAND_PROP_HDR_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_EDPM_INBAND_PROP_HDR_SHIFT: c_int = 2;
pub const XSTORM_ETH_CONN_AG_CTX_EDPM_SEND_EXT_TUNNEL_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_EDPM_SEND_EXT_TUNNEL_SHIFT: c_int = 3;
pub const XSTORM_ETH_CONN_AG_CTX_L2_EDPM_ENABLE_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_L2_EDPM_ENABLE_SHIFT: c_int = 4;
pub const XSTORM_ETH_CONN_AG_CTX_ROCE_EDPM_ENABLE_MASK: c_uint = 0x1;
pub const XSTORM_ETH_CONN_AG_CTX_ROCE_EDPM_ENABLE_SHIFT: c_int = 5;
pub const XSTORM_ETH_CONN_AG_CTX_TPH_ENABLE_MASK: c_uint = 0x3;
pub const XSTORM_ETH_CONN_AG_CTX_TPH_ENABLE_SHIFT: c_int = 6;
    pub edpm_event_id: u8,
    pub physical_q0: __le16,
    pub e5_reserved1: __le16,
    pub edpm_num_bds: __le16,
    pub tx_bd_cons: __le16,
    pub tx_bd_prod: __le16,
    pub updated_qm_pq_id: __le16,
    pub conn_dpi: __le16,
    pub byte3: u8,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub reg3: __le32,
    pub reg4: __le32,
    pub reg5: __le32,
    pub reg6: __le32,
    pub word7: __le16,
    pub word8: __le16,
    pub word9: __le16,
    pub word10: __le16,
    pub reg7: __le32,
    pub reg8: __le32,
    pub reg9: __le32,
    pub byte7: u8,
    pub byte8: u8,
    pub byte9: u8,
    pub byte10: u8,
    pub byte11: u8,
    pub byte12: u8,
    pub byte13: u8,
    pub byte14: u8,
    pub byte15: u8,
    pub e5_reserved: u8,
    pub word11: __le16,
    pub reg10: __le32,
    pub reg11: __le32,
    pub reg12: __le32,
    pub reg13: __le32,
    pub reg14: __le32,
    pub reg15: __le32,
    pub reg16: __le32,
    pub reg17: __le32,
    pub reg18: __le32,
    pub reg19: __le32,
    pub word12: __le16,
    pub word13: __le16,
    pub word14: __le16,
    pub word15: __le16,
}

// The eth storm context for the Ystorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_eth_conn_st_ctx {
    pub reserved: [__le32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_eth_conn_ag_ctx {
    pub byte0: u8,
    pub state: u8,
    pub flags0: u8,
pub const YSTORM_ETH_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const YSTORM_ETH_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const YSTORM_ETH_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const YSTORM_ETH_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const YSTORM_ETH_CONN_AG_CTX_TX_BD_CONS_UPD_CF_MASK: c_uint = 0x3;
pub const YSTORM_ETH_CONN_AG_CTX_TX_BD_CONS_UPD_CF_SHIFT: c_int = 2;
pub const YSTORM_ETH_CONN_AG_CTX_PMD_TERMINATE_CF_MASK: c_uint = 0x3;
pub const YSTORM_ETH_CONN_AG_CTX_PMD_TERMINATE_CF_SHIFT: c_int = 4;
pub const YSTORM_ETH_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const YSTORM_ETH_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const YSTORM_ETH_CONN_AG_CTX_TX_BD_CONS_UPD_CF_EN_MASK: c_uint = 0x1;
pub const YSTORM_ETH_CONN_AG_CTX_TX_BD_CONS_UPD_CF_EN_SHIFT: c_int = 0;
pub const YSTORM_ETH_CONN_AG_CTX_PMD_TERMINATE_CF_EN_MASK: c_uint = 0x1;
pub const YSTORM_ETH_CONN_AG_CTX_PMD_TERMINATE_CF_EN_SHIFT: c_int = 1;
pub const YSTORM_ETH_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const YSTORM_ETH_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const YSTORM_ETH_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const YSTORM_ETH_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const YSTORM_ETH_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const YSTORM_ETH_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const YSTORM_ETH_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const YSTORM_ETH_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const YSTORM_ETH_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const YSTORM_ETH_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const YSTORM_ETH_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const YSTORM_ETH_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub tx_q0_int_coallecing_timeset: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub terminate_spqe: __le32,
    pub reg1: __le32,
    pub tx_bd_cons_upd: __le16,
    pub word2: __le16,
    pub word3: __le16,
    pub word4: __le16,
    pub reg2: __le32,
    pub reg3: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_eth_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const TSTORM_ETH_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const TSTORM_ETH_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const TSTORM_ETH_CONN_AG_CTX_BIT2_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_BIT2_SHIFT: c_int = 2;
pub const TSTORM_ETH_CONN_AG_CTX_BIT3_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_BIT3_SHIFT: c_int = 3;
pub const TSTORM_ETH_CONN_AG_CTX_BIT4_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_BIT4_SHIFT: c_int = 4;
pub const TSTORM_ETH_CONN_AG_CTX_BIT5_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_BIT5_SHIFT: c_int = 5;
pub const TSTORM_ETH_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const TSTORM_ETH_CONN_AG_CTX_CF0_SHIFT: c_int = 6;
    pub flags1: u8,
pub const TSTORM_ETH_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const TSTORM_ETH_CONN_AG_CTX_CF1_SHIFT: c_int = 0;
pub const TSTORM_ETH_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const TSTORM_ETH_CONN_AG_CTX_CF2_SHIFT: c_int = 2;
pub const TSTORM_ETH_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const TSTORM_ETH_CONN_AG_CTX_CF3_SHIFT: c_int = 4;
pub const TSTORM_ETH_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const TSTORM_ETH_CONN_AG_CTX_CF4_SHIFT: c_int = 6;
    pub flags2: u8,
pub const TSTORM_ETH_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const TSTORM_ETH_CONN_AG_CTX_CF5_SHIFT: c_int = 0;
pub const TSTORM_ETH_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const TSTORM_ETH_CONN_AG_CTX_CF6_SHIFT: c_int = 2;
pub const TSTORM_ETH_CONN_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const TSTORM_ETH_CONN_AG_CTX_CF7_SHIFT: c_int = 4;
pub const TSTORM_ETH_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const TSTORM_ETH_CONN_AG_CTX_CF8_SHIFT: c_int = 6;
    pub flags3: u8,
pub const TSTORM_ETH_CONN_AG_CTX_CF9_MASK: c_uint = 0x3;
pub const TSTORM_ETH_CONN_AG_CTX_CF9_SHIFT: c_int = 0;
pub const TSTORM_ETH_CONN_AG_CTX_CF10_MASK: c_uint = 0x3;
pub const TSTORM_ETH_CONN_AG_CTX_CF10_SHIFT: c_int = 2;
pub const TSTORM_ETH_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_CF0EN_SHIFT: c_int = 4;
pub const TSTORM_ETH_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_CF1EN_SHIFT: c_int = 5;
pub const TSTORM_ETH_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_CF2EN_SHIFT: c_int = 6;
pub const TSTORM_ETH_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_CF3EN_SHIFT: c_int = 7;
    pub flags4: u8,
pub const TSTORM_ETH_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_CF4EN_SHIFT: c_int = 0;
pub const TSTORM_ETH_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_CF5EN_SHIFT: c_int = 1;
pub const TSTORM_ETH_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_CF6EN_SHIFT: c_int = 2;
pub const TSTORM_ETH_CONN_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_CF7EN_SHIFT: c_int = 3;
pub const TSTORM_ETH_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_CF8EN_SHIFT: c_int = 4;
pub const TSTORM_ETH_CONN_AG_CTX_CF9EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_CF9EN_SHIFT: c_int = 5;
pub const TSTORM_ETH_CONN_AG_CTX_CF10EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_CF10EN_SHIFT: c_int = 6;
pub const TSTORM_ETH_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 7;
    pub flags5: u8,
pub const TSTORM_ETH_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 0;
pub const TSTORM_ETH_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const TSTORM_ETH_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const TSTORM_ETH_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const TSTORM_ETH_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const TSTORM_ETH_CONN_AG_CTX_RX_BD_EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_RX_BD_EN_SHIFT: c_int = 5;
pub const TSTORM_ETH_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const TSTORM_ETH_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const TSTORM_ETH_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub reg3: __le32,
    pub reg4: __le32,
    pub reg5: __le32,
    pub reg6: __le32,
    pub reg7: __le32,
    pub reg8: __le32,
    pub byte2: u8,
    pub byte3: u8,
    pub rx_bd_cons: __le16,
    pub byte4: u8,
    pub byte5: u8,
    pub rx_bd_prod: __le16,
    pub word2: __le16,
    pub word3: __le16,
    pub reg9: __le32,
    pub reg10: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_eth_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const USTORM_ETH_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const USTORM_ETH_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const USTORM_ETH_CONN_AG_CTX_TX_PMD_TERMINATE_CF_MASK: c_uint = 0x3;
pub const USTORM_ETH_CONN_AG_CTX_TX_PMD_TERMINATE_CF_SHIFT: c_int = 2;
pub const USTORM_ETH_CONN_AG_CTX_RX_PMD_TERMINATE_CF_MASK: c_uint = 0x3;
pub const USTORM_ETH_CONN_AG_CTX_RX_PMD_TERMINATE_CF_SHIFT: c_int = 4;
pub const USTORM_ETH_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const USTORM_ETH_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const USTORM_ETH_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const USTORM_ETH_CONN_AG_CTX_CF3_SHIFT: c_int = 0;
pub const USTORM_ETH_CONN_AG_CTX_TX_ARM_CF_MASK: c_uint = 0x3;
pub const USTORM_ETH_CONN_AG_CTX_TX_ARM_CF_SHIFT: c_int = 2;
pub const USTORM_ETH_CONN_AG_CTX_RX_ARM_CF_MASK: c_uint = 0x3;
pub const USTORM_ETH_CONN_AG_CTX_RX_ARM_CF_SHIFT: c_int = 4;
pub const USTORM_ETH_CONN_AG_CTX_TX_BD_CONS_UPD_CF_MASK: c_uint = 0x3;
pub const USTORM_ETH_CONN_AG_CTX_TX_BD_CONS_UPD_CF_SHIFT: c_int = 6;
    pub flags2: u8,
pub const USTORM_ETH_CONN_AG_CTX_TX_PMD_TERMINATE_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_TX_PMD_TERMINATE_CF_EN_SHIFT: c_int = 0;
pub const USTORM_ETH_CONN_AG_CTX_RX_PMD_TERMINATE_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_RX_PMD_TERMINATE_CF_EN_SHIFT: c_int = 1;
pub const USTORM_ETH_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const USTORM_ETH_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_CF3EN_SHIFT: c_int = 3;
pub const USTORM_ETH_CONN_AG_CTX_TX_ARM_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_TX_ARM_CF_EN_SHIFT: c_int = 4;
pub const USTORM_ETH_CONN_AG_CTX_RX_ARM_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_RX_ARM_CF_EN_SHIFT: c_int = 5;
pub const USTORM_ETH_CONN_AG_CTX_TX_BD_CONS_UPD_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_TX_BD_CONS_UPD_CF_EN_SHIFT: c_int = 6;
pub const USTORM_ETH_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 7;
    pub flags3: u8,
pub const USTORM_ETH_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 0;
pub const USTORM_ETH_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const USTORM_ETH_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const USTORM_ETH_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const USTORM_ETH_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const USTORM_ETH_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 5;
pub const USTORM_ETH_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const USTORM_ETH_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const USTORM_ETH_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub tx_bd_cons: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub tx_int_coallecing_timeset: __le32,
    pub tx_drv_bd_cons: __le16,
    pub rx_drv_cqe_cons: __le16,
}

// The eth storm context for the Ustorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_eth_conn_st_ctx {
    pub reserved: [__le32; 40],
}

// The eth storm context for the Mstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_eth_conn_st_ctx {
    pub reserved: [__le32; 8],
}

// eth connection context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_conn_context {
    pub tstorm_st_context: tstorm_eth_conn_st_ctx,
    pub tstorm_st_padding: [regpair; 2],
    pub pstorm_st_context: pstorm_eth_conn_st_ctx,
    pub xstorm_st_context: xstorm_eth_conn_st_ctx,
    pub xstorm_ag_context: xstorm_eth_conn_ag_ctx,
    pub tstorm_ag_context: tstorm_eth_conn_ag_ctx,
    pub ystorm_st_context: ystorm_eth_conn_st_ctx,
    pub ystorm_ag_context: ystorm_eth_conn_ag_ctx,
    pub ustorm_ag_context: ustorm_eth_conn_ag_ctx,
    pub ustorm_st_context: ustorm_eth_conn_st_ctx,
    pub mstorm_st_context: mstorm_eth_conn_st_ctx,
}

// Ethernet filter types: mac/vlan/pair
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_error_code {
    ETH_OK = 0x00,
    ETH_FILTERS_MAC_ADD_FAIL_FULL,
    ETH_FILTERS_MAC_ADD_FAIL_FULL_MTT2,
    ETH_FILTERS_MAC_ADD_FAIL_DUP_MTT2,
    ETH_FILTERS_MAC_ADD_FAIL_DUP_STT2,
    ETH_FILTERS_MAC_DEL_FAIL_NOF,
    ETH_FILTERS_MAC_DEL_FAIL_NOF_MTT2,
    ETH_FILTERS_MAC_DEL_FAIL_NOF_STT2,
    ETH_FILTERS_MAC_ADD_FAIL_ZERO_MAC,
    ETH_FILTERS_VLAN_ADD_FAIL_FULL,
    ETH_FILTERS_VLAN_ADD_FAIL_DUP,
    ETH_FILTERS_VLAN_DEL_FAIL_NOF,
    ETH_FILTERS_VLAN_DEL_FAIL_NOF_TT1,
    ETH_FILTERS_PAIR_ADD_FAIL_DUP,
    ETH_FILTERS_PAIR_ADD_FAIL_FULL,
    ETH_FILTERS_PAIR_ADD_FAIL_FULL_MAC,
    ETH_FILTERS_PAIR_DEL_FAIL_NOF,
    ETH_FILTERS_PAIR_DEL_FAIL_NOF_TT1,
    ETH_FILTERS_PAIR_ADD_FAIL_ZERO_MAC,
    ETH_FILTERS_VNI_ADD_FAIL_FULL,
    ETH_FILTERS_VNI_ADD_FAIL_DUP,
    ETH_FILTERS_GFT_UPDATE_FAIL,
    ETH_RX_QUEUE_FAIL_LOAD_VF_DATA,
    ETH_FILTERS_GFS_ADD_FILTER_FAIL_MAX_HOPS,
    ETH_FILTERS_GFS_ADD_FILTER_FAIL_NO_FREE_ENRTY,
    ETH_FILTERS_GFS_ADD_FILTER_FAIL_ALREADY_EXISTS,
    ETH_FILTERS_GFS_ADD_FILTER_FAIL_PCI_ERROR,
    ETH_FILTERS_GFS_ADD_FINLER_FAIL_MAGIC_NUM_ERROR,
    ETH_FILTERS_GFS_DEL_FILTER_FAIL_MAX_HOPS,
    ETH_FILTERS_GFS_DEL_FILTER_FAIL_NO_MATCH_ENRTY,
    ETH_FILTERS_GFS_DEL_FILTER_FAIL_PCI_ERROR,
    ETH_FILTERS_GFS_DEL_FILTER_FAIL_MAGIC_NUM_ERROR,
    MAX_ETH_ERROR_CODE
}

// Opcodes for the event ring
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_event_opcode {
    ETH_EVENT_UNUSED,
    ETH_EVENT_VPORT_START,
    ETH_EVENT_VPORT_UPDATE,
    ETH_EVENT_VPORT_STOP,
    ETH_EVENT_TX_QUEUE_START,
    ETH_EVENT_TX_QUEUE_STOP,
    ETH_EVENT_RX_QUEUE_START,
    ETH_EVENT_RX_QUEUE_UPDATE,
    ETH_EVENT_RX_QUEUE_STOP,
    ETH_EVENT_FILTERS_UPDATE,
    ETH_EVENT_RX_ADD_OPENFLOW_FILTER,
    ETH_EVENT_RX_DELETE_OPENFLOW_FILTER,
    ETH_EVENT_RX_CREATE_OPENFLOW_ACTION,
    ETH_EVENT_RX_ADD_UDP_FILTER,
    ETH_EVENT_RX_DELETE_UDP_FILTER,
    ETH_EVENT_RX_CREATE_GFT_ACTION,
    ETH_EVENT_RX_GFT_UPDATE_FILTER,
    ETH_EVENT_TX_QUEUE_UPDATE,
    ETH_EVENT_RGFS_ADD_FILTER,
    ETH_EVENT_RGFS_DEL_FILTER,
    ETH_EVENT_TGFS_ADD_FILTER,
    ETH_EVENT_TGFS_DEL_FILTER,
    ETH_EVENT_GFS_COUNTERS_REPORT_REQUEST,
    MAX_ETH_EVENT_OPCODE
}

// Classify rule types in E2/E3
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_filter_action {
    ETH_FILTER_ACTION_UNUSED,
    ETH_FILTER_ACTION_REMOVE,
    ETH_FILTER_ACTION_ADD,
    ETH_FILTER_ACTION_REMOVE_ALL,
    MAX_ETH_FILTER_ACTION
}

// Command for adding/removing a classification rule $$KEEP_ENDIANNESS$$
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_filter_cmd {
    pub type: u8,
    pub vport_id: u8,
    pub action: u8,
    pub reserved0: u8,
    pub vni: __le32,
    pub mac_lsb: __le16,
    pub mac_mid: __le16,
    pub mac_msb: __le16,
    pub vlan_id: __le16,
}

// $$KEEP_ENDIANNESS$$
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_filter_cmd_header {
    pub rx: u8,
    pub tx: u8,
    pub cmd_cnt: u8,
    pub assert_on_error: u8,
    pub reserved1: [u8; 4],
}

// Ethernet filter types: mac/vlan/pair
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_filter_type {
    ETH_FILTER_TYPE_UNUSED,
    ETH_FILTER_TYPE_MAC,
    ETH_FILTER_TYPE_VLAN,
    ETH_FILTER_TYPE_PAIR,
    ETH_FILTER_TYPE_INNER_MAC,
    ETH_FILTER_TYPE_INNER_VLAN,
    ETH_FILTER_TYPE_INNER_PAIR,
    ETH_FILTER_TYPE_INNER_MAC_VNI_PAIR,
    ETH_FILTER_TYPE_MAC_VNI_PAIR,
    ETH_FILTER_TYPE_VNI,
    MAX_ETH_FILTER_TYPE
}

// inner to inner vlan priority translation configurations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_in_to_in_pri_map_cfg {
    pub inner_vlan_pri_remap_en: u8,
    pub reserved: [u8; 7],
    pub non_rdma_in_to_in_pri_map: [u8; 8],
    pub rdma_in_to_in_pri_map: [u8; 8],
}

// Eth IPv4 Fragment Type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_ipv4_frag_type {
    ETH_IPV4_NOT_FRAG,
    ETH_IPV4_FIRST_FRAG,
    ETH_IPV4_NON_FIRST_FRAG,
    MAX_ETH_IPV4_FRAG_TYPE
}

// eth IPv4 Fragment Type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_ip_type {
    ETH_IPV4,
    ETH_IPV6,
    MAX_ETH_IP_TYPE
}

// Ethernet Ramrod Command IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_ramrod_cmd_id {
    ETH_RAMROD_UNUSED,
    ETH_RAMROD_VPORT_START,
    ETH_RAMROD_VPORT_UPDATE,
    ETH_RAMROD_VPORT_STOP,
    ETH_RAMROD_RX_QUEUE_START,
    ETH_RAMROD_RX_QUEUE_STOP,
    ETH_RAMROD_TX_QUEUE_START,
    ETH_RAMROD_TX_QUEUE_STOP,
    ETH_RAMROD_FILTERS_UPDATE,
    ETH_RAMROD_RX_QUEUE_UPDATE,
    ETH_RAMROD_RX_CREATE_OPENFLOW_ACTION,
    ETH_RAMROD_RX_ADD_OPENFLOW_FILTER,
    ETH_RAMROD_RX_DELETE_OPENFLOW_FILTER,
    ETH_RAMROD_RX_ADD_UDP_FILTER,
    ETH_RAMROD_RX_DELETE_UDP_FILTER,
    ETH_RAMROD_RX_CREATE_GFT_ACTION,
    ETH_RAMROD_RX_UPDATE_GFT_FILTER,
    ETH_RAMROD_TX_QUEUE_UPDATE,
    ETH_RAMROD_RGFS_FILTER_ADD,
    ETH_RAMROD_RGFS_FILTER_DEL,
    ETH_RAMROD_TGFS_FILTER_ADD,
    ETH_RAMROD_TGFS_FILTER_DEL,
    ETH_RAMROD_GFS_COUNTERS_REPORT_REQUEST,
    MAX_ETH_RAMROD_CMD_ID
}

// Return code from eth sp ramrods
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_return_code {
    pub value: u8,
pub const ETH_RETURN_CODE_ERR_CODE_MASK: c_uint = 0x3F;
pub const ETH_RETURN_CODE_ERR_CODE_SHIFT: c_int = 0;
pub const ETH_RETURN_CODE_RESERVED_MASK: c_uint = 0x1;
pub const ETH_RETURN_CODE_RESERVED_SHIFT: c_int = 6;
pub const ETH_RETURN_CODE_RX_TX_MASK: c_uint = 0x1;
pub const ETH_RETURN_CODE_RX_TX_SHIFT: c_int = 7;
}

// tx destination enum
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_tx_dst_mode_config_enum {
    ETH_TX_DST_MODE_CONFIG_DISABLE,
    ETH_TX_DST_MODE_CONFIG_FORWARD_DATA_IN_BD,
    ETH_TX_DST_MODE_CONFIG_FORWARD_DATA_IN_VPORT,
    MAX_ETH_TX_DST_MODE_CONFIG_ENUM
}

// What to do in case an error occurs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_tx_err {
    ETH_TX_ERR_DROP,
    ETH_TX_ERR_ASSERT_MALICIOUS,
    MAX_ETH_TX_ERR
}

// Array of the different error type behaviors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_err_vals {
    pub values: __le16,
pub const ETH_TX_ERR_VALS_ILLEGAL_VLAN_MODE_MASK: c_uint = 0x1;
pub const ETH_TX_ERR_VALS_ILLEGAL_VLAN_MODE_SHIFT: c_int = 0;
pub const ETH_TX_ERR_VALS_PACKET_TOO_SMALL_MASK: c_uint = 0x1;
pub const ETH_TX_ERR_VALS_PACKET_TOO_SMALL_SHIFT: c_int = 1;
pub const ETH_TX_ERR_VALS_ANTI_SPOOFING_ERR_MASK: c_uint = 0x1;
pub const ETH_TX_ERR_VALS_ANTI_SPOOFING_ERR_SHIFT: c_int = 2;
pub const ETH_TX_ERR_VALS_ILLEGAL_INBAND_TAGS_MASK: c_uint = 0x1;
pub const ETH_TX_ERR_VALS_ILLEGAL_INBAND_TAGS_SHIFT: c_int = 3;
pub const ETH_TX_ERR_VALS_VLAN_INSERTION_W_INBAND_TAG_MASK: c_uint = 0x1;
pub const ETH_TX_ERR_VALS_VLAN_INSERTION_W_INBAND_TAG_SHIFT: c_int = 4;
pub const ETH_TX_ERR_VALS_MTU_VIOLATION_MASK: c_uint = 0x1;
pub const ETH_TX_ERR_VALS_MTU_VIOLATION_SHIFT: c_int = 5;
pub const ETH_TX_ERR_VALS_ILLEGAL_CONTROL_FRAME_MASK: c_uint = 0x1;
pub const ETH_TX_ERR_VALS_ILLEGAL_CONTROL_FRAME_SHIFT: c_int = 6;
pub const ETH_TX_ERR_VALS_ILLEGAL_BD_FLAGS_MASK: c_uint = 0x1;
pub const ETH_TX_ERR_VALS_ILLEGAL_BD_FLAGS_SHIFT: c_int = 7;
pub const ETH_TX_ERR_VALS_RESERVED_MASK: c_uint = 0xFF;
pub const ETH_TX_ERR_VALS_RESERVED_SHIFT: c_int = 8;
}

// vport rss configuration data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_vport_rss_config {
    pub capabilities: __le16,
pub const ETH_VPORT_RSS_CONFIG_IPV4_CAPABILITY_MASK: c_uint = 0x1;
pub const ETH_VPORT_RSS_CONFIG_IPV4_CAPABILITY_SHIFT: c_int = 0;
pub const ETH_VPORT_RSS_CONFIG_IPV6_CAPABILITY_MASK: c_uint = 0x1;
pub const ETH_VPORT_RSS_CONFIG_IPV6_CAPABILITY_SHIFT: c_int = 1;
pub const ETH_VPORT_RSS_CONFIG_IPV4_TCP_CAPABILITY_MASK: c_uint = 0x1;
pub const ETH_VPORT_RSS_CONFIG_IPV4_TCP_CAPABILITY_SHIFT: c_int = 2;
pub const ETH_VPORT_RSS_CONFIG_IPV6_TCP_CAPABILITY_MASK: c_uint = 0x1;
pub const ETH_VPORT_RSS_CONFIG_IPV6_TCP_CAPABILITY_SHIFT: c_int = 3;
pub const ETH_VPORT_RSS_CONFIG_IPV4_UDP_CAPABILITY_MASK: c_uint = 0x1;
pub const ETH_VPORT_RSS_CONFIG_IPV4_UDP_CAPABILITY_SHIFT: c_int = 4;
pub const ETH_VPORT_RSS_CONFIG_IPV6_UDP_CAPABILITY_MASK: c_uint = 0x1;
pub const ETH_VPORT_RSS_CONFIG_IPV6_UDP_CAPABILITY_SHIFT: c_int = 5;
pub const ETH_VPORT_RSS_CONFIG_EN_5_TUPLE_CAPABILITY_MASK: c_uint = 0x1;
pub const ETH_VPORT_RSS_CONFIG_EN_5_TUPLE_CAPABILITY_SHIFT: c_int = 6;
pub const ETH_VPORT_RSS_CONFIG_RESERVED0_MASK: c_uint = 0x1FF;
pub const ETH_VPORT_RSS_CONFIG_RESERVED0_SHIFT: c_int = 7;
    pub rss_id: u8,
    pub rss_mode: u8,
    pub update_rss_key: u8,
    pub update_rss_ind_table: u8,
    pub update_rss_capabilities: u8,
    pub tbl_size: u8,
    pub ind_table_mask_valid: u8,
    pub reserved2: [u8; 3],
    pub indirection_table: [__le16; ETH_RSS_IND_TABLE_ENTRIES_NUM],
    pub ind_table_mask: [__le32; ETH_RSS_IND_TABLE_MASK_SIZE_REGS],
    pub rss_key: [__le32; ETH_RSS_KEY_SIZE_REGS],
    pub reserved3: __le32,
}

// eth vport RSS mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_vport_rss_mode {
    ETH_VPORT_RSS_MODE_DISABLED,
    ETH_VPORT_RSS_MODE_REGULAR,
    MAX_ETH_VPORT_RSS_MODE
}

// Command for setting classification flags for a vport $$KEEP_ENDIANNESS$$
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_vport_rx_mode {
    pub state: __le16,
pub const ETH_VPORT_RX_MODE_UCAST_DROP_ALL_MASK: c_uint = 0x1;
pub const ETH_VPORT_RX_MODE_UCAST_DROP_ALL_SHIFT: c_int = 0;
pub const ETH_VPORT_RX_MODE_UCAST_ACCEPT_ALL_MASK: c_uint = 0x1;
pub const ETH_VPORT_RX_MODE_UCAST_ACCEPT_ALL_SHIFT: c_int = 1;
pub const ETH_VPORT_RX_MODE_UCAST_ACCEPT_UNMATCHED_MASK: c_uint = 0x1;
pub const ETH_VPORT_RX_MODE_UCAST_ACCEPT_UNMATCHED_SHIFT: c_int = 2;
pub const ETH_VPORT_RX_MODE_MCAST_DROP_ALL_MASK: c_uint = 0x1;
pub const ETH_VPORT_RX_MODE_MCAST_DROP_ALL_SHIFT: c_int = 3;
pub const ETH_VPORT_RX_MODE_MCAST_ACCEPT_ALL_MASK: c_uint = 0x1;
pub const ETH_VPORT_RX_MODE_MCAST_ACCEPT_ALL_SHIFT: c_int = 4;
pub const ETH_VPORT_RX_MODE_BCAST_ACCEPT_ALL_MASK: c_uint = 0x1;
pub const ETH_VPORT_RX_MODE_BCAST_ACCEPT_ALL_SHIFT: c_int = 5;
pub const ETH_VPORT_RX_MODE_ACCEPT_ANY_VNI_MASK: c_uint = 0x1;
pub const ETH_VPORT_RX_MODE_ACCEPT_ANY_VNI_SHIFT: c_int = 6;
pub const ETH_VPORT_RX_MODE_RESERVED1_MASK: c_uint = 0x1FF;
pub const ETH_VPORT_RX_MODE_RESERVED1_SHIFT: c_int = 7;
}

// Command for setting tpa parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_vport_tpa_param {
    pub tpa_ipv4_en_flg: u8,
    pub tpa_ipv6_en_flg: u8,
    pub tpa_ipv4_tunn_en_flg: u8,
    pub tpa_ipv6_tunn_en_flg: u8,
    pub tpa_pkt_split_flg: u8,
    pub tpa_hdr_data_split_flg: u8,
    pub tpa_gro_consistent_flg: u8,
    pub tpa_max_aggs_num: u8,
    pub tpa_max_size: __le16,
    pub tpa_min_size_to_start: __le16,
    pub tpa_min_size_to_cont: __le16,
    pub max_buff_num: u8,
    pub reserved: u8,
}

// Command for setting classification flags for a vport $$KEEP_ENDIANNESS$$
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_vport_tx_mode {
    pub state: __le16,
pub const ETH_VPORT_TX_MODE_UCAST_DROP_ALL_MASK: c_uint = 0x1;
pub const ETH_VPORT_TX_MODE_UCAST_DROP_ALL_SHIFT: c_int = 0;
pub const ETH_VPORT_TX_MODE_UCAST_ACCEPT_ALL_MASK: c_uint = 0x1;
pub const ETH_VPORT_TX_MODE_UCAST_ACCEPT_ALL_SHIFT: c_int = 1;
pub const ETH_VPORT_TX_MODE_MCAST_DROP_ALL_MASK: c_uint = 0x1;
pub const ETH_VPORT_TX_MODE_MCAST_DROP_ALL_SHIFT: c_int = 2;
pub const ETH_VPORT_TX_MODE_MCAST_ACCEPT_ALL_MASK: c_uint = 0x1;
pub const ETH_VPORT_TX_MODE_MCAST_ACCEPT_ALL_SHIFT: c_int = 3;
pub const ETH_VPORT_TX_MODE_BCAST_ACCEPT_ALL_MASK: c_uint = 0x1;
pub const ETH_VPORT_TX_MODE_BCAST_ACCEPT_ALL_SHIFT: c_int = 4;
pub const ETH_VPORT_TX_MODE_RESERVED1_MASK: c_uint = 0x7FF;
pub const ETH_VPORT_TX_MODE_RESERVED1_SHIFT: c_int = 5;
}

// GFT filter update action type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gft_filter_update_action {
    GFT_ADD_FILTER,
    GFT_DELETE_FILTER,
    MAX_GFT_FILTER_UPDATE_ACTION
}

// Ramrod data for rx create gft action
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_create_gft_action_ramrod_data {
    pub vport_id: u8,
    pub reserved: [u8; 7],
}

// Ramrod data for rx create openflow action
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_create_openflow_action_ramrod_data {
    pub vport_id: u8,
    pub reserved: [u8; 7],
}

// Ramrod data for rx add openflow filter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_openflow_filter_ramrod_data {
    pub action_icid: __le16,
    pub priority: u8,
    pub reserved0: u8,
    pub tenant_id: __le32,
    pub dst_mac_hi: __le16,
    pub dst_mac_mid: __le16,
    pub dst_mac_lo: __le16,
    pub src_mac_hi: __le16,
    pub src_mac_mid: __le16,
    pub src_mac_lo: __le16,
    pub vlan_id: __le16,
    pub l2_eth_type: __le16,
    pub ipv4_dscp: u8,
    pub ipv4_frag_type: u8,
    pub ipv4_over_ip: u8,
    pub tenant_id_exists: u8,
    pub ipv4_dst_addr: __le32,
    pub ipv4_src_addr: __le32,
    pub l4_dst_port: __le16,
    pub l4_src_port: __le16,
}

// Ramrod data for rx queue start ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_queue_start_ramrod_data {
    pub rx_queue_id: __le16,
    pub num_of_pbl_pages: __le16,
    pub bd_max_bytes: __le16,
    pub sb_id: __le16,
    pub sb_index: u8,
    pub vport_id: u8,
    pub default_rss_queue_flg: u8,
    pub complete_cqe_flg: u8,
    pub complete_event_flg: u8,
    pub stats_counter_id: u8,
    pub pin_context: u8,
    pub pxp_tph_valid_bd: u8,
    pub pxp_tph_valid_pkt: u8,
    pub pxp_st_hint: u8,
    pub pxp_st_index: __le16,
    pub pmd_mode: u8,
    pub notify_en: u8,
    pub toggle_val: u8,
    pub vf_rx_prod_index: u8,
    pub vf_rx_prod_use_zone_a: u8,
    pub reserved: [u8; 5],
    pub reserved1: __le16,
    pub cqe_pbl_addr: regpair,
    pub bd_base: regpair,
    pub reserved2: regpair,
}

// Ramrod data for rx queue stop ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_queue_stop_ramrod_data {
    pub rx_queue_id: __le16,
    pub complete_cqe_flg: u8,
    pub complete_event_flg: u8,
    pub vport_id: u8,
    pub reserved: [u8; 3],
}

// Ramrod data for rx queue update ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_queue_update_ramrod_data {
    pub rx_queue_id: __le16,
    pub complete_cqe_flg: u8,
    pub complete_event_flg: u8,
    pub vport_id: u8,
    pub set_default_rss_queue: u8,
    pub reserved: [u8; 3],
    pub reserved1: u8,
    pub reserved2: u8,
    pub reserved3: u8,
    pub reserved4: __le16,
    pub reserved5: __le16,
    pub reserved6: regpair,
}

// Ramrod data for rx Add UDP Filter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_udp_filter_ramrod_data {
    pub action_icid: __le16,
    pub vlan_id: __le16,
    pub ip_type: u8,
    pub tenant_id_exists: u8,
    pub reserved1: __le16,
    pub ip_dst_addr: [__le32; 4],
    pub ip_src_addr: [__le32; 4],
    pub udp_dst_port: __le16,
    pub udp_src_port: __le16,
    pub tenant_id: __le32,
}

// Add or delete GFT filter - filter is packet header of type of packet wished
// to pass certain FW flow.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_update_gft_filter_ramrod_data {
    pub pkt_hdr_addr: regpair,
    pub pkt_hdr_length: __le16,
    pub action_icid: __le16,
    pub rx_qid: __le16,
    pub flow_id: __le16,
    pub vport_id: __le16,
    pub action_icid_valid: u8,
    pub rx_qid_valid: u8,
    pub flow_id_valid: u8,
    pub filter_action: u8,
    pub assert_on_error: u8,
    pub inner_vlan_removal_en: u8,
}

// Ramrod data for tx queue start ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_queue_start_ramrod_data {
    pub sb_id: __le16,
    pub sb_index: u8,
    pub vport_id: u8,
    pub reserved0: u8,
    pub stats_counter_id: u8,
    pub qm_pq_id: __le16,
    pub flags: u8,
pub const TX_QUEUE_START_RAMROD_DATA_DISABLE_OPPORTUNISTIC_MASK: c_uint = 0x1;
pub const TX_QUEUE_START_RAMROD_DATA_DISABLE_OPPORTUNISTIC_SHIFT: c_int = 0;
pub const TX_QUEUE_START_RAMROD_DATA_TEST_MODE_PKT_DUP_MASK: c_uint = 0x1;
pub const TX_QUEUE_START_RAMROD_DATA_TEST_MODE_PKT_DUP_SHIFT: c_int = 1;
pub const TX_QUEUE_START_RAMROD_DATA_PMD_MODE_MASK: c_uint = 0x1;
pub const TX_QUEUE_START_RAMROD_DATA_PMD_MODE_SHIFT: c_int = 2;
pub const TX_QUEUE_START_RAMROD_DATA_NOTIFY_EN_MASK: c_uint = 0x1;
pub const TX_QUEUE_START_RAMROD_DATA_NOTIFY_EN_SHIFT: c_int = 3;
pub const TX_QUEUE_START_RAMROD_DATA_PIN_CONTEXT_MASK: c_uint = 0x1;
pub const TX_QUEUE_START_RAMROD_DATA_PIN_CONTEXT_SHIFT: c_int = 4;
pub const TX_QUEUE_START_RAMROD_DATA_RESERVED1_MASK: c_uint = 0x7;
pub const TX_QUEUE_START_RAMROD_DATA_RESERVED1_SHIFT: c_int = 5;
    pub pxp_st_hint: u8,
    pub pxp_tph_valid_bd: u8,
    pub pxp_tph_valid_pkt: u8,
    pub pxp_st_index: __le16,
    pub comp_agg_size: u8,
    pub reserved3: u8,
    pub queue_zone_id: __le16,
    pub reserved2: __le16,
    pub pbl_size: __le16,
    pub tx_queue_id: __le16,
    pub same_as_last_id: __le16,
    pub reserved: [__le16; 3],
    pub pbl_base_addr: regpair,
    pub bd_cons_address: regpair,
}

// Ramrod data for tx queue stop ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_queue_stop_ramrod_data {
    pub reserved: [__le16; 4],
}

// Ramrod data for tx queue update ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_queue_update_ramrod_data {
    pub update_qm_pq_id_flg: __le16,
    pub qm_pq_id: __le16,
    pub reserved0: __le32,
    pub reserved1: [regpair; 5],
}

// Inner to Inner VLAN priority map update mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum update_in_to_in_pri_map_mode_enum {
    ETH_IN_TO_IN_PRI_MAP_UPDATE_DISABLED,
    ETH_IN_TO_IN_PRI_MAP_UPDATE_NON_RDMA_TBL,
    ETH_IN_TO_IN_PRI_MAP_UPDATE_RDMA_TBL,
    MAX_UPDATE_IN_TO_IN_PRI_MAP_MODE_ENUM
}

// Ramrod data for vport update ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport_filter_update_ramrod_data {
    pub filter_cmd_hdr: eth_filter_cmd_header,
    pub filter_cmds: [eth_filter_cmd; ETH_FILTER_RULES_COUNT],
}

// Ramrod data for vport start ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport_start_ramrod_data {
    pub vport_id: u8,
    pub sw_fid: u8,
    pub mtu: __le16,
    pub drop_ttl0_en: u8,
    pub inner_vlan_removal_en: u8,
    pub rx_mode: eth_vport_rx_mode,
    pub tx_mode: eth_vport_tx_mode,
    pub tpa_param: eth_vport_tpa_param,
    pub default_vlan: __le16,
    pub tx_switching_en: u8,
    pub anti_spoofing_en: u8,
    pub default_vlan_en: u8,
    pub handle_ptp_pkts: u8,
    pub silent_vlan_removal_en: u8,
    pub untagged: u8,
    pub tx_err_behav: eth_tx_err_vals,
    pub zero_placement_offset: u8,
    pub ctl_frame_mac_check_en: u8,
    pub ctl_frame_ethtype_check_en: u8,
    pub reserved0: u8,
    pub reserved1: u8,
    pub tx_dst_port_mode_config: u8,
    pub dst_vport_id: u8,
    pub tx_dst_port_mode: u8,
    pub dst_vport_id_valid: u8,
    pub wipe_inner_vlan_pri_en: u8,
    pub reserved2: [u8; 2],
    pub in_to_in_vlan_pri_map_cfg: eth_in_to_in_pri_map_cfg,
}

// Ramrod data for vport stop ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport_stop_ramrod_data {
    pub vport_id: u8,
    pub reserved: [u8; 7],
}

// Ramrod data for vport update ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport_update_ramrod_data_cmn {
    pub vport_id: u8,
    pub update_rx_active_flg: u8,
    pub rx_active_flg: u8,
    pub update_tx_active_flg: u8,
    pub tx_active_flg: u8,
    pub update_rx_mode_flg: u8,
    pub update_tx_mode_flg: u8,
    pub update_approx_mcast_flg: u8,
    pub update_rss_flg: u8,
    pub update_inner_vlan_removal_en_flg: u8,
    pub inner_vlan_removal_en: u8,
    pub update_tpa_param_flg: u8,
    pub update_tpa_en_flg: u8,
    pub update_tx_switching_en_flg: u8,
    pub tx_switching_en: u8,
    pub update_anti_spoofing_en_flg: u8,
    pub anti_spoofing_en: u8,
    pub update_handle_ptp_pkts: u8,
    pub handle_ptp_pkts: u8,
    pub update_default_vlan_en_flg: u8,
    pub default_vlan_en: u8,
    pub update_default_vlan_flg: u8,
    pub default_vlan: __le16,
    pub update_accept_any_vlan_flg: u8,
    pub accept_any_vlan: u8,
    pub silent_vlan_removal_en: u8,
    pub update_mtu_flg: u8,
    pub mtu: __le16,
    pub update_ctl_frame_checks_en_flg: u8,
    pub ctl_frame_mac_check_en: u8,
    pub ctl_frame_ethtype_check_en: u8,
    pub update_in_to_in_pri_map_mode: u8,
    pub in_to_in_pri_map: [u8; 8],
    pub update_tx_dst_port_mode_flg: u8,
    pub tx_dst_port_mode_config: u8,
    pub dst_vport_id: u8,
    pub tx_dst_port_mode: u8,
    pub dst_vport_id_valid: u8,
    pub reserved: [u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport_update_ramrod_mcast {
    pub bins: [__le32; ETH_MULTICAST_MAC_BINS_IN_REGS],
}

// Ramrod data for vport update ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport_update_ramrod_data {
    pub common: vport_update_ramrod_data_cmn,
    pub rx_mode: eth_vport_rx_mode,
    pub tx_mode: eth_vport_tx_mode,
    pub reserved: [__le32; 3],
    pub tpa_param: eth_vport_tpa_param,
    pub approx_mcast: vport_update_ramrod_mcast,
    pub rss_config: eth_vport_rss_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_eth_conn_ag_ctx_dq_ext_ldpart {
    pub reserved0: u8,
    pub state: u8,
    pub flags0: u8,
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED1_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED1_SHIFT: c_int = 1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED2_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED2_SHIFT: c_int = 2;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_EXIST_IN_QM3_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_EXIST_IN_QM3_SHIFT: c_int = 3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED3_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED3_SHIFT: c_int = 4;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED4_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED4_SHIFT: c_int = 5;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED5_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED5_SHIFT: c_int = 6;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED6_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED6_SHIFT: c_int = 7;
    pub flags1: u8,
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED7_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED7_SHIFT: c_int = 0;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED8_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED8_SHIFT: c_int = 1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED9_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED9_SHIFT: c_int = 2;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_BIT11_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_BIT11_SHIFT: c_int = 3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_E5_RESERVED2_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_E5_RESERVED2_SHIFT: c_int = 4;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_E5_RESERVED3_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_E5_RESERVED3_SHIFT: c_int = 5;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_TX_RULE_ACTIVE_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_TX_RULE_ACTIVE_SHIFT: c_int = 6;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_DQ_CF_ACTIVE_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_DQ_CF_ACTIVE_SHIFT: c_int = 7;
    pub flags2: u8,
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF0_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF0_SHIFT: c_int = 0;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF1_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF1_SHIFT: c_int = 2;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF2_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF2_SHIFT: c_int = 4;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF3_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF3_SHIFT: c_int = 6;
    pub flags3: u8,
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF4_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF4_SHIFT: c_int = 0;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF5_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF5_SHIFT: c_int = 2;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF6_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF6_SHIFT: c_int = 4;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF7_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF7_SHIFT: c_int = 6;
    pub flags4: u8,
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF8_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF8_SHIFT: c_int = 0;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF9_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF9_SHIFT: c_int = 2;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF10_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF10_SHIFT: c_int = 4;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF11_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF11_SHIFT: c_int = 6;
    pub flags5: u8,
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF12_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF12_SHIFT: c_int = 0;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF13_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF13_SHIFT: c_int = 2;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF14_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF14_SHIFT: c_int = 4;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF15_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF15_SHIFT: c_int = 6;
    pub flags6: u8,
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_GO_TO_BD_CONS_CF_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_GO_TO_BD_CONS_CF_SHIFT: c_int = 0;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_MULTI_UNICAST_CF_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_MULTI_UNICAST_CF_SHIFT: c_int = 2;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_DQ_CF_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_DQ_CF_SHIFT: c_int = 4;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_TERMINATE_CF_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_TERMINATE_CF_SHIFT: c_int = 6;
    pub flags7: u8,
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_FLUSH_Q0_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_FLUSH_Q0_SHIFT: c_int = 0;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED10_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED10_SHIFT: c_int = 2;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_SLOW_PATH_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_SLOW_PATH_SHIFT: c_int = 4;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF0EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF0EN_SHIFT: c_int = 6;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF1EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF1EN_SHIFT: c_int = 7;
    pub flags8: u8,
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF2EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF2EN_SHIFT: c_int = 0;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF3EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF3EN_SHIFT: c_int = 1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF4EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF4EN_SHIFT: c_int = 2;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF5EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF5EN_SHIFT: c_int = 3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF6EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF6EN_SHIFT: c_int = 4;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF7EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF7EN_SHIFT: c_int = 5;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF8EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF8EN_SHIFT: c_int = 6;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF9EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF9EN_SHIFT: c_int = 7;
    pub flags9: u8,
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF10EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF10EN_SHIFT: c_int = 0;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF11EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF11EN_SHIFT: c_int = 1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF12EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF12EN_SHIFT: c_int = 2;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF13EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF13EN_SHIFT: c_int = 3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF14EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF14EN_SHIFT: c_int = 4;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF15EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_CF15EN_SHIFT: c_int = 5;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_GO_TO_BD_CONS_CF_EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_GO_TO_BD_CONS_CF_EN_SHIFT: c_int = 6;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_MULTI_UNICAST_CF_EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_MULTI_UNICAST_CF_EN_SHIFT: c_int = 7;
    pub flags10: u8,
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_DQ_CF_EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_DQ_CF_EN_SHIFT: c_int = 0;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_TERMINATE_CF_EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_TERMINATE_CF_EN_SHIFT: c_int = 1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_FLUSH_Q0_EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_FLUSH_Q0_EN_SHIFT: c_int = 2;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED11_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED11_SHIFT: c_int = 3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_SLOW_PATH_EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_SLOW_PATH_EN_SHIFT: c_int = 4;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_TPH_ENABLE_EN_RESERVED_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_TPH_ENABLE_EN_RESERVED_SHIFT: c_int = 5;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED12_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED12_SHIFT: c_int = 6;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED13_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED13_SHIFT: c_int = 7;
    pub flags11: u8,
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED14_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED14_SHIFT: c_int = 0;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED15_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RESERVED15_SHIFT: c_int = 1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_TX_DEC_RULE_EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_TX_DEC_RULE_EN_SHIFT: c_int = 2;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE5EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE5EN_SHIFT: c_int = 3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE6EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE6EN_SHIFT: c_int = 4;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE7EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE7EN_SHIFT: c_int = 5;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED1_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED1_SHIFT: c_int = 6;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE9EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE9EN_SHIFT: c_int = 7;
    pub flags12: u8,
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE10EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE10EN_SHIFT: c_int = 0;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE11EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE11EN_SHIFT: c_int = 1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED2_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED2_SHIFT: c_int = 2;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED3_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED3_SHIFT: c_int = 3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE14EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE14EN_SHIFT: c_int = 4;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE15EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE15EN_SHIFT: c_int = 5;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE16EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE16EN_SHIFT: c_int = 6;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE17EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE17EN_SHIFT: c_int = 7;
    pub flags13: u8,
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE18EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE18EN_SHIFT: c_int = 0;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE19EN_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_RULE19EN_SHIFT: c_int = 1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED4_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED4_SHIFT: c_int = 2;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED5_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED5_SHIFT: c_int = 3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED6_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED6_SHIFT: c_int = 4;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED7_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED7_SHIFT: c_int = 5;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED8_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED8_SHIFT: c_int = 6;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED9_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_A0_RESERVED9_SHIFT: c_int = 7;
    pub flags14: u8,
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_EDPM_USE_EXT_HDR_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_EDPM_USE_EXT_HDR_SHIFT: c_int = 0;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_EDPM_SEND_RAW_L3L4_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_EDPM_SEND_RAW_L3L4_SHIFT: c_int = 1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_EDPM_INBAND_PROP_HDR_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_EDPM_INBAND_PROP_HDR_SHIFT: c_int = 2;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_EDPM_SEND_EXT_TUNNEL_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_EDPM_SEND_EXT_TUNNEL_SHIFT: c_int = 3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_L2_EDPM_ENABLE_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_L2_EDPM_ENABLE_SHIFT: c_int = 4;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_ROCE_EDPM_ENABLE_MASK: c_uint = 0x1;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_ROCE_EDPM_ENABLE_SHIFT: c_int = 5;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_TPH_ENABLE_MASK: c_uint = 0x3;
pub const E4XSTORMETHCONNAGCTXDQEXTLDPART_TPH_ENABLE_SHIFT: c_int = 6;
    pub edpm_event_id: u8,
    pub physical_q0: __le16,
    pub e5_reserved1: __le16,
    pub edpm_num_bds: __le16,
    pub tx_bd_cons: __le16,
    pub tx_bd_prod: __le16,
    pub updated_qm_pq_id: __le16,
    pub conn_dpi: __le16,
    pub byte3: u8,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub reg3: __le32,
    pub reg4: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_eth_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const MSTORM_ETH_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const MSTORM_ETH_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const MSTORM_ETH_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const MSTORM_ETH_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const MSTORM_ETH_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const MSTORM_ETH_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const MSTORM_ETH_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const MSTORM_ETH_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const MSTORM_ETH_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const MSTORM_ETH_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const MSTORM_ETH_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const MSTORM_ETH_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const MSTORM_ETH_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const MSTORM_ETH_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const MSTORM_ETH_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const MSTORM_ETH_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const MSTORM_ETH_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const MSTORM_ETH_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const MSTORM_ETH_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const MSTORM_ETH_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const MSTORM_ETH_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const MSTORM_ETH_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const MSTORM_ETH_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const MSTORM_ETH_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const MSTORM_ETH_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const MSTORM_ETH_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub word0: __le16,
    pub word1: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_eth_hw_conn_ag_ctx {
    pub reserved0: u8,
    pub state: u8,
    pub flags0: u8,
pub const XSTORM_ETH_HW_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED1_SHIFT: c_int = 1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED2_SHIFT: c_int = 2;
pub const XSTORM_ETH_HW_CONN_AG_CTX_EXIST_IN_QM3_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_EXIST_IN_QM3_SHIFT: c_int = 3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED3_SHIFT: c_int = 4;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED4_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED4_SHIFT: c_int = 5;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED5_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED5_SHIFT: c_int = 6;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED6_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED6_SHIFT: c_int = 7;
    pub flags1: u8,
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED7_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED7_SHIFT: c_int = 0;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED8_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED8_SHIFT: c_int = 1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED9_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED9_SHIFT: c_int = 2;
pub const XSTORM_ETH_HW_CONN_AG_CTX_BIT11_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_BIT11_SHIFT: c_int = 3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_E5_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_E5_RESERVED2_SHIFT: c_int = 4;
pub const XSTORM_ETH_HW_CONN_AG_CTX_E5_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_E5_RESERVED3_SHIFT: c_int = 5;
pub const XSTORM_ETH_HW_CONN_AG_CTX_TX_RULE_ACTIVE_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_TX_RULE_ACTIVE_SHIFT: c_int = 6;
pub const XSTORM_ETH_HW_CONN_AG_CTX_DQ_CF_ACTIVE_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_DQ_CF_ACTIVE_SHIFT: c_int = 7;
    pub flags2: u8,
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF0_SHIFT: c_int = 0;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF1_SHIFT: c_int = 2;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF2_SHIFT: c_int = 4;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF3_SHIFT: c_int = 6;
    pub flags3: u8,
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF4_SHIFT: c_int = 0;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF5_SHIFT: c_int = 2;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF6_SHIFT: c_int = 4;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF7_SHIFT: c_int = 6;
    pub flags4: u8,
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF8_SHIFT: c_int = 0;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF9_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF9_SHIFT: c_int = 2;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF10_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF10_SHIFT: c_int = 4;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF11_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF11_SHIFT: c_int = 6;
    pub flags5: u8,
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF12_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF12_SHIFT: c_int = 0;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF13_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF13_SHIFT: c_int = 2;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF14_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF14_SHIFT: c_int = 4;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF15_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF15_SHIFT: c_int = 6;
    pub flags6: u8,
pub const XSTORM_ETH_HW_CONN_AG_CTX_GO_TO_BD_CONS_CF_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_GO_TO_BD_CONS_CF_SHIFT: c_int = 0;
pub const XSTORM_ETH_HW_CONN_AG_CTX_MULTI_UNICAST_CF_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_MULTI_UNICAST_CF_SHIFT: c_int = 2;
pub const XSTORM_ETH_HW_CONN_AG_CTX_DQ_CF_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_DQ_CF_SHIFT: c_int = 4;
pub const XSTORM_ETH_HW_CONN_AG_CTX_TERMINATE_CF_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_TERMINATE_CF_SHIFT: c_int = 6;
    pub flags7: u8,
pub const XSTORM_ETH_HW_CONN_AG_CTX_FLUSH_Q0_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_FLUSH_Q0_SHIFT: c_int = 0;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED10_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED10_SHIFT: c_int = 2;
pub const XSTORM_ETH_HW_CONN_AG_CTX_SLOW_PATH_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_SLOW_PATH_SHIFT: c_int = 4;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF0EN_SHIFT: c_int = 6;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags8: u8,
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF2EN_SHIFT: c_int = 0;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF3EN_SHIFT: c_int = 1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF4EN_SHIFT: c_int = 2;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF5EN_SHIFT: c_int = 3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF6EN_SHIFT: c_int = 4;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF7EN_SHIFT: c_int = 5;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF8EN_SHIFT: c_int = 6;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF9EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF9EN_SHIFT: c_int = 7;
    pub flags9: u8,
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF10EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF10EN_SHIFT: c_int = 0;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF11EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF11EN_SHIFT: c_int = 1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF12EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF12EN_SHIFT: c_int = 2;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF13EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF13EN_SHIFT: c_int = 3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF14EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF14EN_SHIFT: c_int = 4;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF15EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_CF15EN_SHIFT: c_int = 5;
pub const XSTORM_ETH_HW_CONN_AG_CTX_GO_TO_BD_CONS_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_GO_TO_BD_CONS_CF_EN_SHIFT: c_int = 6;
pub const XSTORM_ETH_HW_CONN_AG_CTX_MULTI_UNICAST_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_MULTI_UNICAST_CF_EN_SHIFT: c_int = 7;
    pub flags10: u8,
pub const XSTORM_ETH_HW_CONN_AG_CTX_DQ_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_DQ_CF_EN_SHIFT: c_int = 0;
pub const XSTORM_ETH_HW_CONN_AG_CTX_TERMINATE_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_TERMINATE_CF_EN_SHIFT: c_int = 1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_FLUSH_Q0_EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_FLUSH_Q0_EN_SHIFT: c_int = 2;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED11_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED11_SHIFT: c_int = 3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_SLOW_PATH_EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_SLOW_PATH_EN_SHIFT: c_int = 4;
pub const XSTORM_ETH_HW_CONN_AG_CTX_TPH_ENABLE_EN_RESERVED_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_TPH_ENABLE_EN_RESERVED_SHIFT: c_int = 5;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED12_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED12_SHIFT: c_int = 6;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED13_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED13_SHIFT: c_int = 7;
    pub flags11: u8,
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED14_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED14_SHIFT: c_int = 0;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED15_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RESERVED15_SHIFT: c_int = 1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_TX_DEC_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_TX_DEC_RULE_EN_SHIFT: c_int = 2;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 4;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 5;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED1_SHIFT: c_int = 6;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE9EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE9EN_SHIFT: c_int = 7;
    pub flags12: u8,
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE10EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE10EN_SHIFT: c_int = 0;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE11EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE11EN_SHIFT: c_int = 1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED2_SHIFT: c_int = 2;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED3_SHIFT: c_int = 3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE14EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE14EN_SHIFT: c_int = 4;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE15EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE15EN_SHIFT: c_int = 5;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE16EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE16EN_SHIFT: c_int = 6;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE17EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE17EN_SHIFT: c_int = 7;
    pub flags13: u8,
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE18EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE18EN_SHIFT: c_int = 0;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE19EN_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_RULE19EN_SHIFT: c_int = 1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED4_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED4_SHIFT: c_int = 2;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED5_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED5_SHIFT: c_int = 3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED6_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED6_SHIFT: c_int = 4;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED7_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED7_SHIFT: c_int = 5;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED8_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED8_SHIFT: c_int = 6;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED9_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_A0_RESERVED9_SHIFT: c_int = 7;
    pub flags14: u8,
pub const XSTORM_ETH_HW_CONN_AG_CTX_EDPM_USE_EXT_HDR_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_EDPM_USE_EXT_HDR_SHIFT: c_int = 0;
pub const XSTORM_ETH_HW_CONN_AG_CTX_EDPM_SEND_RAW_L3L4_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_EDPM_SEND_RAW_L3L4_SHIFT: c_int = 1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_EDPM_INBAND_PROP_HDR_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_EDPM_INBAND_PROP_HDR_SHIFT: c_int = 2;
pub const XSTORM_ETH_HW_CONN_AG_CTX_EDPM_SEND_EXT_TUNNEL_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_EDPM_SEND_EXT_TUNNEL_SHIFT: c_int = 3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_L2_EDPM_ENABLE_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_L2_EDPM_ENABLE_SHIFT: c_int = 4;
pub const XSTORM_ETH_HW_CONN_AG_CTX_ROCE_EDPM_ENABLE_MASK: c_uint = 0x1;
pub const XSTORM_ETH_HW_CONN_AG_CTX_ROCE_EDPM_ENABLE_SHIFT: c_int = 5;
pub const XSTORM_ETH_HW_CONN_AG_CTX_TPH_ENABLE_MASK: c_uint = 0x3;
pub const XSTORM_ETH_HW_CONN_AG_CTX_TPH_ENABLE_SHIFT: c_int = 6;
    pub edpm_event_id: u8,
    pub physical_q0: __le16,
    pub e5_reserved1: __le16,
    pub edpm_num_bds: __le16,
    pub tx_bd_cons: __le16,
    pub tx_bd_prod: __le16,
    pub updated_qm_pq_id: __le16,
    pub conn_dpi: __le16,
}

// GFT CAM line struct with fields breakout
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gft_cam_line_mapped {
    pub camline: __le32,
pub const GFT_CAM_LINE_MAPPED_VALID_MASK: c_uint = 0x1;
pub const GFT_CAM_LINE_MAPPED_VALID_SHIFT: c_int = 0;
pub const GFT_CAM_LINE_MAPPED_IP_VERSION_MASK: c_uint = 0x1;
pub const GFT_CAM_LINE_MAPPED_IP_VERSION_SHIFT: c_int = 1;
pub const GFT_CAM_LINE_MAPPED_TUNNEL_IP_VERSION_MASK: c_uint = 0x1;
pub const GFT_CAM_LINE_MAPPED_TUNNEL_IP_VERSION_SHIFT: c_int = 2;
pub const GFT_CAM_LINE_MAPPED_UPPER_PROTOCOL_TYPE_MASK: c_uint = 0xF;
pub const GFT_CAM_LINE_MAPPED_UPPER_PROTOCOL_TYPE_SHIFT: c_int = 3;
pub const GFT_CAM_LINE_MAPPED_TUNNEL_TYPE_MASK: c_uint = 0xF;
pub const GFT_CAM_LINE_MAPPED_TUNNEL_TYPE_SHIFT: c_int = 7;
pub const GFT_CAM_LINE_MAPPED_PF_ID_MASK: c_uint = 0xF;
pub const GFT_CAM_LINE_MAPPED_PF_ID_SHIFT: c_int = 11;
pub const GFT_CAM_LINE_MAPPED_IP_VERSION_MASK_MASK: c_uint = 0x1;
pub const GFT_CAM_LINE_MAPPED_IP_VERSION_MASK_SHIFT: c_int = 15;
pub const GFT_CAM_LINE_MAPPED_TUNNEL_IP_VERSION_MASK_MASK: c_uint = 0x1;
pub const GFT_CAM_LINE_MAPPED_TUNNEL_IP_VERSION_MASK_SHIFT: c_int = 16;
pub const GFT_CAM_LINE_MAPPED_UPPER_PROTOCOL_TYPE_MASK_MASK: c_uint = 0xF;
pub const GFT_CAM_LINE_MAPPED_UPPER_PROTOCOL_TYPE_MASK_SHIFT: c_int = 17;
pub const GFT_CAM_LINE_MAPPED_TUNNEL_TYPE_MASK_MASK: c_uint = 0xF;
pub const GFT_CAM_LINE_MAPPED_TUNNEL_TYPE_MASK_SHIFT: c_int = 21;
pub const GFT_CAM_LINE_MAPPED_PF_ID_MASK_MASK: c_uint = 0xF;
pub const GFT_CAM_LINE_MAPPED_PF_ID_MASK_SHIFT: c_int = 25;
pub const GFT_CAM_LINE_MAPPED_RESERVED1_MASK: c_uint = 0x7;
pub const GFT_CAM_LINE_MAPPED_RESERVED1_SHIFT: c_int = 29;
}

// Used in gft_profile_key: Indication for ip version
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gft_profile_ip_version {
    GFT_PROFILE_IPV4 = 0,
    GFT_PROFILE_IPV6 = 1,
    MAX_GFT_PROFILE_IP_VERSION
}

// Profile key stucr fot GFT logic in Prs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gft_profile_key {
    pub profile_key: __le16,
pub const GFT_PROFILE_KEY_IP_VERSION_MASK: c_uint = 0x1;
pub const GFT_PROFILE_KEY_IP_VERSION_SHIFT: c_int = 0;
pub const GFT_PROFILE_KEY_TUNNEL_IP_VERSION_MASK: c_uint = 0x1;
pub const GFT_PROFILE_KEY_TUNNEL_IP_VERSION_SHIFT: c_int = 1;
pub const GFT_PROFILE_KEY_UPPER_PROTOCOL_TYPE_MASK: c_uint = 0xF;
pub const GFT_PROFILE_KEY_UPPER_PROTOCOL_TYPE_SHIFT: c_int = 2;
pub const GFT_PROFILE_KEY_TUNNEL_TYPE_MASK: c_uint = 0xF;
pub const GFT_PROFILE_KEY_TUNNEL_TYPE_SHIFT: c_int = 6;
pub const GFT_PROFILE_KEY_PF_ID_MASK: c_uint = 0xF;
pub const GFT_PROFILE_KEY_PF_ID_SHIFT: c_int = 10;
pub const GFT_PROFILE_KEY_RESERVED0_MASK: c_uint = 0x3;
pub const GFT_PROFILE_KEY_RESERVED0_SHIFT: c_int = 14;
}

// Used in gft_profile_key: Indication for tunnel type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gft_profile_tunnel_type {
    GFT_PROFILE_NO_TUNNEL = 0,
    GFT_PROFILE_VXLAN_TUNNEL = 1,
    GFT_PROFILE_GRE_MAC_OR_NVGRE_TUNNEL = 2,
    GFT_PROFILE_GRE_IP_TUNNEL = 3,
    GFT_PROFILE_GENEVE_MAC_TUNNEL = 4,
    GFT_PROFILE_GENEVE_IP_TUNNEL = 5,
    MAX_GFT_PROFILE_TUNNEL_TYPE
}

// Used in gft_profile_key: Indication for protocol type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gft_profile_upper_protocol_type {
    GFT_PROFILE_ROCE_PROTOCOL = 0,
    GFT_PROFILE_RROCE_PROTOCOL = 1,
    GFT_PROFILE_FCOE_PROTOCOL = 2,
    GFT_PROFILE_ICMP_PROTOCOL = 3,
    GFT_PROFILE_ARP_PROTOCOL = 4,
    GFT_PROFILE_USER_TCP_SRC_PORT_1_INNER = 5,
    GFT_PROFILE_USER_TCP_DST_PORT_1_INNER = 6,
    GFT_PROFILE_TCP_PROTOCOL = 7,
    GFT_PROFILE_USER_UDP_DST_PORT_1_INNER = 8,
    GFT_PROFILE_USER_UDP_DST_PORT_2_OUTER = 9,
    GFT_PROFILE_UDP_PROTOCOL = 10,
    GFT_PROFILE_USER_IP_1_INNER = 11,
    GFT_PROFILE_USER_IP_2_OUTER = 12,
    GFT_PROFILE_USER_ETH_1_INNER = 13,
    GFT_PROFILE_USER_ETH_2_OUTER = 14,
    GFT_PROFILE_RAW = 15,
    MAX_GFT_PROFILE_UPPER_PROTOCOL_TYPE
}

// GFT RAM line struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gft_ram_line {
    pub lo: __le32,
pub const GFT_RAM_LINE_VLAN_SELECT_MASK: c_uint = 0x3;
pub const GFT_RAM_LINE_VLAN_SELECT_SHIFT: c_int = 0;
pub const GFT_RAM_LINE_TUNNEL_ENTROPHY_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TUNNEL_ENTROPHY_SHIFT: c_int = 2;
pub const GFT_RAM_LINE_TUNNEL_TTL_EQUAL_ONE_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TUNNEL_TTL_EQUAL_ONE_SHIFT: c_int = 3;
pub const GFT_RAM_LINE_TUNNEL_TTL_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TUNNEL_TTL_SHIFT: c_int = 4;
pub const GFT_RAM_LINE_TUNNEL_ETHERTYPE_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TUNNEL_ETHERTYPE_SHIFT: c_int = 5;
pub const GFT_RAM_LINE_TUNNEL_DST_PORT_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TUNNEL_DST_PORT_SHIFT: c_int = 6;
pub const GFT_RAM_LINE_TUNNEL_SRC_PORT_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TUNNEL_SRC_PORT_SHIFT: c_int = 7;
pub const GFT_RAM_LINE_TUNNEL_DSCP_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TUNNEL_DSCP_SHIFT: c_int = 8;
pub const GFT_RAM_LINE_TUNNEL_OVER_IP_PROTOCOL_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TUNNEL_OVER_IP_PROTOCOL_SHIFT: c_int = 9;
pub const GFT_RAM_LINE_TUNNEL_DST_IP_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TUNNEL_DST_IP_SHIFT: c_int = 10;
pub const GFT_RAM_LINE_TUNNEL_SRC_IP_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TUNNEL_SRC_IP_SHIFT: c_int = 11;
pub const GFT_RAM_LINE_TUNNEL_PRIORITY_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TUNNEL_PRIORITY_SHIFT: c_int = 12;
pub const GFT_RAM_LINE_TUNNEL_PROVIDER_VLAN_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TUNNEL_PROVIDER_VLAN_SHIFT: c_int = 13;
pub const GFT_RAM_LINE_TUNNEL_VLAN_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TUNNEL_VLAN_SHIFT: c_int = 14;
pub const GFT_RAM_LINE_TUNNEL_DST_MAC_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TUNNEL_DST_MAC_SHIFT: c_int = 15;
pub const GFT_RAM_LINE_TUNNEL_SRC_MAC_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TUNNEL_SRC_MAC_SHIFT: c_int = 16;
pub const GFT_RAM_LINE_TTL_EQUAL_ONE_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TTL_EQUAL_ONE_SHIFT: c_int = 17;
pub const GFT_RAM_LINE_TTL_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TTL_SHIFT: c_int = 18;
pub const GFT_RAM_LINE_ETHERTYPE_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_ETHERTYPE_SHIFT: c_int = 19;
pub const GFT_RAM_LINE_RESERVED0_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_RESERVED0_SHIFT: c_int = 20;
pub const GFT_RAM_LINE_TCP_FLAG_FIN_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TCP_FLAG_FIN_SHIFT: c_int = 21;
pub const GFT_RAM_LINE_TCP_FLAG_SYN_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TCP_FLAG_SYN_SHIFT: c_int = 22;
pub const GFT_RAM_LINE_TCP_FLAG_RST_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TCP_FLAG_RST_SHIFT: c_int = 23;
pub const GFT_RAM_LINE_TCP_FLAG_PSH_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TCP_FLAG_PSH_SHIFT: c_int = 24;
pub const GFT_RAM_LINE_TCP_FLAG_ACK_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TCP_FLAG_ACK_SHIFT: c_int = 25;
pub const GFT_RAM_LINE_TCP_FLAG_URG_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TCP_FLAG_URG_SHIFT: c_int = 26;
pub const GFT_RAM_LINE_TCP_FLAG_ECE_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TCP_FLAG_ECE_SHIFT: c_int = 27;
pub const GFT_RAM_LINE_TCP_FLAG_CWR_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TCP_FLAG_CWR_SHIFT: c_int = 28;
pub const GFT_RAM_LINE_TCP_FLAG_NS_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TCP_FLAG_NS_SHIFT: c_int = 29;
pub const GFT_RAM_LINE_DST_PORT_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_DST_PORT_SHIFT: c_int = 30;
pub const GFT_RAM_LINE_SRC_PORT_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_SRC_PORT_SHIFT: c_int = 31;
    pub hi: __le32,
pub const GFT_RAM_LINE_DSCP_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_DSCP_SHIFT: c_int = 0;
pub const GFT_RAM_LINE_OVER_IP_PROTOCOL_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_OVER_IP_PROTOCOL_SHIFT: c_int = 1;
pub const GFT_RAM_LINE_DST_IP_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_DST_IP_SHIFT: c_int = 2;
pub const GFT_RAM_LINE_SRC_IP_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_SRC_IP_SHIFT: c_int = 3;
pub const GFT_RAM_LINE_PRIORITY_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_PRIORITY_SHIFT: c_int = 4;
pub const GFT_RAM_LINE_PROVIDER_VLAN_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_PROVIDER_VLAN_SHIFT: c_int = 5;
pub const GFT_RAM_LINE_VLAN_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_VLAN_SHIFT: c_int = 6;
pub const GFT_RAM_LINE_DST_MAC_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_DST_MAC_SHIFT: c_int = 7;
pub const GFT_RAM_LINE_SRC_MAC_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_SRC_MAC_SHIFT: c_int = 8;
pub const GFT_RAM_LINE_TENANT_ID_MASK: c_uint = 0x1;
pub const GFT_RAM_LINE_TENANT_ID_SHIFT: c_int = 9;
pub const GFT_RAM_LINE_RESERVED1_MASK: c_uint = 0x3FFFFF;
pub const GFT_RAM_LINE_RESERVED1_SHIFT: c_int = 10;
}

// Used in the first 2 bits for gft_ram_line: Indication for vlan mask
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gft_vlan_select {
    INNER_PROVIDER_VLAN = 0,
    INNER_VLAN = 1,
    OUTER_PROVIDER_VLAN = 2,
    OUTER_VLAN = 3,
    MAX_GFT_VLAN_SELECT
}

// The rdma task context of Mstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_rdma_task_st_ctx {
    pub temp: [regpair; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_rdma_task_ag_ctx {
    pub reserved: u8,
    pub byte1: u8,
    pub msem_ctx_upd_seq: __le16,
    pub flags0: u8,
pub const YSTORM_RDMA_TASK_AG_CTX_CONNECTION_TYPE_MASK: c_uint = 0xF;
pub const YSTORM_RDMA_TASK_AG_CTX_CONNECTION_TYPE_SHIFT: c_int = 0;
pub const YSTORM_RDMA_TASK_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const YSTORM_RDMA_TASK_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 4;
pub const YSTORM_RDMA_TASK_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const YSTORM_RDMA_TASK_AG_CTX_BIT1_SHIFT: c_int = 5;
pub const YSTORM_RDMA_TASK_AG_CTX_VALID_MASK: c_uint = 0x1;
pub const YSTORM_RDMA_TASK_AG_CTX_VALID_SHIFT: c_int = 6;
pub const YSTORM_RDMA_TASK_AG_CTX_DIF_FIRST_IO_MASK: c_uint = 0x1;
pub const YSTORM_RDMA_TASK_AG_CTX_DIF_FIRST_IO_SHIFT: c_int = 7;
    pub flags1: u8,
pub const YSTORM_RDMA_TASK_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const YSTORM_RDMA_TASK_AG_CTX_CF0_SHIFT: c_int = 0;
pub const YSTORM_RDMA_TASK_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const YSTORM_RDMA_TASK_AG_CTX_CF1_SHIFT: c_int = 2;
pub const YSTORM_RDMA_TASK_AG_CTX_CF2SPECIAL_MASK: c_uint = 0x3;
pub const YSTORM_RDMA_TASK_AG_CTX_CF2SPECIAL_SHIFT: c_int = 4;
pub const YSTORM_RDMA_TASK_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const YSTORM_RDMA_TASK_AG_CTX_CF0EN_SHIFT: c_int = 6;
pub const YSTORM_RDMA_TASK_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const YSTORM_RDMA_TASK_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags2: u8,
pub const YSTORM_RDMA_TASK_AG_CTX_BIT4_MASK: c_uint = 0x1;
pub const YSTORM_RDMA_TASK_AG_CTX_BIT4_SHIFT: c_int = 0;
pub const YSTORM_RDMA_TASK_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const YSTORM_RDMA_TASK_AG_CTX_RULE0EN_SHIFT: c_int = 1;
pub const YSTORM_RDMA_TASK_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const YSTORM_RDMA_TASK_AG_CTX_RULE1EN_SHIFT: c_int = 2;
pub const YSTORM_RDMA_TASK_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const YSTORM_RDMA_TASK_AG_CTX_RULE2EN_SHIFT: c_int = 3;
pub const YSTORM_RDMA_TASK_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const YSTORM_RDMA_TASK_AG_CTX_RULE3EN_SHIFT: c_int = 4;
pub const YSTORM_RDMA_TASK_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const YSTORM_RDMA_TASK_AG_CTX_RULE4EN_SHIFT: c_int = 5;
pub const YSTORM_RDMA_TASK_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const YSTORM_RDMA_TASK_AG_CTX_RULE5EN_SHIFT: c_int = 6;
pub const YSTORM_RDMA_TASK_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const YSTORM_RDMA_TASK_AG_CTX_RULE6EN_SHIFT: c_int = 7;
    pub key: u8,
    pub mw_cnt_or_qp_id: __le32,
    pub ref_cnt_seq: u8,
    pub ctx_upd_seq: u8,
    pub dif_flags: __le16,
    pub tx_ref_count: __le16,
    pub last_used_ltid: __le16,
    pub parent_mr_lo: __le16,
    pub parent_mr_hi: __le16,
    pub fbo_lo: __le32,
    pub fbo_hi: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_rdma_task_ag_ctx {
    pub reserved: u8,
    pub byte1: u8,
    pub icid: __le16,
    pub flags0: u8,
pub const MSTORM_RDMA_TASK_AG_CTX_CONNECTION_TYPE_MASK: c_uint = 0xF;
pub const MSTORM_RDMA_TASK_AG_CTX_CONNECTION_TYPE_SHIFT: c_int = 0;
pub const MSTORM_RDMA_TASK_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const MSTORM_RDMA_TASK_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 4;
pub const MSTORM_RDMA_TASK_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const MSTORM_RDMA_TASK_AG_CTX_BIT1_SHIFT: c_int = 5;
pub const MSTORM_RDMA_TASK_AG_CTX_BIT2_MASK: c_uint = 0x1;
pub const MSTORM_RDMA_TASK_AG_CTX_BIT2_SHIFT: c_int = 6;
pub const MSTORM_RDMA_TASK_AG_CTX_DIF_FIRST_IO_MASK: c_uint = 0x1;
pub const MSTORM_RDMA_TASK_AG_CTX_DIF_FIRST_IO_SHIFT: c_int = 7;
    pub flags1: u8,
pub const MSTORM_RDMA_TASK_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const MSTORM_RDMA_TASK_AG_CTX_CF0_SHIFT: c_int = 0;
pub const MSTORM_RDMA_TASK_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const MSTORM_RDMA_TASK_AG_CTX_CF1_SHIFT: c_int = 2;
pub const MSTORM_RDMA_TASK_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const MSTORM_RDMA_TASK_AG_CTX_CF2_SHIFT: c_int = 4;
pub const MSTORM_RDMA_TASK_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const MSTORM_RDMA_TASK_AG_CTX_CF0EN_SHIFT: c_int = 6;
pub const MSTORM_RDMA_TASK_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const MSTORM_RDMA_TASK_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags2: u8,
pub const MSTORM_RDMA_TASK_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const MSTORM_RDMA_TASK_AG_CTX_CF2EN_SHIFT: c_int = 0;
pub const MSTORM_RDMA_TASK_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const MSTORM_RDMA_TASK_AG_CTX_RULE0EN_SHIFT: c_int = 1;
pub const MSTORM_RDMA_TASK_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const MSTORM_RDMA_TASK_AG_CTX_RULE1EN_SHIFT: c_int = 2;
pub const MSTORM_RDMA_TASK_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const MSTORM_RDMA_TASK_AG_CTX_RULE2EN_SHIFT: c_int = 3;
pub const MSTORM_RDMA_TASK_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const MSTORM_RDMA_TASK_AG_CTX_RULE3EN_SHIFT: c_int = 4;
pub const MSTORM_RDMA_TASK_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const MSTORM_RDMA_TASK_AG_CTX_RULE4EN_SHIFT: c_int = 5;
pub const MSTORM_RDMA_TASK_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const MSTORM_RDMA_TASK_AG_CTX_RULE5EN_SHIFT: c_int = 6;
pub const MSTORM_RDMA_TASK_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const MSTORM_RDMA_TASK_AG_CTX_RULE6EN_SHIFT: c_int = 7;
    pub key: u8,
    pub mw_cnt_or_qp_id: __le32,
    pub ref_cnt_seq: u8,
    pub ctx_upd_seq: u8,
    pub dif_flags: __le16,
    pub tx_ref_count: __le16,
    pub last_used_ltid: __le16,
    pub parent_mr_lo: __le16,
    pub parent_mr_hi: __le16,
    pub fbo_lo: __le32,
    pub fbo_hi: __le32,
}

// The roce task context of Mstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_rdma_task_st_ctx {
    pub temp: [regpair; 4],
}

// The roce task context of Ustorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_rdma_task_st_ctx {
    pub temp: [regpair; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_rdma_task_ag_ctx {
    pub reserved: u8,
    pub state: u8,
    pub icid: __le16,
    pub flags0: u8,
pub const USTORM_RDMA_TASK_AG_CTX_CONNECTION_TYPE_MASK: c_uint = 0xF;
pub const USTORM_RDMA_TASK_AG_CTX_CONNECTION_TYPE_SHIFT: c_int = 0;
pub const USTORM_RDMA_TASK_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const USTORM_RDMA_TASK_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 4;
pub const USTORM_RDMA_TASK_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const USTORM_RDMA_TASK_AG_CTX_BIT1_SHIFT: c_int = 5;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_WRITE_RESULT_CF_MASK: c_uint = 0x3;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_WRITE_RESULT_CF_SHIFT: c_int = 6;
    pub flags1: u8,
pub const USTORM_RDMA_TASK_AG_CTX_DIF_RESULT_TOGGLE_BIT_MASK: c_uint = 0x3;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_RESULT_TOGGLE_BIT_SHIFT: c_int = 0;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_TX_IO_FLG_MASK: c_uint = 0x3;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_TX_IO_FLG_SHIFT: c_int = 2;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_BLOCK_SIZE_MASK: c_uint = 0x3;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_BLOCK_SIZE_SHIFT: c_int = 4;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_ERROR_CF_MASK: c_uint = 0x3;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_ERROR_CF_SHIFT: c_int = 6;
    pub flags2: u8,
pub const USTORM_RDMA_TASK_AG_CTX_DIF_WRITE_RESULT_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_WRITE_RESULT_CF_EN_SHIFT: c_int = 0;
pub const USTORM_RDMA_TASK_AG_CTX_RESERVED2_MASK: c_uint = 0x1;
pub const USTORM_RDMA_TASK_AG_CTX_RESERVED2_SHIFT: c_int = 1;
pub const USTORM_RDMA_TASK_AG_CTX_RESERVED3_MASK: c_uint = 0x1;
pub const USTORM_RDMA_TASK_AG_CTX_RESERVED3_SHIFT: c_int = 2;
pub const USTORM_RDMA_TASK_AG_CTX_RESERVED4_MASK: c_uint = 0x1;
pub const USTORM_RDMA_TASK_AG_CTX_RESERVED4_SHIFT: c_int = 3;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_ERROR_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_ERROR_CF_EN_SHIFT: c_int = 4;
pub const USTORM_RDMA_TASK_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_TASK_AG_CTX_RULE0EN_SHIFT: c_int = 5;
pub const USTORM_RDMA_TASK_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_TASK_AG_CTX_RULE1EN_SHIFT: c_int = 6;
pub const USTORM_RDMA_TASK_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_TASK_AG_CTX_RULE2EN_SHIFT: c_int = 7;
    pub flags3: u8,
pub const USTORM_RDMA_TASK_AG_CTX_DIF_RXMIT_PROD_CONS_EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_RXMIT_PROD_CONS_EN_SHIFT: c_int = 0;
pub const USTORM_RDMA_TASK_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_TASK_AG_CTX_RULE4EN_SHIFT: c_int = 1;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_WRITE_PROD_CONS_EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_WRITE_PROD_CONS_EN_SHIFT: c_int = 2;
pub const USTORM_RDMA_TASK_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_TASK_AG_CTX_RULE6EN_SHIFT: c_int = 3;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_ERROR_TYPE_MASK: c_uint = 0xF;
pub const USTORM_RDMA_TASK_AG_CTX_DIF_ERROR_TYPE_SHIFT: c_int = 4;
    pub dif_err_intervals: __le32,
    pub dif_error_1st_interval: __le32,
    pub dif_rxmit_cons: __le32,
    pub dif_rxmit_prod: __le32,
    pub sge_index: __le32,
    pub sq_cons: __le32,
    pub byte2: u8,
    pub byte3: u8,
    pub dif_write_cons: __le16,
    pub dif_write_prod: __le16,
    pub word3: __le16,
    pub dif_error_buffer_address_lo: __le32,
    pub dif_error_buffer_address_hi: __le32,
}

// RDMA task context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_task_context {
    pub ystorm_st_context: ystorm_rdma_task_st_ctx,
    pub ystorm_ag_context: ystorm_rdma_task_ag_ctx,
    pub tdif_context: tdif_task_context,
    pub mstorm_ag_context: mstorm_rdma_task_ag_ctx,
    pub mstorm_st_context: mstorm_rdma_task_st_ctx,
    pub rdif_context: rdif_task_context,
    pub ustorm_st_context: ustorm_rdma_task_st_ctx,
    pub ustorm_st_padding: [regpair; 2],
    pub ustorm_ag_context: ustorm_rdma_task_ag_ctx,
}

pub const TOE_MAX_RAMROD_PER_PF: c_int = 8;
pub const TOE_TX_PAGE_SIZE_BYTES: c_int = 4096;
pub const TOE_GRQ_PAGE_SIZE_BYTES: c_int = 4096;
pub const TOE_RX_CQ_PAGE_SIZE_BYTES: c_int = 4096;
pub const TOE_RX_MAX_RSS_CHAINS: c_int = 64;
pub const TOE_TX_MAX_TSS_CHAINS: c_int = 64;
pub const TOE_RSS_INDIRECTION_TABLE_SIZE: c_int = 128;
// The toe storm context of Mstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_toe_conn_st_ctx {
    pub reserved: [__le32; 24],
}

// The toe storm context of Pstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstorm_toe_conn_st_ctx {
    pub reserved: [__le32; 36],
}

// The toe storm context of Ystorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_toe_conn_st_ctx {
    pub reserved: [__le32; 8],
}

// The toe storm context of Xstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_toe_conn_st_ctx {
    pub reserved: [__le32; 44],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_toe_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const YSTORM_TOE_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const YSTORM_TOE_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const YSTORM_TOE_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const YSTORM_TOE_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const YSTORM_TOE_CONN_AG_CTX_SLOW_PATH_CF_MASK: c_uint = 0x3;
pub const YSTORM_TOE_CONN_AG_CTX_SLOW_PATH_CF_SHIFT: c_int = 2;
pub const YSTORM_TOE_CONN_AG_CTX_RESET_RECEIVED_CF_MASK: c_uint = 0x3;
pub const YSTORM_TOE_CONN_AG_CTX_RESET_RECEIVED_CF_SHIFT: c_int = 4;
pub const YSTORM_TOE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const YSTORM_TOE_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const YSTORM_TOE_CONN_AG_CTX_SLOW_PATH_CF_EN_MASK: c_uint = 0x1;
pub const YSTORM_TOE_CONN_AG_CTX_SLOW_PATH_CF_EN_SHIFT: c_int = 0;
pub const YSTORM_TOE_CONN_AG_CTX_RESET_RECEIVED_CF_EN_MASK: c_uint = 0x1;
pub const YSTORM_TOE_CONN_AG_CTX_RESET_RECEIVED_CF_EN_SHIFT: c_int = 1;
pub const YSTORM_TOE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const YSTORM_TOE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const YSTORM_TOE_CONN_AG_CTX_REL_SEQ_EN_MASK: c_uint = 0x1;
pub const YSTORM_TOE_CONN_AG_CTX_REL_SEQ_EN_SHIFT: c_int = 3;
pub const YSTORM_TOE_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const YSTORM_TOE_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const YSTORM_TOE_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const YSTORM_TOE_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const YSTORM_TOE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const YSTORM_TOE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const YSTORM_TOE_CONN_AG_CTX_CONS_PROD_EN_MASK: c_uint = 0x1;
pub const YSTORM_TOE_CONN_AG_CTX_CONS_PROD_EN_SHIFT: c_int = 7;
    pub completion_opcode: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub rel_seq: __le32,
    pub rel_seq_threshold: __le32,
    pub app_prod: __le16,
    pub app_cons: __le16,
    pub word3: __le16,
    pub word4: __le16,
    pub reg2: __le32,
    pub reg3: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_toe_conn_ag_ctx {
    pub reserved0: u8,
    pub state: u8,
    pub flags0: u8,
pub const XSTORM_TOE_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const XSTORM_TOE_CONN_AG_CTX_EXIST_IN_QM1_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_EXIST_IN_QM1_SHIFT: c_int = 1;
pub const XSTORM_TOE_CONN_AG_CTX_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RESERVED1_SHIFT: c_int = 2;
pub const XSTORM_TOE_CONN_AG_CTX_EXIST_IN_QM3_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_EXIST_IN_QM3_SHIFT: c_int = 3;
pub const XSTORM_TOE_CONN_AG_CTX_TX_DEC_RULE_RES_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_TX_DEC_RULE_RES_SHIFT: c_int = 4;
pub const XSTORM_TOE_CONN_AG_CTX_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RESERVED2_SHIFT: c_int = 5;
pub const XSTORM_TOE_CONN_AG_CTX_BIT6_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT6_SHIFT: c_int = 6;
pub const XSTORM_TOE_CONN_AG_CTX_BIT7_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT7_SHIFT: c_int = 7;
    pub flags1: u8,
pub const XSTORM_TOE_CONN_AG_CTX_BIT8_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT8_SHIFT: c_int = 0;
pub const XSTORM_TOE_CONN_AG_CTX_BIT9_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT9_SHIFT: c_int = 1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT10_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT10_SHIFT: c_int = 2;
pub const XSTORM_TOE_CONN_AG_CTX_BIT11_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT11_SHIFT: c_int = 3;
pub const XSTORM_TOE_CONN_AG_CTX_BIT12_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT12_SHIFT: c_int = 4;
pub const XSTORM_TOE_CONN_AG_CTX_BIT13_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT13_SHIFT: c_int = 5;
pub const XSTORM_TOE_CONN_AG_CTX_BIT14_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT14_SHIFT: c_int = 6;
pub const XSTORM_TOE_CONN_AG_CTX_BIT15_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT15_SHIFT: c_int = 7;
    pub flags2: u8,
pub const XSTORM_TOE_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF0_SHIFT: c_int = 0;
pub const XSTORM_TOE_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF1_SHIFT: c_int = 2;
pub const XSTORM_TOE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF2_SHIFT: c_int = 4;
pub const XSTORM_TOE_CONN_AG_CTX_TIMER_STOP_ALL_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_TIMER_STOP_ALL_SHIFT: c_int = 6;
    pub flags3: u8,
pub const XSTORM_TOE_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF4_SHIFT: c_int = 0;
pub const XSTORM_TOE_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF5_SHIFT: c_int = 2;
pub const XSTORM_TOE_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF6_SHIFT: c_int = 4;
pub const XSTORM_TOE_CONN_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF7_SHIFT: c_int = 6;
    pub flags4: u8,
pub const XSTORM_TOE_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF8_SHIFT: c_int = 0;
pub const XSTORM_TOE_CONN_AG_CTX_CF9_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF9_SHIFT: c_int = 2;
pub const XSTORM_TOE_CONN_AG_CTX_CF10_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF10_SHIFT: c_int = 4;
pub const XSTORM_TOE_CONN_AG_CTX_CF11_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF11_SHIFT: c_int = 6;
    pub flags5: u8,
pub const XSTORM_TOE_CONN_AG_CTX_CF12_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF12_SHIFT: c_int = 0;
pub const XSTORM_TOE_CONN_AG_CTX_CF13_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF13_SHIFT: c_int = 2;
pub const XSTORM_TOE_CONN_AG_CTX_CF14_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF14_SHIFT: c_int = 4;
pub const XSTORM_TOE_CONN_AG_CTX_CF15_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF15_SHIFT: c_int = 6;
    pub flags6: u8,
pub const XSTORM_TOE_CONN_AG_CTX_CF16_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF16_SHIFT: c_int = 0;
pub const XSTORM_TOE_CONN_AG_CTX_CF17_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF17_SHIFT: c_int = 2;
pub const XSTORM_TOE_CONN_AG_CTX_CF18_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF18_SHIFT: c_int = 4;
pub const XSTORM_TOE_CONN_AG_CTX_DQ_FLUSH_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_DQ_FLUSH_SHIFT: c_int = 6;
    pub flags7: u8,
pub const XSTORM_TOE_CONN_AG_CTX_FLUSH_Q0_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_FLUSH_Q0_SHIFT: c_int = 0;
pub const XSTORM_TOE_CONN_AG_CTX_FLUSH_Q1_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_FLUSH_Q1_SHIFT: c_int = 2;
pub const XSTORM_TOE_CONN_AG_CTX_SLOW_PATH_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_SLOW_PATH_SHIFT: c_int = 4;
pub const XSTORM_TOE_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF0EN_SHIFT: c_int = 6;
pub const XSTORM_TOE_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags8: u8,
pub const XSTORM_TOE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 0;
pub const XSTORM_TOE_CONN_AG_CTX_TIMER_STOP_ALL_EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_TIMER_STOP_ALL_EN_SHIFT: c_int = 1;
pub const XSTORM_TOE_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF4EN_SHIFT: c_int = 2;
pub const XSTORM_TOE_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF5EN_SHIFT: c_int = 3;
pub const XSTORM_TOE_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF6EN_SHIFT: c_int = 4;
pub const XSTORM_TOE_CONN_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF7EN_SHIFT: c_int = 5;
pub const XSTORM_TOE_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF8EN_SHIFT: c_int = 6;
pub const XSTORM_TOE_CONN_AG_CTX_CF9EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF9EN_SHIFT: c_int = 7;
    pub flags9: u8,
pub const XSTORM_TOE_CONN_AG_CTX_CF10EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF10EN_SHIFT: c_int = 0;
pub const XSTORM_TOE_CONN_AG_CTX_CF11EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF11EN_SHIFT: c_int = 1;
pub const XSTORM_TOE_CONN_AG_CTX_CF12EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF12EN_SHIFT: c_int = 2;
pub const XSTORM_TOE_CONN_AG_CTX_CF13EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF13EN_SHIFT: c_int = 3;
pub const XSTORM_TOE_CONN_AG_CTX_CF14EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF14EN_SHIFT: c_int = 4;
pub const XSTORM_TOE_CONN_AG_CTX_CF15EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF15EN_SHIFT: c_int = 5;
pub const XSTORM_TOE_CONN_AG_CTX_CF16EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF16EN_SHIFT: c_int = 6;
pub const XSTORM_TOE_CONN_AG_CTX_CF17EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF17EN_SHIFT: c_int = 7;
    pub flags10: u8,
pub const XSTORM_TOE_CONN_AG_CTX_CF18EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF18EN_SHIFT: c_int = 0;
pub const XSTORM_TOE_CONN_AG_CTX_DQ_FLUSH_EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_DQ_FLUSH_EN_SHIFT: c_int = 1;
pub const XSTORM_TOE_CONN_AG_CTX_FLUSH_Q0_EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_FLUSH_Q0_EN_SHIFT: c_int = 2;
pub const XSTORM_TOE_CONN_AG_CTX_FLUSH_Q1_EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_FLUSH_Q1_EN_SHIFT: c_int = 3;
pub const XSTORM_TOE_CONN_AG_CTX_SLOW_PATH_EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_SLOW_PATH_EN_SHIFT: c_int = 4;
pub const XSTORM_TOE_CONN_AG_CTX_CF23EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_CF23EN_SHIFT: c_int = 5;
pub const XSTORM_TOE_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 6;
pub const XSTORM_TOE_CONN_AG_CTX_MORE_TO_SEND_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_MORE_TO_SEND_RULE_EN_SHIFT: c_int = 7;
    pub flags11: u8,
pub const XSTORM_TOE_CONN_AG_CTX_TX_BLOCKED_EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_TX_BLOCKED_EN_SHIFT: c_int = 0;
pub const XSTORM_TOE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 1;
pub const XSTORM_TOE_CONN_AG_CTX_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RESERVED3_SHIFT: c_int = 2;
pub const XSTORM_TOE_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 3;
pub const XSTORM_TOE_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 4;
pub const XSTORM_TOE_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 5;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED1_SHIFT: c_int = 6;
pub const XSTORM_TOE_CONN_AG_CTX_RULE9EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RULE9EN_SHIFT: c_int = 7;
    pub flags12: u8,
pub const XSTORM_TOE_CONN_AG_CTX_RULE10EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RULE10EN_SHIFT: c_int = 0;
pub const XSTORM_TOE_CONN_AG_CTX_RULE11EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RULE11EN_SHIFT: c_int = 1;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED2_SHIFT: c_int = 2;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED3_SHIFT: c_int = 3;
pub const XSTORM_TOE_CONN_AG_CTX_RULE14EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RULE14EN_SHIFT: c_int = 4;
pub const XSTORM_TOE_CONN_AG_CTX_RULE15EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RULE15EN_SHIFT: c_int = 5;
pub const XSTORM_TOE_CONN_AG_CTX_RULE16EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RULE16EN_SHIFT: c_int = 6;
pub const XSTORM_TOE_CONN_AG_CTX_RULE17EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RULE17EN_SHIFT: c_int = 7;
    pub flags13: u8,
pub const XSTORM_TOE_CONN_AG_CTX_RULE18EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RULE18EN_SHIFT: c_int = 0;
pub const XSTORM_TOE_CONN_AG_CTX_RULE19EN_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_RULE19EN_SHIFT: c_int = 1;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED4_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED4_SHIFT: c_int = 2;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED5_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED5_SHIFT: c_int = 3;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED6_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED6_SHIFT: c_int = 4;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED7_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED7_SHIFT: c_int = 5;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED8_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED8_SHIFT: c_int = 6;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED9_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_A0_RESERVED9_SHIFT: c_int = 7;
    pub flags14: u8,
pub const XSTORM_TOE_CONN_AG_CTX_BIT16_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT16_SHIFT: c_int = 0;
pub const XSTORM_TOE_CONN_AG_CTX_BIT17_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT17_SHIFT: c_int = 1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT18_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT18_SHIFT: c_int = 2;
pub const XSTORM_TOE_CONN_AG_CTX_BIT19_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT19_SHIFT: c_int = 3;
pub const XSTORM_TOE_CONN_AG_CTX_BIT20_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT20_SHIFT: c_int = 4;
pub const XSTORM_TOE_CONN_AG_CTX_BIT21_MASK: c_uint = 0x1;
pub const XSTORM_TOE_CONN_AG_CTX_BIT21_SHIFT: c_int = 5;
pub const XSTORM_TOE_CONN_AG_CTX_CF23_MASK: c_uint = 0x3;
pub const XSTORM_TOE_CONN_AG_CTX_CF23_SHIFT: c_int = 6;
    pub byte2: u8,
    pub physical_q0: __le16,
    pub physical_q1: __le16,
    pub word2: __le16,
    pub word3: __le16,
    pub bd_prod: __le16,
    pub word5: __le16,
    pub word6: __le16,
    pub byte3: u8,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub more_to_send_seq: __le32,
    pub local_adv_wnd_seq: __le32,
    pub reg5: __le32,
    pub reg6: __le32,
    pub word7: __le16,
    pub word8: __le16,
    pub word9: __le16,
    pub word10: __le16,
    pub reg7: __le32,
    pub reg8: __le32,
    pub reg9: __le32,
    pub byte7: u8,
    pub byte8: u8,
    pub byte9: u8,
    pub byte10: u8,
    pub byte11: u8,
    pub byte12: u8,
    pub byte13: u8,
    pub byte14: u8,
    pub byte15: u8,
    pub e5_reserved: u8,
    pub word11: __le16,
    pub reg10: __le32,
    pub reg11: __le32,
    pub reg12: __le32,
    pub reg13: __le32,
    pub reg14: __le32,
    pub reg15: __le32,
    pub reg16: __le32,
    pub reg17: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_toe_conn_ag_ctx {
    pub reserved0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const TSTORM_TOE_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const TSTORM_TOE_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const TSTORM_TOE_CONN_AG_CTX_BIT2_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_BIT2_SHIFT: c_int = 2;
pub const TSTORM_TOE_CONN_AG_CTX_BIT3_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_BIT3_SHIFT: c_int = 3;
pub const TSTORM_TOE_CONN_AG_CTX_BIT4_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_BIT4_SHIFT: c_int = 4;
pub const TSTORM_TOE_CONN_AG_CTX_BIT5_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_BIT5_SHIFT: c_int = 5;
pub const TSTORM_TOE_CONN_AG_CTX_TIMEOUT_CF_MASK: c_uint = 0x3;
pub const TSTORM_TOE_CONN_AG_CTX_TIMEOUT_CF_SHIFT: c_int = 6;
    pub flags1: u8,
pub const TSTORM_TOE_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const TSTORM_TOE_CONN_AG_CTX_CF1_SHIFT: c_int = 0;
pub const TSTORM_TOE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const TSTORM_TOE_CONN_AG_CTX_CF2_SHIFT: c_int = 2;
pub const TSTORM_TOE_CONN_AG_CTX_TIMER_STOP_ALL_MASK: c_uint = 0x3;
pub const TSTORM_TOE_CONN_AG_CTX_TIMER_STOP_ALL_SHIFT: c_int = 4;
pub const TSTORM_TOE_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const TSTORM_TOE_CONN_AG_CTX_CF4_SHIFT: c_int = 6;
    pub flags2: u8,
pub const TSTORM_TOE_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const TSTORM_TOE_CONN_AG_CTX_CF5_SHIFT: c_int = 0;
pub const TSTORM_TOE_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const TSTORM_TOE_CONN_AG_CTX_CF6_SHIFT: c_int = 2;
pub const TSTORM_TOE_CONN_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const TSTORM_TOE_CONN_AG_CTX_CF7_SHIFT: c_int = 4;
pub const TSTORM_TOE_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const TSTORM_TOE_CONN_AG_CTX_CF8_SHIFT: c_int = 6;
    pub flags3: u8,
pub const TSTORM_TOE_CONN_AG_CTX_FLUSH_Q0_MASK: c_uint = 0x3;
pub const TSTORM_TOE_CONN_AG_CTX_FLUSH_Q0_SHIFT: c_int = 0;
pub const TSTORM_TOE_CONN_AG_CTX_CF10_MASK: c_uint = 0x3;
pub const TSTORM_TOE_CONN_AG_CTX_CF10_SHIFT: c_int = 2;
pub const TSTORM_TOE_CONN_AG_CTX_TIMEOUT_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_TIMEOUT_CF_EN_SHIFT: c_int = 4;
pub const TSTORM_TOE_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_CF1EN_SHIFT: c_int = 5;
pub const TSTORM_TOE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 6;
pub const TSTORM_TOE_CONN_AG_CTX_TIMER_STOP_ALL_EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_TIMER_STOP_ALL_EN_SHIFT: c_int = 7;
    pub flags4: u8,
pub const TSTORM_TOE_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_CF4EN_SHIFT: c_int = 0;
pub const TSTORM_TOE_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_CF5EN_SHIFT: c_int = 1;
pub const TSTORM_TOE_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_CF6EN_SHIFT: c_int = 2;
pub const TSTORM_TOE_CONN_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_CF7EN_SHIFT: c_int = 3;
pub const TSTORM_TOE_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_CF8EN_SHIFT: c_int = 4;
pub const TSTORM_TOE_CONN_AG_CTX_FLUSH_Q0_EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_FLUSH_Q0_EN_SHIFT: c_int = 5;
pub const TSTORM_TOE_CONN_AG_CTX_CF10EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_CF10EN_SHIFT: c_int = 6;
pub const TSTORM_TOE_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 7;
    pub flags5: u8,
pub const TSTORM_TOE_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 0;
pub const TSTORM_TOE_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const TSTORM_TOE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const TSTORM_TOE_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const TSTORM_TOE_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const TSTORM_TOE_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 5;
pub const TSTORM_TOE_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const TSTORM_TOE_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const TSTORM_TOE_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub reg3: __le32,
    pub reg4: __le32,
    pub reg5: __le32,
    pub reg6: __le32,
    pub reg7: __le32,
    pub reg8: __le32,
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_toe_conn_ag_ctx {
    pub reserved: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const USTORM_TOE_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const USTORM_TOE_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const USTORM_TOE_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const USTORM_TOE_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const USTORM_TOE_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const USTORM_TOE_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const USTORM_TOE_CONN_AG_CTX_PUSH_TIMER_CF_MASK: c_uint = 0x3;
pub const USTORM_TOE_CONN_AG_CTX_PUSH_TIMER_CF_SHIFT: c_int = 6;
    pub flags1: u8,
pub const USTORM_TOE_CONN_AG_CTX_TIMER_STOP_ALL_MASK: c_uint = 0x3;
pub const USTORM_TOE_CONN_AG_CTX_TIMER_STOP_ALL_SHIFT: c_int = 0;
pub const USTORM_TOE_CONN_AG_CTX_SLOW_PATH_CF_MASK: c_uint = 0x3;
pub const USTORM_TOE_CONN_AG_CTX_SLOW_PATH_CF_SHIFT: c_int = 2;
pub const USTORM_TOE_CONN_AG_CTX_DQ_CF_MASK: c_uint = 0x3;
pub const USTORM_TOE_CONN_AG_CTX_DQ_CF_SHIFT: c_int = 4;
pub const USTORM_TOE_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const USTORM_TOE_CONN_AG_CTX_CF6_SHIFT: c_int = 6;
    pub flags2: u8,
pub const USTORM_TOE_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const USTORM_TOE_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const USTORM_TOE_CONN_AG_CTX_PUSH_TIMER_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_PUSH_TIMER_CF_EN_SHIFT: c_int = 2;
pub const USTORM_TOE_CONN_AG_CTX_TIMER_STOP_ALL_EN_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_TIMER_STOP_ALL_EN_SHIFT: c_int = 3;
pub const USTORM_TOE_CONN_AG_CTX_SLOW_PATH_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_SLOW_PATH_CF_EN_SHIFT: c_int = 4;
pub const USTORM_TOE_CONN_AG_CTX_DQ_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_DQ_CF_EN_SHIFT: c_int = 5;
pub const USTORM_TOE_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_CF6EN_SHIFT: c_int = 6;
pub const USTORM_TOE_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 7;
    pub flags3: u8,
pub const USTORM_TOE_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 0;
pub const USTORM_TOE_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const USTORM_TOE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const USTORM_TOE_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const USTORM_TOE_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const USTORM_TOE_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 5;
pub const USTORM_TOE_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const USTORM_TOE_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const USTORM_TOE_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub word1: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub reg3: __le32,
    pub word2: __le16,
    pub word3: __le16,
}

// The toe storm context of Tstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_toe_conn_st_ctx {
    pub reserved: [__le32; 16],
}

// The toe storm context of Ustorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_toe_conn_st_ctx {
    pub reserved: [__le32; 52],
}

// toe connection context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_conn_context {
    pub ystorm_st_context: ystorm_toe_conn_st_ctx,
    pub pstorm_st_context: pstorm_toe_conn_st_ctx,
    pub pstorm_st_padding: [regpair; 2],
    pub xstorm_st_context: xstorm_toe_conn_st_ctx,
    pub xstorm_st_padding: [regpair; 2],
    pub ystorm_ag_context: ystorm_toe_conn_ag_ctx,
    pub xstorm_ag_context: xstorm_toe_conn_ag_ctx,
    pub tstorm_ag_context: tstorm_toe_conn_ag_ctx,
    pub tstorm_ag_padding: [regpair; 2],
    pub timer_context: timers_context,
    pub ustorm_ag_context: ustorm_toe_conn_ag_ctx,
    pub tstorm_st_context: tstorm_toe_conn_st_ctx,
    pub mstorm_st_context: mstorm_toe_conn_st_ctx,
    pub ustorm_st_context: ustorm_toe_conn_st_ctx,
}

// toe init ramrod header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_init_ramrod_header {
    pub first_rss: u8,
    pub num_rss: u8,
    pub reserved: [u8; 6],
}

// toe pf init parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_pf_init_params {
    pub push_timeout: __le32,
    pub grq_buffer_size: __le16,
    pub grq_sb_id: __le16,
    pub grq_sb_index: u8,
    pub max_seg_retransmit: u8,
    pub doubt_reachability: u8,
    pub ll2_rx_queue_id: u8,
    pub grq_fetch_threshold: __le16,
    pub reserved1: [u8; 2],
    pub grq_page_addr: regpair,
}

// toe tss parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_tss_params {
    pub curr_page_addr: regpair,
    pub next_page_addr: regpair,
    pub reserved0: u8,
    pub status_block_index: u8,
    pub status_block_id: __le16,
    pub reserved1: [__le16; 2],
}

// toe rss parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_rss_params {
    pub curr_page_addr: regpair,
    pub next_page_addr: regpair,
    pub reserved0: u8,
    pub status_block_index: u8,
    pub status_block_id: __le16,
    pub reserved1: [__le16; 2],
}

// toe init ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_init_ramrod_data {
    pub hdr: toe_init_ramrod_header,
    pub tcp_params: tcp_init_params,
    pub pf_params: toe_pf_init_params,
    pub tss_params: [toe_tss_params; TOE_TX_MAX_TSS_CHAINS],
    pub rss_params: [toe_rss_params; TOE_RX_MAX_RSS_CHAINS],
}

// toe offload parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_offload_params {
    pub tx_bd_page_addr: regpair,
    pub tx_app_page_addr: regpair,
    pub more_to_send_seq: __le32,
    pub rcv_indication_size: __le16,
    pub rss_tss_id: u8,
    pub ignore_grq_push: u8,
    pub rx_db_data_ptr: regpair,
}

// TOE offload ramrod data - DMAed by firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_offload_ramrod_data {
    pub tcp_ofld_params: tcp_offload_params,
    pub toe_ofld_params: toe_offload_params,
}

// TOE ramrod command IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum toe_ramrod_cmd_id {
    TOE_RAMROD_UNUSED,
    TOE_RAMROD_FUNC_INIT,
    TOE_RAMROD_INITATE_OFFLOAD,
    TOE_RAMROD_FUNC_CLOSE,
    TOE_RAMROD_SEARCHER_DELETE,
    TOE_RAMROD_TERMINATE,
    TOE_RAMROD_QUERY,
    TOE_RAMROD_UPDATE,
    TOE_RAMROD_EMPTY,
    TOE_RAMROD_RESET_SEND,
    TOE_RAMROD_INVALIDATE,
    MAX_TOE_RAMROD_CMD_ID
}

// Toe RQ buffer descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_rx_bd {
    pub addr: regpair,
    pub size: __le16,
    pub flags: __le16,
pub const TOE_RX_BD_START_MASK: c_uint = 0x1;
pub const TOE_RX_BD_START_SHIFT: c_int = 0;
pub const TOE_RX_BD_END_MASK: c_uint = 0x1;
pub const TOE_RX_BD_END_SHIFT: c_int = 1;
pub const TOE_RX_BD_NO_PUSH_MASK: c_uint = 0x1;
pub const TOE_RX_BD_NO_PUSH_SHIFT: c_int = 2;
pub const TOE_RX_BD_SPLIT_MASK: c_uint = 0x1;
pub const TOE_RX_BD_SPLIT_SHIFT: c_int = 3;
pub const TOE_RX_BD_RESERVED0_MASK: c_uint = 0xFFF;
pub const TOE_RX_BD_RESERVED0_SHIFT: c_int = 4;
    pub reserved1: __le32,
}

// TOE RX completion queue opcodes (opcode 0 is illegal)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum toe_rx_cmp_opcode {
    TOE_RX_CMP_OPCODE_GA = 1,
    TOE_RX_CMP_OPCODE_GR = 2,
    TOE_RX_CMP_OPCODE_GNI = 3,
    TOE_RX_CMP_OPCODE_GAIR = 4,
    TOE_RX_CMP_OPCODE_GAIL = 5,
    TOE_RX_CMP_OPCODE_GRI = 6,
    TOE_RX_CMP_OPCODE_GJ = 7,
    TOE_RX_CMP_OPCODE_DGI = 8,
    TOE_RX_CMP_OPCODE_CMP = 9,
    TOE_RX_CMP_OPCODE_REL = 10,
    TOE_RX_CMP_OPCODE_SKP = 11,
    TOE_RX_CMP_OPCODE_URG = 12,
    TOE_RX_CMP_OPCODE_RT_TO = 13,
    TOE_RX_CMP_OPCODE_KA_TO = 14,
    TOE_RX_CMP_OPCODE_MAX_RT = 15,
    TOE_RX_CMP_OPCODE_DBT_RE = 16,
    TOE_RX_CMP_OPCODE_SYN = 17,
    TOE_RX_CMP_OPCODE_OPT_ERR = 18,
    TOE_RX_CMP_OPCODE_FW2_TO = 19,
    TOE_RX_CMP_OPCODE_2WY_CLS = 20,
    TOE_RX_CMP_OPCODE_RST_RCV = 21,
    TOE_RX_CMP_OPCODE_FIN_RCV = 22,
    TOE_RX_CMP_OPCODE_FIN_UPL = 23,
    TOE_RX_CMP_OPCODE_INIT = 32,
    TOE_RX_CMP_OPCODE_RSS_UPDATE = 33,
    TOE_RX_CMP_OPCODE_CLOSE = 34,
    TOE_RX_CMP_OPCODE_INITIATE_OFFLOAD = 80,
    TOE_RX_CMP_OPCODE_SEARCHER_DELETE = 81,
    TOE_RX_CMP_OPCODE_TERMINATE = 82,
    TOE_RX_CMP_OPCODE_QUERY = 83,
    TOE_RX_CMP_OPCODE_RESET_SEND = 84,
    TOE_RX_CMP_OPCODE_INVALIDATE = 85,
    TOE_RX_CMP_OPCODE_EMPTY = 86,
    TOE_RX_CMP_OPCODE_UPDATE = 87,
    MAX_TOE_RX_CMP_OPCODE
}

// TOE rx ooo completion data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_rx_cqe_ooo_params {
    pub nbytes: __le32,
    pub grq_buff_id: __le16,
    pub isle_num: u8,
    pub reserved0: u8,
}

// TOE rx in order completion data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_rx_cqe_in_order_params {
    pub nbytes: __le32,
    pub grq_buff_id: __le16,
    pub reserved1: __le16,
}

// Union for TOE rx completion data
#[repr(C)]
#[derive(Copy, Clone)]
pub union toe_rx_cqe_data_union {
    pub ooo_params: toe_rx_cqe_ooo_params,
    pub in_order_params: toe_rx_cqe_in_order_params,
    pub raw_data: regpair,
}

// TOE rx completion element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_rx_cqe {
    pub icid: __le16,
    pub completion_opcode: u8,
    pub reserved0: u8,
    pub reserved1: __le32,
    pub data: toe_rx_cqe_data_union,
}

// toe RX doorbel data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_rx_db_data {
    pub local_adv_wnd_seq: __le32,
    pub reserved: [__le32; 3],
}

// Toe GRQ buffer descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_rx_grq_bd {
    pub addr: regpair,
    pub buff_id: __le16,
    pub reserved0: __le16,
    pub reserved1: __le32,
}

// Toe transmission application buffer descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_tx_app_buff_desc {
    pub next_buffer_start_seq: __le32,
    pub reserved: __le32,
}

// Toe transmission application buffer descriptor page pointer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_tx_app_buff_page_pointer {
    pub next_page_addr: regpair,
}

// Toe transmission buffer descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_tx_bd {
    pub addr: regpair,
    pub size: __le16,
    pub flags: __le16,
pub const TOE_TX_BD_PUSH_MASK: c_uint = 0x1;
pub const TOE_TX_BD_PUSH_SHIFT: c_int = 0;
pub const TOE_TX_BD_NOTIFY_MASK: c_uint = 0x1;
pub const TOE_TX_BD_NOTIFY_SHIFT: c_int = 1;
pub const TOE_TX_BD_LARGE_IO_MASK: c_uint = 0x1;
pub const TOE_TX_BD_LARGE_IO_SHIFT: c_int = 2;
pub const TOE_TX_BD_BD_CONS_MASK: c_uint = 0x1FFF;
pub const TOE_TX_BD_BD_CONS_SHIFT: c_int = 3;
    pub next_bd_start_seq: __le32,
}

// TOE completion opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum toe_tx_cmp_opcode {
    TOE_TX_CMP_OPCODE_DATA,
    TOE_TX_CMP_OPCODE_TERMINATE,
    TOE_TX_CMP_OPCODE_EMPTY,
    TOE_TX_CMP_OPCODE_RESET_SEND,
    TOE_TX_CMP_OPCODE_INVALIDATE,
    TOE_TX_CMP_OPCODE_RST_RCV,
    MAX_TOE_TX_CMP_OPCODE
}

// Toe transmission completion element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_tx_cqe {
    pub icid: __le16,
    pub opcode: u8,
    pub reserved: u8,
    pub size: __le32,
}

// Toe transmission page pointer bd
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_tx_page_pointer_bd {
    pub next_page_addr: regpair,
    pub prev_page_addr: regpair,
}

// Toe transmission completion element page pointer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_tx_page_pointer_cqe {
    pub next_page_addr: regpair,
}

// toe update parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_update_params {
    pub flags: __le16,
pub const TOE_UPDATE_PARAMS_RCV_INDICATION_SIZE_CHANGED_MASK: c_uint = 0x1;
pub const TOE_UPDATE_PARAMS_RCV_INDICATION_SIZE_CHANGED_SHIFT: c_int = 0;
pub const TOE_UPDATE_PARAMS_RESERVED_MASK: c_uint = 0x7FFF;
pub const TOE_UPDATE_PARAMS_RESERVED_SHIFT: c_int = 1;
    pub rcv_indication_size: __le16,
    pub reserved1: [__le16; 2],
}

// TOE update ramrod data - DMAed by firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_update_ramrod_data {
    pub tcp_upd_params: tcp_update_params,
    pub toe_upd_params: toe_update_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_toe_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const MSTORM_TOE_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const MSTORM_TOE_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const MSTORM_TOE_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const MSTORM_TOE_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const MSTORM_TOE_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const MSTORM_TOE_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const MSTORM_TOE_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const MSTORM_TOE_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const MSTORM_TOE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const MSTORM_TOE_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const MSTORM_TOE_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const MSTORM_TOE_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const MSTORM_TOE_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const MSTORM_TOE_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const MSTORM_TOE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const MSTORM_TOE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const MSTORM_TOE_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const MSTORM_TOE_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const MSTORM_TOE_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const MSTORM_TOE_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const MSTORM_TOE_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const MSTORM_TOE_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const MSTORM_TOE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const MSTORM_TOE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const MSTORM_TOE_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const MSTORM_TOE_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub word0: __le16,
    pub word1: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
}

// TOE doorbell data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toe_db_data {
    pub params: u8,
pub const TOE_DB_DATA_DEST_MASK: c_uint = 0x3;
pub const TOE_DB_DATA_DEST_SHIFT: c_int = 0;
pub const TOE_DB_DATA_AGG_CMD_MASK: c_uint = 0x3;
pub const TOE_DB_DATA_AGG_CMD_SHIFT: c_int = 2;
pub const TOE_DB_DATA_BYPASS_EN_MASK: c_uint = 0x1;
pub const TOE_DB_DATA_BYPASS_EN_SHIFT: c_int = 4;
pub const TOE_DB_DATA_RESERVED_MASK: c_uint = 0x1;
pub const TOE_DB_DATA_RESERVED_SHIFT: c_int = 5;
pub const TOE_DB_DATA_AGG_VAL_SEL_MASK: c_uint = 0x3;
pub const TOE_DB_DATA_AGG_VAL_SEL_SHIFT: c_int = 6;
    pub agg_flags: u8,
    pub bd_prod: __le16,
}

// rdma function init ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_close_func_ramrod_data {
    pub cnq_start_offset: u8,
    pub num_cnqs: u8,
    pub vf_id: u8,
    pub vf_valid: u8,
    pub reserved: [u8; 4],
}

// rdma function init CNQ parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_cnq_params {
    pub sb_num: __le16,
    pub sb_index: u8,
    pub num_pbl_pages: u8,
    pub reserved: __le32,
    pub pbl_base_addr: regpair,
    pub queue_zone_num: __le16,
    pub reserved1: [u8; 6],
}

// rdma create cq ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_create_cq_ramrod_data {
    pub cq_handle: regpair,
    pub pbl_addr: regpair,
    pub max_cqes: __le32,
    pub pbl_num_pages: __le16,
    pub dpi: __le16,
    pub is_two_level_pbl: u8,
    pub cnq_id: u8,
    pub pbl_log_page_size: u8,
    pub toggle_bit: u8,
    pub int_timeout: __le16,
    pub vf_id: u8,
    pub flags: u8,
pub const RDMA_CREATE_CQ_RAMROD_DATA_VF_ID_VALID_MASK: c_uint = 0x1;
pub const RDMA_CREATE_CQ_RAMROD_DATA_VF_ID_VALID_SHIFT: c_int = 0;
pub const RDMA_CREATE_CQ_RAMROD_DATA_RESERVED1_MASK: c_uint = 0x7F;
pub const RDMA_CREATE_CQ_RAMROD_DATA_RESERVED1_SHIFT: c_int = 1;
}

// rdma deregister tid ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_deregister_tid_ramrod_data {
    pub itid: __le32,
    pub reserved: __le32,
}

// rdma destroy cq output params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_destroy_cq_output_params {
    pub cnq_num: __le16,
    pub reserved0: __le16,
    pub reserved1: __le32,
}

// rdma destroy cq ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_destroy_cq_ramrod_data {
    pub output_params_addr: regpair,
}

// RDMA slow path EQ cmd IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_event_opcode {
    RDMA_EVENT_UNUSED,
    RDMA_EVENT_FUNC_INIT,
    RDMA_EVENT_FUNC_CLOSE,
    RDMA_EVENT_REGISTER_MR,
    RDMA_EVENT_DEREGISTER_MR,
    RDMA_EVENT_CREATE_CQ,
    RDMA_EVENT_RESIZE_CQ,
    RDMA_EVENT_DESTROY_CQ,
    RDMA_EVENT_CREATE_SRQ,
    RDMA_EVENT_MODIFY_SRQ,
    RDMA_EVENT_DESTROY_SRQ,
    RDMA_EVENT_START_NAMESPACE_TRACKING,
    RDMA_EVENT_STOP_NAMESPACE_TRACKING,
    MAX_RDMA_EVENT_OPCODE
}

// RDMA FW return code for slow path ramrods
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_fw_return_code {
    RDMA_RETURN_OK = 0,
    RDMA_RETURN_REGISTER_MR_BAD_STATE_ERR,
    RDMA_RETURN_DEREGISTER_MR_BAD_STATE_ERR,
    RDMA_RETURN_RESIZE_CQ_ERR,
    RDMA_RETURN_NIG_DRAIN_REQ,
    RDMA_RETURN_GENERAL_ERR,
    MAX_RDMA_FW_RETURN_CODE
}

// rdma function init header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_init_func_hdr {
    pub cnq_start_offset: u8,
    pub num_cnqs: u8,
    pub cq_ring_mode: u8,
    pub vf_id: u8,
    pub vf_valid: u8,
    pub relaxed_ordering: u8,
    pub first_reg_srq_id: __le16,
    pub reg_srq_base_addr: __le32,
    pub flags: u8,
pub const RDMA_INIT_FUNC_HDR_SEARCHER_MODE_MASK: c_uint = 0x1;
pub const RDMA_INIT_FUNC_HDR_SEARCHER_MODE_SHIFT: c_int = 0;
pub const RDMA_INIT_FUNC_HDR_PVRDMA_MODE_MASK: c_uint = 0x1;
pub const RDMA_INIT_FUNC_HDR_PVRDMA_MODE_SHIFT: c_int = 1;
pub const RDMA_INIT_FUNC_HDR_DPT_MODE_MASK: c_uint = 0x1;
pub const RDMA_INIT_FUNC_HDR_DPT_MODE_SHIFT: c_int = 2;
pub const RDMA_INIT_FUNC_HDR_RESERVED0_MASK: c_uint = 0x1F;
pub const RDMA_INIT_FUNC_HDR_RESERVED0_SHIFT: c_int = 3;
    pub dpt_byte_threshold_log: u8,
    pub dpt_common_queue_id: u8,
    pub max_num_ns_log: u8,
}

// rdma function init ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_init_func_ramrod_data {
    pub params_header: rdma_init_func_hdr,
    pub dptq_params: rdma_cnq_params,
    pub cnq_params: [rdma_cnq_params; NUM_OF_GLOBAL_QUEUES],
}

// rdma namespace tracking ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_namespace_tracking_ramrod_data {
    pub name_space: u8,
    pub reserved: [u8; 7],
}

// RDMA ramrod command IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_ramrod_cmd_id {
    RDMA_RAMROD_UNUSED,
    RDMA_RAMROD_FUNC_INIT,
    RDMA_RAMROD_FUNC_CLOSE,
    RDMA_RAMROD_REGISTER_MR,
    RDMA_RAMROD_DEREGISTER_MR,
    RDMA_RAMROD_CREATE_CQ,
    RDMA_RAMROD_RESIZE_CQ,
    RDMA_RAMROD_DESTROY_CQ,
    RDMA_RAMROD_CREATE_SRQ,
    RDMA_RAMROD_MODIFY_SRQ,
    RDMA_RAMROD_DESTROY_SRQ,
    RDMA_RAMROD_START_NS_TRACKING,
    RDMA_RAMROD_STOP_NS_TRACKING,
    MAX_RDMA_RAMROD_CMD_ID
}

// rdma register tid ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_register_tid_ramrod_data {
    pub flags: __le16,
pub const RDMA_REGISTER_TID_RAMROD_DATA_PAGE_SIZE_LOG_MASK: c_uint = 0x1F;
pub const RDMA_REGISTER_TID_RAMROD_DATA_PAGE_SIZE_LOG_SHIFT: c_int = 0;
pub const RDMA_REGISTER_TID_RAMROD_DATA_TWO_LEVEL_PBL_MASK: c_uint = 0x1;
pub const RDMA_REGISTER_TID_RAMROD_DATA_TWO_LEVEL_PBL_SHIFT: c_int = 5;
pub const RDMA_REGISTER_TID_RAMROD_DATA_ZERO_BASED_MASK: c_uint = 0x1;
pub const RDMA_REGISTER_TID_RAMROD_DATA_ZERO_BASED_SHIFT: c_int = 6;
pub const RDMA_REGISTER_TID_RAMROD_DATA_PHY_MR_MASK: c_uint = 0x1;
pub const RDMA_REGISTER_TID_RAMROD_DATA_PHY_MR_SHIFT: c_int = 7;
pub const RDMA_REGISTER_TID_RAMROD_DATA_REMOTE_READ_MASK: c_uint = 0x1;
pub const RDMA_REGISTER_TID_RAMROD_DATA_REMOTE_READ_SHIFT: c_int = 8;
pub const RDMA_REGISTER_TID_RAMROD_DATA_REMOTE_WRITE_MASK: c_uint = 0x1;
pub const RDMA_REGISTER_TID_RAMROD_DATA_REMOTE_WRITE_SHIFT: c_int = 9;
pub const RDMA_REGISTER_TID_RAMROD_DATA_REMOTE_ATOMIC_MASK: c_uint = 0x1;
pub const RDMA_REGISTER_TID_RAMROD_DATA_REMOTE_ATOMIC_SHIFT: c_int = 10;
pub const RDMA_REGISTER_TID_RAMROD_DATA_LOCAL_WRITE_MASK: c_uint = 0x1;
pub const RDMA_REGISTER_TID_RAMROD_DATA_LOCAL_WRITE_SHIFT: c_int = 11;
pub const RDMA_REGISTER_TID_RAMROD_DATA_LOCAL_READ_MASK: c_uint = 0x1;
pub const RDMA_REGISTER_TID_RAMROD_DATA_LOCAL_READ_SHIFT: c_int = 12;
pub const RDMA_REGISTER_TID_RAMROD_DATA_ENABLE_MW_BIND_MASK: c_uint = 0x1;
pub const RDMA_REGISTER_TID_RAMROD_DATA_ENABLE_MW_BIND_SHIFT: c_int = 13;
pub const RDMA_REGISTER_TID_RAMROD_DATA_RESERVED_MASK: c_uint = 0x3;
pub const RDMA_REGISTER_TID_RAMROD_DATA_RESERVED_SHIFT: c_int = 14;
    pub flags1: u8,
pub const RDMA_REGISTER_TID_RAMROD_DATA_PBL_PAGE_SIZE_LOG_MASK: c_uint = 0x1F;
pub const RDMA_REGISTER_TID_RAMROD_DATA_PBL_PAGE_SIZE_LOG_SHIFT: c_int = 0;
pub const RDMA_REGISTER_TID_RAMROD_DATA_TID_TYPE_MASK: c_uint = 0x7;
pub const RDMA_REGISTER_TID_RAMROD_DATA_TID_TYPE_SHIFT: c_int = 5;
    pub flags2: u8,
pub const RDMA_REGISTER_TID_RAMROD_DATA_DMA_MR_MASK: c_uint = 0x1;
pub const RDMA_REGISTER_TID_RAMROD_DATA_DMA_MR_SHIFT: c_int = 0;
pub const RDMA_REGISTER_TID_RAMROD_DATA_DIF_ON_HOST_FLG_MASK: c_uint = 0x1;
pub const RDMA_REGISTER_TID_RAMROD_DATA_DIF_ON_HOST_FLG_SHIFT: c_int = 1;
pub const RDMA_REGISTER_TID_RAMROD_DATA_RESERVED1_MASK: c_uint = 0x3F;
pub const RDMA_REGISTER_TID_RAMROD_DATA_RESERVED1_SHIFT: c_int = 2;
    pub key: u8,
    pub length_hi: u8,
    pub vf_id: u8,
    pub vf_valid: u8,
    pub pd: __le16,
    pub reserved2: __le16,
    pub length_lo: __le32,
    pub itid: __le32,
    pub reserved3: __le32,
    pub va: regpair,
    pub pbl_base: regpair,
    pub dif_error_addr: regpair,
    pub reserved4: [__le32; 4],
}

// rdma resize cq output params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_resize_cq_output_params {
    pub old_cq_cons: __le32,
    pub old_cq_prod: __le32,
}

// rdma resize cq ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_resize_cq_ramrod_data {
    pub flags: u8,
pub const RDMA_RESIZE_CQ_RAMROD_DATA_TOGGLE_BIT_MASK: c_uint = 0x1;
pub const RDMA_RESIZE_CQ_RAMROD_DATA_TOGGLE_BIT_SHIFT: c_int = 0;
pub const RDMA_RESIZE_CQ_RAMROD_DATA_IS_TWO_LEVEL_PBL_MASK: c_uint = 0x1;
pub const RDMA_RESIZE_CQ_RAMROD_DATA_IS_TWO_LEVEL_PBL_SHIFT: c_int = 1;
pub const RDMA_RESIZE_CQ_RAMROD_DATA_VF_ID_VALID_MASK: c_uint = 0x1;
pub const RDMA_RESIZE_CQ_RAMROD_DATA_VF_ID_VALID_SHIFT: c_int = 2;
pub const RDMA_RESIZE_CQ_RAMROD_DATA_RESERVED_MASK: c_uint = 0x1F;
pub const RDMA_RESIZE_CQ_RAMROD_DATA_RESERVED_SHIFT: c_int = 3;
    pub pbl_log_page_size: u8,
    pub pbl_num_pages: __le16,
    pub max_cqes: __le32,
    pub pbl_addr: regpair,
    pub output_params_addr: regpair,
    pub vf_id: u8,
    pub reserved1: [u8; 7],
}

// The rdma SRQ context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_srq_context {
    pub temp: [regpair; 8],
}

// rdma create qp requester ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_srq_create_ramrod_data {
    pub flags: u8,
pub const RDMA_SRQ_CREATE_RAMROD_DATA_XRC_FLAG_MASK: c_uint = 0x1;
pub const RDMA_SRQ_CREATE_RAMROD_DATA_XRC_FLAG_SHIFT: c_int = 0;
pub const RDMA_SRQ_CREATE_RAMROD_DATA_RESERVED_KEY_EN_MASK: c_uint = 0x1;
pub const RDMA_SRQ_CREATE_RAMROD_DATA_RESERVED_KEY_EN_SHIFT: c_int = 1;
pub const RDMA_SRQ_CREATE_RAMROD_DATA_RESERVED1_MASK: c_uint = 0x3F;
pub const RDMA_SRQ_CREATE_RAMROD_DATA_RESERVED1_SHIFT: c_int = 2;
    pub reserved2: u8,
    pub xrc_domain: __le16,
    pub xrc_srq_cq_cid: __le32,
    pub pbl_base_addr: regpair,
    pub pages_in_srq_pbl: __le16,
    pub pd_id: __le16,
    pub srq_id: rdma_srq_id,
    pub page_size: __le16,
    pub reserved3: __le16,
    pub reserved4: __le32,
    pub producers_addr: regpair,
}

// rdma create qp requester ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_srq_destroy_ramrod_data {
    pub srq_id: rdma_srq_id,
    pub reserved: __le32,
}

// rdma create qp requester ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_srq_modify_ramrod_data {
    pub srq_id: rdma_srq_id,
    pub wqe_limit: __le32,
}

// RDMA Tid type enumeration (for register_tid ramrod)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_tid_type {
    RDMA_TID_REGISTERED_MR,
    RDMA_TID_FMR,
    RDMA_TID_MW,
    MAX_RDMA_TID_TYPE
}

// The rdma XRC SRQ context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_xrc_srq_context {
    pub temp: [regpair; 9],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_rdma_task_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub word0: __le16,
    pub flags0: u8,
pub const TSTORM_RDMA_TASK_AG_CTX_NIBBLE0_MASK: c_uint = 0xF;
pub const TSTORM_RDMA_TASK_AG_CTX_NIBBLE0_SHIFT: c_int = 0;
pub const TSTORM_RDMA_TASK_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_BIT0_SHIFT: c_int = 4;
pub const TSTORM_RDMA_TASK_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_BIT1_SHIFT: c_int = 5;
pub const TSTORM_RDMA_TASK_AG_CTX_BIT2_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_BIT2_SHIFT: c_int = 6;
pub const TSTORM_RDMA_TASK_AG_CTX_BIT3_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_BIT3_SHIFT: c_int = 7;
    pub flags1: u8,
pub const TSTORM_RDMA_TASK_AG_CTX_BIT4_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_BIT4_SHIFT: c_int = 0;
pub const TSTORM_RDMA_TASK_AG_CTX_BIT5_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_BIT5_SHIFT: c_int = 1;
pub const TSTORM_RDMA_TASK_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const TSTORM_RDMA_TASK_AG_CTX_CF0_SHIFT: c_int = 2;
pub const TSTORM_RDMA_TASK_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const TSTORM_RDMA_TASK_AG_CTX_CF1_SHIFT: c_int = 4;
pub const TSTORM_RDMA_TASK_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const TSTORM_RDMA_TASK_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags2: u8,
pub const TSTORM_RDMA_TASK_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const TSTORM_RDMA_TASK_AG_CTX_CF3_SHIFT: c_int = 0;
pub const TSTORM_RDMA_TASK_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const TSTORM_RDMA_TASK_AG_CTX_CF4_SHIFT: c_int = 2;
pub const TSTORM_RDMA_TASK_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const TSTORM_RDMA_TASK_AG_CTX_CF5_SHIFT: c_int = 4;
pub const TSTORM_RDMA_TASK_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const TSTORM_RDMA_TASK_AG_CTX_CF6_SHIFT: c_int = 6;
    pub flags3: u8,
pub const TSTORM_RDMA_TASK_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const TSTORM_RDMA_TASK_AG_CTX_CF7_SHIFT: c_int = 0;
pub const TSTORM_RDMA_TASK_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_CF0EN_SHIFT: c_int = 2;
pub const TSTORM_RDMA_TASK_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_CF1EN_SHIFT: c_int = 3;
pub const TSTORM_RDMA_TASK_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_CF2EN_SHIFT: c_int = 4;
pub const TSTORM_RDMA_TASK_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_CF3EN_SHIFT: c_int = 5;
pub const TSTORM_RDMA_TASK_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_CF4EN_SHIFT: c_int = 6;
pub const TSTORM_RDMA_TASK_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_CF5EN_SHIFT: c_int = 7;
    pub flags4: u8,
pub const TSTORM_RDMA_TASK_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_CF6EN_SHIFT: c_int = 0;
pub const TSTORM_RDMA_TASK_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_CF7EN_SHIFT: c_int = 1;
pub const TSTORM_RDMA_TASK_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_RULE0EN_SHIFT: c_int = 2;
pub const TSTORM_RDMA_TASK_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_RULE1EN_SHIFT: c_int = 3;
pub const TSTORM_RDMA_TASK_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_RULE2EN_SHIFT: c_int = 4;
pub const TSTORM_RDMA_TASK_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_RULE3EN_SHIFT: c_int = 5;
pub const TSTORM_RDMA_TASK_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_RULE4EN_SHIFT: c_int = 6;
pub const TSTORM_RDMA_TASK_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const TSTORM_RDMA_TASK_AG_CTX_RULE5EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub word1: __le16,
    pub reg0: __le32,
    pub byte3: u8,
    pub byte4: u8,
    pub word2: __le16,
    pub word3: __le16,
    pub word4: __le16,
    pub reg1: __le32,
    pub reg2: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_rdma_conn_ag_ctx {
    pub reserved: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const USTORM_RDMA_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const USTORM_RDMA_CONN_AG_CTX_DIF_ERROR_REPORTED_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_DIF_ERROR_REPORTED_SHIFT: c_int = 1;
pub const USTORM_RDMA_CONN_AG_CTX_FLUSH_Q0_CF_MASK: c_uint = 0x3;
pub const USTORM_RDMA_CONN_AG_CTX_FLUSH_Q0_CF_SHIFT: c_int = 2;
pub const USTORM_RDMA_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const USTORM_RDMA_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const USTORM_RDMA_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const USTORM_RDMA_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const USTORM_RDMA_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const USTORM_RDMA_CONN_AG_CTX_CF3_SHIFT: c_int = 0;
pub const USTORM_RDMA_CONN_AG_CTX_CQ_ARM_SE_CF_MASK: c_uint = 0x3;
pub const USTORM_RDMA_CONN_AG_CTX_CQ_ARM_SE_CF_SHIFT: c_int = 2;
pub const USTORM_RDMA_CONN_AG_CTX_CQ_ARM_CF_MASK: c_uint = 0x3;
pub const USTORM_RDMA_CONN_AG_CTX_CQ_ARM_CF_SHIFT: c_int = 4;
pub const USTORM_RDMA_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const USTORM_RDMA_CONN_AG_CTX_CF6_SHIFT: c_int = 6;
    pub flags2: u8,
pub const USTORM_RDMA_CONN_AG_CTX_FLUSH_Q0_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_FLUSH_Q0_CF_EN_SHIFT: c_int = 0;
pub const USTORM_RDMA_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const USTORM_RDMA_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const USTORM_RDMA_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_CF3EN_SHIFT: c_int = 3;
pub const USTORM_RDMA_CONN_AG_CTX_CQ_ARM_SE_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_CQ_ARM_SE_CF_EN_SHIFT: c_int = 4;
pub const USTORM_RDMA_CONN_AG_CTX_CQ_ARM_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_CQ_ARM_CF_EN_SHIFT: c_int = 5;
pub const USTORM_RDMA_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_CF6EN_SHIFT: c_int = 6;
pub const USTORM_RDMA_CONN_AG_CTX_CQ_SE_EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_CQ_SE_EN_SHIFT: c_int = 7;
    pub flags3: u8,
pub const USTORM_RDMA_CONN_AG_CTX_CQ_EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_CQ_EN_SHIFT: c_int = 0;
pub const USTORM_RDMA_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const USTORM_RDMA_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const USTORM_RDMA_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const USTORM_RDMA_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const USTORM_RDMA_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 5;
pub const USTORM_RDMA_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const USTORM_RDMA_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const USTORM_RDMA_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub nvmf_only: u8,
    pub conn_dpi: __le16,
    pub word1: __le16,
    pub cq_cons: __le32,
    pub cq_se_prod: __le32,
    pub cq_prod: __le32,
    pub reg3: __le32,
    pub int_timeout: __le16,
    pub word3: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_roce_conn_ag_ctx {
    pub reserved0: u8,
    pub state: u8,
    pub flags0: u8,
pub const XSTORM_ROCE_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT2_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT2_SHIFT: c_int = 2;
pub const XSTORM_ROCE_CONN_AG_CTX_EXIST_IN_QM3_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_EXIST_IN_QM3_SHIFT: c_int = 3;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT4_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT4_SHIFT: c_int = 4;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT5_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT5_SHIFT: c_int = 5;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT6_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT6_SHIFT: c_int = 6;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT7_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT7_SHIFT: c_int = 7;
    pub flags1: u8,
pub const XSTORM_ROCE_CONN_AG_CTX_BIT8_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT8_SHIFT: c_int = 0;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT9_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT9_SHIFT: c_int = 1;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT10_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT10_SHIFT: c_int = 2;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT11_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT11_SHIFT: c_int = 3;
pub const XSTORM_ROCE_CONN_AG_CTX_MSDM_FLUSH_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_MSDM_FLUSH_SHIFT: c_int = 4;
pub const XSTORM_ROCE_CONN_AG_CTX_MSEM_FLUSH_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_MSEM_FLUSH_SHIFT: c_int = 5;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT14_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT14_SHIFT: c_int = 6;
pub const XSTORM_ROCE_CONN_AG_CTX_YSTORM_FLUSH_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_YSTORM_FLUSH_SHIFT: c_int = 7;
    pub flags2: u8,
pub const XSTORM_ROCE_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF0_SHIFT: c_int = 0;
pub const XSTORM_ROCE_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF1_SHIFT: c_int = 2;
pub const XSTORM_ROCE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF2_SHIFT: c_int = 4;
pub const XSTORM_ROCE_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF3_SHIFT: c_int = 6;
    pub flags3: u8,
pub const XSTORM_ROCE_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF4_SHIFT: c_int = 0;
pub const XSTORM_ROCE_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF5_SHIFT: c_int = 2;
pub const XSTORM_ROCE_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF6_SHIFT: c_int = 4;
pub const XSTORM_ROCE_CONN_AG_CTX_FLUSH_Q0_CF_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_FLUSH_Q0_CF_SHIFT: c_int = 6;
    pub flags4: u8,
pub const XSTORM_ROCE_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF8_SHIFT: c_int = 0;
pub const XSTORM_ROCE_CONN_AG_CTX_CF9_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF9_SHIFT: c_int = 2;
pub const XSTORM_ROCE_CONN_AG_CTX_CF10_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF10_SHIFT: c_int = 4;
pub const XSTORM_ROCE_CONN_AG_CTX_CF11_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF11_SHIFT: c_int = 6;
    pub flags5: u8,
pub const XSTORM_ROCE_CONN_AG_CTX_CF12_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF12_SHIFT: c_int = 0;
pub const XSTORM_ROCE_CONN_AG_CTX_CF13_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF13_SHIFT: c_int = 2;
pub const XSTORM_ROCE_CONN_AG_CTX_CF14_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF14_SHIFT: c_int = 4;
pub const XSTORM_ROCE_CONN_AG_CTX_CF15_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF15_SHIFT: c_int = 6;
    pub flags6: u8,
pub const XSTORM_ROCE_CONN_AG_CTX_CF16_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF16_SHIFT: c_int = 0;
pub const XSTORM_ROCE_CONN_AG_CTX_CF17_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF17_SHIFT: c_int = 2;
pub const XSTORM_ROCE_CONN_AG_CTX_CF18_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF18_SHIFT: c_int = 4;
pub const XSTORM_ROCE_CONN_AG_CTX_CF19_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF19_SHIFT: c_int = 6;
    pub flags7: u8,
pub const XSTORM_ROCE_CONN_AG_CTX_CF20_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF20_SHIFT: c_int = 0;
pub const XSTORM_ROCE_CONN_AG_CTX_CF21_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF21_SHIFT: c_int = 2;
pub const XSTORM_ROCE_CONN_AG_CTX_SLOW_PATH_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_SLOW_PATH_SHIFT: c_int = 4;
pub const XSTORM_ROCE_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF0EN_SHIFT: c_int = 6;
pub const XSTORM_ROCE_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags8: u8,
pub const XSTORM_ROCE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF3EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF4EN_SHIFT: c_int = 2;
pub const XSTORM_ROCE_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF5EN_SHIFT: c_int = 3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF6EN_SHIFT: c_int = 4;
pub const XSTORM_ROCE_CONN_AG_CTX_FLUSH_Q0_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_FLUSH_Q0_CF_EN_SHIFT: c_int = 5;
pub const XSTORM_ROCE_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF8EN_SHIFT: c_int = 6;
pub const XSTORM_ROCE_CONN_AG_CTX_CF9EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF9EN_SHIFT: c_int = 7;
    pub flags9: u8,
pub const XSTORM_ROCE_CONN_AG_CTX_CF10EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF10EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_CONN_AG_CTX_CF11EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF11EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF12EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF12EN_SHIFT: c_int = 2;
pub const XSTORM_ROCE_CONN_AG_CTX_CF13EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF13EN_SHIFT: c_int = 3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF14EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF14EN_SHIFT: c_int = 4;
pub const XSTORM_ROCE_CONN_AG_CTX_CF15EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF15EN_SHIFT: c_int = 5;
pub const XSTORM_ROCE_CONN_AG_CTX_CF16EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF16EN_SHIFT: c_int = 6;
pub const XSTORM_ROCE_CONN_AG_CTX_CF17EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF17EN_SHIFT: c_int = 7;
    pub flags10: u8,
pub const XSTORM_ROCE_CONN_AG_CTX_CF18EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF18EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_CONN_AG_CTX_CF19EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF19EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF20EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF20EN_SHIFT: c_int = 2;
pub const XSTORM_ROCE_CONN_AG_CTX_CF21EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF21EN_SHIFT: c_int = 3;
pub const XSTORM_ROCE_CONN_AG_CTX_SLOW_PATH_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_SLOW_PATH_EN_SHIFT: c_int = 4;
pub const XSTORM_ROCE_CONN_AG_CTX_CF23EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_CF23EN_SHIFT: c_int = 5;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 6;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 7;
    pub flags11: u8,
pub const XSTORM_ROCE_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 2;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 3;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 4;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 5;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED1_SHIFT: c_int = 6;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE9EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE9EN_SHIFT: c_int = 7;
    pub flags12: u8,
pub const XSTORM_ROCE_CONN_AG_CTX_RULE10EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE10EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE11EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE11EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED2_SHIFT: c_int = 2;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED3_SHIFT: c_int = 3;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE14EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE14EN_SHIFT: c_int = 4;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE15EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE15EN_SHIFT: c_int = 5;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE16EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE16EN_SHIFT: c_int = 6;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE17EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE17EN_SHIFT: c_int = 7;
    pub flags13: u8,
pub const XSTORM_ROCE_CONN_AG_CTX_RULE18EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE18EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE19EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RULE19EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED4_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED4_SHIFT: c_int = 2;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED5_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED5_SHIFT: c_int = 3;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED6_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED6_SHIFT: c_int = 4;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED7_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED7_SHIFT: c_int = 5;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED8_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED8_SHIFT: c_int = 6;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED9_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_A0_RESERVED9_SHIFT: c_int = 7;
    pub flags14: u8,
pub const XSTORM_ROCE_CONN_AG_CTX_MIGRATION_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_MIGRATION_SHIFT: c_int = 0;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT17_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_BIT17_SHIFT: c_int = 1;
pub const XSTORM_ROCE_CONN_AG_CTX_DPM_PORT_NUM_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_DPM_PORT_NUM_SHIFT: c_int = 2;
pub const XSTORM_ROCE_CONN_AG_CTX_RESERVED_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_RESERVED_SHIFT: c_int = 4;
pub const XSTORM_ROCE_CONN_AG_CTX_ROCE_EDPM_ENABLE_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_CONN_AG_CTX_ROCE_EDPM_ENABLE_SHIFT: c_int = 5;
pub const XSTORM_ROCE_CONN_AG_CTX_CF23_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_CONN_AG_CTX_CF23_SHIFT: c_int = 6;
    pub byte2: u8,
    pub physical_q0: __le16,
    pub word1: __le16,
    pub word2: __le16,
    pub word3: __le16,
    pub word4: __le16,
    pub word5: __le16,
    pub conn_dpi: __le16,
    pub byte3: u8,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub snd_nxt_psn: __le32,
    pub reg4: __le32,
    pub reg5: __le32,
    pub reg6: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_roce_conn_ag_ctx {
    pub reserved0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const TSTORM_ROCE_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const TSTORM_ROCE_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const TSTORM_ROCE_CONN_AG_CTX_BIT2_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_BIT2_SHIFT: c_int = 2;
pub const TSTORM_ROCE_CONN_AG_CTX_BIT3_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_BIT3_SHIFT: c_int = 3;
pub const TSTORM_ROCE_CONN_AG_CTX_BIT4_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_BIT4_SHIFT: c_int = 4;
pub const TSTORM_ROCE_CONN_AG_CTX_BIT5_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_BIT5_SHIFT: c_int = 5;
pub const TSTORM_ROCE_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_CONN_AG_CTX_CF0_SHIFT: c_int = 6;
    pub flags1: u8,
pub const TSTORM_ROCE_CONN_AG_CTX_MSTORM_FLUSH_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_CONN_AG_CTX_MSTORM_FLUSH_CF_SHIFT: c_int = 0;
pub const TSTORM_ROCE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_CONN_AG_CTX_CF2_SHIFT: c_int = 2;
pub const TSTORM_ROCE_CONN_AG_CTX_TIMER_STOP_ALL_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_CONN_AG_CTX_TIMER_STOP_ALL_CF_SHIFT: c_int = 4;
pub const TSTORM_ROCE_CONN_AG_CTX_FLUSH_Q0_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_CONN_AG_CTX_FLUSH_Q0_CF_SHIFT: c_int = 6;
    pub flags2: u8,
pub const TSTORM_ROCE_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_CONN_AG_CTX_CF5_SHIFT: c_int = 0;
pub const TSTORM_ROCE_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_CONN_AG_CTX_CF6_SHIFT: c_int = 2;
pub const TSTORM_ROCE_CONN_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_CONN_AG_CTX_CF7_SHIFT: c_int = 4;
pub const TSTORM_ROCE_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_CONN_AG_CTX_CF8_SHIFT: c_int = 6;
    pub flags3: u8,
pub const TSTORM_ROCE_CONN_AG_CTX_CF9_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_CONN_AG_CTX_CF9_SHIFT: c_int = 0;
pub const TSTORM_ROCE_CONN_AG_CTX_CF10_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_CONN_AG_CTX_CF10_SHIFT: c_int = 2;
pub const TSTORM_ROCE_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_CF0EN_SHIFT: c_int = 4;
pub const TSTORM_ROCE_CONN_AG_CTX_MSTORM_FLUSH_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_MSTORM_FLUSH_CF_EN_SHIFT: c_int = 5;
pub const TSTORM_ROCE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 6;
pub const TSTORM_ROCE_CONN_AG_CTX_TIMER_STOP_ALL_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_TIMER_STOP_ALL_CF_EN_SHIFT: c_int = 7;
    pub flags4: u8,
pub const TSTORM_ROCE_CONN_AG_CTX_FLUSH_Q0_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_FLUSH_Q0_CF_EN_SHIFT: c_int = 0;
pub const TSTORM_ROCE_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_CF5EN_SHIFT: c_int = 1;
pub const TSTORM_ROCE_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_CF6EN_SHIFT: c_int = 2;
pub const TSTORM_ROCE_CONN_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_CF7EN_SHIFT: c_int = 3;
pub const TSTORM_ROCE_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_CF8EN_SHIFT: c_int = 4;
pub const TSTORM_ROCE_CONN_AG_CTX_CF9EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_CF9EN_SHIFT: c_int = 5;
pub const TSTORM_ROCE_CONN_AG_CTX_CF10EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_CF10EN_SHIFT: c_int = 6;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 7;
    pub flags5: u8,
pub const TSTORM_ROCE_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 0;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 5;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub reg3: __le32,
    pub reg4: __le32,
    pub reg5: __le32,
    pub reg6: __le32,
    pub reg7: __le32,
    pub reg8: __le32,
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub byte4: u8,
    pub byte5: u8,
    pub word1: __le16,
    pub word2: __le16,
    pub word3: __le16,
    pub reg9: __le32,
    pub reg10: __le32,
}

// The roce storm context of Ystorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_roce_conn_st_ctx {
    pub temp: [regpair; 2],
}

// The roce storm context of Mstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstorm_roce_conn_st_ctx {
    pub temp: [regpair; 16],
}

// The roce storm context of Xstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_roce_conn_st_ctx {
    pub temp: [regpair; 24],
}

// The roce storm context of Tstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_roce_conn_st_ctx {
    pub temp: [regpair; 30],
}

// The roce storm context of Mstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_roce_conn_st_ctx {
    pub temp: [regpair; 6],
}

// The roce storm context of Ustorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_roce_conn_st_ctx {
    pub temp: [regpair; 14],
}

// roce connection context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_conn_context {
    pub ystorm_st_context: ystorm_roce_conn_st_ctx,
    pub ystorm_st_padding: [regpair; 2],
    pub pstorm_st_context: pstorm_roce_conn_st_ctx,
    pub xstorm_st_context: xstorm_roce_conn_st_ctx,
    pub xstorm_ag_context: xstorm_roce_conn_ag_ctx,
    pub tstorm_ag_context: tstorm_roce_conn_ag_ctx,
    pub timer_context: timers_context,
    pub ustorm_ag_context: ustorm_rdma_conn_ag_ctx,
    pub tstorm_st_context: tstorm_roce_conn_st_ctx,
    pub tstorm_st_padding: [regpair; 2],
    pub mstorm_st_context: mstorm_roce_conn_st_ctx,
    pub mstorm_st_padding: [regpair; 2],
    pub ustorm_st_context: ustorm_roce_conn_st_ctx,
    pub ustorm_st_padding: [regpair; 2],
}

// roce cqes statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_cqe_stats {
    pub req_cqe_error: __le32,
    pub req_remote_access_errors: __le32,
    pub req_remote_invalid_request: __le32,
    pub resp_cqe_error: __le32,
    pub resp_local_length_error: __le32,
    pub reserved: __le32,
}

// roce create qp requester ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_create_qp_req_ramrod_data {
    pub flags: __le16,
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_ROCE_FLAVOR_MASK: c_uint = 0x3;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_ROCE_FLAVOR_SHIFT: c_int = 0;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_FMR_AND_RESERVED_EN_MASK: c_uint = 0x1;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_FMR_AND_RESERVED_EN_SHIFT: c_int = 2;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_SIGNALED_COMP_MASK: c_uint = 0x1;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_SIGNALED_COMP_SHIFT: c_int = 3;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_PRI_MASK: c_uint = 0x7;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_PRI_SHIFT: c_int = 4;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_XRC_FLAG_MASK: c_uint = 0x1;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_XRC_FLAG_SHIFT: c_int = 7;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_ERR_RETRY_CNT_MASK: c_uint = 0xF;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_ERR_RETRY_CNT_SHIFT: c_int = 8;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_RNR_NAK_CNT_MASK: c_uint = 0xF;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_RNR_NAK_CNT_SHIFT: c_int = 12;
    pub max_ord: u8,
    pub traffic_class: u8,
    pub hop_limit: u8,
    pub orq_num_pages: u8,
    pub p_key: __le16,
    pub flow_label: __le32,
    pub dst_qp_id: __le32,
    pub ack_timeout_val: __le32,
    pub initial_psn: __le32,
    pub mtu: __le16,
    pub pd: __le16,
    pub sq_num_pages: __le16,
    pub low_latency_phy_queue: __le16,
    pub sq_pbl_addr: regpair,
    pub orq_pbl_addr: regpair,
    pub local_mac_addr: [__le16; 3],
    pub remote_mac_addr: [__le16; 3],
    pub vlan_id: __le16,
    pub udp_src_port: __le16,
    pub src_gid: [__le32; 4],
    pub dst_gid: [__le32; 4],
    pub cq_cid: __le32,
    pub qp_handle_for_cqe: regpair,
    pub qp_handle_for_async: regpair,
    pub stats_counter_id: u8,
    pub vf_id: u8,
    pub vport_id: u8,
    pub flags2: u8,
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_EDPM_MODE_MASK: c_uint = 0x1;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_EDPM_MODE_SHIFT: c_int = 0;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_VF_ID_VALID_MASK: c_uint = 0x1;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_VF_ID_VALID_SHIFT: c_int = 1;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_FORCE_LB_MASK: c_uint = 0x1;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_FORCE_LB_SHIFT: c_int = 2;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_RESERVED_MASK: c_uint = 0x1F;
pub const ROCE_CREATE_QP_REQ_RAMROD_DATA_RESERVED_SHIFT: c_int = 3;
    pub name_space: u8,
    pub reserved3: [u8; 3],
    pub regular_latency_phy_queue: __le16,
    pub dpi: __le16,
}

// roce create qp responder ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_create_qp_resp_ramrod_data {
    pub flags: __le32,
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_ROCE_FLAVOR_MASK: c_uint = 0x3;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_ROCE_FLAVOR_SHIFT: c_int = 0;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_RDMA_RD_EN_MASK: c_uint = 0x1;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_RDMA_RD_EN_SHIFT: c_int = 2;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_RDMA_WR_EN_MASK: c_uint = 0x1;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_RDMA_WR_EN_SHIFT: c_int = 3;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_ATOMIC_EN_MASK: c_uint = 0x1;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_ATOMIC_EN_SHIFT: c_int = 4;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_SRQ_FLG_MASK: c_uint = 0x1;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_SRQ_FLG_SHIFT: c_int = 5;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_E2E_FLOW_CONTROL_EN_MASK: c_uint = 0x1;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_E2E_FLOW_CONTROL_EN_SHIFT: c_int = 6;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_RESERVED_KEY_EN_MASK: c_uint = 0x1;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_RESERVED_KEY_EN_SHIFT: c_int = 7;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_PRI_MASK: c_uint = 0x7;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_PRI_SHIFT: c_int = 8;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_MIN_RNR_NAK_TIMER_MASK: c_uint = 0x1F;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_MIN_RNR_NAK_TIMER_SHIFT: c_int = 11;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_XRC_FLAG_MASK: c_uint = 0x1;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_XRC_FLAG_SHIFT: c_int = 16;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_VF_ID_VALID_MASK: c_uint = 0x1;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_VF_ID_VALID_SHIFT: c_int = 17;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_FORCE_LB_MASK: c_uint = 0x1;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_FORCE_LB_SHIFT: c_int = 18;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_RESERVED_MASK: c_uint = 0x1FFF;
pub const ROCE_CREATE_QP_RESP_RAMROD_DATA_RESERVED_SHIFT: c_int = 19;
    pub xrc_domain: __le16,
    pub max_ird: u8,
    pub traffic_class: u8,
    pub hop_limit: u8,
    pub irq_num_pages: u8,
    pub p_key: __le16,
    pub flow_label: __le32,
    pub dst_qp_id: __le32,
    pub stats_counter_id: u8,
    pub reserved1: u8,
    pub mtu: __le16,
    pub initial_psn: __le32,
    pub pd: __le16,
    pub rq_num_pages: __le16,
    pub srq_id: rdma_srq_id,
    pub rq_pbl_addr: regpair,
    pub irq_pbl_addr: regpair,
    pub local_mac_addr: [__le16; 3],
    pub remote_mac_addr: [__le16; 3],
    pub vlan_id: __le16,
    pub udp_src_port: __le16,
    pub src_gid: [__le32; 4],
    pub dst_gid: [__le32; 4],
    pub qp_handle_for_cqe: regpair,
    pub qp_handle_for_async: regpair,
    pub low_latency_phy_queue: __le16,
    pub vf_id: u8,
    pub vport_id: u8,
    pub cq_cid: __le32,
    pub regular_latency_phy_queue: __le16,
    pub dpi: __le16,
    pub src_qp_id: __le32,
    pub name_space: u8,
    pub reserved3: [u8; 3],
}

// RoCE Create Suspended qp requester runtime ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_create_suspended_qp_req_runtime_ramrod_data {
    pub flags: __le32,
pub const ROCE_CREATE_SUSPENDED_QP_REQ_RUNTIME_RAMROD_DATA_ERR_FLG_MASK: c_uint = 0x1;
pub const ROCE_CREATE_SUSPENDED_QP_REQ_RUNTIME_RAMROD_DATA_ERR_FLG_SHIFT: c_int = 0;

pub const ROCE_CREATE_SUSPENDED_QP_REQ_RUNTIME_RAMROD_DATA_RESERVED0_SHIFT: c_int = 1;
    pub send_msg_psn: __le32,
    pub inflight_sends: __le32,
    pub ssn: __le32,
}

// RoCE Create Suspended QP requester ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_create_suspended_qp_req_ramrod_data {
    pub qp_params: roce_create_qp_req_ramrod_data,
}

// RoCE Create Suspended QP responder runtime params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_create_suspended_qp_resp_runtime_params {
    pub flags: __le32,
pub const ROCE_CREATE_SUSPENDED_QP_RESP_RUNTIME_PARAMS_ERR_FLG_MASK: c_uint = 0x1;
pub const ROCE_CREATE_SUSPENDED_QP_RESP_RUNTIME_PARAMS_ERR_FLG_SHIFT: c_int = 0;
pub const ROCE_CREATE_SUSPENDED_QP_RESP_RUNTIME_PARAMS_RDMA_ACTIVE_MASK: c_uint = 0x1;
pub const ROCE_CREATE_SUSPENDED_QP_RESP_RUNTIME_PARAMS_RDMA_ACTIVE_SHIFT: c_int = 1;
pub const ROCE_CREATE_SUSPENDED_QP_RESP_RUNTIME_PARAMS_RESERVED0_MASK: c_uint = 0x3FFFFFFF;
pub const ROCE_CREATE_SUSPENDED_QP_RESP_RUNTIME_PARAMS_RESERVED0_SHIFT: c_int = 2;
    pub receive_msg_psn: __le32,
    pub inflight_receives: __le32,
    pub rmsn: __le32,
    pub rdma_key: __le32,
    pub rdma_va: regpair,
    pub rdma_length: __le32,
    pub num_rdb_entries: __le32,
    pub resreved: __le32,
}

// RoCE RDB array entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_resp_qp_rdb_entry {
    pub atomic_data: regpair,
    pub va: regpair,
    pub psn: __le32,
    pub rkey: __le32,
    pub byte_count: __le32,
    pub op_type: u8,
    pub reserved: [u8; 3],
}

// RoCE Create Suspended QP responder runtime ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_create_suspended_qp_resp_runtime_ramrod_data {
    pub params: roce_create_suspended_qp_resp_runtime_params,
}

// RoCE Create Suspended QP responder ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_create_suspended_qp_resp_ramrod_data {
}

// RoCE create ud qp ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_create_ud_qp_ramrod_data {
    pub local_mac_addr: [__le16; 3],
    pub vlan_id: __le16,
    pub src_qp_id: __le32,
    pub name_space: u8,
    pub reserved: [u8; 3],
}

// roce DCQCN received statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_dcqcn_received_stats {
    pub ecn_pkt_rcv: regpair,
    pub cnp_pkt_rcv: regpair,
    pub cnp_pkt_reject: regpair,
}

// roce DCQCN sent statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_dcqcn_sent_stats {
    pub cnp_pkt_sent: regpair,
}

// RoCE destroy qp requester output params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_destroy_qp_req_output_params {
    pub cq_prod: __le32,
    pub reserved: __le32,
}

// RoCE destroy qp requester ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_destroy_qp_req_ramrod_data {
    pub output_params_addr: regpair,
}

// RoCE destroy qp responder output params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_destroy_qp_resp_output_params {
    pub cq_prod: __le32,
    pub reserved: __le32,
}

// RoCE destroy qp responder ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_destroy_qp_resp_ramrod_data {
    pub output_params_addr: regpair,
    pub src_qp_id: __le32,
    pub reserved: __le32,
}

// RoCE destroy ud qp ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_destroy_ud_qp_ramrod_data {
    pub src_qp_id: __le32,
    pub reserved: __le32,
}

// roce error statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_error_stats {
    pub resp_remote_access_errors: __le32,
    pub reserved: __le32,
}

// roce special events statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_events_stats {
    pub silent_drops: __le32,
    pub rnr_naks_sent: __le32,
    pub retransmit_count: __le32,
    pub icrc_error_count: __le32,
    pub implied_nak_seq_err: __le32,
    pub duplicate_request: __le32,
    pub local_ack_timeout_err: __le32,
    pub out_of_sequence: __le32,
    pub packet_seq_err: __le32,
    pub rnr_nak_retry_err: __le32,
}

// roce slow path EQ cmd IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum roce_event_opcode {
    ROCE_EVENT_CREATE_QP = 13,
    ROCE_EVENT_MODIFY_QP,
    ROCE_EVENT_QUERY_QP,
    ROCE_EVENT_DESTROY_QP,
    ROCE_EVENT_CREATE_UD_QP,
    ROCE_EVENT_DESTROY_UD_QP,
    ROCE_EVENT_FUNC_UPDATE,
    ROCE_EVENT_SUSPEND_QP,
    ROCE_EVENT_QUERY_SUSPENDED_QP,
    ROCE_EVENT_CREATE_SUSPENDED_QP,
    ROCE_EVENT_RESUME_QP,
    ROCE_EVENT_SUSPEND_UD_QP,
    ROCE_EVENT_RESUME_UD_QP,
    ROCE_EVENT_CREATE_SUSPENDED_UD_QP,
    ROCE_EVENT_FLUSH_DPT_QP,
    MAX_ROCE_EVENT_OPCODE
}

// roce func init ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_init_func_params {
    pub ll2_queue_id: u8,
    pub cnp_vlan_priority: u8,
    pub cnp_dscp: u8,
    pub flags: u8,
pub const ROCE_INIT_FUNC_PARAMS_DCQCN_NP_EN_MASK: c_uint = 0x1;
pub const ROCE_INIT_FUNC_PARAMS_DCQCN_NP_EN_SHIFT: c_int = 0;
pub const ROCE_INIT_FUNC_PARAMS_DCQCN_RP_EN_MASK: c_uint = 0x1;
pub const ROCE_INIT_FUNC_PARAMS_DCQCN_RP_EN_SHIFT: c_int = 1;
pub const ROCE_INIT_FUNC_PARAMS_RESERVED0_MASK: c_uint = 0x3F;
pub const ROCE_INIT_FUNC_PARAMS_RESERVED0_SHIFT: c_int = 2;
    pub cnp_send_timeout: __le32,
    pub rl_offset: __le16,
    pub rl_count_log: u8,
    pub reserved1: [u8; 5],
}

// roce func init ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_init_func_ramrod_data {
    pub rdma: rdma_init_func_ramrod_data,
    pub roce: roce_init_func_params,
}

// roce_ll2_cqe_data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_ll2_cqe_data {
    pub name_space: u8,
    pub flags: u8,
pub const ROCE_LL2_CQE_DATA_QP_SUSPENDED_MASK: c_uint = 0x1;
pub const ROCE_LL2_CQE_DATA_QP_SUSPENDED_SHIFT: c_int = 0;
pub const ROCE_LL2_CQE_DATA_RESERVED0_MASK: c_uint = 0x7F;
pub const ROCE_LL2_CQE_DATA_RESERVED0_SHIFT: c_int = 1;
    pub reserved1: [u8; 2],
    pub cid: __le32,
}

// roce modify qp requester ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_modify_qp_req_ramrod_data {
    pub flags: __le16,
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_MOVE_TO_ERR_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_MOVE_TO_ERR_FLG_SHIFT: c_int = 0;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_MOVE_TO_SQD_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_MOVE_TO_SQD_FLG_SHIFT: c_int = 1;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_EN_SQD_ASYNC_NOTIFY_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_EN_SQD_ASYNC_NOTIFY_SHIFT: c_int = 2;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_P_KEY_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_P_KEY_FLG_SHIFT: c_int = 3;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_ADDRESS_VECTOR_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_ADDRESS_VECTOR_FLG_SHIFT: c_int = 4;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_MAX_ORD_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_MAX_ORD_FLG_SHIFT: c_int = 5;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_RNR_NAK_CNT_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_RNR_NAK_CNT_FLG_SHIFT: c_int = 6;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_ERR_RETRY_CNT_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_ERR_RETRY_CNT_FLG_SHIFT: c_int = 7;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_ACK_TIMEOUT_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_ACK_TIMEOUT_FLG_SHIFT: c_int = 8;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_PRI_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_PRI_FLG_SHIFT: c_int = 9;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_PRI_MASK: c_uint = 0x7;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_PRI_SHIFT: c_int = 10;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_PHYSICAL_QUEUE_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_PHYSICAL_QUEUE_FLG_SHIFT: c_int = 13;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_FORCE_LB_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_FORCE_LB_SHIFT: c_int = 14;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_RESERVED1_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_RESERVED1_SHIFT: c_int = 15;
    pub fields: u8,
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_ERR_RETRY_CNT_MASK: c_uint = 0xF;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_ERR_RETRY_CNT_SHIFT: c_int = 0;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_RNR_NAK_CNT_MASK: c_uint = 0xF;
pub const ROCE_MODIFY_QP_REQ_RAMROD_DATA_RNR_NAK_CNT_SHIFT: c_int = 4;
    pub max_ord: u8,
    pub traffic_class: u8,
    pub hop_limit: u8,
    pub p_key: __le16,
    pub flow_label: __le32,
    pub ack_timeout_val: __le32,
    pub mtu: __le16,
    pub reserved2: __le16,
    pub reserved3: [__le32; 2],
    pub low_latency_phy_queue: __le16,
    pub regular_latency_phy_queue: __le16,
    pub src_gid: [__le32; 4],
    pub dst_gid: [__le32; 4],
}

// roce modify qp responder ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_modify_qp_resp_ramrod_data {
    pub flags: __le16,
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_MOVE_TO_ERR_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_MOVE_TO_ERR_FLG_SHIFT: c_int = 0;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_RDMA_RD_EN_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_RDMA_RD_EN_SHIFT: c_int = 1;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_RDMA_WR_EN_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_RDMA_WR_EN_SHIFT: c_int = 2;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_ATOMIC_EN_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_ATOMIC_EN_SHIFT: c_int = 3;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_P_KEY_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_P_KEY_FLG_SHIFT: c_int = 4;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_ADDRESS_VECTOR_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_ADDRESS_VECTOR_FLG_SHIFT: c_int = 5;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_MAX_IRD_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_MAX_IRD_FLG_SHIFT: c_int = 6;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_PRI_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_PRI_FLG_SHIFT: c_int = 7;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_MIN_RNR_NAK_TIMER_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_MIN_RNR_NAK_TIMER_FLG_SHIFT: c_int = 8;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_RDMA_OPS_EN_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_RDMA_OPS_EN_FLG_SHIFT: c_int = 9;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_PHYSICAL_QUEUE_FLG_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_PHYSICAL_QUEUE_FLG_SHIFT: c_int = 10;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_FORCE_LB_MASK: c_uint = 0x1;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_FORCE_LB_SHIFT: c_int = 11;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_RESERVED1_MASK: c_uint = 0xF;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_RESERVED1_SHIFT: c_int = 12;
    pub fields: u8,
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_PRI_MASK: c_uint = 0x7;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_PRI_SHIFT: c_int = 0;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_MIN_RNR_NAK_TIMER_MASK: c_uint = 0x1F;
pub const ROCE_MODIFY_QP_RESP_RAMROD_DATA_MIN_RNR_NAK_TIMER_SHIFT: c_int = 3;
    pub max_ird: u8,
    pub traffic_class: u8,
    pub hop_limit: u8,
    pub p_key: __le16,
    pub flow_label: __le32,
    pub mtu: __le16,
    pub low_latency_phy_queue: __le16,
    pub regular_latency_phy_queue: __le16,
    pub reserved2: [u8; 6],
    pub src_gid: [__le32; 4],
    pub dst_gid: [__le32; 4],
}

// RoCE query qp requester output params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_query_qp_req_output_params {
    pub psn: __le32,
    pub flags: __le32,
pub const ROCE_QUERY_QP_REQ_OUTPUT_PARAMS_ERR_FLG_MASK: c_uint = 0x1;
pub const ROCE_QUERY_QP_REQ_OUTPUT_PARAMS_ERR_FLG_SHIFT: c_int = 0;
pub const ROCE_QUERY_QP_REQ_OUTPUT_PARAMS_SQ_DRAINING_FLG_MASK: c_uint = 0x1;
pub const ROCE_QUERY_QP_REQ_OUTPUT_PARAMS_SQ_DRAINING_FLG_SHIFT: c_int = 1;
pub const ROCE_QUERY_QP_REQ_OUTPUT_PARAMS_RESERVED0_MASK: c_uint = 0x3FFFFFFF;
pub const ROCE_QUERY_QP_REQ_OUTPUT_PARAMS_RESERVED0_SHIFT: c_int = 2;
}

// RoCE query qp requester ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_query_qp_req_ramrod_data {
    pub output_params_addr: regpair,
}

// RoCE query qp responder output params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_query_qp_resp_output_params {
    pub psn: __le32,
    pub flags: __le32,
pub const ROCE_QUERY_QP_RESP_OUTPUT_PARAMS_ERROR_FLG_MASK: c_uint = 0x1;
pub const ROCE_QUERY_QP_RESP_OUTPUT_PARAMS_ERROR_FLG_SHIFT: c_int = 0;
pub const ROCE_QUERY_QP_RESP_OUTPUT_PARAMS_RESERVED0_MASK: c_uint = 0x7FFFFFFF;
pub const ROCE_QUERY_QP_RESP_OUTPUT_PARAMS_RESERVED0_SHIFT: c_int = 1;
}

// RoCE query qp responder ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_query_qp_resp_ramrod_data {
    pub output_params_addr: regpair,
}

// RoCE Query Suspended QP requester output params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_query_suspended_qp_req_output_params {
    pub psn: __le32,
    pub flags: __le32,
pub const ROCE_QUERY_SUSPENDED_QP_REQ_OUTPUT_PARAMS_ERR_FLG_MASK: c_uint = 0x1;
pub const ROCE_QUERY_SUSPENDED_QP_REQ_OUTPUT_PARAMS_ERR_FLG_SHIFT: c_int = 0;
pub const ROCE_QUERY_SUSPENDED_QP_REQ_OUTPUT_PARAMS_RESERVED0_MASK: c_uint = 0x7FFFFFFF;
pub const ROCE_QUERY_SUSPENDED_QP_REQ_OUTPUT_PARAMS_RESERVED0_SHIFT: c_int = 1;
    pub send_msg_psn: __le32,
    pub inflight_sends: __le32,
    pub ssn: __le32,
    pub reserved: __le32,
}

// RoCE Query Suspended QP requester ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_query_suspended_qp_req_ramrod_data {
    pub output_params_addr: regpair,
}

// RoCE Query Suspended QP responder runtime params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_query_suspended_qp_resp_runtime_params {
    pub psn: __le32,
    pub flags: __le32,
pub const ROCE_QUERY_SUSPENDED_QP_RESP_RUNTIME_PARAMS_ERR_FLG_MASK: c_uint = 0x1;
pub const ROCE_QUERY_SUSPENDED_QP_RESP_RUNTIME_PARAMS_ERR_FLG_SHIFT: c_int = 0;
pub const ROCE_QUERY_SUSPENDED_QP_RESP_RUNTIME_PARAMS_RDMA_ACTIVE_MASK: c_uint = 0x1;
pub const ROCE_QUERY_SUSPENDED_QP_RESP_RUNTIME_PARAMS_RDMA_ACTIVE_SHIFT: c_int = 1;
pub const ROCE_QUERY_SUSPENDED_QP_RESP_RUNTIME_PARAMS_RESERVED0_MASK: c_uint = 0x3FFFFFFF;
pub const ROCE_QUERY_SUSPENDED_QP_RESP_RUNTIME_PARAMS_RESERVED0_SHIFT: c_int = 2;
    pub receive_msg_psn: __le32,
    pub inflight_receives: __le32,
    pub rmsn: __le32,
    pub rdma_key: __le32,
    pub rdma_va: regpair,
    pub rdma_length: __le32,
    pub num_rdb_entries: __le32,
}

// RoCE Query Suspended QP responder output params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_query_suspended_qp_resp_output_params {
    pub runtime_params: roce_query_suspended_qp_resp_runtime_params,
}

// RoCE Query Suspended QP responder ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_query_suspended_qp_resp_ramrod_data {
    pub output_params_addr: regpair,
}

// ROCE ramrod command IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum roce_ramrod_cmd_id {
    ROCE_RAMROD_CREATE_QP = 13,
    ROCE_RAMROD_MODIFY_QP,
    ROCE_RAMROD_QUERY_QP,
    ROCE_RAMROD_DESTROY_QP,
    ROCE_RAMROD_CREATE_UD_QP,
    ROCE_RAMROD_DESTROY_UD_QP,
    ROCE_RAMROD_FUNC_UPDATE,
    ROCE_RAMROD_SUSPEND_QP,
    ROCE_RAMROD_QUERY_SUSPENDED_QP,
    ROCE_RAMROD_CREATE_SUSPENDED_QP,
    ROCE_RAMROD_RESUME_QP,
    ROCE_RAMROD_SUSPEND_UD_QP,
    ROCE_RAMROD_RESUME_UD_QP,
    ROCE_RAMROD_CREATE_SUSPENDED_UD_QP,
    ROCE_RAMROD_FLUSH_DPT_QP,
    MAX_ROCE_RAMROD_CMD_ID
}

// ROCE RDB array entry type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum roce_resp_qp_rdb_entry_type {
    ROCE_QP_RDB_ENTRY_RDMA_RESPONSE = 0,
    ROCE_QP_RDB_ENTRY_ATOMIC_RESPONSE = 1,
    ROCE_QP_RDB_ENTRY_INVALID = 2,
    MAX_ROCE_RESP_QP_RDB_ENTRY_TYPE
}

// RoCE func init ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_update_func_params {
    pub cnp_vlan_priority: u8,
    pub cnp_dscp: u8,
    pub flags: __le16,
pub const ROCE_UPDATE_FUNC_PARAMS_DCQCN_NP_EN_MASK: c_uint = 0x1;
pub const ROCE_UPDATE_FUNC_PARAMS_DCQCN_NP_EN_SHIFT: c_int = 0;
pub const ROCE_UPDATE_FUNC_PARAMS_DCQCN_RP_EN_MASK: c_uint = 0x1;
pub const ROCE_UPDATE_FUNC_PARAMS_DCQCN_RP_EN_SHIFT: c_int = 1;
pub const ROCE_UPDATE_FUNC_PARAMS_RESERVED0_MASK: c_uint = 0x3FFF;
pub const ROCE_UPDATE_FUNC_PARAMS_RESERVED0_SHIFT: c_int = 2;
    pub cnp_send_timeout: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_roce_conn_ag_ctx_dq_ext_ld_part {
    pub reserved0: u8,
    pub state: u8,
    pub flags0: u8,
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT1_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT1_SHIFT: c_int = 1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT2_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT2_SHIFT: c_int = 2;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_EXIST_IN_QM3_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_EXIST_IN_QM3_SHIFT: c_int = 3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT4_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT4_SHIFT: c_int = 4;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT5_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT5_SHIFT: c_int = 5;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT6_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT6_SHIFT: c_int = 6;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT7_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT7_SHIFT: c_int = 7;
    pub flags1: u8,
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT8_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT8_SHIFT: c_int = 0;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT9_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT9_SHIFT: c_int = 1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT10_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT10_SHIFT: c_int = 2;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT11_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT11_SHIFT: c_int = 3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_MSDM_FLUSH_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_MSDM_FLUSH_SHIFT: c_int = 4;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_MSEM_FLUSH_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_MSEM_FLUSH_SHIFT: c_int = 5;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT14_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT14_SHIFT: c_int = 6;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_YSTORM_FLUSH_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_YSTORM_FLUSH_SHIFT: c_int = 7;
    pub flags2: u8,
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF0_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF0_SHIFT: c_int = 0;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF1_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF1_SHIFT: c_int = 2;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF2_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF2_SHIFT: c_int = 4;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF3_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF3_SHIFT: c_int = 6;
    pub flags3: u8,
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF4_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF4_SHIFT: c_int = 0;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF5_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF5_SHIFT: c_int = 2;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF6_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF6_SHIFT: c_int = 4;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_FLUSH_Q0_CF_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_FLUSH_Q0_CF_SHIFT: c_int = 6;
    pub flags4: u8,
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF8_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF8_SHIFT: c_int = 0;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF9_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF9_SHIFT: c_int = 2;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF10_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF10_SHIFT: c_int = 4;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF11_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF11_SHIFT: c_int = 6;
    pub flags5: u8,
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF12_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF12_SHIFT: c_int = 0;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF13_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF13_SHIFT: c_int = 2;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF14_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF14_SHIFT: c_int = 4;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF15_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF15_SHIFT: c_int = 6;
    pub flags6: u8,
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF16_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF16_SHIFT: c_int = 0;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF17_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF17_SHIFT: c_int = 2;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF18_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF18_SHIFT: c_int = 4;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF19_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF19_SHIFT: c_int = 6;
    pub flags7: u8,
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF20_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF20_SHIFT: c_int = 0;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF21_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF21_SHIFT: c_int = 2;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_SLOW_PATH_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_SLOW_PATH_SHIFT: c_int = 4;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF0EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF0EN_SHIFT: c_int = 6;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF1EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF1EN_SHIFT: c_int = 7;
    pub flags8: u8,
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF2EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF2EN_SHIFT: c_int = 0;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF3EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF3EN_SHIFT: c_int = 1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF4EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF4EN_SHIFT: c_int = 2;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF5EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF5EN_SHIFT: c_int = 3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF6EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF6EN_SHIFT: c_int = 4;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_FLUSH_Q0_CF_EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_FLUSH_Q0_CF_EN_SHIFT: c_int = 5;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF8EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF8EN_SHIFT: c_int = 6;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF9EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF9EN_SHIFT: c_int = 7;
    pub flags9: u8,
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF10EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF10EN_SHIFT: c_int = 0;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF11EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF11EN_SHIFT: c_int = 1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF12EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF12EN_SHIFT: c_int = 2;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF13EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF13EN_SHIFT: c_int = 3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF14EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF14EN_SHIFT: c_int = 4;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF15EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF15EN_SHIFT: c_int = 5;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF16EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF16EN_SHIFT: c_int = 6;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF17EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF17EN_SHIFT: c_int = 7;
    pub flags10: u8,
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF18EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF18EN_SHIFT: c_int = 0;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF19EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF19EN_SHIFT: c_int = 1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF20EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF20EN_SHIFT: c_int = 2;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF21EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF21EN_SHIFT: c_int = 3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_SLOW_PATH_EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_SLOW_PATH_EN_SHIFT: c_int = 4;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF23EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF23EN_SHIFT: c_int = 5;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE0EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE0EN_SHIFT: c_int = 6;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE1EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE1EN_SHIFT: c_int = 7;
    pub flags11: u8,
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE2EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE2EN_SHIFT: c_int = 0;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE3EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE3EN_SHIFT: c_int = 1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE4EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE4EN_SHIFT: c_int = 2;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE5EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE5EN_SHIFT: c_int = 3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE6EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE6EN_SHIFT: c_int = 4;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE7EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE7EN_SHIFT: c_int = 5;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED1_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED1_SHIFT: c_int = 6;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE9EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE9EN_SHIFT: c_int = 7;
    pub flags12: u8,
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE10EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE10EN_SHIFT: c_int = 0;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE11EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE11EN_SHIFT: c_int = 1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED2_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED2_SHIFT: c_int = 2;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED3_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED3_SHIFT: c_int = 3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE14EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE14EN_SHIFT: c_int = 4;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE15EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE15EN_SHIFT: c_int = 5;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE16EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE16EN_SHIFT: c_int = 6;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE17EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE17EN_SHIFT: c_int = 7;
    pub flags13: u8,
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE18EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE18EN_SHIFT: c_int = 0;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE19EN_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RULE19EN_SHIFT: c_int = 1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED4_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED4_SHIFT: c_int = 2;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED5_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED5_SHIFT: c_int = 3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED6_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED6_SHIFT: c_int = 4;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED7_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED7_SHIFT: c_int = 5;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED8_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED8_SHIFT: c_int = 6;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED9_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_A0_RESERVED9_SHIFT: c_int = 7;
    pub flags14: u8,
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_MIGRATION_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_MIGRATION_SHIFT: c_int = 0;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT17_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_BIT17_SHIFT: c_int = 1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_DPM_PORT_NUM_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_DPM_PORT_NUM_SHIFT: c_int = 2;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RESERVED_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_RESERVED_SHIFT: c_int = 4;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_ROCE_EDPM_ENABLE_MASK: c_uint = 0x1;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_ROCE_EDPM_ENABLE_SHIFT: c_int = 5;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF23_MASK: c_uint = 0x3;
pub const E4XSTORMROCECONNAGCTXDQEXTLDPART_CF23_SHIFT: c_int = 6;
    pub byte2: u8,
    pub physical_q0: __le16,
    pub word1: __le16,
    pub word2: __le16,
    pub word3: __le16,
    pub word4: __le16,
    pub word5: __le16,
    pub conn_dpi: __le16,
    pub byte3: u8,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub snd_nxt_psn: __le32,
    pub reg4: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_roce_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const MSTORM_ROCE_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const MSTORM_ROCE_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const MSTORM_ROCE_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const MSTORM_ROCE_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const MSTORM_ROCE_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const MSTORM_ROCE_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const MSTORM_ROCE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const MSTORM_ROCE_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const MSTORM_ROCE_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const MSTORM_ROCE_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const MSTORM_ROCE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const MSTORM_ROCE_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const MSTORM_ROCE_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const MSTORM_ROCE_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const MSTORM_ROCE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const MSTORM_ROCE_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub word0: __le16,
    pub word1: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_roce_req_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_REQ_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub word0: __le16,
    pub word1: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_roce_resp_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const MSTORM_ROCE_RESP_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub word0: __le16,
    pub word1: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_roce_req_conn_ag_ctx {
    pub reserved0: u8,
    pub state: u8,
    pub flags0: u8,
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RX_ERROR_OCCURRED_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RX_ERROR_OCCURRED_SHIFT: c_int = 1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_TX_CQE_ERROR_OCCURRED_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_TX_CQE_ERROR_OCCURRED_SHIFT: c_int = 2;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_BIT3_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_BIT3_SHIFT: c_int = 3;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_MSTORM_FLUSH_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_MSTORM_FLUSH_SHIFT: c_int = 4;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_CACHED_ORQ_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_CACHED_ORQ_SHIFT: c_int = 5;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_TIMER_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_TIMER_CF_SHIFT: c_int = 6;
    pub flags1: u8,
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_MSTORM_FLUSH_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_MSTORM_FLUSH_CF_SHIFT: c_int = 0;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_FLUSH_SQ_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_FLUSH_SQ_CF_SHIFT: c_int = 2;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_TIMER_STOP_ALL_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_TIMER_STOP_ALL_CF_SHIFT: c_int = 4;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_FLUSH_Q0_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_FLUSH_Q0_CF_SHIFT: c_int = 6;
    pub flags2: u8,
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_FORCE_COMP_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_FORCE_COMP_CF_SHIFT: c_int = 0;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_SET_TIMER_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_SET_TIMER_CF_SHIFT: c_int = 2;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_TX_ASYNC_ERROR_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_TX_ASYNC_ERROR_CF_SHIFT: c_int = 4;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RXMIT_DONE_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RXMIT_DONE_CF_SHIFT: c_int = 6;
    pub flags3: u8,
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_ERROR_SCAN_COMPLETED_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_ERROR_SCAN_COMPLETED_CF_SHIFT: c_int = 0;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_SQ_DRAIN_COMPLETED_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_SQ_DRAIN_COMPLETED_CF_SHIFT: c_int = 2;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_TIMER_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_TIMER_CF_EN_SHIFT: c_int = 4;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_MSTORM_FLUSH_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_MSTORM_FLUSH_CF_EN_SHIFT: c_int = 5;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_FLUSH_SQ_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_FLUSH_SQ_CF_EN_SHIFT: c_int = 6;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_TIMER_STOP_ALL_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_TIMER_STOP_ALL_CF_EN_SHIFT: c_int = 7;
    pub flags4: u8,
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_FLUSH_Q0_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_FLUSH_Q0_CF_EN_SHIFT: c_int = 0;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_FORCE_COMP_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_FORCE_COMP_CF_EN_SHIFT: c_int = 1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_SET_TIMER_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_SET_TIMER_CF_EN_SHIFT: c_int = 2;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_TX_ASYNC_ERROR_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_TX_ASYNC_ERROR_CF_EN_SHIFT: c_int = 3;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RXMIT_DONE_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RXMIT_DONE_CF_EN_SHIFT: c_int = 4;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_ERROR_SCAN_COMPLETED_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_ERROR_SCAN_COMPLETED_CF_EN_SHIFT: c_int = 5;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_SQ_DRAIN_COMPLETED_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_SQ_DRAIN_COMPLETED_CF_EN_SHIFT: c_int = 6;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 7;
    pub flags5: u8,
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 0;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_DIF_CNT_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_DIF_CNT_EN_SHIFT: c_int = 1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_SND_SQ_CONS_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_SND_SQ_CONS_EN_SHIFT: c_int = 5;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_REQ_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub dif_rxmit_cnt: __le32,
    pub snd_nxt_psn: __le32,
    pub snd_max_psn: __le32,
    pub orq_prod: __le32,
    pub reg4: __le32,
    pub dif_acked_cnt: __le32,
    pub dif_cnt: __le32,
    pub reg7: __le32,
    pub reg8: __le32,
    pub tx_cqe_error_type: u8,
    pub orq_cache_idx: u8,
    pub snd_sq_cons_th: __le16,
    pub byte4: u8,
    pub byte5: u8,
    pub snd_sq_cons: __le16,
    pub conn_dpi: __le16,
    pub force_comp_cons: __le16,
    pub dif_rxmit_acked_cnt: __le32,
    pub reg10: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_roce_resp_conn_ag_ctx {
    pub byte0: u8,
    pub state: u8,
    pub flags0: u8,
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RX_ERROR_NOTIFY_REQUESTER_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RX_ERROR_NOTIFY_REQUESTER_SHIFT: c_int = 1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_BIT2_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_BIT2_SHIFT: c_int = 2;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_BIT3_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_BIT3_SHIFT: c_int = 3;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_MSTORM_FLUSH_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_MSTORM_FLUSH_SHIFT: c_int = 4;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_BIT5_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_BIT5_SHIFT: c_int = 5;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF0_SHIFT: c_int = 6;
    pub flags1: u8,
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_MSTORM_FLUSH_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_MSTORM_FLUSH_CF_SHIFT: c_int = 0;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_TX_ERROR_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_TX_ERROR_CF_SHIFT: c_int = 2;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF3_SHIFT: c_int = 4;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_FLUSH_Q0_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_FLUSH_Q0_CF_SHIFT: c_int = 6;
    pub flags2: u8,
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RX_ERROR_CF_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RX_ERROR_CF_SHIFT: c_int = 0;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF6_SHIFT: c_int = 2;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF7_SHIFT: c_int = 4;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF8_SHIFT: c_int = 6;
    pub flags3: u8,
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF9_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF9_SHIFT: c_int = 0;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF10_MASK: c_uint = 0x3;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF10_SHIFT: c_int = 2;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF0EN_SHIFT: c_int = 4;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_MSTORM_FLUSH_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_MSTORM_FLUSH_CF_EN_SHIFT: c_int = 5;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_TX_ERROR_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_TX_ERROR_CF_EN_SHIFT: c_int = 6;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF3EN_SHIFT: c_int = 7;
    pub flags4: u8,
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_FLUSH_Q0_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_FLUSH_Q0_CF_EN_SHIFT: c_int = 0;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RX_ERROR_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RX_ERROR_CF_EN_SHIFT: c_int = 1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF6EN_SHIFT: c_int = 2;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF7EN_SHIFT: c_int = 3;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF8EN_SHIFT: c_int = 4;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF9EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF9EN_SHIFT: c_int = 5;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF10EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_CF10EN_SHIFT: c_int = 6;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 7;
    pub flags5: u8,
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 0;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RQ_RULE_EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RQ_RULE_EN_SHIFT: c_int = 5;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const TSTORM_ROCE_RESP_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub psn_and_rxmit_id_echo: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub reg3: __le32,
    pub reg4: __le32,
    pub reg5: __le32,
    pub reg6: __le32,
    pub reg7: __le32,
    pub reg8: __le32,
    pub tx_async_error_type: u8,
    pub byte3: u8,
    pub rq_cons: __le16,
    pub byte4: u8,
    pub byte5: u8,
    pub rq_prod: __le16,
    pub conn_dpi: __le16,
    pub irq_cons: __le16,
    pub reg9: __le32,
    pub reg10: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_roce_req_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const USTORM_ROCE_REQ_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF3_SHIFT: c_int = 0;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF4_SHIFT: c_int = 2;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF5_SHIFT: c_int = 4;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF6_SHIFT: c_int = 6;
    pub flags2: u8,
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF3EN_SHIFT: c_int = 3;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF4EN_SHIFT: c_int = 4;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF5EN_SHIFT: c_int = 5;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_CF6EN_SHIFT: c_int = 6;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 7;
    pub flags3: u8,
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 0;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 5;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_REQ_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub word1: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub reg3: __le32,
    pub word2: __le16,
    pub word3: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_roce_resp_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const USTORM_ROCE_RESP_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF3_SHIFT: c_int = 0;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF4_SHIFT: c_int = 2;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF5_SHIFT: c_int = 4;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF6_SHIFT: c_int = 6;
    pub flags2: u8,
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF3EN_SHIFT: c_int = 3;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF4EN_SHIFT: c_int = 4;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF5EN_SHIFT: c_int = 5;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_CF6EN_SHIFT: c_int = 6;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 7;
    pub flags3: u8,
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 0;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 5;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const USTORM_ROCE_RESP_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub word1: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub reg3: __le32,
    pub word2: __le16,
    pub word3: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_roce_req_conn_ag_ctx {
    pub reserved0: u8,
    pub state: u8,
    pub flags0: u8,
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED1_SHIFT: c_int = 1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED2_SHIFT: c_int = 2;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_EXIST_IN_QM3_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_EXIST_IN_QM3_SHIFT: c_int = 3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED3_SHIFT: c_int = 4;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED4_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED4_SHIFT: c_int = 5;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED5_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED5_SHIFT: c_int = 6;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED6_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED6_SHIFT: c_int = 7;
    pub flags1: u8,
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED7_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED7_SHIFT: c_int = 0;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED8_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED8_SHIFT: c_int = 1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_BIT10_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_BIT10_SHIFT: c_int = 2;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_BIT11_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_BIT11_SHIFT: c_int = 3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_MSDM_FLUSH_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_MSDM_FLUSH_SHIFT: c_int = 4;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_MSEM_FLUSH_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_MSEM_FLUSH_SHIFT: c_int = 5;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_ERROR_STATE_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_ERROR_STATE_SHIFT: c_int = 6;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_YSTORM_FLUSH_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_YSTORM_FLUSH_SHIFT: c_int = 7;
    pub flags2: u8,
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF0_SHIFT: c_int = 0;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF1_SHIFT: c_int = 2;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF2_SHIFT: c_int = 4;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF3_SHIFT: c_int = 6;
    pub flags3: u8,
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SQ_FLUSH_CF_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SQ_FLUSH_CF_SHIFT: c_int = 0;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RX_ERROR_CF_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RX_ERROR_CF_SHIFT: c_int = 2;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SND_RXMIT_CF_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SND_RXMIT_CF_SHIFT: c_int = 4;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_FLUSH_Q0_CF_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_FLUSH_Q0_CF_SHIFT: c_int = 6;
    pub flags4: u8,
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_DIF_ERROR_CF_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_DIF_ERROR_CF_SHIFT: c_int = 0;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SCAN_SQ_FOR_COMP_CF_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SCAN_SQ_FOR_COMP_CF_SHIFT: c_int = 2;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF10_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF10_SHIFT: c_int = 4;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF11_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF11_SHIFT: c_int = 6;
    pub flags5: u8,
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF12_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF12_SHIFT: c_int = 0;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF13_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF13_SHIFT: c_int = 2;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_FMR_ENDED_CF_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_FMR_ENDED_CF_SHIFT: c_int = 4;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF15_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF15_SHIFT: c_int = 6;
    pub flags6: u8,
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF16_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF16_SHIFT: c_int = 0;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF17_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF17_SHIFT: c_int = 2;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF18_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF18_SHIFT: c_int = 4;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF19_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF19_SHIFT: c_int = 6;
    pub flags7: u8,
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF20_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF20_SHIFT: c_int = 0;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF21_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF21_SHIFT: c_int = 2;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SLOW_PATH_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SLOW_PATH_SHIFT: c_int = 4;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF0EN_SHIFT: c_int = 6;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags8: u8,
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF2EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF3EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SQ_FLUSH_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SQ_FLUSH_CF_EN_SHIFT: c_int = 2;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RX_ERROR_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RX_ERROR_CF_EN_SHIFT: c_int = 3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SND_RXMIT_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SND_RXMIT_CF_EN_SHIFT: c_int = 4;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_FLUSH_Q0_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_FLUSH_Q0_CF_EN_SHIFT: c_int = 5;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_DIF_ERROR_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_DIF_ERROR_CF_EN_SHIFT: c_int = 6;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SCAN_SQ_FOR_COMP_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SCAN_SQ_FOR_COMP_CF_EN_SHIFT: c_int = 7;
    pub flags9: u8,
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF10EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF10EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF11EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF11EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF12EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF12EN_SHIFT: c_int = 2;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF13EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF13EN_SHIFT: c_int = 3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_FME_ENDED_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_FME_ENDED_CF_EN_SHIFT: c_int = 4;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF15EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF15EN_SHIFT: c_int = 5;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF16EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF16EN_SHIFT: c_int = 6;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF17EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF17EN_SHIFT: c_int = 7;
    pub flags10: u8,
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF18EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF18EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF19EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF19EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF20EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF20EN_SHIFT: c_int = 2;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF21EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF21EN_SHIFT: c_int = 3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SLOW_PATH_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SLOW_PATH_EN_SHIFT: c_int = 4;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF23EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF23EN_SHIFT: c_int = 5;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 6;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 7;
    pub flags11: u8,
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 2;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 4;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_E2E_CREDIT_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_E2E_CREDIT_RULE_EN_SHIFT: c_int = 5;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED1_SHIFT: c_int = 6;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE9EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE9EN_SHIFT: c_int = 7;
    pub flags12: u8,
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SQ_PROD_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_SQ_PROD_EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE11EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE11EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED2_SHIFT: c_int = 2;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED3_SHIFT: c_int = 3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_INV_FENCE_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_INV_FENCE_RULE_EN_SHIFT: c_int = 4;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE15EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE15EN_SHIFT: c_int = 5;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_ORQ_FENCE_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_ORQ_FENCE_RULE_EN_SHIFT: c_int = 6;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_MAX_ORD_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_MAX_ORD_RULE_EN_SHIFT: c_int = 7;
    pub flags13: u8,
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE18EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE18EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE19EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RULE19EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED4_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED4_SHIFT: c_int = 2;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED5_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED5_SHIFT: c_int = 3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED6_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED6_SHIFT: c_int = 4;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED7_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED7_SHIFT: c_int = 5;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED8_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED8_SHIFT: c_int = 6;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED9_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_A0_RESERVED9_SHIFT: c_int = 7;
    pub flags14: u8,
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_MIGRATION_FLAG_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_MIGRATION_FLAG_SHIFT: c_int = 0;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_BIT17_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_BIT17_SHIFT: c_int = 1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_DPM_PORT_NUM_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_DPM_PORT_NUM_SHIFT: c_int = 2;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_RESERVED_SHIFT: c_int = 4;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_ROCE_EDPM_ENABLE_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_ROCE_EDPM_ENABLE_SHIFT: c_int = 5;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF23_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_REQ_CONN_AG_CTX_CF23_SHIFT: c_int = 6;
    pub byte2: u8,
    pub physical_q0: __le16,
    pub word1: __le16,
    pub sq_cmp_cons: __le16,
    pub sq_cons: __le16,
    pub sq_prod: __le16,
    pub dif_error_first_sq_cons: __le16,
    pub conn_dpi: __le16,
    pub dif_error_sge_index: u8,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub lsn: __le32,
    pub ssn: __le32,
    pub snd_una_psn: __le32,
    pub snd_nxt_psn: __le32,
    pub dif_error_offset: __le32,
    pub orq_cons_th: __le32,
    pub orq_cons: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_roce_resp_conn_ag_ctx {
    pub reserved0: u8,
    pub state: u8,
    pub flags0: u8,
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RESERVED1_SHIFT: c_int = 1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RESERVED2_SHIFT: c_int = 2;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_EXIST_IN_QM3_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_EXIST_IN_QM3_SHIFT: c_int = 3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RESERVED3_SHIFT: c_int = 4;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RESERVED4_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RESERVED4_SHIFT: c_int = 5;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RESERVED5_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RESERVED5_SHIFT: c_int = 6;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RESERVED6_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RESERVED6_SHIFT: c_int = 7;
    pub flags1: u8,
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RESERVED7_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RESERVED7_SHIFT: c_int = 0;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RESERVED8_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RESERVED8_SHIFT: c_int = 1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_BIT10_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_BIT10_SHIFT: c_int = 2;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_BIT11_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_BIT11_SHIFT: c_int = 3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_MSDM_FLUSH_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_MSDM_FLUSH_SHIFT: c_int = 4;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_MSEM_FLUSH_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_MSEM_FLUSH_SHIFT: c_int = 5;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_ERROR_STATE_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_ERROR_STATE_SHIFT: c_int = 6;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_YSTORM_FLUSH_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_YSTORM_FLUSH_SHIFT: c_int = 7;
    pub flags2: u8,
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF0_SHIFT: c_int = 0;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF1_SHIFT: c_int = 2;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF2_SHIFT: c_int = 4;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF3_SHIFT: c_int = 6;
    pub flags3: u8,
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RXMIT_CF_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RXMIT_CF_SHIFT: c_int = 0;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RX_ERROR_CF_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RX_ERROR_CF_SHIFT: c_int = 2;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_FORCE_ACK_CF_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_FORCE_ACK_CF_SHIFT: c_int = 4;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_FLUSH_Q0_CF_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_FLUSH_Q0_CF_SHIFT: c_int = 6;
    pub flags4: u8,
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF8_SHIFT: c_int = 0;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF9_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF9_SHIFT: c_int = 2;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF10_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF10_SHIFT: c_int = 4;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF11_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF11_SHIFT: c_int = 6;
    pub flags5: u8,
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF12_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF12_SHIFT: c_int = 0;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF13_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF13_SHIFT: c_int = 2;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF14_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF14_SHIFT: c_int = 4;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF15_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF15_SHIFT: c_int = 6;
    pub flags6: u8,
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF16_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF16_SHIFT: c_int = 0;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF17_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF17_SHIFT: c_int = 2;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF18_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF18_SHIFT: c_int = 4;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF19_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF19_SHIFT: c_int = 6;
    pub flags7: u8,
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF20_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF20_SHIFT: c_int = 0;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF21_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF21_SHIFT: c_int = 2;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_SLOW_PATH_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_SLOW_PATH_SHIFT: c_int = 4;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF0EN_SHIFT: c_int = 6;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags8: u8,
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF2EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF3EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RXMIT_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RXMIT_CF_EN_SHIFT: c_int = 2;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RX_ERROR_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RX_ERROR_CF_EN_SHIFT: c_int = 3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_FORCE_ACK_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_FORCE_ACK_CF_EN_SHIFT: c_int = 4;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_FLUSH_Q0_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_FLUSH_Q0_CF_EN_SHIFT: c_int = 5;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF8EN_SHIFT: c_int = 6;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF9EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF9EN_SHIFT: c_int = 7;
    pub flags9: u8,
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF10EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF10EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF11EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF11EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF12EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF12EN_SHIFT: c_int = 2;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF13EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF13EN_SHIFT: c_int = 3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF14EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF14EN_SHIFT: c_int = 4;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF15EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF15EN_SHIFT: c_int = 5;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF16EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF16EN_SHIFT: c_int = 6;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF17EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF17EN_SHIFT: c_int = 7;
    pub flags10: u8,
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF18EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF18EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF19EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF19EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF20EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF20EN_SHIFT: c_int = 2;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF21EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF21EN_SHIFT: c_int = 3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_SLOW_PATH_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_SLOW_PATH_EN_SHIFT: c_int = 4;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF23EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF23EN_SHIFT: c_int = 5;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 6;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 7;
    pub flags11: u8,
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 2;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 4;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 5;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED1_SHIFT: c_int = 6;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE9EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE9EN_SHIFT: c_int = 7;
    pub flags12: u8,
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_IRQ_PROD_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_IRQ_PROD_RULE_EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE11EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE11EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED2_SHIFT: c_int = 2;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED3_SHIFT: c_int = 3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE14EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE14EN_SHIFT: c_int = 4;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE15EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE15EN_SHIFT: c_int = 5;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE16EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE16EN_SHIFT: c_int = 6;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE17EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE17EN_SHIFT: c_int = 7;
    pub flags13: u8,
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE18EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE18EN_SHIFT: c_int = 0;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE19EN_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_RULE19EN_SHIFT: c_int = 1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED4_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED4_SHIFT: c_int = 2;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED5_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED5_SHIFT: c_int = 3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED6_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED6_SHIFT: c_int = 4;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED7_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED7_SHIFT: c_int = 5;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED8_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED8_SHIFT: c_int = 6;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED9_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_A0_RESERVED9_SHIFT: c_int = 7;
    pub flags14: u8,
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_BIT16_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_BIT16_SHIFT: c_int = 0;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_BIT17_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_BIT17_SHIFT: c_int = 1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_BIT18_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_BIT18_SHIFT: c_int = 2;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_BIT19_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_BIT19_SHIFT: c_int = 3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_BIT20_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_BIT20_SHIFT: c_int = 4;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_BIT21_MASK: c_uint = 0x1;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_BIT21_SHIFT: c_int = 5;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF23_MASK: c_uint = 0x3;
pub const XSTORM_ROCE_RESP_CONN_AG_CTX_CF23_SHIFT: c_int = 6;
    pub byte2: u8,
    pub physical_q0: __le16,
    pub irq_prod_shadow: __le16,
    pub word2: __le16,
    pub irq_cons: __le16,
    pub irq_prod: __le16,
    pub e5_reserved1: __le16,
    pub conn_dpi: __le16,
    pub rxmit_opcode: u8,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub rxmit_psn_and_id: __le32,
    pub rxmit_bytes_length: __le32,
    pub psn: __le32,
    pub reg3: __le32,
    pub reg4: __le32,
    pub reg5: __le32,
    pub msn_and_syndrome: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_roce_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const YSTORM_ROCE_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const YSTORM_ROCE_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const YSTORM_ROCE_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const YSTORM_ROCE_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const YSTORM_ROCE_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const YSTORM_ROCE_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const YSTORM_ROCE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const YSTORM_ROCE_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const YSTORM_ROCE_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const YSTORM_ROCE_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const YSTORM_ROCE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const YSTORM_ROCE_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const YSTORM_ROCE_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const YSTORM_ROCE_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const YSTORM_ROCE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const YSTORM_ROCE_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
    pub word1: __le16,
    pub word2: __le16,
    pub word3: __le16,
    pub word4: __le16,
    pub reg2: __le32,
    pub reg3: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_roce_req_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_REQ_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
    pub word1: __le16,
    pub word2: __le16,
    pub word3: __le16,
    pub word4: __le16,
    pub reg2: __le32,
    pub reg3: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_roce_resp_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const YSTORM_ROCE_RESP_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
    pub word1: __le16,
    pub word2: __le16,
    pub word3: __le16,
    pub word4: __le16,
    pub reg2: __le32,
    pub reg3: __le32,
}

// Roce doorbell data
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum roce_flavor {
    PLAIN_ROCE,
    RROCE_IPV4,
    RROCE_IPV6,
    MAX_ROCE_FLAVOR
}

// The iwarp storm context of Ystorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_iwarp_conn_st_ctx {
    pub reserved: [__le32; 4],
}

// The iwarp storm context of Pstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstorm_iwarp_conn_st_ctx {
    pub reserved: [__le32; 36],
}

// The iwarp storm context of Xstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_iwarp_conn_st_ctx {
    pub reserved: [__le32; 48],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_iwarp_conn_ag_ctx {
    pub reserved0: u8,
    pub state: u8,
    pub flags0: u8,
pub const XSTORM_IWARP_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const XSTORM_IWARP_CONN_AG_CTX_EXIST_IN_QM1_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_EXIST_IN_QM1_SHIFT: c_int = 1;
pub const XSTORM_IWARP_CONN_AG_CTX_EXIST_IN_QM2_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_EXIST_IN_QM2_SHIFT: c_int = 2;
pub const XSTORM_IWARP_CONN_AG_CTX_EXIST_IN_QM3_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_EXIST_IN_QM3_SHIFT: c_int = 3;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT4_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT4_SHIFT: c_int = 4;
pub const XSTORM_IWARP_CONN_AG_CTX_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_RESERVED2_SHIFT: c_int = 5;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT6_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT6_SHIFT: c_int = 6;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT7_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT7_SHIFT: c_int = 7;
    pub flags1: u8,
pub const XSTORM_IWARP_CONN_AG_CTX_BIT8_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT8_SHIFT: c_int = 0;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT9_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT9_SHIFT: c_int = 1;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT10_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT10_SHIFT: c_int = 2;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT11_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT11_SHIFT: c_int = 3;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT12_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT12_SHIFT: c_int = 4;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT13_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT13_SHIFT: c_int = 5;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT14_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT14_SHIFT: c_int = 6;
pub const XSTORM_IWARP_CONN_AG_CTX_YSTORM_FLUSH_OR_REWIND_SND_MAX_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_YSTORM_FLUSH_OR_REWIND_SND_MAX_SHIFT: c_int = 7;
    pub flags2: u8,
pub const XSTORM_IWARP_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF0_SHIFT: c_int = 0;
pub const XSTORM_IWARP_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF1_SHIFT: c_int = 2;
pub const XSTORM_IWARP_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF2_SHIFT: c_int = 4;
pub const XSTORM_IWARP_CONN_AG_CTX_TIMER_STOP_ALL_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_TIMER_STOP_ALL_SHIFT: c_int = 6;
    pub flags3: u8,
pub const XSTORM_IWARP_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF4_SHIFT: c_int = 0;
pub const XSTORM_IWARP_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF5_SHIFT: c_int = 2;
pub const XSTORM_IWARP_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF6_SHIFT: c_int = 4;
pub const XSTORM_IWARP_CONN_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF7_SHIFT: c_int = 6;
    pub flags4: u8,
pub const XSTORM_IWARP_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF8_SHIFT: c_int = 0;
pub const XSTORM_IWARP_CONN_AG_CTX_CF9_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF9_SHIFT: c_int = 2;
pub const XSTORM_IWARP_CONN_AG_CTX_CF10_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF10_SHIFT: c_int = 4;
pub const XSTORM_IWARP_CONN_AG_CTX_CF11_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF11_SHIFT: c_int = 6;
    pub flags5: u8,
pub const XSTORM_IWARP_CONN_AG_CTX_CF12_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF12_SHIFT: c_int = 0;
pub const XSTORM_IWARP_CONN_AG_CTX_CF13_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF13_SHIFT: c_int = 2;
pub const XSTORM_IWARP_CONN_AG_CTX_SQ_FLUSH_CF_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_SQ_FLUSH_CF_SHIFT: c_int = 4;
pub const XSTORM_IWARP_CONN_AG_CTX_CF15_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF15_SHIFT: c_int = 6;
    pub flags6: u8,
pub const XSTORM_IWARP_CONN_AG_CTX_MPA_OR_ERROR_WAKEUP_TRIGGER_CF_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_MPA_OR_ERROR_WAKEUP_TRIGGER_CF_SHIFT: c_int = 0;
pub const XSTORM_IWARP_CONN_AG_CTX_CF17_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF17_SHIFT: c_int = 2;
pub const XSTORM_IWARP_CONN_AG_CTX_CF18_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF18_SHIFT: c_int = 4;
pub const XSTORM_IWARP_CONN_AG_CTX_DQ_FLUSH_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_DQ_FLUSH_SHIFT: c_int = 6;
    pub flags7: u8,
pub const XSTORM_IWARP_CONN_AG_CTX_FLUSH_Q0_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_FLUSH_Q0_SHIFT: c_int = 0;
pub const XSTORM_IWARP_CONN_AG_CTX_FLUSH_Q1_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_FLUSH_Q1_SHIFT: c_int = 2;
pub const XSTORM_IWARP_CONN_AG_CTX_SLOW_PATH_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_SLOW_PATH_SHIFT: c_int = 4;
pub const XSTORM_IWARP_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF0EN_SHIFT: c_int = 6;
pub const XSTORM_IWARP_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags8: u8,
pub const XSTORM_IWARP_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF2EN_SHIFT: c_int = 0;
pub const XSTORM_IWARP_CONN_AG_CTX_TIMER_STOP_ALL_EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_TIMER_STOP_ALL_EN_SHIFT: c_int = 1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF4EN_SHIFT: c_int = 2;
pub const XSTORM_IWARP_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF5EN_SHIFT: c_int = 3;
pub const XSTORM_IWARP_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF6EN_SHIFT: c_int = 4;
pub const XSTORM_IWARP_CONN_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF7EN_SHIFT: c_int = 5;
pub const XSTORM_IWARP_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF8EN_SHIFT: c_int = 6;
pub const XSTORM_IWARP_CONN_AG_CTX_CF9EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF9EN_SHIFT: c_int = 7;
    pub flags9: u8,
pub const XSTORM_IWARP_CONN_AG_CTX_CF10EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF10EN_SHIFT: c_int = 0;
pub const XSTORM_IWARP_CONN_AG_CTX_CF11EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF11EN_SHIFT: c_int = 1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF12EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF12EN_SHIFT: c_int = 2;
pub const XSTORM_IWARP_CONN_AG_CTX_CF13EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF13EN_SHIFT: c_int = 3;
pub const XSTORM_IWARP_CONN_AG_CTX_SQ_FLUSH_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_SQ_FLUSH_CF_EN_SHIFT: c_int = 4;
pub const XSTORM_IWARP_CONN_AG_CTX_CF15EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF15EN_SHIFT: c_int = 5;
pub const XSTORM_IWARP_CONN_AG_CTX_MPA_OR_ERROR_WAKEUP_TRIGGER_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_MPA_OR_ERROR_WAKEUP_TRIGGER_CF_EN_SHIFT: c_int = 6;
pub const XSTORM_IWARP_CONN_AG_CTX_CF17EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF17EN_SHIFT: c_int = 7;
    pub flags10: u8,
pub const XSTORM_IWARP_CONN_AG_CTX_CF18EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_CF18EN_SHIFT: c_int = 0;
pub const XSTORM_IWARP_CONN_AG_CTX_DQ_FLUSH_EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_DQ_FLUSH_EN_SHIFT: c_int = 1;
pub const XSTORM_IWARP_CONN_AG_CTX_FLUSH_Q0_EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_FLUSH_Q0_EN_SHIFT: c_int = 2;
pub const XSTORM_IWARP_CONN_AG_CTX_FLUSH_Q1_EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_FLUSH_Q1_EN_SHIFT: c_int = 3;
pub const XSTORM_IWARP_CONN_AG_CTX_SLOW_PATH_EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_SLOW_PATH_EN_SHIFT: c_int = 4;
pub const XSTORM_IWARP_CONN_AG_CTX_SEND_TERMINATE_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_SEND_TERMINATE_CF_EN_SHIFT: c_int = 5;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 6;
pub const XSTORM_IWARP_CONN_AG_CTX_MORE_TO_SEND_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_MORE_TO_SEND_RULE_EN_SHIFT: c_int = 7;
    pub flags11: u8,
pub const XSTORM_IWARP_CONN_AG_CTX_TX_BLOCKED_EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_TX_BLOCKED_EN_SHIFT: c_int = 0;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 1;
pub const XSTORM_IWARP_CONN_AG_CTX_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_RESERVED3_SHIFT: c_int = 2;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 3;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 4;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 5;
pub const XSTORM_IWARP_CONN_AG_CTX_A0_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_A0_RESERVED1_SHIFT: c_int = 6;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE9EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE9EN_SHIFT: c_int = 7;
    pub flags12: u8,
pub const XSTORM_IWARP_CONN_AG_CTX_SQ_NOT_EMPTY_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_SQ_NOT_EMPTY_RULE_EN_SHIFT: c_int = 0;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE11EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE11EN_SHIFT: c_int = 1;
pub const XSTORM_IWARP_CONN_AG_CTX_A0_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_A0_RESERVED2_SHIFT: c_int = 2;
pub const XSTORM_IWARP_CONN_AG_CTX_A0_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_A0_RESERVED3_SHIFT: c_int = 3;
pub const XSTORM_IWARP_CONN_AG_CTX_SQ_FENCE_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_SQ_FENCE_RULE_EN_SHIFT: c_int = 4;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE15EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE15EN_SHIFT: c_int = 5;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE16EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE16EN_SHIFT: c_int = 6;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE17EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE17EN_SHIFT: c_int = 7;
    pub flags13: u8,
pub const XSTORM_IWARP_CONN_AG_CTX_IRQ_NOT_EMPTY_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_IRQ_NOT_EMPTY_RULE_EN_SHIFT: c_int = 0;
pub const XSTORM_IWARP_CONN_AG_CTX_HQ_NOT_FULL_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_HQ_NOT_FULL_RULE_EN_SHIFT: c_int = 1;
pub const XSTORM_IWARP_CONN_AG_CTX_ORQ_RD_FENCE_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_ORQ_RD_FENCE_RULE_EN_SHIFT: c_int = 2;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE21EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_RULE21EN_SHIFT: c_int = 3;
pub const XSTORM_IWARP_CONN_AG_CTX_A0_RESERVED6_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_A0_RESERVED6_SHIFT: c_int = 4;
pub const XSTORM_IWARP_CONN_AG_CTX_ORQ_NOT_FULL_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_ORQ_NOT_FULL_RULE_EN_SHIFT: c_int = 5;
pub const XSTORM_IWARP_CONN_AG_CTX_A0_RESERVED8_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_A0_RESERVED8_SHIFT: c_int = 6;
pub const XSTORM_IWARP_CONN_AG_CTX_A0_RESERVED9_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_A0_RESERVED9_SHIFT: c_int = 7;
    pub flags14: u8,
pub const XSTORM_IWARP_CONN_AG_CTX_BIT16_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT16_SHIFT: c_int = 0;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT17_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT17_SHIFT: c_int = 1;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT18_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_BIT18_SHIFT: c_int = 2;
pub const XSTORM_IWARP_CONN_AG_CTX_E5_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_E5_RESERVED1_SHIFT: c_int = 3;
pub const XSTORM_IWARP_CONN_AG_CTX_E5_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_E5_RESERVED2_SHIFT: c_int = 4;
pub const XSTORM_IWARP_CONN_AG_CTX_E5_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_IWARP_CONN_AG_CTX_E5_RESERVED3_SHIFT: c_int = 5;
pub const XSTORM_IWARP_CONN_AG_CTX_SEND_TERMINATE_CF_MASK: c_uint = 0x3;
pub const XSTORM_IWARP_CONN_AG_CTX_SEND_TERMINATE_CF_SHIFT: c_int = 6;
    pub byte2: u8,
    pub physical_q0: __le16,
    pub physical_q1: __le16,
    pub sq_comp_cons: __le16,
    pub sq_tx_cons: __le16,
    pub sq_prod: __le16,
    pub word5: __le16,
    pub conn_dpi: __le16,
    pub byte3: u8,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub more_to_send_seq: __le32,
    pub reg4: __le32,
    pub rewinded_snd_max_or_term_opcode: __le32,
    pub rd_msn: __le32,
    pub irq_prod_via_msdm: __le16,
    pub irq_cons: __le16,
    pub hq_cons_th_or_mpa_data: __le16,
    pub hq_cons: __le16,
    pub atom_msn: __le32,
    pub orq_cons: __le32,
    pub orq_cons_th: __le32,
    pub byte7: u8,
    pub wqe_data_pad_bytes: u8,
    pub max_ord: u8,
    pub former_hq_prod: u8,
    pub irq_prod_via_msem: u8,
    pub byte12: u8,
    pub max_pkt_pdu_size_lo: u8,
    pub max_pkt_pdu_size_hi: u8,
    pub byte15: u8,
    pub e5_reserved: u8,
    pub e5_reserved4: __le16,
    pub reg10: __le32,
    pub reg11: __le32,
    pub shared_queue_page_addr_lo: __le32,
    pub shared_queue_page_addr_hi: __le32,
    pub reg14: __le32,
    pub reg15: __le32,
    pub reg16: __le32,
    pub reg17: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_iwarp_conn_ag_ctx {
    pub reserved0: u8,
    pub state: u8,
    pub flags0: u8,
pub const TSTORM_IWARP_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const TSTORM_IWARP_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const TSTORM_IWARP_CONN_AG_CTX_BIT2_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_BIT2_SHIFT: c_int = 2;
pub const TSTORM_IWARP_CONN_AG_CTX_MSTORM_FLUSH_OR_TERMINATE_SENT_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_MSTORM_FLUSH_OR_TERMINATE_SENT_SHIFT: c_int = 3;
pub const TSTORM_IWARP_CONN_AG_CTX_BIT4_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_BIT4_SHIFT: c_int = 4;
pub const TSTORM_IWARP_CONN_AG_CTX_CACHED_ORQ_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_CACHED_ORQ_SHIFT: c_int = 5;
pub const TSTORM_IWARP_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const TSTORM_IWARP_CONN_AG_CTX_CF0_SHIFT: c_int = 6;
    pub flags1: u8,
pub const TSTORM_IWARP_CONN_AG_CTX_RQ_POST_CF_MASK: c_uint = 0x3;
pub const TSTORM_IWARP_CONN_AG_CTX_RQ_POST_CF_SHIFT: c_int = 0;
pub const TSTORM_IWARP_CONN_AG_CTX_MPA_TIMEOUT_CF_MASK: c_uint = 0x3;
pub const TSTORM_IWARP_CONN_AG_CTX_MPA_TIMEOUT_CF_SHIFT: c_int = 2;
pub const TSTORM_IWARP_CONN_AG_CTX_TIMER_STOP_ALL_MASK: c_uint = 0x3;
pub const TSTORM_IWARP_CONN_AG_CTX_TIMER_STOP_ALL_SHIFT: c_int = 4;
pub const TSTORM_IWARP_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const TSTORM_IWARP_CONN_AG_CTX_CF4_SHIFT: c_int = 6;
    pub flags2: u8,
pub const TSTORM_IWARP_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const TSTORM_IWARP_CONN_AG_CTX_CF5_SHIFT: c_int = 0;
pub const TSTORM_IWARP_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const TSTORM_IWARP_CONN_AG_CTX_CF6_SHIFT: c_int = 2;
pub const TSTORM_IWARP_CONN_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const TSTORM_IWARP_CONN_AG_CTX_CF7_SHIFT: c_int = 4;
pub const TSTORM_IWARP_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const TSTORM_IWARP_CONN_AG_CTX_CF8_SHIFT: c_int = 6;
    pub flags3: u8,
pub const TSTORM_IWARP_CONN_AG_CTX_FLUSH_Q0_AND_TCP_HANDSHAKE_COMPLETE_MASK: c_uint = 0x3;
pub const TSTORM_IWARP_CONN_AG_CTX_FLUSH_Q0_AND_TCP_HANDSHAKE_COMPLETE_SHIFT: c_int = 0;
pub const TSTORM_IWARP_CONN_AG_CTX_FLUSH_OR_ERROR_DETECTED_MASK: c_uint = 0x3;
pub const TSTORM_IWARP_CONN_AG_CTX_FLUSH_OR_ERROR_DETECTED_SHIFT: c_int = 2;
pub const TSTORM_IWARP_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_CF0EN_SHIFT: c_int = 4;
pub const TSTORM_IWARP_CONN_AG_CTX_RQ_POST_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_RQ_POST_CF_EN_SHIFT: c_int = 5;
pub const TSTORM_IWARP_CONN_AG_CTX_MPA_TIMEOUT_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_MPA_TIMEOUT_CF_EN_SHIFT: c_int = 6;
pub const TSTORM_IWARP_CONN_AG_CTX_TIMER_STOP_ALL_EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_TIMER_STOP_ALL_EN_SHIFT: c_int = 7;
    pub flags4: u8,
pub const TSTORM_IWARP_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_CF4EN_SHIFT: c_int = 0;
pub const TSTORM_IWARP_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_CF5EN_SHIFT: c_int = 1;
pub const TSTORM_IWARP_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_CF6EN_SHIFT: c_int = 2;
pub const TSTORM_IWARP_CONN_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_CF7EN_SHIFT: c_int = 3;
pub const TSTORM_IWARP_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_CF8EN_SHIFT: c_int = 4;
pub const TSTORM_IWARP_CONN_AG_CTX_FLUSH_Q0_AND_TCP_HANDSHAKE_COMPL_EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_FLUSH_Q0_AND_TCP_HANDSHAKE_COMPL_EN_SHIFT: c_int = 5;
pub const TSTORM_IWARP_CONN_AG_CTX_FLUSH_OR_ERROR_DETECTED_EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_FLUSH_OR_ERROR_DETECTED_EN_SHIFT: c_int = 6;
pub const TSTORM_IWARP_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 7;
    pub flags5: u8,
pub const TSTORM_IWARP_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 0;
pub const TSTORM_IWARP_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const TSTORM_IWARP_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const TSTORM_IWARP_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const TSTORM_IWARP_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const TSTORM_IWARP_CONN_AG_CTX_SND_SQ_CONS_RULE_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_SND_SQ_CONS_RULE_SHIFT: c_int = 5;
pub const TSTORM_IWARP_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const TSTORM_IWARP_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const TSTORM_IWARP_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub reg0: __le32,
    pub reg1: __le32,
    pub unaligned_nxt_seq: __le32,
    pub reg3: __le32,
    pub reg4: __le32,
    pub reg5: __le32,
    pub reg6: __le32,
    pub reg7: __le32,
    pub reg8: __le32,
    pub orq_cache_idx: u8,
    pub hq_prod: u8,
    pub sq_tx_cons_th: __le16,
    pub orq_prod: u8,
    pub irq_cons: u8,
    pub sq_tx_cons: __le16,
    pub conn_dpi: __le16,
    pub rq_prod: __le16,
    pub snd_seq: __le32,
    pub last_hq_sequence: __le32,
}

// The iwarp storm context of Tstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_iwarp_conn_st_ctx {
    pub reserved: [__le32; 60],
}

// The iwarp storm context of Mstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_iwarp_conn_st_ctx {
    pub reserved: [__le32; 32],
}

// The iwarp storm context of Ustorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_iwarp_conn_st_ctx {
    pub reserved: [regpair; 14],
}

// iwarp connection context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_conn_context {
    pub ystorm_st_context: ystorm_iwarp_conn_st_ctx,
    pub ystorm_st_padding: [regpair; 2],
    pub pstorm_st_context: pstorm_iwarp_conn_st_ctx,
    pub pstorm_st_padding: [regpair; 2],
    pub xstorm_st_context: xstorm_iwarp_conn_st_ctx,
    pub xstorm_ag_context: xstorm_iwarp_conn_ag_ctx,
    pub tstorm_ag_context: tstorm_iwarp_conn_ag_ctx,
    pub timer_context: timers_context,
    pub ustorm_ag_context: ustorm_rdma_conn_ag_ctx,
    pub tstorm_st_context: tstorm_iwarp_conn_st_ctx,
    pub tstorm_st_padding: [regpair; 2],
    pub mstorm_st_context: mstorm_iwarp_conn_st_ctx,
    pub ustorm_st_context: ustorm_iwarp_conn_st_ctx,
    pub ustorm_st_padding: [regpair; 2],
}

// iWARP create QP params passed by driver to FW in CreateQP Request Ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_create_qp_ramrod_data {
    pub flags: u8,
pub const IWARP_CREATE_QP_RAMROD_DATA_FMR_AND_RESERVED_EN_MASK: c_uint = 0x1;
pub const IWARP_CREATE_QP_RAMROD_DATA_FMR_AND_RESERVED_EN_SHIFT: c_int = 0;
pub const IWARP_CREATE_QP_RAMROD_DATA_SIGNALED_COMP_MASK: c_uint = 0x1;
pub const IWARP_CREATE_QP_RAMROD_DATA_SIGNALED_COMP_SHIFT: c_int = 1;
pub const IWARP_CREATE_QP_RAMROD_DATA_RDMA_RD_EN_MASK: c_uint = 0x1;
pub const IWARP_CREATE_QP_RAMROD_DATA_RDMA_RD_EN_SHIFT: c_int = 2;
pub const IWARP_CREATE_QP_RAMROD_DATA_RDMA_WR_EN_MASK: c_uint = 0x1;
pub const IWARP_CREATE_QP_RAMROD_DATA_RDMA_WR_EN_SHIFT: c_int = 3;
pub const IWARP_CREATE_QP_RAMROD_DATA_ATOMIC_EN_MASK: c_uint = 0x1;
pub const IWARP_CREATE_QP_RAMROD_DATA_ATOMIC_EN_SHIFT: c_int = 4;
pub const IWARP_CREATE_QP_RAMROD_DATA_SRQ_FLG_MASK: c_uint = 0x1;
pub const IWARP_CREATE_QP_RAMROD_DATA_SRQ_FLG_SHIFT: c_int = 5;
pub const IWARP_CREATE_QP_RAMROD_DATA_LOW_LATENCY_QUEUE_EN_MASK: c_uint = 0x1;
pub const IWARP_CREATE_QP_RAMROD_DATA_LOW_LATENCY_QUEUE_EN_SHIFT: c_int = 6;
pub const IWARP_CREATE_QP_RAMROD_DATA_RESERVED0_MASK: c_uint = 0x1;
pub const IWARP_CREATE_QP_RAMROD_DATA_RESERVED0_SHIFT: c_int = 7;
    pub reserved1: u8,
    pub pd: __le16,
    pub sq_num_pages: __le16,
    pub rq_num_pages: __le16,
    pub reserved3: [__le32; 2],
    pub qp_handle_for_cqe: regpair,
    pub srq_id: rdma_srq_id,
    pub cq_cid_for_sq: __le32,
    pub cq_cid_for_rq: __le32,
    pub dpi: __le16,
    pub physical_q0: __le16,
    pub physical_q1: __le16,
    pub reserved2: [u8; 6],
}

// iWARP completion queue types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwarp_eqe_async_opcode {
    IWARP_EVENT_TYPE_ASYNC_CONNECT_COMPLETE,
    IWARP_EVENT_TYPE_ASYNC_ENHANCED_MPA_REPLY_ARRIVED,
    IWARP_EVENT_TYPE_ASYNC_MPA_HANDSHAKE_COMPLETE,
    IWARP_EVENT_TYPE_ASYNC_CID_CLEANED,
    IWARP_EVENT_TYPE_ASYNC_EXCEPTION_DETECTED,
    IWARP_EVENT_TYPE_ASYNC_QP_IN_ERROR_STATE,
    IWARP_EVENT_TYPE_ASYNC_CQ_OVERFLOW,
    IWARP_EVENT_TYPE_ASYNC_SRQ_LIMIT,
    IWARP_EVENT_TYPE_ASYNC_SRQ_EMPTY,
    MAX_IWARP_EQE_ASYNC_OPCODE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_eqe_data_mpa_async_completion {
    pub ulp_data_len: __le16,
    pub rtr_type_sent: u8,
    pub reserved: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_eqe_data_tcp_async_completion {
    pub ulp_data_len: __le16,
    pub mpa_handshake_mode: u8,
    pub reserved: [u8; 5],
}

// iWARP completion queue types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwarp_eqe_sync_opcode {
    IWARP_EVENT_TYPE_TCP_OFFLOAD = 13,
    IWARP_EVENT_TYPE_MPA_OFFLOAD,
    IWARP_EVENT_TYPE_MPA_OFFLOAD_SEND_RTR,
    IWARP_EVENT_TYPE_CREATE_QP,
    IWARP_EVENT_TYPE_QUERY_QP,
    IWARP_EVENT_TYPE_MODIFY_QP,
    IWARP_EVENT_TYPE_DESTROY_QP,
    IWARP_EVENT_TYPE_ABORT_TCP_OFFLOAD,
    MAX_IWARP_EQE_SYNC_OPCODE
}

// iWARP EQE completion status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwarp_fw_return_code {
    IWARP_CONN_ERROR_TCP_CONNECT_INVALID_PACKET = 6,
    IWARP_CONN_ERROR_TCP_CONNECTION_RST,
    IWARP_CONN_ERROR_TCP_CONNECT_TIMEOUT,
    IWARP_CONN_ERROR_MPA_ERROR_REJECT,
    IWARP_CONN_ERROR_MPA_NOT_SUPPORTED_VER,
    IWARP_CONN_ERROR_MPA_RST,
    IWARP_CONN_ERROR_MPA_FIN,
    IWARP_CONN_ERROR_MPA_RTR_MISMATCH,
    IWARP_CONN_ERROR_MPA_INSUF_IRD,
    IWARP_CONN_ERROR_MPA_INVALID_PACKET,
    IWARP_CONN_ERROR_MPA_LOCAL_ERROR,
    IWARP_CONN_ERROR_MPA_TIMEOUT,
    IWARP_CONN_ERROR_MPA_TERMINATE,
    IWARP_QP_IN_ERROR_GOOD_CLOSE,
    IWARP_QP_IN_ERROR_BAD_CLOSE,
    IWARP_EXCEPTION_DETECTED_LLP_CLOSED,
    IWARP_EXCEPTION_DETECTED_LLP_RESET,
    IWARP_EXCEPTION_DETECTED_IRQ_FULL,
    IWARP_EXCEPTION_DETECTED_RQ_EMPTY,
    IWARP_EXCEPTION_DETECTED_LLP_TIMEOUT,
    IWARP_EXCEPTION_DETECTED_REMOTE_PROTECTION_ERROR,
    IWARP_EXCEPTION_DETECTED_CQ_OVERFLOW,
    IWARP_EXCEPTION_DETECTED_LOCAL_CATASTROPHIC,
    IWARP_EXCEPTION_DETECTED_LOCAL_ACCESS_ERROR,
    IWARP_EXCEPTION_DETECTED_REMOTE_OPERATION_ERROR,
    IWARP_EXCEPTION_DETECTED_TERMINATE_RECEIVED,
    MAX_IWARP_FW_RETURN_CODE
}

// unaligned opaque data received from LL2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_init_func_params {
    pub ll2_ooo_q_index: u8,
    pub reserved1: [u8; 7],
}

// iwarp func init ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_init_func_ramrod_data {
    pub rdma: rdma_init_func_ramrod_data,
    pub tcp: tcp_init_params,
    pub iwarp: iwarp_init_func_params,
}

// iWARP QP - possible states to transition to
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwarp_modify_qp_new_state_type {
    IWARP_MODIFY_QP_STATE_CLOSING = 1,
    IWARP_MODIFY_QP_STATE_ERROR = 2,
    MAX_IWARP_MODIFY_QP_NEW_STATE_TYPE
}

// iwarp modify qp responder ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_modify_qp_ramrod_data {
    pub transition_to_state: __le16,
    pub flags: __le16,
pub const IWARP_MODIFY_QP_RAMROD_DATA_RDMA_RD_EN_MASK: c_uint = 0x1;
pub const IWARP_MODIFY_QP_RAMROD_DATA_RDMA_RD_EN_SHIFT: c_int = 0;
pub const IWARP_MODIFY_QP_RAMROD_DATA_RDMA_WR_EN_MASK: c_uint = 0x1;
pub const IWARP_MODIFY_QP_RAMROD_DATA_RDMA_WR_EN_SHIFT: c_int = 1;
pub const IWARP_MODIFY_QP_RAMROD_DATA_ATOMIC_EN_MASK: c_uint = 0x1;
pub const IWARP_MODIFY_QP_RAMROD_DATA_ATOMIC_EN_SHIFT: c_int = 2;
pub const IWARP_MODIFY_QP_RAMROD_DATA_STATE_TRANS_EN_MASK: c_uint = 0x1;
pub const IWARP_MODIFY_QP_RAMROD_DATA_STATE_TRANS_EN_SHIFT: c_int = 3;
pub const IWARP_MODIFY_QP_RAMROD_DATA_RDMA_OPS_EN_FLG_MASK: c_uint = 0x1;
pub const IWARP_MODIFY_QP_RAMROD_DATA_RDMA_OPS_EN_FLG_SHIFT: c_int = 4;
pub const IWARP_MODIFY_QP_RAMROD_DATA_PHYSICAL_QUEUE_FLG_MASK: c_uint = 0x1;
pub const IWARP_MODIFY_QP_RAMROD_DATA_PHYSICAL_QUEUE_FLG_SHIFT: c_int = 5;
pub const IWARP_MODIFY_QP_RAMROD_DATA_RESERVED_MASK: c_uint = 0x3FF;
pub const IWARP_MODIFY_QP_RAMROD_DATA_RESERVED_SHIFT: c_int = 6;
    pub physical_q0: __le16,
    pub physical_q1: __le16,
    pub reserved1: [__le32; 10],
}

// MPA params for Enhanced mode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpa_rq_params {
    pub ird: __le32,
    pub ord: __le32,
}

// MPA host Address-Len for private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpa_ulp_buffer {
    pub addr: regpair,
    pub len: __le16,
    pub reserved: [__le16; 3],
}

// iWARP MPA offload params common to Basic and Enhanced modes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpa_outgoing_params {
    pub crc_needed: u8,
    pub reject: u8,
    pub reserved: [u8; 6],
    pub out_rq: mpa_rq_params,
    pub outgoing_ulp_buffer: mpa_ulp_buffer,
}

// iWARP MPA offload params passed by driver to FW in MPA Offload Request
// Ramrod.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_mpa_offload_ramrod_data {
    pub common: mpa_outgoing_params,
    pub tcp_cid: __le32,
    pub mode: u8,
    pub tcp_connect_side: u8,
    pub rtr_pref: u8,
pub const IWARP_MPA_OFFLOAD_RAMROD_DATA_RTR_SUPPORTED_MASK: c_uint = 0x7;
pub const IWARP_MPA_OFFLOAD_RAMROD_DATA_RTR_SUPPORTED_SHIFT: c_int = 0;
pub const IWARP_MPA_OFFLOAD_RAMROD_DATA_RESERVED1_MASK: c_uint = 0x1F;
pub const IWARP_MPA_OFFLOAD_RAMROD_DATA_RESERVED1_SHIFT: c_int = 3;
    pub reserved2: u8,
    pub incoming_ulp_buffer: mpa_ulp_buffer,
    pub async_eqe_output_buf: regpair,
    pub handle_for_async: regpair,
    pub shared_queue_addr: regpair,
    pub additional_setup_time: __le32,
    pub rcv_wnd: __le16,
    pub stats_counter_id: u8,
    pub reserved3: [u8; 9],
}

// iWARP TCP connection offload params passed by driver to FW
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_offload_params {
    pub incoming_ulp_buffer: mpa_ulp_buffer,
    pub async_eqe_output_buf: regpair,
    pub handle_for_async: regpair,
    pub additional_setup_time: __le32,
    pub physical_q0: __le16,
    pub physical_q1: __le16,
    pub stats_counter_id: u8,
    pub mpa_mode: u8,
    pub src_vport_id: u8,
    pub reserved: [u8; 5],
}

// iWARP query QP output params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_query_qp_output_params {
    pub flags: __le32,
pub const IWARP_QUERY_QP_OUTPUT_PARAMS_ERROR_FLG_MASK: c_uint = 0x1;
pub const IWARP_QUERY_QP_OUTPUT_PARAMS_ERROR_FLG_SHIFT: c_int = 0;
pub const IWARP_QUERY_QP_OUTPUT_PARAMS_RESERVED0_MASK: c_uint = 0x7FFFFFFF;
pub const IWARP_QUERY_QP_OUTPUT_PARAMS_RESERVED0_SHIFT: c_int = 1;
    pub reserved1: [u8; 4],
}

// iWARP query QP ramrod data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_query_qp_ramrod_data {
    pub output_params_addr: regpair,
}

// iWARP Ramrod Command IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwarp_ramrod_cmd_id {
    IWARP_RAMROD_CMD_ID_TCP_OFFLOAD = 13,
    IWARP_RAMROD_CMD_ID_MPA_OFFLOAD,
    IWARP_RAMROD_CMD_ID_MPA_OFFLOAD_SEND_RTR,
    IWARP_RAMROD_CMD_ID_CREATE_QP,
    IWARP_RAMROD_CMD_ID_QUERY_QP,
    IWARP_RAMROD_CMD_ID_MODIFY_QP,
    IWARP_RAMROD_CMD_ID_DESTROY_QP,
    IWARP_RAMROD_CMD_ID_ABORT_TCP_OFFLOAD,
    MAX_IWARP_RAMROD_CMD_ID
}

// Per PF iWARP retransmit path statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_rxmit_stats_drv {
    pub tx_go_to_slow_start_event_cnt: regpair,
    pub tx_fast_retransmit_event_cnt: regpair,
}

// iWARP and TCP connection offload params passed by driver to FW in iWARP
// offload ramrod.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwarp_tcp_offload_ramrod_data {
    pub tcp: tcp_offload_params_opt2,
    pub iwarp: iwarp_offload_params,
}

// iWARP MPA negotiation types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpa_negotiation_mode {
    MPA_NEGOTIATION_TYPE_BASIC = 1,
    MPA_NEGOTIATION_TYPE_ENHANCED = 2,
    MAX_MPA_NEGOTIATION_MODE
}

// iWARP MPA Enhanced mode RTR types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpa_rtr_type {
    MPA_RTR_TYPE_NONE = 0,
    MPA_RTR_TYPE_ZERO_SEND = 1,
    MPA_RTR_TYPE_ZERO_WRITE = 2,
    MPA_RTR_TYPE_ZERO_SEND_AND_WRITE = 3,
    MPA_RTR_TYPE_ZERO_READ = 4,
    MPA_RTR_TYPE_ZERO_SEND_AND_READ = 5,
    MPA_RTR_TYPE_ZERO_WRITE_AND_READ = 6,
    MPA_RTR_TYPE_ZERO_SEND_AND_WRITE_AND_READ = 7,
    MAX_MPA_RTR_TYPE
}

// unaligned opaque data received from LL2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unaligned_opaque_data {
    pub first_mpa_offset: __le16,
    pub tcp_payload_offset: u8,
    pub flags: u8,
pub const UNALIGNED_OPAQUE_DATA_PKT_REACHED_WIN_RIGHT_EDGE_MASK: c_uint = 0x1;
pub const UNALIGNED_OPAQUE_DATA_PKT_REACHED_WIN_RIGHT_EDGE_SHIFT: c_int = 0;
pub const UNALIGNED_OPAQUE_DATA_CONNECTION_CLOSED_MASK: c_uint = 0x1;
pub const UNALIGNED_OPAQUE_DATA_CONNECTION_CLOSED_SHIFT: c_int = 1;
pub const UNALIGNED_OPAQUE_DATA_RESERVED_MASK: c_uint = 0x3F;
pub const UNALIGNED_OPAQUE_DATA_RESERVED_SHIFT: c_int = 2;
    pub cid: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_iwarp_conn_ag_ctx {
    pub reserved: u8,
    pub state: u8,
    pub flags0: u8,
pub const MSTORM_IWARP_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const MSTORM_IWARP_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const MSTORM_IWARP_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const MSTORM_IWARP_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const MSTORM_IWARP_CONN_AG_CTX_INV_STAG_DONE_CF_MASK: c_uint = 0x3;
pub const MSTORM_IWARP_CONN_AG_CTX_INV_STAG_DONE_CF_SHIFT: c_int = 2;
pub const MSTORM_IWARP_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const MSTORM_IWARP_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const MSTORM_IWARP_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const MSTORM_IWARP_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const MSTORM_IWARP_CONN_AG_CTX_INV_STAG_DONE_CF_EN_MASK: c_uint = 0x1;
pub const MSTORM_IWARP_CONN_AG_CTX_INV_STAG_DONE_CF_EN_SHIFT: c_int = 0;
pub const MSTORM_IWARP_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const MSTORM_IWARP_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const MSTORM_IWARP_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const MSTORM_IWARP_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const MSTORM_IWARP_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const MSTORM_IWARP_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const MSTORM_IWARP_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const MSTORM_IWARP_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const MSTORM_IWARP_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const MSTORM_IWARP_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const MSTORM_IWARP_CONN_AG_CTX_RCQ_CONS_EN_MASK: c_uint = 0x1;
pub const MSTORM_IWARP_CONN_AG_CTX_RCQ_CONS_EN_SHIFT: c_int = 6;
pub const MSTORM_IWARP_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const MSTORM_IWARP_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub rcq_cons: __le16,
    pub rcq_cons_th: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_iwarp_conn_ag_ctx {
    pub reserved: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const USTORM_IWARP_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const USTORM_IWARP_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const USTORM_IWARP_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const USTORM_IWARP_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const USTORM_IWARP_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const USTORM_IWARP_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const USTORM_IWARP_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const USTORM_IWARP_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const USTORM_IWARP_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const USTORM_IWARP_CONN_AG_CTX_CF3_SHIFT: c_int = 0;
pub const USTORM_IWARP_CONN_AG_CTX_CQ_ARM_SE_CF_MASK: c_uint = 0x3;
pub const USTORM_IWARP_CONN_AG_CTX_CQ_ARM_SE_CF_SHIFT: c_int = 2;
pub const USTORM_IWARP_CONN_AG_CTX_CQ_ARM_CF_MASK: c_uint = 0x3;
pub const USTORM_IWARP_CONN_AG_CTX_CQ_ARM_CF_SHIFT: c_int = 4;
pub const USTORM_IWARP_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const USTORM_IWARP_CONN_AG_CTX_CF6_SHIFT: c_int = 6;
    pub flags2: u8,
pub const USTORM_IWARP_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const USTORM_IWARP_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const USTORM_IWARP_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const USTORM_IWARP_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_CF3EN_SHIFT: c_int = 3;
pub const USTORM_IWARP_CONN_AG_CTX_CQ_ARM_SE_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_CQ_ARM_SE_CF_EN_SHIFT: c_int = 4;
pub const USTORM_IWARP_CONN_AG_CTX_CQ_ARM_CF_EN_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_CQ_ARM_CF_EN_SHIFT: c_int = 5;
pub const USTORM_IWARP_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_CF6EN_SHIFT: c_int = 6;
pub const USTORM_IWARP_CONN_AG_CTX_CQ_SE_EN_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_CQ_SE_EN_SHIFT: c_int = 7;
    pub flags3: u8,
pub const USTORM_IWARP_CONN_AG_CTX_CQ_EN_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_CQ_EN_SHIFT: c_int = 0;
pub const USTORM_IWARP_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const USTORM_IWARP_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const USTORM_IWARP_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const USTORM_IWARP_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const USTORM_IWARP_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 5;
pub const USTORM_IWARP_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const USTORM_IWARP_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const USTORM_IWARP_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub word1: __le16,
    pub cq_cons: __le32,
    pub cq_se_prod: __le32,
    pub cq_prod: __le32,
    pub reg3: __le32,
    pub word2: __le16,
    pub word3: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_iwarp_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const YSTORM_IWARP_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const YSTORM_IWARP_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const YSTORM_IWARP_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const YSTORM_IWARP_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const YSTORM_IWARP_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const YSTORM_IWARP_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const YSTORM_IWARP_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const YSTORM_IWARP_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const YSTORM_IWARP_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const YSTORM_IWARP_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const YSTORM_IWARP_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const YSTORM_IWARP_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const YSTORM_IWARP_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const YSTORM_IWARP_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const YSTORM_IWARP_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const YSTORM_IWARP_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const YSTORM_IWARP_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const YSTORM_IWARP_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const YSTORM_IWARP_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const YSTORM_IWARP_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const YSTORM_IWARP_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const YSTORM_IWARP_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const YSTORM_IWARP_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const YSTORM_IWARP_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const YSTORM_IWARP_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const YSTORM_IWARP_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
    pub word1: __le16,
    pub word2: __le16,
    pub word3: __le16,
    pub word4: __le16,
    pub reg2: __le32,
    pub reg3: __le32,
}

// The fcoe storm context of Ystorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_fcoe_conn_st_ctx {
    pub func_mode: u8,
    pub cos: u8,
    pub conf_version: u8,
    pub eth_hdr_size: u8,
    pub stat_ram_addr: __le16,
    pub mtu: __le16,
    pub max_fc_payload_len: __le16,
    pub tx_max_fc_pay_len: __le16,
    pub fcp_cmd_size: u8,
    pub fcp_rsp_size: u8,
    pub mss: __le16,
    pub reserved: regpair,
    pub min_frame_size: __le16,
    pub protection_info_flags: u8,
pub const YSTORM_FCOE_CONN_ST_CTX_SUPPORT_PROTECTION_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_CONN_ST_CTX_SUPPORT_PROTECTION_SHIFT: c_int = 0;
pub const YSTORM_FCOE_CONN_ST_CTX_VALID_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_CONN_ST_CTX_VALID_SHIFT: c_int = 1;
pub const YSTORM_FCOE_CONN_ST_CTX_RESERVED1_MASK: c_uint = 0x3F;
pub const YSTORM_FCOE_CONN_ST_CTX_RESERVED1_SHIFT: c_int = 2;
    pub dst_protection_per_mss: u8,
    pub src_protection_per_mss: u8,
    pub ptu_log_page_size: u8,
    pub flags: u8,
pub const YSTORM_FCOE_CONN_ST_CTX_INNER_VLAN_FLAG_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_CONN_ST_CTX_INNER_VLAN_FLAG_SHIFT: c_int = 0;
pub const YSTORM_FCOE_CONN_ST_CTX_OUTER_VLAN_FLAG_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_CONN_ST_CTX_OUTER_VLAN_FLAG_SHIFT: c_int = 1;
pub const YSTORM_FCOE_CONN_ST_CTX_RSRV_MASK: c_uint = 0x3F;
pub const YSTORM_FCOE_CONN_ST_CTX_RSRV_SHIFT: c_int = 2;
    pub fcp_xfer_size: u8,
}

// FCoE 16-bits vlan structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_vlan_fields {
    pub fields: __le16,
pub const FCOE_VLAN_FIELDS_VID_MASK: c_uint = 0xFFF;
pub const FCOE_VLAN_FIELDS_VID_SHIFT: c_int = 0;
pub const FCOE_VLAN_FIELDS_CLI_MASK: c_uint = 0x1;
pub const FCOE_VLAN_FIELDS_CLI_SHIFT: c_int = 12;
pub const FCOE_VLAN_FIELDS_PRI_MASK: c_uint = 0x7;
pub const FCOE_VLAN_FIELDS_PRI_SHIFT: c_int = 13;
}

// FCoE 16-bits vlan union
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_vlan_field_union {
    pub fields: fcoe_vlan_fields,
    pub val: __le16,
}

// FCoE 16-bits vlan, vif union
#[repr(C)]
#[derive(Copy, Clone)]
pub union fcoe_vlan_vif_field_union {
    pub vlan: fcoe_vlan_field_union,
    pub vif: __le16,
}

// Ethernet context section
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstorm_fcoe_eth_context_section {
    pub remote_addr_3: u8,
    pub remote_addr_2: u8,
    pub remote_addr_1: u8,
    pub remote_addr_0: u8,
    pub local_addr_1: u8,
    pub local_addr_0: u8,
    pub remote_addr_5: u8,
    pub remote_addr_4: u8,
    pub local_addr_5: u8,
    pub local_addr_4: u8,
    pub local_addr_3: u8,
    pub local_addr_2: u8,
    pub vif_outer_vlan: fcoe_vlan_vif_field_union,
    pub vif_outer_eth_type: __le16,
    pub inner_vlan: fcoe_vlan_vif_field_union,
    pub inner_eth_type: __le16,
}

// The fcoe storm context of Pstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstorm_fcoe_conn_st_ctx {
    pub func_mode: u8,
    pub cos: u8,
    pub conf_version: u8,
    pub rsrv: u8,
    pub stat_ram_addr: __le16,
    pub mss: __le16,
    pub abts_cleanup_addr: regpair,
    pub eth: pstorm_fcoe_eth_context_section,
    pub sid_2: u8,
    pub sid_1: u8,
    pub sid_0: u8,
    pub flags: u8,
pub const PSTORM_FCOE_CONN_ST_CTX_VNTAG_VLAN_MASK: c_uint = 0x1;
pub const PSTORM_FCOE_CONN_ST_CTX_VNTAG_VLAN_SHIFT: c_int = 0;
pub const PSTORM_FCOE_CONN_ST_CTX_SUPPORT_REC_RR_TOV_MASK: c_uint = 0x1;
pub const PSTORM_FCOE_CONN_ST_CTX_SUPPORT_REC_RR_TOV_SHIFT: c_int = 1;
pub const PSTORM_FCOE_CONN_ST_CTX_INNER_VLAN_FLAG_MASK: c_uint = 0x1;
pub const PSTORM_FCOE_CONN_ST_CTX_INNER_VLAN_FLAG_SHIFT: c_int = 2;
pub const PSTORM_FCOE_CONN_ST_CTX_OUTER_VLAN_FLAG_MASK: c_uint = 0x1;
pub const PSTORM_FCOE_CONN_ST_CTX_OUTER_VLAN_FLAG_SHIFT: c_int = 3;
pub const PSTORM_FCOE_CONN_ST_CTX_SINGLE_VLAN_FLAG_MASK: c_uint = 0x1;
pub const PSTORM_FCOE_CONN_ST_CTX_SINGLE_VLAN_FLAG_SHIFT: c_int = 4;
pub const PSTORM_FCOE_CONN_ST_CTX_RESERVED_MASK: c_uint = 0x7;
pub const PSTORM_FCOE_CONN_ST_CTX_RESERVED_SHIFT: c_int = 5;
    pub did_2: u8,
    pub did_1: u8,
    pub did_0: u8,
    pub src_mac_index: u8,
    pub rec_rr_tov_val: __le16,
    pub q_relative_offset: u8,
    pub reserved1: u8,
}

// The fcoe storm context of Xstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_fcoe_conn_st_ctx {
    pub func_mode: u8,
    pub src_mac_index: u8,
    pub conf_version: u8,
    pub cached_wqes_avail: u8,
    pub stat_ram_addr: __le16,
    pub flags: u8,
pub const XSTORM_FCOE_CONN_ST_CTX_SQ_DEFERRED_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_ST_CTX_SQ_DEFERRED_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_ST_CTX_INNER_VLAN_FLAG_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_ST_CTX_INNER_VLAN_FLAG_SHIFT: c_int = 1;
pub const XSTORM_FCOE_CONN_ST_CTX_INNER_VLAN_FLAG_ORIG_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_ST_CTX_INNER_VLAN_FLAG_ORIG_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_ST_CTX_LAST_QUEUE_HANDLED_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_ST_CTX_LAST_QUEUE_HANDLED_SHIFT: c_int = 3;
pub const XSTORM_FCOE_CONN_ST_CTX_RSRV_MASK: c_uint = 0x7;
pub const XSTORM_FCOE_CONN_ST_CTX_RSRV_SHIFT: c_int = 5;
    pub cached_wqes_offset: u8,
    pub reserved2: u8,
    pub eth_hdr_size: u8,
    pub seq_id: u8,
    pub max_conc_seqs: u8,
    pub num_pages_in_pbl: __le16,
    pub reserved: __le16,
    pub sq_pbl_addr: regpair,
    pub sq_curr_page_addr: regpair,
    pub sq_next_page_addr: regpair,
    pub xferq_pbl_addr: regpair,
    pub xferq_curr_page_addr: regpair,
    pub xferq_next_page_addr: regpair,
    pub respq_pbl_addr: regpair,
    pub respq_curr_page_addr: regpair,
    pub respq_next_page_addr: regpair,
    pub mtu: __le16,
    pub tx_max_fc_pay_len: __le16,
    pub max_fc_payload_len: __le16,
    pub min_frame_size: __le16,
    pub sq_pbl_next_index: __le16,
    pub respq_pbl_next_index: __le16,
    pub fcp_cmd_byte_credit: u8,
    pub fcp_rsp_byte_credit: u8,
    pub protection_info: __le16,
pub const XSTORM_FCOE_CONN_ST_CTX_PROTECTION_PERF_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_ST_CTX_PROTECTION_PERF_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_ST_CTX_SUPPORT_PROTECTION_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_ST_CTX_SUPPORT_PROTECTION_SHIFT: c_int = 1;
pub const XSTORM_FCOE_CONN_ST_CTX_VALID_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_ST_CTX_VALID_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_ST_CTX_FRAME_PROT_ALIGNED_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_ST_CTX_FRAME_PROT_ALIGNED_SHIFT: c_int = 3;
pub const XSTORM_FCOE_CONN_ST_CTX_RESERVED3_MASK: c_uint = 0xF;
pub const XSTORM_FCOE_CONN_ST_CTX_RESERVED3_SHIFT: c_int = 4;
pub const XSTORM_FCOE_CONN_ST_CTX_DST_PROTECTION_PER_MSS_MASK: c_uint = 0xFF;
pub const XSTORM_FCOE_CONN_ST_CTX_DST_PROTECTION_PER_MSS_SHIFT: c_int = 8;
    pub xferq_pbl_next_index: __le16,
    pub page_size: __le16,
    pub mid_seq: u8,
    pub fcp_xfer_byte_credit: u8,
    pub reserved1: [u8; 2],
    pub cached_wqes: [fcoe_wqe; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_fcoe_conn_ag_ctx {
    pub reserved0: u8,
    pub state: u8,
    pub flags0: u8,
pub const XSTORM_FCOE_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED1_SHIFT: c_int = 1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED2_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_AG_CTX_EXIST_IN_QM3_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_EXIST_IN_QM3_SHIFT: c_int = 3;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED3_SHIFT: c_int = 4;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED4_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED4_SHIFT: c_int = 5;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED5_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED5_SHIFT: c_int = 6;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED6_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED6_SHIFT: c_int = 7;
    pub flags1: u8,
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED7_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED7_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED8_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED8_SHIFT: c_int = 1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED9_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED9_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT11_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT11_SHIFT: c_int = 3;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT12_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT12_SHIFT: c_int = 4;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT13_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT13_SHIFT: c_int = 5;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT14_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT14_SHIFT: c_int = 6;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT15_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT15_SHIFT: c_int = 7;
    pub flags2: u8,
pub const XSTORM_FCOE_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF0_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF1_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF2_SHIFT: c_int = 4;
pub const XSTORM_FCOE_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF3_SHIFT: c_int = 6;
    pub flags3: u8,
pub const XSTORM_FCOE_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF4_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF5_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF6_SHIFT: c_int = 4;
pub const XSTORM_FCOE_CONN_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF7_SHIFT: c_int = 6;
    pub flags4: u8,
pub const XSTORM_FCOE_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF8_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_AG_CTX_CF9_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF9_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_AG_CTX_CF10_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF10_SHIFT: c_int = 4;
pub const XSTORM_FCOE_CONN_AG_CTX_CF11_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF11_SHIFT: c_int = 6;
    pub flags5: u8,
pub const XSTORM_FCOE_CONN_AG_CTX_CF12_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF12_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_AG_CTX_CF13_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF13_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_AG_CTX_CF14_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF14_SHIFT: c_int = 4;
pub const XSTORM_FCOE_CONN_AG_CTX_CF15_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF15_SHIFT: c_int = 6;
    pub flags6: u8,
pub const XSTORM_FCOE_CONN_AG_CTX_CF16_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF16_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_AG_CTX_CF17_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF17_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_AG_CTX_CF18_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF18_SHIFT: c_int = 4;
pub const XSTORM_FCOE_CONN_AG_CTX_DQ_CF_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_DQ_CF_SHIFT: c_int = 6;
    pub flags7: u8,
pub const XSTORM_FCOE_CONN_AG_CTX_FLUSH_Q0_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_FLUSH_Q0_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED10_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED10_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_AG_CTX_SLOW_PATH_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_SLOW_PATH_SHIFT: c_int = 4;
pub const XSTORM_FCOE_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF0EN_SHIFT: c_int = 6;
pub const XSTORM_FCOE_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags8: u8,
pub const XSTORM_FCOE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF3EN_SHIFT: c_int = 1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF4EN_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF5EN_SHIFT: c_int = 3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF6EN_SHIFT: c_int = 4;
pub const XSTORM_FCOE_CONN_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF7EN_SHIFT: c_int = 5;
pub const XSTORM_FCOE_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF8EN_SHIFT: c_int = 6;
pub const XSTORM_FCOE_CONN_AG_CTX_CF9EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF9EN_SHIFT: c_int = 7;
    pub flags9: u8,
pub const XSTORM_FCOE_CONN_AG_CTX_CF10EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF10EN_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_AG_CTX_CF11EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF11EN_SHIFT: c_int = 1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF12EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF12EN_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_AG_CTX_CF13EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF13EN_SHIFT: c_int = 3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF14EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF14EN_SHIFT: c_int = 4;
pub const XSTORM_FCOE_CONN_AG_CTX_CF15EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF15EN_SHIFT: c_int = 5;
pub const XSTORM_FCOE_CONN_AG_CTX_CF16EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF16EN_SHIFT: c_int = 6;
pub const XSTORM_FCOE_CONN_AG_CTX_CF17EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF17EN_SHIFT: c_int = 7;
    pub flags10: u8,
pub const XSTORM_FCOE_CONN_AG_CTX_CF18EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF18EN_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_AG_CTX_DQ_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_DQ_CF_EN_SHIFT: c_int = 1;
pub const XSTORM_FCOE_CONN_AG_CTX_FLUSH_Q0_EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_FLUSH_Q0_EN_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED11_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED11_SHIFT: c_int = 3;
pub const XSTORM_FCOE_CONN_AG_CTX_SLOW_PATH_EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_SLOW_PATH_EN_SHIFT: c_int = 4;
pub const XSTORM_FCOE_CONN_AG_CTX_CF23EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_CF23EN_SHIFT: c_int = 5;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED12_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED12_SHIFT: c_int = 6;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED13_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED13_SHIFT: c_int = 7;
    pub flags11: u8,
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED14_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED14_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED15_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED15_SHIFT: c_int = 1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED16_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESERVED16_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 3;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 4;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 5;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED1_SHIFT: c_int = 6;
pub const XSTORM_FCOE_CONN_AG_CTX_XFERQ_DECISION_EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_XFERQ_DECISION_EN_SHIFT: c_int = 7;
    pub flags12: u8,
pub const XSTORM_FCOE_CONN_AG_CTX_SQ_DECISION_EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_SQ_DECISION_EN_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE11EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE11EN_SHIFT: c_int = 1;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED2_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED3_SHIFT: c_int = 3;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE14EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE14EN_SHIFT: c_int = 4;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE15EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE15EN_SHIFT: c_int = 5;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE16EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE16EN_SHIFT: c_int = 6;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE17EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE17EN_SHIFT: c_int = 7;
    pub flags13: u8,
pub const XSTORM_FCOE_CONN_AG_CTX_RESPQ_DECISION_EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RESPQ_DECISION_EN_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE19EN_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_RULE19EN_SHIFT: c_int = 1;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED4_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED4_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED5_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED5_SHIFT: c_int = 3;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED6_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED6_SHIFT: c_int = 4;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED7_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED7_SHIFT: c_int = 5;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED8_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED8_SHIFT: c_int = 6;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED9_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_A0_RESERVED9_SHIFT: c_int = 7;
    pub flags14: u8,
pub const XSTORM_FCOE_CONN_AG_CTX_BIT16_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT16_SHIFT: c_int = 0;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT17_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT17_SHIFT: c_int = 1;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT18_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT18_SHIFT: c_int = 2;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT19_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT19_SHIFT: c_int = 3;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT20_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT20_SHIFT: c_int = 4;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT21_MASK: c_uint = 0x1;
pub const XSTORM_FCOE_CONN_AG_CTX_BIT21_SHIFT: c_int = 5;
pub const XSTORM_FCOE_CONN_AG_CTX_CF23_MASK: c_uint = 0x3;
pub const XSTORM_FCOE_CONN_AG_CTX_CF23_SHIFT: c_int = 6;
    pub byte2: u8,
    pub physical_q0: __le16,
    pub word1: __le16,
    pub word2: __le16,
    pub sq_cons: __le16,
    pub sq_prod: __le16,
    pub xferq_prod: __le16,
    pub xferq_cons: __le16,
    pub byte3: u8,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub remain_io: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub reg3: __le32,
    pub reg4: __le32,
    pub reg5: __le32,
    pub reg6: __le32,
    pub respq_prod: __le16,
    pub respq_cons: __le16,
    pub word9: __le16,
    pub word10: __le16,
    pub reg7: __le32,
    pub reg8: __le32,
}

// The fcoe storm context of Ustorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_fcoe_conn_st_ctx {
    pub respq_pbl_addr: regpair,
    pub num_pages_in_pbl: __le16,
    pub ptu_log_page_size: u8,
    pub log_page_size: u8,
    pub respq_prod: __le16,
    pub reserved: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_fcoe_conn_ag_ctx {
    pub reserved0: u8,
    pub state: u8,
    pub flags0: u8,
pub const TSTORM_FCOE_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const TSTORM_FCOE_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const TSTORM_FCOE_CONN_AG_CTX_BIT2_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_BIT2_SHIFT: c_int = 2;
pub const TSTORM_FCOE_CONN_AG_CTX_BIT3_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_BIT3_SHIFT: c_int = 3;
pub const TSTORM_FCOE_CONN_AG_CTX_BIT4_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_BIT4_SHIFT: c_int = 4;
pub const TSTORM_FCOE_CONN_AG_CTX_BIT5_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_BIT5_SHIFT: c_int = 5;
pub const TSTORM_FCOE_CONN_AG_CTX_DUMMY_TIMER_CF_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_CONN_AG_CTX_DUMMY_TIMER_CF_SHIFT: c_int = 6;
    pub flags1: u8,
pub const TSTORM_FCOE_CONN_AG_CTX_FLUSH_Q0_CF_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_CONN_AG_CTX_FLUSH_Q0_CF_SHIFT: c_int = 0;
pub const TSTORM_FCOE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_CONN_AG_CTX_CF2_SHIFT: c_int = 2;
pub const TSTORM_FCOE_CONN_AG_CTX_TIMER_STOP_ALL_CF_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_CONN_AG_CTX_TIMER_STOP_ALL_CF_SHIFT: c_int = 4;
pub const TSTORM_FCOE_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_CONN_AG_CTX_CF4_SHIFT: c_int = 6;
    pub flags2: u8,
pub const TSTORM_FCOE_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_CONN_AG_CTX_CF5_SHIFT: c_int = 0;
pub const TSTORM_FCOE_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_CONN_AG_CTX_CF6_SHIFT: c_int = 2;
pub const TSTORM_FCOE_CONN_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_CONN_AG_CTX_CF7_SHIFT: c_int = 4;
pub const TSTORM_FCOE_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_CONN_AG_CTX_CF8_SHIFT: c_int = 6;
    pub flags3: u8,
pub const TSTORM_FCOE_CONN_AG_CTX_CF9_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_CONN_AG_CTX_CF9_SHIFT: c_int = 0;
pub const TSTORM_FCOE_CONN_AG_CTX_CF10_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_CONN_AG_CTX_CF10_SHIFT: c_int = 2;
pub const TSTORM_FCOE_CONN_AG_CTX_DUMMY_TIMER_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_DUMMY_TIMER_CF_EN_SHIFT: c_int = 4;
pub const TSTORM_FCOE_CONN_AG_CTX_FLUSH_Q0_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_FLUSH_Q0_CF_EN_SHIFT: c_int = 5;
pub const TSTORM_FCOE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 6;
pub const TSTORM_FCOE_CONN_AG_CTX_TIMER_STOP_ALL_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_TIMER_STOP_ALL_CF_EN_SHIFT: c_int = 7;
    pub flags4: u8,
pub const TSTORM_FCOE_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_CF4EN_SHIFT: c_int = 0;
pub const TSTORM_FCOE_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_CF5EN_SHIFT: c_int = 1;
pub const TSTORM_FCOE_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_CF6EN_SHIFT: c_int = 2;
pub const TSTORM_FCOE_CONN_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_CF7EN_SHIFT: c_int = 3;
pub const TSTORM_FCOE_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_CF8EN_SHIFT: c_int = 4;
pub const TSTORM_FCOE_CONN_AG_CTX_CF9EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_CF9EN_SHIFT: c_int = 5;
pub const TSTORM_FCOE_CONN_AG_CTX_CF10EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_CF10EN_SHIFT: c_int = 6;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 7;
    pub flags5: u8,
pub const TSTORM_FCOE_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 0;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 5;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub reg0: __le32,
    pub reg1: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_fcoe_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const USTORM_FCOE_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const USTORM_FCOE_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const USTORM_FCOE_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const USTORM_FCOE_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const USTORM_FCOE_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const USTORM_FCOE_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const USTORM_FCOE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const USTORM_FCOE_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const USTORM_FCOE_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const USTORM_FCOE_CONN_AG_CTX_CF3_SHIFT: c_int = 0;
pub const USTORM_FCOE_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const USTORM_FCOE_CONN_AG_CTX_CF4_SHIFT: c_int = 2;
pub const USTORM_FCOE_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const USTORM_FCOE_CONN_AG_CTX_CF5_SHIFT: c_int = 4;
pub const USTORM_FCOE_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const USTORM_FCOE_CONN_AG_CTX_CF6_SHIFT: c_int = 6;
    pub flags2: u8,
pub const USTORM_FCOE_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const USTORM_FCOE_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const USTORM_FCOE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const USTORM_FCOE_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_CF3EN_SHIFT: c_int = 3;
pub const USTORM_FCOE_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_CF4EN_SHIFT: c_int = 4;
pub const USTORM_FCOE_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_CF5EN_SHIFT: c_int = 5;
pub const USTORM_FCOE_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_CF6EN_SHIFT: c_int = 6;
pub const USTORM_FCOE_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 7;
    pub flags3: u8,
pub const USTORM_FCOE_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 0;
pub const USTORM_FCOE_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const USTORM_FCOE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const USTORM_FCOE_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const USTORM_FCOE_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const USTORM_FCOE_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 5;
pub const USTORM_FCOE_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const USTORM_FCOE_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const USTORM_FCOE_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub word1: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub reg3: __le32,
    pub word2: __le16,
    pub word3: __le16,
}

// The fcoe storm context of Tstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_fcoe_conn_st_ctx {
    pub stat_ram_addr: __le16,
    pub rx_max_fc_payload_len: __le16,
    pub e_d_tov_val: __le16,
    pub flags: u8,
pub const TSTORM_FCOE_CONN_ST_CTX_INC_SEQ_CNT_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_ST_CTX_INC_SEQ_CNT_SHIFT: c_int = 0;
pub const TSTORM_FCOE_CONN_ST_CTX_SUPPORT_CONF_MASK: c_uint = 0x1;
pub const TSTORM_FCOE_CONN_ST_CTX_SUPPORT_CONF_SHIFT: c_int = 1;
pub const TSTORM_FCOE_CONN_ST_CTX_DEF_Q_IDX_MASK: c_uint = 0x3F;
pub const TSTORM_FCOE_CONN_ST_CTX_DEF_Q_IDX_SHIFT: c_int = 2;
    pub timers_cleanup_invocation_cnt: u8,
    pub reserved1: [__le32; 2],
    pub dst_mac_address_bytes_0_to_3: __le32,
    pub dst_mac_address_bytes_4_to_5: __le16,
    pub ramrod_echo: __le16,
    pub flags1: u8,
pub const TSTORM_FCOE_CONN_ST_CTX_MODE_MASK: c_uint = 0x3;
pub const TSTORM_FCOE_CONN_ST_CTX_MODE_SHIFT: c_int = 0;
pub const TSTORM_FCOE_CONN_ST_CTX_RESERVED_MASK: c_uint = 0x3F;
pub const TSTORM_FCOE_CONN_ST_CTX_RESERVED_SHIFT: c_int = 2;
    pub cq_relative_offset: u8,
    pub cmdq_relative_offset: u8,
    pub bdq_resource_id: u8,
    pub reserved0: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_fcoe_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const MSTORM_FCOE_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const MSTORM_FCOE_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const MSTORM_FCOE_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const MSTORM_FCOE_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const MSTORM_FCOE_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const MSTORM_FCOE_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const MSTORM_FCOE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const MSTORM_FCOE_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const MSTORM_FCOE_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const MSTORM_FCOE_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const MSTORM_FCOE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const MSTORM_FCOE_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const MSTORM_FCOE_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const MSTORM_FCOE_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const MSTORM_FCOE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const MSTORM_FCOE_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const MSTORM_FCOE_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub word0: __le16,
    pub word1: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
}

// Fast path part of the fcoe storm context of Mstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_mstorm_fcoe_conn_st_ctx_fp {
    pub xfer_prod: __le16,
    pub num_cqs: u8,
    pub reserved1: u8,
    pub protection_info: u8,
pub const FCOE_MSTORM_FCOE_CONN_ST_CTX_FP_SUPPORT_PROTECTION_MASK: c_uint = 0x1;
pub const FCOE_MSTORM_FCOE_CONN_ST_CTX_FP_SUPPORT_PROTECTION_SHIFT: c_int = 0;
pub const FCOE_MSTORM_FCOE_CONN_ST_CTX_FP_VALID_MASK: c_uint = 0x1;
pub const FCOE_MSTORM_FCOE_CONN_ST_CTX_FP_VALID_SHIFT: c_int = 1;
pub const FCOE_MSTORM_FCOE_CONN_ST_CTX_FP_RESERVED0_MASK: c_uint = 0x3F;
pub const FCOE_MSTORM_FCOE_CONN_ST_CTX_FP_RESERVED0_SHIFT: c_int = 2;
    pub q_relative_offset: u8,
    pub reserved2: [u8; 2],
}

// Non fast path part of the fcoe storm context of Mstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_mstorm_fcoe_conn_st_ctx_non_fp {
    pub conn_id: __le16,
    pub stat_ram_addr: __le16,
    pub num_pages_in_pbl: __le16,
    pub ptu_log_page_size: u8,
    pub log_page_size: u8,
    pub unsolicited_cq_count: __le16,
    pub cmdq_count: __le16,
    pub bdq_resource_id: u8,
    pub reserved0: [u8; 3],
    pub xferq_pbl_addr: regpair,
    pub reserved1: regpair,
    pub reserved2: [regpair; 3],
}

// The fcoe storm context of Mstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_fcoe_conn_st_ctx {
    pub fp: fcoe_mstorm_fcoe_conn_st_ctx_fp,
    pub non_fp: fcoe_mstorm_fcoe_conn_st_ctx_non_fp,
}

// fcoe connection context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_conn_context {
    pub ystorm_st_context: ystorm_fcoe_conn_st_ctx,
    pub pstorm_st_context: pstorm_fcoe_conn_st_ctx,
    pub pstorm_st_padding: [regpair; 2],
    pub xstorm_st_context: xstorm_fcoe_conn_st_ctx,
    pub xstorm_ag_context: xstorm_fcoe_conn_ag_ctx,
    pub xstorm_ag_padding: [regpair; 6],
    pub ustorm_st_context: ustorm_fcoe_conn_st_ctx,
    pub ustorm_st_padding: [regpair; 2],
    pub tstorm_ag_context: tstorm_fcoe_conn_ag_ctx,
    pub tstorm_ag_padding: [regpair; 2],
    pub timer_context: timers_context,
    pub ustorm_ag_context: ustorm_fcoe_conn_ag_ctx,
    pub tstorm_st_context: tstorm_fcoe_conn_st_ctx,
    pub mstorm_ag_context: mstorm_fcoe_conn_ag_ctx,
    pub mstorm_st_context: mstorm_fcoe_conn_st_ctx,
}

// FCoE connection offload params passed by driver to FW in FCoE offload
// ramrod.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_conn_offload_ramrod_params {
    pub offload_ramrod_data: fcoe_conn_offload_ramrod_data,
}

// FCoE connection terminate params passed by driver to FW in FCoE terminate
// conn ramrod.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_conn_terminate_ramrod_params {
    pub terminate_ramrod_data: fcoe_conn_terminate_ramrod_data,
}

// FCoE event type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcoe_event_type {
    FCOE_EVENT_INIT_FUNC,
    FCOE_EVENT_DESTROY_FUNC,
    FCOE_EVENT_STAT_FUNC,
    FCOE_EVENT_OFFLOAD_CONN,
    FCOE_EVENT_TERMINATE_CONN,
    FCOE_EVENT_ERROR,
    MAX_FCOE_EVENT_TYPE
}

// FCoE init params passed by driver to FW in FCoE init ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_init_ramrod_params {
    pub init_ramrod_data: fcoe_init_func_ramrod_data,
}

// FCoE ramrod Command IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcoe_ramrod_cmd_id {
    FCOE_RAMROD_CMD_ID_INIT_FUNC,
    FCOE_RAMROD_CMD_ID_DESTROY_FUNC,
    FCOE_RAMROD_CMD_ID_STAT_FUNC,
    FCOE_RAMROD_CMD_ID_OFFLOAD_CONN,
    FCOE_RAMROD_CMD_ID_TERMINATE_CONN,
    MAX_FCOE_RAMROD_CMD_ID
}

// FCoE statistics params buffer passed by driver to FW in FCoE statistics
// ramrod.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_stat_ramrod_params {
    pub stat_ramrod_data: fcoe_stat_ramrod_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_fcoe_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const YSTORM_FCOE_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const YSTORM_FCOE_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const YSTORM_FCOE_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const YSTORM_FCOE_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const YSTORM_FCOE_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const YSTORM_FCOE_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const YSTORM_FCOE_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const YSTORM_FCOE_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const YSTORM_FCOE_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const YSTORM_FCOE_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const YSTORM_FCOE_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const YSTORM_FCOE_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const YSTORM_FCOE_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const YSTORM_FCOE_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const YSTORM_FCOE_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const YSTORM_FCOE_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const YSTORM_FCOE_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
    pub word1: __le16,
    pub word2: __le16,
    pub word3: __le16,
    pub word4: __le16,
    pub reg2: __le32,
    pub reg3: __le32,
}

// The iscsi storm connection context of Ystorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_iscsi_conn_st_ctx {
    pub reserved: [__le32; 8],
}

// Combined iSCSI and TCP storm connection of Pstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pstorm_iscsi_tcp_conn_st_ctx {
    pub tcp: [__le32; 32],
    pub iscsi: [__le32; 4],
}

// The combined tcp and iscsi storm context of Xstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_iscsi_tcp_conn_st_ctx {
    pub reserved_tcp: [__le32; 4],
    pub reserved_iscsi: [__le32; 44],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_iscsi_conn_ag_ctx {
    pub cdu_validation: u8,
    pub state: u8,
    pub flags0: u8,
pub const XSTORM_ISCSI_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const XSTORM_ISCSI_CONN_AG_CTX_EXIST_IN_QM1_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_EXIST_IN_QM1_SHIFT: c_int = 1;
pub const XSTORM_ISCSI_CONN_AG_CTX_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_RESERVED1_SHIFT: c_int = 2;
pub const XSTORM_ISCSI_CONN_AG_CTX_EXIST_IN_QM3_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_EXIST_IN_QM3_SHIFT: c_int = 3;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT4_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT4_SHIFT: c_int = 4;
pub const XSTORM_ISCSI_CONN_AG_CTX_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_RESERVED2_SHIFT: c_int = 5;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT6_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT6_SHIFT: c_int = 6;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT7_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT7_SHIFT: c_int = 7;
    pub flags1: u8,
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT8_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT8_SHIFT: c_int = 0;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT9_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT9_SHIFT: c_int = 1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT10_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT10_SHIFT: c_int = 2;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT11_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT11_SHIFT: c_int = 3;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT12_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT12_SHIFT: c_int = 4;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT13_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT13_SHIFT: c_int = 5;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT14_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT14_SHIFT: c_int = 6;
pub const XSTORM_ISCSI_CONN_AG_CTX_TX_TRUNCATE_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_TX_TRUNCATE_SHIFT: c_int = 7;
    pub flags2: u8,
pub const XSTORM_ISCSI_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF0_SHIFT: c_int = 0;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF1_SHIFT: c_int = 2;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF2_SHIFT: c_int = 4;
pub const XSTORM_ISCSI_CONN_AG_CTX_TIMER_STOP_ALL_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_TIMER_STOP_ALL_SHIFT: c_int = 6;
    pub flags3: u8,
pub const XSTORM_ISCSI_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF4_SHIFT: c_int = 0;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF5_SHIFT: c_int = 2;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF6_SHIFT: c_int = 4;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF7_SHIFT: c_int = 6;
    pub flags4: u8,
pub const XSTORM_ISCSI_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF8_SHIFT: c_int = 0;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF9_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF9_SHIFT: c_int = 2;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF10_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF10_SHIFT: c_int = 4;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF11_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF11_SHIFT: c_int = 6;
    pub flags5: u8,
pub const XSTORM_ISCSI_CONN_AG_CTX_CF12_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF12_SHIFT: c_int = 0;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF13_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF13_SHIFT: c_int = 2;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF14_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF14_SHIFT: c_int = 4;
pub const XSTORM_ISCSI_CONN_AG_CTX_UPDATE_STATE_TO_BASE_CF_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_UPDATE_STATE_TO_BASE_CF_SHIFT: c_int = 6;
    pub flags6: u8,
pub const XSTORM_ISCSI_CONN_AG_CTX_CF16_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF16_SHIFT: c_int = 0;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF17_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF17_SHIFT: c_int = 2;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF18_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF18_SHIFT: c_int = 4;
pub const XSTORM_ISCSI_CONN_AG_CTX_DQ_FLUSH_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_DQ_FLUSH_SHIFT: c_int = 6;
    pub flags7: u8,
pub const XSTORM_ISCSI_CONN_AG_CTX_MST_XCM_Q0_FLUSH_CF_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_MST_XCM_Q0_FLUSH_CF_SHIFT: c_int = 0;
pub const XSTORM_ISCSI_CONN_AG_CTX_UST_XCM_Q1_FLUSH_CF_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_UST_XCM_Q1_FLUSH_CF_SHIFT: c_int = 2;
pub const XSTORM_ISCSI_CONN_AG_CTX_SLOW_PATH_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_SLOW_PATH_SHIFT: c_int = 4;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF0EN_SHIFT: c_int = 6;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF1EN_SHIFT: c_int = 7;
    pub flags8: u8,
pub const XSTORM_ISCSI_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF2EN_SHIFT: c_int = 0;
pub const XSTORM_ISCSI_CONN_AG_CTX_TIMER_STOP_ALL_EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_TIMER_STOP_ALL_EN_SHIFT: c_int = 1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF4EN_SHIFT: c_int = 2;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF5EN_SHIFT: c_int = 3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF6EN_SHIFT: c_int = 4;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF7EN_SHIFT: c_int = 5;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF8EN_SHIFT: c_int = 6;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF9EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF9EN_SHIFT: c_int = 7;
    pub flags9: u8,
pub const XSTORM_ISCSI_CONN_AG_CTX_CF10EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF10EN_SHIFT: c_int = 0;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF11EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF11EN_SHIFT: c_int = 1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF12EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF12EN_SHIFT: c_int = 2;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF13EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF13EN_SHIFT: c_int = 3;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF14EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF14EN_SHIFT: c_int = 4;
pub const XSTORM_ISCSI_CONN_AG_CTX_UPDATE_STATE_TO_BASE_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_UPDATE_STATE_TO_BASE_CF_EN_SHIFT: c_int = 5;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF16EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF16EN_SHIFT: c_int = 6;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF17EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF17EN_SHIFT: c_int = 7;
    pub flags10: u8,
pub const XSTORM_ISCSI_CONN_AG_CTX_CF18EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_CF18EN_SHIFT: c_int = 0;
pub const XSTORM_ISCSI_CONN_AG_CTX_DQ_FLUSH_EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_DQ_FLUSH_EN_SHIFT: c_int = 1;
pub const XSTORM_ISCSI_CONN_AG_CTX_MST_XCM_Q0_FLUSH_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_MST_XCM_Q0_FLUSH_CF_EN_SHIFT: c_int = 2;
pub const XSTORM_ISCSI_CONN_AG_CTX_UST_XCM_Q1_FLUSH_CF_EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_UST_XCM_Q1_FLUSH_CF_EN_SHIFT: c_int = 3;
pub const XSTORM_ISCSI_CONN_AG_CTX_SLOW_PATH_EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_SLOW_PATH_EN_SHIFT: c_int = 4;
pub const XSTORM_ISCSI_CONN_AG_CTX_PROC_ONLY_CLEANUP_EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_PROC_ONLY_CLEANUP_EN_SHIFT: c_int = 5;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 6;
pub const XSTORM_ISCSI_CONN_AG_CTX_MORE_TO_SEND_DEC_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_MORE_TO_SEND_DEC_RULE_EN_SHIFT: c_int = 7;
    pub flags11: u8,
pub const XSTORM_ISCSI_CONN_AG_CTX_TX_BLOCKED_EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_TX_BLOCKED_EN_SHIFT: c_int = 0;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 1;
pub const XSTORM_ISCSI_CONN_AG_CTX_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_RESERVED3_SHIFT: c_int = 2;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 3;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 4;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 5;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED1_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED1_SHIFT: c_int = 6;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE9EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE9EN_SHIFT: c_int = 7;
    pub flags12: u8,
pub const XSTORM_ISCSI_CONN_AG_CTX_SQ_DEC_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_SQ_DEC_RULE_EN_SHIFT: c_int = 0;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE11EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE11EN_SHIFT: c_int = 1;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED2_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED2_SHIFT: c_int = 2;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED3_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED3_SHIFT: c_int = 3;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE14EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE14EN_SHIFT: c_int = 4;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE15EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE15EN_SHIFT: c_int = 5;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE16EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE16EN_SHIFT: c_int = 6;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE17EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_RULE17EN_SHIFT: c_int = 7;
    pub flags13: u8,
pub const XSTORM_ISCSI_CONN_AG_CTX_R2TQ_DEC_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_R2TQ_DEC_RULE_EN_SHIFT: c_int = 0;
pub const XSTORM_ISCSI_CONN_AG_CTX_HQ_DEC_RULE_EN_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_HQ_DEC_RULE_EN_SHIFT: c_int = 1;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED4_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED4_SHIFT: c_int = 2;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED5_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED5_SHIFT: c_int = 3;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED6_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED6_SHIFT: c_int = 4;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED7_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED7_SHIFT: c_int = 5;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED8_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED8_SHIFT: c_int = 6;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED9_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_A0_RESERVED9_SHIFT: c_int = 7;
    pub flags14: u8,
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT16_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT16_SHIFT: c_int = 0;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT17_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT17_SHIFT: c_int = 1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT18_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT18_SHIFT: c_int = 2;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT19_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT19_SHIFT: c_int = 3;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT20_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_BIT20_SHIFT: c_int = 4;
pub const XSTORM_ISCSI_CONN_AG_CTX_DUMMY_READ_DONE_MASK: c_uint = 0x1;
pub const XSTORM_ISCSI_CONN_AG_CTX_DUMMY_READ_DONE_SHIFT: c_int = 5;
pub const XSTORM_ISCSI_CONN_AG_CTX_PROC_ONLY_CLEANUP_MASK: c_uint = 0x3;
pub const XSTORM_ISCSI_CONN_AG_CTX_PROC_ONLY_CLEANUP_SHIFT: c_int = 6;
    pub byte2: u8,
    pub physical_q0: __le16,
    pub physical_q1: __le16,
    pub dummy_dorq_var: __le16,
    pub sq_cons: __le16,
    pub sq_prod: __le16,
    pub word5: __le16,
    pub slow_io_total_data_tx_update: __le16,
    pub byte3: u8,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub more_to_send_seq: __le32,
    pub reg4: __le32,
    pub reg5: __le32,
    pub hq_scan_next_relevant_ack: __le32,
    pub r2tq_prod: __le16,
    pub r2tq_cons: __le16,
    pub hq_prod: __le16,
    pub hq_cons: __le16,
    pub remain_seq: __le32,
    pub bytes_to_next_pdu: __le32,
    pub hq_tcp_seq: __le32,
    pub byte7: u8,
    pub byte8: u8,
    pub byte9: u8,
    pub byte10: u8,
    pub byte11: u8,
    pub byte12: u8,
    pub byte13: u8,
    pub byte14: u8,
    pub byte15: u8,
    pub e5_reserved: u8,
    pub word11: __le16,
    pub reg10: __le32,
    pub reg11: __le32,
    pub exp_stat_sn: __le32,
    pub ongoing_fast_rxmit_seq: __le32,
    pub reg14: __le32,
    pub reg15: __le32,
    pub reg16: __le32,
    pub reg17: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_iscsi_conn_ag_ctx {
    pub reserved0: u8,
    pub state: u8,
    pub flags0: u8,
pub const TSTORM_ISCSI_CONN_AG_CTX_EXIST_IN_QM0_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_EXIST_IN_QM0_SHIFT: c_int = 0;
pub const TSTORM_ISCSI_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const TSTORM_ISCSI_CONN_AG_CTX_BIT2_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_BIT2_SHIFT: c_int = 2;
pub const TSTORM_ISCSI_CONN_AG_CTX_BIT3_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_BIT3_SHIFT: c_int = 3;
pub const TSTORM_ISCSI_CONN_AG_CTX_BIT4_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_BIT4_SHIFT: c_int = 4;
pub const TSTORM_ISCSI_CONN_AG_CTX_BIT5_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_BIT5_SHIFT: c_int = 5;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF0_SHIFT: c_int = 6;
    pub flags1: u8,
pub const TSTORM_ISCSI_CONN_AG_CTX_P2T_FLUSH_CF_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_CONN_AG_CTX_P2T_FLUSH_CF_SHIFT: c_int = 0;
pub const TSTORM_ISCSI_CONN_AG_CTX_M2T_FLUSH_CF_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_CONN_AG_CTX_M2T_FLUSH_CF_SHIFT: c_int = 2;
pub const TSTORM_ISCSI_CONN_AG_CTX_TIMER_STOP_ALL_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_CONN_AG_CTX_TIMER_STOP_ALL_SHIFT: c_int = 4;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF4_SHIFT: c_int = 6;
    pub flags2: u8,
pub const TSTORM_ISCSI_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF5_SHIFT: c_int = 0;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF6_SHIFT: c_int = 2;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF7_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF7_SHIFT: c_int = 4;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF8_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF8_SHIFT: c_int = 6;
    pub flags3: u8,
pub const TSTORM_ISCSI_CONN_AG_CTX_FLUSH_Q0_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_CONN_AG_CTX_FLUSH_Q0_SHIFT: c_int = 0;
pub const TSTORM_ISCSI_CONN_AG_CTX_FLUSH_OOO_ISLES_CF_MASK: c_uint = 0x3;
pub const TSTORM_ISCSI_CONN_AG_CTX_FLUSH_OOO_ISLES_CF_SHIFT: c_int = 2;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF0EN_SHIFT: c_int = 4;
pub const TSTORM_ISCSI_CONN_AG_CTX_P2T_FLUSH_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_P2T_FLUSH_CF_EN_SHIFT: c_int = 5;
pub const TSTORM_ISCSI_CONN_AG_CTX_M2T_FLUSH_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_M2T_FLUSH_CF_EN_SHIFT: c_int = 6;
pub const TSTORM_ISCSI_CONN_AG_CTX_TIMER_STOP_ALL_EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_TIMER_STOP_ALL_EN_SHIFT: c_int = 7;
    pub flags4: u8,
pub const TSTORM_ISCSI_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF4EN_SHIFT: c_int = 0;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF5EN_SHIFT: c_int = 1;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF6EN_SHIFT: c_int = 2;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF7EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF7EN_SHIFT: c_int = 3;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF8EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_CF8EN_SHIFT: c_int = 4;
pub const TSTORM_ISCSI_CONN_AG_CTX_FLUSH_Q0_EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_FLUSH_Q0_EN_SHIFT: c_int = 5;
pub const TSTORM_ISCSI_CONN_AG_CTX_FLUSH_OOO_ISLES_CF_EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_FLUSH_OOO_ISLES_CF_EN_SHIFT: c_int = 6;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 7;
    pub flags5: u8,
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 0;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 5;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const TSTORM_ISCSI_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub reg0: __le32,
    pub reg1: __le32,
    pub rx_tcp_checksum_err_cnt: __le32,
    pub reg3: __le32,
    pub reg4: __le32,
    pub reg5: __le32,
    pub reg6: __le32,
    pub reg7: __le32,
    pub reg8: __le32,
    pub cid_offload_cnt: u8,
    pub byte3: u8,
    pub word0: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_iscsi_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const USTORM_ISCSI_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const USTORM_ISCSI_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const USTORM_ISCSI_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const USTORM_ISCSI_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const USTORM_ISCSI_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const USTORM_ISCSI_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const USTORM_ISCSI_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const USTORM_ISCSI_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const USTORM_ISCSI_CONN_AG_CTX_CF3_MASK: c_uint = 0x3;
pub const USTORM_ISCSI_CONN_AG_CTX_CF3_SHIFT: c_int = 0;
pub const USTORM_ISCSI_CONN_AG_CTX_CF4_MASK: c_uint = 0x3;
pub const USTORM_ISCSI_CONN_AG_CTX_CF4_SHIFT: c_int = 2;
pub const USTORM_ISCSI_CONN_AG_CTX_CF5_MASK: c_uint = 0x3;
pub const USTORM_ISCSI_CONN_AG_CTX_CF5_SHIFT: c_int = 4;
pub const USTORM_ISCSI_CONN_AG_CTX_CF6_MASK: c_uint = 0x3;
pub const USTORM_ISCSI_CONN_AG_CTX_CF6_SHIFT: c_int = 6;
    pub flags2: u8,
pub const USTORM_ISCSI_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const USTORM_ISCSI_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const USTORM_ISCSI_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const USTORM_ISCSI_CONN_AG_CTX_CF3EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_CF3EN_SHIFT: c_int = 3;
pub const USTORM_ISCSI_CONN_AG_CTX_CF4EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_CF4EN_SHIFT: c_int = 4;
pub const USTORM_ISCSI_CONN_AG_CTX_CF5EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_CF5EN_SHIFT: c_int = 5;
pub const USTORM_ISCSI_CONN_AG_CTX_CF6EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_CF6EN_SHIFT: c_int = 6;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 7;
    pub flags3: u8,
pub const USTORM_ISCSI_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 0;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 1;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 2;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 3;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE5EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE5EN_SHIFT: c_int = 4;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE6EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE6EN_SHIFT: c_int = 5;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE7EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE7EN_SHIFT: c_int = 6;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE8EN_MASK: c_uint = 0x1;
pub const USTORM_ISCSI_CONN_AG_CTX_RULE8EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub word1: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
    pub reg2: __le32,
    pub reg3: __le32,
    pub word2: __le16,
    pub word3: __le16,
}

// The iscsi storm connection context of Tstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstorm_iscsi_conn_st_ctx {
    pub reserved: [__le32; 44],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_iscsi_conn_ag_ctx {
    pub reserved: u8,
    pub state: u8,
    pub flags0: u8,
pub const MSTORM_ISCSI_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const MSTORM_ISCSI_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const MSTORM_ISCSI_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const MSTORM_ISCSI_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const MSTORM_ISCSI_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const MSTORM_ISCSI_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const MSTORM_ISCSI_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const MSTORM_ISCSI_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const MSTORM_ISCSI_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const MSTORM_ISCSI_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const MSTORM_ISCSI_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const MSTORM_ISCSI_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const MSTORM_ISCSI_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const MSTORM_ISCSI_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const MSTORM_ISCSI_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const MSTORM_ISCSI_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const MSTORM_ISCSI_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub word0: __le16,
    pub word1: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
}

// Combined iSCSI and TCP storm connection of Mstorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_iscsi_tcp_conn_st_ctx {
    pub reserved_tcp: [__le32; 20],
    pub reserved_iscsi: [__le32; 12],
}

// The iscsi storm context of Ustorm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustorm_iscsi_conn_st_ctx {
    pub reserved: [__le32; 52],
}

// iscsi connection context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_conn_context {
    pub ystorm_st_context: ystorm_iscsi_conn_st_ctx,
    pub pstorm_st_context: pstorm_iscsi_tcp_conn_st_ctx,
    pub pstorm_st_padding: [regpair; 2],
    pub xpb2_context: pb_context,
    pub xstorm_st_context: xstorm_iscsi_tcp_conn_st_ctx,
    pub xstorm_st_padding: [regpair; 2],
    pub xstorm_ag_context: xstorm_iscsi_conn_ag_ctx,
    pub tstorm_ag_context: tstorm_iscsi_conn_ag_ctx,
    pub tstorm_ag_padding: [regpair; 2],
    pub timer_context: timers_context,
    pub ustorm_ag_context: ustorm_iscsi_conn_ag_ctx,
    pub upb_context: pb_context,
    pub tstorm_st_context: tstorm_iscsi_conn_st_ctx,
    pub tstorm_st_padding: [regpair; 2],
    pub mstorm_ag_context: mstorm_iscsi_conn_ag_ctx,
    pub mstorm_st_context: mstorm_iscsi_tcp_conn_st_ctx,
    pub ustorm_st_context: ustorm_iscsi_conn_st_ctx,
}

// iSCSI init params passed by driver to FW in iSCSI init ramrod
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_init_ramrod_params {
    pub iscsi_init_spe: iscsi_spe_func_init,
    pub tcp_init: tcp_init_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystorm_iscsi_conn_ag_ctx {
    pub byte0: u8,
    pub byte1: u8,
    pub flags0: u8,
pub const YSTORM_ISCSI_CONN_AG_CTX_BIT0_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_CONN_AG_CTX_BIT0_SHIFT: c_int = 0;
pub const YSTORM_ISCSI_CONN_AG_CTX_BIT1_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_CONN_AG_CTX_BIT1_SHIFT: c_int = 1;
pub const YSTORM_ISCSI_CONN_AG_CTX_CF0_MASK: c_uint = 0x3;
pub const YSTORM_ISCSI_CONN_AG_CTX_CF0_SHIFT: c_int = 2;
pub const YSTORM_ISCSI_CONN_AG_CTX_CF1_MASK: c_uint = 0x3;
pub const YSTORM_ISCSI_CONN_AG_CTX_CF1_SHIFT: c_int = 4;
pub const YSTORM_ISCSI_CONN_AG_CTX_CF2_MASK: c_uint = 0x3;
pub const YSTORM_ISCSI_CONN_AG_CTX_CF2_SHIFT: c_int = 6;
    pub flags1: u8,
pub const YSTORM_ISCSI_CONN_AG_CTX_CF0EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_CONN_AG_CTX_CF0EN_SHIFT: c_int = 0;
pub const YSTORM_ISCSI_CONN_AG_CTX_CF1EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_CONN_AG_CTX_CF1EN_SHIFT: c_int = 1;
pub const YSTORM_ISCSI_CONN_AG_CTX_CF2EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_CONN_AG_CTX_CF2EN_SHIFT: c_int = 2;
pub const YSTORM_ISCSI_CONN_AG_CTX_RULE0EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_CONN_AG_CTX_RULE0EN_SHIFT: c_int = 3;
pub const YSTORM_ISCSI_CONN_AG_CTX_RULE1EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_CONN_AG_CTX_RULE1EN_SHIFT: c_int = 4;
pub const YSTORM_ISCSI_CONN_AG_CTX_RULE2EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_CONN_AG_CTX_RULE2EN_SHIFT: c_int = 5;
pub const YSTORM_ISCSI_CONN_AG_CTX_RULE3EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_CONN_AG_CTX_RULE3EN_SHIFT: c_int = 6;
pub const YSTORM_ISCSI_CONN_AG_CTX_RULE4EN_MASK: c_uint = 0x1;
pub const YSTORM_ISCSI_CONN_AG_CTX_RULE4EN_SHIFT: c_int = 7;
    pub byte2: u8,
    pub byte3: u8,
    pub word0: __le16,
    pub reg0: __le32,
    pub reg1: __le32,
    pub word1: __le16,
    pub word2: __le16,
    pub word3: __le16,
    pub word4: __le16,
    pub reg2: __le32,
    pub reg3: __le32,
}
