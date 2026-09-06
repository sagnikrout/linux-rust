//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dm-io.h
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
// Copyright (C) 2003 Sistina Software
// Copyright (C) 2004 - 2008 Red Hat, Inc. All rights reserved.
//
// Device-Mapper low-level I/O.
//
// This file is released under the GPL.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_io_region {
    pub bdev: *mut block_device,
    pub sector: sector_t,
    pub /: *mut *mut sector_t count; / If this is zero the region is ignored.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_list {
    pub next: *mut page_list,
    pub page: *mut page,
}

extern "C" {
    pub fn void(error: *mut *mut io_notify_fn)(unsigned long int, unsup: unsigned long int, context: *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dm_io_mem_type {
    DM_IO_PAGE_LIST,/* Page list */
    DM_IO_BIO,	/* Bio vector */
    DM_IO_VMA,	/* Virtual memory area */
    DM_IO_KMEM,	/* Kernel memory */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_io_memory {
    pub type: dm_io_mem_type,
    pub offset: c_uint,
    pub pl: *mut page_list,
    pub bio: *mut bio,
    pub vma: *mut c_void,
    pub addr: *mut c_void,
    pub ptr: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_io_notify {
    pub /: *mut *mut io_notify_fn fn; / Callback for asynchronous requests,
    pub /: *mut *mut *mut void context; / Passed to callback,
}

//
// IO request structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_io_request {
    pub /: *mut *mut blk_opf_t bi_opf; / Request type and flags,
    pub /: *mut *mut dm_io_memory mem; / Memory to use for io,
    pub /: *mut *mut dm_io_notify notify; / Synchronous if notify.fn is NULL,
    pub /: *mut *mut *mut dm_io_client client; / Client memory handler,
}

//
// For async io calls, users can alternatively use the dm_io() function below
// and dm_io_client_create() to create private mempools for the client.
//
// Create/destroy may block.
//
extern "C" {
    pub fn dm_io_client_destroy(client: *mut dm_io_client);
}
//
// IO interface using private per-client pools.
// Each bit in the optional 'sync_error_bits' bitset indicates whether an
// error occurred doing io to the corresponding region.
//

