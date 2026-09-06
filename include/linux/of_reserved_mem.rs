//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/of_reserved_mem.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reserved_mem {
    pub name: *const c_char,
    pub ops: *const reserved_mem_ops,
    pub base: phys_addr_t,
    pub size: phys_addr_t,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reserved_mem_ops {
    pub align): *mut *mut int (node_validate)(unsigned long fdt_node, phys_addr_t,
    pub size): phys_addr_t,
    pub rmem): *mut *mut int (node_init)(unsigned long fdt_node, struct reserved_mem,
    pub dev): *mut device,
    pub dev): *mut device,
}

extern "C" {
    pub fn of_reserved_mem_device_release(dev: *mut device);
}
extern "C" {
    pub fn devm_of_reserved_mem_device_init(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn of_reserved_mem_region_count(np: *const device_node) -> c_int;
}

//
// of_reserved_mem_device_init() - assign reserved memory region to given device
// @dev:	Pointer to the device to configure
//
// This function assigns respective DMA-mapping operations based on the first
// reserved memory region specified by 'memory-region' property in device tree
// node of the given device.
//
// Returns error code or zero on success.
//
extern "C" {
    pub fn of_reserved_mem_device_init_by_idx(_arg: dev, _arg: dev->of_node, _arg: 0) -> return;
}
