//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/mei/hw.h
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
// Copyright (c) 2003-2022, Intel Corporation. All rights reserved
// Intel Management Engine Interface (Intel MEI) Linux driver
//

//
// Timeouts in Seconds
//

//
// FW page size for DMA allocations
//

//
// MEI Version
//
pub const HBM_MINOR_VERSION: c_int = 2;
pub const HBM_MAJOR_VERSION: c_int = 2;
//
// MEI version with PGI support
//
pub const HBM_MINOR_VERSION_PGI: c_int = 1;
pub const HBM_MAJOR_VERSION_PGI: c_int = 1;
//
// MEI version with Dynamic clients support
//
pub const HBM_MINOR_VERSION_DC: c_int = 0;
pub const HBM_MAJOR_VERSION_DC: c_int = 2;
//
// MEI version with immediate reply to enum request support
//
pub const HBM_MINOR_VERSION_IE: c_int = 0;
pub const HBM_MAJOR_VERSION_IE: c_int = 2;
//
// MEI version with disconnect on connection timeout support
//
pub const HBM_MINOR_VERSION_DOT: c_int = 0;
pub const HBM_MAJOR_VERSION_DOT: c_int = 2;
//
// MEI version with notification support
//
pub const HBM_MINOR_VERSION_EV: c_int = 0;
pub const HBM_MAJOR_VERSION_EV: c_int = 2;
//
// MEI version with fixed address client support
//
pub const HBM_MINOR_VERSION_FA: c_int = 0;
pub const HBM_MAJOR_VERSION_FA: c_int = 2;
//
// MEI version with OS ver message support
//
pub const HBM_MINOR_VERSION_OS: c_int = 0;
pub const HBM_MAJOR_VERSION_OS: c_int = 2;
//
// MEI version with dma ring support
//
pub const HBM_MINOR_VERSION_DR: c_int = 1;
pub const HBM_MAJOR_VERSION_DR: c_int = 2;
//
// MEI version with vm tag support
//
pub const HBM_MINOR_VERSION_VT: c_int = 2;
pub const HBM_MAJOR_VERSION_VT: c_int = 2;
//
// MEI version with GSC support
//
pub const HBM_MINOR_VERSION_GSC: c_int = 2;
pub const HBM_MAJOR_VERSION_GSC: c_int = 2;
//
// MEI version with capabilities message support
//
pub const HBM_MINOR_VERSION_CAP: c_int = 2;
pub const HBM_MAJOR_VERSION_CAP: c_int = 2;
//
// MEI version with client DMA support
//
pub const HBM_MINOR_VERSION_CD: c_int = 2;
pub const HBM_MAJOR_VERSION_CD: c_int = 2;
// Host bus message command opcode
pub const MEI_HBM_CMD_OP_MSK: c_uint = 0x7f;
// Host bus message command RESPONSE
pub const MEI_HBM_CMD_RES_MSK: c_uint = 0x80;
//
// MEI Bus Message Command IDs
//
pub const HOST_START_REQ_CMD: c_uint = 0x01;
pub const HOST_START_RES_CMD: c_uint = 0x81;
pub const HOST_STOP_REQ_CMD: c_uint = 0x02;
pub const HOST_STOP_RES_CMD: c_uint = 0x82;
pub const ME_STOP_REQ_CMD: c_uint = 0x03;
pub const HOST_ENUM_REQ_CMD: c_uint = 0x04;
pub const HOST_ENUM_RES_CMD: c_uint = 0x84;
pub const HOST_CLIENT_PROPERTIES_REQ_CMD: c_uint = 0x05;
pub const HOST_CLIENT_PROPERTIES_RES_CMD: c_uint = 0x85;
pub const CLIENT_CONNECT_REQ_CMD: c_uint = 0x06;
pub const CLIENT_CONNECT_RES_CMD: c_uint = 0x86;
pub const CLIENT_DISCONNECT_REQ_CMD: c_uint = 0x07;
pub const CLIENT_DISCONNECT_RES_CMD: c_uint = 0x87;
pub const MEI_FLOW_CONTROL_CMD: c_uint = 0x08;
pub const MEI_PG_ISOLATION_ENTRY_REQ_CMD: c_uint = 0x0a;
pub const MEI_PG_ISOLATION_ENTRY_RES_CMD: c_uint = 0x8a;
pub const MEI_PG_ISOLATION_EXIT_REQ_CMD: c_uint = 0x0b;
pub const MEI_PG_ISOLATION_EXIT_RES_CMD: c_uint = 0x8b;
pub const MEI_HBM_ADD_CLIENT_REQ_CMD: c_uint = 0x0f;
pub const MEI_HBM_ADD_CLIENT_RES_CMD: c_uint = 0x8f;
pub const MEI_HBM_NOTIFY_REQ_CMD: c_uint = 0x10;
pub const MEI_HBM_NOTIFY_RES_CMD: c_uint = 0x90;
pub const MEI_HBM_NOTIFICATION_CMD: c_uint = 0x11;
pub const MEI_HBM_DMA_SETUP_REQ_CMD: c_uint = 0x12;
pub const MEI_HBM_DMA_SETUP_RES_CMD: c_uint = 0x92;
pub const MEI_HBM_CAPABILITIES_REQ_CMD: c_uint = 0x13;
pub const MEI_HBM_CAPABILITIES_RES_CMD: c_uint = 0x93;
pub const MEI_HBM_CLIENT_DMA_MAP_REQ_CMD: c_uint = 0x14;
pub const MEI_HBM_CLIENT_DMA_MAP_RES_CMD: c_uint = 0x94;
pub const MEI_HBM_CLIENT_DMA_UNMAP_REQ_CMD: c_uint = 0x15;
pub const MEI_HBM_CLIENT_DMA_UNMAP_RES_CMD: c_uint = 0x95;
//
// MEI Stop Reason
// used by hbm_host_stop_request.reason
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mei_stop_reason_types {
    DRIVER_STOP_REQUEST = 0x00,
    DEVICE_D1_ENTRY = 0x01,
    DEVICE_D2_ENTRY = 0x02,
    DEVICE_D3_ENTRY = 0x03,
    SYSTEM_S1_ENTRY = 0x04,
    SYSTEM_S2_ENTRY = 0x05,
    SYSTEM_S3_ENTRY = 0x06,
    SYSTEM_S4_ENTRY = 0x07,
    SYSTEM_S5_ENTRY = 0x08
}

