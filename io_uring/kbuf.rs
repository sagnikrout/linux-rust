//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/kbuf.h
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

// ring mapped provided buffers
// buffers are consumed incrementally rather than always fully
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_buffer_list {
//
// If the IOBL_BUF_RING flag is set, then buf_ring is used. If not, then
// these are classic provided buffers and ->buf_list is used.
//
    pub buf_list: list_head,
    pub buf_ring: *mut io_uring_buf_ring,
}

// count of classic/legacy buffers in buffer list
// below is for ring provided buffers
//
// minimum required amount to be left to reuse an incrementally
// consumed buffer. If less than this is left at consumption time,
// buffer is done and head is incremented to the next buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_buffer {
    pub list: list_head,
    pub addr: __u64,
    pub len: __u32,
    pub bid: __u16,
    pub bgid: __u16,
}

// can alloc a bigger vec
// if bigger vec allocated, free old one
#[repr(C)]
#[derive(Copy, Clone)]
pub struct buf_sel_arg {
    pub iovs: *mut iovec,
    pub out_len: usize,
    pub max_len: usize,
    pub nr_iovs: c_ushort,
    pub mode: c_ushort,
    pub buf_group: c_ushort,
    pub partial_map: c_ushort,
}

extern "C" {
    pub fn io_destroy_buffers(ctx: *mut io_ring_ctx);
}
extern "C" {
    pub fn io_remove_buffers_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_provide_buffers_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_manage_buffers_legacy(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_register_pbuf_ring(ctx: *mut io_ring_ctx, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn io_unregister_pbuf_ring(ctx: *mut io_ring_ctx, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn io_register_pbuf_status(ctx: *mut io_ring_ctx, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn io_kbuf_recycle_legacy(req: *mut io_kiocb, issue_flags: unsigned) -> bool;
}
extern "C" {
    pub fn io_kbuf_drop_legacy(req: *mut io_kiocb);
}
extern "C" {
    pub fn io_kbuf_recycle_ring(_arg: req, _arg: bl) -> return;
}
extern "C" {
    pub fn io_kbuf_recycle_legacy(_arg: req, _arg: issue_flags) -> return;
}
extern "C" {
    pub fn __io_put_kbufs(_arg: req, _arg: bl, _arg: len, _arg: 1) -> return;
}
extern "C" {
    pub fn __io_put_kbufs(_arg: req, _arg: bl, _arg: len, _arg: nbufs) -> return;
}
