//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nvmem/internals.h
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

// Hold pointers to callbacks owned by the nvmem provider module.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmem_operations {
    pub reg_read: nvmem_reg_read_t,
    pub reg_write: nvmem_reg_write_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmem_device {
    pub owner: *mut module,
    pub dev: device,
    pub stride: c_int,
    pub word_size: c_int,
    pub id: c_int,
    pub refcnt: kref,
    pub size: usize,
    pub read_only: bool,
    pub root_only: bool,
    pub flags: c_int,
    pub type: nvmem_type,
    pub eeprom: bin_attribute,
    pub base_dev: *mut device,
    pub cells: list_head,
    pub cell): *mut nvmem_cell_info,
    pub keepout: *const nvmem_keepout,
    pub nkeepout: c_uint,
    pub wp_gpio: *mut gpio_desc,
    pub layout: *mut nvmem_layout,
    pub ops: *mut nvmem_operations,
    pub priv: *mut c_void,
    pub sysfs_cells_populated: bool,
}

extern "C" {
    pub fn nvmem_add_cells_from_dt(nvmem: *mut nvmem_device, np: *mut device_node) -> c_int;
}

extern "C" {
    pub fn nvmem_layout_bus_register() -> c_int;
}
extern "C" {
    pub fn nvmem_layout_bus_unregister();
}
extern "C" {
    pub fn nvmem_populate_layout(nvmem: *mut nvmem_device) -> c_int;
}
extern "C" {
    pub fn nvmem_destroy_layout(nvmem: *mut nvmem_device);
}

