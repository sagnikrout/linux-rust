//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sof/ext_manifest4.h
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
// Copyright(c) 2022 Intel Corporation
//
// Extended manifest is a place to store metadata about firmware, known during
// compilation time - for example firmware version or used compiler.
// Given information are read on host side before firmware startup.
// This part of output binary is not signed.
//

// In ASCII  $AE1
pub const SOF_EXT_MAN4_MAGIC_NUMBER: c_uint = 0x31454124;
pub const MAX_MODULE_NAME_LEN: c_int = 8;
pub const MAX_FW_BINARY_NAME: c_int = 8;
pub const DEFAULT_HASH_SHA256_LEN: c_int = 32;
pub const SOF_MAN4_FW_HDR_OFFSET: c_uint = 0x2000;
pub const SOF_MAN4_FW_HDR_OFFSET_CAVS_1_5: c_uint = 0x284;
//
// extended manifest		(struct sof_ext_manifest4_hdr)
// -------------------
// css_manifest hdr
// -------------------
// offset reserved for future
// -------------------
// fw_hdr				(struct sof_man4_fw_binary_header)
// -------------------
// module_entry[0]			(struct sof_man4_module)
// -------------------
// module_entry[1]
// -------------------
// ...
// -------------------
// module_entry[n]
// -------------------
// module_config[0]		(struct sof_man4_module_config)
// -------------------
// module_config[1]
// -------------------
// ...
// -------------------
// module_config[m]
// -------------------
// FW content
// -------------------
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ext_manifest4_hdr {
    pub id: u32,
    pub /: *mut *mut uint32_t len; / length of extension manifest,
    pub /: *mut *mut uint16_t version_major; / header version,
    pub version_minor: u16,
    pub num_module_entries: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_man4_fw_binary_header {
// This part must be unchanged to be backward compatible with SPT-LP ROM
    pub id: u32,
    pub /: *mut *mut uint32_t len; / sizeof(sof_man4_fw_binary_header) in bytes,
    pub name: [u8; MAX_FW_BINARY_NAME],
    pub /: *mut *mut uint32_t preload_page_count; / number of pages of preloaded image,
    pub fw_image_flags: u32,
    pub feature_mask: u32,
    pub /: *mut *mut uint16_t major_version; / Firmware version,
    pub minor_version: u16,
    pub hotfix_version: u16,
    pub build_version: u16,
    pub num_module_entries: u32,
// This part may change to contain any additional data for BaseFw that is skipped by ROM
    pub hw_buf_base_addr: u32,
    pub hw_buf_length: u32,
    pub /: *mut *mut uint32_t load_offset; / This value is used by ROM,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_man4_segment_desc {
    pub flags: u32,
    pub v_base_addr: u32,
    pub file_offset: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_man4_module {
    pub id: u32,
    pub name: [u8; MAX_MODULE_NAME_LEN],
    pub uuid: guid_t,
    pub type: u32,
    pub hash: [u8; DEFAULT_HASH_SHA256_LEN],
    pub entry_point: u32,
    pub cfg_offset: u16,
    pub cfg_count: u16,
    pub affinity_mask: u32,
    pub instance_max_count: u16,
    pub instance_stack_size: u16,
    pub segments: [sof_man4_segment_desc; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_man4_module_config {
    pub /: *mut *mut uint32_t par[4]; / module parameters,
    pub /: *mut *mut uint32_t is_bytes; / actual size of instance .bss (bytes),
    pub /: *mut *mut uint32_t cps; / cycles per second,
    pub /: *mut *mut uint32_t ibs; / input buffer size (bytes),
    pub /: *mut *mut uint32_t obs; / output buffer size (bytes),
    pub /: *mut *mut uint32_t module_flags; / flags, reserved for future use,
    pub /: *mut *mut uint32_t cpc; / cycles per single run,
    pub /: *mut *mut uint32_t obls; / output block size, reserved for future use,
    pub __packed: },
