//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/phonet/phonet.h
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
// File: af_phonet.h
//
// Phonet sockets kernel definitions
//
// Copyright (C) 2008 Nokia Corporation.
//

//
// The lower layers may not require more space, ever. Make sure it's
// enough.
//

//
// Every Phonet* socket has this structure first in its
// protocol-specific structure under name c.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pn_sock {
    pub sk: sock,
    pub sobject: u16,
    pub dobject: u16,
    pub resource: u8,
}

extern "C" {
    pub fn pn_sock_init();
}
extern "C" {
    pub fn pn_deliver_sock_broadcast(net: *mut net, skb: *mut sk_buff);
}
extern "C" {
    pub fn phonet_get_local_port_range(min: *mut c_int, max: *mut c_int);
}
extern "C" {
    pub fn pn_sock_hash(sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn pn_sock_unhash(sk: *mut sock);
}
extern "C" {
    pub fn pn_sock_get_port(sk: *mut sock, sport: c_ushort) -> c_int;
}
extern "C" {
    pub fn pn_sock_bind_res(sock: *mut sock, res: u8) -> c_int;
}
extern "C" {
    pub fn pn_sock_unbind_res(sk: *mut sock, res: u8) -> c_int;
}
extern "C" {
    pub fn pn_sock_unbind_all_res(sk: *mut sock);
}
//
// Get the other party's sockaddr from received skb. The skb begins
// with a Phonet header.
//
// Protocols in Phonet protocol family.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phonet_protocol {
    pub ops: *const proto_ops,
    pub prot: *mut proto,
    pub sock_type: c_int,
}

extern "C" {
    pub fn phonet_sysctl_init() -> c_int;
}
extern "C" {
    pub fn phonet_sysctl_exit();
}
extern "C" {
    pub fn isi_register() -> c_int;
}
extern "C" {
    pub fn isi_unregister();
}
// A positive return value means that the ioctl was not processed
