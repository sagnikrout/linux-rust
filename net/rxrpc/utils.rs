//! Automatically rewritten from C to Rust
//! Source: net/rxrpc/utils.c
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
// Utility routines
//
// Copyright (C) 2015 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Fill out a peer address from a socket buffer containing a packet.
//
#[no_mangle]
pub unsafe extern "C" fn rxrpc_extract_addr_from_skb(srx: *mut sockaddr_rxrpc, skb: *mut sk_buff) -> c_int {
    int rxrpc_extract_addr_from_skb(struct sockaddr_rxrpc *srx, struct sk_buff *skb)
    {
    memset(srx, 0, sizeof(*srx));
    switch (ntohs(skb.protocol)) {
    case ETH_P_IP:
    srx.transport_type = SOCK_DGRAM;
    srx.transport_len = sizeof(srx.transport.sin);
    srx.transport.sin.sin_family = AF_INET;
    srx.transport.sin.sin_port = udp_hdr(skb).source;
    srx.transport.sin.sin_addr.s_addr = ip_hdr(skb).saddr;
    return 0;

    case ETH_P_IPV6:
    srx.transport_type = SOCK_DGRAM;
    srx.transport_len = sizeof(srx.transport.sin6);
    srx.transport.sin6.sin6_family = AF_INET6;
    srx.transport.sin6.sin6_port = udp_hdr(skb).source;
    srx.transport.sin6.sin6_addr = ipv6_hdr(skb).saddr;
    return 0;

    default:
    pr_warn_ratelimited("AF_RXRPC: Unknown eth protocol %u\n",
    ntohs(skb.protocol));
    return -EAFNOSUPPORT;
    }
    }
