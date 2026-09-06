//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/protocol.h
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
// Definitions for the protocol dispatcher.
//
// Version:	@(#)protocol.h	1.0.2	05/07/93
//
// Author:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//
// Changes:
// Alan Cox	:	Added a name field and a frag handler
// field for later.
// Alan Cox	:	Cleaned up, and sorted types.
// Pedro Roque	:	inet6 protocols
//

// This is one larger than the largest protocol value that can be
// found in an ipv4 or ipv6 header.  Since in both cases the protocol
// value is presented in a __u8, this is defined to be 256.
//
pub const MAX_INET_PROTOS: c_int = 256;
// This is used to register protocols.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_protocol {
    pub skb): *mut *mut int (handler)(struct sk_buff,
// This returns an error if we weren't able to handle the error.
    pub info): *mut *mut *mut int (err_handler)(struct sk_buff skb, u32,
// does the protocol do more stringent
// icmp tag validation than simple
// socket lookup?
//
    pub secret: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet6_protocol {
    pub skb): *mut *mut int (handler)(struct sk_buff,
// This returns an error if we weren't able to handle the error.
    pub info): __be32,
    pub /: *mut *mut unsigned int flags; / INET6_PROTO_xxx,
    pub secret: u32,
}

pub const INET6_PROTO_NOPOLICY: c_uint = 0x1;
pub const INET6_PROTO_FINAL: c_uint = 0x2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_offload {
    pub callbacks: offload_callbacks,
    pub /: *mut *mut unsigned int flags; / Flags used by IPv6 for now,
    pub secret: u32,
}

// This should be set for any extension header which is compatible with GSO.
pub const INET6_PROTO_GSO_EXTHDR: c_uint = 0x1;
// This is used to register socket interfaces for IP protocols.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_protosw {
    pub list: list_head,
// These two fields form the lookup key.
    pub /: *mut *mut unsigned short type; / This is the 2nd argument to socket(2).,
    pub /: *mut *mut unsigned short protocol; / This is the L4 protocol number.,
    pub prot: *mut proto,
    pub ops: *const proto_ops,
    pub /: *mut *mut *mut unsigned char flags; / See INET_PROTOSW_ below.,
}

pub const INET_PROTOSW_REUSE: c_uint = 0x01	     /* Are ports automatically reusable? */;
pub const INET_PROTOSW_PERMANENT: c_uint = 0x02  /* Permanent protocols are unremovable. */;
pub const INET_PROTOSW_ICSK: c_uint = 0x04  /* Is this an inet_connection_sock? */;

extern "C" {
    pub fn inet_add_protocol(prot: *const net_protocol, num: c_uchar) -> c_int;
}
extern "C" {
    pub fn inet_del_protocol(prot: *const net_protocol, num: c_uchar) -> c_int;
}
extern "C" {
    pub fn inet_add_offload(prot: *const net_offload, num: c_uchar) -> c_int;
}
extern "C" {
    pub fn inet_del_offload(prot: *const net_offload, num: c_uchar) -> c_int;
}
extern "C" {
    pub fn inet_register_protosw(p: *mut inet_protosw);
}
extern "C" {
    pub fn inet_unregister_protosw(p: *mut inet_protosw);
}

extern "C" {
    pub fn inet6_add_protocol(prot: *const inet6_protocol, num: c_uchar) -> c_int;
}
extern "C" {
    pub fn inet6_del_protocol(prot: *const inet6_protocol, num: c_uchar) -> c_int;
}
extern "C" {
    pub fn inet6_register_protosw(p: *mut inet_protosw) -> c_int;
}
extern "C" {
    pub fn inet6_unregister_protosw(p: *mut inet_protosw);
}

extern "C" {
    pub fn inet6_add_offload(prot: *const net_offload, num: c_uchar) -> c_int;
}
extern "C" {
    pub fn inet6_del_offload(prot: *const net_offload, num: c_uchar) -> c_int;
}
