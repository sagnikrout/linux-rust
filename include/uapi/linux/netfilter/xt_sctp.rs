//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_sctp.h
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

pub const XT_SCTP_SRC_PORTS: c_uint = 0x01;
pub const XT_SCTP_DEST_PORTS: c_uint = 0x02;
pub const XT_SCTP_CHUNK_TYPES: c_uint = 0x04;
pub const XT_SCTP_VALID_FLAGS: c_uint = 0x07;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_sctp_flag_info {
    pub chunktype: __u8,
    pub flag: __u8,
    pub flag_mask: __u8,
}

pub const XT_NUM_SCTP_FLAGS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_sctp_info {
    pub /: *mut *mut __u16 dpts[2]; / Min, Max,
    pub /: *mut *mut __u16 spts[2]; / Min, Max,
    pub /: *mut *mut __u32 chunkmap[256 / sizeof (__u32)]; / Bit mask of chunks to be matched according to RFC 2960,
pub const SCTP_CHUNK_MATCH_ANY: c_uint = 0x01  /* Match if any of the chunk types are present */;
pub const SCTP_CHUNK_MATCH_ALL: c_uint = 0x02  /* Match if all of the chunk types are present */;
pub const SCTP_CHUNK_MATCH_ONLY: c_uint = 0x04  /* Match if these are the only chunk types present */;
    pub chunk_match_type: __u32,
    pub flag_info: [xt_sctp_flag_info; XT_NUM_SCTP_FLAGS],
    pub flag_count: c_int,
    pub flags: __u32,
    pub invflags: __u32,
}

