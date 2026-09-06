//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/property.h
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
// property.h - Unified device property interface.
//
// Copyright (C) 2014, Intel Corporation
// Authors: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
// Mika Westerberg <mika.westerberg@linux.intel.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_prop_type {
    DEV_PROP_U8,
    DEV_PROP_U16,
    DEV_PROP_U32,
    DEV_PROP_U64,
    DEV_PROP_STRING,
    DEV_PROP_REF,
}

extern "C" {
    pub fn device_property_present(dev: *const device, propname: *const c_char) -> bool;
}
extern "C" {
    pub fn device_property_read_bool(dev: *const device, propname: *const c_char) -> bool;
}
extern "C" {
    pub fn fwnode_device_is_available(fwnode: *const fwnode_handle) -> bool;
}
//
// device_is_big_endian - check if a device has BE registers
// @dev: Pointer to the struct device
//
// Returns: true if the device has a "big-endian" property, or if the kernel
// was compiled for BE *and* the device has a "native-endian" property.
// Returns false otherwise.
//
// Callers would nominally use ioread32be/iowrite32be if
// device_is_big_endian() == true, or readl/writel otherwise.
//
extern "C" {
    pub fn fwnode_device_is_big_endian(_arg: dev_fwnode(dev)) -> return;
}
//
// device_is_compatible - match 'compatible' property of the device with a given string
// @dev: Pointer to the struct device
// @compat: The string to match 'compatible' property with
//
// Returns: true if matches, otherwise false.
//
extern "C" {
    pub fn fwnode_device_is_compatible(_arg: dev_fwnode(dev), _arg: compat) -> return;
}
extern "C" {
    pub fn fwnode_property_match_property_string(_arg: dev_fwnode(dev), _arg: propname, _arg: array, _arg: n) -> return;
}
extern "C" {
    pub fn fwnode_name_eq(fwnode: *const fwnode_handle, name: *const c_char) -> bool;
}

extern "C" {
    pub fn fwnode_count_parents(fwn: *const fwnode_handle) -> c_uint;
}

//
// fwnode_handle_put - Drop reference to a device node
// @fwnode: Pointer to the device node to drop the reference to.
//
// This has to be used when terminating device_for_each_child_node() iteration
// with break or return to prevent stale device node references from being left
// behind.
//
extern "C" {
    pub fn fwnode_irq_get(fwnode: *const fwnode_handle, index: c_uint) -> c_int;
}
extern "C" {
    pub fn fwnode_irq_get_byname(fwnode: *const fwnode_handle, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn fwnode_get_child_node_count(fwnode: *const fwnode_handle) -> c_uint;
}
extern "C" {
    pub fn fwnode_get_child_node_count(_arg: dev_fwnode(dev)) -> return;
}
extern "C" {
    pub fn fwnode_get_named_child_node_count(_arg: dev_fwnode(dev), _arg: name) -> return;
}
extern "C" {
    pub fn device_property_read_u8_array(_arg: dev, _arg: propname, _arg: val, _arg: 1) -> return;
}
extern "C" {
    pub fn device_property_read_u16_array(_arg: dev, _arg: propname, _arg: val, _arg: 1) -> return;
}
extern "C" {
    pub fn device_property_read_u32_array(_arg: dev, _arg: propname, _arg: val, _arg: 1) -> return;
}
extern "C" {
    pub fn device_property_read_u64_array(_arg: dev, _arg: propname, _arg: val, _arg: 1) -> return;
}
extern "C" {
    pub fn device_property_read_u8_array(_arg: dev, _arg: propname, _arg: NULL, _arg: 0) -> return;
}
extern "C" {
    pub fn device_property_read_u16_array(_arg: dev, _arg: propname, _arg: NULL, _arg: 0) -> return;
}
extern "C" {
    pub fn device_property_read_u32_array(_arg: dev, _arg: propname, _arg: NULL, _arg: 0) -> return;
}
extern "C" {
    pub fn device_property_read_u64_array(_arg: dev, _arg: propname, _arg: NULL, _arg: 0) -> return;
}
extern "C" {
    pub fn device_property_read_string_array(_arg: dev, _arg: propname, _arg: NULL, _arg: 0) -> return;
}
extern "C" {
    pub fn fwnode_property_read_u8_array(_arg: fwnode, _arg: propname, _arg: val, _arg: 1) -> return;
}
extern "C" {
    pub fn fwnode_property_read_u16_array(_arg: fwnode, _arg: propname, _arg: val, _arg: 1) -> return;
}
extern "C" {
    pub fn fwnode_property_read_u32_array(_arg: fwnode, _arg: propname, _arg: val, _arg: 1) -> return;
}
extern "C" {
    pub fn fwnode_property_read_u64_array(_arg: fwnode, _arg: propname, _arg: val, _arg: 1) -> return;
}
extern "C" {
    pub fn fwnode_property_read_u8_array(_arg: fwnode, _arg: propname, _arg: NULL, _arg: 0) -> return;
}
extern "C" {
    pub fn fwnode_property_read_u16_array(_arg: fwnode, _arg: propname, _arg: NULL, _arg: 0) -> return;
}
extern "C" {
    pub fn fwnode_property_read_u32_array(_arg: fwnode, _arg: propname, _arg: NULL, _arg: 0) -> return;
}
extern "C" {
    pub fn fwnode_property_read_u64_array(_arg: fwnode, _arg: propname, _arg: NULL, _arg: 0) -> return;
}
extern "C" {
    pub fn fwnode_property_read_string_array(_arg: fwnode, _arg: propname, _arg: NULL, _arg: 0) -> return;
}
//
// struct software_node_ref_args - Reference property with additional arguments
// @swnode: Reference to a software node
// @fwnode: Alternative reference to a firmware node handle
// @nargs: Number of elements in @args array
// @args: Integer arguments
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct software_node_ref_args {
    pub swnode: *const software_node,
    pub fwnode: *mut fwnode_handle,
    pub nargs: c_uint,
    pub args: [u64; NR_FWNODE_REFERENCE_ARGS],
}

//
// struct property_entry - "Built-in" device property representation.
// @name: Name of the property.
// @length: Length of data making up the value.
// @is_inline: True when the property value is stored inline.
// @type: Type of the data in unions.
// @pointer: Pointer to the property when it is not stored inline.
// @value: Value of the property when it is stored inline.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct property_entry {
    pub name: *const c_char,
    pub length: usize,
    pub is_inline: bool,
    pub type: dev_prop_type,
    pub pointer: *const c_void,
// private: internal representation of @value
    pub sizeof(u8)]: u8 u8_data[sizeof(u64) /,
    pub sizeof(u16)]: u16 u16_data[sizeof(u64) /,
    pub sizeof(u32)]: u32 u32_data[sizeof(u64) /,
    pub sizeof(u64)]: u64 u64_data[sizeof(u64) /,
    pub )]: *const *const char str[sizeof(u64) / sizeof(char,
// public:
    pub value: },
}

