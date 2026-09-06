//! Automatically rewritten from C Header to Rust Module
//! Source: include/keys/rxrpc-type.h
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
// RxRPC key type
//
// Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// key type for AF_RXRPC keys
//
// RxRPC key for Kerberos IV (type-2 security)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxkad_key {
    pub vice_id: u32,
    pub /: *mut *mut u32 start; / time at which ticket starts,
    pub /: *mut *mut u32 expiry; / time at which ticket expires,
    pub /: *mut *mut u32 kvno; / key version number,
    pub /: *mut *mut u8 primary_flag; / T if key for primary cell for this user,
    pub /: *mut *mut u16 ticket_len; / length of ticket[],
    pub /: *mut *mut u8 session_key[8]; / DES session key,
    pub /: *mut *mut u8 ticket[]; / the encrypted ticket,
}

//
// RxRPC key for YFS-RxGK (type-6 security)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxgk_key {
    pub /: *mut *mut s64 begintime; / Time at which the ticket starts,
    pub /: *mut *mut s64 endtime; / Time at which the ticket ends,
    pub /: *mut *mut u64 lifetime; / Maximum lifespan of a connection (seconds),
    pub /: *mut *mut u64 bytelife; / Maximum number of bytes on a connection,
    pub /: *mut *mut unsigned int enctype; / Encoding type,
    pub /: *mut *mut s8 level; / Negotiated security RXRPC_SECURITY_PLAIN/AUTH/ENCRYPT,
    pub /: *mut *mut krb5_buffer key; / Master key, K0,
    pub /: *mut *mut krb5_buffer ticket; / Ticket to be passed to server,
    pub /: *mut *mut u8 _key[]; / Key storage,
}

//
// list of tokens attached to an rxrpc key
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_key_token {
    pub /: *mut *mut u16 security_index; / RxRPC header security index,
    pub /: *mut *mut bool no_leak_key; / Don't copy the key to userspace,
    pub /: *mut *mut *mut rxrpc_key_token next; / the next token in the list,
    pub kad: *mut rxkad_key,
    pub rxgk: *mut rxgk_key,
}

//
// structure of raw payloads passed to add_key() or instantiate key
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_key_data_v1 {
    pub security_index: u16,
    pub ticket_length: u16,
    pub /: *mut *mut u32 expiry; / time_t,
    pub kvno: u32,
    pub session_key: [u8; 8],
    pub ticket: [u8; ],
}

//
// AF_RXRPC key payload derived from XDR format
// - based on openafs-1.4.10/src/auth/afs_token.xg
//

//
// Truncate a time64_t to the range from 1970 to 2106 as in the network
// protocol.
//
// Extend u32 back to time64_t using the same 1970-2106 range.
//
