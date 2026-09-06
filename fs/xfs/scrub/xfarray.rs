//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/xfarray.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2021-2023 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
// xfile array index type, along with cursor initialization
pub type xfarray_idx_t = u64;

// Iterate each index of an xfile array.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfarray {
// Underlying file that backs the array.
    pub xfile: *mut xfile,
// Number of array elements.
    pub nr: xfarray_idx_t,
// Maximum possible array size.
    pub max_nr: xfarray_idx_t,
// Number of unset slots in the array below @nr.
    pub unset_slots: u64,
// Size of an array element.
    pub obj_size: usize,
// log2 of array element size, if possible.
    pub obj_size_log: c_int,
}

extern "C" {
    pub fn xfarray_destroy(array: *mut xfarray);
}
extern "C" {
    pub fn xfarray_load(array: *mut xfarray, idx: xfarray_idx_t, ptr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn xfarray_unset(array: *mut xfarray, idx: xfarray_idx_t) -> c_int;
}
extern "C" {
    pub fn xfarray_store(array: *mut xfarray, idx: xfarray_idx_t, ptr: *const c_void) -> c_int;
}
extern "C" {
    pub fn xfarray_store_anywhere(array: *mut xfarray, ptr: *const c_void) -> c_int;
}
extern "C" {
    pub fn xfarray_element_is_null(array: *mut xfarray, ptr: *const c_void) -> bool;
}
extern "C" {
    pub fn xfarray_truncate(array: *mut xfarray);
}
extern "C" {
    pub fn xfarray_bytes(array: *mut xfarray) -> c_ulonglong;
}
//
// Load an array element, but zero the buffer if there's no data because we
// haven't stored to that array element yet.
//
// Append an element to the array.
extern "C" {
    pub fn xfarray_store(_arg: array, _arg: array->nr, _arg: ptr) -> return;
}
extern "C" {
    pub fn xfarray_length(array: *mut xfarray) -> u64;
}
extern "C" {
    pub fn xfarray_load_next(array: *mut xfarray, idx: *mut xfarray_idx_t, rec: *mut c_void) -> c_int;
}
//
// Iterate the non-null elements in a sparse xfarray.  Callers should
// initialize *idx to XFARRAY_CURSOR_INIT before the first call; on return, it
// will be set to one more than the index of the record that was retrieved.
// Returns 1 if a record was retrieved, 0 if there weren't any more records, or
// a negative errno.
//
// Declarations for xfile array sort functionality.
pub type xfarray_cmp_fn = cmp_func_t;
// Perform an in-memory heapsort for small subsets.

// Evalulate this many points to find the qsort pivot.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfarray_sortinfo {
    pub array: *mut xfarray,
// Comparison function for the sort.
    pub cmp_fn: xfarray_cmp_fn,
// Maximum height of the partition stack.
    pub max_stack_depth: u8,
// Current height of the partition stack.
    pub stack_depth: i8,
// Maximum stack depth ever used.
    pub max_stack_used: u8,
// XFARRAY_SORT_* flags; see below.
    pub flags: c_uint,
// next time we want to cond_resched()
    pub relax: xchk_relax,
// Cache a folio here for faster scanning for pivots
    pub folio: *mut folio,
// First array index in folio that is completely readable
    pub first_folio_idx: xfarray_idx_t,
// Last array index in folio that is completely readable
    pub last_folio_idx: xfarray_idx_t,

// Performance statistics.
    pub loads: u64,
    pub stores: u64,
    pub compares: u64,
    pub heapsorts: u64,

//
// Extra bytes are allocated beyond the end of the structure to store
// quicksort information.  C does not permit multiple VLAs per struct,
// so we document all of this in a comment.
//
// Pretend that we have a typedef for array records:
//
// typedef char[array->obj_size]	xfarray_rec_t;
//
// First comes the quicksort partition stack:
//
// xfarray_idx_t	lo[max_stack_depth];
// xfarray_idx_t	hi[max_stack_depth];
//
// union {
//
// If for a given subset we decide to use an in-memory sort, we use a
// block of scratchpad records here to compare items:
//
// xfarray_rec_t	scratch[ISORT_NR];
//
// Otherwise, we want to partition the records to partition the array.
// We store the chosen pivot record at the start of the scratchpad area
// and use the rest to sample some records to estimate the median.
// The format of the qsort_pivot array enables us to use the kernel
// heapsort function to place the median value in the middle.
//
// struct {
// xfarray_rec_t	pivot;
// struct {
// xfarray_rec_t	rec;  (rounded up to 8 bytes)
// xfarray_idx_t	idx;
// } qsort_pivot[QSORT_PIVOT_NR];
// };
// }
//
}

// Sort can be interrupted by a fatal signal.

