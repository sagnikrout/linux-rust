//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/ocxl/ocxl_internal.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright 2017 IBM Corp.

pub const MAX_IRQ_PER_LINK: c_int = 2000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_fn {
    pub dev: device,
    pub bar_used: [c_int; 3],
    pub config: ocxl_fn_config,
    pub afu_list: list_head,
    pub pasid_base: c_int,
    pub actag_base: c_int,
    pub actag_enabled: c_int,
    pub actag_supported: c_int,
    pub pasid_list: list_head,
    pub actag_list: list_head,
    pub link: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_file_info {
    pub afu: *mut ocxl_afu,
    pub dev: device,
    pub cdev: cdev,
    pub attr_global_mmio: bin_attribute,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_afu {
    pub kref: kref,
    pub fn: *mut ocxl_fn,
    pub list: list_head,
    pub config: ocxl_afu_config,
    pub pasid_base: c_int,
    pub /: *mut *mut int pasid_count; / opened contexts,
    pub /: *mut *mut int pasid_max; / maximum number of contexts,
    pub actag_base: c_int,
    pub actag_enabled: c_int,
    pub contexts_lock: mutex,
    pub contexts_idr: idr,
    pub afu_control_lock: mutex,
    pub global_mmio_start: u64,
    pub irq_base_offset: u64,
    pub global_mmio_ptr: *mut void __iomem,
    pub pp_mmio_start: u64,
    pub private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocxl_context_status {
    CLOSED,
    OPENED,
    ATTACHED,
}

// Contains metadata about a translation fault
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_xsl_error {
    pub fault: u64 addr; // The address that triggered the,
    pub register: u64 dsisr; // the value of the dsisr,
    pub triggered: u64 count; // The number of times this fault has been,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_context {
    pub afu: *mut ocxl_afu,
    pub pasid: c_int,
    pub status_mutex: mutex,
    pub status: ocxl_context_status,
    pub mapping: *mut address_space,
    pub mapping_lock: mutex,
    pub events_wq: wait_queue_head_t,
    pub xsl_error_lock: mutex,
    pub xsl_error: ocxl_xsl_error,
    pub irq_lock: mutex,
    pub irq_idr: idr,
    pub implementation: u16 tidr; // Thread ID used for P9 wait,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_process_element {
    pub config_state: __be64,
    pub pasid: __be32,
    pub bdf: __be16,
    pub reserved1: __be16,
    pub reserved2: [__be32; 9],
    pub lpid: __be32,
    pub tid: __be32,
    pub pid: __be32,
    pub reserved3: [__be32; 10],
    pub amr: __be64,
    pub reserved4: [__be32; 3],
    pub software_state: __be32,
}

extern "C" {
    pub fn ocxl_file_register_afu(afu: *mut ocxl_afu) -> c_int;
}
extern "C" {
    pub fn ocxl_file_unregister_afu(afu: *mut ocxl_afu);
}
extern "C" {
    pub fn ocxl_file_init() -> c_int;
}
extern "C" {
    pub fn ocxl_file_exit();
}
extern "C" {
    pub fn ocxl_pasid_afu_alloc(fn: *mut ocxl_fn, size: u32) -> c_int;
}
extern "C" {
    pub fn ocxl_pasid_afu_free(fn: *mut ocxl_fn, start: u32, size: u32);
}
extern "C" {
    pub fn ocxl_actag_afu_alloc(fn: *mut ocxl_fn, size: u32) -> c_int;
}
extern "C" {
    pub fn ocxl_actag_afu_free(fn: *mut ocxl_fn, start: u32, size: u32);
}
//
// Get the max PASID value that can be used by the function
//
extern "C" {
    pub fn ocxl_config_get_pasid_info(dev: *mut pci_dev, count: *mut c_int) -> c_int;
}
//
// Control whether the FPGA is reloaded on a link reset
//
extern "C" {
    pub fn ocxl_config_get_reset_reload(dev: *mut pci_dev, val: *mut c_int) -> c_int;
}
extern "C" {
    pub fn ocxl_config_set_reset_reload(dev: *mut pci_dev, val: c_int) -> c_int;
}
//
// Check if an AFU index is valid for the given function.
//
// AFU indexes can be sparse, so a driver should check all indexes up
// to the maximum found in the function description
//
// ocxl_link_update_pe() - Update values within a Process Element
// @link_handle: the link handle associated with the process element
// @pasid: the PASID for the AFU context
// @tid: the new thread id for the process element
//
// Returns 0 on success
//
extern "C" {
    pub fn ocxl_link_update_pe(link_handle: *mut c_void, pasid: c_int, tid: __u16) -> c_int;
}
extern "C" {
    pub fn ocxl_context_detach_all(afu: *mut ocxl_afu);
}
extern "C" {
    pub fn ocxl_sysfs_register_afu(info: *mut ocxl_file_info) -> c_int;
}
extern "C" {
    pub fn ocxl_sysfs_unregister_afu(info: *mut ocxl_file_info);
}
extern "C" {
    pub fn ocxl_irq_offset_to_id(ctx: *mut ocxl_context, offset: u64) -> c_int;
}
extern "C" {
    pub fn ocxl_irq_id_to_offset(ctx: *mut ocxl_context, irq_id: c_int) -> u64;
}
extern "C" {
    pub fn ocxl_afu_irq_free_all(ctx: *mut ocxl_context);
}
