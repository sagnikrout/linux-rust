//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_nic_cfg.h
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

pub const HINIC3_MIN_MTU_SIZE: c_int = 256;
pub const HINIC3_MAX_JUMBO_FRAME_SIZE: c_int = 9600;
pub const HINIC3_VLAN_ID_MASK: c_uint = 0x7FFF;
pub const HINIC3_PF_SET_VF_ALREADY: c_uint = 0x4;
pub const HINIC3_MGMT_STATUS_EXIST: c_uint = 0x6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_nic_event_type {
    HINIC3_NIC_EVENT_LINK_DOWN = 0,
    HINIC3_NIC_EVENT_LINK_UP   = 1,
    HINIC3_NIC_EVENT_PORT_MODULE_EVENT = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_sq_attr {
    pub dma_attr_off: u8,
    pub pending_limit: u8,
    pub coalescing_time: u8,
    pub intr_en: u8,
    pub intr_idx: u16,
    pub l2nic_sqn: u32,
    pub ci_dma_base: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mag_cmd_port_an {
    PORT_CFG_AN_ON  = 1,
}

// mag supported/advertised link mode bitmap
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mag_cmd_link_mode {
    LINK_MODE_GE            = 0,
    LINK_MODE_10GE_BASE_R   = 1,
    LINK_MODE_25GE_BASE_R   = 2,
    LINK_MODE_40GE_BASE_R4  = 3,
    LINK_MODE_50GE_BASE_R   = 4,
    LINK_MODE_50GE_BASE_R2  = 5,
    LINK_MODE_100GE_BASE_R  = 6,
    LINK_MODE_100GE_BASE_R2 = 7,
    LINK_MODE_100GE_BASE_R4 = 8,
    LINK_MODE_200GE_BASE_R2 = 9,
    LINK_MODE_200GE_BASE_R4 = 10,
    LINK_MODE_MAX_NUMBERS,

    LINK_MODE_UNKNOWN       = 0xFFFF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mag_cmd_get_port_info {
    pub head: mgmt_msg_head,
    pub port_id: u8,
    pub rsvd0: [u8; 3],
    pub wire_type: u8,
    pub an_support: u8,
    pub an_en: u8,
    pub duplex: u8,
    pub speed: u8,
    pub fec: u8,
    pub lanes: u8,
    pub rsvd1: u8,
    pub supported_mode: u32,
    pub advertised_mode: u32,
    pub rsvd2: [u8; 8],
}

pub const MAG_CMD_PORT_DISABLE: c_uint = 0x0;
pub const MAG_CMD_TX_ENABLE: c_uint = 0x1;
pub const MAG_CMD_RX_ENABLE: c_uint = 0x2;
// the physical port is disabled only when all pf of the port are set to down,
// if any pf is enabled, the port is enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mag_cmd_set_port_enable {
    pub head: mgmt_msg_head,
    pub function_id: u16,
    pub rsvd0: u16,
// bitmap bit0:tx_en bit1:rx_en
    pub state: u8,
    pub rsvd1: [u8; 3],
}

// xsfp wire type, refers to cmis protocol definition
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mag_wire_type {
    MAG_CMD_WIRE_TYPE_UNKNOWN   = 0x0,
    MAG_CMD_WIRE_TYPE_MM        = 0x1,
    MAG_CMD_WIRE_TYPE_SM        = 0x2,
    MAG_CMD_WIRE_TYPE_COPPER    = 0x3,
    MAG_CMD_WIRE_TYPE_ACC       = 0x4,
    MAG_CMD_WIRE_TYPE_BASET     = 0x5,
    MAG_CMD_WIRE_TYPE_AOC       = 0x40,
    MAG_CMD_WIRE_TYPE_ELECTRIC  = 0x41,
    MAG_CMD_WIRE_TYPE_BACKPLANE = 0x42
}

pub const XSFP_INFO_MAX_SIZE: c_int = 640;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mag_cmd_get_xsfp_info {
    pub head: mgmt_msg_head,
    pub port_id: u8,
    pub wire_type: u8,
    pub out_len: u16,
    pub rsvd: u32,
    pub sfp_info: [u8; XSFP_INFO_MAX_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mag_cmd_get_xsfp_present {
    pub head: mgmt_msg_head,
    pub port_id: u8,
// 0:present, 1:absent
    pub abs_status: u8,
    pub rsvd: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum link_err_type {
    LINK_ERR_MODULE_UNRECOGENIZED,
    LINK_ERR_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum port_module_event_type {
    HINIC3_PORT_MODULE_CABLE_PLUGGED,
    HINIC3_PORT_MODULE_CABLE_UNPLUGGED,
    HINIC3_PORT_MODULE_LINK_ERR,
    HINIC3_PORT_MODULE_MAX_EVENT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_port_module_event {
    pub type: port_module_event_type,
    pub err_type: link_err_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_nic_port_info {
    pub port_type: u8,
    pub autoneg_cap: u8,
    pub autoneg_state: u8,
    pub duplex: u8,
    pub speed: u8,
    pub fec: u8,
    pub supported_mode: u32,
    pub advertised_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_nic_pause_config {
    pub auto_neg: u8,
    pub rx_pause: u8,
    pub tx_pause: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_nic_cfg {
// Valid when pfc is disabled
    pub pause_set: bool,
    pub nic_pause: hinic3_nic_pause_config,
    pub pfc_en: u8,
    pub pfc_bitmap: u8,
    pub port_info: hinic3_nic_port_info,
}

extern "C" {
    pub fn hinic3_get_nic_feature_from_hw(nic_dev: *mut hinic3_nic_dev) -> c_int;
}
extern "C" {
    pub fn hinic3_set_nic_feature_to_hw(nic_dev: *mut hinic3_nic_dev) -> c_int;
}
extern "C" {
    pub fn hinic3_update_nic_feature(nic_dev: *mut hinic3_nic_dev, feature_cap: u64);
}
extern "C" {
    pub fn hinic3_set_rx_vlan_offload(hwdev: *mut hinic3_hwdev, en: u8) -> c_int;
}
extern "C" {
    pub fn hinic3_set_vlan_filter(hwdev: *mut hinic3_hwdev, vlan_filter_ctrl: u32) -> c_int;
}
extern "C" {
    pub fn hinic3_init_function_table(nic_dev: *mut hinic3_nic_dev) -> c_int;
}
extern "C" {
    pub fn hinic3_set_port_mtu(netdev: *mut net_device, new_mtu: u16) -> c_int;
}
extern "C" {
    pub fn hinic3_get_default_mac(hwdev: *mut hinic3_hwdev, mac_addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn hinic3_flush_qps_res(hwdev: *mut hinic3_hwdev) -> c_int;
}
extern "C" {
    pub fn hinic3_force_drop_tx_pkt(hwdev: *mut hinic3_hwdev) -> c_int;
}
extern "C" {
    pub fn hinic3_set_rx_mode(hwdev: *mut hinic3_hwdev, rx_mode: u32) -> c_int;
}
extern "C" {
    pub fn hinic3_sync_dcb_state(hwdev: *mut hinic3_hwdev, op_code: u8, state: u8) -> c_int;
}
extern "C" {
    pub fn hinic3_set_port_enable(hwdev: *mut hinic3_hwdev, enable: bool) -> c_int;
}
extern "C" {
    pub fn hinic3_get_link_status(hwdev: *mut hinic3_hwdev, link_status_up: *mut bool) -> c_int;
}
extern "C" {
    pub fn hinic3_add_vlan(hwdev: *mut hinic3_hwdev, vlan_id: u16, func_id: u16) -> c_int;
}
extern "C" {
    pub fn hinic3_del_vlan(hwdev: *mut hinic3_hwdev, vlan_id: u16, func_id: u16) -> c_int;
}
