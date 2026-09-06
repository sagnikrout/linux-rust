//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/efa/efa_admin_defs.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright 2018-2026 Amazon.com, Inc. or its affiliates. All rights reserved.
//
pub const EFA_ADMIN_API_VERSION_MAJOR: c_int = 0;
pub const EFA_ADMIN_API_VERSION_MINOR: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_admin_aq_completion_status {
    EFA_ADMIN_SUCCESS                           = 0,
    EFA_ADMIN_RESOURCE_ALLOCATION_FAILURE       = 1,
    EFA_ADMIN_BAD_OPCODE                        = 2,
    EFA_ADMIN_UNSUPPORTED_OPCODE                = 3,
    EFA_ADMIN_MALFORMED_REQUEST                 = 4,
    EFA_ADMIN_ILLEGAL_PARAMETER                 = 5,
    EFA_ADMIN_UNKNOWN_ERROR                     = 6,
    EFA_ADMIN_RESOURCE_BUSY                     = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_aq_common_desc {
//
// 11:0 : command_id
// 15:12 : reserved12
//
    pub command_id: u16,
// as appears in efa_admin_aq_opcode
    pub opcode: u8,
//
// 0 : phase
// 1 : ctrl_data - control buffer address valid
// 2 : ctrl_data_indirect - control buffer address
// points to list of pages with addresses of control
// buffers
// 7:3 : reserved3
//
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_aq_common_desc_v2 {
    pub common: efa_admin_aq_common_desc,
//
// Poly 0x8005 CRC16 with initial value 0xFFFF and final XOR of
// 0xFFFF. The checksum covers the entire admin command entry
// including the zeroed checksum field.
//
    pub checksum: u16,
    pub payload_ver: u8,
    pub reserved: [u8; 5],
}

//
// used in efa_admin_aq_entry. Can point directly to control data, or to a
// page list chunk. Used also at the end of indirect mode page list chunks,
// for chaining.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_ctrl_buff_info {
    pub length: u32,
    pub address: efa_common_mem_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_aq_entry {
    pub aq_common_descriptor: efa_admin_aq_common_desc,
    pub request_payload: [u32; 15],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_aq_entry_v2 {
    pub aq_common_descriptor: efa_admin_aq_common_desc_v2,
    pub request_payload: [u32; 29],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_acq_common_desc {
//
// command identifier to associate it with the aq descriptor
// 11:0 : command_id
// 15:12 : reserved12
//
    pub command: u16,
    pub status: u8,
//
// 0 : phase
// 7:1 : reserved1
//
    pub flags: u8,
//
// Poly 0x8005 CRC16 with initial value 0xFFFF and final XOR of 0xFFFF.
// The checksum covers the entire admin completion entry including the
// zeroed checksum field.
//
    pub checksum: u16,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_acq_entry {
    pub acq_common_descriptor: efa_admin_acq_common_desc,
    pub response_specific_data: [u32; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_aenq_common_desc {
    pub group: u16,
    pub syndrome: u16,
//
// 0 : phase
// 7:1 : reserved - MBZ
//
    pub flags: u8,
    pub reserved1: [u8; 3],
    pub timestamp_low: u32,
    pub timestamp_high: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_aenq_entry {
    pub aenq_common_desc: efa_admin_aenq_common_desc,
// command specific inline data
    pub inline_data_w4: [u32; 12],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efa_admin_eqe_event_type {
    EFA_ADMIN_EQE_EVENT_TYPE_COMPLETION         = 0,
}

// Completion event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_comp_event {
// CQ number
    pub cqn: u16,
// MBZ
    pub reserved: u16,
// MBZ
    pub reserved2: u32,
}

// Event Queue Element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_admin_eqe {
//
// 0 : phase
// 8:1 : event_type - Event type
// 31:9 : reserved - MBZ
//
    pub common: u32,
// MBZ
    pub reserved: u32,
// Event data
    pub event_data: [u32; 2],
// Completion Event
    pub comp_event: efa_admin_comp_event,
    pub u: },
}

// aq_common_desc

// acq_common_desc

// aenq_common_desc

// eqe