//
// enum mei_hbm_status  - mei host bus messages return values
//
// @MEI_HBMS_SUCCESS           : status success
// @MEI_HBMS_CLIENT_NOT_FOUND  : client not found
// @MEI_HBMS_ALREADY_EXISTS    : connection already established
// @MEI_HBMS_REJECTED          : connection is rejected
// @MEI_HBMS_INVALID_PARAMETER : invalid parameter
// @MEI_HBMS_NOT_ALLOWED       : operation not allowed
// @MEI_HBMS_ALREADY_STARTED   : system is already started
// @MEI_HBMS_NOT_STARTED       : system not started
//
// @MEI_HBMS_MAX               : sentinel
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mei_hbm_status {
    MEI_HBMS_SUCCESS           = 0,
    MEI_HBMS_CLIENT_NOT_FOUND  = 1,
    MEI_HBMS_ALREADY_EXISTS    = 2,
    MEI_HBMS_REJECTED          = 3,
    MEI_HBMS_INVALID_PARAMETER = 4,
    MEI_HBMS_NOT_ALLOWED       = 5,
    MEI_HBMS_ALREADY_STARTED   = 6,
    MEI_HBMS_NOT_STARTED       = 7,

    MEI_HBMS_MAX
}

//
// Client Connect Status
// used by hbm_client_connect_response.status
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mei_cl_connect_status {
    MEI_CL_CONN_SUCCESS          = MEI_HBMS_SUCCESS,
    MEI_CL_CONN_NOT_FOUND        = MEI_HBMS_CLIENT_NOT_FOUND,
    MEI_CL_CONN_ALREADY_STARTED  = MEI_HBMS_ALREADY_EXISTS,
    MEI_CL_CONN_OUT_OF_RESOURCES = MEI_HBMS_REJECTED,
    MEI_CL_CONN_MESSAGE_SMALL    = MEI_HBMS_INVALID_PARAMETER,
    MEI_CL_CONN_NOT_ALLOWED      = MEI_HBMS_NOT_ALLOWED,
}

