//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/smbdirect/pdu.h
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
// Copyright (c) 2017 Stefan Metzmacher
//
pub const SMBDIRECT_V1: c_uint = 0x0100;
// SMBD minimum receive size and fragmented sized defined in [MS-SMBD]
pub const SMBDIRECT_MIN_RECEIVE_SIZE: c_int = 128;
pub const SMBDIRECT_MIN_FRAGMENTED_SIZE: c_int = 131072;
// SMBD negotiation request packet [MS-SMBD] 2.2.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbdirect_negotiate_req {
    pub min_version: __le16,
    pub max_version: __le16,
    pub reserved: __le16,
    pub credits_requested: __le16,
    pub preferred_send_size: __le32,
    pub max_receive_size: __le32,
    pub max_fragmented_size: __le32,
    pub __packed: },
// SMBD negotiation response packet [MS-SMBD] 2.2.2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbdirect_negotiate_resp {
    pub min_version: __le16,
    pub max_version: __le16,
    pub negotiated_version: __le16,
    pub reserved: __le16,
    pub credits_requested: __le16,
    pub credits_granted: __le16,
    pub status: __le32,
    pub max_readwrite_size: __le32,
    pub preferred_send_size: __le32,
    pub max_receive_size: __le32,
    pub max_fragmented_size: __le32,
    pub __packed: },
pub const SMBDIRECT_DATA_MIN_HDR_SIZE: c_uint = 0x14;
pub const SMBDIRECT_DATA_OFFSET: c_uint = 0x18;
pub const SMBDIRECT_FLAG_RESPONSE_REQUESTED: c_uint = 0x0001;
// SMBD data transfer packet with payload [MS-SMBD] 2.2.3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbdirect_data_transfer {
    pub credits_requested: __le16,
    pub credits_granted: __le16,
    pub flags: __le16,
    pub reserved: __le16,
    pub remaining_data_length: __le32,
    pub data_offset: __le32,
    pub data_length: __le32,
    pub padding: __le32,
    pub buffer: [__u8; ],
    pub __packed: },
