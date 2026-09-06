//! Automatically rewritten from C to Rust
//! Source: crypto/crypto_null.c
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
// Cryptographic API.
//
// Null algorithms, aka Much Ado About Nothing.
//
// These are needed for IPsec, and may be useful in general for
// testing & debugging.
//
// The null cipher is compliant with RFC2410.
//
// Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
//

#[no_mangle]
unsafe extern "C" fn null_init(desc: *mut shash_desc) -> c_int {
    static int null_init(struct shash_desc *desc)
    {
    return 0;
    }
    static int null_update(struct shash_desc *desc, const u8 *data,
    unsigned int len)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn null_final(desc: *mut shash_desc, out: *mut u8) -> c_int {
    static int null_final(struct shash_desc *desc, u8 *out)
    {
    return 0;
    }
    static int null_digest(struct shash_desc *desc, const u8 *data,
    unsigned int len, u8 *out)
    {
    return 0;
    }
    static int null_hash_setkey(struct crypto_shash *tfm, const u8 *key,
    unsigned int keylen)
    { return 0; }
    static int null_skcipher_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int keylen)
    { return 0; }
#[no_mangle]
unsafe extern "C" fn null_skcipher_crypt(req: *mut skcipher_request) -> c_int {
    static int null_skcipher_crypt(struct skcipher_request *req)
    {
    if (req.src != req.dst)
    memcpy_sglist(req.dst, req.src, req.cryptlen);
    return 0;
    }
    static struct shash_alg digest_null = {
    .digestsize		=	NULL_DIGEST_SIZE,
    .setkey   		=	null_hash_setkey,
    .init   		=	null_init,
    .update 		=	null_update,
    .finup 			=	null_digest,
    .digest 		=	null_digest,
    .final  		=	null_final,
    .base			=	{
    .cra_name		=	"digest_null",
    .cra_driver_name	=	"digest_null-generic",
    .cra_blocksize		=	NULL_BLOCK_SIZE,
    .cra_module		=	THIS_MODULE,
    }
    };
    static struct skcipher_alg skcipher_null = {
    .base.cra_name		=	"ecb(cipher_null)",
    .base.cra_driver_name	=	"ecb-cipher_null",
    .base.cra_priority	=	100,
    .base.cra_blocksize	=	NULL_BLOCK_SIZE,
    .base.cra_ctxsize	=	0,
    .base.cra_module	=	THIS_MODULE,
    .min_keysize		=	NULL_KEY_SIZE,
    .max_keysize		=	NULL_KEY_SIZE,
    .ivsize			=	NULL_IV_SIZE,
    .setkey			=	null_skcipher_setkey,
    .encrypt		=	null_skcipher_crypt,
    .decrypt		=	null_skcipher_crypt,
    };
    MODULE_ALIAS_CRYPTO("digest_null");
    MODULE_ALIAS_CRYPTO("ecb(cipher_null)");
#[no_mangle]
unsafe extern "C" fn crypto_null_mod_init() -> int __init {
    static int __init crypto_null_mod_init(void)
    {
    let mut ret: c_int = 0;
    ret = crypto_register_shash(&digest_null);
    if (ret < 0)
    goto out;
    ret = crypto_register_skcipher(&skcipher_null);
    if (ret < 0)
    goto out_unregister_shash;
    return 0;
    out_unregister_shash:
    crypto_unregister_shash(&digest_null);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn crypto_null_mod_fini() -> void __exit {
    static void __exit crypto_null_mod_fini(void)
    {
    crypto_unregister_shash(&digest_null);
    crypto_unregister_skcipher(&skcipher_null);
    }
    module_init(crypto_null_mod_init);
    module_exit(crypto_null_mod_fini);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Null Cryptographic Algorithms");
