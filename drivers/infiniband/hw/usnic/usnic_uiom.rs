//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/usnic/usnic_uiom.h
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
// Copyright (c) 2013, Cisco Systems, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// BSD license below:
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_uiom_dev {
    pub dev: *mut device,
    pub link: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_uiom_pd {
    pub domain: *mut iommu_domain,
    pub lock: spinlock_t,
    pub root: rb_root_cached,
    pub devs: list_head,
    pub dev_cnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_uiom_reg {
    pub pd: *mut usnic_uiom_pd,
    pub va: c_ulong,
    pub length: usize,
    pub offset: c_int,
    pub page_size: c_int,
    pub writable: c_int,
    pub chunk_list: list_head,
    pub work: work_struct,
    pub owning_mm: *mut mm_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usnic_uiom_chunk {
    pub list: list_head,
    pub nents: c_int,
    pub __counted_by(nents): scatterlist page_list[],
}

extern "C" {
    pub fn usnic_uiom_dealloc_pd(pd: *mut usnic_uiom_pd);
}
extern "C" {
    pub fn usnic_uiom_attach_dev_to_pd(pd: *mut usnic_uiom_pd, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn usnic_uiom_free_dev_list(devs: *mut device);
}
extern "C" {
    pub fn usnic_uiom_reg_release(uiomr: *mut usnic_uiom_reg);
}
