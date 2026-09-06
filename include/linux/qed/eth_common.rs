//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/eth_common.h
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
// Copyright (c) 2019-2020 Marvell International Ltd.
//
// ETH FW CONSTANTS
//
pub const ETH_HSI_VER_MAJOR: c_int = 3;
pub const ETH_HSI_VER_MINOR: c_int = 11;
pub const ETH_HSI_VER_NO_PKT_LEN_TUNN: c_int = 5;
// Maximum number of pinned L2 connections (CIDs)
pub const ETH_PINNED_CONN_MAX_NUM: c_int = 32;
pub const ETH_CACHE_LINE_SIZE: c_int = 64;
pub const ETH_RX_CQE_GAP: c_int = 32;
pub const ETH_MAX_RAMROD_PER_CON: c_int = 8;
pub const ETH_TX_BD_PAGE_SIZE_BYTES: c_int = 4096;
pub const ETH_RX_BD_PAGE_SIZE_BYTES: c_int = 4096;
pub const ETH_RX_CQE_PAGE_SIZE_BYTES: c_int = 4096;
pub const ETH_RX_NUM_NEXT_PAGE_BDS: c_int = 2;
pub const ETH_MAX_TUNN_LSO_INNER_IPV4_OFFSET: c_int = 253;
pub const ETH_MAX_TUNN_LSO_INNER_IPV6_OFFSET: c_int = 251;
pub const ETH_TX_MIN_BDS_PER_NON_LSO_PKT: c_int = 1;
pub const ETH_TX_MAX_BDS_PER_NON_LSO_PACKET: c_int = 18;
pub const ETH_TX_MAX_BDS_PER_LSO_PACKET: c_int = 255;
pub const ETH_TX_MAX_LSO_HDR_NBD: c_int = 4;
pub const ETH_TX_MIN_BDS_PER_LSO_PKT: c_int = 3;
pub const ETH_TX_MIN_BDS_PER_TUNN_IPV6_WITH_EXT_PKT: c_int = 3;
pub const ETH_TX_MIN_BDS_PER_IPV6_WITH_EXT_PKT: c_int = 2;
pub const ETH_TX_MIN_BDS_PER_PKT_W_LOOPBACK_MODE: c_int = 2;
pub const ETH_TX_MIN_BDS_PER_PKT_W_VPORT_FORWARDING: c_int = 4;

pub const ETH_TX_MAX_LSO_HDR_BYTES: c_int = 510;

pub const ETH_TX_LSO_WINDOW_MIN_LEN: c_int = 9700;
pub const ETH_TX_MAX_LSO_PAYLOAD_LEN: c_uint = 0xFE000;
pub const ETH_TX_NUM_SAME_AS_LAST_ENTRIES: c_int = 320;
pub const ETH_TX_INACTIVE_SAME_AS_LAST: c_uint = 0xFFFF;

pub const ETH_RX_MAX_BUFF_PER_PKT: c_int = 5;
pub const ETH_RX_BD_THRESHOLD: c_int = 16;
// Num of MAC/VLAN filters
pub const ETH_NUM_MAC_FILTERS: c_int = 512;
pub const ETH_NUM_VLAN_FILTERS: c_int = 512;
// Approx. multicast constants
pub const ETH_MULTICAST_BIN_FROM_MAC_SEED: c_int = 0;
pub const ETH_MULTICAST_MAC_BINS: c_int = 256;

// Ethernet vport update constants
pub const ETH_FILTER_RULES_COUNT: c_int = 10;
pub const ETH_RSS_IND_TABLE_ENTRIES_NUM: c_int = 128;

