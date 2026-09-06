//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/rdmavt_mr.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2016 Intel Corporation.
//
// For Memory Regions. This stuff should probably be moved into rdmavt/mr.h once
// drivers no longer need access to the MR directly.
//

//
// A segment is a linear region of low physical memory.
// Used by the verbs layer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_seg {
    pub vaddr: *mut c_void,
    pub length: usize,
}

// The number of rvt_segs that fit in a page.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_segarray {
    pub segs: [rvt_seg; RVT_SEGSZ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_mregion {
    pub /: *mut *mut *mut ib_pd pd; / shares refcnt of ibmr.pd,
    pub /: *mut *mut u64 user_base; / User's address for this region,
    pub /: *mut *mut u64 iova; / IB start address of this region,
    pub length: usize,
    pub lkey: u32,
    pub /: *mut *mut u32 offset; / offset (bytes) to start of region,
    pub access_flags: c_int,
    pub /: *mut *mut u32 max_segs; / number of rvt_segs in all the arrays,
    pub /: *mut *mut u32 mapsz; / size of the map array,
    pub /: *mut *mut atomic_t lkey_invalid; / true if current lkey is invalid,
    pub /: *mut *mut u8 page_shift; / 0 - non unform/non powerof2 sizes,
    pub /: *mut *mut u8 lkey_published; / in global table,
    pub refcount: percpu_ref,
    pub /: *mut *mut completion comp; / complete when refcount goes to zero,
    pub /: *mut *mut *mut rvt_segarray map[]; / the segments,
}

pub const RVT_MAX_LKEY_TABLE_BITS: c_int = 23;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_lkey_table {
// read mostly fields
    pub /: *mut *mut u32 max; / size of the table,
    pub /: *mut *mut u32 shift; / lkey/rkey shift,
    pub table: *mut rvt_mregion __rcu,
// writeable fields
// protect changes in this struct
    pub ____cacheline_aligned_in_smp: spinlock_t lock,
    pub /: *mut *mut u32 next; / next unused index (speeds search),
    pub /: *mut *mut u32 gen; / generation count,
}

//
// These keep track of the copy progress within a memory region.
// Used by the verbs layer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_sge {
    pub mr: *mut rvt_mregion,
    pub /: *mut *mut *mut void vaddr; / kernel virtual address of segment,
    pub /: *mut *mut u32 sge_length; / length of the SGE,
    pub /: *mut *mut u32 length; / remaining length of the segment,
    pub /: *mut *mut u16 m; / current index: mr->map[m],
    pub /: *mut *mut u16 n; / current index: mr->map[m]->segs[n],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_sge_state {
    pub /: *mut *mut *mut rvt_sge sg_list; / next SGE to be used if any,
    pub /: *mut *mut rvt_sge sge; / progress state for the current SGE,
    pub total_len: u32,
    pub num_sge: u8,
}

// sge = *ss->sg_list++;
extern "C" {
    pub fn rvt_ss_has_lkey(ss: *mut rvt_sge_state, lkey: u32) -> bool;
}
extern "C" {
    pub fn rvt_mr_has_lkey(mr: *mut rvt_mregion, lkey: u32) -> bool;
}
