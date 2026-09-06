//! Automatically rewritten from C to Rust
//! Source: crypto/sm3.c
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
// SM3 secure hash, as specified by OSCCA GM/T 0004-2012 SM3 and
// described at https://tools.ietf.org/html/draft-shen-sm3-hash-01
//
// Copyright (C) 2017 ARM Limited or its affiliates.
// Written by Gilad Ben-Yossef <gilad@benyossef.com>
// Copyright (C) 2021 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>
// Copyright 2026 Google LLC
//

#[no_mangle]
unsafe extern "C" fn crypto_sm3_init(desc: *mut shash_desc) -> c_int {
    static int crypto_sm3_init(struct shash_desc *desc)
    {
    sm3_init(SM3_CTX(desc));
    return 0;
    }
    static int crypto_sm3_update(struct shash_desc *desc,
    const u8 *data, unsigned int len)
    {
    sm3_update(SM3_CTX(desc), data, len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_sm3_final(desc: *mut shash_desc, out: *mut u8) -> c_int {
    static int crypto_sm3_final(struct shash_desc *desc, u8 *out)
    {
    sm3_final(SM3_CTX(desc), out);
    return 0;
    }
    static int crypto_sm3_digest(struct shash_desc *desc,
    const u8 *data, unsigned int len, u8 *out)
    {
    sm3(data, len, out);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_sm3_export_core(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int crypto_sm3_export_core(struct shash_desc *desc, void *out)
    {
    memcpy(out, SM3_CTX(desc), sizeof(struct sm3_ctx));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_sm3_import_core(desc: *mut shash_desc, in: *const c_void) -> c_int {
    static int crypto_sm3_import_core(struct shash_desc *desc, const void *in)
    {
    memcpy(SM3_CTX(desc), in, sizeof(struct sm3_ctx));
    return 0;
    }
    static struct shash_alg sm3_alg = {
    .base.cra_name		= "sm3",
    .base.cra_driver_name	= "sm3-lib",
    .base.cra_priority	= 300,
    .base.cra_blocksize	= SM3_BLOCK_SIZE,
    .base.cra_module	= THIS_MODULE,
    .digestsize		= SM3_DIGEST_SIZE,
    .init			= crypto_sm3_init,
    .update			= crypto_sm3_update,
    .final			= crypto_sm3_final,
    .digest			= crypto_sm3_digest,
    .export_core		= crypto_sm3_export_core,
    .import_core		= crypto_sm3_import_core,
    .descsize		= sizeof(struct sm3_ctx),
    };
#[no_mangle]
unsafe extern "C" fn crypto_sm3_mod_init() -> int __init {
    static int __init crypto_sm3_mod_init(void)
    {
    return crypto_register_shash(&sm3_alg);
    }
    module_init(crypto_sm3_mod_init);
#[no_mangle]
unsafe extern "C" fn crypto_sm3_mod_exit() -> void __exit {
    static void __exit crypto_sm3_mod_exit(void)
    {
    crypto_unregister_shash(&sm3_alg);
    }
    module_exit(crypto_sm3_mod_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Crypto API support for SM3");
    MODULE_ALIAS_CRYPTO("sm3");
    MODULE_ALIAS_CRYPTO("sm3-lib");
