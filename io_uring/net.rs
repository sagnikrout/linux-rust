//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/net.h
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
pub struct io_async_msghdr {

    pub vec: iou_vec,
    pub namelen: c_int,
    pub fast_iov: iovec,
    pub controllen: __kernel_size_t,
    pub payloadlen: __kernel_size_t,
    pub uaddr: *mut sockaddr __user,
    pub msg: msghdr,
    pub addr: sockaddr_storage,

}

extern "C" {
    pub fn io_shutdown_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_shutdown(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_sendmsg_recvmsg_cleanup(req: *mut io_kiocb);
}
extern "C" {
    pub fn io_sendmsg_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_sendmsg(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_send(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_recvmsg_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_recvmsg(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_recv(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_sendrecv_fail(req: *mut io_kiocb);
}
extern "C" {
    pub fn io_accept_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_accept(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_socket_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_socket(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_socket_bpf_populate(bctx: *mut io_uring_bpf_ctx, req: *mut io_kiocb);
}
extern "C" {
    pub fn io_connect_bpf_populate(bctx: *mut io_uring_bpf_ctx, req: *mut io_kiocb);
}
extern "C" {
    pub fn io_connect_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_connect(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_sendmsg_zc(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_send_zc_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_send_zc_cleanup(req: *mut io_kiocb);
}
extern "C" {
    pub fn io_bind_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_bind(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_listen_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int;
}
extern "C" {
    pub fn io_listen(req: *mut io_kiocb, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn io_netmsg_cache_free(entry: *const c_void);
}

