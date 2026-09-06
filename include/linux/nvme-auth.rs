//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nvme-auth.h
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
// Copyright (c) 2021 Hannes Reinecke, SUSE Software Solutions
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_dhchap_key {
    pub len: usize,
    pub hash: u8,
    pub __counted_by(len): u8 key[],
}

extern "C" {
    pub fn nvme_auth_get_seqnum() -> u32;
}
extern "C" {
    pub fn nvme_auth_dhgroup_id(dhgroup_name: *const c_char) -> u8;
}
extern "C" {
    pub fn nvme_auth_hmac_hash_len(hmac_id: u8) -> usize;
}
extern "C" {
    pub fn nvme_auth_hmac_id(hmac_name: *const c_char) -> u8;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_auth_hmac_ctx {
    pub hmac_id: u8,
    pub sha256: hmac_sha256_ctx,
    pub sha384: hmac_sha384_ctx,
    pub sha512: hmac_sha512_ctx,
}

extern "C" {
    pub fn nvme_auth_hmac_final(hmac: *mut nvme_auth_hmac_ctx, out: *mut u8);
}
extern "C" {
    pub fn nvme_auth_key_struct_size(key_len: u32) -> u32;
}
extern "C" {
    pub fn nvme_auth_free_key(key: *mut nvme_dhchap_key);
}
extern "C" {
    pub fn nvme_auth_parse_key(secret: *const c_char, ret_key: *mut nvme_dhchap_key) -> c_int;
}
extern "C" {
    pub fn nvme_auth_gen_privkey(dh_tfm: *mut crypto_kpp, dh_gid: u8) -> c_int;
}
