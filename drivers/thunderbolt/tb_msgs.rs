//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thunderbolt/tb_msgs.h
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
// Thunderbolt control channel messages
//
// Copyright (C) 2014 Andreas Noever <andreas.noever@gmail.com>
// Copyright (C) 2017, Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_cfg_space {
    TB_CFG_HOPS = 0,
    TB_CFG_PORT = 1,
    TB_CFG_SWITCH = 2,
    TB_CFG_COUNTERS = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_cfg_error {
    TB_CFG_ERROR_PORT_NOT_CONNECTED = 0,
    TB_CFG_ERROR_LINK_ERROR = 1,
    TB_CFG_ERROR_INVALID_CONFIG_SPACE = 2,
    TB_CFG_ERROR_NO_SUCH_PORT = 4,
    TB_CFG_ERROR_ACK_PLUG_EVENT = 7, /* send as reply to TB_CFG_PKG_EVENT */
    TB_CFG_ERROR_LOOP = 8,
    TB_CFG_ERROR_HEC_ERROR_DETECTED = 12,
    TB_CFG_ERROR_FLOW_CONTROL_ERROR = 13,
    TB_CFG_ERROR_LOCK = 15,
    TB_CFG_ERROR_DP_BW = 32,
    TB_CFG_ERROR_ROP_CMPLT = 33,
    TB_CFG_ERROR_POP_CMPLT = 34,
    TB_CFG_ERROR_PCIE_WAKE = 35,
    TB_CFG_ERROR_DP_CON_CHANGE = 36,
    TB_CFG_ERROR_DPTX_DISCOVERY = 37,
    TB_CFG_ERROR_LINK_RECOVERY = 38,
    TB_CFG_ERROR_ASYM_LINK = 39,
}

// common header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_cfg_header {
    pub route_hi:22: u32,
    pub /: *mut *mut u32 unknown:10; / highest order bit is set on replies,
    pub route_lo: u32,
    pub __packed: },
// additional header for read/write packets
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_cfg_address {
    pub /: *mut *mut u32 offset:13; / in dwords,
    pub /: *mut *mut u32 length:6; / in dwords,
    pub port:6: u32,
    pub space:2: tb_cfg_space,
    pub /: *mut *mut u32 seq:2; / sequence number,
    pub zero:3: u32,
    pub __packed: },
// TB_CFG_PKG_READ, response for TB_CFG_PKG_WRITE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg_read_pkg {
    pub header: tb_cfg_header,
    pub addr: tb_cfg_address,
    pub __packed: },
// TB_CFG_PKG_WRITE, response for TB_CFG_PKG_READ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg_write_pkg {
    pub header: tb_cfg_header,
    pub addr: tb_cfg_address,
    pub /: *mut *mut u32 data[64]; / maximum size, tb_cfg_address.length has 6 bits,
    pub __packed: },
// TB_CFG_PKG_ERROR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg_error_pkg {
    pub header: tb_cfg_header,
    pub error:8: tb_cfg_error,
    pub port:6: u32,
    pub reserved:16: u32,
    pub pg:2: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg_ack_pkg {
    pub header: tb_cfg_header,
}

pub const TB_CFG_ERROR_PG_HOT_PLUG: c_uint = 0x2;
pub const TB_CFG_ERROR_PG_HOT_UNPLUG: c_uint = 0x3;
// TB_CFG_PKG_EVENT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg_event_pkg {
    pub header: tb_cfg_header,
    pub port:6: u32,
    pub zero:25: u32,
    pub unplug:1: bool,
    pub __packed: },
// TB_CFG_PKG_RESET
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg_reset_pkg {
    pub header: tb_cfg_header,
    pub __packed: },
