//! Automatically rewritten from C to Rust
//! Source: net/rxrpc/local_event.c
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
// AF_RXRPC local endpoint management
//
// Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

    static char rxrpc_version_string[65]; // "linux-" UTS_RELEASE " AF_RXRPC";
//
// Generate the VERSION packet string.
//
#[no_mangle]
pub unsafe extern "C" fn rxrpc_gen_version_string() {
    void rxrpc_gen_version_string(void)
    {
    snprintf(rxrpc_version_string, sizeof(rxrpc_version_string),
    "linux-%.49s AF_RXRPC", UTS_RELEASE);
    }
//
// Reply to a version request
//
    void rxrpc_send_version_request(struct rxrpc_local *local,
    struct rxrpc_host_header *hdr,
    struct sk_buff *skb)
    {
    struct rxrpc_wire_header whdr;
    struct rxrpc_skb_priv *sp = rxrpc_skb(skb);
    struct sockaddr_rxrpc srx;
    struct msghdr msg;
    struct kvec iov[2];
    size_t len;
    int ret;
    _enter("");
    if (rxrpc_extract_addr_from_skb(&srx, skb) < 0)
    return;
    msg.msg_name	= &srx.transport;
    msg.msg_namelen	= srx.transport_len;
    msg.msg_control	= core::ptr::null_mut();
    msg.msg_controllen = 0;
    msg.msg_flags	= 0;
    whdr.epoch	= htonl(sp.hdr.epoch);
    whdr.cid	= htonl(sp.hdr.cid);
    whdr.callNumber	= htonl(sp.hdr.callNumber);
    whdr.seq	= 0;
    whdr.serial	= 0;
    whdr.type	= RXRPC_PACKET_TYPE_VERSION;
    whdr.flags	= RXRPC_LAST_PACKET | (~hdr.flags & RXRPC_CLIENT_INITIATED);
    whdr.userStatus	= 0;
    whdr.securityIndex = 0;
    whdr._rsvd	= 0;
    whdr.serviceId	= htons(sp.hdr.serviceId);
    iov[0].iov_base	= &whdr;
    iov[0].iov_len	= sizeof(whdr);
    iov[1].iov_base	= (char *)rxrpc_version_string;
    iov[1].iov_len	= sizeof(rxrpc_version_string);
    len = iov[0].iov_len + iov[1].iov_len;
    ret = kernel_sendmsg(local.socket, &msg, iov, 2, len);
    if (ret < 0)
    trace_rxrpc_tx_fail(local.debug_id, 0, ret,
    rxrpc_tx_point_version_reply);
    else
    trace_rxrpc_tx_packet(local.debug_id, &whdr,
    rxrpc_tx_point_version_reply);
    _leave("");
    }
