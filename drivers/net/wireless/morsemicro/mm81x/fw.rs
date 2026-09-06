//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/morsemicro/mm81x/fw.h
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
// Copyright (c) 2017-2026 Morse Micro
//

pub const MM81X_FW_VER_MIN: c_int = 56;
// FW_CAPABILITIES_FLAGS_WIDTH = ceil(MM81X_CAPS_MAX_HW_LEN / 32)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_elf32_ehdr {
    pub e_ident: [c_uchar; EI_NIDENT],
    pub e_type: __le16,
    pub e_machine: __le16,
    pub e_version: __le32,
    pub e_entry: __le32,
    pub e_phoff: __le32,
    pub e_shoff: __le32,
    pub e_flags: __le32,
    pub e_ehsize: __le16,
    pub e_phentsize: __le16,
    pub e_phnum: __le16,
    pub e_shentsize: __le16,
    pub e_shnum: __le16,
    pub e_shstrndx: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_elf32_shdr {
    pub sh_name: __le32,
    pub sh_type: __le32,
    pub sh_flags: __le32,
    pub sh_addr: __le32,
    pub sh_offset: __le32,
    pub sh_size: __le32,
    pub sh_link: __le32,
    pub sh_info: __le32,
    pub sh_addralign: __le32,
    pub sh_entsize: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_elf32_phdr {
    pub p_type: __le32,
    pub p_offset: __le32,
    pub p_vaddr: __le32,
    pub p_paddr: __le32,
    pub p_filesz: __le32,
    pub p_memsz: __le32,
    pub p_flags: __le32,
    pub p_align: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_fw_info_tlv_type {
    MM81X_FW_INFO_TLV_BCF_ADDR = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_fw_info_tlv {
    pub type: __le16,
    pub length: __le16,
    pub val: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_fw_ext_host_tbl_tag {
// The S1G capability tag
    MM81X_FW_HOST_TABLE_TAG_S1G_CAPABILITIES = 0,
    MM81X_FW_HOST_TABLE_TAG_PAGER_BYPASS_TX_STATUS = 1,
    MM81X_FW_HOST_TABLE_TAG_INSERT_SKB_CHECKSUM = 2,
    MM81X_FW_HOST_TABLE_TAG_YAPS_TABLE = 3,
    MM81X_FW_HOST_TABLE_TAG_PAGER_PKT_MEMORY = 4,
    MM81X_FW_HOST_TABLE_TAG_PAGER_BYPASS_CMD_RESP = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_host_tbl_tlv_hdr {
// The tag used to identify which capability this represents
    pub tag: __le16,
// The length of the capability structure including this header
    pub length: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_host_tbl_s1g_caps {
    pub header: ext_host_tbl_tlv_hdr,
    pub flags: [__le32; FW_CAPABILITIES_FLAGS_WIDTH],
//
// The minimum A-MPDU start spacing required by firmware.
// Value | Description
// ------|------------
// 0     | No restriction
// 1     | 1/4 us
// 2     | 1/2 us
// 3     | 1 us
// 4     | 2 us
// 5     | 4 us
// 6     | 8 us
// 7     | 16 us
//
    pub ampdu_mss: u8,
    pub beamformee_sts_capability: u8,
    pub number_sounding_dimensions: u8,
//
// The maximum A-MPDU length. This is the exponent value such that
// (2^(13 + exponent) - 1) is the length
//
    pub maximum_ampdu_length: u8,
//
// Offset to apply to the specification's MMSS table to signal further
// minimum MPDU start spacing.
//
    pub mm81x_mmss_offset: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_host_tbl_insert_skb_checksum {
    pub header: ext_host_tbl_tlv_hdr,
    pub insert_and_validate_checksum: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_host_tbl_yaps_table {
    pub header: ext_host_tbl_tlv_hdr,
    pub yaps_table: mm81x_yaps_hw_table,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_host_tbl {
    pub ext_host_tbl_length: __le32,
    pub dev_mac_addr: [u8; 6],
    pub ext_host_table_data_tlvs: [u8; ],
    pub __packed: },
    pub reset): *mut *mut int mm81x_fw_init(struct mm81x mors, bool,
    pub mors): *mut int mm81x_fw_parse_ext_host_tbl(struct mm81x,
