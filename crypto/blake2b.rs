//! Automatically rewritten from C to Rust
//! Source: crypto/blake2b.c
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
//
// Crypto API support for BLAKE2b
//
// Copyright 2025 Google LLC
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blake2b_tfm_ctx {
    pub keylen: c_uint,
    pub key: [u8; BLAKE2B_KEY_SIZE],
}

    static int crypto_blake2b_setkey(struct crypto_shash *tfm,
    const u8 *key, unsigned int keylen)
    {
    struct blake2b_tfm_ctx *tctx = crypto_shash_ctx(tfm);
    if (keylen > BLAKE2B_KEY_SIZE)
    return -EINVAL;
    memcpy(tctx.key, key, keylen);
    tctx.keylen = keylen;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn crypto_blake2b_init(desc: *mut shash_desc) -> c_int {
    static int crypto_blake2b_init(struct shash_desc *desc)
    {
    const struct blake2b_tfm_ctx *tctx = crypto_shash_ctx(desc.tfm);
    let mut digestsize: c_uint = crypto_shash_digestsize(desc.tfm);
    blake2b_init_key(BLAKE2B_CTX(desc), digestsize,
    tctx.key, tctx.keylen);
    return 0;
    }
    static int crypto_blake2b_update(struct shash_desc *desc,
    const u8 *data, unsigned int len)
    {
    blake2b_update(BLAKE2B_CTX(desc), data, len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_blake2b_final(desc: *mut shash_desc, out: *mut u8) -> c_int {
    static int crypto_blake2b_final(struct shash_desc *desc, u8 *out)
    {
    blake2b_final(BLAKE2B_CTX(desc), out);
    return 0;
    }
    static int crypto_blake2b_digest(struct shash_desc *desc,
    const u8 *data, unsigned int len, u8 *out)
    {
    const struct blake2b_tfm_ctx *tctx = crypto_shash_ctx(desc.tfm);
    let mut digestsize: c_uint = crypto_shash_digestsize(desc.tfm);
    blake2b(tctx.key, tctx.keylen, data, len, out, digestsize);
    return 0;
    }

    {								\
    .base.cra_name		= name,				\
    .base.cra_driver_name	= name "-lib",			\
    .base.cra_priority	= 300,				\
    .base.cra_flags		= CRYPTO_ALG_OPTIONAL_KEY,	\
    .base.cra_blocksize	= BLAKE2B_BLOCK_SIZE,		\
    .base.cra_ctxsize	= sizeof(struct blake2b_tfm_ctx), \
    .base.cra_module	= THIS_MODULE,			\
    .digestsize		= digest_size,			\
    .setkey			= crypto_blake2b_setkey,	\
    .init			= crypto_blake2b_init,		\
    .update			= crypto_blake2b_update,	\
    .final			= crypto_blake2b_final,		\
    .digest			= crypto_blake2b_digest,	\
    .descsize		= sizeof(struct blake2b_ctx),	\
    }
    static struct shash_alg algs[] = {
    BLAKE2B_ALG("blake2b-160", BLAKE2B_160_HASH_SIZE),
    BLAKE2B_ALG("blake2b-256", BLAKE2B_256_HASH_SIZE),
    BLAKE2B_ALG("blake2b-384", BLAKE2B_384_HASH_SIZE),
    BLAKE2B_ALG("blake2b-512", BLAKE2B_512_HASH_SIZE),
    };
#[no_mangle]
unsafe extern "C" fn crypto_blake2b_mod_init() -> int __init {
    static int __init crypto_blake2b_mod_init(void)
    {
    return crypto_register_shashes(algs, ARRAY_SIZE(algs));
    }
    module_init(crypto_blake2b_mod_init);
#[no_mangle]
unsafe extern "C" fn crypto_blake2b_mod_exit() -> void __exit {
    static void __exit crypto_blake2b_mod_exit(void)
    {
    crypto_unregister_shashes(algs, ARRAY_SIZE(algs));
    }
    module_exit(crypto_blake2b_mod_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Crypto API support for BLAKE2b");
    MODULE_ALIAS_CRYPTO("blake2b-160");
    MODULE_ALIAS_CRYPTO("blake2b-160-lib");
    MODULE_ALIAS_CRYPTO("blake2b-256");
    MODULE_ALIAS_CRYPTO("blake2b-256-lib");
    MODULE_ALIAS_CRYPTO("blake2b-384");
    MODULE_ALIAS_CRYPTO("blake2b-384-lib");
    MODULE_ALIAS_CRYPTO("blake2b-512");
    MODULE_ALIAS_CRYPTO("blake2b-512-lib");
