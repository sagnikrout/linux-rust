//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/intel_gsc_binary_headers.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gsc_version {
    pub major: u16,
    pub minor: u16,
    pub hotfix: u16,
    pub build: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gsc_partition {
    pub offset: u32,
    pub size: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gsc_layout_pointers {
    pub rom_bypass_vector: [u8; 16],
// size of pointers layout not including ROM bypass vector
    pub size: u16,
//
// bit0: Backup copy of layout pointers exist
// bits1-15: reserved
//
    pub flags: u8,
    pub reserved: u8,
    pub crc32: u32,
    pub datap: intel_gsc_partition,
    pub boot1: intel_gsc_partition,
    pub boot2: intel_gsc_partition,
    pub boot3: intel_gsc_partition,
    pub boot4: intel_gsc_partition,
    pub boot5: intel_gsc_partition,
    pub temp_pages: intel_gsc_partition,
    pub __packed: },
// Boot partition structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gsc_bpdt_header {
    pub signature: u32,
pub const INTEL_GSC_BPDT_HEADER_SIGNATURE: c_uint = 0x000055AA;
    pub /: *mut *mut u16 descriptor_count; / num of entries after the header,
    pub version: u8,
    pub configuration: u8,
    pub crc32: u32,
    pub build_version: u32,
    pub tool_version: intel_gsc_version,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gsc_bpdt_entry {
//
// Bits 0-15: BPDT entry type
// Bits 16-17: reserved
// Bit 18: code sub-partition
// Bits 19-31: reserved
//
    pub type: u32,

pub const INTEL_GSC_BPDT_ENTRY_TYPE_GSC_RBE: c_uint = 0x1;
    pub /: *mut *mut u32 sub_partition_offset; / from the base of the BPDT header,
    pub sub_partition_size: u32,
    pub __packed: },
// Code partition directory (CPD) structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gsc_cpd_header_v2 {
    pub header_marker: u32,
pub const INTEL_GSC_CPD_HEADER_MARKER: c_uint = 0x44504324;
    pub num_of_entries: u32,
    pub header_version: u8,
    pub entry_version: u8,
    pub /: *mut *mut u8 header_length; / in bytes,
    pub flags: u8,
    pub partition_name: u32,
    pub crc32: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gsc_cpd_entry {
    pub name: [u8; 12],
//
// Bits 0-24: offset from the beginning of the code partition
// Bit 25: huffman compressed
// Bits 26-31: reserved
//
    pub offset: u32,

//
// Module/Item length, in bytes. For Huffman-compressed modules, this
// refers to the uncompressed size. For software-compressed modules,
// this refers to the compressed size.
//
    pub length: u32,
    pub reserved: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gsc_manifest_header {
    pub /: *mut *mut u32 header_type; / 0x4 for manifest type,
    pub /: *mut *mut u32 header_length; / in dwords,
    pub header_version: u32,
    pub flags: u32,
    pub vendor: u32,
    pub date: u32,
    pub /: *mut *mut u32 size; / In dwords, size of entire manifest (header + extensions),
    pub header_id: u32,
    pub internal_data: u32,
    pub fw_version: intel_gsc_version,
    pub security_version: u32,
    pub meu_kit_version: intel_gsc_version,
    pub meu_manifest_version: u32,
    pub general_data: [u8; 4],
    pub reserved3: [u8; 56],
    pub /: *mut *mut u32 modulus_size; / in dwords,
    pub /: *mut *mut u32 exponent_size; / in dwords,
    pub __packed: },
