//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/ipa_qmi_msg.h
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
// Copyright (c) 2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2018-2024 Linaro Ltd.
//
// === Only "ipa_qmi" and "ipa_qmi_msg.c" should include this file ===

// Request/response/indication QMI message ids used for IPA.  Receiving
// end issues a response for requests; indications require no response.
//
pub const IPA_QMI_INDICATION_REGISTER: c_uint = 0x20	/* modem -> AP request */;
pub const IPA_QMI_INIT_DRIVER: c_uint = 0x21	/* AP -> modem request */;
pub const IPA_QMI_INIT_COMPLETE: c_uint = 0x22	/* AP -> modem indication */;
pub const IPA_QMI_DRIVER_INIT_COMPLETE: c_uint = 0x35	/* modem -> AP request */;
// The maximum size required for message types.  These sizes include
// the message data, along with type (1 byte) and length (2 byte)
// information for each field.  The qmi_send_*() interfaces require
// the message size to be provided.
//

// Maximum size of messages we expect the AP to receive (max of above)
pub const IPA_QMI_SERVER_MAX_RCV_SZ: c_int = 8;
pub const IPA_QMI_CLIENT_MAX_RCV_SZ: c_int = 25;
// Request message for the IPA_QMI_INDICATION_REGISTER request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_indication_register_req {
    pub master_driver_init_complete_valid: u8,
    pub master_driver_init_complete: u8,
    pub data_usage_quota_reached_valid: u8,
    pub data_usage_quota_reached: u8,
    pub ipa_mhi_ready_ind_valid: u8,
    pub ipa_mhi_ready_ind: u8,
    pub endpoint_desc_ind_valid: u8,
    pub endpoint_desc_ind: u8,
    pub bw_change_ind_valid: u8,
    pub bw_change_ind: u8,
}

// The response to a IPA_QMI_INDICATION_REGISTER request consists only of
// a standard QMI response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_indication_register_rsp {
    pub rsp: qmi_response_type_v01,
}

// Request message for the IPA_QMI_DRIVER_INIT_COMPLETE request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_driver_init_complete_req {
    pub status: u8,
}

// The response to a IPA_QMI_DRIVER_INIT_COMPLETE request consists only
// of a standard QMI response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_driver_init_complete_rsp {
    pub rsp: qmi_response_type_v01,
}

// The message for the IPA_QMI_INIT_COMPLETE_IND indication consists
// only of a standard QMI response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_init_complete_ind {
    pub status: qmi_response_type_v01,
}

// The AP tells the modem its platform type.  We assume Android.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipa_platform_type {
    IPA_QMI_PLATFORM_TYPE_INVALID		= 0x0,	/* Invalid */
    IPA_QMI_PLATFORM_TYPE_TN		= 0x1,	/* Data card */
    IPA_QMI_PLATFORM_TYPE_LE		= 0x2,	/* Data router */
    IPA_QMI_PLATFORM_TYPE_MSM_ANDROID	= 0x3,	/* Android MSM */
    IPA_QMI_PLATFORM_TYPE_MSM_WINDOWS	= 0x4,	/* Windows MSM */
    IPA_QMI_PLATFORM_TYPE_MSM_QNX_V01	= 0x5,	/* QNX MSM */
}

// This defines the start and end offset of a range of memory.  The start
// value is a byte offset relative to the start of IPA shared memory.  The
// end value is the last addressable unit *within* the range.  Typically
// the end value is in units of bytes, however it can also be a maximum
// array index value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_mem_bounds {
    pub start: u32,
    pub end: u32,
}

// This defines the location and size of an array.  The start value
// is an offset relative to the start of IPA shared memory.  The
// size of the array is implied by the number of entries (the entry
// size is assumed to be known).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_mem_array {
    pub start: u32,
    pub count: u32,
}

// This defines the location and size of a range of memory.  The
// start is an offset relative to the start of IPA shared memory.
// This differs from the ipa_mem_bounds structure in that the size
// (in bytes) of the memory region is specified rather than the
// offset of its last byte.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_mem_range {
    pub start: u32,
    pub size: u32,
}

// The message for the IPA_QMI_INIT_DRIVER request contains information
// from the AP that affects modem initialization.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_init_modem_driver_req {
    pub platform_type_valid: u8,
    pub /: *mut *mut u32 platform_type; / enum ipa_platform_type,
// Modem header table information.  This defines the IPA shared
// memory in which the modem may insert header table entries.
//
    pub hdr_tbl_info_valid: u8,
    pub hdr_tbl_info: ipa_mem_bounds,
