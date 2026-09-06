//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/dpaa2/dpsw-cmd.h
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
//
// Copyright 2014-2016 Freescale Semiconductor Inc.
// Copyright 2017-2021 NXP
//

// DPSW Version
pub const DPSW_VER_MAJOR: c_int = 8;
pub const DPSW_VER_MINOR: c_int = 13;
pub const DPSW_CMD_BASE_VERSION: c_int = 1;
pub const DPSW_CMD_VERSION_2: c_int = 2;
pub const DPSW_CMD_ID_OFFSET: c_int = 4;

// Command IDs

// Macros for accessing command fields smaller than 1byte

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_open {
    pub dpsw_id: __le32,
}

pub const DPSW_COMPONENT_TYPE_SHIFT: c_int = 0;
pub const DPSW_COMPONENT_TYPE_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_create {
// cmd word 0
    pub num_ifs: __le16,
    pub max_fdbs: u8,
    pub max_meters_per_if: u8,
// from LSB: only the first 4 bits
    pub component_type: u8,
    pub pad: [u8; 3],
// cmd word 1
    pub max_vlans: __le16,
    pub max_fdb_entries: __le16,
    pub fdb_aging_time: __le16,
    pub max_fdb_mc_groups: __le16,
// cmd word 2
    pub options: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_destroy {
    pub dpsw_id: __le32,
}

pub const DPSW_ENABLE_SHIFT: c_int = 0;
pub const DPSW_ENABLE_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_rsp_is_enabled {
// from LSB: enable:1
    pub enabled: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_set_irq_enable {
    pub enable_state: u8,
    pub pad: [u8; 3],
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_get_irq_enable {
    pub pad: __le32,
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_rsp_get_irq_enable {
    pub enable_state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_set_irq_mask {
    pub mask: __le32,
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_get_irq_mask {
    pub pad: __le32,
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_rsp_get_irq_mask {
    pub mask: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_get_irq_status {
    pub status: __le32,
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_rsp_get_irq_status {
    pub status: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_clear_irq_status {
    pub status: __le32,
    pub irq_index: u8,
}

pub const DPSW_COMPONENT_TYPE_SHIFT: c_int = 0;
pub const DPSW_COMPONENT_TYPE_SIZE: c_int = 4;
pub const DPSW_FLOODING_CFG_SHIFT: c_int = 0;
pub const DPSW_FLOODING_CFG_SIZE: c_int = 4;
pub const DPSW_BROADCAST_CFG_SHIFT: c_int = 4;
pub const DPSW_BROADCAST_CFG_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_rsp_get_attr {
// cmd word 0
    pub num_ifs: __le16,
    pub max_fdbs: u8,
    pub num_fdbs: u8,
    pub max_vlans: __le16,
    pub num_vlans: __le16,
// cmd word 1
    pub max_fdb_entries: __le16,
    pub fdb_aging_time: __le16,
    pub dpsw_id: __le32,
// cmd word 2
    pub mem_size: __le16,
    pub max_fdb_mc_groups: __le16,
    pub max_meters_per_if: u8,
// from LSB only the first 4 bits
    pub component_type: u8,
// [0:3] - flooding configuration
// [4:7] - broadcast configuration
//
    pub repl_cfg: u8,
    pub pad: u8,
// cmd word 3
    pub options: __le64,
}

pub const DPSW_VLAN_ID_SHIFT: c_int = 0;
pub const DPSW_VLAN_ID_SIZE: c_int = 12;
pub const DPSW_DEI_SHIFT: c_int = 12;
pub const DPSW_DEI_SIZE: c_int = 1;
pub const DPSW_PCP_SHIFT: c_int = 13;
pub const DPSW_PCP_SIZE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_if_set_tci {
    pub if_id: __le16,
// from LSB: VLAN_ID:12 DEI:1 PCP:3
    pub conf: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_if_get_tci {
    pub if_id: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_rsp_if_get_tci {
    pub pad: __le16,
    pub vlan_id: __le16,
    pub dei: u8,
    pub pcp: u8,
}

pub const DPSW_STATE_SHIFT: c_int = 0;
pub const DPSW_STATE_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_if_set_stp {
    pub if_id: __le16,
    pub vlan_id: __le16,
// only the first LSB 4 bits
    pub state: u8,
}

pub const DPSW_COUNTER_TYPE_SHIFT: c_int = 0;
pub const DPSW_COUNTER_TYPE_SIZE: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_if_get_counter {
    pub if_id: __le16,
// from LSB: type:5
    pub type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_rsp_if_get_counter {
    pub pad: __le64,
    pub counter: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_if {
    pub if_id: __le16,
}

pub const DPSW_ADMIT_UNTAGGED_SHIFT: c_int = 0;
pub const DPSW_ADMIT_UNTAGGED_SIZE: c_int = 4;
pub const DPSW_ENABLED_SHIFT: c_int = 5;
pub const DPSW_ENABLED_SIZE: c_int = 1;
pub const DPSW_ACCEPT_ALL_VLAN_SHIFT: c_int = 6;
pub const DPSW_ACCEPT_ALL_VLAN_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_rsp_if_get_attr {
// cmd word 0
// from LSB: admit_untagged:4 enabled:1 accept_all_vlan:1
    pub conf: u8,
    pub pad1: u8,
    pub num_tcs: u8,
    pub pad2: u8,
    pub qdid: __le16,
// cmd word 1
    pub options: __le32,
    pub pad3: __le32,
// cmd word 2
    pub rate: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_if_set_max_frame_length {
    pub if_id: __le16,
    pub frame_length: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_if_set_link_cfg {
// cmd word 0
    pub if_id: __le16,
    pub pad: [u8; 6],
// cmd word 1
    pub rate: __le32,
    pub pad1: __le32,
// cmd word 2
    pub options: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_if_get_link_state {
    pub if_id: __le16,
}

pub const DPSW_UP_SHIFT: c_int = 0;
pub const DPSW_UP_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_rsp_if_get_link_state {
// cmd word 0
    pub pad0: __le32,
    pub up: u8,
    pub pad1: [u8; 3],
// cmd word 1
    pub rate: __le32,
    pub pad2: __le32,
// cmd word 2
    pub options: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_vlan_add {
    pub fdb_id: __le16,
    pub vlan_id: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_vlan_add_if {
// cmd word 0
    pub options: __le16,
    pub vlan_id: __le16,
    pub fdb_id: __le16,
    pub pad0: __le16,
// cmd word 1-4
    pub if_id: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_vlan_manage_if {
// cmd word 0
    pub pad0: __le16,
    pub vlan_id: __le16,
    pub pad1: __le32,
// cmd word 1-4
    pub if_id: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_vlan_remove {
    pub pad: __le16,
    pub vlan_id: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_fdb_add {
    pub pad: __le32,
    pub fdb_ageing_time: __le16,
    pub num_fdb_entries: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_rsp_fdb_add {
    pub fdb_id: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_fdb_remove {
    pub fdb_id: __le16,
}

pub const DPSW_ENTRY_TYPE_SHIFT: c_int = 0;
pub const DPSW_ENTRY_TYPE_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_fdb_unicast_op {
// cmd word 0
    pub fdb_id: __le16,
    pub mac_addr: [u8; 6],
// cmd word 1
    pub if_egress: __le16,
// only the first 4 bits from LSB
    pub type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_fdb_multicast_op {
// cmd word 0
    pub fdb_id: __le16,
    pub num_ifs: __le16,
// only the first 4 bits from LSB
    pub type: u8,
    pub pad: [u8; 3],
// cmd word 1
    pub mac_addr: [u8; 6],
    pub pad2: __le16,
// cmd word 2-5
    pub if_id: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_fdb_dump {
    pub fdb_id: __le16,
    pub pad0: __le16,
    pub pad1: __le32,
    pub iova_addr: __le64,
    pub iova_size: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_rsp_fdb_dump {
    pub num_entries: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_rsp_ctrl_if_get_attr {
    pub pad: __le64,
    pub rx_fqid: __le32,
    pub rx_err_fqid: __le32,
    pub tx_err_conf_fqid: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_ctrl_if_set_pools {
    pub num_dpbp: u8,
    pub backup_pool_mask: u8,
    pub pad: __le16,
    pub dpbp_id: [__le32; DPSW_MAX_DPBP],
    pub buffer_size: [__le16; DPSW_MAX_DPBP],
}

pub const DPSW_DEST_TYPE_SHIFT: c_int = 0;
pub const DPSW_DEST_TYPE_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_ctrl_if_set_queue {
    pub dest_id: __le32,
    pub dest_priority: u8,
    pub pad: u8,
// from LSB: dest_type:4
    pub dest_type: u8,
    pub qtype: u8,
    pub user_ctx: __le64,
    pub options: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_rsp_get_api_version {
    pub version_major: __le16,
    pub version_minor: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_rsp_if_get_mac_addr {
    pub pad: __le16,
    pub mac_addr: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_set_egress_flood {
    pub fdb_id: __le16,
    pub flood_type: u8,
    pub pad: [u8; 5],
    pub if_id: __le64,
}

pub const DPSW_LEARNING_MODE_SHIFT: c_int = 0;
pub const DPSW_LEARNING_MODE_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_if_set_learning_mode {
    pub if_id: __le16,
// only the first 4 bits from LSB
    pub mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_acl_add {
    pub pad: __le16,
    pub max_entries: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_rsp_acl_add {
    pub acl_id: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_acl_remove {
    pub acl_id: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_acl_if {
    pub acl_id: __le16,
    pub num_ifs: __le16,
    pub pad: __le32,
    pub if_id: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_prep_acl_entry {
    pub match_l2_dest_mac: [u8; 6],
    pub match_l2_tpid: __le16,
    pub match_l2_source_mac: [u8; 6],
    pub match_l2_vlan_id: __le16,
    pub match_l3_dest_ip: __le32,
    pub match_l3_source_ip: __le32,
    pub match_l4_dest_port: __le16,
    pub match_l4_source_port: __le16,
    pub match_l2_ether_type: __le16,
    pub match_l2_pcp_dei: u8,
    pub match_l3_dscp: u8,
    pub mask_l2_dest_mac: [u8; 6],
    pub mask_l2_tpid: __le16,
    pub mask_l2_source_mac: [u8; 6],
    pub mask_l2_vlan_id: __le16,
    pub mask_l3_dest_ip: __le32,
    pub mask_l3_source_ip: __le32,
    pub mask_l4_dest_port: __le16,
    pub mask_l4_source_port: __le16,
    pub mask_l2_ether_type: __le16,
    pub mask_l2_pcp_dei: u8,
    pub mask_l3_dscp: u8,
    pub match_l3_protocol: u8,
    pub mask_l3_protocol: u8,
}

pub const DPSW_RESULT_ACTION_SHIFT: c_int = 0;
pub const DPSW_RESULT_ACTION_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_acl_entry {
    pub acl_id: __le16,
    pub result_if_id: __le16,
    pub precedence: __le32,
// from LSB only the first 4 bits
    pub result_action: u8,
    pub pad: [u8; 7],
    pub pad2: [__le64; 4],
    pub key_iova: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_set_reflection_if {
    pub if_id: __le16,
}

pub const DPSW_FILTER_SHIFT: c_int = 0;
pub const DPSW_FILTER_SIZE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_if_reflection {
    pub if_id: __le16,
    pub vlan_id: __le16,
// only 2 bits from the LSB
    pub filter: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_lag {
    pub group_id: u8,
    pub num_ifs: u8,
    pub pad: [u8; 6],
    pub if_id: [u8; DPSW_MAX_LAG_IFS],
    pub phase: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_cmd_if_set_lag_state {
    pub if_id: __le16,
    pub tx_enabled: u8,
}

