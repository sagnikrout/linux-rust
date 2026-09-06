//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnxt/bnxt_coredump.h
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


// Broadcom NetXtreme-C/E network driver.
//
// Copyright (c) 2018 Broadcom Inc
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_coredump_segment_hdr {
    pub signature: [__u8; 4],
    pub component_id: __le32,
    pub segment_id: __le32,
    pub flags: __le32,
    pub low_version: __u8,
    pub high_version: __u8,
    pub function_id: __le16,
    pub offset: __le32,
    pub length: __le32,
    pub status: __le32,
    pub duration: __le32,
    pub data_offset: __le32,
    pub instance: __le32,
    pub rsvd: [__le32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_coredump_record {
    pub signature: [__u8; 4],
    pub flags: __le32,
    pub low_version: __u8,
    pub high_version: __u8,
    pub asic_state: __u8,
    pub rsvd0: [__u8; 5],
    pub system_name: [c_char; 32],
    pub year: __le16,
    pub month: __le16,
    pub day: __le16,
    pub hour: __le16,
    pub minute: __le16,
    pub second: __le16,
    pub utc_bias: __le16,
    pub rsvd1: __le16,
    pub commandline: [c_char; 256],
    pub total_segments: __le32,
    pub os_ver_major: __le32,
    pub os_ver_minor: __le32,
    pub rsvd2: __le32,
    pub os_name: [c_char; 32],
    pub end_year: __le16,
    pub end_month: __le16,
    pub end_day: __le16,
    pub end_hour: __le16,
    pub end_minute: __le16,
    pub end_second: __le16,
    pub end_utc_bias: __le16,
    pub asic_id1: __le32,
    pub asic_id2: __le32,
    pub coredump_status: __le32,
    pub ioctl_low_version: __u8,
    pub ioctl_high_version: __u8,
    pub rsvd3: [__le16; 313],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_driver_segment_record {
    pub max_entries: __le32,
    pub entry_size: __le32,
    pub offset: __le32,
    pub wrapped:1: __u8,
    pub unused: [__u8; 3],
}

pub const BNXT_VER_GET_COMP_ID: c_int = 2;
pub const BNXT_DRV_COMP_ID: c_uint = 0xd;
pub const BNXT_CTX_MEM_SEG_ID_START: c_uint = 0x200;

pub const BNXT_CTX_MEM_SEG_SRT: c_uint = 0x1;
pub const BNXT_CTX_MEM_SEG_SRT2: c_uint = 0x2;
pub const BNXT_CTX_MEM_SEG_CRT: c_uint = 0x3;
pub const BNXT_CTX_MEM_SEG_CRT2: c_uint = 0x4;
pub const BNXT_CTX_MEM_SEG_RIGP0: c_uint = 0x5;
pub const BNXT_CTX_MEM_SEG_L2HWRM: c_uint = 0x6;
pub const BNXT_CTX_MEM_SEG_REHWRM: c_uint = 0x7;
pub const BNXT_CTX_MEM_SEG_CA0: c_uint = 0x8;
pub const BNXT_CTX_MEM_SEG_CA1: c_uint = 0x9;
pub const BNXT_CTX_MEM_SEG_CA2: c_uint = 0xa;
pub const BNXT_CTX_MEM_SEG_RIGP1: c_uint = 0xb;
pub const BNXT_CTX_MEM_SEG_QPC: c_uint = 0xc;
pub const BNXT_CTX_MEM_SEG_KONG: c_uint = 0xd;

pub const COREDUMP_LIST_BUF_LEN: c_int = 2048;
pub const COREDUMP_RETRIEVE_BUF_LEN: c_int = 4096;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_coredump {
    pub data: *mut c_void,
    pub data_size: c_int,
    pub total_segs: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_hwrm_dbg_dma_info {
    pub dest_buf: *mut c_void,
    pub dest_buf_size: c_int,
    pub dma_len: u16,
    pub seq_off: u16,
    pub data_len_off: u16,
    pub segs: u16,
    pub seg_start: u32,
    pub buf_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwrm_dbg_cmn_input {
    pub req_type: __le16,
    pub cmpl_ring: __le16,
    pub seq_id: __le16,
    pub target_id: __le16,
    pub resp_addr: __le64,
    pub host_dest_addr: __le64,
    pub host_buf_len: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwrm_dbg_cmn_output {
    pub error_code: __le16,
    pub req_type: __le16,
    pub seq_id: __le16,
    pub resp_len: __le16,
    pub flags: u8,
pub const HWRM_DBG_CMN_FLAGS_MORE: c_int = 1;
}

extern "C" {
    pub fn bnxt_get_coredump(bp: *mut bnxt, dump_type: u16, buf: *mut c_void, dump_len: *mut u32) -> c_int;
}
extern "C" {
    pub fn bnxt_hwrm_get_dump_len(bp: *mut bnxt, dump_type: u16, dump_len: *mut u32) -> c_int;
}
extern "C" {
    pub fn bnxt_get_coredump_length(bp: *mut bnxt, dump_type: u16) -> u32;
}
