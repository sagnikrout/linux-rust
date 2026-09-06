//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/rxrpc.h
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


// SPDX-License-Identifier: GPL-2.0-or-later WITH Linux-syscall-note
// Types and definitions for AF_RXRPC.
//
// Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// RxRPC socket address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_rxrpc {
    pub /: *mut *mut __kernel_sa_family_t srx_family; / address family,
    pub /: *mut *mut __u16 srx_service; / service desired,
    pub /: *mut *mut __u16 transport_type; / type of transport socket (SOCK_DGRAM),
    pub /: *mut *mut __u16 transport_len; / length of transport address,
    pub /: *mut *mut __kernel_sa_family_t family; / transport address family,
    pub /: *mut *mut sockaddr_in sin; / IPv4 transport address,
    pub /: *mut *mut sockaddr_in6 sin6; / IPv6 transport address,
    pub transport: },
}

//
// RxRPC socket options
//

//
// RxRPC control messages
// - If neither abort or accept are specified, the message is a data message.
// - terminal messages mean that a user call ID tag can be recycled
// - C/S/- indicate whether these are applicable to client, server or both
// - s/r/- indicate whether these are applicable to sendmsg() and/or recvmsg()
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_cmsg_type {
    RXRPC_USER_CALL_ID	= 1,	/* -sr: User call ID specifier */
    RXRPC_ABORT		= 2,	/* -sr: Abort request / notification [terminal] */
    RXRPC_ACK		= 3,	/* S-r: RPC op final ACK received [terminal] */
    RXRPC_NET_ERROR		= 5,	/* --r: Network error received [terminal] */
    RXRPC_BUSY		= 6,	/* C-r: Server busy received [terminal] */
    RXRPC_LOCAL_ERROR	= 7,	/* --r: Local error generated [terminal] */
    RXRPC_NEW_CALL		= 8,	/* S-r: New incoming call notification */
    RXRPC_EXCLUSIVE_CALL	= 10,	/* Cs-: Call should be on exclusive connection */
    RXRPC_UPGRADE_SERVICE	= 11,	/* Cs-: Request service upgrade for client call */
    RXRPC_TX_LENGTH		= 12,	/* -s-: Total length of Tx data */
    RXRPC_SET_CALL_TIMEOUT	= 13,	/* -s-: Set one or more call timeouts */
    RXRPC_CHARGE_ACCEPT	= 14,	/* Ss-: Charge the accept pool with a user call ID */
    RXRPC_OOB_ID		= 15,	/* -sr: OOB message ID */
    RXRPC_CHALLENGED	= 16,	/* C-r: Info on a received CHALLENGE */
    RXRPC_RESPOND		= 17,	/* Cs-: Respond to a challenge */
    RXRPC_RESPONDED		= 18,	/* S-r: Data received in RESPONSE */
    RXRPC_RESP_RXGK_APPDATA	= 19,	/* Cs-: RESPONSE: RxGK app data to include */
    RXRPC__SUPPORTED
}

//
// RxRPC security levels
//

//
// RxRPC security indices
//

//
// RxRPC-level abort codes
//

//
// (un)marshalling abort codes (rxgen)
//

//
// Rx kerberos security abort codes
// - unfortunately we have no generalised security abort codes to say things
// like "unsupported security", so we have to use these instead and hope the
// other side understands
//

//
// RxGK GSSAPI security abort codes.
//

//
// Challenge information in the RXRPC_CHALLENGED control message.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_challenge {
    pub /: *mut *mut __u16 service_id; / The service ID of the connection (may be upgraded),
    pub /: *mut *mut __u8 security_index; / The security index of the connection,
    pub /: *mut *mut __u8 pad; / Round out to a multiple of 4 bytes.,
// ... The security class gets to append extra information ...
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxgk_challenge {
    pub base: rxrpc_challenge,
    pub /: *mut *mut __u32 enctype; / Krb5 encoding type,
}
