//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nvdimm/pmem.h
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

// this definition is in it's own header for tools/testing/nvdimm to consume
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmem_device {
// One contiguous memory region per device
    pub phys_addr: phys_addr_t,
// when non-zero this device is hosting a 'pfn' instance
    pub data_offset: phys_addr_t,
    pub virt_addr: *mut c_void,
// immutable base size of the namespace
    pub size: usize,
// trim size when namespace capacity has been section aligned
    pub pfn_pad: u32,
    pub bb_state: *mut kernfs_node,
    pub bb: badblocks,
    pub dax_dev: *mut dax_device,
    pub disk: *mut gendisk,
    pub pgmap: dev_pagemap,
}

extern "C" {
    pub fn TestClearPageHWPoison(_arg: page) -> return;
}

