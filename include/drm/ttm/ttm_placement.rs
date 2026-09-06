//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/ttm/ttm_placement.h
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
// Copyright (c) 2006-2009 VMware, Inc., Palo Alto, CA., USA
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Thomas Hellstrom <thellstrom-at-vmware-dot-com>
//

//
// Memory regions for data placement.
//
// Buffers placed in TTM_PL_SYSTEM are considered under TTMs control and can
// be swapped out whenever TTMs thinks it is a good idea.
// In cases where drivers would like to use TTM_PL_SYSTEM as a valid
// placement they need to be able to handle the issues that arise due to the
// above manually.
//
// For BO's which reside in system memory but for which the accelerator
// requires direct access (i.e. their usage needs to be synchronized
// between the CPU and accelerator via fences) a new, driver private
// placement that can handle such scenarios is a good idea.
//
pub const TTM_PL_SYSTEM: c_int = 0;
pub const TTM_PL_TT: c_int = 1;
pub const TTM_PL_VRAM: c_int = 2;
pub const TTM_PL_PRIV: c_int = 3;
//
// TTM_PL_FLAG_TOPDOWN requests to be placed from the
// top of the memory area, instead of the bottom.
//

// For multihop handling

// Placement is never used during eviction

// Placement is only used during eviction

//
// struct ttm_place
//
// @fpfn:	first valid page frame number to put the object
// @lpfn:	last valid page frame number to put the object
// @mem_type:	One of TTM_PL_* where the resource should be allocated from.
// @flags:	memory domain and caching flags for the object
//
// Structure indicating a possible place to put an object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_place {
    pub fpfn: u64,
    pub lpfn: u64,
    pub mem_type: u32,
    pub flags: u32,
}

//
// struct ttm_placement
//
// @num_placement:	number of preferred placements
// @placement:		preferred placements
//
// Structure indicating the placement you request for an object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_placement {
    pub num_placement: unsigned,
    pub placement: *const ttm_place,
}
