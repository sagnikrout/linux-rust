//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ndctl.h
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


//
// Copyright (c) 2014-2016, Intel Corporation.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms and conditions of the GNU Lesser General Public License,
// version 2.1, as published by the Free Software Foundation.
//
// This program is distributed in the hope it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for
// more details.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_cmd_dimm_flags {
    pub status: __u32,
    pub flags: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_cmd_get_config_size {
    pub status: __u32,
    pub config_size: __u32,
    pub max_xfer: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_cmd_get_config_data_hdr {
    pub in_offset: __u32,
    pub in_length: __u32,
    pub status: __u32,
    pub out_buf: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_cmd_set_config_hdr {
    pub in_offset: __u32,
    pub in_length: __u32,
    pub in_buf: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_cmd_vendor_hdr {
    pub opcode: __u32,
    pub in_length: __u32,
    pub in_buf: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_cmd_vendor_tail {
    pub status: __u32,
    pub out_length: __u32,
    pub out_buf: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_cmd_ars_cap {
    pub address: __u64,
    pub length: __u64,
    pub status: __u32,
    pub max_ars_out: __u32,
    pub clear_err_unit: __u32,
    pub flags: __u16,
    pub reserved: __u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_cmd_ars_start {
    pub address: __u64,
    pub length: __u64,
    pub type: __u16,
    pub flags: __u8,
    pub reserved: [__u8; 5],
    pub status: __u32,
    pub scrub_time: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_cmd_ars_status {
    pub status: __u32,
    pub out_length: __u32,
    pub address: __u64,
    pub length: __u64,
    pub restart_address: __u64,
    pub restart_length: __u64,
    pub type: __u16,
    pub flags: __u16,
    pub num_records: __u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_ars_record {
    pub handle: __u32,
    pub reserved: __u32,
    pub err_address: __u64,
    pub length: __u64,
    pub records: [} __packed; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_cmd_clear_error {
    pub address: __u64,
    pub length: __u64,
    pub status: __u32,
    pub reserved: [__u8; 4],
    pub cleared: __u64,
    pub __packed: },
// bus commands
// per-dimm commands
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nd_driver_flags {
    ND_DRIVER_DIMM            = 1 << ND_DEVICE_DIMM,
    ND_DRIVER_REGION_PMEM     = 1 << ND_DEVICE_REGION_PMEM,
    ND_DRIVER_REGION_BLK      = 1 << ND_DEVICE_REGION_BLK,
    ND_DRIVER_NAMESPACE_IO    = 1 << ND_DEVICE_NAMESPACE_IO,
    ND_DRIVER_NAMESPACE_PMEM  = 1 << ND_DEVICE_NAMESPACE_PMEM,
    ND_DRIVER_DAX_PMEM	  = 1 << ND_DEVICE_DAX_PMEM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ars_masks {
    ARS_STATUS_MASK = 0x0000FFFF,
    ARS_EXT_STATUS_SHIFT = 16,
}

//
// struct nd_cmd_pkg
//
// is a wrapper to a quasi pass thru interface for invoking firmware
// associated with nvdimms.
//
// INPUT PARAMETERS
//
// nd_family corresponds to the firmware (e.g. DSM) interface.
//
// nd_command are the function index advertised by the firmware.
//
// nd_size_in is the size of the input parameters being passed to firmware
//
// OUTPUT PARAMETERS
//
// nd_fw_size is the size of the data firmware wants to return for
// the call.  If nd_fw_size is greater than size of nd_size_out, only
// the first nd_size_out bytes are returned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_cmd_pkg {
    pub /: *mut *mut __u64 nd_family; / family of commands,
    pub nd_command: __u64,
    pub /: *mut *mut __u32 nd_size_in; / INPUT: size of input args,
    pub /: *mut *mut __u32 nd_size_out; / INPUT: size of payload,
    pub /: *mut *mut __u32 nd_reserved2[9]; / reserved must be zero,
    pub /: *mut *mut __u32 nd_fw_size; / OUTPUT: size fw wants to return,
    pub /: *mut *mut unsigned char nd_payload[]; / Contents of call,
}

// These NVDIMM families represent pre-standardization command sets
pub const NVDIMM_FAMILY_INTEL: c_int = 0;
pub const NVDIMM_FAMILY_HPE1: c_int = 1;
pub const NVDIMM_FAMILY_HPE2: c_int = 2;
pub const NVDIMM_FAMILY_MSFT: c_int = 3;
pub const NVDIMM_FAMILY_HYPERV: c_int = 4;
pub const NVDIMM_FAMILY_PAPR: c_int = 5;

pub const NVDIMM_BUS_FAMILY_NFIT: c_int = 0;
pub const NVDIMM_BUS_FAMILY_INTEL: c_int = 1;

