//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/sof/header.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//

//
// struct sof_abi_hdr - Header for all non IPC ABI data.
// @magic: Magic number for validation
// for IPC3 data: 0x00464F53 ('S', 'O', 'F', '\0')
// for IPC4 data: 0x34464F53 ('S', 'O', 'F', '4')
// @type: module specific parameter
// for IPC3: Component specific type
// for IPC4: parameter ID (param_id) of the data
// @size: The size in bytes of the data, excluding this struct
// @abi: SOF ABI version. The version is valid in scope of the 'magic', IPC3 and
// IPC4 ABI version numbers have no relationship.
// @reserved: Reserved for future use
// @data: Component data - opaque to core
//
// Identifies data type, size and ABI.
// Used by any bespoke component data structures or binary blobs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_abi_hdr {
    pub magic: __u32,
    pub type: __u32,
    pub size: __u32,
    pub abi: __u32,
    pub reserved: [__u32; 4],
    pub data: [__u32; ],
    pub __packed: },
pub const SOF_MANIFEST_DATA_TYPE_NHLT: c_int = 1;
//
// struct sof_manifest_tlv - SOF manifest TLV data
// @type: type of data
// @size: data size (not including the size of this struct)
// @data: payload data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_manifest_tlv {
    pub type: __le32,
    pub size: __le32,
    pub data: [__u8; ],
}

//
// struct sof_manifest - SOF topology manifest
// @abi_major: Major ABI version
// @abi_minor: Minor ABI version
// @abi_patch: ABI patch
// @count: count of tlv items
// @items: consecutive variable size tlv items
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_manifest {
    pub abi_major: __le16,
    pub abi_minor: __le16,
    pub abi_patch: __le16,
    pub count: __le16,
    pub items: [sof_manifest_tlv; ],
}
