//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/rw.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_meta_state {
    pub seed: u32,
    pub iter_meta: iov_iter_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_async_rw {
    pub vec: iou_vec,
    pub bytes_done: usize,
    pub iter: iov_iter,
    pub iter_state: iov_iter_state,
    pub fast_iov: iovec,
    pub buf_group: unsigned,
//
// wpq is for buffered io, while meta fields are used with
// direct io
//
    pub wpq: wait_page_queue,
    pub meta: uio_meta,
    pub meta_state: io_meta_state,
}

extern "C" {
    pub fn io_prep_read_fixed(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_prep_write_fixed(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_prep_readv_fixed(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_prep_writev_fixed(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_prep_readv(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_prep_writev(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_prep_read(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_prep_write(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_read(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_write(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_read_fixed(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_write_fixed(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_readv_writev_cleanup(req: *mut io_kiocb);
}
extern "C" {
    pub fn io_rw_fail(req: *mut io_kiocb);
}
extern "C" {
    pub fn io_req_rw_complete(tw_req: io_tw_req, tw: io_tw_token_t);
}
extern "C" {
    pub fn io_read_mshot_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_read_mshot(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_rw_cache_free(entry: *const c_void);
}
