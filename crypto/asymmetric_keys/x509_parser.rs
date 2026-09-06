//! Automatically rewritten from C Header to Rust Module
//! Source: crypto/asymmetric_keys/x509_parser.h
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
// X.509 certificate parser internal definitions
//
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x509_certificate {
    pub next: *mut x509_certificate,
    pub /: *mut *mut *mut x509_certificate signer; / Certificate that signed this one,
    pub /: *mut *mut *mut public_key pub; / Public key details,
    pub /: *mut *mut *mut public_key_signature sig; / Signature parameters,
    pub /: *mut *mut u8 sha256[SHA256_DIGEST_SIZE]; / Hash for blacklist purposes,
    pub /: *mut *mut *mut char issuer; / Name of certificate issuer,
    pub /: *mut *mut *mut char subject; / Name of certificate subject,
    pub /: *mut *mut *mut asymmetric_key_id id; / Issuer + Serial number,
    pub /: *mut *mut *mut asymmetric_key_id skid; / Subject + subjectKeyId (optional),
    pub valid_from: time64_t,
    pub valid_to: time64_t,
    pub /: *const *const *const void tbs; / Signed data,
    pub /: *mut *mut unsigned tbs_size; / Size of signed data,
    pub /: *mut *mut unsigned raw_sig_size; / Size of signature,
    pub /: *const *const *const void raw_sig; / Signature data,
    pub /: *const *const *const void raw_serial; / Raw serial number in ASN.1,
    pub raw_serial_size: unsigned,
    pub raw_issuer_size: unsigned,
    pub /: *const *const *const void raw_issuer; / Raw issuer name in ASN.1,
    pub /: *const *const *const void raw_subject; / Raw subject name in ASN.1,
    pub raw_subject_size: unsigned,
    pub raw_skid_size: unsigned,
    pub /: *const *const *const void raw_skid; / Raw subjectKeyId in ASN.1,
    pub index: unsigned,
    pub /: *mut *mut bool seen; / Infinite recursion prevention,
    pub verified: bool,
    pub /: *mut *mut bool self_signed; / T if self-signed (check unsupported_sig too),
    pub /: *mut *mut bool unsupported_sig; / T if signature uses unsupported crypto,
    pub blacklisted: bool,
}

//
// x509_cert_parser.c
//
extern "C" {
    pub fn x509_free_certificate(cert: *mut x509_certificate);
}
//
// x509_public_key.c
//
extern "C" {
    pub fn x509_get_sig_params(cert: *mut x509_certificate) -> c_int;
}
extern "C" {
    pub fn x509_check_for_self_signed(cert: *mut x509_certificate) -> c_int;
}
