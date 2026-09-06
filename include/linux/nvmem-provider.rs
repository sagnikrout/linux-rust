//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nvmem-provider.h
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


// SPDX-License-Identifier: GPL-2.0
//
// nvmem framework provider.
//
// Copyright (C) 2015 Srinivas Kandagatla <srinivas.kandagatla@linaro.org>
// Copyright (C) 2013 Maxime Ripard <maxime.ripard@free-electrons.com>
//

// used for vendor specific post processing of cell data
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvmem_type {
    NVMEM_TYPE_UNKNOWN = 0,
    NVMEM_TYPE_EEPROM,
    NVMEM_TYPE_OTP,
    NVMEM_TYPE_BATTERY_BACKED,
    NVMEM_TYPE_FRAM,
}

//
// struct nvmem_keepout - NVMEM register keepout range.
//
// @start:	The first byte offset to avoid.
// @end:	One beyond the last byte offset to avoid.
// @value:	The byte to fill reads with for this region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmem_keepout {
    pub start: c_uint,
    pub end: c_uint,
    pub value: c_uchar,
}

//
// struct nvmem_cell_info - NVMEM cell description
// @name:	Name.
// @offset:	Offset within the NVMEM device.
// @raw_len:	Length of raw data (without post processing).
// @bytes:	Length of the cell.
// @bit_offset:	Bit offset if cell is smaller than a byte.
// @nbits:	Number of bits.
// @np:		Optional device_node pointer.
// @read_post_process:	Callback for optional post processing of cell data
// on reads.
// @priv:	Opaque data passed to the read_post_process hook.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmem_cell_info {
    pub name: *const c_char,
    pub offset: c_uint,
    pub raw_len: usize,
    pub bytes: c_uint,
    pub bit_offset: c_uint,
    pub nbits: c_uint,
    pub np: *mut device_node,
    pub read_post_process: nvmem_cell_post_process_t,
    pub priv: *mut c_void,
}

//
// struct nvmem_config - NVMEM device configuration
//
// @dev:	Parent device.
// @name:	Optional name.
// @id:		Optional device ID used in full name. Ignored if name is NULL.
// @owner:	Pointer to exporter module. Used for refcounting.
// @cells:	Optional array of pre-defined NVMEM cells.
// @ncells:	Number of elements in cells.
// @add_legacy_fixed_of_cells:	Read fixed NVMEM cells from old OF syntax.
// @fixup_dt_cell_info: Will be called before a cell is added. Can be
// used to modify the nvmem_cell_info.
// @keepout:	Optional array of keepout ranges (sorted ascending by start).
// @nkeepout:	Number of elements in the keepout array.
// @type:	Type of the nvmem storage
// @read_only:	Device is read-only.
// @root_only:	Device is accessibly to root only.
// @of_node:	If given, this will be used instead of the parent's of_node.
// @reg_read:	Callback to read data; return zero if successful.
// @reg_write:	Callback to write data; return zero if successful.
// @size:	Device size.
// @word_size:	Minimum read/write access granularity.
// @stride:	Minimum read/write access stride.
// @priv:	User context passed to read/write callbacks.
// @ignore_wp:  Write Protect pin is managed by the provider.
// @layout:	Fixed layout associated with this nvmem device.
//
// Note: A default "nvmem<id>" name will be assigned to the device if
// no name is specified in its configuration. In such case "<id>" is
// generated with ida_alloc() and provided id field is ignored.
//
// Note: Specifying name and setting id to -1 implies a unique device
// whose name is provided as-is (kept unaltered).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmem_config {
    pub dev: *mut device,
    pub name: *const c_char,
    pub id: c_int,
    pub owner: *mut module,
    pub cells: *const nvmem_cell_info,
    pub ncells: c_int,
    pub add_legacy_fixed_of_cells: bool,
    pub cell): *mut nvmem_cell_info,
    pub keepout: *const nvmem_keepout,
    pub nkeepout: c_uint,
    pub type: nvmem_type,
    pub read_only: bool,
    pub root_only: bool,
    pub ignore_wp: bool,
    pub layout: *mut nvmem_layout,
    pub of_node: *mut device_node,
    pub reg_read: nvmem_reg_read_t,
    pub reg_write: nvmem_reg_write_t,
    pub size: c_int,
    pub word_size: c_int,
    pub stride: c_int,
    pub priv: *mut c_void,
// To be only used by old driver/misc/eeprom drivers
    pub compat: bool,
    pub base_dev: *mut device,
}

//
// struct nvmem_layout - NVMEM layout definitions
//
// @dev:		Device-model layout device.
// @nvmem:		The underlying NVMEM device
// @add_cells:		Will be called if a nvmem device is found which
// has this layout. The function will add layout
// specific cells with nvmem_add_one_cell().
//
// A nvmem device can hold a well defined structure which can just be
// evaluated during runtime. For example a TLV list, or a list of "name=val"
// pairs. A nvmem layout can parse the nvmem device and add appropriate
// cells.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmem_layout {
    pub dev: device,
    pub nvmem: *mut nvmem_device,
    pub layout): *mut *mut int (add_cells)(struct nvmem_layout,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmem_layout_driver {
    pub driver: device_driver,
    pub layout): *mut *mut int (probe)(struct nvmem_layout,
    pub layout): *mut *mut void (remove)(struct nvmem_layout,
}

extern "C" {
    pub fn nvmem_unregister(nvmem: *mut nvmem_device);
}
extern "C" {
    pub fn nvmem_layout_register(layout: *mut nvmem_layout) -> c_int;
}
extern "C" {
    pub fn nvmem_layout_unregister(layout: *mut nvmem_layout);
}

extern "C" {
    pub fn nvmem_layout_driver_unregister(drv: *mut nvmem_layout_driver);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn nvmem_register(_arg: c) -> return;
}

//
// of_nvmem_layout_get_container() - Get OF node of layout container
//
// @nvmem: nvmem device
//
// Return: a node pointer with refcount incremented or NULL if no
// container exists. Use of_node_put() on it when done.
//

