//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/android/binder_alloc.h
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
// Copyright (C) 2017 Google, Inc.
//

//
// struct binder_buffer - buffer used for binder transactions
// @entry:              entry alloc->buffers
// @rb_node:            node for allocated_buffers/free_buffers rb trees
// @free:               %true if buffer is free
// @clear_on_free:      %true if buffer must be zeroed after use
// @allow_user_free:    %true if user is allowed to free buffer
// @async_transaction:  %true if buffer is in use for an async txn
// @oneway_spam_suspect: %true if total async allocate size just exceed
// spamming detect threshold
// @debug_id:           unique ID for debugging
// @transaction:        pointer to associated struct binder_transaction
// @target_node:        struct binder_node associated with this buffer
// @data_size:          size of @transaction data
// @offsets_size:       size of array of offsets
// @extra_buffers_size: size of space for other objects (like sg lists)
// @user_data:          user pointer to base of buffer space
// @pid:                pid to attribute the buffer to (caller)
//
// Bookkeeping structure for binder transaction buffers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_buffer {
    pub /: *mut *mut list_head entry; / free and allocated entries by address,
    pub /: *mut *mut rb_node rb_node; / free entry by size or allocated entry,
// by address
    pub free:1: unsigned,
    pub clear_on_free:1: unsigned,
    pub allow_user_free:1: unsigned,
    pub async_transaction:1: unsigned,
    pub oneway_spam_suspect:1: unsigned,
    pub debug_id:27: unsigned,
    pub transaction: *mut binder_transaction,
    pub target_node: *mut binder_node,
    pub data_size: usize,
    pub offsets_size: usize,
    pub extra_buffers_size: usize,
    pub user_data: c_ulong,
    pub pid: c_int,
}

//
// struct binder_shrinker_mdata - binder metadata used to reclaim pages
// @lru:         LRU entry in binder_freelist
// @alloc:       binder_alloc owning the page to reclaim
// @page_index:  offset in @alloc->pages[] into the page to reclaim
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_shrinker_mdata {
    pub lru: list_head,
    pub alloc: *mut binder_alloc,
    pub page_index: c_ulong,
}

//
// struct binder_alloc - per-binder proc state for binder allocator
// @mutex:              protects binder_alloc fields
// @mm:                 copy of task->mm (invariant after open)
// @vm_start:           base of per-proc address space mapped via mmap
// @buffers:            list of all buffers for this proc
// @free_buffers:       rb tree of buffers available for allocation
// sorted by size
// @allocated_buffers:  rb tree of allocated buffers sorted by address
// @free_async_space:   VA space available for async buffers. This is
// initialized at mmap time to 1/2 the full VA space
// @pages:              array of struct page
// @freelist:           lru list to use for free pages (invariant after init)
// @buffer_size:        size of address space specified via mmap
// @pid:                pid for associated binder_proc (invariant after init)
// @pages_high:         high watermark of offset in @pages
// @mapped:             whether the vm area is mapped, each binder instance is
// allowed a single mapping throughout its lifetime
// @oneway_spam_detected: %true if oneway spam detection fired, clear that
// flag once the async buffer has returned to a healthy state
//
// Bookkeeping structure for per-proc address space management for binder
// buffers. It is normally initialized during binder_init() and binder_mmap()
// calls. The address space is used for both user-visible buffers and for
// struct binder_buffer objects used to track the user buffers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct binder_alloc {
    pub mutex: mutex,
    pub mm: *mut mm_struct,
    pub vm_start: c_ulong,
    pub buffers: list_head,
    pub free_buffers: rb_root,
    pub allocated_buffers: rb_root,
    pub free_async_space: usize,
    pub pages: *mut page,
    pub freelist: *mut list_lru,
    pub buffer_size: usize,
    pub pid: c_int,
    pub pages_high: usize,
    pub mapped: bool,
    pub oneway_spam_detected: bool,
}

extern "C" {
    pub fn binder_alloc_init(alloc: *mut binder_alloc);
}
extern "C" {
    pub fn binder_alloc_shrinker_init() -> c_int;
}
extern "C" {
    pub fn binder_alloc_shrinker_exit();
}
extern "C" {
    pub fn binder_alloc_vma_close(alloc: *mut binder_alloc);
}
extern "C" {
    pub fn binder_alloc_deferred_release(alloc: *mut binder_alloc);
}
extern "C" {
    pub fn binder_alloc_get_allocated_count(alloc: *mut binder_alloc) -> c_int;
}
//
// binder_alloc_get_free_async_space() - get free space available for async
// @alloc:	binder_alloc for this proc
//
// Return:	the bytes remaining in the address-space for async transactions
//

extern "C" {
    pub fn __binder_alloc_init(alloc: *mut binder_alloc, freelist: *mut list_lru);
}

