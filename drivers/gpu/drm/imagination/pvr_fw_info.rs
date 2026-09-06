//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_fw_info.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

//
// Firmware binary block unit in bytes.
// Raw data stored in FW binary will be aligned to this size.
//

// Maximum number of entries in firmware layout table.
pub const PVR_FW_INFO_MAX_NUM_ENTRIES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvr_fw_section_id {
    META_CODE = 0,
    META_PRIVATE_DATA,
    META_COREMEM_CODE,
    META_COREMEM_DATA,
    MIPS_CODE,
    MIPS_EXCEPTIONS_CODE,
    MIPS_BOOT_CODE,
    MIPS_PRIVATE_DATA,
    MIPS_BOOT_DATA,
    MIPS_STACK,
    RISCV_UNCACHED_CODE,
    RISCV_CACHED_CODE,
    RISCV_PRIVATE_DATA,
    RISCV_COREMEM_CODE,
    RISCV_COREMEM_DATA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvr_fw_section_type {
    NONE = 0,
    FW_CODE,
    FW_DATA,
    FW_COREMEM_CODE,
    FW_COREMEM_DATA,
}

//
// FW binary format with FW info attached:
//
// Contents        Offset
// +-----------------+
// |                 |    0
// |                 |
// | Original binary |
// |      file       |
// |   (.ldr/.elf)   |
// |                 |
// +-----------------+
// |   Device info   |  FILE_SIZE - 4K - device_info_size
// +-----------------+
// | FW info header  |  FILE_SIZE - 4K
// +-----------------+
// |                 |
// | FW layout table |
// |                 |
// +-----------------+
// FILE_SIZE
//
pub const PVR_FW_INFO_VERSION: c_int = 3;

// struct pvr_fw_info_header - Firmware header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_fw_info_header {
// @info_version: FW info header version.
    pub info_version: u32,
// @header_len: Header length.
    pub header_len: u32,
// @layout_entry_num: Number of entries in the layout table.
    pub layout_entry_num: u32,
// @layout_entry_size: Size of an entry in the layout table.
    pub layout_entry_size: u32,
// @bvnc: GPU ID supported by firmware.
    pub bvnc: aligned_u64,
// @fw_page_size: Page size of processor on which firmware executes.
    pub fw_page_size: u32,
// @flags: Compatibility flags.
    pub flags: u32,
// @fw_version_major: Firmware major version number.
    pub fw_version_major: u16,
// @fw_version_minor: Firmware minor version number.
    pub fw_version_minor: u16,
// @fw_version_build: Firmware build number.
    pub fw_version_build: u32,
// @device_info_size: Size of device info structure.
    pub device_info_size: u32,
// @padding: Padding.
    pub padding: u32,
}

//
// struct pvr_fw_layout_entry - Entry in firmware layout table, describing a
// section of the firmware image
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_fw_layout_entry {
// @id: Section ID.
    pub id: pvr_fw_section_id,
// @type: Section type.
    pub type: pvr_fw_section_type,
// @base_addr: Base address of section in FW address space.
    pub base_addr: u32,
// @max_size: Maximum size of section, in bytes.
    pub max_size: u32,
// @alloc_size: Allocation size of section, in bytes.
    pub alloc_size: u32,
// @alloc_offset: Allocation offset of section.
    pub alloc_offset: u32,
}

//
// struct pvr_fw_device_info_header - Device information header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_fw_device_info_header {
// @brn_mask_size: BRN mask size (in u64s).
    pub brn_mask_size: u64,
// @ern_mask_size: ERN mask size (in u64s).
    pub ern_mask_size: u64,
// @feature_mask_size: Feature mask size (in u64s).
    pub feature_mask_size: u64,
// @feature_param_size: Feature parameter size (in u64s).
    pub feature_param_size: u64,
}
