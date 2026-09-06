//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/intel-ish-hid/ishtp/loader.h
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
// ISHTP firmware loader header
//
// Copyright (c) 2024, Intel Corporation.
//

//
// ISHTP firmware loader protocol definition
//

// Only support DMA mode

//
// union loader_msg_header - ISHTP firmware loader message header
// @command: Command type
// @is_response: Indicates if the message is a response
// @has_next: Indicates if there is a next message
// @reserved: Reserved for future use
// @status: Status of the message
// @val32: entire header as a 32-bit value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union loader_msg_header {
    pub command:7: __u32,
    pub is_response:1: __u32,
    pub has_next:1: __u32,
    pub reserved:15: __u32,
    pub status:8: __u32,
}

//
// struct loader_xfer_query - ISHTP firmware loader transfer query packet
// @header: Header of the message
// @image_size: Size of the image
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loader_xfer_query {
    pub header: __le32,
    pub image_size: __le32,
}

//
// struct loader_version - ISHTP firmware loader version
// @value: Value of the version
// @major: Major version
// @minor: Minor version
// @hotfix: Hotfix version
// @build: Build version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loader_version {
    pub value: __le32,
    pub major: __u8,
    pub minor: __u8,
    pub hotfix: __u8,
    pub build: __u8,
}

//
// struct loader_capability - ISHTP firmware loader capability
// @max_fw_image_size: Maximum firmware image size
// @support_mode: Support mode
// @reserved: Reserved for future use
// @platform: Platform
// @max_dma_buf_size: Maximum DMA buffer size, multiples of 4096
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loader_capability {
    pub max_fw_image_size: __le32,
    pub support_mode: __le16,
    pub reserved: __u8,
    pub platform: __u8,
    pub max_dma_buf_size: __le32,
}

//
// struct loader_xfer_query_ack - ISHTP firmware loader transfer query acknowledgment
// @header: Header of the message
// @version_major: ISH Major version
// @version_minor: ISH Minor version
// @version_hotfix: ISH Hotfix version
// @version_build: ISH Build version
// @protocol_version: Protocol version
// @loader_version: Loader version
// @capability: Loader capability
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loader_xfer_query_ack {
    pub header: __le32,
    pub version_major: __le16,
    pub version_minor: __le16,
    pub version_hotfix: __le16,
    pub version_build: __le16,
    pub protocol_version: __le32,
    pub loader_version: loader_version,
    pub capability: loader_capability,
}

//
// struct loader_xfer_fragment - ISHTP firmware loader transfer fragment
// @header: Header of the message
// @xfer_mode: Transfer mode
// @offset: Offset
// @size: Size
// @is_last: Is last
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loader_xfer_fragment {
    pub header: __le32,
    pub xfer_mode: __le32,
    pub offset: __le32,
    pub size: __le32,
    pub is_last: __le32,
}

//
// struct loader_xfer_fragment_ack - ISHTP firmware loader transfer fragment acknowledgment
// @header: Header of the message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loader_xfer_fragment_ack {
    pub header: __le32,
}

//
// struct fragment_dscrpt - ISHTP firmware loader fragment descriptor
// @ddr_adrs: The address in host DDR
// @fw_off: The offset of the fragment in the fw image
// @length: The length of the fragment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fragment_dscrpt {
    pub ddr_adrs: __le64,
    pub fw_off: __le32,
    pub length: __le32,
}

//
// struct loader_xfer_dma_fragment - ISHTP firmware loader transfer DMA fragment
// @fragment: Fragment
// @fragment_cnt: How many descriptors in the fragment_tbl
// @fragment_tbl: Fragment table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loader_xfer_dma_fragment {
    pub fragment: loader_xfer_fragment,
    pub fragment_cnt: __le32,
    pub __counted_by(fragment_cnt): fragment_dscrpt fragment_tbl[],
}

//
// struct loader_start - ISHTP firmware loader start
// @header: Header of the message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loader_start {
    pub header: __le32,
}

//
// struct loader_start_ack - ISHTP firmware loader start acknowledgment
// @header: Header of the message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loader_start_ack {
    pub header: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union loader_recv_message {
    pub header: __le32,
    pub query_ack: loader_xfer_query_ack,
    pub fragment_ack: loader_xfer_fragment_ack,
    pub start_ack: loader_start_ack,
    pub raw_data: [__u8; LOADER_MSG_SIZE],
}

//
// ISHTP firmware loader internal use
//
// ISHTP firmware loader command timeout

// ISHTP firmware loader retry times
pub const ISHTP_LOADER_RETRY_TIMES: c_int = 3;
//
// struct ish_firmware_variant - ISH firmware variant
// @device: PCI Device ID
// @filename: The firmware file name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ish_firmware_variant {
    pub device: c_ushort,
    pub filename: *const c_char,
}

//
// ISHTP firmware loader API for ISHTP hbm
//
// ISHTP capability bit for firmware loader

// Firmware loader address
pub const ISHTP_LOADER_CLIENT_ADDR: c_int = 16;
//
// ishtp_loader_work - The work function to start the firmware loading process
// @work: The work structure
//
extern "C" {
    pub fn ishtp_loader_work(work: *mut work_struct);
}
// ISH Manifest alignment in binary is 4KB aligned

// Signature for ISH global manifest
pub const ISH_GLOBAL_SIG: c_uint = 0x47485349	/* FourCC 'I', 'S', 'H', 'G' */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct version_in_manifest {
    pub major: __le16,
    pub minor: __le16,
    pub hotfix: __le16,
    pub build: __le16,
}

//
// struct ish_global_manifest - global manifest for ISH
// @sig_fourcc: Signature FourCC, should be 'I', 'S', 'H', 'G'.
// @len: Length of the manifest.
// @header_version: Version of the manifest header.
// @flags: Flags for additional information.
// @base_ver: Base version of Intel's released firmware.
// @reserved: Reserved space for future use.
// @prj_ver: Vendor-customized project version.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ish_global_manifest {
    pub sig_fourcc: __le32,
    pub len: __le32,
    pub header_version: __le32,
    pub flags: __le32,
    pub base_ver: version_in_manifest,
    pub reserved: [__le32; 13],
    pub prj_ver: version_in_manifest,
}