pub const ETH_RSS_KEY_SIZE_REGS: c_int = 10;
pub const ETH_RSS_ENGINE_NUM_K2: c_int = 207;
pub const ETH_RSS_ENGINE_NUM_BB: c_int = 127;
// TPA constants
pub const ETH_TPA_MAX_AGGS_NUM: c_int = 64;
pub const ETH_TPA_CQE_START_BW_LEN_LIST_SIZE: c_int = 2;
pub const ETH_TPA_CQE_CONT_LEN_LIST_SIZE: c_int = 6;
pub const ETH_TPA_CQE_END_LEN_LIST_SIZE: c_int = 4;
// Control frame check constants
pub const ETH_CTL_FRAME_ETH_TYPE_NUM: c_int = 4;
// GFS constants
pub const ETH_GFT_TRASHCAN_VPORT: c_uint = 0x1FF	/* GFT drop flow vport number */;
// Destination port mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dst_port_mode {
    DST_PORT_PHY,
    DST_PORT_LOOPBACK,
    DST_PORT_PHY_LOOPBACK,
    DST_PORT_DROP,
    MAX_DST_PORT_MODE
}

// Ethernet address type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_addr_type {
    BROADCAST_ADDRESS,
    MULTICAST_ADDRESS,
    UNICAST_ADDRESS,
    UNKNOWN_ADDRESS,
    MAX_ETH_ADDR_TYPE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_1st_bd_flags {
    pub bitfields: u8,
pub const ETH_TX_1ST_BD_FLAGS_START_BD_MASK: c_uint = 0x1;
pub const ETH_TX_1ST_BD_FLAGS_START_BD_SHIFT: c_int = 0;
pub const ETH_TX_1ST_BD_FLAGS_FORCE_VLAN_MODE_MASK: c_uint = 0x1;
pub const ETH_TX_1ST_BD_FLAGS_FORCE_VLAN_MODE_SHIFT: c_int = 1;
pub const ETH_TX_1ST_BD_FLAGS_IP_CSUM_MASK: c_uint = 0x1;
pub const ETH_TX_1ST_BD_FLAGS_IP_CSUM_SHIFT: c_int = 2;
pub const ETH_TX_1ST_BD_FLAGS_L4_CSUM_MASK: c_uint = 0x1;
pub const ETH_TX_1ST_BD_FLAGS_L4_CSUM_SHIFT: c_int = 3;
pub const ETH_TX_1ST_BD_FLAGS_VLAN_INSERTION_MASK: c_uint = 0x1;
pub const ETH_TX_1ST_BD_FLAGS_VLAN_INSERTION_SHIFT: c_int = 4;
pub const ETH_TX_1ST_BD_FLAGS_LSO_MASK: c_uint = 0x1;
pub const ETH_TX_1ST_BD_FLAGS_LSO_SHIFT: c_int = 5;
pub const ETH_TX_1ST_BD_FLAGS_TUNN_IP_CSUM_MASK: c_uint = 0x1;
pub const ETH_TX_1ST_BD_FLAGS_TUNN_IP_CSUM_SHIFT: c_int = 6;
pub const ETH_TX_1ST_BD_FLAGS_TUNN_L4_CSUM_MASK: c_uint = 0x1;
pub const ETH_TX_1ST_BD_FLAGS_TUNN_L4_CSUM_SHIFT: c_int = 7;
}

// The parsing information data fo rthe first tx bd of a given packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_data_1st_bd {
    pub vlan: __le16,
    pub nbds: u8,
    pub bd_flags: eth_tx_1st_bd_flags,
    pub bitfields: __le16,
pub const ETH_TX_DATA_1ST_BD_TUNN_FLAG_MASK: c_uint = 0x1;
pub const ETH_TX_DATA_1ST_BD_TUNN_FLAG_SHIFT: c_int = 0;
pub const ETH_TX_DATA_1ST_BD_RESERVED0_MASK: c_uint = 0x1;
pub const ETH_TX_DATA_1ST_BD_RESERVED0_SHIFT: c_int = 1;
pub const ETH_TX_DATA_1ST_BD_PKT_LEN_MASK: c_uint = 0x3FFF;
pub const ETH_TX_DATA_1ST_BD_PKT_LEN_SHIFT: c_int = 2;
}

