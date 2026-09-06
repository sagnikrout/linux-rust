//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/rsrc.h
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

pub const IO_VEC_CACHE_SOFT_CAP: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_rsrc_node {
    pub type: c_uchar,
    pub refs: c_int,
    pub tag: u64,
    pub file_ptr: c_ulong,
    pub buf: *mut io_mapped_ubuf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_mapped_ubuf {
    pub ubuf: u64,
    pub len: usize,
    pub nr_bvecs: c_uint,
    pub folio_shift: c_uint,
    pub refs: refcount_t,
    pub flags: u8,
    pub dir: u8,
    pub ): *mut *mut void (release)(void,
    pub priv: *mut c_void,
    pub __counted_by(nr_bvecs): bio_vec bvec[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_imu_folio_data {
// Head folio can be partially included in the fixed buf
    pub nr_pages_head: c_uint,
// For non-head/tail folios, has to be fully included
    pub nr_pages_mid: c_uint,
    pub folio_shift: c_uint,
    pub nr_folios: c_uint,
    pub first_folio_page_idx: c_ulong,
}

extern "C" {
    pub fn io_rsrc_cache_init(ctx: *mut io_ring_ctx) -> bool;
}
extern "C" {
    pub fn io_rsrc_cache_free(ctx: *mut io_ring_ctx);
}
extern "C" {
    pub fn io_free_rsrc_node(ctx: *mut io_ring_ctx, node: *mut io_rsrc_node);
}
extern "C" {
    pub fn io_rsrc_data_free(ctx: *mut io_ring_ctx, data: *mut io_rsrc_data);
}
extern "C" {
    pub fn io_rsrc_data_alloc(data: *mut io_rsrc_data, nr: unsigned) -> c_int;
}
extern "C" {
    pub fn io_register_clone_buffers(ctx: *mut io_ring_ctx, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn io_sqe_buffers_unregister(ctx: *mut io_ring_ctx) -> c_int;
}
extern "C" {
    pub fn io_sqe_files_unregister(ctx: *mut io_ring_ctx) -> c_int;
}
extern "C" {
    pub fn io_validate_user_buf_range(uaddr: u64, ulen: u64) -> c_int;
}
extern "C" {
    pub fn io_files_update(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_files_update_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn __io_account_mem(user: *mut user_struct, nr_pages: c_ulong) -> c_int;
}
extern "C" {
    pub fn io_vec_free(iv: *mut iou_vec);
}
extern "C" {
    pub fn io_vec_realloc(iv: *mut iou_vec, nr_entries: unsigned) -> c_int;
}