// ICM messages
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icm_pkg_code {
    ICM_GET_TOPOLOGY = 0x1,
    ICM_DRIVER_READY = 0x3,
    ICM_APPROVE_DEVICE = 0x4,
    ICM_CHALLENGE_DEVICE = 0x5,
    ICM_ADD_DEVICE_KEY = 0x6,
    ICM_GET_ROUTE = 0xa,
    ICM_APPROVE_XDOMAIN = 0x10,
    ICM_DISCONNECT_XDOMAIN = 0x11,
    ICM_PREBOOT_ACL = 0x18,
    ICM_USB4_SWITCH_OP = 0x20,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icm_event_code {
    ICM_EVENT_DEVICE_CONNECTED = 0x3,
    ICM_EVENT_DEVICE_DISCONNECTED = 0x4,
    ICM_EVENT_XDOMAIN_CONNECTED = 0x6,
    ICM_EVENT_XDOMAIN_DISCONNECTED = 0x7,
    ICM_EVENT_DP_CONFIG_CHANGED = 0x8,
    ICM_EVENT_RTD3_VETO = 0xa,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_pkg_header {
    pub code: u8,
    pub flags: u8,
    pub packet_id: u8,
    pub total_packets: u8,
}

pub const ICM_FLAGS_SLEVEL_SHIFT: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_pkg_driver_ready {
    pub hdr: icm_pkg_header,
}

// Falcon Ridge only messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_fr_pkg_driver_ready_response {
    pub hdr: icm_pkg_header,
    pub romver: u8,
    pub ramver: u8,
    pub security_level: u16,
}

pub const ICM_FR_SLEVEL_MASK: c_uint = 0xf;
// Falcon Ridge & Alpine Ridge common messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_fr_pkg_get_topology {
    pub hdr: icm_pkg_header,
}

pub const ICM_GET_TOPOLOGY_PACKETS: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_fr_pkg_get_topology_response {
    pub hdr: icm_pkg_header,
    pub route_lo: u32,
    pub route_hi: u32,
    pub first_data: u8,
    pub second_data: u8,
    pub drom_i2c_address_index: u8,
    pub switch_index: u8,
    pub reserved: [u32; 2],
    pub ports: [u32; 16],
    pub port_hop_info: [u32; 16],
}

pub const ICM_SWITCH_UPSTREAM_PORT_SHIFT: c_int = 1;

pub const ICM_PORT_INDEX_SHIFT: c_int = 24;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_fr_event_device_connected {
    pub hdr: icm_pkg_header,
    pub ep_uuid: uuid_t,
    pub connection_key: u8,
    pub connection_id: u8,
    pub link_info: u16,
    pub ep_name: [u32; 55],
}