// The parsing information data for the second tx bd of a given packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_data_2nd_bd {
    pub tunn_ip_size: __le16,
    pub bitfields1: __le16,
pub const ETH_TX_DATA_2ND_BD_TUNN_INNER_L2_HDR_SIZE_W_MASK: c_uint = 0xF;
pub const ETH_TX_DATA_2ND_BD_TUNN_INNER_L2_HDR_SIZE_W_SHIFT: c_int = 0;
pub const ETH_TX_DATA_2ND_BD_TUNN_INNER_ETH_TYPE_MASK: c_uint = 0x3;
pub const ETH_TX_DATA_2ND_BD_TUNN_INNER_ETH_TYPE_SHIFT: c_int = 4;
pub const ETH_TX_DATA_2ND_BD_DST_PORT_MODE_MASK: c_uint = 0x3;
pub const ETH_TX_DATA_2ND_BD_DST_PORT_MODE_SHIFT: c_int = 6;
pub const ETH_TX_DATA_2ND_BD_START_BD_MASK: c_uint = 0x1;
pub const ETH_TX_DATA_2ND_BD_START_BD_SHIFT: c_int = 8;
pub const ETH_TX_DATA_2ND_BD_TUNN_TYPE_MASK: c_uint = 0x3;
pub const ETH_TX_DATA_2ND_BD_TUNN_TYPE_SHIFT: c_int = 9;
pub const ETH_TX_DATA_2ND_BD_TUNN_INNER_IPV6_MASK: c_uint = 0x1;
pub const ETH_TX_DATA_2ND_BD_TUNN_INNER_IPV6_SHIFT: c_int = 11;
pub const ETH_TX_DATA_2ND_BD_IPV6_EXT_MASK: c_uint = 0x1;
pub const ETH_TX_DATA_2ND_BD_IPV6_EXT_SHIFT: c_int = 12;
pub const ETH_TX_DATA_2ND_BD_TUNN_IPV6_EXT_MASK: c_uint = 0x1;
pub const ETH_TX_DATA_2ND_BD_TUNN_IPV6_EXT_SHIFT: c_int = 13;
pub const ETH_TX_DATA_2ND_BD_L4_UDP_MASK: c_uint = 0x1;
pub const ETH_TX_DATA_2ND_BD_L4_UDP_SHIFT: c_int = 14;
pub const ETH_TX_DATA_2ND_BD_L4_PSEUDO_CSUM_MODE_MASK: c_uint = 0x1;
pub const ETH_TX_DATA_2ND_BD_L4_PSEUDO_CSUM_MODE_SHIFT: c_int = 15;
    pub bitfields2: __le16,
pub const ETH_TX_DATA_2ND_BD_L4_HDR_START_OFFSET_W_MASK: c_uint = 0x1FFF;
pub const ETH_TX_DATA_2ND_BD_L4_HDR_START_OFFSET_W_SHIFT: c_int = 0;
pub const ETH_TX_DATA_2ND_BD_RESERVED0_MASK: c_uint = 0x7;
pub const ETH_TX_DATA_2ND_BD_RESERVED0_SHIFT: c_int = 13;
}

// Firmware data for L2-EDPM packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_edpm_fw_data {
    pub data_1st_bd: eth_tx_data_1st_bd,
    pub data_2nd_bd: eth_tx_data_2nd_bd,
    pub reserved: __le32,
}

// Tunneling parsing flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tunnel_parsing_flags {
    pub flags: u8,
