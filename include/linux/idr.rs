//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/idr.h
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
// include/linux/idr.h
//
// 2002-10-18  written by Jim Houston jim.houston@ccur.com
// Copyright (C) 2002 by Concurrent Computer Corporation
//
// Small id to pointer translation service avoiding fixed sized
// tables.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idr {
    pub idr_rt: radix_tree_root,
    pub idr_base: c_uint,
    pub idr_next: c_uint,
}

//
// The IDR API does not expose the tagging functionality of the radix tree
// to users.  Use tag 0 to track whether a node has free space below it.
//
pub const IDR_FREE: c_int = 0;
// Set the IDR flag and the IDR_FREE tag

//
// IDR_INIT() - Initialise an IDR.
// @name: Name of IDR.
//
// A freshly-initialised IDR contains no IDs.
//

//
// DEFINE_IDR() - Define a statically-allocated IDR.
// @name: Name of IDR.
//
// An IDR defined using this macro is ready for use with no additional
// initialisation required.  It contains no IDs.
//

//
// idr_get_cursor - Return the current position of the cyclic allocator
// @idr: idr handle
//
// The value returned is the value that will be next returned from
// idr_alloc_cyclic() if it is free (otherwise the search will start from
// this position).
//
extern "C" {
    pub fn READ_ONCE(_arg: idr->idr_next) -> return;
}
//
// idr_set_cursor - Set the current position of the cyclic allocator
// @idr: idr handle
// @val: new position
//
// The next call to idr_alloc_cyclic() will return @val if it is free
// (otherwise the search will start from this position).
//
// DOC: idr sync
// idr synchronization (stolen from radix-tree.h)
//
// idr_find() is able to be called locklessly, using RCU. The caller must
// ensure calls to this function are made within rcu_read_lock() regions.
// Other readers (lock-free or otherwise) and modifications may be running
// concurrently.
//
// It is still required that the caller manage the synchronization and
// lifetimes of the items. So if RCU lock-free lookups are used, typically
// this would mean that the items have their own locks, or are amenable to
// lock-free access; and that the items are freed by RCU (or only freed after
// having been deleted from the idr tree *and* a synchronize_rcu() grace
// period).
//

extern "C" {
    pub fn idr_preload(gfp_mask: gfp_t);
}
extern "C" {
    pub fn idr_alloc(: *mut idr, ptr: *mut c_void, start: c_int, end: c_int, _arg: gfp_t) -> c_int;
}
extern "C" {
    pub fn idr_alloc_cyclic(: *mut idr, ptr: *mut c_void, start: c_int, end: c_int, _arg: gfp_t) -> c_int;
}
extern "C" {
    pub fn idr_destroy(: *mut idr);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __class_idr {
    pub idr: *mut idr,
    pub id: c_int,
}

//
// idr_init_base() - Initialise an IDR.
// @idr: IDR handle.
// @base: The base value for the IDR.
//
// This variation of idr_init() creates an IDR which will allocate IDs
// starting at %base.
//
// idr_init() - Initialise an IDR.
// @idr: IDR handle.
//
// Initialise a dynamically allocated IDR.  To initialise a
// statically allocated IDR, use DEFINE_IDR().
//
// idr_is_empty() - Are there any IDs allocated?
// @idr: IDR handle.
//
// Return: %true if any IDs have been allocated from this IDR.
//
// idr_preload_end - end preload section started with idr_preload()
//
// Each idr_preload() should be matched with an invocation of this
// function.  See idr_preload() for details.
//
// idr_for_each_entry() - Iterate over an IDR's elements of a given type.
// @idr: IDR handle.
// @entry: The type * to use as cursor
// @id: Entry ID.
//
// @entry and @id do not need to be initialized before the loop, and
// after normal termination @entry is left with the value NULL.  This
// is convenient for a "not found" value.
//

//
// idr_for_each_entry_ul() - Iterate over an IDR's elements of a given type.
// @idr: IDR handle.
// @entry: The type * to use as cursor.
// @tmp: A temporary placeholder for ID.
// @id: Entry ID.
//
// @entry and @id do not need to be initialized before the loop, and
// after normal termination @entry is left with the value NULL.  This
// is convenient for a "not found" value.
//

//
// idr_for_each_entry_continue() - Continue iteration over an IDR's elements of a given type
// @idr: IDR handle.
// @entry: The type * to use as a cursor.
// @id: Entry ID.
//
// Continue to iterate over entries, continuing after the current position.
//

//
// idr_for_each_entry_continue_ul() - Continue iteration over an IDR's elements of a given type
// @idr: IDR handle.
// @entry: The type * to use as a cursor.
// @tmp: A temporary placeholder for ID.
// @id: Entry ID.
//
// Continue to iterate over entries, continuing after the current position.
// After normal termination @entry is left with the value NULL.  This
// is convenient for a "not found" value.
//

//
// IDA - ID Allocator, use when translation from id to pointer isn't necessary.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ida_bitmap {
    pub bitmap: [c_ulong; IDA_BITMAP_LONGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ida {
    pub xa: xarray,
}

extern "C" {
    pub fn ida_alloc_range(: *mut ida, min: c_uint, max: c_uint, _arg: gfp_t) -> c_int;
}
extern "C" {
    pub fn ida_free(: *mut ida, id: c_uint);
}
extern "C" {
    pub fn ida_destroy(ida: *mut ida);
}
extern "C" {
    pub fn ida_find_first_range(ida: *mut ida, min: c_uint, max: c_uint) -> c_int;
}
//
// ida_alloc() - Allocate an unused ID.
// @ida: IDA handle.
// @gfp: Memory allocation flags.
//
// Allocate an ID between 0 and %INT_MAX, inclusive.
//
// Context: Any context. It is safe to call this function without
// locking in your code.
// Return: The allocated ID, or %-ENOMEM if memory could not be allocated,
// or %-ENOSPC if there are no free IDs.
//
extern "C" {
    pub fn ida_alloc_range(_arg: ida, _arg: 0, _arg: ~0, _arg: gfp) -> return;
}
//
// ida_alloc_min() - Allocate an unused ID.
// @ida: IDA handle.
// @min: Lowest ID to allocate.
// @gfp: Memory allocation flags.
//
// Allocate an ID between @min and %INT_MAX, inclusive.
//
// Context: Any context. It is safe to call this function without
// locking in your code.
// Return: The allocated ID, or %-ENOMEM if memory could not be allocated,
// or %-ENOSPC if there are no free IDs.
//
extern "C" {
    pub fn ida_alloc_range(_arg: ida, _arg: min, _arg: ~0, _arg: gfp) -> return;
}
//
// ida_alloc_max() - Allocate an unused ID.
// @ida: IDA handle.
// @max: Highest ID to allocate.
// @gfp: Memory allocation flags.
//
// Allocate an ID between 0 and @max, inclusive.
//
// Context: Any context. It is safe to call this function without
// locking in your code.
// Return: The allocated ID, or %-ENOMEM if memory could not be allocated,
// or %-ENOSPC if there are no free IDs.
//
extern "C" {
    pub fn ida_alloc_range(_arg: ida, _arg: 0, _arg: max, _arg: gfp) -> return;
}
extern "C" {
    pub fn xa_empty(_arg: &ida->xa) -> return;
}
extern "C" {
    pub fn ida_find_first_range(_arg: ida, _arg: 0, _arg: ~0) -> return;
}
