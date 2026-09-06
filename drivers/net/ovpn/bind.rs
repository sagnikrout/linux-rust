//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ovpn/bind.h
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
// OpenVPN data channel offload
//
// Copyright (C) 2012-2025 OpenVPN, Inc.
//
// Author:	James Yonan <james@openvpn.net>
// Antonio Quartulli <antonio@openvpn.net>
//

//
// union ovpn_sockaddr - basic transport layer address
// @in4: IPv4 address
// @in6: IPv6 address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ovpn_sockaddr {
    pub in4: sockaddr_in,
    pub in6: sockaddr_in6,
}

//
// struct ovpn_bind - remote peer binding
// @remote: the remote peer sockaddress
// @local: local endpoint used to talk to the peer
// @local.ipv4: local IPv4 used to talk to the peer
// @local.ipv6: local IPv6 used to talk to the peer
// @rcu: used to schedule RCU cleanup job
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovpn_bind {
    pub /: *mut *mut ovpn_sockaddr remote; / remote sockaddr,
    pub ipv4: in_addr,
    pub ipv6: in6_addr,
    pub local: },
    pub rcu: rcu_head,
}

//
// ovpn_bind_skb_src_match - match packet source with binding
// @bind: the binding to match
// @skb: the packet to match
//
// Return: true if the packet source matches the remote peer sockaddr
// in the binding
//
extern "C" {
    pub fn ovpn_bind_reset(peer: *mut ovpn_peer, bind: *mut ovpn_bind);
}
