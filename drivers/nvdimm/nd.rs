//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nvdimm/nd.h
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
// Limits the maximum number of block apertures a dimm can
// support and is an input to the geometry/on-disk-format of a
// BTT instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvdimm_drvdata {
    pub dev: *mut device,
    pub nslabel_size: c_int,
    pub nsarea: nd_cmd_get_config_size,
    pub data: *mut c_void,
    pub cxl: bool,
    pub ns_next: int ns_current,,
    pub dpa: resource,
    pub kref: kref,
}

extern "C" {
    pub fn memcpy(_arg: name, _arg: nd_label->cxl.name, _arg: NSLABEL_NAME_LEN) -> return;
}
extern "C" {
    pub fn memcpy(_arg: name, _arg: nd_label->efi.name, _arg: NSLABEL_NAME_LEN) -> return;
}
extern "C" {
    pub fn memcpy(_arg: nd_label->cxl.name, _arg: name, _arg: NSLABEL_NAME_LEN) -> return;
}
extern "C" {
    pub fn memcpy(_arg: nd_label->efi.name, _arg: name, _arg: NSLABEL_NAME_LEN) -> return;
}
extern "C" {
    pub fn __le32_to_cpu(_arg: nd_label->cxl.slot) -> return;
}
extern "C" {
    pub fn __le32_to_cpu(_arg: nd_label->efi.slot) -> return;
}
extern "C" {
    pub fn __le64_to_cpu(_arg: nd_label->cxl.checksum) -> return;
}
extern "C" {
    pub fn __le64_to_cpu(_arg: nd_label->efi.checksum) -> return;
}
extern "C" {
    pub fn __le32_to_cpu(_arg: nd_label->cxl.flags) -> return;
}
extern "C" {
    pub fn __le32_to_cpu(_arg: nd_label->efi.flags) -> return;
}
extern "C" {
    pub fn __le64_to_cpu(_arg: nd_label->cxl.dpa) -> return;
}
extern "C" {
    pub fn __le64_to_cpu(_arg: nd_label->efi.dpa) -> return;
}
extern "C" {
    pub fn __le64_to_cpu(_arg: nd_label->cxl.rawsize) -> return;
}
extern "C" {
    pub fn __le64_to_cpu(_arg: nd_label->efi.rawsize) -> return;
}
// WARN future refactor attempts that break this assumption
extern "C" {
    pub fn __le64_to_cpu(_arg: nd_label->efi.isetcookie) -> return;
}
//
// Let the EFI and CXL validation comingle, where fields that
// don't matter to CXL always validate.
//
extern "C" {
    pub fn __le16_to_cpu(_arg: nd_label->cxl.position) -> return;
}
extern "C" {
    pub fn __le16_to_cpu(_arg: nd_label->efi.position) -> return;
}
extern "C" {
    pub fn __le16_to_cpu(_arg: nd_label->efi.nlabel) -> return;
}
extern "C" {
    pub fn __le16_to_cpu(_arg: nd_label->cxl.nrange) -> return;
}
//
// Yes, for some reason the EFI labels convey a massive 64-bit
// lbasize, that got fixed for CXL.
//
extern "C" {
    pub fn __le16_to_cpu(_arg: nd_label->cxl.lbasize) -> return;
}
extern "C" {
    pub fn __le64_to_cpu(_arg: nd_label->efi.lbasize) -> return;
}
extern "C" {
    pub fn uuid_equal(_arg: &tmp, _arg: uuid) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_region_data {
    pub ns_count: c_int,
    pub ns_active: c_int,
    pub hints_shift: c_uint,
    pub flush_wpq: [*mut void __iomem; ],
}

extern "C" {
    pub fn to_namespace_index(_arg: ndd, _arg: ndd->ns_current) -> return;
}
extern "C" {
    pub fn to_namespace_index(_arg: ndd, _arg: ndd->ns_next) -> return;
}
extern "C" {
    pub fn sizeof_namespace_label(ndd: *mut nvdimm_drvdata) -> unsigned;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nd_label_flags {
    ND_LABEL_REAP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_label_ent {
    pub list: list_head,
    pub flags: c_ulong,
    pub label: *mut nd_namespace_label,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nd_mapping_lock_class {
    ND_MAPPING_CLASS0,
    ND_MAPPING_UUID_SCAN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_mapping {
    pub nvdimm: *mut nvdimm,
    pub start: u64,
    pub size: u64,
    pub position: c_int,
    pub labels: list_head,
    pub lock: mutex,
//
// @ndd is for private use at region enable / disable time for
// get_ndd() + put_ndd(), all other nd_mapping to ndd
// conversions use to_ndd() which respects enabled state of the
// nvdimm.
//
    pub ndd: *mut nvdimm_drvdata,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_lane {
    pub /: *mut *mut mutex lock; / serialize lane access,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_region {
    pub dev: device,
    pub ns_ida: ida,
    pub btt_ida: ida,
    pub pfn_ida: ida,
    pub dax_ida: ida,
    pub flags: c_ulong,
    pub ns_seed: *mut device,
    pub btt_seed: *mut device,
    pub pfn_seed: *mut device,
    pub dax_seed: *mut device,
    pub align: c_ulong,
    pub ndr_mappings: u16,
    pub ndr_size: u64,
    pub ndr_start: u64,
    pub target_node: int id, num_lanes, ro, numa_node,,
    pub provider_data: *mut c_void,
    pub bb_state: *mut kernfs_node,
    pub bb: badblocks,
    pub nd_set: *mut nd_interleave_set,
    pub lane: *mut nd_lane,
    pub bio): *mut *mut *mut int (flush)(struct nd_region nd_region, struct bio,
    pub __counted_by(ndr_mappings): nd_mapping mapping[],
}

//
// Lookup next in the repeating sequence of 01, 10, and 11.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_btt {
    pub dev: device,
    pub ndns: *mut nd_namespace_common,
    pub btt: *mut btt,
    pub lbasize: c_ulong,
    pub size: u64,
    pub uuid: *mut uuid_t,
    pub id: c_int,
    pub initial_offset: c_int,
    pub version_major: u16,
    pub version_minor: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nd_pfn_mode {
    PFN_MODE_NONE,
    PFN_MODE_RAM,
    PFN_MODE_PMEM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_pfn {
    pub id: c_int,
    pub uuid: *mut uuid_t,
    pub dev: device,
    pub align: c_ulong,
    pub npfns: c_ulong,
    pub mode: nd_pfn_mode,
    pub pfn_sb: *mut nd_pfn_sb,
    pub ndns: *mut nd_namespace_common,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_dax {
    pub nd_pfn: nd_pfn,
}

extern "C" {
    pub fn ALIGN(_arg: SZ_8K, _arg: PAGE_SIZE) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nd_async_mode {
    ND_SYNC,
    ND_ASYNC,
}

extern "C" {
    pub fn wait_nvdimm_bus_probe_idle(dev: *mut device);
}
extern "C" {
    pub fn nd_device_register(dev: *mut device);
}
extern "C" {
    pub fn nd_device_unregister(dev: *mut device, mode: nd_async_mode);
}
extern "C" {
    pub fn nd_device_notify(dev: *mut device, event: nvdimm_event);
}
extern "C" {
    pub fn nvdimm_init() -> int __init;
}
extern "C" {
    pub fn nd_region_init() -> int __init;
}
extern "C" {
    pub fn nd_label_init() -> int __init;
}
extern "C" {
    pub fn nvdimm_exit();
}
extern "C" {
    pub fn nd_region_exit();
}
extern "C" {
    pub fn nvdimm_check_config_data(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn nvdimm_init_nsarea(ndd: *mut nvdimm_drvdata) -> c_int;
}
extern "C" {
    pub fn nvdimm_init_config_data(ndd: *mut nvdimm_drvdata) -> c_int;
}
extern "C" {
    pub fn nvdimm_set_labeling(dev: *mut device);
}
extern "C" {
    pub fn nvdimm_set_locked(dev: *mut device);
}
extern "C" {
    pub fn nvdimm_clear_locked(dev: *mut device);
}
extern "C" {
    pub fn nvdimm_security_setup_events(dev: *mut device) -> c_int;
}

extern "C" {
    pub fn nvdimm_security_unlock(dev: *mut device) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_gen_sb {
    pub 8]: char reserved[SZ_4K -,
    pub checksum: __le64,
}

extern "C" {
    pub fn nd_sb_checksum(sb: *mut nd_gen_sb) -> u64;
}

extern "C" {
    pub fn nd_btt_probe(dev: *mut device, ndns: *mut nd_namespace_common) -> c_int;
}
extern "C" {
    pub fn is_nd_btt(dev: *mut device) -> bool;
}

pub const MAX_NVDIMM_ALIGN: c_int = 4;
extern "C" {
    pub fn nd_pfn_probe(dev: *mut device, ndns: *mut nd_namespace_common) -> c_int;
}
extern "C" {
    pub fn is_nd_pfn(dev: *mut device) -> bool;
}
extern "C" {
    pub fn nd_pfn_validate(nd_pfn: *mut nd_pfn, sig: *const c_char) -> c_int;
}

extern "C" {
    pub fn nd_dax_probe(dev: *mut device, ndns: *mut nd_namespace_common) -> c_int;
}
extern "C" {
    pub fn is_nd_dax(dev: *const device) -> bool;
}
extern "C" {
    pub fn nd_pfn_devinit(_arg: &nd_dax->nd_pfn, _arg: ndns) -> return;
}

extern "C" {
    pub fn nd_region_to_nstype(nd_region: *mut nd_region) -> c_int;
}
extern "C" {
    pub fn nd_region_register_namespaces(nd_region: *mut nd_region, err: *mut c_int) -> c_int;
}
extern "C" {
    pub fn nd_region_interleave_set_altcookie(nd_region: *mut nd_region) -> u64;
}
extern "C" {
    pub fn nvdimm_bus_lock(dev: *mut device);
}
extern "C" {
    pub fn nvdimm_bus_unlock(dev: *mut device);
}
extern "C" {
    pub fn is_nvdimm_bus_locked(dev: *mut device) -> bool;
}
extern "C" {
    pub fn nvdimm_check_and_set_ro(disk: *mut gendisk);
}
extern "C" {
    pub fn nvdimm_drvdata_release(kref: *mut kref);
}
extern "C" {
    pub fn put_ndd(ndd: *mut nvdimm_drvdata);
}
extern "C" {
    pub fn nd_label_reserve_dpa(ndd: *mut nvdimm_drvdata) -> c_int;
}
extern "C" {
    pub fn nvdimm_free_dpa(ndd: *mut nvdimm_drvdata, res: *mut resource);
}
extern "C" {
    pub fn nvdimm_namespace_capacity(ndns: *mut nd_namespace_common) -> resource_size_t;
}
extern "C" {
    pub fn nvdimm_namespace_locked(ndns: *mut nd_namespace_common) -> bool;
}
extern "C" {
    pub fn nvdimm_namespace_attach_btt(ndns: *mut nd_namespace_common) -> c_int;
}
extern "C" {
    pub fn nvdimm_namespace_detach_btt(nd_btt: *mut nd_btt) -> c_int;
}
extern "C" {
    pub fn pmem_sector_size(ndns: *mut nd_namespace_common) -> c_uint;
}

// max struct page size independent of kernel config
pub const MAX_STRUCT_PAGE_SIZE: c_int = 64;
extern "C" {
    pub fn nvdimm_setup_pfn(nd_pfn: *mut nd_pfn, pgmap: *mut dev_pagemap) -> c_int;
}

extern "C" {
    pub fn nd_region_activate(nd_region: *mut nd_region) -> c_int;
}
extern "C" {
    pub fn pmem_should_map_pages(dev: *mut device) -> bool;
}
