//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/viosrp.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// srp.h -- SCSI RDMA Protocol definitions
//
// Written By: Colin Devilbis, IBM Corporation
//
// Copyright (C) 2003 IBM Corporation
//
// This file contains structures and definitions for IBM RPA (RS/6000
// platform architecture) implementation of the SRP (SCSI RDMA Protocol)
// standard.  SRP is used on IBM iSeries and pSeries platforms to send SCSI
// commands between logical partitions.
//
// SRP Information Units (IUs) are sent on a "Command/Response Queue" (CRQ)
// between partitions.  The definitions in this file are architected,
// and cannot be changed without breaking compatibility with other versions
// of Linux and other operating systems (AIX, OS/400) that talk this protocol
// between logical partitions
//

pub const SRP_MAX_IU_LEN: c_int = 256;
pub const SRP_MAX_LOC_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub union srp_iu {
    pub login_req: srp_login_req,
    pub login_rsp: srp_login_rsp,
    pub login_rej: srp_login_rej,
    pub i_logout: srp_i_logout,
    pub t_logout: srp_t_logout,
    pub tsk_mgmt: srp_tsk_mgmt,
    pub cmd: srp_cmd,
    pub rsp: srp_rsp,
    pub reserved: [u8; SRP_MAX_IU_LEN],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum viosrp_crq_headers {
    VIOSRP_CRQ_FREE = 0x00,
    VIOSRP_CRQ_CMD_RSP = 0x80,
    VIOSRP_CRQ_INIT_RSP = 0xC0,
    VIOSRP_CRQ_XPORT_EVENT = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum viosrp_crq_init_formats {
    VIOSRP_CRQ_INIT = 0x01,
    VIOSRP_CRQ_INIT_COMPLETE = 0x02
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum viosrp_crq_formats {
    VIOSRP_SRP_FORMAT = 0x01,
    VIOSRP_MAD_FORMAT = 0x02,
    VIOSRP_OS400_FORMAT = 0x03,
    VIOSRP_AIX_FORMAT = 0x04,
    VIOSRP_LINUX_FORMAT = 0x05,
    VIOSRP_INLINE_FORMAT = 0x06
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum viosrp_crq_status {
    VIOSRP_OK = 0x0,
    VIOSRP_NONRECOVERABLE_ERR = 0x1,
    VIOSRP_VIOLATES_MAX_XFER = 0x2,
    VIOSRP_PARTNER_PANIC = 0x3,
    VIOSRP_DEVICE_BUSY = 0x8,
    VIOSRP_ADAPTER_FAIL = 0x10,
    VIOSRP_OK2 = 0x99,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct viosrp_crq {
    pub /: *mut *mut __be64 high; / High 64 bits,
    pub /: *mut *mut u8 valid; / used by RPA,
    pub /: *mut *mut u8 format; / SCSI vs out-of-band,
    pub reserved: u8,
    pub /: *mut *mut u8 status; / non-scsi failure? (e.g. DMA failure),
    pub /: *mut *mut __be16 timeout; / in seconds,
    pub /: *mut *mut __be16 IU_length; / in bytes,
}

// MADs are Management requests above and beyond the IUs defined in the SRP
// standard.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum viosrp_mad_types {
    VIOSRP_EMPTY_IU_TYPE = 0x01,
    VIOSRP_ERROR_LOG_TYPE = 0x02,
    VIOSRP_ADAPTER_INFO_TYPE = 0x03,
    VIOSRP_CAPABILITIES_TYPE = 0x05,
    VIOSRP_ENABLE_FAST_FAIL = 0x08,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum viosrp_mad_status {
    VIOSRP_MAD_SUCCESS = 0x00,
    VIOSRP_MAD_NOT_SUPPORTED = 0xF1,
    VIOSRP_MAD_FAILED = 0xF7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum viosrp_capability_type {
    MIGRATION_CAPABILITIES = 0x01,
    RESERVATION_CAPABILITIES = 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum viosrp_capability_support {
    SERVER_DOES_NOT_SUPPORTS_CAP = 0x0,
    SERVER_SUPPORTS_CAP = 0x01,
    SERVER_CAP_DATA = 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum viosrp_reserve_type {
    CLIENT_RESERVE_SCSI_2 = 0x01,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum viosrp_capability_flag {
    CLIENT_MIGRATED = 0x01,
    CLIENT_RECONNECT = 0x02,
    CAP_LIST_SUPPORTED = 0x04,
    CAP_LIST_DATA = 0x08,
}

//
// Common MAD header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mad_common {
    pub type: __be32,
    pub status: __be16,
    pub length: __be16,
    pub tag: __be64,
}

//
// All SRP (and MAD) requests normally flow from the
// client to the server.  There is no way for the server to send
// an asynchronous message back to the client.  The Empty IU is used
// to hang out a meaningless request to the server so that it can respond
// asynchrouously with something like a SCSI AER
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct viosrp_empty_iu {
    pub common: mad_common,
    pub buffer: __be64,
    pub port: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct viosrp_error_log {
    pub common: mad_common,
    pub buffer: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct viosrp_adapter_info {
    pub common: mad_common,
    pub buffer: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct viosrp_fast_fail {
    pub common: mad_common,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct viosrp_capabilities {
    pub common: mad_common,
    pub buffer: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mad_capability_common {
    pub cap_type: __be32,
    pub length: __be16,
    pub server_support: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mad_reserve_cap {
    pub common: mad_capability_common,
    pub type: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mad_migration_cap {
    pub common: mad_capability_common,
    pub ecl: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct capabilities {
    pub flags: __be32,
    pub name: [c_char; SRP_MAX_LOC_LEN],
    pub loc: [c_char; SRP_MAX_LOC_LEN],
    pub migration: mad_migration_cap,
    pub reserve: mad_reserve_cap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mad_iu {
    pub empty_iu: viosrp_empty_iu,
    pub error_log: viosrp_error_log,
    pub adapter_info: viosrp_adapter_info,
    pub fast_fail: viosrp_fast_fail,
    pub capabilities: viosrp_capabilities,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union viosrp_iu {
    pub srp: srp_iu,
    pub mad: mad_iu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mad_adapter_info_data {
    pub srp_version: [c_char; 8],
    pub partition_name: [c_char; 96],
    pub partition_number: __be32,
pub const SRP_MAD_VERSION_1: c_int = 1;
    pub mad_version: __be32,
pub const SRP_MAD_OS_LINUX: c_int = 2;
pub const SRP_MAD_OS_AIX: c_int = 3;
    pub os_type: __be32,
    pub /: *mut *mut __be32 port_max_txu[8]; / per-port maximum transfer,
}
