//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/logic_iomem.h
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
// Copyright (C) 2021 Intel Corporation
// Author: johannes@sipsolutions.net
//

//
// struct logic_iomem_ops - emulated IO memory ops
// @read: read an 8, 16, 32 or 64 bit quantity from the given offset,
// size is given in bytes (1, 2, 4 or 8)
// (64-bit only necessary if CONFIG_64BIT is set)
// @write: write an 8, 16 32 or 64 bit quantity to the given offset,
// size is given in bytes (1, 2, 4 or 8)
// (64-bit only necessary if CONFIG_64BIT is set)
// @set: optional, for memset_io()
// @copy_from: optional, for memcpy_fromio()
// @copy_to: optional, for memcpy_toio()
// @unmap: optional, this region is getting unmapped
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct logic_iomem_ops {
    pub size): *mut *mut *mut unsigned long (read)(void priv, unsigned int offset, int,
    pub val): c_ulong,
    pub size): *mut *mut *mut void (set)(void priv, unsigned int offset, u8 value, int,
    pub size): c_int,
    pub size): c_int,
    pub priv): *mut *mut void (unmap)(void,
}

//
// struct logic_iomem_region_ops - ops for an IO memory handler
// @map: map a range in the registered IO memory region, must
// fill *ops with the ops and may fill *priv to be passed
// to the ops. The offset is given as the offset into the
// registered resource region.
// The return value is negative for errors, or >= 0 for
// success. On success, the return value is added to the
// offset for later ops, to allow for partial mappings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct logic_iomem_region_ops {
    pub priv): *mut c_void,
}

//
// logic_iomem_add_region - register an IO memory region
// @resource: the resource description for this region
// @ops: the IO memory mapping ops for this resource
//