pub const ICM_LINK_INFO_LINK_MASK: c_uint = 0x7;
pub const ICM_LINK_INFO_DEPTH_SHIFT: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_fr_pkg_approve_device {
    pub hdr: icm_pkg_header,
    pub ep_uuid: uuid_t,
    pub connection_key: u8,
    pub connection_id: u8,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_fr_event_device_disconnected {
    pub hdr: icm_pkg_header,
    pub reserved: u16,
    pub link_info: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_fr_event_xdomain_connected {
    pub hdr: icm_pkg_header,
    pub reserved: u16,
    pub link_info: u16,
    pub remote_uuid: uuid_t,
    pub local_uuid: uuid_t,
    pub local_route_hi: u32,
    pub local_route_lo: u32,
    pub remote_route_hi: u32,
    pub remote_route_lo: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_fr_event_xdomain_disconnected {
    pub hdr: icm_pkg_header,
    pub reserved: u16,
    pub link_info: u16,
    pub remote_uuid: uuid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_fr_pkg_add_device_key {
    pub hdr: icm_pkg_header,
    pub ep_uuid: uuid_t,
    pub connection_key: u8,
    pub connection_id: u8,
    pub reserved: u16,
    pub key: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_fr_pkg_add_device_key_response {
    pub hdr: icm_pkg_header,
    pub ep_uuid: uuid_t,
    pub connection_key: u8,
    pub connection_id: u8,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_fr_pkg_challenge_device {
    pub hdr: icm_pkg_header,
    pub ep_uuid: uuid_t,
    pub connection_key: u8,
    pub connection_id: u8,
    pub reserved: u16,
    pub challenge: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_fr_pkg_challenge_device_response {
    pub hdr: icm_pkg_header,
    pub ep_uuid: uuid_t,
    pub connection_key: u8,
    pub connection_id: u8,
    pub reserved: u16,
    pub challenge: [u32; 8],
    pub response: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_fr_pkg_approve_xdomain {
    pub hdr: icm_pkg_header,
    pub reserved: u16,
    pub link_info: u16,
    pub remote_uuid: uuid_t,
    pub transmit_path: u16,
    pub transmit_ring: u16,
    pub receive_path: u16,
    pub receive_ring: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_fr_pkg_approve_xdomain_response {
    pub hdr: icm_pkg_header,
    pub reserved: u16,
    pub link_info: u16,
    pub remote_uuid: uuid_t,
    pub transmit_path: u16,
    pub transmit_ring: u16,
    pub receive_path: u16,
    pub receive_ring: u16,
}

// Alpine Ridge only messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_ar_pkg_driver_ready_response {
    pub hdr: icm_pkg_header,
    pub romver: u8,
    pub ramver: u8,
    pub info: u16,
}

pub const ICM_AR_INFO_BOOT_ACL_SHIFT: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_ar_pkg_get_route {
    pub hdr: icm_pkg_header,
    pub reserved: u16,
    pub link_info: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_ar_pkg_get_route_response {
    pub hdr: icm_pkg_header,
    pub reserved: u16,
    pub link_info: u16,
    pub route_hi: u32,
    pub route_lo: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_ar_boot_acl_entry {
    pub uuid_lo: u32,
    pub uuid_hi: u32,
}

pub const ICM_AR_PREBOOT_ACL_ENTRIES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_ar_pkg_preboot_acl {
    pub hdr: icm_pkg_header,
    pub acl: [icm_ar_boot_acl_entry; ICM_AR_PREBOOT_ACL_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_ar_pkg_preboot_acl_response {
    pub hdr: icm_pkg_header,
    pub acl: [icm_ar_boot_acl_entry; ICM_AR_PREBOOT_ACL_ENTRIES],
}

// Titan Ridge messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_tr_pkg_driver_ready_response {
    pub hdr: icm_pkg_header,
    pub reserved1: u16,
    pub info: u16,
    pub nvm_version: u32,
    pub device_id: u16,
    pub reserved2: u16,
}

pub const ICM_TR_INFO_PROTO_VERSION_SHIFT: c_int = 4;
pub const ICM_TR_INFO_BOOT_ACL_SHIFT: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_tr_event_device_connected {
    pub hdr: icm_pkg_header,
    pub ep_uuid: uuid_t,
    pub route_hi: u32,
    pub route_lo: u32,
    pub connection_id: u8,
    pub reserved: u8,
    pub link_info: u16,
    pub ep_name: [u32; 55],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_tr_event_device_disconnected {
    pub hdr: icm_pkg_header,
    pub route_hi: u32,
    pub route_lo: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_tr_event_xdomain_connected {
    pub hdr: icm_pkg_header,
    pub reserved: u16,
    pub link_info: u16,
    pub remote_uuid: uuid_t,
    pub local_uuid: uuid_t,
    pub local_route_hi: u32,
    pub local_route_lo: u32,
    pub remote_route_hi: u32,
    pub remote_route_lo: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_tr_event_xdomain_disconnected {
    pub hdr: icm_pkg_header,
    pub route_hi: u32,
    pub route_lo: u32,
    pub remote_uuid: uuid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_tr_pkg_approve_device {
    pub hdr: icm_pkg_header,
    pub ep_uuid: uuid_t,
    pub route_hi: u32,
    pub route_lo: u32,
    pub connection_id: u8,
    pub reserved1: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_tr_pkg_add_device_key {
    pub hdr: icm_pkg_header,
    pub ep_uuid: uuid_t,
    pub route_hi: u32,
    pub route_lo: u32,
    pub connection_id: u8,
    pub reserved: [u8; 3],
    pub key: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_tr_pkg_challenge_device {
    pub hdr: icm_pkg_header,
    pub ep_uuid: uuid_t,
    pub route_hi: u32,
    pub route_lo: u32,
    pub connection_id: u8,
    pub reserved: [u8; 3],
    pub challenge: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_tr_pkg_approve_xdomain {
    pub hdr: icm_pkg_header,
    pub route_hi: u32,
    pub route_lo: u32,
    pub remote_uuid: uuid_t,
    pub transmit_path: u16,
    pub transmit_ring: u16,
    pub receive_path: u16,
    pub receive_ring: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_tr_pkg_disconnect_xdomain {
    pub hdr: icm_pkg_header,
    pub stage: u8,
    pub reserved: [u8; 3],
    pub route_hi: u32,
    pub route_lo: u32,
    pub remote_uuid: uuid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_tr_pkg_challenge_device_response {
    pub hdr: icm_pkg_header,
    pub ep_uuid: uuid_t,
    pub route_hi: u32,
    pub route_lo: u32,
    pub connection_id: u8,
    pub reserved: [u8; 3],
    pub challenge: [u32; 8],
    pub response: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_tr_pkg_add_device_key_response {
    pub hdr: icm_pkg_header,
    pub ep_uuid: uuid_t,
    pub route_hi: u32,
    pub route_lo: u32,
    pub connection_id: u8,
    pub reserved: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_tr_pkg_approve_xdomain_response {
    pub hdr: icm_pkg_header,
    pub route_hi: u32,
    pub route_lo: u32,
    pub remote_uuid: uuid_t,
    pub transmit_path: u16,
    pub transmit_ring: u16,
    pub receive_path: u16,
    pub receive_ring: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_tr_pkg_disconnect_xdomain_response {
    pub hdr: icm_pkg_header,
    pub stage: u8,
    pub reserved: [u8; 3],
    pub route_hi: u32,
    pub route_lo: u32,
    pub remote_uuid: uuid_t,
}

// Ice Lake messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_icl_event_rtd3_veto {
    pub hdr: icm_pkg_header,
    pub veto_reason: u32,
}

// USB4 ICM messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_usb4_switch_op {
    pub hdr: icm_pkg_header,
    pub route_hi: u32,
    pub route_lo: u32,
    pub metadata: u32,
    pub opcode: u16,
    pub data_len_valid: u16,
    pub data: [u32; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icm_usb4_switch_op_response {
    pub hdr: icm_pkg_header,
    pub route_hi: u32,
    pub route_lo: u32,
    pub metadata: u32,
    pub opcode: u16,
    pub status: u16,
    pub data: [u32; 16],
}

// XDomain messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_xdomain_header {
    pub route_hi: u32,
    pub route_lo: u32,
    pub length_sn: u32,
}

pub const TB_XDOMAIN_SN_SHIFT: c_int = 27;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_xdp_type {
    UUID_REQUEST_OLD = 1,
    UUID_RESPONSE = 2,
    PROPERTIES_REQUEST,
    PROPERTIES_RESPONSE,
    PROPERTIES_CHANGED_REQUEST,
    PROPERTIES_CHANGED_RESPONSE,
    ERROR_RESPONSE,
    UUID_REQUEST = 12,
    LINK_STATE_STATUS_REQUEST = 15,
    LINK_STATE_STATUS_RESPONSE,
    LINK_STATE_CHANGE_REQUEST,
    LINK_STATE_CHANGE_RESPONSE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_xdp_header {
    pub xd_hdr: tb_xdomain_header,
    pub uuid: uuid_t,
    pub type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_xdp_error_response {
    pub hdr: tb_xdp_header,
    pub error: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_xdp_link_state_status {
    pub hdr: tb_xdp_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_xdp_link_state_status_response {
    pub err: tb_xdp_error_response,
    pub hdr: tb_xdp_header,
    pub status: u32,
    pub slw: u8,
    pub tlw: u8,
    pub sls: u8,
    pub tls: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_xdp_link_state_change {
    pub hdr: tb_xdp_header,
    pub tlw: u8,
    pub tls: u8,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_xdp_link_state_change_response {
    pub err: tb_xdp_error_response,
    pub hdr: tb_xdp_header,
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_xdp_uuid {
    pub hdr: tb_xdp_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_xdp_uuid_response {
    pub err: tb_xdp_error_response,
    pub hdr: tb_xdp_header,
    pub src_uuid: uuid_t,
    pub src_route_hi: u32,
    pub src_route_lo: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_xdp_properties {
    pub hdr: tb_xdp_header,
    pub src_uuid: uuid_t,
    pub dst_uuid: uuid_t,
    pub offset: u16,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_xdp_properties_response {
    pub err: tb_xdp_error_response,
    pub hdr: tb_xdp_header,
    pub src_uuid: uuid_t,
    pub dst_uuid: uuid_t,
    pub offset: u16,
    pub data_length: u16,
    pub generation: u32,
    pub data: [u32; ],
}

//
// Max length of data array single XDomain property response is allowed
// to carry.
//

// Maximum size of the total property block in dwords we allow
pub const TB_XDP_PROPERTIES_MAX_LENGTH: c_int = 500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_xdp_properties_changed {
    pub hdr: tb_xdp_header,
    pub src_uuid: uuid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_xdp_properties_changed_response {
    pub err: tb_xdp_error_response,
    pub hdr: tb_xdp_header,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_xdp_error {
    ERROR_SUCCESS,
    ERROR_UNKNOWN_PACKET,
    ERROR_UNKNOWN_DOMAIN,
    ERROR_NOT_SUPPORTED,
    ERROR_NOT_READY,
}
