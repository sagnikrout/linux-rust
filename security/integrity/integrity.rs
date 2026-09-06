//! Automatically rewritten from C Header to Rust Module
//! Source: security/integrity/integrity.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2009-2010 IBM Corporation
//
// Authors:
// Mimi Zohar <zohar@us.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum evm_ima_xattr_type {
    IMA_XATTR_DIGEST = 0x01,
    EVM_XATTR_HMAC,
    EVM_IMA_XATTR_DIGSIG,
    IMA_XATTR_DIGEST_NG,
    EVM_XATTR_PORTABLE_DIGSIG,
    IMA_VERITY_DIGSIG,
    IMA_XATTR_LAST
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evm_ima_xattr_data {
// New members must be added within the __struct_group() macro below.
    pub type: u8,
    pub data: [u8; ],
    pub __packed: },
    pub __struct_group()"): "struct member likely outside of,
// Only used in the EVM HMAC code.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evm_xattr {
    pub data: evm_ima_xattr_data_hdr,
    pub digest: [u8; SHA1_DIGEST_SIZE],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ima_digest_data {
// New members must be added within the __struct_group() macro below.
    pub algo: u8,
    pub length: u8,
    pub unused: u8,
    pub type: u8,
    pub sha1: },
    pub type: u8,
    pub algo: u8,
    pub ng: },
    pub data: [u8; 2],
    pub xattr: },
    pub digest: [u8; ],
    pub __packed: },
    pub __struct_group()"): "struct member likely outside of,
//
// Instead of wrapping the ima_digest_data struct inside a local structure
// with the maximum hash size, define ima_max_digest_data struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ima_max_digest_data {
    pub hdr: ima_digest_data_hdr,
    pub digest: [u8; HASH_MAX_DIGESTSIZE],
    pub __packed: },
//
// signature header format v2 - for using with asymmetric keys
//
// The signature_v2_hdr struct includes a signature format version
// to simplify defining new signature formats.
//
// signature format:
// version 2: regular file data hash based signature
// version 3: struct ima_file_id data based signature
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct signature_v2_hdr {
    pub /: *mut *mut uint8_t type; / xattr type,
    pub /: *mut *mut uint8_t version; / signature format version,
    pub /: *mut *mut uint8_t hash_algo; / Digest algorithm [enum hash_algo],
    pub /: *mut *mut __be32 keyid; / IMA key identifier - not X509/PGP specific,
    pub /: *mut *mut __be16 sig_size; / signature size,
    pub /: *mut *mut uint8_t sig[]; / signature payload,
    pub __packed: },
//
// IMA signature version 3 disambiguates the data that is signed, by
// indirectly signing the hash of the ima_file_id structure data,
// containing either the fsverity_descriptor struct digest or, in the
// future, the regular IMA file hash.
//
// (The hash of the ima_file_id structure is only of the portion used.)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ima_file_id {
    pub /: *mut *mut __u8 hash_type; / xattr type [enum evm_ima_xattr_type],
    pub /: *mut *mut __u8 hash_algorithm; / Digest algorithm [enum hash_algo],
    pub hash: [__u8; HASH_MAX_DIGESTSIZE],
    pub __packed: },
    pub count): *mut *mut void addr, unsigned long,
    pub integrity_fs_init(void): int __init,
    pub integrity_fs_fini(void): void __init,
pub const INTEGRITY_KEYRING_EVM: c_int = 0;
pub const INTEGRITY_KEYRING_IMA: c_int = 1;
pub const INTEGRITY_KEYRING_PLATFORM: c_int = 2;
pub const INTEGRITY_KEYRING_MACHINE: c_int = 3;
pub const INTEGRITY_KEYRING_MAX: c_int = 4;
    pub integrity_dir: *mut extern struct dentry,
    pub modsig: struct,

    pub algo): *const *const char digest, int digestlen, u8,
    pub modsig): *const int integrity_modsig_verify(unsigned int id, struct modsig,
    pub id): int __init integrity_init_keyring(unsigned int,
    pub path): *const int __init integrity_load_x509(unsigned int id, char,
    pub perm): *const *const void data, size_t len, key_perm_t,

    pub -EOPNOTSUPP: return,
    pub -EOPNOTSUPP: return,
    pub 0: return,
    pub 0: return,

    pub datalen): *const *const int siglen, char data, int,
    pub algo): *const *const int siglen, char data, int datalen, u8,

    pub -EOPNOTSUPP: return,
    pub -EOPNOTSUPP: return,

    pub modsig): *const *const int ima_modsig_verify(struct key keyring, struct modsig,

    pub -EOPNOTSUPP: return,

    pub ima_load_x509(void): void __init,

    pub evm_load_x509(void): void __init,

// declarations
    pub info): *const *const char cause, int result, int,
    pub errno): c_int,
    pub type): return audit_log_start(ctx, gfp_mask,,

    pub NULL: return,

    pub len): usize,

    pub len): *const *const *const void __init add_to_machine_keyring(char source, void data, size_t,
    pub imputed_trust_enabled(void): bool __init,

    pub false: return,
