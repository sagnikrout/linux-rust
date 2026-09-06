//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/libcxgb/libcxgb_ppm.h
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
// libcxgb_ppm.h: Chelsio common library for T3/T4/T5 iSCSI ddp operation
//
// Copyright (c) 2016 Chelsio Communications, Inc. All rights reserved.
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
// Written by: Karen Xie (kxie@chelsio.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_pagepod_hdr {
    pub vld_tid: u32,
    pub pgsz_tag_clr: u32,
    pub max_offset: u32,
    pub page_offset: u32,
    pub rsvd: u64,
}

pub const PPOD_PAGES_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_pagepod {
    pub hdr: cxgbi_pagepod_hdr,
    pub 1]: __be64 addr[PPOD_PAGES_MAX +,
}

// ddp tag format
// for a 32-bit tag:
// bit #
// 31 .....   .....  0
// X   Y...Y Z...Z, where
// ^   ^^^^^ ^^^^
// |   |      |____ when ddp bit = 0: color bits
// |   |
// |   |____ when ddp bit = 0: idx into the ddp memory region
// |
// |____ ddp bit: 0 - ddp tag, 1 - non-ddp tag
//
// [page selector:2] [sw/free bits] [0] [idx] [color:6]
//
pub const DDP_PGIDX_MAX: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_task_tag_info {
    pub flags: c_uchar,
pub const CXGBI_PPOD_INFO_FLAG_VALID: c_uint = 0x1;
pub const CXGBI_PPOD_INFO_FLAG_MAPPED: c_uint = 0x2;
    pub cid: c_uchar,
    pub pg_shift: c_ushort,
    pub npods: c_uint,
    pub idx: c_uint,
    pub tag: c_uint,
    pub hdr: cxgbi_pagepod_hdr,
    pub nents: c_int,
    pub nr_pages: c_int,
    pub sgl: *mut scatterlist,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_tag_format {
    pub pgsz_order: [c_uchar; DDP_PGIDX_MAX],
    pub pgsz_idx_dflt: c_uchar,
    pub free_bits:4: c_uchar,
    pub color_bits:4: c_uchar,
    pub idx_bits: c_uchar,
    pub rsvd_bits: c_uchar,
    pub no_ddp_mask: c_uint,
    pub idx_mask: c_uint,
    pub color_mask: c_uint,
    pub idx_clr_mask: c_uint,
    pub rsvd_mask: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_ppod_data {
    pub pg_idx:2: c_uchar,
    pub color:6: c_uchar,
    pub chan_id: c_uchar,
    pub npods: c_ushort,
    pub caller_data: c_ulong,
}

// per cpu ppm pool
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_ppm_pool {
    pub /: *mut *mut unsigned int base; / base index,
    pub /: *mut *mut unsigned int next; / next possible free index,
    pub /: *mut *mut spinlock_t lock; / ppm pool lock,
    pub bmap: [c_ulong; ],
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbi_ppm {
    pub refcnt: kref,
    pub /: *mut *mut *mut net_device ndev; / net_device, 1st port,
    pub pdev: *mut pci_dev,
    pub lldev: *mut c_void,
    pub ppm_pp: *mut c_void,
    pub tformat: cxgbi_tag_format,
    pub ppmax: c_uint,
    pub llimit: c_uint,
    pub base_idx: c_uint,
    pub pool_rsvd: c_uint,
    pub pool_index_max: c_uint,
    pub pool: *mut cxgbi_ppm_pool __percpu,
// map lock
    pub /: *mut *mut spinlock_t map_lock; / ppm map lock,
    pub bmap_index_max: c_uint,
    pub next: c_uint,
    pub max_index_in_edram: c_uint,
    pub ppod_bmap: *mut c_ulong,
    pub ppod_data: [cxgbi_ppod_data; ],
}

pub const DDP_THRESHOLD: c_int = 512;

pub const PPOD_SIZE_SHIFT: c_int = 6;
// page pods are allocated in groups of this size (must be power of 2)

pub const PPOD_COLOR_SHIFT: c_int = 0;

pub const PPOD_IDX_SHIFT: c_int = 6;
pub const PPOD_IDX_MAX_SIZE: c_int = 24;
pub const PPOD_TID_SHIFT: c_int = 0;

pub const PPOD_TAG_SHIFT: c_int = 6;

pub const PPOD_VALID_SHIFT: c_int = 24;

pub const PPOD_PI_EXTRACT_CTL_SHIFT: c_int = 31;

pub const PPOD_PI_TYPE_SHIFT: c_int = 29;
pub const PPOD_PI_TYPE_MASK: c_uint = 0x3;

pub const PPOD_PI_CHECK_CTL_SHIFT: c_int = 27;
pub const PPOD_PI_CHECK_CTL_MASK: c_uint = 0x3;

pub const PPOD_PI_REPORT_CTL_SHIFT: c_int = 25;
pub const PPOD_PI_REPORT_CTL_MASK: c_uint = 0x3;

// the sw tag must be using <= 31 bits
// final_tag = tformat->no_ddp_mask;
// final_tag = upper | tformat->no_ddp_mask | lower;
// sw bits are the free bits
// final_tag = (val << tformat->rsvd_bits) |
// reserve top most 2 bits for page selector
extern "C" {
    pub fn cxgbi_ppm_find_page_index(ppm: *mut cxgbi_ppm, pgsz: c_ulong) -> c_int;
}
extern "C" {
    pub fn cxgbi_ppm_ppod_release(: *mut cxgbi_ppm, idx: u32);
}
extern "C" {
    pub fn cxgbi_ppm_release(ppm: *mut cxgbi_ppm) -> c_int;
}
extern "C" {
    pub fn cxgbi_tagmask_check(tagmask: c_uint, : *mut cxgbi_tag_format);
}
extern "C" {
    pub fn cxgbi_tagmask_set(ppmax: c_uint) -> c_uint;
}
