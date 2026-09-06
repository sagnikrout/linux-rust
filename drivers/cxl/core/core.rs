//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/cxl/core/core.h
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
// Copyright(c) 2020 Intel Corporation.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxl_detach_mode {
    DETACH_ONLY,
    DETACH_INVALIDATE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_region_context {
    pub cxled: *mut cxl_endpoint_decoder,
    pub hpa_range: range,
    pub interleave_ways: c_int,
    pub interleave_granularity: c_int,
}

extern "C" {
    pub fn cxl_region_init() -> c_int;
}
extern "C" {
    pub fn cxl_region_exit();
}
extern "C" {
    pub fn cxl_get_poison_by_endpoint(port: *mut cxl_port) -> c_int;
}
extern "C" {
    pub fn devm_cxl_add_dax_region(cxlr: *mut cxl_region) -> c_int;
}
extern "C" {
    pub fn devm_cxl_add_pmem_region(cxlr: *mut cxl_region) -> c_int;
}
extern "C" {
    pub fn kill_regions(cxlrd: *mut cxl_root_decoder);
}

// Macro flag: #define SET_CXL_REGION_ATTR(x)

extern "C" {
    pub fn cxl_send_cmd(cxl_mbox: *mut cxl_mailbox, s: *mut cxl_send_command __user) -> c_int;
}
extern "C" {
    pub fn cxl_mem_get_partition_info(mds: *mut cxl_memdev_state) -> c_int;
}
extern "C" {
    pub fn cxl_dpa_alloc(cxled: *mut cxl_endpoint_decoder, size: u64) -> c_int;
}
extern "C" {
    pub fn cxl_dpa_free(cxled: *mut cxl_endpoint_decoder) -> c_int;
}
extern "C" {
    pub fn cxl_dpa_size(cxled: *mut cxl_endpoint_decoder) -> resource_size_t;
}
extern "C" {
    pub fn cxl_dpa_resource_start(cxled: *mut cxl_endpoint_decoder) -> resource_size_t;
}
extern "C" {
    pub fn cxl_resource_contains_addr(res: *const resource, addr: resource_size_t) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxl_rcrb {
    CXL_RCRB_DOWNSTREAM,
    CXL_RCRB_UPSTREAM,
}

extern "C" {
    pub fn cxl_rcrb_to_aer(dev: *mut device, rcrb: resource_size_t) -> u16;
}

pub const PCI_CAP_EXP_SIZEOF: c_uint = 0x3c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_rwsem {
//
// All changes to HPA (interleave configuration) occur with this
// lock held for write.
//
    pub region: rw_semaphore,
//
// All changes to a device DPA space occur with this lock held
// for write.
//
    pub dpa: rw_semaphore,
}

extern "C" {
    pub fn cxl_memdev_init() -> c_int;
}
extern "C" {
    pub fn cxl_memdev_exit();
}
extern "C" {
    pub fn cxl_mbox_init();
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxl_poison_trace_type {
    CXL_POISON_TRACE_LIST,
    CXL_POISON_TRACE_INJECT,
    CXL_POISON_TRACE_CLEAR,
}

extern "C" {
    pub fn cxl_pci_get_latency(pdev: *mut pci_dev) -> c_long;
}
extern "C" {
    pub fn cxl_pci_get_bandwidth(pdev: *mut pci_dev, c: *mut access_coordinate) -> c_int;
}
//
// The host of CXL root port and the first level of ports is
// the platform firmware device, the host of all other ports
// is their parent port.
//

extern "C" {
    pub fn cxl_ras_init();
}
extern "C" {
    pub fn cxl_ras_exit();
}
extern "C" {
    pub fn cxl_handle_ras(dev: *mut device, ras_base: *mut void __iomem) -> bool;
}
extern "C" {
    pub fn cxl_handle_cor_ras(dev: *mut device, ras_base: *mut void __iomem);
}
extern "C" {
    pub fn cxl_dport_map_rch_aer(dport: *mut cxl_dport);
}
extern "C" {
    pub fn cxl_disable_rch_root_ints(dport: *mut cxl_dport);
}
extern "C" {
    pub fn cxl_handle_rdport_errors(cxlds: *mut cxl_dev_state);
}
extern "C" {
    pub fn devm_cxl_dport_ras_setup(dport: *mut cxl_dport);
}

extern "C" {
    pub fn cxl_gpf_port_setup(dport: *mut cxl_dport) -> c_int;
}
extern "C" {
    pub fn cxl_port_get_possible_dports(port: *mut cxl_port) -> c_int;
}

