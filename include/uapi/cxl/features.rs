//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/cxl/features.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright (c) 2024,2025, Intel Corporation
//
// These are definitions for the mailbox command interface of CXL subsystem.
//

//
// Note, __uapi_uuid_t is 1-byte aligned on modern compilers and 4-byte
// aligned on others. Ensure that __uapi_uuid_t in a struct is placed at
// a 4-byte aligned offset, or the structure is packed, to ensure
// consistent padding.
//

//
// struct cxl_mbox_get_sup_feats_in - Get Supported Features input
//
// @count: bytes of Feature data to return in output
// @start_idx: index of first requested Supported Feature Entry, 0 based.
// @reserved: reserved field, must be 0s.
//
// Get Supported Features (0x500h) CXL r3.2 8.2.9.6.1 command.
// Input block for Get support Feature
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_get_sup_feats_in {
    pub count: __le32,
    pub start_idx: __le16,
    pub reserved: [__u8; 2],
// C attribute field omitted
// CXL spec r3.2 Table 8-87 command effects

//
// struct cxl_feat_entry - Supported Feature Entry
// @uuid: UUID of the Feature
// @id: id to identify the feature. 0 based
// @get_feat_size: max bytes required for Get Feature command for this Feature
// @set_feat_size: max bytes required for Set Feature command for this Feature
// @flags: attribute flags
// @get_feat_ver: Get Feature version
// @set_feat_ver: Set Feature version
// @effects: Set Feature command effects
// @reserved: reserved, must be 0
//
// CXL spec r3.2 Table 8-109
// Get Supported Features Supported Feature Entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_feat_entry {
    pub uuid: __uapi_uuid_t,
    pub id: __le16,
    pub get_feat_size: __le16,
    pub set_feat_size: __le16,
    pub flags: __le32,
    pub get_feat_ver: __u8,
    pub set_feat_ver: __u8,
    pub effects: __le16,
    pub reserved: [__u8; 18],
// C attribute field omitted
// @flags field for 'struct cxl_feat_entry'

//
// struct cxl_mbox_get_sup_feats_out - Get Supported Features output
// @num_entries: number of Supported Feature Entries returned
// @supported_feats: number of supported Features
// @reserved: reserved, must be 0s.
// @ents: Supported Feature Entries array
//
// CXL spec r3.2 Table 8-108
// Get supported Features Output Payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_get_sup_feats_out {
    pub num_entries: __le16,
    pub supported_feats: __le16,
    pub reserved: [__u8; 4],
    pub __counted_by_le(num_entries): cxl_feat_entry ents[],
// C attribute field omitted
//
// Get Feature CXL spec r3.2 Spec 8.2.9.6.2
//
// struct cxl_mbox_get_feat_in - Get Feature input
// @uuid: UUID for Feature
// @offset: offset of the first byte in Feature data for output payload
// @count: count in bytes of Feature data returned
// @selection: 0 current value, 1 default value, 2 saved value
//
// CXL spec r3.2 section 8.2.9.6.2 Table 8-99
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_get_feat_in {
    pub uuid: __uapi_uuid_t,
    pub offset: __le16,
    pub count: __le16,
    pub selection: __u8,
// C attribute field omitted
//
// enum cxl_get_feat_selection - selection field of Get Feature input
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxl_get_feat_selection {
    CXL_GET_FEAT_SEL_CURRENT_VALUE,
    CXL_GET_FEAT_SEL_DEFAULT_VALUE,
    CXL_GET_FEAT_SEL_SAVED_VALUE,
    CXL_GET_FEAT_SEL_MAX
}

//
// Set Feature CXL spec r3.2  8.2.9.6.3
//
// struct cxl_mbox_set_feat_in - Set Features input
// @uuid: UUID for Feature
// @flags: set feature flags
// @offset: byte offset of Feature data to update
// @version: Feature version of the data in Feature Data
// @rsvd: reserved, must be 0s.
// @feat_data: raw byte stream of Features data to update
//
// CXL spec r3.2 section 8.2.9.6.3 Table 8-101
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_mbox_set_feat_in {
    pub uuid: __uapi_uuid_t,
    pub flags: __le32,
    pub offset: __le16,
    pub version: __u8,
    pub rsvd: [__u8; 9],
    pub feat_data: [__u8; ],
    pub __packed: },
//
// enum cxl_set_feat_flag_data_transfer - Set Feature flags field
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxl_set_feat_flag_data_transfer {
    CXL_SET_FEAT_FLAG_FULL_DATA_TRANSFER = 0,
    CXL_SET_FEAT_FLAG_INITIATE_DATA_TRANSFER,
    CXL_SET_FEAT_FLAG_CONTINUE_DATA_TRANSFER,
    CXL_SET_FEAT_FLAG_FINISH_DATA_TRANSFER,
    CXL_SET_FEAT_FLAG_ABORT_DATA_TRANSFER,
    CXL_SET_FEAT_FLAG_DATA_TRANSFER_MAX
}

