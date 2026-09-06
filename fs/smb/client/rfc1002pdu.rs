//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/rfc1002pdu.h
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


// SPDX-License-Identifier: LGPL-2.1
//
// Protocol Data Unit definitions for RFC 1001/1002 support
//
// Copyright (c) International Business Machines  Corp., 2004
// Author(s): Steve French (sfrench@us.ibm.com)
//
// NB: unlike smb/cifs packets, the RFC1002 structures are big endian
// RFC 1002 session packet types
pub const RFC1002_SESSION_MESSAGE: c_uint = 0x00;
pub const RFC1002_SESSION_REQUEST: c_uint = 0x81;
pub const RFC1002_POSITIVE_SESSION_RESPONSE: c_uint = 0x82;
pub const RFC1002_NEGATIVE_SESSION_RESPONSE: c_uint = 0x83;
pub const RFC1002_RETARGET_SESSION_RESPONSE: c_uint = 0x84;
pub const RFC1002_SESSION_KEEP_ALIVE: c_uint = 0x85;
// RFC 1002 flags (only one defined
pub const RFC1002_LENGTH_EXTEND: c_uint = 0x80 /* high order bit of length (ie +64K) */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfc1002_session_packet {
    pub type: __u8,
    pub flags: __u8,
    pub length: __be16,
    pub called_len: __u8,
    pub called_name: [__u8; 32],
    pub /: *mut *mut __u8 scope1; / null,
    pub calling_len: __u8,
    pub calling_name: [__u8; 32],
    pub /: *mut *mut __u8 scope2; / null,
    pub session_req: } __packed,
    pub retarget_ip_addr: __be32,
    pub port: __be16,
    pub retarget_resp: } __packed,
    pub neg_ses_resp_error_code: __u8,
// POSITIVE_SESSION_RESPONSE packet does not include trailer.
    pub trailer: } __packed,
    pub __packed: },
// Negative Session Response error codes
pub const RFC1002_NOT_LISTENING_CALLED: c_uint = 0x80 /* not listening on called name */;
pub const RFC1002_NOT_LISTENING_CALLING: c_uint = 0x81 /* not listening on calling name */;
pub const RFC1002_NOT_PRESENT: c_uint = 0x82 /* called name not present */;
pub const RFC1002_INSUFFICIENT_RESOURCE: c_uint = 0x83;
pub const RFC1002_UNSPECIFIED_ERROR: c_uint = 0x8F;
// RFC 1002 Datagram service packets are not defined here as they
