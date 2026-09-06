//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/sctp/stream_interleave.h
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
// SCTP kernel implementation
// (C) Copyright Red Hat Inc. 2017
//
// These are definitions used by the stream schedulers, defined in RFC
// draft ndata (https://tools.ietf.org/html/draft-ietf-tsvwg-sctp-ndata-11)
//
// Please send any bug reports or fixes you make to the
// email addresses:
// lksctp developers <linux-sctp@vger.kernel.org>
//
// Written or modified by:
// Xin Long <lucien.xin@gmail.com>
//

// Macro flag: #define __sctp_stream_interleave_h__
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_stream_interleave {
    pub data_chunk_len: __u16,
    pub ftsn_chunk_len: __u16,
// (I-)DATA process
    pub gfp): int len, __u8 flags, gfp_t,
    pub chunk): *mut *mut void (assign_number)(struct sctp_chunk,
    pub chunk): *mut *mut bool (validate_data)(struct sctp_chunk,
    pub gfp): *mut *mut sctp_chunk chunk, gfp_t,
    pub event): *mut sctp_ulpevent,
    pub gfp): *mut *mut sctp_chunk chunk, gfp_t,
    pub gfp): *mut *mut *mut void (start_pd)(struct sctp_ulpq ulpq, gfp_t,
    pub gfp): *mut *mut *mut void (abort_pd)(struct sctp_ulpq ulpq, gfp_t,
// (I-)FORWARD-TSN process
    pub ctsn): *mut *mut *mut void (generate_ftsn)(struct sctp_outq q, __u32,
    pub chunk): *mut *mut bool (validate_ftsn)(struct sctp_chunk,
    pub ftsn): *mut *mut *mut void (report_ftsn)(struct sctp_ulpq ulpq, __u32,
    pub chunk): *mut sctp_chunk,
}

extern "C" {
    pub fn sctp_stream_interleave_init(stream: *mut sctp_stream);
}
