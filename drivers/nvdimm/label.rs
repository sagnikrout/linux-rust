//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nvdimm/label.h
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
// Copyright(c) 2013-2015 Intel Corporation. All rights reserved.
//

//
// struct nd_namespace_index - label set superblock
// @sig: NAMESPACE_INDEX\0
// @flags: placeholder
// @labelsize: log2 size (v1 labels 128 bytes v2 labels 256 bytes)
// @seq: sequence number for this index
// @myoff: offset of this index in label area
// @mysize: size of this index struct
// @otheroff: offset of other index
// @labeloff: offset of first label slot
// @nslot: total number of label slots
// @major: label area major version
// @minor: label area minor version
// @checksum: fletcher64 of all fields
// @free: bitmap, nlabel bits
//
// The size of free[] is rounded up so the total struct size is a
// multiple of NSINDEX_ALIGN bytes.  Any bits this allocates beyond
// nlabel bits must be zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_namespace_index {
    pub sig: [u8; NSINDEX_SIG_LEN],
    pub flags: [u8; 3],
    pub labelsize: u8,
    pub seq: __le32,
    pub myoff: __le64,
    pub mysize: __le64,
    pub otheroff: __le64,
    pub labeloff: __le64,
    pub nslot: __le32,
    pub major: __le16,
    pub minor: __le16,
    pub checksum: __le64,
    pub free: [u8; ],
}

//
// struct cxl_region_label - CXL 2.0 Table 211
// @type: uuid identifying this label format (region)
// @uuid: uuid for the region this label describes
// @flags: NSLABEL_FLAG_UPDATING (all other flags reserved)
// @nlabel: 1 per interleave-way in the region
// @position: this label's position in the set
// @dpa: start address in device-local capacity for this label
// @rawsize: size of this label's contribution to region
// @hpa: mandatory system physical address to map this region
// @slot: slot id of this label in label area
// @ig: interleave granularity (1 << @ig) * 256 bytes
// @align: alignment in SZ_256M blocks
// @reserved: reserved
// @checksum: fletcher64 sum of this label
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_region_label {
    pub type: [u8; NSLABEL_UUID_LEN],
    pub uuid: [u8; NSLABEL_UUID_LEN],
    pub flags: __le32,
    pub nlabel: __le16,
    pub position: __le16,
    pub dpa: __le64,
    pub rawsize: __le64,
    pub hpa: __le64,
    pub slot: __le32,
    pub ig: __le32,
    pub align: __le32,
    pub reserved: [u8; 0xac],
    pub checksum: __le64,
}

//
// struct nvdimm_efi_label - namespace superblock
// @uuid: UUID per RFC 4122
// @name: optional name (NULL-terminated)
// @flags: see NSLABEL_FLAG_
// @nlabel: num labels to describe this ns
// @position: labels position in set
// @isetcookie: interleave set cookie
// @lbasize: LBA size in bytes or 0 for pmem
// @dpa: DPA of NVM range on this DIMM
// @rawsize: size of namespace
// @slot: slot of this label in label area
// @align: physical address alignment of the namespace
// @reserved: reserved
// @type_guid: copy of struct acpi_nfit_system_address.range_guid
// @abstraction_guid: personality id (btt, btt2, fsdax, devdax....)
// @reserved2: reserved
// @checksum: fletcher64 sum of this object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvdimm_efi_label {
    pub uuid: [u8; NSLABEL_UUID_LEN],
    pub name: [u8; NSLABEL_NAME_LEN],
    pub flags: __le32,
    pub nlabel: __le16,
    pub position: __le16,
    pub isetcookie: __le64,
    pub lbasize: __le64,
    pub dpa: __le64,
    pub rawsize: __le64,
    pub slot: __le32,
//
// Accessing fields past this point should be gated by a
// efi_namespace_label_has() check.
//
    pub align: u8,
    pub reserved: [u8; 3],
    pub type_guid: guid_t,
    pub abstraction_guid: guid_t,
    pub reserved2: [u8; 88],
    pub checksum: __le64,
}

//
// struct nvdimm_cxl_label - CXL 2.0 Table 212
// @type: uuid identifying this label format (namespace)
// @uuid: uuid for the namespace this label describes
// @name: friendly name for the namespace
// @flags: NSLABEL_FLAG_UPDATING (all other flags reserved)
// @nrange: discontiguous namespace support
// @position: this label's position in the set
// @dpa: start address in device-local capacity for this label
// @rawsize: size of this label's contribution to namespace
// @slot: slot id of this label in label area
// @align: alignment in SZ_256M blocks
// @region_uuid: host interleave set identifier
// @abstraction_uuid: personality driver for this namespace
// @lbasize: address geometry for disk-like personalities
// @reserved: reserved
// @checksum: fletcher64 sum of this label
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvdimm_cxl_label {
    pub type: [u8; NSLABEL_UUID_LEN],
    pub uuid: [u8; NSLABEL_UUID_LEN],
    pub name: [u8; NSLABEL_NAME_LEN],
    pub flags: __le32,
    pub nrange: __le16,
    pub position: __le16,
    pub dpa: __le64,
    pub rawsize: __le64,
    pub slot: __le32,
    pub align: __le32,
    pub region_uuid: [u8; 16],
    pub abstraction_uuid: [u8; 16],
    pub lbasize: __le16,
    pub reserved: [u8; 0x56],
    pub checksum: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_namespace_label {
    pub cxl: nvdimm_cxl_label,
    pub efi: nvdimm_efi_label,
}

//
// struct nd_label_id - identifier string for dpa allocation
// @id: "pmem-<namespace uuid>"
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_label_id {
    pub id: [c_char; ND_LABEL_ID_SIZE],
}

//
// If the 'best' index is invalid, so is the 'next' index.  Otherwise,
// the next index is MOD(index+1, 2)
//
extern "C" {
    pub fn nd_label_data_init(ndd: *mut nvdimm_drvdata) -> c_int;
}
extern "C" {
    pub fn sizeof_namespace_index(ndd: *mut nvdimm_drvdata) -> usize;
}
extern "C" {
    pub fn nd_label_active_count(ndd: *mut nvdimm_drvdata) -> c_int;
}
extern "C" {
    pub fn nd_label_alloc_slot(ndd: *mut nvdimm_drvdata) -> u32;
}
extern "C" {
    pub fn nd_label_free_slot(ndd: *mut nvdimm_drvdata, slot: u32) -> bool;
}
extern "C" {
    pub fn nd_label_nfree(ndd: *mut nvdimm_drvdata) -> u32;
}
