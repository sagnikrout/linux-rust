//! Automatically rewritten from C Header to Rust Module
//! Source: net/sunrpc/auth_gss/gss_krb5_internal.h
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


// SPDX-License-Identifier: GPL-2.0 or BSD-3-Clause
//
// SunRPC GSS Kerberos 5 mechanism internal definitions
//
// Copyright (c) 2022 Oracle and/or its affiliates.
//

// krb5_ctx flags definitions
pub const KRB5_CTX_FLAG_INITIATOR: c_uint = 0x00000001;
pub const KRB5_CTX_FLAG_ACCEPTOR_SUBKEY: c_uint = 0x00000004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5_ctx {
    pub /: *mut *mut int initiate; / 1 = initiating, 0 = accepting,
    pub enctype: u32,
    pub flags: u32,
    pub /: *const *const *const krb5_enctype krb5e; / crypto/krb5 enctype,
    pub initiator_enc_aead: *mut crypto_aead,
    pub acceptor_enc_aead: *mut crypto_aead,
    pub initiator_sign_shash: *mut crypto_shash,
    pub acceptor_sign_shash: *mut crypto_shash,
    pub /: *mut *mut u8 Ksess[GSS_KRB5_MAX_KEYLEN]; / session key,
    pub seq_send64: core::sync::atomic::AtomicI64,
    pub endtime: time64_t,
    pub mech_used: xdr_netobj,
}

//
// GSS Kerberos 5 mechanism Per-Message calls.
//
// Implementation internal functions
//
extern "C" {
    pub fn gss_krb5_errno_to_status(err: c_int) -> u32;
}
