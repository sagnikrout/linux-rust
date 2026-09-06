//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireguard/cookie.h
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
// Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cookie_checker {
    pub secret: [u8; NOISE_HASH_LEN],
    pub cookie_encryption_key: [u8; NOISE_SYMMETRIC_KEY_LEN],
    pub message_mac1_key: [u8; NOISE_SYMMETRIC_KEY_LEN],
    pub secret_birthdate: u64,
    pub secret_lock: rw_semaphore,
    pub device: *mut wg_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cookie {
    pub birthdate: u64,
    pub is_valid: bool,
    pub cookie: [u8; COOKIE_LEN],
    pub have_sent_mac1: bool,
    pub last_mac1_sent: [u8; COOKIE_LEN],
    pub cookie_decryption_key: [u8; NOISE_SYMMETRIC_KEY_LEN],
    pub message_mac1_key: [u8; NOISE_SYMMETRIC_KEY_LEN],
    pub lock: rw_semaphore,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cookie_mac_state {
    INVALID_MAC,
    VALID_MAC_BUT_NO_COOKIE,
    VALID_MAC_WITH_COOKIE_BUT_RATELIMITED,
    VALID_MAC_WITH_COOKIE
}

extern "C" {
    pub fn wg_cookie_checker_precompute_device_keys(checker: *mut cookie_checker);
}
extern "C" {
    pub fn wg_cookie_checker_precompute_peer_keys(peer: *mut wg_peer);
}
extern "C" {
    pub fn wg_cookie_init(cookie: *mut cookie);
}
