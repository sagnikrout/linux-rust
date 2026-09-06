//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dax/dax-private.h
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
// Copyright(c) 2016 Intel Corporation. All rights reserved.
//

// private routines between core files
extern "C" {
    pub fn dax_bus_init() -> c_int;
}
extern "C" {
    pub fn dax_bus_exit();
}
//
// struct dax_region - mapping infrastructure for dax devices
// @id: kernel-wide unique region for a memory range
// @target_node: effective numa node if this memory range is onlined
// @kref: to pin while other agents have a need to do lookups
// @dev: parent device backing this region
// @align: allocation and mapping alignment for child dax devices
// @ida: instance id allocator
// @res: resource tree to track instance allocations
// @seed: allow userspace to find the first unbound seed device
// @youngest: allow userspace to find the most recently created device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dax_region {
    pub id: c_int,
    pub target_node: c_int,
    pub kref: kref,
    pub dev: *mut device,
    pub align: c_uint,
    pub ida: ida,
    pub res: resource,
    pub seed: *mut device,
    pub youngest: *mut device,
}

//
// struct dax_mapping - device to display mapping range attributes
// @dev: device representing this range
// @range_id: index within dev_dax ranges array
// @id: ida of this mapping
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dax_mapping {
    pub dev: device,
    pub range_id: c_int,
    pub id: c_int,
}

//
// struct dev_dax_range - tuple represenging a range of memory used by dev_dax
// @pgoff: page offset
// @range: resource-span
// @mapping: reference to the dax_mapping for this range
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_dax_range {
    pub pgoff: c_ulong,
    pub range: range,
    pub mapping: *mut dax_mapping,
}

//
// struct dev_dax - instance data for a subdivision of a dax region, and
// data while the device is activated in the driver.
// @region: parent region
// @dax_dev: core dax functionality
// @cached_size: size of daxdev cached by fsdev_dax
// @align: alignment of this instance
// @target_node: effective numa node if dev_dax memory range is onlined
// @dyn_id: is this a dynamic or statically created instance
// @id: ida allocated id when the dax_region is not static
// @ida: mapping id allocator
// @dev: device core
// @pgmap: pgmap for memmap setup / lifetime (driver owned)
// @memmap_on_memory: allow kmem to put the memmap in the memory
// @nr_range: size of @ranges
// @ranges: range tuples of memory used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_dax {
    pub region: *mut dax_region,
    pub dax_dev: *mut dax_device,
    pub cached_size: u64,
    pub align: c_uint,
    pub target_node: c_int,
    pub dyn_id: bool,
    pub id: c_int,
    pub ida: ida,
    pub dev: device,
    pub pgmap: *mut dev_pagemap,
    pub memmap_on_memory: bool,
    pub nr_range: c_int,
    pub ranges: *mut dev_dax_range,
}

//
// While run_dax() is potentially a generic operation that could be
// defined in include/linux/dax.h we don't want to grow any users
// outside of drivers/dax
//
extern "C" {
    pub fn run_dax(dax_dev: *mut dax_device);
}
extern "C" {
    pub fn container_of(_arg: dev, dev_dax: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: dev, dax_mapping: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn dax_pgoff_to_phys(dev_dax: *mut dev_dax, pgoff: pgoff_t, size: c_ulong) -> phys_addr_t;
}

