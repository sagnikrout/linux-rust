//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/fsverity.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// fs-verity user API
//
// These ioctls can be used on filesystems that support fs-verity.  See the
// "User API" section of Documentation/filesystems/fsverity.rst.
//
// Copyright 2019 Google LLC
//

pub const FS_VERITY_HASH_ALG_SHA256: c_int = 1;
pub const FS_VERITY_HASH_ALG_SHA512: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsverity_enable_arg {
    pub version: __u32,
    pub hash_algorithm: __u32,
    pub block_size: __u32,
    pub salt_size: __u32,
    pub salt_ptr: __u64,
    pub sig_size: __u32,
    pub __reserved1: __u32,
    pub sig_ptr: __u64,
    pub __reserved2: [__u64; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsverity_digest {
    pub digest_algorithm: __u16,
    pub /: *mut *mut __u16 digest_size; / input/output,
    pub digest: [__u8; ],
}

//
// Struct containing a file's Merkle tree properties.  The fs-verity file digest
// is the hash of this struct.  A userspace program needs this struct only if it
// needs to compute fs-verity file digests itself, e.g. in order to sign files.
// It isn't needed just to enable fs-verity on a file.
//
// Note: when computing the file digest, 'sig_size' and 'signature' must be left
// zero and empty, respectively.  These fields are present only because some
// filesystems reuse this struct as part of their on-disk format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsverity_descriptor {
    pub /: *mut *mut __u8 version; / must be 1,
    pub /: *mut *mut __u8 hash_algorithm; / Merkle tree hash algorithm,
    pub /: *mut *mut __u8 log_blocksize; / log2 of size of data and tree blocks,
    pub /: *mut *mut __u8 salt_size; / size of salt in bytes; 0 if none,

    pub sig_size: __le32,

    pub /: *mut *mut __le32 __reserved_0x04; / must be 0,

    pub /: *mut *mut __le64 data_size; / size of file the Merkle tree is built over,
    pub /: *mut *mut __u8 root_hash[64]; / Merkle tree root hash,
    pub /: *mut *mut __u8 salt[32]; / salt prepended to each hashed block,
    pub /: *mut *mut __u8 __reserved[144]; / must be 0's,
    pub signature: [__u8; ],
}

//
// Format in which fs-verity file digests are signed in built-in signatures.
// This is the same as 'struct fsverity_digest', except here some magic bytes
// are prepended to provide some context about what is being signed in case the
// same key is used for non-fsverity purposes, and here the fields have fixed
// endianness.
//
// This struct is specific to the built-in signature verification support, which
// is optional.  fs-verity users may also verify signatures in userspace, in
// which case userspace is responsible for deciding on what bytes are signed.
// This struct may still be used, but it doesn't have to be.  For example,
// userspace could instead use a string like "sha256:$digest_as_hex_string".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsverity_formatted_digest {
    pub /: *mut *mut char magic[8]; / must be "FSVerity",
    pub digest_algorithm: __le16,
    pub digest_size: __le16,
    pub digest: [__u8; ],
}

pub const FS_VERITY_METADATA_TYPE_MERKLE_TREE: c_int = 1;
pub const FS_VERITY_METADATA_TYPE_DESCRIPTOR: c_int = 2;
pub const FS_VERITY_METADATA_TYPE_SIGNATURE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsverity_read_metadata_arg {
    pub metadata_type: __u64,
    pub offset: __u64,
    pub length: __u64,
    pub buf_ptr: __u64,
    pub __reserved: __u64,
}

