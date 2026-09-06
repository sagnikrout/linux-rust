//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-verity-fec.h
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
//
// Copyright (C) 2015 Google, Inc.
//
// Author: Sami Tolvanen <samitolvanen@google.com>
//

// Reed-Solomon(n, k) parameters
pub const DM_VERITY_FEC_RS_N: c_int = 255;

// buffers for deinterleaving and decoding

// configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_verity_fec {
    pub /: *mut *mut *mut dm_dev dev; / parity data device,
    pub /: *mut *mut *mut dm_bufio_client data_bufio; / for data dev access,
    pub /: *mut *mut *mut dm_bufio_client bufio; / for parity data access,
    pub /: *mut *mut size_t block_size; / size of data, hash, and parity blocks in bytes,
    pub /: *mut *mut sector_t start; / parity data start in blocks,
    pub /: *mut *mut sector_t blocks; / number of blocks covered,
    pub /: *mut *mut sector_t region_blocks; / blocks per region: ceil(blocks / rs_k),
    pub /: *mut *mut sector_t hash_blocks; / blocks covered after v->hash_start,
    pub /: *mut *mut unsigned char roots; / parity bytes per RS codeword, n-k of RS(n, k),
    pub /: *mut *mut unsigned char rs_k; / message bytes per RS codeword, k of RS(n, k),
    pub /: *mut *mut mempool_t fio_pool; / mempool for dm_verity_fec_io,
    pub /: *mut *mut mempool_t rs_pool; / mempool for fio->rs,
    pub /: *mut *mut mempool_t prealloc_pool; / mempool for preallocated buffers,
    pub /: *mut *mut mempool_t output_pool; / mempool for output,
    pub /: *mut *mut *mut kmem_cache cache; / cache for buffers,
    pub /: *mut *mut atomic64_t corrected; / corrected errors,
}

// per-bio data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_verity_fec_io {
    pub /: *mut *mut *mut rs_control rs; / Reed-Solomon state,
    pub /: *mut *mut int erasures[DM_VERITY_FEC_MAX_ROOTS]; / erasures for decode_rs8,
    pub /: *mut *mut *mut u8 output; / buffer for corrected output,
    pub /: *mut *mut unsigned int level; / recursion level,
    pub /: *mut *mut unsigned int nbufs; / number of buffers allocated,
//
// Buffers for deinterleaving RS codewords.  Each buffer has space for
// the message bytes of (1 << DM_VERITY_FEC_BUF_RS_BITS) RS codewords.
// The array length is fec_max_nbufs(v), and we try to allocate that
// many buffers.  However, in low-memory situations we may be unable to
// allocate all buffers.  'nbufs' holds the number actually allocated.
//
    pub bufs: [*mut u8; ],
}

// each feature parameter requires a value
pub const DM_VERITY_OPTS_FEC: c_int = 8;
// Returns true if forward error correction is enabled.
extern "C" {
    pub fn __verity_fec_finish_io(io: *mut dm_verity_io);
}
extern "C" {
    pub fn verity_is_fec_opt_arg(arg_name: *const c_char) -> bool;
}
extern "C" {
    pub fn verity_fec_dtr(v: *mut dm_verity);
}
extern "C" {
    pub fn verity_fec_ctr_alloc(v: *mut dm_verity) -> c_int;
}
extern "C" {
    pub fn verity_fec_ctr(v: *mut dm_verity) -> c_int;
}

pub const DM_VERITY_OPTS_FEC: c_int = 0;

