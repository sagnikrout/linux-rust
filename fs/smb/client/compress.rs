//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/compress.h
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
// Copyright (C) 2024, SUSE LLC
//
// Authors: Enzo Matsumiya <ematsumiya@suse.de>
//
// This file implements I/O compression support for SMB2 messages (SMB 3.1.1 only).
// See compress/ for implementation details of each algorithm.
//
// References:
// MS-SMB2 "3.1.4.4 Compressing the Message" - for compression details
// MS-SMB2 "3.1.5.3 Decompressing the Chained Message" - for decompression details
// MS-XCA - for details of the supported algorithms
//

// sizeof(smb2_compression_hdr) - sizeof(OriginalPayloadSize)
pub const SMB_COMPRESS_HDR_LEN: c_int = 16;
// sizeof(smb2_compression_payload_hdr) - sizeof(OriginalPayloadSize)
pub const SMB_COMPRESS_PAYLOAD_HDR_LEN: c_int = 8;

extern "C" {
    pub fn int(: *mut *mut compress_send_fn)(struct TCP_Server_Info, _arg: c_int, : *mut smb_rqst) -> typedef;
}
extern "C" {
    pub fn should_compress(tcon: *const cifs_tcon, rq: *const smb_rqst) -> bool;
}

