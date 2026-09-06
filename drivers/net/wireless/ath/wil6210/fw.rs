//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/wil6210/fw.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2014,2016 Qualcomm Atheros, Inc.
// Copyright (c) 2018-2019, The Linux Foundation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wil_fw_record_type {
    wil_fw_type_comment = 1,
    wil_fw_type_data = 2,
    wil_fw_type_fill = 3,
    wil_fw_type_action = 4,
    wil_fw_type_verify = 5,
    wil_fw_type_file_header = 6,
    wil_fw_type_direct_write = 7,
    wil_fw_type_gateway_data = 8,
    wil_fw_type_gateway_data4 = 9,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_record_head {
    pub /: *mut *mut __le16 type; / enum wil_fw_record_type,
    pub /: *mut *mut __le16 flags; / to be defined,
    pub /: *mut *mut __le32 size; / whole record, bytes after head,
    pub __packed: },
// data block. write starting from @addr
// data_size inferred from the @head.size. For this case,
// data_size = @head.size - offsetof(struct wil_fw_record_data, data)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_record_data {
    pub addr: __le32,
    pub /: *mut *mut __le32 data[]; / [data_size], see above,
    pub __packed: },
// fill with constant @value, @size bytes starting from @addr
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_record_fill {
    pub addr: __le32,
    pub value: __le32,
    pub size: __le32,
    pub __packed: },
// free-form comment
// for informational purpose, data_size is @head.size from record header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_record_comment {
    pub /: *mut *mut DECLARE_FLEX_ARRAY(u8, data); / free-form data [data_size], see above,
    pub __packed: },
// Comment header - common for all comment record types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_record_comment_hdr {
    pub magic: __le32,
}

// FW capabilities encoded inside a comment record

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_record_capabilities {
// identifies capabilities record
    pub hdr: wil_fw_record_comment_hdr,
// capabilities (variable size), see enum wmi_fw_capability
    pub capabilities: [u8; ],
    pub __packed: },
// FW VIF concurrency encoded inside a comment record
// Format is similar to wiphy->iface_combinations
//

pub const WIL_FW_CONCURRENCY_REC_VER: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_concurrency_limit {
    pub /: *mut *mut __le16 max; / maximum number of interfaces of these types,
    pub /: *mut *mut __le16 types; / interface types (bit mask of enum nl80211_iftype),
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_concurrency_combo {
    pub /: *mut *mut u8 n_limits; / number of wil_fw_concurrency_limit entries,
    pub /: *mut *mut u8 max_interfaces; / max number of concurrent interfaces allowed,
    pub /: *mut *mut u8 n_diff_channels; / total number of different channels allowed,
    pub /: *mut *mut u8 same_bi; / for APs, 1 if all APs must have same BI,
// keep last - concurrency limits, variable size by n_limits
    pub limits: [wil_fw_concurrency_limit; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_record_concurrency {
// identifies concurrency record
    pub magic: __le32,
// structure version, currently always 1
    pub version: u8,
// maximum number of supported MIDs _in addition_ to MID 0
    pub n_mids: u8,
// number of concurrency combinations that follow
    pub n_combos: __le16,
// keep last - combinations, variable size by n_combos
    pub __packed: },
// brd file info encoded inside a comment record

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brd_info {
    pub base_addr: __le32,
    pub max_size_bytes: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_record_brd_file {
// identifies brd file record
    pub hdr: wil_fw_record_comment_hdr,
    pub version: __le32,
    pub brd_info: [brd_info; ],
    pub __packed: },
// perform action
// data_size = @head.size - offsetof(struct wil_fw_record_action, data)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_record_action {
    pub /: *mut *mut __le32 action; / action to perform: reset, wait for fw ready etc.,
    pub /: *mut *mut __le32 data[]; / action specific, [data_size], see above,
    pub __packed: },
// data block for struct wil_fw_record_direct_write
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_data_dwrite {
    pub addr: __le32,
    pub value: __le32,
    pub mask: __le32,
    pub __packed: },
// write @value to the @addr,
// preserve original bits accordingly to the @mask
// data_size is @head.size where @head is record header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_record_direct_write {
    pub data): DECLARE_FLEX_ARRAY(struct wil_fw_data_dwrite,,
    pub __packed: },
// verify condition: [@addr] & @mask == @value
// if condition not met, firmware download fails
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_record_verify {
    pub /: *mut *mut __le32 addr; / read from this address,
    pub /: *mut *mut __le32 value; / reference value,
    pub /: *mut *mut __le32 mask; / mask for verification,
    pub __packed: },
// file header
// First record of every file
//
// the FW version prefix in the comment

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_record_file_header {
    pub /: *mut *mut __le32 signature ; / Wilocity signature,
    pub reserved: __le32,
    pub /: *mut *mut __le32 crc; / crc32 of the following data,
    pub /: *mut *mut __le32 version; / format version,
    pub /: *mut *mut __le32 data_len; / total data in file, including this record,
    pub /: *mut *mut u8 comment[32]; / short description,
    pub __packed: },
// 1-dword gateway
// data block for the struct wil_fw_record_gateway_data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_data_gw {
    pub addr: __le32,
    pub value: __le32,
    pub __packed: },
// gateway write block.
// write starting address and values from the data buffer
// through the gateway
// data_size inferred from the @head.size. For this case,
// data_size = @head.size - offsetof(struct wil_fw_record_gateway_data, data)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_record_gateway_data {
    pub gateway_addr_addr: __le32,
    pub gateway_value_addr: __le32,
    pub gateway_cmd_addr: __le32,
    pub gateway_ctrl_address: __le32,

    pub command: __le32,
    pub /: *mut *mut wil_fw_data_gw data[]; / total size [data_size], see above,
    pub __packed: },
// 4-dword gateway
// data block for the struct wil_fw_record_gateway_data4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_data_gw4 {
    pub addr: __le32,
    pub value: [__le32; 4],
    pub __packed: },
// gateway write block.
// write starting address and values from the data buffer
// through the gateway
// data_size inferred from the @head.size. For this case,
// data_size = @head.size - offsetof(struct wil_fw_record_gateway_data4, data)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_record_gateway_data4 {
    pub gateway_addr_addr: __le32,
    pub gateway_value_addr: [__le32; 4],
    pub gateway_cmd_addr: __le32,
    pub /: *mut *mut __le32 gateway_ctrl_address; / same logic as for 1-dword gw,
    pub command: __le32,
    pub /: *mut *mut wil_fw_data_gw4 data[]; / total size [data_size], see above,
    pub __packed: },
