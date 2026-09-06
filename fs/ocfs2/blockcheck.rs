//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/blockcheck.h
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
// blockcheck.h
//
// Checksum and ECC codes for the OCFS2 userspace library.
//
// Copyright (C) 2004, 2008 Oracle.  All rights reserved.
//
// Count errors and error correction from blockcheck.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_blockcheck_stats {
    pub b_lock: spinlock_t,
    pub /: *mut *mut u64 b_check_count; / Number of blocks we've checked,
    pub /: *mut *mut u64 b_failure_count; / Number of failed checksums,
    pub /: *mut *mut u64 b_recover_count; / Number of blocks fixed by ecc,
//
// debugfs entries, used if this is passed to
// ocfs2_blockcheck_stats_debugfs_install()
//
    pub /: *mut *mut *mut dentry b_debug_dir; / Parent of the debugfs files,
}

// High level block API
// Lower level API
// Debug Initialization
extern "C" {
    pub fn ocfs2_blockcheck_stats_debugfs_remove(stats: *mut ocfs2_blockcheck_stats);
}
//
// Hamming code functions
//
// Encoding hamming code parity bits for a buffer.
//
// This is the low level encoder function.  It can be called across
// multiple hunks just like the crc32 code.  'd' is the number of bits
// _in_this_hunk_.  nr is the bit offset of this hunk.  So, if you had
// two 512B buffers, you would do it like so:
//
// parity = ocfs2_hamming_encode(0, buf1, 512 * 8, 0);
// parity = ocfs2_hamming_encode(parity, buf2, 512 * 8, 512 * 8);
//
// If you just have one buffer, use ocfs2_hamming_encode_block().
//
// Fix a buffer with a bit error.  The 'fix' is the original parity
// xor'd with the parity calculated now.
//
// Like ocfs2_hamming_encode(), this can handle hunks.  nr is the bit
// offset of the current hunk.  If bit to be fixed is not part of the
// current hunk, this does nothing.
//
// If you only have one buffer, use ocfs2_hamming_fix_block().
//
// Convenience wrappers for a single buffer of data
extern "C" {
    pub fn ocfs2_hamming_encode_block(data: *mut c_void, blocksize: c_uint) -> u32;
}
