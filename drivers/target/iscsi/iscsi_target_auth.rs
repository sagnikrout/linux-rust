//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/iscsi/iscsi_target_auth.h
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

pub const CHAP_DIGEST_UNKNOWN: c_int = 0;
pub const CHAP_DIGEST_MD5: c_int = 5;
pub const CHAP_DIGEST_SHA1: c_int = 6;
pub const CHAP_DIGEST_SHA256: c_int = 7;
pub const CHAP_DIGEST_SHA3_256: c_int = 8;
pub const MAX_CHAP_CHALLENGE_LEN: c_int = 32;
pub const CHAP_CHALLENGE_STR_LEN: c_int = 4096;

pub const MAX_CHAP_N_SIZE: c_int = 512;

pub const CHAP_STAGE_CLIENT_A: c_int = 1;
pub const CHAP_STAGE_SERVER_AIC: c_int = 2;
pub const CHAP_STAGE_CLIENT_NR: c_int = 3;
pub const CHAP_STAGE_CLIENT_NRIC: c_int = 4;
pub const CHAP_STAGE_SERVER_NR: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_chap {
    pub id: c_uchar,
    pub challenge: [c_uchar; MAX_CHAP_CHALLENGE_LEN],
    pub challenge_len: c_uint,
    pub digest_name: *mut c_uchar,
    pub digest_size: c_uint,
    pub authenticate_target: c_uint,
    pub chap_state: c_uint,
    pub ____cacheline_aligned: },
