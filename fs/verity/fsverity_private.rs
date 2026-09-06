//! Automatically rewritten from C Header to Rust Module
//! Source: fs/verity/fsverity_private.h
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


// SPDX-License-Identifier: GPL-2.0
//
// fs-verity: read-only file-based authenticity protection
//
// Copyright 2019 Google LLC
//

//
// Implementation limit: maximum depth of the Merkle tree.  For now 8 is plenty;
// it's enough for over U64_MAX bytes of data using SHA-256 and 4K blocks.
//
pub const FS_VERITY_MAX_LEVELS: c_int = 8;
// A hash algorithm supported by fs-verity
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsverity_hash_alg {
    pub /: *const *const *const char name; / crypto API name, e.g. sha256,
    pub /: *mut *mut unsigned int digest_size; / digest size in bytes, e.g. 32 for SHA-256,
    pub /: *mut *mut unsigned int block_size; / block size in bytes, e.g. 64 for SHA-256,
//
// The HASH_ALGO_* constant for this algorithm.  This is different from
// FS_VERITY_HASH_ALG_*, which uses a different numbering scheme.
//
    pub algo_id: hash_algo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fsverity_hash_ctx {
    pub sha256: sha256_ctx,
    pub sha512: sha512_ctx,
}

// Merkle tree parameters: hash algorithm, initial hash state, and topology
#[repr(C)]
#[derive(Copy, Clone)]
pub struct merkle_tree_params {
    pub /: *const *const *const fsverity_hash_alg hash_alg; / the hash algorithm,
// initial hash state if salted, NULL if unsalted
    pub hashstate: *const fsverity_hash_ctx,
    pub /: *mut *mut unsigned int digest_size; / same as hash_alg->digest_size,
    pub /: *mut *mut unsigned int block_size; / size of data and tree blocks,
    pub /: *mut *mut unsigned int hashes_per_block; / number of hashes per tree block,
    pub /: *mut *mut unsigned int blocks_per_page; / PAGE_SIZE / block_size,
    pub /: *mut *mut u8 log_digestsize; / log2(digest_size),
    pub /: *mut *mut u8 log_blocksize; / log2(block_size),
    pub /: *mut *mut u8 log_arity; / log2(hashes_per_block),
    pub /: *mut *mut u8 log_blocks_per_page; / log2(blocks_per_page),
    pub /: *mut *mut unsigned int num_levels; / number of levels in Merkle tree,
    pub /: *mut *mut u64 tree_size; / Merkle tree size in bytes,
    pub /: *mut *mut unsigned long tree_pages; / Merkle tree size in pages,
// the hash of an all-zeroes block
    pub zero_digest: [u8; FS_VERITY_MAX_DIGEST_SIZE],
//
// Starting block index for each tree level, ordered from leaf level (0)
// to root level ('num_levels - 1')
//
    pub level_start: [c_ulong; FS_VERITY_MAX_LEVELS],
}

//
// fsverity_info - cached verity metadata for an inode
//
// When a verity file is first opened, an instance of this struct is allocated
// and a pointer to it is stored in the global hash table, indexed by the inode
// pointer value.  It remains alive until the inode is evicted.  It caches
// information about the Merkle tree that's needed to efficiently verify data
// read from the file.  It also caches the file digest.  The Merkle tree pages
// themselves are not cached here, but the filesystem may cache them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsverity_info {
    pub rhash_head: rhash_head,
    pub tree_params: merkle_tree_params,
    pub root_hash: [u8; FS_VERITY_MAX_DIGEST_SIZE],
    pub file_digest: [u8; FS_VERITY_MAX_DIGEST_SIZE],
    pub inode: *mut inode,
    pub hash_block_verified: *mut c_ulong,
}

// hash_algs.c
extern "C" {
    pub fn fsverity_check_hash_algs() -> void __init;
}
// init.c

// measure.c

extern "C" {
    pub fn fsverity_init_bpf() -> void __init;
}

// open.c
extern "C" {
    pub fn fsverity_set_info(vi: *mut fsverity_info) -> c_int;
}
extern "C" {
    pub fn fsverity_free_info(vi: *mut fsverity_info);
}
extern "C" {
    pub fn fsverity_remove_info(vi: *mut fsverity_info);
}
extern "C" {
    pub fn fsverity_init_info_cache() -> void __init;
}
// signature.c

extern "C" {
    pub fn fsverity_init_signature() -> void __init;
}

// verify.c
extern "C" {
    pub fn fsverity_init_workqueue() -> void __init;
}

