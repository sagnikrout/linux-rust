//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-verity.h
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
// Copyright (C) 2012 Red Hat, Inc.
// Copyright (C) 2015 Google, Inc.
//
// Author: Mikulas Patocka <mpatocka@redhat.com>
//
// Based on Chromium dm-verity driver (C) 2011 The Chromium OS Authors
//

pub const DM_VERITY_MAX_LEVELS: c_int = 63;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum verity_mode {
    DM_VERITY_MODE_EIO,
    DM_VERITY_MODE_LOGGING,
    DM_VERITY_MODE_RESTART,
    DM_VERITY_MODE_PANIC
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum verity_block_type {
    DM_VERITY_BLOCK_TYPE_DATA,
    DM_VERITY_BLOCK_TYPE_METADATA
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_verity {
    pub data_dev: *mut dm_dev,
    pub hash_dev: *mut dm_dev,
    pub ti: *mut dm_target,
    pub bufio: *mut dm_bufio_client,
    pub alg_name: *mut c_char,
    pub shash_tfm: *mut crypto_shash,
    pub /: *mut *mut *mut u8 root_digest; / digest of the root block,
    pub /: *mut *mut *mut u8 salt; / salt: its size is salt_size,
    pub /: *mut *mut *mut sha256_ctx sha256; / for use_sha256_lib=1,
    pub /: *mut *mut *mut u8 shash; / for use_sha256_lib=0,
    pub /: *mut *mut } initial_hashstate; / salted initial state, if version >= 1,
    pub /: *mut *mut *mut u8 zero_digest; / digest for a zero block,

    pub /: *mut *mut *mut u8 root_digest_sig; / signature of the root digest,
    pub /: *mut *mut unsigned int sig_size; / root digest signature size,

    pub salt_size: c_uint,
    pub /: *mut *mut sector_t hash_start; / index of first hash block on hash_dev,
    pub /: *mut *mut sector_t hash_end; / 1 + index of last hash block on hash dev,
    pub /: *mut *mut sector_t data_blocks; / the number of data blocks,
    pub /: *mut *mut unsigned char data_dev_block_bits; / log2(data blocksize),
    pub /: *mut *mut unsigned char hash_dev_block_bits; / log2(hash blocksize),
    pub /: *mut *mut unsigned char hash_per_block_bits; / log2(hashes in hash block),
    pub /: *mut *mut unsigned char levels; / the number of tree levels,
    pub version: c_uchar,
    pub /: *mut *mut bool hash_failed:1; / set if hash of any block failed,
    pub /: *mut *mut bool use_bh_wq:1; / try to verify in BH wq before normal work-queue,
    pub /: *mut *mut bool use_sha256_lib:1; / use SHA-256 library instead of generic crypto API,
    pub /: *mut *mut bool use_sha256_finup_2x:1; / use interleaved hashing optimization,
    pub /: *mut *mut unsigned int digest_size; / digest size for the current hash algorithm,
    pub /: *mut *mut verity_mode mode; / mode for handling verification errors,
    pub /: *mut *mut verity_mode error_mode;/ mode for handling I/O errors,
    pub /: *mut *mut atomic_t corrupted_errs;/ Number of errors for corrupted blocks,
    pub verify_wq: *mut workqueue_struct,
// starting blocks for each tree level. 0 is the lowest level.
    pub hash_level_block: [sector_t; DM_VERITY_MAX_LEVELS],
    pub /: *mut *mut *mut dm_verity_fec fec; / forward error correction,
    pub /: *mut *mut *mut unsigned long validated_blocks; / bitset blocks validated,
    pub /: *mut *mut *mut char signature_key_desc; / signature keyring reference,
    pub io: *mut dm_io_client,
    pub recheck_pool: mempool_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pending_block {
    pub data: *mut c_void,
    pub blkno: sector_t,
    pub want_digest: [u8; HASH_MAX_DIGESTSIZE],
    pub real_digest: [u8; HASH_MAX_DIGESTSIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_verity_io {
    pub v: *mut dm_verity,
// original value of bio->bi_end_io
    pub orig_bi_end_io: *mut bio_end_io_t,
    pub iter: bvec_iter,
    pub block: sector_t,
    pub n_blocks: c_uint,
    pub in_bh: bool,
    pub had_mismatch: bool,

    pub fec_io: *mut dm_verity_fec_io,

    pub work: work_struct,
    pub tmp_digest: [u8; HASH_MAX_DIGESTSIZE],
//
// This is the queue of data blocks that are pending verification.  When
// the crypto layer supports interleaved hashing, we allow multiple
// blocks to be queued up in order to utilize it.  This can improve
// performance significantly vs. sequential hashing of each block.
//
    pub num_pending: c_int,
    pub pending_blocks: [pending_block; 2],
//
// Temporary space for hashing.  Either sha256 or shash is used,
// depending on the value of use_sha256_lib.  If shash is used,
// then this field is variable-length, with total size
// sizeof(struct shash_desc) + crypto_shash_descsize(shash_tfm).
// For this reason, this field must be the end of the struct.
//
    pub sha256: sha256_ctx,
    pub shash: shash_desc,
    pub hash_ctx: },
}

extern "C" {
    pub fn dm_is_verity_target(ti: *mut dm_target) -> bool;
}
extern "C" {
    pub fn dm_verity_get_mode(ti: *mut dm_target) -> c_int;
}
