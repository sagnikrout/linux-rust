//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/if_pppox.h
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
// Linux PPP over X - Generic PPP transport layer sockets
// Linux PPP over Ethernet (PPPoE) Socket Implementation (RFC 2516)
//
// This file supplies definitions required by the PPP over Ethernet driver
// (pppox.c).  All version information wrt this file is located in pppox.c
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pppoe_opt {
    pub socket*/: *mut *mut *mut net_device dev; / device associated with,
    pub /: *mut *mut int ifindex; / ifindex of device associated with socket,
    pub to*/: *mut *mut pppoe_addr pa; / what this socket is bound,
    pub /: *mut *mut work_padt_work;/ Work item for handling PADT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pptp_opt {
    pub src_addr: pptp_addr,
    pub dst_addr: pptp_addr,
    pub ack_recv: u32 ack_sent,,
    pub seq_recv: u32 seq_sent,,
    pub ppp_flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pppox_sock {
// struct sock must be the first member of pppox_sock
    pub sk: sock,
    pub chan: ppp_channel,
    pub /: *mut *mut *mut pppox_sock __rcu next; / for hash table,
    pub pppoe: pppoe_opt,
    pub pptp: pptp_opt,
    pub proto: },
    pub num: __be16,
}

extern "C" {
    pub fn container_of(_arg: sk, pppox_sock: struct, _arg: sk) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pppox_proto {
    pub kern): *mut *mut *mut *mut int (create)(struct net net, struct socket sock, int,
    pub arg): c_ulong,
    pub owner: *mut module,
}

extern "C" {
    pub fn register_pppox_proto(proto_num: c_int, pp: *const pppox_proto) -> c_int;
}
extern "C" {
    pub fn unregister_pppox_proto(proto_num: c_int);
}
extern "C" {
    pub fn pppox_ioctl(sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn pppox_compat_ioctl(sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int;
}
// PPPoX socket states