//
// Note: the below initializers for the anonymous union are carefully
// crafted to avoid gcc-4.4.4's problems with initialization of anon unions
// and structs.
//

extern "C" {
    pub fn property_entries_free(properties: *const property_entry);
}
extern "C" {
    pub fn device_dma_supported(dev: *const device) -> bool;
}
extern "C" {
    pub fn device_get_dma_attr(dev: *const device) -> dev_dma_attr;
}
extern "C" {
    pub fn device_get_phy_mode(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn fwnode_get_phy_mode(fwnode: *const fwnode_handle) -> c_int;
}
extern "C" {
    pub fn fwnode_property_present(_arg: fwnode, _arg: "remote-endpoint") -> return;
}
//
// Fwnode lookup flags
//
// @FWNODE_GRAPH_ENDPOINT_NEXT: In the case of no exact match, look for the
// closest endpoint ID greater than the specified
// one.
// @FWNODE_GRAPH_DEVICE_DISABLED: That the device to which the remote
// endpoint of the given endpoint belongs to,
// may be disabled, or that the endpoint is not
// connected.
//

extern "C" {
    pub fn fwnode_connection_find_match(_arg: dev_fwnode(dev), _arg: con_id, _arg: data, _arg: match) -> return;
}
// --------------------------------------------------------------------------
// Software fwnode support - when HW description is incomplete or missing
//
// struct software_node - Software node description
// @name: Name of the software node
// @parent: Parent of the software node
// @properties: Array of device properties
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct software_node {
    pub name: *const c_char,
    pub parent: *const software_node,
    pub properties: *const property_entry,
}

extern "C" {
    pub fn is_software_node(fwnode: *const fwnode_handle) -> bool;
}
extern "C" {
    pub fn software_node_register_node_group(node_group: *const *const software_node) -> c_int;
}
extern "C" {
    pub fn software_node_unregister_node_group(node_group: *const *const software_node);
}
extern "C" {
    pub fn software_node_register(node: *const software_node) -> c_int;
}
extern "C" {
    pub fn software_node_unregister(node: *const software_node);
}
extern "C" {
    pub fn fwnode_remove_software_node(fwnode: *mut fwnode_handle);
}
extern "C" {
    pub fn device_add_software_node(dev: *mut device, node: *const software_node) -> c_int;
}
extern "C" {
    pub fn device_remove_software_node(dev: *mut device);
}