//
// Client Disconnect Status
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mei_cl_disconnect_status {
    MEI_CL_DISCONN_SUCCESS = MEI_HBMS_SUCCESS
}

//
// enum mei_ext_hdr_type - extended header type used in
// extended header TLV
//
// @MEI_EXT_HDR_NONE: sentinel
// @MEI_EXT_HDR_VTAG: vtag header
// @MEI_EXT_HDR_GSC: gsc header
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mei_ext_hdr_type {
    MEI_EXT_HDR_NONE = 0,
    MEI_EXT_HDR_VTAG = 1,
    MEI_EXT_HDR_GSC = 2,
}

//
// struct mei_ext_hdr - extend header descriptor (TLV)
// @type: enum mei_ext_hdr_type
// @length: length excluding descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_ext_hdr {
    pub type: u8,
    pub length: u8,
    pub __packed: },
//
// struct mei_ext_meta_hdr - extend header meta data
// @count: number of headers
// @size: total size of the extended header list excluding meta header
// @reserved: reserved
// @hdrs: extended headers TLV list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_ext_meta_hdr {
    pub count: u8,
    pub size: u8,
    pub reserved: [u8; 2],
    pub hdrs: [u8; ],
    pub __packed: },
//
// struct mei_ext_hdr_vtag - extend header for vtag
//
// @hdr: standard extend header
// @vtag: virtual tag
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_ext_hdr_vtag {
    pub hdr: mei_ext_hdr,
    pub vtag: u8,
    pub reserved: u8,
    pub __packed: },
//
// Extended header iterator functions
//
// mei_ext_begin - extended header iterator begin
//
// @meta: meta header of the extended header list
//
// Return: The first extended header
//
    pub )meta->hdrs: *mut return (struct mei_ext_hdr,
//
// mei_ext_last - check if the ext is the last one in the TLV list
//
// @meta: meta header of the extended header list
// @ext: a meta header on the list
//
// Return: true if ext is the last header on the list
//
    pub 4): *mut *mut *mut *mut *mut return (u8 )ext >= (u8 )meta + sizeof(meta) + (meta->size,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_gsc_sgl {
    pub low: u32,
    pub high: u32,
    pub length: u32,
    pub __packed: },
pub const GSC_HECI_MSG_KERNEL: c_int = 0;
pub const GSC_HECI_MSG_USER: c_int = 1;
pub const GSC_ADDRESS_TYPE_GTT: c_int = 0;
pub const GSC_ADDRESS_TYPE_PPGTT: c_int = 1;

pub const GSC_ADDRESS_TYPE_PHYSICAL_SGL: c_int = 3;
//
// struct mei_ext_hdr_gsc_h2f - extended header: gsc host to firmware interface
//
// @hdr: extended header
// @client_id: GSC_HECI_MSG_KERNEL or GSC_HECI_MSG_USER
// @addr_type: GSC_ADDRESS_TYPE_{GTT, PPGTT, PHYSICAL_CONTINUOUS, PHYSICAL_SGL}
// @fence_id: synchronization marker
// @input_address_count: number of input sgl buffers
// @output_address_count: number of output sgl buffers
// @reserved: reserved
// @sgl: sg list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_ext_hdr_gsc_h2f {
    pub hdr: mei_ext_hdr,
    pub client_id: u8,
    pub addr_type: u8,
    pub fence_id: u32,
    pub input_address_count: u8,
    pub output_address_count: u8,
    pub reserved: [u8; 2],
    pub sgl: [mei_gsc_sgl; ],
    pub __packed: },
