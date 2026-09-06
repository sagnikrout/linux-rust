//! Automatically rewritten from C Header to Rust Module
//! Source: include/cxl/event.h
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
// Copyright(c) 2023 Intel Corporation.

//
// Common Event Record Format
// CXL rev 3.0 section 8.2.9.2.1; Table 8-42
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_event_record_hdr {
    pub length: u8,
    pub flags: [u8; 3],
    pub handle: __le16,
    pub related_handle: __le16,
    pub timestamp: __le64,
    pub maint_op_class: u8,
    pub maint_op_sub_class: u8,
    pub ld_id: __le16,
    pub head_id: u8,
    pub reserved: [u8; 11],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_event_media_hdr {
    pub hdr: cxl_event_record_hdr,
    pub phys_addr: __le64,
    pub descriptor: u8,
    pub type: u8,
    pub transaction_type: u8,
//
// The meaning of Validity Flags from bit 2 is
// different across DRAM and General Media records
//
    pub validity_flags: [u8; 2],
    pub channel: u8,
    pub rank: u8,
    pub __packed: },
pub const CXL_EVENT_RECORD_DATA_LENGTH: c_uint = 0x50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_event_generic {
    pub hdr: cxl_event_record_hdr,
    pub data: [u8; CXL_EVENT_RECORD_DATA_LENGTH],
    pub __packed: },
//
// General Media Event Record
// CXL rev 3.1 Section 8.2.9.2.1.1; Table 8-45
//
pub const CXL_EVENT_GEN_MED_COMP_ID_SIZE: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_event_gen_media {
    pub media_hdr: cxl_event_media_hdr,
    pub device: [u8; 3],
    pub component_id: [u8; CXL_EVENT_GEN_MED_COMP_ID_SIZE],
    pub cme_threshold_ev_flags: u8,
    pub cme_count: [u8; 3],
    pub sub_type: u8,
    pub reserved: [u8; 41],
    pub __packed: },
//
// DRAM Event Record - DER
// CXL rev 3.1 section 8.2.9.2.1.2; Table 8-46
//
pub const CXL_EVENT_DER_CORRECTION_MASK_SIZE: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_event_dram {
    pub media_hdr: cxl_event_media_hdr,
    pub nibble_mask: [u8; 3],
    pub bank_group: u8,
    pub bank: u8,
    pub row: [u8; 3],
    pub column: [u8; 2],
    pub correction_mask: [u8; CXL_EVENT_DER_CORRECTION_MASK_SIZE],
    pub component_id: [u8; CXL_EVENT_GEN_MED_COMP_ID_SIZE],
    pub sub_channel: u8,
    pub cme_threshold_ev_flags: u8,
    pub cvme_count: [u8; 3],
    pub sub_type: u8,
    pub reserved: u8,
    pub __packed: },
//
// Get Health Info Record
// CXL rev 3.1 section 8.2.9.9.3.1; Table 8-133
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_get_health_info {
    pub health_status: u8,
    pub media_status: u8,
    pub add_status: u8,
    pub life_used: u8,
    pub device_temp: [u8; 2],
    pub dirty_shutdown_cnt: [u8; 4],
    pub cor_vol_err_cnt: [u8; 4],
    pub cor_per_err_cnt: [u8; 4],
    pub __packed: },
//
// Memory Module Event Record
// CXL rev 3.1 section 8.2.9.2.1.3; Table 8-47
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_event_mem_module {
    pub hdr: cxl_event_record_hdr,
    pub event_type: u8,
    pub info: cxl_get_health_info,
    pub validity_flags: [u8; 2],
    pub component_id: [u8; CXL_EVENT_GEN_MED_COMP_ID_SIZE],
    pub event_sub_type: u8,
    pub reserved: [u8; 0x2a],
    pub __packed: },
//
// Memory Sparing Event Record - MSER
// CXL rev 3.2 section 8.2.10.2.1.4; Table 8-60
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_event_mem_sparing {
    pub hdr: cxl_event_record_hdr,
//
// The fields maintenance operation class and maintenance operation
// subclass defined in the Memory Sparing Event Record are the
// duplication of the same in the common event record. Thus defined
// as reserved and to be removed after the spec correction.
//
    pub rsv1: u8,
    pub rsv2: u8,
    pub flags: u8,
    pub result: u8,
    pub validity_flags: __le16,
    pub reserved1: [u8; 6],
    pub res_avail: __le16,
    pub channel: u8,
    pub rank: u8,
    pub nibble_mask: [u8; 3],
    pub bank_group: u8,
    pub bank: u8,
    pub row: [u8; 3],
    pub column: __le16,
    pub component_id: [u8; CXL_EVENT_GEN_MED_COMP_ID_SIZE],
    pub sub_channel: u8,
    pub reserved2: [u8; 0x25],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union cxl_event {
    pub generic: cxl_event_generic,
    pub gen_media: cxl_event_gen_media,
    pub dram: cxl_event_dram,
    pub mem_module: cxl_event_mem_module,
    pub mem_sparing: cxl_event_mem_sparing,
// dram & gen_media event header
    pub media_hdr: cxl_event_media_hdr,
    pub __packed: },
//
// Common Event Record Format; in event logs
// CXL rev 3.0 section 8.2.9.2.1; Table 8-42
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_event_record_raw {
    pub id: uuid_t,
    pub event: cxl_event,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxl_event_type {
    CXL_CPER_EVENT_GENERIC,
    CXL_CPER_EVENT_GEN_MEDIA,
    CXL_CPER_EVENT_DRAM,
    CXL_CPER_EVENT_MEM_MODULE,
    CXL_CPER_EVENT_MEM_SPARING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_cper_event_rec {
    pub length: u32,
    pub validation_bits: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_cxl_event_devid {
    pub vendor_id: u16,
    pub device_id: u16,
    pub func_num: u8,
    pub device_num: u8,
    pub bus_num: u8,
    pub segment_num: u16,
    pub /: *mut *mut u16 slot_num; / bits 2:0 reserved,
    pub reserved: u8,
    pub device_id: } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_cxl_event_sn {
    pub lower_dw: u32,
    pub upper_dw: u32,
    pub dev_serial_num: } __packed,
    pub hdr: } __packed,
    pub event: cxl_event,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_cper_work_data {
    pub event_type: cxl_event_type,
    pub rec: cxl_cper_event_rec,
}

//
// The layout of the enumeration and the values matches CXL Agent Type
// field in the UEFI 2.10 Section N.2.13,
//

// Compute Express Link Protocol Error Section, UEFI v2.10 sec N.2.13
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_cper_sec_prot_err {
    pub valid_bits: u64,
    pub agent_type: u8,
    pub reserved: [u8; 7],
//
// Except for RCH Downstream Port, all the remaining CXL Agent
// types are uniquely identified by the PCIe compatible SBDF number.
//
    pub rcrb_base_addr: u64,
    pub function: u8,
    pub device: u8,
    pub bus: u8,
    pub segment: u16,
    pub reserved_1: [u8; 3],
}

// CXL RAS Capability Structure, CXL v3.0 sec 8.2.4.16
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_ras_capability_regs {
    pub uncor_status: u32,
    pub uncor_mask: u32,
    pub uncor_severity: u32,
    pub cor_status: u32,
    pub cor_mask: u32,
    pub cap_control: u32,
    pub header_log: [u32; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_cper_prot_err_work_data {
    pub prot_err: cxl_cper_sec_prot_err,
    pub ras_cap: cxl_ras_capability_regs,
    pub severity: c_int,
}

extern "C" {
    pub fn cxl_cper_register_work(work: *mut work_struct) -> c_int;
}
extern "C" {
    pub fn cxl_cper_unregister_work(work: *mut work_struct);
}
extern "C" {
    pub fn cxl_cper_kfifo_get(wd: *mut cxl_cper_work_data) -> c_int;
}
extern "C" {
    pub fn cxl_cper_register_prot_err_work(work: *mut work_struct);
}
extern "C" {
    pub fn cxl_cper_unregister_prot_err_work();
}
extern "C" {
    pub fn cxl_cper_prot_err_kfifo_get(wd: *mut cxl_cper_prot_err_work_data) -> c_int;
}

extern "C" {
    pub fn cxl_cper_sec_prot_err_valid(prot_err: *mut cxl_cper_sec_prot_err) -> c_int;
}

extern "C" {
    pub fn cxl_cper_handle_prot_err(wd: *mut cxl_cper_prot_err_work_data);
}
