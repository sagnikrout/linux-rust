//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/dpaa2/dpni-cmd.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
// Copyright 2013-2016 Freescale Semiconductor Inc.
// Copyright 2016 NXP
// Copyright 2020 NXP
//

// DPNI Version
pub const DPNI_VER_MAJOR: c_int = 7;
pub const DPNI_VER_MINOR: c_int = 0;
pub const DPNI_CMD_BASE_VERSION: c_int = 1;
pub const DPNI_CMD_2ND_VERSION: c_int = 2;
pub const DPNI_CMD_3RD_VERSION: c_int = 3;
pub const DPNI_CMD_ID_OFFSET: c_int = 4;

// Macros for accessing command fields smaller than 1byte

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_open {
    pub dpni_id: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_pool {
    pub dpbp_id: __le16,
    pub priority_mask: u8,
    pub pad: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_pools {
    pub num_dpbp: u8,
    pub backup_pool_mask: u8,
    pub pad: u8,
    pub pool_options: u8,
    pub pool: [dpni_cmd_pool; DPNI_MAX_DPBP],
    pub buffer_size: [__le16; DPNI_MAX_DPBP],
}

// The enable indication is always the least significant bit
pub const DPNI_ENABLE_SHIFT: c_int = 0;
pub const DPNI_ENABLE_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_is_enabled {
    pub enabled: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_irq {
// response word 0
    pub irq_val: __le32,
    pub pad: __le32,
// response word 1
    pub irq_addr: __le64,
// response word 2
    pub irq_num: __le32,
    pub type: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_irq_enable {
    pub enable: u8,
    pub pad: [u8; 3],
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_get_irq_enable {
    pub pad: __le32,
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_irq_enable {
    pub enabled: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_irq_mask {
    pub mask: __le32,
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_get_irq_mask {
    pub pad: __le32,
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_irq_mask {
    pub mask: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_get_irq_status {
    pub status: __le32,
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_irq_status {
    pub status: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_clear_irq_status {
    pub status: __le32,
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_attr {
// response word 0
    pub options: __le32,
    pub num_queues: u8,
    pub num_tcs: u8,
    pub mac_filter_entries: u8,
    pub pad0: u8,
// response word 1
    pub vlan_filter_entries: u8,
    pub pad1: u8,
    pub qos_entries: u8,
    pub pad2: u8,
    pub fs_entries: __le16,
    pub pad3: __le16,
// response word 2
    pub qos_key_size: u8,
    pub fs_key_size: u8,
    pub wriop_version: __le16,
}

pub const DPNI_ERROR_ACTION_SHIFT: c_int = 0;
pub const DPNI_ERROR_ACTION_SIZE: c_int = 4;
pub const DPNI_FRAME_ANN_SHIFT: c_int = 4;
pub const DPNI_FRAME_ANN_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_errors_behavior {
    pub errors: __le32,
// from least significant bit: error_action:4, set_frame_annotation:1
    pub flags: u8,
}

// There are 3 separate commands for configuring Rx, Tx and Tx confirmation
// buffer layouts, but they all share the same parameters.
// If one of the functions changes, below structure needs to be split.
//
pub const DPNI_PASS_TS_SHIFT: c_int = 0;
pub const DPNI_PASS_TS_SIZE: c_int = 1;
pub const DPNI_PASS_PR_SHIFT: c_int = 1;
pub const DPNI_PASS_PR_SIZE: c_int = 1;
pub const DPNI_PASS_FS_SHIFT: c_int = 2;
pub const DPNI_PASS_FS_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_get_buffer_layout {
    pub qtype: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_buffer_layout {
// response word 0
    pub pad0: [u8; 6],
// from LSB: pass_timestamp:1, parser_result:1, frame_status:1
    pub flags: u8,
    pub pad1: u8,
// response word 1
    pub private_data_size: __le16,
    pub data_align: __le16,
    pub head_room: __le16,
    pub tail_room: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_buffer_layout {
// cmd word 0
    pub qtype: u8,
    pub pad0: [u8; 3],
    pub options: __le16,
// from LSB: pass_timestamp:1, parser_result:1, frame_status:1
    pub flags: u8,
    pub pad1: u8,
// cmd word 1
    pub private_data_size: __le16,
    pub data_align: __le16,
    pub head_room: __le16,
    pub tail_room: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_offload {
    pub pad: [u8; 3],
    pub dpni_offload: u8,
    pub config: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_get_offload {
    pub pad: [u8; 3],
    pub dpni_offload: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_offload {
    pub pad: __le32,
    pub config: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_get_qdid {
    pub qtype: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_qdid {
    pub qdid: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_tx_data_offset {
    pub data_offset: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_get_statistics {
    pub page_number: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_statistics {
    pub counter: [__le64; DPNI_STATISTICS_CNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_link_cfg {
// cmd word 0
    pub pad0: __le64,
// cmd word 1
    pub rate: __le32,
    pub pad1: __le32,
// cmd word 2
    pub options: __le64,
}

pub const DPNI_LINK_STATE_SHIFT: c_int = 0;
pub const DPNI_LINK_STATE_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_link_state {
// response word 0
    pub pad0: __le32,
// from LSB: up:1
    pub flags: u8,
    pub pad1: [u8; 3],
// response word 1
    pub rate: __le32,
    pub pad2: __le32,
// response word 2
    pub options: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_max_frame_length {
    pub max_frame_length: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_max_frame_length {
    pub max_frame_length: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_multicast_promisc {
    pub enable: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_multicast_promisc {
    pub enabled: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_unicast_promisc {
    pub enable: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_unicast_promisc {
    pub enabled: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_primary_mac_addr {
    pub pad: __le16,
    pub mac_addr: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_primary_mac_addr {
    pub pad: __le16,
    pub mac_addr: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_port_mac_addr {
    pub pad: __le16,
    pub mac_addr: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_add_mac_addr {
    pub pad: __le16,
    pub mac_addr: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_remove_mac_addr {
    pub pad: __le16,
    pub mac_addr: [u8; 6],
}

pub const DPNI_UNICAST_FILTERS_SHIFT: c_int = 0;
pub const DPNI_UNICAST_FILTERS_SIZE: c_int = 1;
pub const DPNI_MULTICAST_FILTERS_SHIFT: c_int = 1;
pub const DPNI_MULTICAST_FILTERS_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_clear_mac_filters {
// from LSB: unicast:1, multicast:1
    pub flags: u8,
}

pub const DPNI_DIST_MODE_SHIFT: c_int = 0;
pub const DPNI_DIST_MODE_SIZE: c_int = 4;
pub const DPNI_MISS_ACTION_SHIFT: c_int = 4;
pub const DPNI_MISS_ACTION_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_rx_tc_dist {
// cmd word 0
    pub dist_size: __le16,
    pub tc_id: u8,
// from LSB: dist_mode:4, miss_action:4
    pub flags: u8,
    pub pad0: __le16,
    pub default_flow_id: __le16,
// cmd word 1..5
    pub pad1: [__le64; 5],
// cmd word 6
    pub key_cfg_iova: __le64,
}

// dpni_set_rx_tc_dist extension (structure of the DMA-able memory at
// key_cfg_iova)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_mask_cfg {
    pub mask: u8,
    pub offset: u8,
}

pub const DPNI_EFH_TYPE_SHIFT: c_int = 0;
pub const DPNI_EFH_TYPE_SIZE: c_int = 4;
pub const DPNI_EXTRACT_TYPE_SHIFT: c_int = 0;
pub const DPNI_EXTRACT_TYPE_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_dist_extract {
// word 0
    pub prot: u8,
// EFH type stored in the 4 least significant bits
    pub efh_type: u8,
    pub size: u8,
    pub offset: u8,
    pub field: __le32,
// word 1
    pub hdr_index: u8,
    pub constant: u8,
    pub num_of_repeats: u8,
    pub num_of_byte_masks: u8,
// Extraction type is stored in the 4 LSBs
    pub extract_type: u8,
    pub pad: [u8; 3],
// word 2
    pub masks: [dpni_mask_cfg; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_ext_set_rx_tc_dist {
// extension word 0
    pub num_extracts: u8,
    pub pad: [u8; 7],
// words 1..25
    pub extracts: [dpni_dist_extract; DPKG_MAX_NUM_OF_EXTRACTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_get_queue {
    pub qtype: u8,
    pub tc: u8,
    pub index: u8,
}

pub const DPNI_DEST_TYPE_SHIFT: c_int = 0;
pub const DPNI_DEST_TYPE_SIZE: c_int = 4;
pub const DPNI_STASH_CTRL_SHIFT: c_int = 6;
pub const DPNI_STASH_CTRL_SIZE: c_int = 1;
pub const DPNI_HOLD_ACTIVE_SHIFT: c_int = 7;
pub const DPNI_HOLD_ACTIVE_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_queue {
// response word 0
    pub pad0: __le64,
// response word 1
    pub dest_id: __le32,
    pub pad1: __le16,
    pub dest_prio: u8,
// From LSB: dest_type:4, pad:2, flc_stash_ctrl:1, hold_active:1
    pub flags: u8,
// response word 2
    pub flc: __le64,
// response word 3
    pub user_context: __le64,
// response word 4
    pub fqid: __le32,
    pub qdbin: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_queue {
// cmd word 0
    pub qtype: u8,
    pub tc: u8,
    pub index: u8,
    pub options: u8,
    pub pad0: __le32,
// cmd word 1
    pub dest_id: __le32,
    pub pad1: __le16,
    pub dest_prio: u8,
    pub flags: u8,
// cmd word 2
    pub flc: __le64,
// cmd word 3
    pub user_context: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_taildrop {
// cmd word 0
    pub congestion_point: u8,
    pub qtype: u8,
    pub tc: u8,
    pub index: u8,
    pub pad0: __le32,
// cmd word 1
// Only least significant bit is relevant
    pub enable: u8,
    pub pad1: u8,
    pub units: u8,
    pub pad2: u8,
    pub threshold: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_get_taildrop {
    pub congestion_point: u8,
    pub qtype: u8,
    pub tc: u8,
    pub index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_taildrop {
// cmd word 0
    pub pad0: __le64,
// cmd word 1
// only least significant bit is relevant
    pub enable: u8,
    pub pad1: u8,
    pub units: u8,
    pub pad2: u8,
    pub threshold: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_get_api_version {
    pub major: __le16,
    pub minor: __le16,
}

pub const DPNI_RX_FS_DIST_ENABLE_SHIFT: c_int = 0;
pub const DPNI_RX_FS_DIST_ENABLE_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_rx_fs_dist {
    pub dist_size: __le16,
    pub enable: u8,
    pub tc: u8,
    pub miss_flow_id: __le16,
    pub pad: __le16,
    pub key_cfg_iova: __le64,
}

pub const DPNI_RX_HASH_DIST_ENABLE_SHIFT: c_int = 0;
pub const DPNI_RX_HASH_DIST_ENABLE_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_rx_hash_dist {
    pub dist_size: __le16,
    pub enable: u8,
    pub tc: u8,
    pub pad: __le32,
    pub key_cfg_iova: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_add_fs_entry {
// cmd word 0
    pub options: __le16,
    pub tc_id: u8,
    pub key_size: u8,
    pub index: __le16,
    pub flow_id: __le16,
// cmd word 1
    pub key_iova: __le64,
// cmd word 2
    pub mask_iova: __le64,
// cmd word 3
    pub flc: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_remove_fs_entry {
// cmd word 0
    pub pad0: __le16,
    pub tc_id: u8,
    pub key_size: u8,
    pub pad1: __le32,
// cmd word 1
    pub key_iova: __le64,
// cmd word 2
    pub mask_iova: __le64,
}

pub const DPNI_DISCARD_ON_MISS_SHIFT: c_int = 0;
pub const DPNI_DISCARD_ON_MISS_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_qos_table {
    pub pad: __le32,
    pub default_tc: u8,
// only the LSB
    pub discard_on_miss: u8,
    pub pad1: [__le16; 21],
    pub key_cfg_iova: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_add_qos_entry {
    pub pad: __le16,
    pub tc_id: u8,
    pub key_size: u8,
    pub index: __le16,
    pub pad1: __le16,
    pub key_iova: __le64,
    pub mask_iova: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_remove_qos_entry {
    pub pad: [u8; 3],
    pub key_size: u8,
    pub pad1: __le32,
    pub key_iova: __le64,
    pub mask_iova: __le64,
}

pub const DPNI_DEST_TYPE_SHIFT: c_int = 0;
pub const DPNI_DEST_TYPE_SIZE: c_int = 4;
pub const DPNI_CONG_UNITS_SHIFT: c_int = 4;
pub const DPNI_CONG_UNITS_SIZE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_congestion_notification {
// cmd word 0
    pub qtype: u8,
    pub tc: u8,
    pub pad: [u8; 6],
// cmd word 1
    pub dest_id: __le32,
    pub notification_mode: __le16,
    pub dest_priority: u8,
// from LSB: dest_type: 4 units:2
    pub type_units: u8,
// cmd word 2
    pub message_iova: __le64,
// cmd word 3
    pub message_ctx: __le64,
// cmd word 4
    pub threshold_entry: __le32,
    pub threshold_exit: __le32,
}

pub const DPNI_COUPLED_SHIFT: c_int = 0;
pub const DPNI_COUPLED_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_set_tx_shaping {
    pub tx_cr_max_burst_size: __le16,
    pub tx_er_max_burst_size: __le16,
    pub pad: __le32,
    pub tx_cr_rate_limit: __le32,
    pub tx_er_rate_limit: __le32,
// from LSB: coupled:1
    pub coupled: u8,
}

pub const DPNI_PTP_ENABLE_SHIFT: c_int = 0;
pub const DPNI_PTP_ENABLE_SIZE: c_int = 1;
pub const DPNI_PTP_CH_UPDATE_SHIFT: c_int = 1;
pub const DPNI_PTP_CH_UPDATE_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_single_step_cfg {
    pub flags: __le16,
    pub offset: __le16,
    pub peer_delay: __le32,
    pub ptp_onestep_reg_base: __le32,
    pub pad0: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rsp_single_step_cfg {
    pub flags: __le16,
    pub offset: __le16,
    pub peer_delay: __le32,
    pub ptp_onestep_reg_base: __le32,
    pub pad0: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_enable_vlan_filter {
// only the LSB
    pub en: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_cmd_vlan_id {
    pub flags: u8,
    pub tc_id: u8,
    pub flow_id: u8,
    pub pad: u8,
    pub vlan_id: __le16,
}
