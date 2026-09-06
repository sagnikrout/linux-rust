//! Automatically rewritten from C to Rust
//! Source: crypto/bpf_crypto_skcipher.c
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
// Copyright (c) 2024 Meta, Inc

    static void *bpf_crypto_lskcipher_alloc_tfm(const char *algo)
    {
    return crypto_alloc_lskcipher(algo, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn bpf_crypto_lskcipher_free_tfm(tfm: *mut c_void) {
    static void bpf_crypto_lskcipher_free_tfm(void *tfm)
    {
    crypto_free_lskcipher(tfm);
    }
#[no_mangle]
unsafe extern "C" fn bpf_crypto_lskcipher_has_algo(algo: *const c_char) -> c_int {
    static int bpf_crypto_lskcipher_has_algo(const char *algo)
    {
    return crypto_has_skcipher(algo, CRYPTO_ALG_TYPE_LSKCIPHER, CRYPTO_ALG_TYPE_MASK);
    }
#[no_mangle]
unsafe extern "C" fn bpf_crypto_lskcipher_setkey(tfm: *mut c_void, key: *const u8, keylen: c_uint) -> c_int {
    static int bpf_crypto_lskcipher_setkey(void *tfm, const u8 *key, unsigned int keylen)
    {
    return crypto_lskcipher_setkey(tfm, key, keylen);
    }
#[no_mangle]
unsafe extern "C" fn bpf_crypto_lskcipher_get_flags(tfm: *mut c_void) -> u32 {
    static u32 bpf_crypto_lskcipher_get_flags(void *tfm)
    {
    return crypto_lskcipher_get_flags(tfm);
    }
#[no_mangle]
unsafe extern "C" fn bpf_crypto_lskcipher_ivsize(tfm: *mut c_void) -> c_uint {
    static unsigned int bpf_crypto_lskcipher_ivsize(void *tfm)
    {
    return crypto_lskcipher_ivsize(tfm);
    }
#[no_mangle]
unsafe extern "C" fn bpf_crypto_lskcipher_statesize(tfm: *mut c_void) -> c_uint {
    static unsigned int bpf_crypto_lskcipher_statesize(void *tfm)
    {
    return crypto_lskcipher_statesize(tfm);
    }
    static int bpf_crypto_lskcipher_encrypt(void *tfm, const u8 *src, u8 *dst,
    unsigned int len, u8 *siv)
    {
    return crypto_lskcipher_encrypt(tfm, src, dst, len, siv);
    }
    static int bpf_crypto_lskcipher_decrypt(void *tfm, const u8 *src, u8 *dst,
    unsigned int len, u8 *siv)
    {
    return crypto_lskcipher_decrypt(tfm, src, dst, len, siv);
    }
    static const struct bpf_crypto_type bpf_crypto_lskcipher_type = {
    .alloc_tfm	= bpf_crypto_lskcipher_alloc_tfm,
    .free_tfm	= bpf_crypto_lskcipher_free_tfm,
    .has_algo	= bpf_crypto_lskcipher_has_algo,
    .setkey		= bpf_crypto_lskcipher_setkey,
    .encrypt	= bpf_crypto_lskcipher_encrypt,
    .decrypt	= bpf_crypto_lskcipher_decrypt,
    .ivsize		= bpf_crypto_lskcipher_ivsize,
    .statesize	= bpf_crypto_lskcipher_statesize,
    .get_flags	= bpf_crypto_lskcipher_get_flags,
    .owner		= THIS_MODULE,
    .name		= "skcipher",
    };
#[no_mangle]
unsafe extern "C" fn bpf_crypto_skcipher_init() -> int __init {
    static int __init bpf_crypto_skcipher_init(void)
    {
    return bpf_crypto_register_type(&bpf_crypto_lskcipher_type);
    }
#[no_mangle]
unsafe extern "C" fn bpf_crypto_skcipher_exit() -> void __exit {
    static void __exit bpf_crypto_skcipher_exit(void)
    {
    let mut err: c_int = bpf_crypto_unregister_type(&bpf_crypto_lskcipher_type);
    WARN_ON_ONCE(err);
    }
    module_init(bpf_crypto_skcipher_init);
    module_exit(bpf_crypto_skcipher_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Symmetric key cipher support for BPF");