//
// struct mei_ext_hdr_gsc_f2h - gsc firmware to host interface
//
// @hdr: extended header
// @client_id: GSC_HECI_MSG_KERNEL or GSC_HECI_MSG_USER
// @reserved: reserved
// @fence_id: synchronization marker
// @written: number of bytes written to firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_ext_hdr_gsc_f2h {
    pub hdr: mei_ext_hdr,
    pub client_id: u8,
    pub reserved: u8,
    pub fence_id: u32,
    pub written: u32,
    pub __packed: },
//
// mei_ext_next - following extended header on the TLV list
//
// @ext: current extend header
//
// Context: The function does not check for the overflows,
// one should call mei_ext_last before.
//
// Return: The following extend header after @ext
//
    pub 4)): *mut *mut *mut *mut return (struct mei_ext_hdr )((u8 )ext + (ext->length,
//
// mei_ext_hdr_len - get ext header length in bytes
//
// @ext: extend header
//
// Return: extend header length in bytes
//
    pub 0: return,
    pub sizeof(u32): *mut *mut return ext->length,
//
// struct mei_msg_hdr - MEI BUS Interface Section
//
// @me_addr: device address
// @host_addr: host address
// @length: message length
// @reserved: reserved
// @extended: message has extended header
// @dma_ring: message is on dma ring
// @internal: message is internal
// @msg_complete: last packet of the message
// @extension: extension of the header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_msg_hdr {
    pub me_addr:8: u32,
    pub host_addr:8: u32,
    pub length:9: u32,
    pub reserved:3: u32,
    pub extended:1: u32,
    pub dma_ring:1: u32,
    pub internal:1: u32,
    pub msg_complete:1: u32,
    pub extension: [u32; ],
    pub __packed: },
// The length is up to 9 bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_bus_message {
    pub hbm_cmd: u8,
    pub data: [u8; ],
    pub __packed: },
//
// struct mei_hbm_cl_cmd - client specific host bus command
// CONNECT, DISCONNECT, and FlOW CONTROL
//
// @hbm_cmd: bus message command header
// @me_addr: address of the client in ME
// @host_addr: address of the client in the driver
// @data: generic data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_hbm_cl_cmd {
    pub hbm_cmd: u8,
    pub me_addr: u8,
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
    pub me_max_version: hbm_version,
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
pub struct hbm_me_stop_request {
    pub hbm_cmd: u8,
    pub reason: u8,
    pub reserved: [u8; 2],
    pub __packed: },
//
// enum hbm_host_enum_flags - enumeration request flags (HBM version >= 2.0)
//
// @MEI_HBM_ENUM_F_ALLOW_ADD: allow dynamic clients add
// @MEI_HBM_ENUM_F_IMMEDIATE_ENUM: allow FW to send answer immediately
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hbm_host_enum_flags {
    MEI_HBM_ENUM_F_ALLOW_ADD = BIT(0),
    MEI_HBM_ENUM_F_IMMEDIATE_ENUM = BIT(1),
}

