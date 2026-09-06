//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ping.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Definitions for the "ping" module.
//

// PING_HTABLE_SIZE must be power of 2
pub const PING_HTABLE_SIZE: c_int = 64;

// Compatibility glue so we can support IPv6 when it's compiled as a module
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pingv6_ops {
    pub len): *mut *mut *mut *mut int (ipv6_recv_error)(struct sock sk, struct msghdr msg, int,
    pub skb): *mut sk_buff,
    pub skb): *mut sk_buff,
    pub err): *mut *mut int (icmpv6_err_convert)(u8 type, u8 code, int,
    pub payload): *mut __be16 port, u32 info, u8,
    pub strict): *const *const net_device dev, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ping_iter_state {
    pub p: seq_net_private,
    pub bucket: c_int,
    pub family: sa_family_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pingfakehdr {
    pub icmph: icmphdr,
    pub msg: *mut msghdr,
    pub family: sa_family_t,
    pub wcheck: __wsum,
}

extern "C" {
    pub fn ping_get_port(sk: *mut sock, ident: c_ushort) -> c_int;
}
extern "C" {
    pub fn ping_unhash(sk: *mut sock);
}
extern "C" {
    pub fn ping_init_sock(sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn ping_close(sk: *mut sock, timeout: c_long);
}
extern "C" {
    pub fn ping_bind(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: c_int) -> c_int;
}
extern "C" {
    pub fn ping_err(skb: *mut sk_buff, offset: c_int, info: u32);
}
extern "C" {
    pub fn ping_queue_rcv_skb(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ping_rcv(skb: *mut sk_buff) -> skb_drop_reason;
}

extern "C" {
    pub fn ping_seq_stop(seq: *mut seq_file, v: *mut c_void);
}
extern "C" {
    pub fn ping_proc_init() -> int __init;
}
extern "C" {
    pub fn ping_proc_exit();
}

extern "C" {
    pub fn ping_init() -> void __init;
}
extern "C" {
    pub fn pingv6_init() -> int  __init;
}
extern "C" {
    pub fn pingv6_exit();
}