pub const ETH_TUNNEL_PARSING_FLAGS_TYPE_MASK: c_uint = 0x3;
pub const ETH_TUNNEL_PARSING_FLAGS_TYPE_SHIFT: c_int = 0;
pub const ETH_TUNNEL_PARSING_FLAGS_TENNANT_ID_EXIST_MASK: c_uint = 0x1;
pub const ETH_TUNNEL_PARSING_FLAGS_TENNANT_ID_EXIST_SHIFT: c_int = 2;
pub const ETH_TUNNEL_PARSING_FLAGS_NEXT_PROTOCOL_MASK: c_uint = 0x3;
pub const ETH_TUNNEL_PARSING_FLAGS_NEXT_PROTOCOL_SHIFT: c_int = 3;
pub const ETH_TUNNEL_PARSING_FLAGS_FIRSTHDRIPMATCH_MASK: c_uint = 0x1;
pub const ETH_TUNNEL_PARSING_FLAGS_FIRSTHDRIPMATCH_SHIFT: c_int = 5;
pub const ETH_TUNNEL_PARSING_FLAGS_IPV4_FRAGMENT_MASK: c_uint = 0x1;
pub const ETH_TUNNEL_PARSING_FLAGS_IPV4_FRAGMENT_SHIFT: c_int = 6;
pub const ETH_TUNNEL_PARSING_FLAGS_IPV4_OPTIONS_MASK: c_uint = 0x1;
pub const ETH_TUNNEL_PARSING_FLAGS_IPV4_OPTIONS_SHIFT: c_int = 7;
}

// PMD flow control bits
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_pmd_flow_flags {
    pub flags: u8,
pub const ETH_PMD_FLOW_FLAGS_VALID_MASK: c_uint = 0x1;
pub const ETH_PMD_FLOW_FLAGS_VALID_SHIFT: c_int = 0;
pub const ETH_PMD_FLOW_FLAGS_TOGGLE_MASK: c_uint = 0x1;
pub const ETH_PMD_FLOW_FLAGS_TOGGLE_SHIFT: c_int = 1;
pub const ETH_PMD_FLOW_FLAGS_RESERVED_MASK: c_uint = 0x3F;
pub const ETH_PMD_FLOW_FLAGS_RESERVED_SHIFT: c_int = 2;
}

// Regular ETH Rx FP CQE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_fast_path_rx_reg_cqe {
    pub type: u8,
    pub bitfields: u8,
pub const ETH_FAST_PATH_RX_REG_CQE_RSS_HASH_TYPE_MASK: c_uint = 0x7;
pub const ETH_FAST_PATH_RX_REG_CQE_RSS_HASH_TYPE_SHIFT: c_int = 0;
pub const ETH_FAST_PATH_RX_REG_CQE_TC_MASK: c_uint = 0xF;
pub const ETH_FAST_PATH_RX_REG_CQE_TC_SHIFT: c_int = 3;
pub const ETH_FAST_PATH_RX_REG_CQE_RESERVED0_MASK: c_uint = 0x1;
pub const ETH_FAST_PATH_RX_REG_CQE_RESERVED0_SHIFT: c_int = 7;
    pub pkt_len: __le16,
    pub pars_flags: parsing_and_err_flags,
    pub vlan_tag: __le16,
    pub rss_hash: __le32,
    pub len_on_first_bd: __le16,
    pub placement_offset: u8,
    pub tunnel_pars_flags: eth_tunnel_parsing_flags,
    pub bd_num: u8,
    pub reserved: u8,
    pub reserved2: __le16,
    pub flow_id_or_resource_id: __le32,
    pub reserved1: [u8; 7],
    pub pmd_flags: eth_pmd_flow_flags,
}

// TPA-continue ETH Rx FP CQE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_fast_path_rx_tpa_cont_cqe {
    pub type: u8,
    pub tpa_agg_index: u8,
    pub len_list: [__le16; ETH_TPA_CQE_CONT_LEN_LIST_SIZE],
    pub reserved: u8,
    pub reserved1: u8,
    pub reserved2: [__le16; ETH_TPA_CQE_CONT_LEN_LIST_SIZE],
    pub reserved3: [u8; 3],
    pub pmd_flags: eth_pmd_flow_flags,
}

