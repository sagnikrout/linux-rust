//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nvmem-consumer.h
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
// nvmem framework consumer.
//
// Copyright (C) 2015 Srinivas Kandagatla <srinivas.kandagatla@linaro.org>
// Copyright (C) 2013 Maxime Ripard <maxime.ripard@free-electrons.com>
//

// consumer cookie
//
// struct nvmem_cell_lookup - cell lookup entry
//
// @nvmem_name:	Name of the provider.
// @cell_name:	Name of the nvmem cell as defined in the name field of
// struct nvmem_cell_info.
// @dev_id:	Name of the consumer device that will be associated with
// this cell.
// @con_id:	Connector id for this cell lookup.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmem_cell_lookup {
    pub nvmem_name: *const c_char,
    pub cell_name: *const c_char,
    pub dev_id: *const c_char,
    pub con_id: *const c_char,
    pub node: list_head,
}

// Cell based interface
extern "C" {
    pub fn nvmem_cell_put(cell: *mut nvmem_cell);
}
extern "C" {
    pub fn devm_nvmem_cell_put(dev: *mut device, cell: *mut nvmem_cell);
}
extern "C" {
    pub fn nvmem_cell_write(cell: *mut nvmem_cell, buf: *mut c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn nvmem_cell_read_u8(dev: *mut device, cell_id: *const c_char, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn nvmem_cell_read_u16(dev: *mut device, cell_id: *const c_char, val: *mut u16) -> c_int;
}
extern "C" {
    pub fn nvmem_cell_read_u32(dev: *mut device, cell_id: *const c_char, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn nvmem_cell_read_u64(dev: *mut device, cell_id: *const c_char, val: *mut u64) -> c_int;
}
// direct nvmem device read/write interface
extern "C" {
    pub fn nvmem_device_put(nvmem: *mut nvmem_device);
}
extern "C" {
    pub fn devm_nvmem_device_put(dev: *mut device, nvmem: *mut nvmem_device);
}
extern "C" {
    pub fn nvmem_dev_size(nvmem: *mut nvmem_device) -> usize;
}
extern "C" {
    pub fn nvmem_register_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn nvmem_unregister_notifier(nb: *mut notifier_block) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

