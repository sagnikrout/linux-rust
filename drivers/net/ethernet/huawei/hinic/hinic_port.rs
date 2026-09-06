//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_port.h
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
// Huawei HiNIC PCI Express Linux driver
// Copyright(c) 2017 Huawei Technologies Co., Ltd
//

pub const HINIC_RSS_KEY_SIZE: c_int = 40;
pub const HINIC_RSS_INDIR_SIZE: c_int = 256;
pub const HINIC_PORT_STATS_VERSION: c_int = 0;
pub const HINIC_FW_VERSION_NAME: c_int = 16;
pub const HINIC_COMPILE_TIME_LEN: c_int = 20;
pub const HINIC_MGMT_VERSION_MAX_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_version_info {
    pub status: u8,
    pub version: u8,
    pub rsvd: [u8; 6],
    pub ver: [u8; HINIC_FW_VERSION_NAME],
    pub time: [u8; HINIC_COMPILE_TIME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_rx_mode {
    HINIC_RX_MODE_UC        = BIT(0),
    HINIC_RX_MODE_MC        = BIT(1),
    HINIC_RX_MODE_BC        = BIT(2),
    HINIC_RX_MODE_MC_ALL    = BIT(3),
    HINIC_RX_MODE_PROMISC   = BIT(4),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_port_link_state {
    HINIC_LINK_STATE_DOWN,
    HINIC_LINK_STATE_UP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_port_state {
    HINIC_PORT_DISABLE      = 0,
    HINIC_PORT_ENABLE       = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_func_port_state {
    HINIC_FUNC_PORT_DISABLE = 0,
    HINIC_FUNC_PORT_ENABLE  = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_autoneg_cap {
    HINIC_AUTONEG_UNSUPPORTED,
    HINIC_AUTONEG_SUPPORTED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_autoneg_state {
    HINIC_AUTONEG_DISABLED,
    HINIC_AUTONEG_ACTIVE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_duplex {
    HINIC_DUPLEX_HALF,
    HINIC_DUPLEX_FULL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_speed {
    HINIC_SPEED_10MB_LINK = 0,
    HINIC_SPEED_100MB_LINK,
    HINIC_SPEED_1000MB_LINK,
    HINIC_SPEED_10GB_LINK,
    HINIC_SPEED_25GB_LINK,
    HINIC_SPEED_40GB_LINK,
    HINIC_SPEED_100GB_LINK,

    HINIC_SPEED_UNKNOWN = 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_link_mode {
    HINIC_10GE_BASE_KR = 0,
    HINIC_40GE_BASE_KR4 = 1,
    HINIC_40GE_BASE_CR4 = 2,
    HINIC_100GE_BASE_KR4 = 3,
    HINIC_100GE_BASE_CR4 = 4,
    HINIC_25GE_BASE_KR_S = 5,
    HINIC_25GE_BASE_CR_S = 6,
    HINIC_25GE_BASE_KR = 7,
    HINIC_25GE_BASE_CR = 8,
    HINIC_GE_BASE_KX = 9,
    HINIC_LINK_MODE_NUMBERS,

    HINIC_SUPPORTED_UNKNOWN = 0xFFFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_port_type {
    HINIC_PORT_TP,		/* BASET */
    HINIC_PORT_AUI,
    HINIC_PORT_MII,
    HINIC_PORT_FIBRE,	/* OPTICAL */
    HINIC_PORT_BNC,
    HINIC_PORT_ELEC,
    HINIC_PORT_COPPER,	/* PORT_DA */
    HINIC_PORT_AOC,
    HINIC_PORT_BACKPLANE,
    HINIC_PORT_NONE = 0xEF,
    HINIC_PORT_OTHER = 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_valid_link_settings {
    HILINK_LINK_SET_SPEED = 0x1,
    HILINK_LINK_SET_AUTONEG = 0x2,
    HILINK_LINK_SET_FEC = 0x4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_tso_state {
    HINIC_TSO_DISABLE = 0,
    HINIC_TSO_ENABLE  = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_port_mac_cmd {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub vlan_id: u16,
    pub rsvd1: u16,
    pub mac: [c_uchar; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_port_mtu_cmd {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub rsvd1: u16,
    pub mtu: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_port_vlan_cmd {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub vlan_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_port_rx_mode_cmd {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub rsvd: u16,
    pub rx_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_port_link_cmd {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub state: u8,
    pub rsvd1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_port_state_cmd {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub state: u8,
    pub rsvd1: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_port_link_status {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub link: u8,
    pub port_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cable_plug_event {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub /: *mut *mut u8 plugged; / 0: unplugged, 1: plugged,
    pub port_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum link_err_type {
    LINK_ERR_MODULE_UNRECOGENIZED,
    LINK_ERR_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_link_err_event {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub err_type: u8,
    pub port_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_port_func_state_cmd {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub rsvd1: u16,
    pub state: u8,
    pub rsvd2: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_port_cap {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub rsvd1: u16,
    pub port_type: u8,
    pub autoneg_cap: u8,
    pub autoneg_state: u8,
    pub duplex: u8,
    pub speed: u8,
    pub rsvd2: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_link_mode_cmd {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub rsvd1: u16,
    pub /: *mut *mut u16 supported; / 0xFFFF represents invalid value,
    pub advertised: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_speed_cmd {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub speed: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_set_autoneg_cmd {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub /: *mut *mut u16 enable; / 1: enable , 0: disable,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_link_ksettings_info {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub rsvd1: u16,
    pub valid_bitmap: u32,
    pub /: *mut *mut u32 speed; / enum nic_speed_level,
    pub /: *mut *mut u8 autoneg; / 0 - off; 1 - on,
    pub /: *mut *mut u8 fec; / 0 - RSFEC; 1 - BASEFEC; 2 - NOFEC,
    pub /: *mut *mut u8 rsvd2[18]; / reserved for duplex, port, etc.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_tso_config {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub rsvd1: u16,
    pub tso_en: u8,
    pub resv2: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_checksum_offload {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub rsvd1: u16,
    pub rx_csum_offload: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rq_num {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub rsvd1: [u16; 33],
    pub num_rqs: u32,
    pub rq_depth: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_lro_config {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub rsvd1: u16,
    pub lro_ipv4_en: u8,
    pub lro_ipv6_en: u8,
    pub lro_max_wqe_num: u8,
    pub resv2: [u8; 13],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_lro_timer {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub /: *mut *mut u8 type; / 0: set timer value, 1: get timer value,
    pub /: *mut *mut u8 enable; / when set lro time, enable should be 1,
    pub rsvd1: u16,
    pub timer: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_vlan_cfg {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub vlan_rx_offload: u8,
    pub rsvd1: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_vlan_filter {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub rsvd1: [u8; 2],
    pub enable: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rss_template_mgmt {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub cmd: u8,
    pub template_id: u8,
    pub rsvd1: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rss_template_key {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub template_id: u8,
    pub rsvd1: u8,
    pub key: [u8; HINIC_RSS_KEY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rss_context_tbl {
    pub group_index: u32,
    pub offset: u32,
    pub size: u32,
    pub rsvd: u32,
    pub ctx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rss_context_table {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub template_id: u8,
    pub rsvd1: u8,
    pub context: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rss_indirect_tbl {
    pub group_index: u32,
    pub offset: u32,
    pub size: u32,
    pub rsvd: u32,
    pub entry: [u8; HINIC_RSS_INDIR_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rss_indir_table {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub template_id: u8,
    pub rsvd1: u8,
    pub indir: [u8; HINIC_RSS_INDIR_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rss_key {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub template_id: u8,
    pub rsvd1: u8,
    pub key: [u8; HINIC_RSS_KEY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rss_engine_type {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub template_id: u8,
    pub hash_engine: u8,
    pub rsvd1: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rss_config {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub rss_en: u8,
    pub template_id: u8,
    pub rq_priority_number: u8,
    pub rsvd1: [u8; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_stats {
    pub name: [c_char; ETH_GSTRING_LEN],
    pub size: u32,
    pub offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_vport_stats {
    pub tx_unicast_pkts_vport: u64,
    pub tx_unicast_bytes_vport: u64,
    pub tx_multicast_pkts_vport: u64,
    pub tx_multicast_bytes_vport: u64,
    pub tx_broadcast_pkts_vport: u64,
    pub tx_broadcast_bytes_vport: u64,
    pub rx_unicast_pkts_vport: u64,
    pub rx_unicast_bytes_vport: u64,
    pub rx_multicast_pkts_vport: u64,
    pub rx_multicast_bytes_vport: u64,
    pub rx_broadcast_pkts_vport: u64,
    pub rx_broadcast_bytes_vport: u64,
    pub tx_discard_vport: u64,
    pub rx_discard_vport: u64,
    pub tx_err_vport: u64,
    pub rx_err_vport: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_phy_port_stats {
    pub mac_rx_total_pkt_num: u64,
    pub mac_rx_total_oct_num: u64,
    pub mac_rx_bad_pkt_num: u64,
    pub mac_rx_bad_oct_num: u64,
    pub mac_rx_good_pkt_num: u64,
    pub mac_rx_good_oct_num: u64,
    pub mac_rx_uni_pkt_num: u64,
    pub mac_rx_multi_pkt_num: u64,
    pub mac_rx_broad_pkt_num: u64,
    pub mac_tx_total_pkt_num: u64,
    pub mac_tx_total_oct_num: u64,
    pub mac_tx_bad_pkt_num: u64,
    pub mac_tx_bad_oct_num: u64,
    pub mac_tx_good_pkt_num: u64,
    pub mac_tx_good_oct_num: u64,
    pub mac_tx_uni_pkt_num: u64,
    pub mac_tx_multi_pkt_num: u64,
    pub mac_tx_broad_pkt_num: u64,
    pub mac_rx_fragment_pkt_num: u64,
    pub mac_rx_undersize_pkt_num: u64,
    pub mac_rx_undermin_pkt_num: u64,
    pub mac_rx_64_oct_pkt_num: u64,
    pub mac_rx_65_127_oct_pkt_num: u64,
    pub mac_rx_128_255_oct_pkt_num: u64,
    pub mac_rx_256_511_oct_pkt_num: u64,
    pub mac_rx_512_1023_oct_pkt_num: u64,
    pub mac_rx_1024_1518_oct_pkt_num: u64,
    pub mac_rx_1519_2047_oct_pkt_num: u64,
    pub mac_rx_2048_4095_oct_pkt_num: u64,
    pub mac_rx_4096_8191_oct_pkt_num: u64,
    pub mac_rx_8192_9216_oct_pkt_num: u64,
    pub mac_rx_9217_12287_oct_pkt_num: u64,
    pub mac_rx_12288_16383_oct_pkt_num: u64,
    pub mac_rx_1519_max_bad_pkt_num: u64,
    pub mac_rx_1519_max_good_pkt_num: u64,
    pub mac_rx_oversize_pkt_num: u64,
    pub mac_rx_jabber_pkt_num: u64,
    pub mac_rx_pause_num: u64,
    pub mac_rx_pfc_pkt_num: u64,
    pub mac_rx_pfc_pri0_pkt_num: u64,
    pub mac_rx_pfc_pri1_pkt_num: u64,
    pub mac_rx_pfc_pri2_pkt_num: u64,
    pub mac_rx_pfc_pri3_pkt_num: u64,
    pub mac_rx_pfc_pri4_pkt_num: u64,
    pub mac_rx_pfc_pri5_pkt_num: u64,
    pub mac_rx_pfc_pri6_pkt_num: u64,
    pub mac_rx_pfc_pri7_pkt_num: u64,
    pub mac_rx_control_pkt_num: u64,
    pub mac_rx_y1731_pkt_num: u64,
    pub mac_rx_sym_err_pkt_num: u64,
    pub mac_rx_fcs_err_pkt_num: u64,
    pub mac_rx_send_app_good_pkt_num: u64,
    pub mac_rx_send_app_bad_pkt_num: u64,
    pub mac_tx_fragment_pkt_num: u64,
    pub mac_tx_undersize_pkt_num: u64,
    pub mac_tx_undermin_pkt_num: u64,
    pub mac_tx_64_oct_pkt_num: u64,
    pub mac_tx_65_127_oct_pkt_num: u64,
    pub mac_tx_128_255_oct_pkt_num: u64,
    pub mac_tx_256_511_oct_pkt_num: u64,
    pub mac_tx_512_1023_oct_pkt_num: u64,
    pub mac_tx_1024_1518_oct_pkt_num: u64,
    pub mac_tx_1519_2047_oct_pkt_num: u64,
    pub mac_tx_2048_4095_oct_pkt_num: u64,
    pub mac_tx_4096_8191_oct_pkt_num: u64,
    pub mac_tx_8192_9216_oct_pkt_num: u64,
    pub mac_tx_9217_12287_oct_pkt_num: u64,
    pub mac_tx_12288_16383_oct_pkt_num: u64,
    pub mac_tx_1519_max_bad_pkt_num: u64,
    pub mac_tx_1519_max_good_pkt_num: u64,
    pub mac_tx_oversize_pkt_num: u64,
    pub mac_tx_jabber_pkt_num: u64,
    pub mac_tx_pause_num: u64,
    pub mac_tx_pfc_pkt_num: u64,
    pub mac_tx_pfc_pri0_pkt_num: u64,
    pub mac_tx_pfc_pri1_pkt_num: u64,
    pub mac_tx_pfc_pri2_pkt_num: u64,
    pub mac_tx_pfc_pri3_pkt_num: u64,
    pub mac_tx_pfc_pri4_pkt_num: u64,
    pub mac_tx_pfc_pri5_pkt_num: u64,
    pub mac_tx_pfc_pri6_pkt_num: u64,
    pub mac_tx_pfc_pri7_pkt_num: u64,
    pub mac_tx_control_pkt_num: u64,
    pub mac_tx_y1731_pkt_num: u64,
    pub mac_tx_1588_pkt_num: u64,
    pub mac_tx_err_all_pkt_num: u64,
    pub mac_tx_from_app_good_pkt_num: u64,
    pub mac_tx_from_app_bad_pkt_num: u64,
    pub mac_rx_higig2_ext_pkt_num: u64,
    pub mac_rx_higig2_message_pkt_num: u64,
    pub mac_rx_higig2_error_pkt_num: u64,
    pub mac_rx_higig2_cpu_ctrl_pkt_num: u64,
    pub mac_rx_higig2_unicast_pkt_num: u64,
    pub mac_rx_higig2_broadcast_pkt_num: u64,
    pub mac_rx_higig2_l2_multicast_pkt_num: u64,
    pub mac_rx_higig2_l3_multicast_pkt_num: u64,
    pub mac_tx_higig2_message_pkt_num: u64,
    pub mac_tx_higig2_ext_pkt_num: u64,
    pub mac_tx_higig2_cpu_ctrl_pkt_num: u64,
    pub mac_tx_higig2_unicast_pkt_num: u64,
    pub mac_tx_higig2_broadcast_pkt_num: u64,
    pub mac_tx_higig2_l2_multicast_pkt_num: u64,
    pub mac_tx_higig2_l3_multicast_pkt_num: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_port_stats_info {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub rsvd1: u16,
    pub stats_version: u32,
    pub stats_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_port_stats {
    pub status: u8,
    pub version: u8,
    pub rsvd: [u8; 6],
    pub stats: hinic_phy_port_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmd_vport_stats {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub stats: hinic_vport_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_tx_rate_cfg_max_min {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub rsvd1: u16,
    pub min_rate: u32,
    pub max_rate: u32,
    pub rsvd2: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_tx_rate_cfg {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub rsvd1: u16,
    pub tx_rate: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nic_speed_level {
    LINK_SPEED_10MB = 0,
    LINK_SPEED_100MB,
    LINK_SPEED_1GB,
    LINK_SPEED_10GB,
    LINK_SPEED_25GB,
    LINK_SPEED_40GB,
    LINK_SPEED_100GB,
    LINK_SPEED_LEVELS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_spoofchk_set {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub state: u8,
    pub rsvd1: u8,
    pub func_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_pause_config {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub rsvd1: u16,
    pub auto_neg: u32,
    pub rx_pause: u32,
    pub tx_pause: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_set_pfc {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_id: u16,
    pub pfc_en: u8,
    pub pfc_bitmap: u8,
    pub rsvd1: [u8; 4],
}

// get or set loopback mode, need to modify by base API
pub const HINIC_INTERNAL_LP_MODE: c_int = 5;
pub const LOOP_MODE_MIN: c_int = 1;
pub const LOOP_MODE_MAX: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_port_loopback {
    pub status: u8,
    pub version: u8,
    pub rsvd: [u8; 6],
    pub mode: u32,
    pub en: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_led_info {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub port: u8,
    pub type: u8,
    pub mode: u8,
    pub reset: u8,
}

pub const STD_SFP_INFO_MAX_SIZE: c_int = 640;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmd_get_light_module_abs {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub port_id: u8,
    pub /: *mut *mut u8 abs_status; / 0:present, 1:absent,
    pub rsv: [u8; 2],
}

pub const STD_SFP_INFO_MAX_SIZE: c_int = 640;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmd_get_std_sfp_info {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub port_id: u8,
    pub wire_type: u8,
    pub eeprom_len: u16,
    pub rsvd: u32,
    pub sfp_info: [u8; STD_SFP_INFO_MAX_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmd_update_fw {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub SL:1: u32,
    pub SF:1: u32,
    pub flag:1: u32,
    pub reserved:13: u32,
    pub fragment_len:16: u32,
    pub ctl_info: },
    pub FW_section_CRC: u32,
    pub FW_section_type: u32,
    pub section_info: },
    pub total_len: u32,
    pub setion_total_len: u32,
    pub fw_section_version: u32,
    pub section_offset: u32,
    pub data: [u32; 384],
}

extern "C" {
    pub fn hinic_port_get_mac(nic_dev: *mut hinic_dev, addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn hinic_port_set_mtu(nic_dev: *mut hinic_dev, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn hinic_port_add_vlan(nic_dev: *mut hinic_dev, vlan_id: u16) -> c_int;
}
extern "C" {
    pub fn hinic_port_del_vlan(nic_dev: *mut hinic_dev, vlan_id: u16) -> c_int;
}
extern "C" {
    pub fn hinic_port_set_rx_mode(nic_dev: *mut hinic_dev, rx_mode: u32) -> c_int;
}
extern "C" {
    pub fn hinic_set_max_qnum(nic_dev: *mut hinic_dev, num_rqs: u8) -> c_int;
}
extern "C" {
    pub fn hinic_port_set_tso(nic_dev: *mut hinic_dev, state: hinic_tso_state) -> c_int;
}
extern "C" {
    pub fn hinic_set_rx_csum_offload(nic_dev: *mut hinic_dev, en: u32) -> c_int;
}
extern "C" {
    pub fn hinic_rss_cfg(nic_dev: *mut hinic_dev, rss_en: u8, template_id: u8) -> c_int;
}
extern "C" {
    pub fn hinic_rss_template_alloc(nic_dev: *mut hinic_dev, tmpl_idx: *mut u8) -> c_int;
}
extern "C" {
    pub fn hinic_rss_template_free(nic_dev: *mut hinic_dev, tmpl_idx: u8) -> c_int;
}
extern "C" {
    pub fn hinic_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn hinic_set_rx_vlan_offload(nic_dev: *mut hinic_dev, en: u8) -> c_int;
}
extern "C" {
    pub fn hinic_set_vlan_fliter(nic_dev: *mut hinic_dev, en: u32) -> c_int;
}
extern "C" {
    pub fn hinic_get_mgmt_version(nic_dev: *mut hinic_dev, mgmt_ver: *mut u8) -> c_int;
}
extern "C" {
    pub fn hinic_set_autoneg(hwdev: *mut hinic_hwdev, enable: bool) -> c_int;
}
extern "C" {
    pub fn hinic_set_speed(hwdev: *mut hinic_hwdev, speed: nic_speed_level) -> c_int;
}
extern "C" {
    pub fn hinic_dcb_set_pfc(hwdev: *mut hinic_hwdev, pfc_en: u8, pfc_bitmap: u8) -> c_int;
}
extern "C" {
    pub fn hinic_set_loopback_mode(hwdev: *mut hinic_hwdev, mode: u32, enable: u32) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_led_mode {
    HINIC_LED_MODE_ON,
    HINIC_LED_MODE_OFF,
    HINIC_LED_MODE_FORCE_1HZ,
    HINIC_LED_MODE_FORCE_2HZ,
    HINIC_LED_MODE_FORCE_4HZ,
    HINIC_LED_MODE_1HZ,
    HINIC_LED_MODE_2HZ,
    HINIC_LED_MODE_4HZ,
    HINIC_LED_MODE_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_led_type {
    HINIC_LED_TYPE_LINK,
    HINIC_LED_TYPE_LOW_SPEED,
    HINIC_LED_TYPE_HIGH_SPEED,
    HINIC_LED_TYPE_INVALID,
}

extern "C" {
    pub fn hinic_reset_led_status(hwdev: *mut hinic_hwdev, port: u8) -> c_int;
}
extern "C" {
    pub fn hinic_get_sfp_type(hwdev: *mut hinic_hwdev, data0: *mut u8, data1: *mut u8) -> c_int;
}
extern "C" {
    pub fn hinic_get_sfp_eeprom(hwdev: *mut hinic_hwdev, data: *mut u8, len: *mut u16) -> c_int;
}
extern "C" {
    pub fn hinic_open(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn hinic_close(netdev: *mut net_device) -> c_int;
}
