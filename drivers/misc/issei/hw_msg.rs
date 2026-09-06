//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/issei/hw_msg.h
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
// Copyright (C) 2023-2026 Intel Corporation

pub const HAM_CB_MESSAGE_ID_REQ: c_uint = 0x8086cafe;
pub const HAM_CB_MESSAGE_ID_RES: c_uint = 0xcafe8086;
pub const HAM_CB_MESSAGE_VER: c_uint = 0x1;
//
// struct ham_setup_shared_memory_req - shared memory setup request
// @msg_id: message id, should be %HAM_CB_MESSAGE_ID_REQ
// @ver: message version (%HAM_CB_MESSAGE_VER)
// @reserved: reserved
// @buffer_physical_address: physical address of DMA buffer
// @host_to_fw_section_length: memory size for host to fw communication
// @fw_to_host_section_length: memory size for fw to host communication
// @control_length: memory size for control buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ham_setup_shared_memory_req {
    pub msg_id: u32,
    pub ver: u16,
    pub reserved: u16,
    pub buffer_physical_address: u64,
    pub host_to_fw_section_length: u32,
    pub fw_to_host_section_length: u32,
    pub control_length: u32,
    pub __aligned(4): } __packed,
//
// struct ham_setup_shared_memory_res - shared memory setup response
// @msg_id: message id, should be %HAM_CB_MESSAGE_ID_RES
// @status: operation status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ham_setup_shared_memory_res {
    pub msg_id: u32,
    pub status: u32,
}

//
// struct control_buffer - control buffer structure
// @h2f_counter_wr: write counter host to fw
// @h2f_counter_rd: read counter host to fw
// @f2h_counter_wr: write counter fw to host
// @f2h_counter_rd: read counter fw to host
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_buffer {
    pub h2f_counter_wr: u32,
    pub h2f_counter_rd: u32,
    pub f2h_counter_wr: u32,
    pub f2h_counter_rd: u32,
}

// HAM messages over DMA
//
// struct ham_message_header - message header over DMA
// @length: message length (payload only, not including header)
// @fw_id: firmware client id (0 means Bus Message)
// @host_id: host client id (0 means Bus Message)
// @flags: message flags
// @status: operation status
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ham_message_header {
    pub length: u32,
    pub fw_id: u16,
    pub host_id: u16,
    pub flags: u32,
    pub status: u32,
    pub reserved: u32,
}

// Bus Commands
pub const HAM_BUS_CMD_START_REQ: c_uint = 0x00;
pub const HAM_BUS_CMD_START_RSP: c_uint = 0x80;
pub const HAM_BUS_CMD_CLIENT_REQ: c_uint = 0x01;
pub const HAM_BUS_CMD_CLIENT_RSP: c_uint = 0x81;
//
// struct ham_bus_message - bus message header
// @cmd: command code
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ham_bus_message {
    pub cmd: u32,
}

pub const HAM_SUPPORTED_VERSION: c_uint = 0x01;
//
// struct ham_start_message_req - start message
// @header: bus message header (%HAM_BUS_CMD_START_REQ)
// @supported_version: supported protocol version
// @heci_capabilities_length: protocol capabilities length in bytes
// @heci_capabilities: protocol capabilities data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ham_start_message_req {
    pub header: ham_bus_message,
    pub supported_version: u16,
    pub heci_capabilities_length: u8,
    pub __counted_by(heci_capabilities_length): u8 heci_capabilities[],
    pub __packed: },
//
// struct ham_start_message_res - start message response
// @header: bus message header (%HAM_BUS_CMD_START_RSP)
// @fw_version: firmware version (four u16 blocks)
// @supported_version: supported protocol version
// @heci_capabilities_length: protocol capabilities length in bytes
// @heci_capabilities: protocol capabilities data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ham_start_message_res {
    pub header: ham_bus_message,
    pub fw_version: [u16; 4],
    pub supported_version: u16,
    pub heci_capabilities_length: u8,
    pub __counted_by(heci_capabilities_length): u8 heci_capabilities[],
    pub __packed: },
//
// struct ham_get_clients_req - clients list request
// @header: bus message header (%HAM_BUS_CMD_CLIENT_REQ)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ham_get_clients_req {
    pub header: ham_bus_message,
}

//
// struct ham_client_properties - single client properties
// @client_number: client id in firmware
// @protocol_ver: client protocol version
// @reserved: reserved
// @client_uuid: protocol name (UUID)
// @client_mtu: max message length supported by client
// @flags: client flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ham_client_properties {
    pub client_number: u16,
    pub protocol_ver: u8,
    pub reserved: u8,
    pub client_uuid: uuid_t,
    pub client_mtu: u32,
    pub flags: u32,
}

//
// struct ham_get_clients_res - client properties response
// @header: bus message header (%HAM_BUS_CMD_CLIENT_RSP)
// @client_count: number of clients in firmware
// @reserved: reserved
// @clients_props: list of client properties
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ham_get_clients_res {
    pub header: ham_bus_message,
    pub client_count: u16,
    pub reserved: u16,
    pub __counted_by(client_count): ham_client_properties clients_props[],
}
