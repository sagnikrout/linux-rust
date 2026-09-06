//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/intel-ish-hid/ishtp/hbm.h
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
// ISHTP bus layer messages handling
//
// Copyright (c) 2003-2016, Intel Corporation.
//

//
// Timeouts in Seconds
//

//
// ISHTP Version
//
pub const HBM_MINOR_VERSION: c_int = 0;
pub const HBM_MAJOR_VERSION: c_int = 1;
// Host bus message command opcode
pub const ISHTP_HBM_CMD_OP_MSK: c_uint = 0x7f;
// Host bus message command RESPONSE
pub const ISHTP_HBM_CMD_RES_MSK: c_uint = 0x80;
//
// ISHTP Bus Message Command IDs
//
pub const HOST_START_REQ_CMD: c_uint = 0x01;
pub const HOST_START_RES_CMD: c_uint = 0x81;
pub const HOST_STOP_REQ_CMD: c_uint = 0x02;
pub const HOST_STOP_RES_CMD: c_uint = 0x82;
pub const FW_STOP_REQ_CMD: c_uint = 0x03;
pub const HOST_ENUM_REQ_CMD: c_uint = 0x04;
pub const HOST_ENUM_RES_CMD: c_uint = 0x84;
pub const HOST_CLIENT_PROPERTIES_REQ_CMD: c_uint = 0x05;
pub const HOST_CLIENT_PROPERTIES_RES_CMD: c_uint = 0x85;
pub const CLIENT_CONNECT_REQ_CMD: c_uint = 0x06;
pub const CLIENT_CONNECT_RES_CMD: c_uint = 0x86;
pub const CLIENT_DISCONNECT_REQ_CMD: c_uint = 0x07;
pub const CLIENT_DISCONNECT_RES_CMD: c_uint = 0x87;
pub const ISHTP_FLOW_CONTROL_CMD: c_uint = 0x08;
pub const DMA_BUFFER_ALLOC_NOTIFY: c_uint = 0x11;
pub const DMA_BUFFER_ALLOC_RESPONSE: c_uint = 0x91;
pub const DMA_XFER: c_uint = 0x12;
pub const DMA_XFER_ACK: c_uint = 0x92;
//
// ISHTP Stop Reason
// used by hbm_host_stop_request.reason
//
pub const DRIVER_STOP_REQUEST: c_uint = 0x00;
//
// ISHTP BUS Interface Section
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_msg_hdr {
    pub fw_addr:8: u32,
    pub host_addr:8: u32,
    pub length:9: u32,
    pub reserved:6: u32,
    pub msg_complete:1: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_bus_message {
    pub hbm_cmd: u8,
    pub data: [u8; ],
    pub __packed: },
//
// struct hbm_cl_cmd - client specific host bus command
// CONNECT, DISCONNECT, and FlOW CONTROL
//
// @hbm_cmd - bus message command header
// @fw_addr - address of the fw client
// @host_addr - address of the client in the driver
// @data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_hbm_cl_cmd {
    pub hbm_cmd: u8,
    pub fw_addr: u8,
    pub host_addr: u8,
    pub data: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_version {
    pub minor_version: u8,
    pub major_version: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_host_version_request {
    pub hbm_cmd: u8,
    pub reserved: u8,
    pub host_version: hbm_version,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_host_version_response {
    pub hbm_cmd: u8,
    pub host_version_supported: u8,
    pub fw_max_version: hbm_version,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_host_stop_request {
    pub hbm_cmd: u8,
    pub reason: u8,
    pub reserved: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_host_stop_response {
    pub hbm_cmd: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_host_enum_request {
    pub hbm_cmd: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_host_enum_response {
    pub hbm_cmd: u8,
    pub reserved: [u8; 3],
    pub valid_addresses: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ishtp_client_properties {
    pub protocol_name: guid_t,
    pub protocol_version: u8,
    pub max_number_of_connections: u8,
    pub fixed_address: u8,
    pub single_recv_buf: u8,
    pub max_msg_length: u32,
    pub dma_hdr_len: u8,
pub const ISHTP_CLIENT_DMA_ENABLED: c_uint = 0x80;
    pub reserved4: u8,
    pub reserved5: u8,
    pub reserved6: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_props_request {
    pub hbm_cmd: u8,
    pub address: u8,
    pub reserved: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_props_response {
    pub hbm_cmd: u8,
    pub address: u8,
    pub status: u8,
    pub reserved: [u8; 1],
    pub client_properties: ishtp_client_properties,
    pub __packed: },
//
// struct hbm_client_connect_request - connect/disconnect request
//
// @hbm_cmd - bus message command header
// @fw_addr - address of the fw client
// @host_addr - address of the client in the driver
// @reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_client_connect_request {
    pub hbm_cmd: u8,
    pub fw_addr: u8,
    pub host_addr: u8,
    pub reserved: u8,
    pub __packed: },
//
// struct hbm_client_connect_response - connect/disconnect response
//
// @hbm_cmd - bus message command header
// @fw_addr - address of the fw client
// @host_addr - address of the client in the driver
// @status - status of the request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_client_connect_response {
    pub hbm_cmd: u8,
    pub fw_addr: u8,
    pub host_addr: u8,
    pub status: u8,
    pub __packed: },
pub const ISHTP_FC_MESSAGE_RESERVED_LENGTH: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_flow_control {
    pub hbm_cmd: u8,
    pub fw_addr: u8,
    pub host_addr: u8,
    pub reserved: [u8; ISHTP_FC_MESSAGE_RESERVED_LENGTH],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_alloc_notify {
    pub hbm: u8,
    pub status: u8,
    pub reserved: [u8; 2],
    pub buf_size: u32,
    pub buf_address: u64,
// [...] May come more size/address pairs
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_xfer_hbm {
    pub hbm: u8,
    pub fw_client_id: u8,
    pub host_client_id: u8,
    pub reserved: u8,
    pub msg_addr: u64,
    pub msg_length: u32,
    pub reserved2: u32,
    pub __packed: },
// System state
pub const ISHTP_SYSTEM_STATE_CLIENT_ADDR: c_int = 13;
pub const SYSTEM_STATE_SUBSCRIBE: c_uint = 0x1;
pub const SYSTEM_STATE_STATUS: c_uint = 0x2;
pub const SYSTEM_STATE_QUERY_SUBSCRIBERS: c_uint = 0x3;
pub const SYSTEM_STATE_STATE_CHANGE_REQ: c_uint = 0x4;
// indicates suspend and resume states

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ish_system_states_header {
    pub cmd: u32,
    pub set*/: *mut *mut uint32_t cmd_status; /responses will have this,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ish_system_states_subscribe {
    pub hdr: ish_system_states_header,
    pub states: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ish_system_states_status {
    pub hdr: ish_system_states_header,
    pub supported_states: u32,
    pub states_status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ish_system_states_query_subscribers {
    pub hdr: ish_system_states_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ish_system_states_state_change_req {
    pub hdr: ish_system_states_header,
    pub requested_states: u32,
    pub states_status: u32,
    pub __packed: },
//
// enum ishtp_hbm_state - host bus message protocol state
//
// @ISHTP_HBM_IDLE : protocol not started
// @ISHTP_HBM_START : start request message was sent
// @ISHTP_HBM_ENUM_CLIENTS : enumeration request was sent
// @ISHTP_HBM_CLIENT_PROPERTIES : acquiring clients properties
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ishtp_hbm_state {
    ISHTP_HBM_IDLE = 0,
    ISHTP_HBM_START,
    ISHTP_HBM_STARTED,
    ISHTP_HBM_ENUM_CLIENTS,
    ISHTP_HBM_CLIENT_PROPERTIES,
    ISHTP_HBM_WORKING,
    ISHTP_HBM_STOPPED,
}

    pub 0: hdr->host_addr =,
    pub 0: hdr->fw_addr =,
    pub length: hdr->length =,
    pub 1: hdr->msg_complete =,
    pub 0: hdr->reserved =,
    pub dev): *mut int ishtp_hbm_start_req(struct ishtp_device,
    pub dev): *mut int ishtp_hbm_start_wait(struct ishtp_device,
    pub cl): *mut ishtp_cl,
    pub cl): *mut *mut int ishtp_hbm_cl_disconnect_req(struct ishtp_device dev, struct ishtp_cl,
    pub cl): *mut *mut int ishtp_hbm_cl_connect_req(struct ishtp_device dev, struct ishtp_cl,
    pub dev): *mut void ishtp_hbm_enum_clients_req(struct ishtp_device,
    pub work): *mut void bh_hbm_work_fn(struct work_struct,
    pub ishtp_hdr): *mut *mut void recv_hbm(struct ishtp_device dev, struct ishtp_msg_hdr,
    pub ishtp_hdr): *mut ishtp_msg_hdr,
    pub hdr): *mut ishtp_bus_message,
    pub dev): *mut void ishtp_query_subscribers(struct ishtp_device,
// Exported I/F
    pub dev): *mut void ishtp_send_suspend(struct ishtp_device,
    pub dev): *mut void ishtp_send_resume(struct ishtp_device,
