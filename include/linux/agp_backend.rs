//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/agp_backend.h
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
// AGPGART backend specific includes. Not for userspace consumption.
//
// Copyright (C) 2004 Silicon Graphics, Inc.
// Copyright (C) 2002-2003 Dave Jones
// Copyright (C) 1999 Jeff Hartmann
// Copyright (C) 1999 Precision Insight, Inc.
// Copyright (C) 1999 Xi Graphics, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// JEFF HARTMANN, OR ANY OTHER CONTRIBUTORS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
// OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
pub const _AGP_BACKEND_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chipset_type {
    NOT_SUPPORTED,
    SUPPORTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_version {
    pub major: u16,
    pub minor: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_kern_info {
    pub version: agp_version,
    pub device: *mut pci_dev,
    pub chipset: chipset_type,
    pub mode: c_ulong,
    pub aper_base: c_ulong,
    pub aper_size: usize,
    pub /: *mut *mut int max_memory; / In pages,
    pub current_memory: c_int,
    pub cant_use_aperture: bool,
    pub page_mask: c_ulong,
    pub vm_ops: *const vm_operations_struct,
}

//
// The agp_memory structure has information about the block of agp memory
// allocated.  A caller may manipulate the next and prev pointers to link
// each allocated item into a list.  These pointers are ignored by the backend.
// Everything else should never be written to, but the caller may read any of
// the items to determine the status of this block of agp memory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_memory {
    pub next: *mut agp_memory,
    pub prev: *mut agp_memory,
    pub bridge: *mut agp_bridge_data,
    pub pages: *mut page,
    pub page_count: usize,
    pub key: c_int,
    pub num_scratch_pages: c_int,
    pub pg_start: off_t,
    pub type: u32,
    pub physical: u32,
    pub is_bound: bool,
    pub is_flushed: bool,
// list of agp_memory mapped to the aperture
    pub mapped_list: list_head,
// DMA-mapped addresses
    pub sg_list: *mut scatterlist,
    pub num_sg: c_int,
}

pub const AGP_NORMAL_MEMORY: c_int = 0;

extern "C" {
    pub fn agp_free_memory(: *mut agp_memory);
}
extern "C" {
    pub fn agp_copy_info(: *mut agp_bridge_data, : *mut agp_kern_info) -> c_int;
}
extern "C" {
    pub fn agp_bind_memory(: *mut agp_memory, _arg: off_t) -> c_int;
}
extern "C" {
    pub fn agp_unbind_memory(: *mut agp_memory) -> c_int;
}
extern "C" {
    pub fn agp_enable(: *mut agp_bridge_data, _arg: u32);
}
extern "C" {
    pub fn agp_backend_release(: *mut agp_bridge_data);
}
