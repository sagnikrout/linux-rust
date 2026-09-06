//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/ps3stor.h
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
// PS3 Storage Devices
//
// Copyright (C) 2007 Sony Computer Entertainment Inc.
// Copyright 2007 Sony Corp.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_storage_region {
    pub id: c_uint,
    pub start: u64,
    pub size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_storage_device {
    pub sbd: ps3_system_bus_device,
    pub dma_region: ps3_dma_region,
    pub irq: c_uint,
    pub blk_size: u64,
    pub tag: u64,
    pub lv1_status: u64,
    pub done: completion,
    pub bounce_size: c_ulong,
    pub bounce_buf: *mut c_void,
    pub bounce_lpar: u64,
    pub bounce_dma: dma_addr_t,
    pub num_regions: c_uint,
    pub accessible_regions: c_ulong,
    pub /: *mut *mut unsigned int region_idx; / first accessible region,
    pub /: *mut *mut ps3_storage_region regions[]; / Must be last,
}

extern "C" {
    pub fn container_of(_arg: dev, ps3_storage_device: struct, _arg: sbd.core) -> return;
}
extern "C" {
    pub fn ps3stor_teardown(dev: *mut ps3_storage_device);
}