// TPA-end ETH Rx FP CQE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_fast_path_rx_tpa_end_cqe {
    pub type: u8,
    pub tpa_agg_index: u8,
    pub total_packet_len: __le16,
    pub num_of_bds: u8,
    pub end_reason: u8,
    pub num_of_coalesced_segs: __le16,
    pub ts_delta: __le32,
    pub len_list: [__le16; ETH_TPA_CQE_END_LEN_LIST_SIZE],
    pub reserved3: [__le16; ETH_TPA_CQE_END_LEN_LIST_SIZE],
    pub reserved1: __le16,
    pub reserved2: u8,
    pub pmd_flags: eth_pmd_flow_flags,
}

// TPA-start ETH Rx FP CQE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_fast_path_rx_tpa_start_cqe {
    pub type: u8,
    pub bitfields: u8,
pub const ETH_FAST_PATH_RX_TPA_START_CQE_RSS_HASH_TYPE_MASK: c_uint = 0x7;
pub const ETH_FAST_PATH_RX_TPA_START_CQE_RSS_HASH_TYPE_SHIFT: c_int = 0;
pub const ETH_FAST_PATH_RX_TPA_START_CQE_TC_MASK: c_uint = 0xF;
pub const ETH_FAST_PATH_RX_TPA_START_CQE_TC_SHIFT: c_int = 3;
pub const ETH_FAST_PATH_RX_TPA_START_CQE_RESERVED0_MASK: c_uint = 0x1;
pub const ETH_FAST_PATH_RX_TPA_START_CQE_RESERVED0_SHIFT: c_int = 7;
    pub seg_len: __le16,
    pub pars_flags: parsing_and_err_flags,
    pub vlan_tag: __le16,
    pub rss_hash: __le32,
    pub len_on_first_bd: __le16,
    pub placement_offset: u8,
    pub tunnel_pars_flags: eth_tunnel_parsing_flags,
    pub tpa_agg_index: u8,
    pub header_len: u8,
    pub bw_ext_bd_len_list: [__le16; ETH_TPA_CQE_START_BW_LEN_LIST_SIZE],
    pub reserved2: __le16,
    pub flow_id_or_resource_id: __le32,
    pub reserved: [u8; 3],
    pub pmd_flags: eth_pmd_flow_flags,
}

// The L4 pseudo checksum mode for Ethernet
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_l4_pseudo_checksum_mode {
    ETH_L4_PSEUDO_CSUM_CORRECT_LENGTH,
    ETH_L4_PSEUDO_CSUM_ZERO_LENGTH,
    MAX_ETH_L4_PSEUDO_CHECKSUM_MODE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_rx_bd {
    pub addr: regpair,
}

// Regular ETH Rx SP CQE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_slow_path_rx_cqe {
    pub type: u8,
    pub ramrod_cmd_id: u8,
    pub error_flag: u8,
    pub reserved: [u8; 25],
    pub echo: __le16,
    pub reserved1: u8,
    pub pmd_flags: eth_pmd_flow_flags,
}

// Union for all ETH Rx CQE types
#[repr(C)]
#[derive(Copy, Clone)]
pub union eth_rx_cqe {
    pub fast_path_regular: eth_fast_path_rx_reg_cqe,
    pub fast_path_tpa_start: eth_fast_path_rx_tpa_start_cqe,
    pub fast_path_tpa_cont: eth_fast_path_rx_tpa_cont_cqe,
    pub fast_path_tpa_end: eth_fast_path_rx_tpa_end_cqe,
    pub slow_path: eth_slow_path_rx_cqe,
}

