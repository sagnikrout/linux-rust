//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fwnode.h
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
// fwnode.h - Firmware device node object handle type definition.
//
// This header file provides low-level data types and definitions for firmware
// and device property providers. The respective API header files supplied by
// them should contain all of the requisite data types and definitions for end
// users, so including it directly should not be necessary.
//
// Copyright (C) 2015, Intel Corporation
// Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_dma_attr {
    DEV_DMA_NOT_SUPPORTED,
    DEV_DMA_NON_COHERENT,
    DEV_DMA_COHERENT,
}

//
// fwnode flags
//
// LINKS_ADDED:	The fwnode has already be parsed to add fwnode links.
// NOT_DEVICE:	The fwnode will never be populated as a struct device.
// INITIALIZED: The hardware corresponding to fwnode has been initialized.
// NEEDS_CHILD_BOUND_ON_ADD: For this fwnode/device to probe successfully, its
// driver needs its child devices to be bound with
// their respective drivers as soon as they are
// added.
// BEST_EFFORT: The fwnode/device needs to probe early and might be missing some
// suppliers. Only enforce ordering with suppliers that have
// drivers.
//
pub const FWNODE_FLAG_LINKS_ADDED: c_int = 0;
pub const FWNODE_FLAG_NOT_DEVICE: c_int = 1;
pub const FWNODE_FLAG_INITIALIZED: c_int = 2;
pub const FWNODE_FLAG_NEEDS_CHILD_BOUND_ON_ADD: c_int = 3;
pub const FWNODE_FLAG_BEST_EFFORT: c_int = 4;
pub const FWNODE_FLAG_VISITED: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwnode_handle {
    pub secondary: *mut fwnode_handle,
    pub ops: *const fwnode_operations,
// The below is used solely by device links, don't use otherwise
    pub dev: *mut device,
    pub suppliers: list_head,
    pub consumers: list_head,
    pub flags: c_ulong,
}

//
// fwnode link flags
//
// CYCLE:	The fwnode link is part of a cycle. Don't defer probe.
// IGNORE:	Completely ignore this link, even during cycle detection.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwnode_link {
    pub supplier: *mut fwnode_handle,
    pub s_hook: list_head,
    pub consumer: *mut fwnode_handle,
    pub c_hook: list_head,
    pub flags: u8,
}

//
// struct fwnode_endpoint - Fwnode graph endpoint
// @port: Port number
// @id: Endpoint id
// @local_fwnode: reference to the related fwnode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwnode_endpoint {
    pub port: c_uint,
    pub id: c_uint,
    pub local_fwnode: *const fwnode_handle,
}

//
// ports and endpoints defined as software_nodes should all follow a common
// naming scheme; use these macros to ensure commonality.
//

pub const NR_FWNODE_REFERENCE_ARGS: c_int = 16;
//
// struct fwnode_reference_args - Fwnode reference with additional arguments
// @fwnode:- A reference to the base fwnode
// @nargs: Number of elements in @args array
// @args: Integer arguments on the fwnode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwnode_reference_args {
    pub fwnode: *mut fwnode_handle,
    pub nargs: c_uint,
    pub args: [u64; NR_FWNODE_REFERENCE_ARGS],
}

//
// struct fwnode_operations - Operations for fwnode interface
// @get: Get a reference to an fwnode.
// @put: Put a reference to an fwnode.
// @device_is_available: Return true if the device is available.
// @device_get_match_data: Return the device driver match data.
// @device_dma_supported: Return true if DMA is supported.
// @device_get_dma_attr: Return the device DMA attribute.
// @property_present: Return true if a property is present.
// @property_read_bool: Return a boolean property value.
// @property_read_int_array: Read an array of integer properties. Return zero on
// success, a negative error code otherwise.
// @property_read_string_array: Read an array of string properties. Return zero
// on success, a negative error code otherwise.
// @get_name: Return the name of an fwnode.
// @get_name_prefix: Get a prefix for a node (for printing purposes).
// @get_parent: Return the parent of an fwnode.
// @get_next_child_node: Return the next child node in an iteration.
// @get_named_child_node: Return a child node with a given name.
// @get_reference_args: Return a reference pointed to by a property, with args
// @graph_get_next_endpoint: Return an endpoint node in an iteration.
// @graph_get_remote_endpoint: Return the remote endpoint node of a local
// endpoint node.
// @graph_get_port_parent: Return the parent node of a port node.
// @graph_parse_endpoint: Parse endpoint for port and endpoint id.
// @iomap: Map the I/O memory of a given index for a fwnode.
// @irq_get: Get the IRQ of a given index for a fwnode.
// @add_links:	Create fwnode links to all the suppliers of the fwnode. Return
// zero on success, a negative error code otherwise.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwnode_operations {
    pub fwnode): *mut *mut *mut fwnode_handle (get)(fwnode_handle,
    pub fwnode): *mut *mut void (put)(struct fwnode_handle,
    pub fwnode): *const *const bool (device_is_available)(struct fwnode_handle,
    pub dev): *const device,
    pub fwnode): *const *const bool (device_dma_supported)(struct fwnode_handle,
    pub fwnode): *const *const (device_get_dma_attr)(struct fwnode_handle,
    pub propname): *const c_char,
    pub propname): *const c_char,
    pub nval): usize,
    pub nval): usize,
    pub fwnode): *const *const *const char (get_name)(struct fwnode_handle,
    pub fwnode): *const *const *const char (get_name_prefix)(struct fwnode_handle,
    pub fwnode): *const *const *const fwnode_handle (get_parent)(fwnode_handle,
    pub child): *mut fwnode_handle,
    pub name): *const c_char,
    pub args): *mut fwnode_reference_args,
    pub prev): *mut fwnode_handle,
    pub fwnode): *const *const (graph_get_remote_endpoint)(struct fwnode_handle,
    pub fwnode): *mut *mut (graph_get_port_parent)(struct fwnode_handle,
    pub endpoint): *mut fwnode_endpoint,
    pub index): *mut *mut *mut *mut void __iomem (iomap)(struct fwnode_handle fwnode, int,
    pub index): *const *const *const int (irq_get)(struct fwnode_handle fwnode, unsigned int,
    pub fwnode): *mut *mut int (add_links)(struct fwnode_handle,
}

extern "C" {
    pub fn test_bit(_arg: bit, _arg: &fwnode->flags) -> return;
}
extern "C" {
    pub fn fwnode_links_purge(fwnode: *mut fwnode_handle);
}
extern "C" {
    pub fn fw_devlink_purge_absent_suppliers(fwnode: *mut fwnode_handle);
}
extern "C" {
    pub fn fw_devlink_refresh_fwnode(fwnode: *mut fwnode_handle);
}
extern "C" {
    pub fn fw_devlink_is_strict() -> bool;
}
