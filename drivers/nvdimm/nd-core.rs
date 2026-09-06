//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nvdimm/nd-core.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvdimm_bus {
    pub nd_desc: *mut nvdimm_bus_descriptor,
    pub wait: wait_queue_head_t,
    pub list: list_head,
    pub dev: device,
    pub probe_active: int id,,
    pub ioctl_active: core::sync::atomic::AtomicI32,
    pub mapping_list: list_head,
    pub reconfig_mutex: mutex,
    pub badrange: badrange,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvdimm {
    pub flags: c_ulong,
    pub provider_data: *mut c_void,
    pub cmd_mask: c_ulong,
    pub dev: device,
    pub busy: core::sync::atomic::AtomicI32,
    pub num_flush: int id,,
    pub flush_wpq: *mut resource,
    pub dimm_id: *const c_char,
    pub ops: *const nvdimm_security_ops,
    pub flags: c_ulong,
    pub ext_flags: c_ulong,
    pub overwrite_tmo: c_uint,
    pub overwrite_state: *mut kernfs_node,
    pub sec: },
    pub dwork: delayed_work,
    pub fw_ops: *const nvdimm_fw_ops,
}

// disabled, locked, unlocked, and overwrite are mutually exclusive
extern "C" {
    pub fn nvdimm_security_freeze(nvdimm: *mut nvdimm) -> c_int;
}

extern "C" {
    pub fn nvdimm_security_store(dev: *mut device, buf: *const c_char, len: usize) -> isize;
}
extern "C" {
    pub fn nvdimm_security_overwrite_query(work: *mut work_struct);
}

extern "C" {
    pub fn is_nvdimm(dev: *const device) -> bool;
}
extern "C" {
    pub fn is_nd_pmem(dev: *const device) -> bool;
}
extern "C" {
    pub fn is_nd_volatile(dev: *const device) -> bool;
}
extern "C" {
    pub fn is_nd_pmem(is_nd_volatile(dev: dev) ||) -> return;
}
extern "C" {
    pub fn is_nd_pmem(is_nd_volatile(dev: dev) ||) -> return;
}
extern "C" {
    pub fn nvdimm_bus_init() -> int __init;
}
extern "C" {
    pub fn nvdimm_bus_exit();
}
extern "C" {
    pub fn nvdimm_devs_exit();
}
extern "C" {
    pub fn nd_region_advance_seeds(nd_region: *mut nd_region, dev: *mut device);
}
extern "C" {
    pub fn nd_region_create_ns_seed(nd_region: *mut nd_region);
}
extern "C" {
    pub fn nd_region_create_btt_seed(nd_region: *mut nd_region);
}
extern "C" {
    pub fn nd_region_create_pfn_seed(nd_region: *mut nd_region);
}
extern "C" {
    pub fn nd_region_create_dax_seed(nd_region: *mut nd_region);
}
extern "C" {
    pub fn nvdimm_bus_create_ndctl(nvdimm_bus: *mut nvdimm_bus) -> c_int;
}
extern "C" {
    pub fn nvdimm_bus_destroy_ndctl(nvdimm_bus: *mut nvdimm_bus);
}
extern "C" {
    pub fn nd_synchronize();
}
extern "C" {
    pub fn nd_device_register(dev: *mut device);
}
extern "C" {
    pub fn nd_device_register_sync(dev: *mut device);
}
extern "C" {
    pub fn nd_is_uuid_unique(dev: *mut device, uuid: *mut uuid_t) -> bool;
}
extern "C" {
    pub fn nd_mapping_free_labels(nd_mapping: *mut nd_mapping);
}
extern "C" {
    pub fn __reserve_free_pmem(dev: *mut device, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn nd_region_allocatable_dpa(nd_region: *mut nd_region) -> resource_size_t;
}
extern "C" {
    pub fn nd_region_available_dpa(nd_region: *mut nd_region) -> resource_size_t;
}
extern "C" {
    pub fn nvdimm_num_label_slots(ndd: *mut nvdimm_drvdata) -> c_int;
}
extern "C" {
    pub fn get_ndd(ndd: *mut nvdimm_drvdata);
}
extern "C" {
    pub fn __nvdimm_namespace_capacity(ndns: *mut nd_namespace_common) -> resource_size_t;
}
extern "C" {
    pub fn nd_detach_ndns(dev: *mut device, _ndns: *mut nd_namespace_common);
}
extern "C" {
    pub fn __nd_detach_ndns(dev: *mut device, _ndns: *mut nd_namespace_common);
}
extern "C" {
    pub fn is_nvdimm_bus(dev: *mut device) -> bool;
}

extern "C" {
    pub fn devm_nsio_disable(dev: *mut device, nsio: *mut nd_namespace_io);
}

