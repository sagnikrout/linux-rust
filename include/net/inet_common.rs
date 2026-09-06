//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/inet_common.h
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
// INET4 prototypes used by INET6
//
extern "C" {
    pub fn inet_release(sock: *mut socket) -> c_int;
}
extern "C" {
    pub fn inet_send_prepare(sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn inet_sendmsg(sock: *mut socket, msg: *mut msghdr, size: usize) -> c_int;
}
extern "C" {
    pub fn inet_splice_eof(sock: *mut socket);
}
extern "C" {
    pub fn inet_shutdown(sock: *mut socket, how: c_int) -> c_int;
}
extern "C" {
    pub fn inet_listen(sock: *mut socket, backlog: c_int) -> c_int;
}
extern "C" {
    pub fn __inet_listen_sk(sk: *mut sock, backlog: c_int) -> c_int;
}
extern "C" {
    pub fn inet_sock_destruct(sk: *mut sock);
}
extern "C" {
    pub fn inet_bind(sock: *mut socket, uaddr: *mut sockaddr_unsized, addr_len: c_int) -> c_int;
}
extern "C" {
    pub fn inet_bind_sk(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: c_int) -> c_int;
}
// Don't allocate port at this moment, defer to connect.

// Grab and release socket lock.

// Called from BPF program.

// Skip CAP_NET_BIND_SERVICE check.

extern "C" {
    pub fn inet_ioctl(sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn inet_recv_error(sk: *mut sock, msg: *mut msghdr, len: c_int) -> c_int;
}
extern "C" {
    pub fn inet_gro_complete(skb: *mut sk_buff, nhoff: c_int) -> c_int;
}

