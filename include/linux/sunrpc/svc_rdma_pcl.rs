//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/svc_rdma_pcl.h
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
// Copyright (c) 2020, Oracle and/or its affiliates
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_rdma_segment {
    pub rs_handle: u32,
    pub rs_length: u32,
    pub rs_offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_rdma_chunk {
    pub ch_list: list_head,
    pub ch_position: u32,
    pub ch_length: u32,
    pub ch_payload_length: u32,
    pub ch_segcount: u32,
    pub ch_segments: [svc_rdma_segment; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_rdma_pcl {
    pub cl_count: c_uint,
    pub cl_chunks: list_head,
}

//
// pcl_init - Initialize a parsed chunk list
// @pcl: parsed chunk list to initialize
//
// pcl_is_empty - Return true if parsed chunk list is empty
// @pcl: parsed chunk list
//
extern "C" {
    pub fn list_empty(_arg: &pcl->cl_chunks) -> return;
}
//
// pcl_first_chunk - Return first chunk in a parsed chunk list
// @pcl: parsed chunk list
//
// Returns the first chunk in the list, or NULL if the list is empty.
//
// pcl_next_chunk - Return next chunk in a parsed chunk list
// @pcl: a parsed chunk list
// @chunk: chunk in @pcl
//
// Returns the next chunk in the list, or NULL if @chunk is already last.
//
extern "C" {
    pub fn list_next_entry(_arg: chunk, _arg: ch_list) -> return;
}
//
// pcl_for_each_chunk - Iterate over chunks in a parsed chunk list
// @pos: the loop cursor
// @pcl: a parsed chunk list
//

//
// pcl_for_each_segment - Iterate over segments in a parsed chunk
// @pos: the loop cursor
// @chunk: a parsed chunk
//

//
// pcl_chunk_end_offset - Return offset of byte range following @chunk
// @chunk: chunk in @pcl
//
// Returns starting offset of the region just after @chunk
//
extern "C" {
    pub fn xdr_align_size(chunk->ch_payload_length: chunk->ch_position +) -> return;
}
extern "C" {
    pub fn pcl_free(pcl: *mut svc_rdma_pcl);
}
extern "C" {
    pub fn pcl_alloc_call(rctxt: *mut svc_rdma_recv_ctxt, p: *mut __be32) -> bool;
}
extern "C" {
    pub fn pcl_alloc_read(rctxt: *mut svc_rdma_recv_ctxt, p: *mut __be32) -> bool;
}
