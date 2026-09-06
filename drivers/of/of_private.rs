//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/of/of_private.h
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
//
// Private symbols used by OF support code
//
// Paul Mackerras	August 1996.
// Copyright (C) 1996-2005 Paul Mackerras.
//
pub const FDT_ALIGN_SIZE: c_int = 8;
pub const MAX_RESERVED_REGIONS: c_int = 64;
//
// struct alias_prop - Alias property in 'aliases' node
// @link:	List node to link the structure in aliases_lookup list
// @alias:	Alias property name
// @np:		Pointer to device_node that the alias stands for
// @id:		Index value from end of alias name
// @stem:	Alias string without the index
//
// The structure represents one alias property of 'aliases' node as
// an entry in aliases_lookup list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alias_prop {
    pub link: list_head,
    pub alias: *const c_char,
    pub np: *mut device_node,
    pub id: c_int,
    pub stem: [c_char; ],
}

pub const OF_ROOT_NODE_ADDR_CELLS_DEFAULT: c_int = 2;

pub const OF_ROOT_NODE_ADDR_CELLS_DEFAULT: c_int = 1;

pub const OF_ROOT_NODE_SIZE_CELLS_DEFAULT: c_int = 1;
extern "C" {
    pub fn of_root_kunit_skip(test: *mut kunit);
}

extern "C" {
    pub fn of_node_release(kobj: *mut kobject);
}
extern "C" {
    pub fn __of_changeset_apply_notify(ocs: *mut of_changeset) -> c_int;
}
extern "C" {
    pub fn __of_changeset_revert_notify(ocs: *mut of_changeset) -> c_int;
}

extern "C" {
    pub fn of_platform_register_reconfig_notifier();
}

extern "C" {
    pub fn of_node_is_attached(node: *const device_node) -> c_int;
}
extern "C" {
    pub fn __of_add_property_sysfs(np: *mut device_node, pp: *mut property) -> c_int;
}
extern "C" {
    pub fn __of_remove_property_sysfs(np: *mut device_node, prop: *const property);
}
extern "C" {
    pub fn __of_attach_node_sysfs(np: *mut device_node) -> c_int;
}
extern "C" {
    pub fn __of_detach_node_sysfs(np: *mut device_node);
}

extern "C" {
    pub fn of_resolve_phandles(tree: *mut device_node) -> c_int;
}

extern "C" {
    pub fn __of_phandle_cache_inv_entry(handle: phandle);
}

extern "C" {
    pub fn of_overlay_mutex_lock();
}
extern "C" {
    pub fn of_overlay_mutex_unlock();
}

extern "C" {
    pub fn unittest_unflatten_overlay_base() -> void __init;
}

extern "C" {
    pub fn of_alias_scan(size: *mut *mut *mut void  (dt_alloc)(u64, align): u64);
}
//
// General utilities for working with live trees.
//
// All functions with two leading underscores operate
// without taking node references, so you either have to
// own the devtree lock or work on detached trees only.
//
extern "C" {
    pub fn __of_prop_free(prop: *mut property);
}
extern "C" {
    pub fn __of_add_property(np: *mut device_node, prop: *mut property) -> c_int;
}
extern "C" {
    pub fn __of_remove_property(np: *mut device_node, prop: *mut property) -> c_int;
}
extern "C" {
    pub fn __of_detach_node(np: *mut device_node);
}
// illegal phandle value (set when unresolved)
pub const OF_PHANDLE_ILLEGAL: c_uint = 0xdeadbeef;
// iterators for transactions, used for overlays
// forward iterator

// reverse iterator

extern "C" {
    pub fn of_bus_n_addr_cells(np: *mut device_node) -> c_int;
}
extern "C" {
    pub fn of_bus_n_size_cells(np: *mut device_node) -> c_int;
}

extern "C" {
    pub fn of_get_parent(_arg: np) -> return;
}

extern "C" {
    pub fn fdt_scan_reserved_mem() -> c_int;
}
extern "C" {
    pub fn fdt_scan_reserved_mem_late() -> void __init;
}
extern "C" {
    pub fn of_fdt_device_is_available(blob: *const c_void, node: c_ulong) -> bool;
}
// Max address size we deal with
pub const OF_MAX_ADDR_CELLS: c_int = 4;

// Debug utility

extern "C" {
    pub fn __of_address_resource_bounds(r: *mut resource, start: u64, size: u64) -> c_int;
}