// Routing table information.  These define the location and maximum
// *index* (not byte) for the modem portion of non-hashable IPv4 and
// IPv6 routing tables.  The start values are byte offsets relative
// to the start of IPA shared memory.
//
    pub v4_route_tbl_info_valid: u8,
    pub v4_route_tbl_info: ipa_mem_bounds,
    pub v6_route_tbl_info_valid: u8,
    pub v6_route_tbl_info: ipa_mem_bounds,
// Filter table information.  These define the location of the
// non-hashable IPv4 and IPv6 filter tables.  The start values are
// byte offsets relative to the start of IPA shared memory.
//
    pub v4_filter_tbl_start_valid: u8,
    pub v4_filter_tbl_start: u32,
    pub v6_filter_tbl_start_valid: u8,
    pub v6_filter_tbl_start: u32,
// Modem memory information.  This defines the location and
// size of memory available for the modem to use.
//
    pub modem_mem_info_valid: u8,
    pub modem_mem_info: ipa_mem_range,
// This defines the destination endpoint on the AP to which
// the modem driver can send control commands.  Must be less
// than ipa_endpoint_max().
//
    pub ctrl_comm_dest_end_pt_valid: u8,
    pub ctrl_comm_dest_end_pt: u32,
// This defines whether the modem should load the microcontroller
// or not.  It is unnecessary to reload it if the modem is being
// restarted.
//
// NOTE: this field is named "is_ssr_bootup" elsewhere.
//
    pub skip_uc_load_valid: u8,
    pub skip_uc_load: u8,
// Processing context memory information.  This defines the memory in
// which the modem may insert header processing context table entries.
//
    pub hdr_proc_ctx_tbl_info_valid: u8,
    pub hdr_proc_ctx_tbl_info: ipa_mem_bounds,
// Compression command memory information.  This defines the memory
// in which the modem may insert compression/decompression commands.
//
    pub zip_tbl_info_valid: u8,
    pub zip_tbl_info: ipa_mem_bounds,
// Routing table information.  These define the location and maximum
// *index* (not byte) for the modem portion of hashable IPv4 and IPv6
// routing tables (if supported by hardware).  The start values are
// byte offsets relative to the start of IPA shared memory.
//
    pub v4_hash_route_tbl_info_valid: u8,
    pub v4_hash_route_tbl_info: ipa_mem_bounds,
    pub v6_hash_route_tbl_info_valid: u8,
    pub v6_hash_route_tbl_info: ipa_mem_bounds,
// Filter table information.  These define the location and size
// of hashable IPv4 and IPv6 filter tables (if supported by hardware).
// The start values are byte offsets relative to the start of IPA
// shared memory.
//
    pub v4_hash_filter_tbl_start_valid: u8,
    pub v4_hash_filter_tbl_start: u32,
    pub v6_hash_filter_tbl_start_valid: u8,
    pub v6_hash_filter_tbl_start: u32,
// Statistics information.  These define the locations of the
// first and last statistics sub-regions.  (IPA v4.0 and above)
//
    pub hw_stats_quota_base_addr_valid: u8,
    pub hw_stats_quota_base_addr: u32,
    pub hw_stats_quota_size_valid: u8,
    pub hw_stats_quota_size: u32,
    pub hw_stats_drop_base_addr_valid: u8,
    pub hw_stats_drop_base_addr: u32,
    pub hw_stats_drop_size_valid: u8,
    pub hw_stats_drop_size: u32,
}

// The response to a IPA_QMI_INIT_DRIVER request begins with a standard
// QMI response, but contains other information as well.  Currently we
// simply wait for the INIT_DRIVER transaction to complete and
// ignore any other data that might be returned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_init_modem_driver_rsp {
    pub rsp: qmi_response_type_v01,
// This defines the destination endpoint on the modem to which
// the AP driver can send control commands.  Must be less than
// ipa_endpoint_max().
//
    pub ctrl_comm_dest_end_pt_valid: u8,
    pub ctrl_comm_dest_end_pt: u32,
// This defines the default endpoint.  The AP driver is not
// required to configure the hardware with this value.  Must
// be less than ipa_endpoint_max().
//
    pub default_end_pt_valid: u8,
    pub default_end_pt: u32,
// This defines whether a second handshake is required to complete
// initialization.
//
    pub modem_driver_init_pending_valid: u8,
    pub modem_driver_init_pending: u8,
}

// Message structure definitions defined in "ipa_qmi_msg.c"