// ETH Rx CQE type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_rx_cqe_type {
    ETH_RX_CQE_TYPE_UNUSED,
    ETH_RX_CQE_TYPE_REGULAR,
    ETH_RX_CQE_TYPE_SLOW_PATH,
    ETH_RX_CQE_TYPE_TPA_START,
    ETH_RX_CQE_TYPE_TPA_CONT,
    ETH_RX_CQE_TYPE_TPA_END,
    MAX_ETH_RX_CQE_TYPE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_rx_pmd_cqe {
    pub cqe: eth_rx_cqe,
    pub reserved: [u8; ETH_RX_CQE_GAP],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_rx_tunn_type {
    ETH_RX_NO_TUNN,
    ETH_RX_TUNN_GENEVE,
    ETH_RX_TUNN_GRE,
    ETH_RX_TUNN_VXLAN,
    MAX_ETH_RX_TUNN_TYPE
}

// Aggregation end reason.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_tpa_end_reason {
    ETH_AGG_END_UNUSED,
    ETH_AGG_END_SP_UPDATE,
    ETH_AGG_END_MAX_LEN,
    ETH_AGG_END_LAST_SEG,
    ETH_AGG_END_TIMEOUT,
    ETH_AGG_END_NOT_CONSISTENT,
    ETH_AGG_END_OUT_OF_ORDER,
    ETH_AGG_END_NON_TPA_SEG,
    MAX_ETH_TPA_END_REASON
}

// The first tx bd of a given packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_1st_bd {
    pub addr: regpair,
    pub nbytes: __le16,
    pub data: eth_tx_data_1st_bd,
}

// The second tx bd of a given packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_2nd_bd {
    pub addr: regpair,
    pub nbytes: __le16,
    pub data: eth_tx_data_2nd_bd,
}

// The parsing information data for the third tx bd of a given packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_data_3rd_bd {
    pub lso_mss: __le16,
    pub bitfields: __le16,
pub const ETH_TX_DATA_3RD_BD_TCP_HDR_LEN_DW_MASK: c_uint = 0xF;
pub const ETH_TX_DATA_3RD_BD_TCP_HDR_LEN_DW_SHIFT: c_int = 0;
pub const ETH_TX_DATA_3RD_BD_HDR_NBD_MASK: c_uint = 0xF;
pub const ETH_TX_DATA_3RD_BD_HDR_NBD_SHIFT: c_int = 4;
pub const ETH_TX_DATA_3RD_BD_START_BD_MASK: c_uint = 0x1;
pub const ETH_TX_DATA_3RD_BD_START_BD_SHIFT: c_int = 8;
pub const ETH_TX_DATA_3RD_BD_RESERVED0_MASK: c_uint = 0x7F;
pub const ETH_TX_DATA_3RD_BD_RESERVED0_SHIFT: c_int = 9;
    pub tunn_l4_hdr_start_offset_w: u8,
    pub tunn_hdr_size_w: u8,
}

// The third tx bd of a given packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_3rd_bd {
    pub addr: regpair,
    pub nbytes: __le16,
    pub data: eth_tx_data_3rd_bd,
}

// The parsing information data for the forth tx bd of a given packet.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_data_4th_bd {
    pub dst_vport_id: u8,
    pub reserved4: u8,
    pub bitfields: __le16,
pub const ETH_TX_DATA_4TH_BD_DST_VPORT_ID_VALID_MASK: c_uint = 0x1;
pub const ETH_TX_DATA_4TH_BD_DST_VPORT_ID_VALID_SHIFT: c_int = 0;
pub const ETH_TX_DATA_4TH_BD_RESERVED1_MASK: c_uint = 0x7F;
pub const ETH_TX_DATA_4TH_BD_RESERVED1_SHIFT: c_int = 1;
pub const ETH_TX_DATA_4TH_BD_START_BD_MASK: c_uint = 0x1;
pub const ETH_TX_DATA_4TH_BD_START_BD_SHIFT: c_int = 8;
pub const ETH_TX_DATA_4TH_BD_RESERVED2_MASK: c_uint = 0x7F;
pub const ETH_TX_DATA_4TH_BD_RESERVED2_SHIFT: c_int = 9;
    pub reserved3: __le16,
}

