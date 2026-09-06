//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/krb5.h
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
// Kerberos 5 crypto
//
// Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Per Kerberos v5 protocol spec crypto types from the wire.  These get mapped
// to linux kernel crypto routines.
//
pub const KRB5_ENCTYPE_NULL: c_uint = 0x0000;
pub const KRB5_ENCTYPE_DES_CBC_CRC: c_uint = 0x0001	/* DES cbc mode with CRC-32 */;
pub const KRB5_ENCTYPE_DES_CBC_MD4: c_uint = 0x0002	/* DES cbc mode with RSA-MD4 */;
pub const KRB5_ENCTYPE_DES_CBC_MD5: c_uint = 0x0003	/* DES cbc mode with RSA-MD5 */;
pub const KRB5_ENCTYPE_DES_CBC_RAW: c_uint = 0x0004	/* DES cbc mode raw */;
// XXX deprecated?
pub const KRB5_ENCTYPE_DES3_CBC_SHA: c_uint = 0x0005	/* DES-3 cbc mode with NIST-SHA */;
pub const KRB5_ENCTYPE_DES3_CBC_RAW: c_uint = 0x0006	/* DES-3 cbc mode raw */;
pub const KRB5_ENCTYPE_DES_HMAC_SHA1: c_uint = 0x0008;
pub const KRB5_ENCTYPE_DES3_CBC_SHA1: c_uint = 0x0010;
pub const KRB5_ENCTYPE_AES128_CTS_HMAC_SHA1_96: c_uint = 0x0011;
pub const KRB5_ENCTYPE_AES256_CTS_HMAC_SHA1_96: c_uint = 0x0012;
pub const KRB5_ENCTYPE_AES128_CTS_HMAC_SHA256_128: c_uint = 0x0013;
pub const KRB5_ENCTYPE_AES256_CTS_HMAC_SHA384_192: c_uint = 0x0014;
pub const KRB5_ENCTYPE_ARCFOUR_HMAC: c_uint = 0x0017;
pub const KRB5_ENCTYPE_ARCFOUR_HMAC_EXP: c_uint = 0x0018;
pub const KRB5_ENCTYPE_CAMELLIA128_CTS_CMAC: c_uint = 0x0019;
pub const KRB5_ENCTYPE_CAMELLIA256_CTS_CMAC: c_uint = 0x001a;
pub const KRB5_ENCTYPE_UNKNOWN: c_uint = 0x01ff;
pub const KRB5_CKSUMTYPE_CRC32: c_uint = 0x0001;
pub const KRB5_CKSUMTYPE_RSA_MD4: c_uint = 0x0002;
pub const KRB5_CKSUMTYPE_RSA_MD4_DES: c_uint = 0x0003;
pub const KRB5_CKSUMTYPE_DESCBC: c_uint = 0x0004;
pub const KRB5_CKSUMTYPE_RSA_MD5: c_uint = 0x0007;
pub const KRB5_CKSUMTYPE_RSA_MD5_DES: c_uint = 0x0008;
pub const KRB5_CKSUMTYPE_NIST_SHA: c_uint = 0x0009;
pub const KRB5_CKSUMTYPE_HMAC_SHA1_DES3: c_uint = 0x000c;
pub const KRB5_CKSUMTYPE_HMAC_SHA1_96_AES128: c_uint = 0x000f;
pub const KRB5_CKSUMTYPE_HMAC_SHA1_96_AES256: c_uint = 0x0010;
pub const KRB5_CKSUMTYPE_CMAC_CAMELLIA128: c_uint = 0x0011;
pub const KRB5_CKSUMTYPE_CMAC_CAMELLIA256: c_uint = 0x0012;
pub const KRB5_CKSUMTYPE_HMAC_SHA256_128_AES128: c_uint = 0x0013;
pub const KRB5_CKSUMTYPE_HMAC_SHA384_192_AES256: c_uint = 0x0014;

//
// Constants used for key derivation
//
// from rfc3961

//
// Standard Kerberos error codes.
//

//
// Mode of operation.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum krb5_crypto_mode {
    KRB5_CHECKSUM_MODE,	/* Checksum only */
    KRB5_ENCRYPT_MODE,	/* Fully encrypted, possibly with integrity checksum */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5_buffer {
    pub len: c_uint,
    pub data: *mut c_void,
}

//
// Kerberos encoding type definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5_enctype {
    pub /: *mut *mut int etype; / Encryption (key) type,
    pub /: *mut *mut int ctype; / Checksum type,
    pub /: *const *const *const char name; / "Friendly" name,
    pub /: *const *const *const char encrypt_name; / Crypto encrypt+checksum name,
    pub /: *const *const *const char cksum_name; / Crypto checksum name,
    pub /: *const *const *const char hash_name; / Crypto hash name,
    pub /: *const *const *const char derivation_enc; / Cipher used in key derivation,
    pub /: *mut *mut u16 block_len; / Length of encryption block,
    pub /: *mut *mut u16 conf_len; / Length of confounder (normally == block_len),
    pub /: *mut *mut u16 cksum_len; / Length of checksum,
    pub /: *mut *mut u16 key_bytes; / Length of raw key, in bytes,
    pub /: *mut *mut u16 key_len; / Length of final key, in bytes,
    pub /: *mut *mut u16 hash_len; / Length of hash in bytes,
    pub /: *mut *mut u16 prf_len; / Length of PRF() result in bytes,
    pub /: *mut *mut u16 Kc_len; / Length of Kc in bytes,
    pub /: *mut *mut u16 Ke_len; / Length of Ke in bytes,
    pub /: *mut *mut u16 Ki_len; / Length of Ki in bytes,
    pub /: *mut *mut bool keyed_cksum; / T if a keyed cksum,
    pub profile: *const krb5_crypto_profile,
    pub /: *mut *mut *mut krb5_buffer out); / complete key generation,
}

//
// krb5_api.c
//
// krb5_kdf.c
//
