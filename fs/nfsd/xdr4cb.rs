//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/xdr4cb.h
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
pub const NFS4_MAXTAGLEN: c_int = 20;
pub const NFS4_enc_cb_null_sz: c_int = 0;
pub const NFS4_dec_cb_null_sz: c_int = 0;
pub const cb_compound_enc_hdr_sz: c_int = 4;

pub const op_enc_sz: c_int = 1;
pub const op_dec_sz: c_int = 2;

//
// 1: CB_GETATTR opcode (32-bit)
// N: file_handle
// 1: number of entry in attribute array (32-bit)
// 3: entry 0-2 in attribute array (32-bit * 3)
//

//
// 4: fattr_bitmap_maxsz
// 1: attribute array len
// 2: change attr (64-bit)
// 2: size (64-bit)
// 2: atime.seconds (64-bit)
// 1: atime.nanoseconds (32-bit)
// 2: mtime.seconds (64-bit)
// 1: mtime.nanoseconds (32-bit)
//
