//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hns/hns_roce_hem.h
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


//
// Copyright (c) 2016 Hisilicon Limited.
// Copyright (c) 2007, 2008 Mellanox Technologies. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
pub const HEM_HOP_STEP_DIRECT: c_uint = 0xff;
// MAP HEM(Hardware Entry Memory)
// UNMAP HEM

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_hem {
    pub buf: *mut c_void,
    pub dma: dma_addr_t,
    pub size: c_ulong,
    pub refcount: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_hem_mhop {
    pub hop_num: u32,
    pub buf_chunk_size: u32,
    pub bt_chunk_size: u32,
    pub ba_l0_num: u32,
    pub /: *mut *mut u32 l0_idx; / level 0 base address table index,
    pub /: *mut *mut u32 l1_idx; / level 1 base address table index,
    pub /: *mut *mut u32 l2_idx; / level 2 base address table index,
}

extern "C" {
    pub fn hns_roce_free_hem(hr_dev: *mut hns_roce_dev, hem: *mut hns_roce_hem);
}
extern "C" {
    pub fn hns_roce_cleanup_hem(hr_dev: *mut hns_roce_dev);
}
extern "C" {
    pub fn hns_roce_check_whether_mhop(hr_dev: *mut hns_roce_dev, type: u32) -> bool;
}
extern "C" {
    pub fn hns_roce_hem_list_init(hem_list: *mut hns_roce_hem_list);
}
