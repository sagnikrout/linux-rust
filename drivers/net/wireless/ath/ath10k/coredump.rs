//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/coredump.h
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
// Copyright (c) 2011-2017 Qualcomm Atheros, Inc.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const ATH10K_FW_CRASH_DUMP_VERSION: c_int = 1;
//
// enum ath10k_fw_crash_dump_type - types of data in the dump file
// @ATH10K_FW_CRASH_DUMP_REGISTERS: Register crash dump in binary format
// @ATH10K_FW_CRASH_DUMP_CE_DATA: Copy Engine crash dump data
// @ATH10K_FW_CRASH_DUMP_RAM_DATA: RAM crash dump data, contains multiple
// struct ath10k_dump_ram_data_hdr
// @ATH10K_FW_CRASH_DUMP_MAX: Maximum enumeration
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_fw_crash_dump_type {
    ATH10K_FW_CRASH_DUMP_REGISTERS = 0,
    ATH10K_FW_CRASH_DUMP_CE_DATA = 1,

// contains multiple struct ath10k_dump_ram_data_hdr
    ATH10K_FW_CRASH_DUMP_RAM_DATA = 2,

    ATH10K_FW_CRASH_DUMP_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_tlv_dump_data {
// see ath10k_fw_crash_dump_type above
    pub type: __le32,
// in bytes
    pub tlv_len: __le32,
// pad to 32-bit boundaries as needed
    pub tlv_data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_dump_file_data {
// dump file information
// "ATH10K-FW-DUMP"
    pub df_magic: [c_char; 16],
    pub len: __le32,
// file dump version
    pub version: __le32,
// some info we can get from ath10k struct that might help
    pub guid: guid_t,
    pub chip_id: __le32,
// 0 for now, in place for later hardware
    pub bus_type: __le32,
    pub target_version: __le32,
    pub fw_version_major: __le32,
    pub fw_version_minor: __le32,
    pub fw_version_release: __le32,
    pub fw_version_build: __le32,
    pub phy_capability: __le32,
    pub hw_min_tx_power: __le32,
    pub hw_max_tx_power: __le32,
    pub ht_cap_info: __le32,
    pub vht_cap_info: __le32,
    pub num_rf_chains: __le32,
// firmware version string
    pub fw_ver: [c_char; ETHTOOL_FWVERS_LEN],
// Kernel related information
// time-of-day stamp
    pub tv_sec: __le64,
// time-of-day stamp, nano-seconds
    pub tv_nsec: __le64,
// LINUX_VERSION_CODE
    pub kernel_ver_code: __le32,
// VERMAGIC_STRING
    pub kernel_ver: [c_char; 64],
// room for growth w/out changing binary format
    pub unused: [u8; 128],
// struct ath10k_tlv_dump_data + more
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_dump_ram_data_hdr {
// enum ath10k_mem_region_type
    pub region_type: __le32,
    pub start: __le32,
// length of payload data, not including this header
    pub length: __le32,
    pub data: [u8; ],
}

// magic number to fill the holes not copied due to sections in regions
pub const ATH10K_MAGIC_NOT_COPIED: c_uint = 0xAA;
// part of user space ABI
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_mem_region_type {
    ATH10K_MEM_REGION_TYPE_REG	= 1,
    ATH10K_MEM_REGION_TYPE_DRAM	= 2,
    ATH10K_MEM_REGION_TYPE_AXI	= 3,
    ATH10K_MEM_REGION_TYPE_IRAM1	= 4,
    ATH10K_MEM_REGION_TYPE_IRAM2	= 5,
    ATH10K_MEM_REGION_TYPE_IOSRAM	= 6,
    ATH10K_MEM_REGION_TYPE_IOREG	= 7,
    ATH10K_MEM_REGION_TYPE_MSA	= 8,
}

// Define a section of the region which should be copied. As not all parts
// of the memory is possible to copy, for example some of the registers can
// be like that, sections can be used to define what is safe to copy.
//
// To minimize the size of the array, the list must obey the format:
// '{start0,stop0},{start1,stop1},{start2,stop2}....' The values below must
// also obey to 'start0 < stop0 < start1 < stop1 < start2 < ...', otherwise
// we may encounter error in the dump processing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_mem_section {
    pub start: u32,
    pub end: u32,
}

// One region of a memory layout. If the sections field is null entire
// region is copied. If sections is non-null only the areas specified in
// sections are copied and rest of the areas are filled with
// ATH10K_MAGIC_NOT_COPIED.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_mem_region {
    pub type: ath10k_mem_region_type,
    pub start: u32,
    pub len: u32,
    pub name: *const c_char,
    pub sections: *const ath10k_mem_section,
    pub size: u32,
    pub section_table: },
}

// Contains the memory layout of a hardware version identified with the
// hardware id, split into regions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hw_mem_layout {
    pub hw_id: u32,
    pub hw_rev: u32,
    pub bus: ath10k_bus,
    pub regions: *const ath10k_mem_region,
    pub size: c_int,
    pub region_table: },
}

// FIXME: where to put this?

extern "C" {
    pub fn ath10k_coredump_submit(ar: *mut ath10k) -> c_int;
}
extern "C" {
    pub fn ath10k_coredump_create(ar: *mut ath10k) -> c_int;
}
extern "C" {
    pub fn ath10k_coredump_register(ar: *mut ath10k) -> c_int;
}
extern "C" {
    pub fn ath10k_coredump_unregister(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_coredump_destroy(ar: *mut ath10k);
}