//
// struct hbm_host_enum_request - enumeration request from host to fw
//
// @hbm_cmd : bus message command header
// @flags   : request flags
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_host_enum_request {
    pub hbm_cmd: u8,
    pub flags: u8,
    pub reserved: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_host_enum_response {
    pub hbm_cmd: u8,
    pub reserved: [u8; 3],
    pub valid_addresses: [u8; 32],
    pub __packed: },
//
// struct mei_client_properties - mei client properties
//
// @protocol_name: guid of the client
// @protocol_version: client protocol version
// @max_number_of_connections: number of possible connections.
// @fixed_address: fixed me address (0 if the client is dynamic)
// @single_recv_buf: 1 if all connections share a single receive buffer.
// @vt_supported: the client support vtag
// @reserved: reserved
// @max_msg_length: MTU of the client
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_client_properties {
    pub protocol_name: uuid_le,
    pub protocol_version: u8,
    pub max_number_of_connections: u8,
    pub fixed_address: u8,
    pub single_recv_buf:1: u8,
    pub vt_supported:1: u8,
    pub reserved:6: u8,
    pub max_msg_length: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_props_request {
    pub hbm_cmd: u8,
    pub me_addr: u8,
    pub reserved: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_props_response {
    pub hbm_cmd: u8,
    pub me_addr: u8,
    pub status: u8,
    pub reserved: u8,
    pub client_properties: mei_client_properties,
    pub __packed: },
//
// struct hbm_add_client_request - request to add a client
// might be sent by fw after enumeration has already completed
//
// @hbm_cmd: bus message command header
// @me_addr: address of the client in ME
// @reserved: reserved
// @client_properties: client properties
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_add_client_request {
    pub hbm_cmd: u8,
    pub me_addr: u8,
    pub reserved: [u8; 2],
    pub client_properties: mei_client_properties,
    pub __packed: },
//
// struct hbm_add_client_response - response to add a client
// sent by the host to report client addition status to fw
//
// @hbm_cmd: bus message command header
// @me_addr: address of the client in ME
// @status: if HBMS_SUCCESS then the client can now accept connections.
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_add_client_response {
    pub hbm_cmd: u8,
    pub me_addr: u8,
    pub status: u8,
    pub reserved: u8,
    pub __packed: },
//
// struct hbm_power_gate - power gate request/response
//
// @hbm_cmd: bus message command header
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_power_gate {
    pub hbm_cmd: u8,
    pub reserved: [u8; 3],
    pub __packed: },
//
// struct hbm_client_connect_request - connect/disconnect request
//
// @hbm_cmd: bus message command header
// @me_addr: address of the client in ME
// @host_addr: address of the client in the driver
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_client_connect_request {
    pub hbm_cmd: u8,
    pub me_addr: u8,
    pub host_addr: u8,
    pub reserved: u8,
    pub __packed: },
//
// struct hbm_client_connect_response - connect/disconnect response
//
// @hbm_cmd: bus message command header
// @me_addr: address of the client in ME
// @host_addr: address of the client in the driver
// @status: status of the request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_client_connect_response {
    pub hbm_cmd: u8,
    pub me_addr: u8,
    pub host_addr: u8,
    pub status: u8,
    pub __packed: },
pub const MEI_FC_MESSAGE_RESERVED_LENGTH: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_flow_control {
    pub hbm_cmd: u8,
    pub me_addr: u8,
    pub host_addr: u8,
    pub reserved: [u8; MEI_FC_MESSAGE_RESERVED_LENGTH],
    pub __packed: },
pub const MEI_HBM_NOTIFICATION_START: c_int = 1;
pub const MEI_HBM_NOTIFICATION_STOP: c_int = 0;
//
// struct hbm_notification_request - start/stop notification request
//
// @hbm_cmd: bus message command header
// @me_addr: address of the client in ME
// @host_addr: address of the client in the driver
// @start:  start = 1 or stop = 0 asynchronous notifications
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_notification_request {
    pub hbm_cmd: u8,
    pub me_addr: u8,
    pub host_addr: u8,
    pub start: u8,
    pub __packed: },
//
// struct hbm_notification_response - start/stop notification response
//
// @hbm_cmd: bus message command header
// @me_addr: address of the client in ME
// @host_addr: - address of the client in the driver
// @status: (mei_hbm_status) response status for the request
// - MEI_HBMS_SUCCESS: successful stop/start
// - MEI_HBMS_CLIENT_NOT_FOUND: if the connection could not be found.
// - MEI_HBMS_ALREADY_STARTED: for start requests for a previously
// started notification.
// - MEI_HBMS_NOT_STARTED: for stop request for a connected client for whom
// asynchronous notifications are currently disabled.
//
// @start:  start = 1 or stop = 0 asynchronous notifications
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_notification_response {
    pub hbm_cmd: u8,
    pub me_addr: u8,
    pub host_addr: u8,
    pub status: u8,
    pub start: u8,
    pub reserved: [u8; 3],
    pub __packed: },
//
// struct hbm_notification - notification event
//
// @hbm_cmd: bus message command header
// @me_addr:  address of the client in ME
// @host_addr:  address of the client in the driver
// @reserved: reserved for alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_notification {
    pub hbm_cmd: u8,
    pub me_addr: u8,
    pub host_addr: u8,
    pub reserved: u8,
    pub __packed: },
//
// struct hbm_dma_mem_dscr - dma ring
//
// @addr_hi: the high 32bits of 64 bit address
// @addr_lo: the low  32bits of 64 bit address
// @size   : size in bytes (must be power of 2)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_dma_mem_dscr {
    pub addr_hi: u32,
    pub addr_lo: u32,
    pub size: u32,
    pub __packed: },
}