// The forth tx bd of a given packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_4th_bd {
    pub /: *mut *mut regpair addr; / Single continuous buffer,
    pub /: *mut *mut __le16 nbytes; / Number of bytes in this BD,
    pub /: *mut *mut eth_tx_data_4th_bd data; / Parsing information data,
}

// Complementary information for the regular tx bd of a given packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_data_bd {
    pub reserved0: __le16,
    pub bitfields: __le16,
pub const ETH_TX_DATA_BD_RESERVED1_MASK: c_uint = 0xFF;
pub const ETH_TX_DATA_BD_RESERVED1_SHIFT: c_int = 0;
pub const ETH_TX_DATA_BD_START_BD_MASK: c_uint = 0x1;
pub const ETH_TX_DATA_BD_START_BD_SHIFT: c_int = 8;
pub const ETH_TX_DATA_BD_RESERVED2_MASK: c_uint = 0x7F;
pub const ETH_TX_DATA_BD_RESERVED2_SHIFT: c_int = 9;
    pub reserved3: __le16,
}

// The common non-special TX BD ring element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_tx_bd {
    pub addr: regpair,
    pub nbytes: __le16,
    pub data: eth_tx_data_bd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union eth_tx_bd_types {
    pub first_bd: eth_tx_1st_bd,
    pub second_bd: eth_tx_2nd_bd,
    pub third_bd: eth_tx_3rd_bd,
    pub fourth_bd: eth_tx_4th_bd,
    pub reg_bd: eth_tx_bd,
}

// Mstorm Queue Zone
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eth_tx_tunn_type {
    ETH_TX_TUNN_GENEVE,
    ETH_TX_TUNN_TTAG,
    ETH_TX_TUNN_GRE,
    ETH_TX_TUNN_VXLAN,
    MAX_ETH_TX_TUNN_TYPE
}

// Mstorm Queue Zone
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mstorm_eth_queue_zone {
    pub rx_producers: eth_rx_prod_data,
    pub reserved: [__le32; 3],
}

// Ystorm Queue Zone
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstorm_eth_queue_zone {
    pub int_coalescing_timeset: coalescing_timeset,
    pub reserved: [u8; 7],
}

// ETH doorbell data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_db_data {
    pub params: u8,
pub const ETH_DB_DATA_DEST_MASK: c_uint = 0x3;
pub const ETH_DB_DATA_DEST_SHIFT: c_int = 0;
pub const ETH_DB_DATA_AGG_CMD_MASK: c_uint = 0x3;
pub const ETH_DB_DATA_AGG_CMD_SHIFT: c_int = 2;
pub const ETH_DB_DATA_BYPASS_EN_MASK: c_uint = 0x1;
pub const ETH_DB_DATA_BYPASS_EN_SHIFT: c_int = 4;
pub const ETH_DB_DATA_RESERVED_MASK: c_uint = 0x1;
pub const ETH_DB_DATA_RESERVED_SHIFT: c_int = 5;
pub const ETH_DB_DATA_AGG_VAL_SEL_MASK: c_uint = 0x3;
pub const ETH_DB_DATA_AGG_VAL_SEL_SHIFT: c_int = 6;
    pub agg_flags: u8,
    pub bd_prod: __le16,
}

// RSS hash type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rss_hash_type {
    RSS_HASH_TYPE_DEFAULT = 0,
    RSS_HASH_TYPE_IPV4 = 1,
    RSS_HASH_TYPE_TCP_IPV4 = 2,
    RSS_HASH_TYPE_IPV6 = 3,
    RSS_HASH_TYPE_TCP_IPV6 = 4,
    RSS_HASH_TYPE_UDP_IPV4 = 5,
    RSS_HASH_TYPE_UDP_IPV6 = 6,
    MAX_RSS_HASH_TYPE
}
