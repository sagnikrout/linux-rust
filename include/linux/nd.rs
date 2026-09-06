//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nd.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvdimm_event {
    NVDIMM_REVALIDATE_POISON,
    NVDIMM_REVALIDATE_REGION,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvdimm_claim_class {
    NVDIMM_CCLASS_NONE,
    NVDIMM_CCLASS_BTT,
    NVDIMM_CCLASS_BTT2,
    NVDIMM_CCLASS_PFN,
    NVDIMM_CCLASS_DAX,
    NVDIMM_CCLASS_UNKNOWN,
}

// Event attribute array index
pub const NVDIMM_PMU_FORMAT_ATTR: c_int = 0;
pub const NVDIMM_PMU_EVENT_ATTR: c_int = 1;
pub const NVDIMM_PMU_CPUMASK_ATTR: c_int = 2;
pub const NVDIMM_PMU_NULL_ATTR: c_int = 3;
//
// struct nvdimm_pmu - data structure for nvdimm perf driver
// @pmu: pmu data structure for nvdimm performance stats.
// @dev: nvdimm device pointer.
// @cpu: designated cpu for counter access.
// @node: node for cpu hotplug notifier link.
// @cpuhp_state: state for cpu hotplug notification.
// @arch_cpumask: cpumask to get designated cpu for counter access.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvdimm_pmu {
    pub pmu: pmu,
    pub dev: *mut device,
    pub cpu: c_int,
    pub node: hlist_node,
    pub cpuhp_state: cpuhp_state,
// cpumask provided by arch/platform specific code
    pub arch_cpumask: cpumask,
}

extern "C" {
    pub fn register_nvdimm_pmu(nvdimm: *mut nvdimm_pmu, pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn unregister_nvdimm_pmu(nd_pmu: *mut nvdimm_pmu);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_device_driver {
    pub drv: device_driver,
    pub type: c_ulong,
    pub dev): *mut *mut int (probe)(struct device,
    pub dev): *mut *mut void (remove)(struct device,
    pub dev): *mut *mut void (shutdown)(struct device,
    pub event): *mut *mut *mut void (notify)(struct device dev, enum nvdimm_event,
}

//
// struct nd_namespace_common - core infrastructure of a namespace
// @force_raw: ignore other personalities for the namespace (e.g. btt)
// @dev: device model node
// @claim: when set a another personality has taken ownership of the namespace
// @claim_class: restrict claim type to a given class
// @rw_bytes: access the raw namespace capacity with byte-aligned transfers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_namespace_common {
    pub force_raw: c_int,
    pub dev: device,
    pub claim: *mut device,
    pub claim_class: nvdimm_claim_class,
    pub flags): *mut *mut void buf, size_t size, int rw, unsigned long,
}

extern "C" {
    pub fn container_of(_arg: dev, nd_namespace_common: struct, _arg: dev) -> return;
}
//
// struct nd_namespace_io - device representation of a persistent memory range
// @common: namespace device core infrastructure created by the nd region driver
// @res: struct resource conversion of a NFIT SPA table
// @size: cached resource_size(@res) for fast path size checks
// @addr: virtual address to access the namespace range
// @bb: badblocks list for the namespace range
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_namespace_io {
    pub common: nd_namespace_common,
    pub res: resource,
    pub size: resource_size_t,
    pub addr: *mut c_void,
    pub bb: badblocks,
}

//
// struct nd_namespace_pmem - namespace device for dimm-backed interleaved memory
// @nsio: device and system physical address range to drive
// @lbasize: logical sector size for the namespace in block-device-mode
// @alt_name: namespace name supplied in the dimm label
// @uuid: namespace name supplied in the dimm label
// @id: ida allocated id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_namespace_pmem {
    pub nsio: nd_namespace_io,
    pub lbasize: c_ulong,
    pub alt_name: *mut c_char,
    pub uuid: *mut uuid_t,
    pub id: c_int,
}

extern "C" {
    pub fn container_of(_arg: dev, nd_namespace_io: struct, _arg: common.dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: nsio, nd_namespace_pmem: struct, _arg: nsio) -> return;
}
//
// nvdimm_read_bytes() - synchronously read bytes from an nvdimm namespace
// @ndns: device to read
// @offset: namespace-relative starting offset
// @buf: buffer to fill
// @size: transfer length
// @flags: process (0) or atomic (1) context
//
// @buf is up-to-date upon return from this routine.
//
// Returns: %0 on success or a negative error code on failure
//
// nvdimm_write_bytes() - synchronously write bytes to an nvdimm namespace
// @ndns: device to write
// @offset: namespace-relative starting offset
// @buf: buffer to drain
// @size: transfer length
// @flags: process (0) or atomic (1) context
//
// NVDIMM Namepaces disks do not implement sectors internally.  Depending on
// the @ndns, the contents of @buf may be in cpu cache, platform buffers,
// or on backing memory media upon return from this routine.  Flushing
// to media is handled internal to the @ndns driver, if at all.
//
// Returns: %0 on success or a negative error code on failure
//

extern "C" {
    pub fn nvdimm_region_notify(nd_region: *mut nd_region, event: nvdimm_event);
}

