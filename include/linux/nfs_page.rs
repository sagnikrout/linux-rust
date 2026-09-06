//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nfs_page.h
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


// SPDX-License-Identifier: GPL-2.0
//
// linux/include/linux/nfs_page.h
//
// Copyright (C) 2000 Trond Myklebust
//
// NFS page cache wrapper.
//

//
// Valid flags for a dirty buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_page {
    pub /: *mut *mut list_head wb_list; / Defines state of page:,
    pub /: *mut *mut *mut page wb_page; / page to read in/write out,
    pub wb_folio: *mut folio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_pageio_ops {
    pub ): *mut *mut *mut void (pg_init)(struct nfs_pageio_descriptor , struct nfs_page,
    pub ): *mut nfs_page,
    pub ): *mut *mut int (pg_doio)(struct nfs_pageio_descriptor,
    pub ): *mut nfs_page,
    pub ): *mut *mut void (pg_cleanup)(struct nfs_pageio_descriptor,
    pub u32): *mut *mut *mut (pg_get_mirror)(struct nfs_pageio_descriptor ,,
    pub u32): *mut *mut *mut u32 (pg_set_mirror)(struct nfs_pageio_descriptor ,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_rw_ops {
    pub (*rw_alloc_header)(void): *mut nfs_pgio_header,
    pub ): *mut *mut void (rw_free_header)(struct nfs_pgio_header,
    pub ): *mut inode,
    pub ): *mut *mut *mut void (rw_result)(struct rpc_task , struct nfs_pgio_header,
    pub int): *mut *mut rpc_task_setup ,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_pgio_mirror {
    pub pg_list: list_head,
    pub pg_bytes_written: c_ulong,
    pub pg_count: usize,
    pub pg_bsize: usize,
    pub pg_base: c_uint,
    pub 1: unsigned char pg_recoalesce :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_pageio_descriptor {
    pub pg_inode: *mut inode,
    pub pg_ops: *const nfs_pageio_ops,
    pub pg_rw_ops: *const nfs_rw_ops,
    pub pg_ioflags: c_int,
    pub pg_error: c_int,
    pub pg_rpc_callops: *const rpc_call_ops,
    pub pg_completion_ops: *const nfs_pgio_completion_ops,
    pub pg_lseg: *mut pnfs_layout_segment,
    pub pg_io_completion: *mut nfs_io_completion,
    pub pg_dreq: *mut nfs_direct_req,

    pub pg_netfs: *mut c_void,

    pub /: *mut *mut unsigned int pg_bsize; / default bsize for mirrors,
    pub pg_mirror_count: u32,
    pub pg_mirrors: *mut nfs_pgio_mirror,
    pub pg_mirrors_static: [nfs_pgio_mirror; 1],
    pub pg_mirrors_dynamic: *mut nfs_pgio_mirror,
    pub /: *mut *mut u32 pg_mirror_idx; / current mirror,
    pub pg_maxretrans: c_ushort,
    pub 1: unsigned char pg_moreio :,
}

// arbitrarily selected limit to number of mirrors
pub const NFS_PAGEIO_DESCRIPTOR_MIRROR_MAX: c_int = 16;
extern "C" {
    pub fn nfs_release_request(: *mut nfs_page);
}
extern "C" {
    pub fn nfs_pageio_complete(desc: *mut nfs_pageio_descriptor);
}
extern "C" {
    pub fn nfs_pageio_cond_complete(: *mut nfs_pageio_descriptor, _arg: pgoff_t);
}
extern "C" {
    pub fn nfs_unlock_request(req: *mut nfs_page);
}
extern "C" {
    pub fn nfs_unlock_and_release_request(: *mut nfs_page);
}
extern "C" {
    pub fn nfs_page_group_lock(: *mut nfs_page) -> c_int;
}
extern "C" {
    pub fn nfs_page_group_unlock(: *mut nfs_page);
}
extern "C" {
    pub fn nfs_page_group_sync_on_bit(: *mut nfs_page, int: unsigned) -> bool;
}
extern "C" {
    pub fn nfs_page_group_sync_on_bit_locked(: *mut nfs_page, int: unsigned) -> bool;
}
extern "C" {
    pub fn nfs_page_set_headlock(req: *mut nfs_page) -> c_int;
}
extern "C" {
    pub fn nfs_page_clear_headlock(req: *mut nfs_page);
}
extern "C" {
    pub fn nfs_async_iocounter_wait(: *mut rpc_task, : *mut nfs_lock_context) -> bool;
}
//
// nfs_page_to_folio - Retrieve a struct folio for the request
// @req: pointer to a struct nfs_page
//
// If a folio was assigned to @req, then return it, otherwise return NULL.
//
// nfs_page_to_page - Retrieve a struct page for the request
// @req: pointer to a struct nfs_page
// @pgbase: folio byte offset
//
// Return the page containing the byte that is at offset @pgbase relative
// to the start of the folio.
// Note: The request starts at offset @req->wb_pgbase.
//
extern "C" {
    pub fn folio_page(_arg: folio, PAGE_SHIFT: pgbase >>) -> return;
}
//
// nfs_page_to_inode - Retrieve an inode for the request
// @req: pointer to a struct nfs_page
//
// nfs_page_max_length - Retrieve the maximum possible length for a request
// @req: pointer to a struct nfs_page
//
// Returns the maximum possible length of a request
//
extern "C" {
    pub fn folio_size(_arg: folio) -> return;
}
//
// Lock the page of an asynchronous request
//
// nfs_list_add_request - Insert a request into a list
// @req: request
// @head: head of list into which to insert the request.
//
// nfs_list_move_request - Move a request to a new list
// @req: request
// @head: head of list into which to insert the request.
//
// nfs_list_remove_request - Remove a request from its wb_list
// @req: request
//
extern "C" {
    pub fn list_entry(_arg: head, nfs_page: struct, _arg: wb_list) -> return;
}
