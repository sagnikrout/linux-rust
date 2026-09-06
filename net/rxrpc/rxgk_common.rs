//! Automatically rewritten from C Header to Rust Module
//! Source: net/rxrpc/rxgk_common.h
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
// Common bits for GSSAPI-based RxRPC security.
//
// Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Per-key number context.  This is replaced when the connection is rekeyed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxgk_context {
    pub usage: refcount_t,
    pub /: *mut *mut unsigned int key_number; / Rekeying number (goes in the rx header),
    pub flags: c_ulong,

    pub /: *mut *mut unsigned long expiry; / Expiration time of this key,
    pub /: *mut *mut long long bytes_remaining; / Remaining Tx lifetime of this key,
    pub /: *const *const *const krb5_enctype krb5; / RxGK encryption type,
    pub key: *const rxgk_key,
// We need up to 7 keys derived from the transport key, but we don't
// actually need the transport key.  Each key is derived by
// DK(TK,constant).
//
    pub /: *mut *mut *mut crypto_aead tx_enc; / Transmission key,
    pub /: *mut *mut *mut crypto_aead rx_enc; / Reception key,
    pub /: *mut *mut *mut crypto_shash tx_Kc; / Transmission checksum key,
    pub /: *mut *mut *mut crypto_shash rx_Kc; / Reception checksum key,
    pub /: *mut *mut *mut crypto_aead resp_enc; / Response packet enc key,
}

//
// rxgk_app.c
//
// rxgk_kdf.c
//
extern "C" {
    pub fn rxgk_put(gk: *mut rxgk_context);
}
//
// Apply decryption and checksumming functions a flat data buffer.  The data
// point and length are updated to reflect the actual content of the encrypted
// region.
//
// _error_code = RXGK_INCONSISTENCY;
// _data += offset;
// _len = len;
// _error_code = RXGK_SEALEDINCON;
// _error_code = RXGK_PACKETSHORT;
// _error_code = RXGK_INCONSISTENCY;
//
// Check the MIC on a flat buffer.  The data pointer and length are updated to
// reflect the actual content of the secure region.
//
// _data += offset;
// _len = len;
// _error_code = RXGK_SEALEDINCON;
// _error_code = RXGK_PACKETSHORT;
// _error_code = RXGK_INCONSISTENCY;