//
// struct hbm_dma_setup_request - dma setup request
//
// @hbm_cmd: bus message command header
// @reserved: reserved for alignment
// @dma_dscr: dma descriptor for HOST, DEVICE, and CTRL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_dma_setup_request {
    pub hbm_cmd: u8,
    pub reserved: [u8; 3],
    pub dma_dscr: [hbm_dma_mem_dscr; DMA_DSCR_NUM],
    pub __packed: },
//
// struct hbm_dma_setup_response - dma setup response
//
// @hbm_cmd: bus message command header
// @status: 0 on success; otherwise DMA setup failed.
// @reserved: reserved for alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_dma_setup_response {
    pub hbm_cmd: u8,
    pub status: u8,
    pub reserved: [u8; 2],
    pub __packed: },
//
// struct hbm_dma_ring_ctrl - dma ring control block
//
// @hbuf_wr_idx: host circular buffer write index in slots
// @reserved1: reserved for alignment
// @hbuf_rd_idx: host circular buffer read index in slots
// @reserved2: reserved for alignment
// @dbuf_wr_idx: device circular buffer write index in slots
// @reserved3: reserved for alignment
// @dbuf_rd_idx: device circular buffer read index in slots
// @reserved4: reserved for alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_dma_ring_ctrl {
    pub hbuf_wr_idx: u32,
    pub reserved1: u32,
    pub hbuf_rd_idx: u32,
    pub reserved2: u32,
    pub dbuf_wr_idx: u32,
    pub reserved3: u32,
    pub dbuf_rd_idx: u32,
    pub reserved4: u32,
    pub __packed: },
// virtual tag supported

// gsc extended header support

// client dma supported

//
// struct hbm_capability_request - capability request from host to fw
//
// @hbm_cmd : bus message command header
// @capability_requested: bitmask of capabilities requested by host
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_capability_request {
    pub hbm_cmd: u8,
    pub capability_requested: [u8; 3],
    pub __packed: },
//
// struct hbm_capability_response - capability response from fw to host
//
// @hbm_cmd : bus message command header
// @capability_granted: bitmask of capabilities granted by FW
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_capability_response {
    pub hbm_cmd: u8,
    pub capability_granted: [u8; 3],
    pub __packed: },
//
// struct hbm_client_dma_map_request - client dma map request from host to fw
//
// @hbm_cmd: bus message command header
// @client_buffer_id: client buffer id
// @reserved: reserved
// @address_lsb: DMA address LSB
// @address_msb: DMA address MSB
// @size: DMA size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_client_dma_map_request {
    pub hbm_cmd: u8,
    pub client_buffer_id: u8,
    pub reserved: [u8; 2],
    pub address_lsb: u32,
    pub address_msb: u32,
    pub size: u32,
    pub __packed: },
//
// struct hbm_client_dma_unmap_request - client dma unmap request
// from the host to the firmware
//
// @hbm_cmd: bus message command header
// @status: unmap status
// @client_buffer_id: client buffer id
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_client_dma_unmap_request {
    pub hbm_cmd: u8,
    pub status: u8,
    pub client_buffer_id: u8,
    pub reserved: u8,
    pub __packed: },
//
// struct hbm_client_dma_response - client dma unmap response
// from the firmware to the host
//
// @hbm_cmd: bus message command header
// @status: command status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_client_dma_response {
    pub hbm_cmd: u8,
    pub status: u8,
    pub __packed: },
