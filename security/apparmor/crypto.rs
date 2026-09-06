//! Automatically rewritten from C to Rust
//! Source: security/apparmor/crypto.c
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
// AppArmor security module
//
// This file contains AppArmor policy loading interface function definitions.
//
// Copyright 2013 Canonical Ltd.
//
// Fns to provide a checksum of policy that has been loaded this can be
// compared to userspace policy compiles to check loaded policy is what
// it should be.
//

#[no_mangle]
pub unsafe extern "C" fn aa_hash_size() -> c_uint {
    unsigned int aa_hash_size(void)
    {
    return SHA256_DIGEST_SIZE;
    }
    char *aa_calc_hash(void *data, size_t len)
    {
    char *hash;
    hash = kzalloc(SHA256_DIGEST_SIZE, GFP_KERNEL);
    if (!hash)
    return ERR_PTR(-ENOMEM);
    sha256(data, len, hash);
    return hash;
    }
    int aa_calc_profile_hash(struct aa_profile *profile, u32 version, void *start,
    size_t len)
    {
    struct sha256_ctx sctx;
    let mut le32_version: __le32 = cpu_to_le32(version);
    if (!aa_g_hash_policy)
    return 0;
    profile.hash = kzalloc(SHA256_DIGEST_SIZE, GFP_KERNEL);
    if (!profile.hash)
    return -ENOMEM;
    sha256_init(&sctx);
    sha256_update(&sctx, (u8 *)&le32_version, 4);
    sha256_update(&sctx, (u8 *)start, len);
    sha256_final(&sctx, profile.hash);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn init_profile_hash() -> int __init {
    int __init init_profile_hash(void)
    {
    if (apparmor_initialized)
    aa_info_message("AppArmor sha256 policy hashing enabled");
    return 0;
    }
