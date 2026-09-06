//! Automatically rewritten from C Header to Rust Module
//! Source: crypto/asymmetric_keys/pkcs7_parser.h
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
// PKCS#7 crypto data parser internal definitions
//
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkcs7_signed_info {
    pub next: *mut pkcs7_signed_info,
    pub /: *mut *mut *mut x509_certificate signer; / Signing certificate (in msg->certs),
    pub index: unsigned,
    pub /: *mut *mut bool unsupported_crypto; / T if not usable due to missing crypto,
    pub blacklisted: bool,
// Message digest - the digest of the Content Data (or NULL)
    pub msgdigest: *const c_void,
    pub msgdigest_len: unsigned,
// Authenticated Attribute data (or NULL)
    pub authattrs_len: unsigned,
    pub authattrs: *const c_void,
    pub aa_set: c_ulong,
pub const sinfo_has_content_type: c_int = 0;
pub const sinfo_has_signing_time: c_int = 1;
pub const sinfo_has_message_digest: c_int = 2;
pub const sinfo_has_smime_caps: c_int = 3;
pub const sinfo_has_ms_opus_info: c_int = 4;
pub const sinfo_has_ms_statement_type: c_int = 5;
    pub signing_time: time64_t,
// Message signature.
//
// This contains the generated digest of _either_ the Content Data or
// the Authenticated Attributes [RFC2315 9.3].  If the latter, one of
// the attributes contains the digest of the Content Data within it.
//
// This also contains the issuing cert serial number and issuer's name
// [PKCS#7 or CMS ver 1] or issuing cert's SKID [CMS ver 3].
//
    pub sig: *mut public_key_signature,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkcs7_message {
    pub /: *mut *mut *mut x509_certificate certs; / Certificate list,
    pub /: *mut *mut *mut x509_certificate crl; / Revocation list,
    pub signed_infos: *mut pkcs7_signed_info,
    pub /: *mut *mut u8 version; / Version of cert (1 -> PKCS#7 or CMS; 3 -> CMS),
    pub /: *mut *mut bool have_authattrs; / T if have authattrs,

    pub /: *mut *mut bool authattrs_rej_waivable; / T if authatts rejection can be waived,

// Content Data (or NULL)
    pub /: *mut *mut OID data_type; / Type of Data,
    pub /: *mut *mut size_t data_len; / Length of Data,
    pub /: *mut *mut size_t data_hdrlen; / Length of Data ASN.1 header,
    pub /: *const *const *const void data; / Content Data (or 0),
}
