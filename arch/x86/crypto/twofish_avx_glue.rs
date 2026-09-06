//! Automatically rewritten from C to Rust
//! Source: arch/x86/crypto/twofish_avx_glue.c
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
// Glue Code for AVX assembler version of Twofish Cipher
//
// Copyright (C) 2012 Johannes Goetzfried
// <Johannes.Goetzfried@informatik.stud.uni-erlangen.de>
//
// Copyright © 2013 Jussi Kivilinna <jussi.kivilinna@iki.fi>
//

pub const TWOFISH_PARALLEL_BLOCKS: c_int = 8;
// 8-way parallel cipher functions
    asmlinkage void twofish_ecb_enc_8way(const void *ctx, u8 *dst, const u8 *src);
    asmlinkage void twofish_ecb_dec_8way(const void *ctx, u8 *dst, const u8 *src);
    asmlinkage void twofish_cbc_dec_8way(const void *ctx, u8 *dst, const u8 *src);
    static int twofish_setkey_skcipher(struct crypto_skcipher *tfm,
    const u8 *key, unsigned int keylen)
    {
    return twofish_setkey(&tfm.base, key, keylen);
    }
#[no_mangle]
pub unsafe extern "C" fn twofish_enc_blk_3way(ctx: *const c_void, dst: *mut u8, src: *const u8) {
    static inline void twofish_enc_blk_3way(const void *ctx, u8 *dst, const u8 *src)
    {
    __twofish_enc_blk_3way(ctx, dst, src, false);
    }
#[no_mangle]
unsafe extern "C" fn ecb_encrypt(req: *mut skcipher_request) -> c_int {
    static int ecb_encrypt(struct skcipher_request *req)
    {
    ECB_WALK_START(req, TF_BLOCK_SIZE, TWOFISH_PARALLEL_BLOCKS);
    ECB_BLOCK(TWOFISH_PARALLEL_BLOCKS, twofish_ecb_enc_8way);
    ECB_BLOCK(3, twofish_enc_blk_3way);
    ECB_BLOCK(1, twofish_enc_blk);
    ECB_WALK_END();
    }
#[no_mangle]
unsafe extern "C" fn ecb_decrypt(req: *mut skcipher_request) -> c_int {
    static int ecb_decrypt(struct skcipher_request *req)
    {
    ECB_WALK_START(req, TF_BLOCK_SIZE, TWOFISH_PARALLEL_BLOCKS);
    ECB_BLOCK(TWOFISH_PARALLEL_BLOCKS, twofish_ecb_dec_8way);
    ECB_BLOCK(3, twofish_dec_blk_3way);
    ECB_BLOCK(1, twofish_dec_blk);
    ECB_WALK_END();
    }
#[no_mangle]
unsafe extern "C" fn cbc_encrypt(req: *mut skcipher_request) -> c_int {
    static int cbc_encrypt(struct skcipher_request *req)
    {
    CBC_WALK_START(req, TF_BLOCK_SIZE, -1);
    CBC_ENC_BLOCK(twofish_enc_blk);
    CBC_WALK_END();
    }
#[no_mangle]
unsafe extern "C" fn cbc_decrypt(req: *mut skcipher_request) -> c_int {
    static int cbc_decrypt(struct skcipher_request *req)
    {
    CBC_WALK_START(req, TF_BLOCK_SIZE, TWOFISH_PARALLEL_BLOCKS);
    CBC_DEC_BLOCK(TWOFISH_PARALLEL_BLOCKS, twofish_cbc_dec_8way);
    CBC_DEC_BLOCK(3, twofish_dec_blk_cbc_3way);
    CBC_DEC_BLOCK(1, twofish_dec_blk);
    CBC_WALK_END();
    }
    static struct skcipher_alg twofish_algs[] = {
    {
    .base.cra_name		= "ecb(twofish)",
    .base.cra_driver_name	= "ecb-twofish-avx",
    .base.cra_priority	= 400,
    .base.cra_blocksize	= TF_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct twofish_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= TF_MIN_KEY_SIZE,
    .max_keysize		= TF_MAX_KEY_SIZE,
    .setkey			= twofish_setkey_skcipher,
    .encrypt		= ecb_encrypt,
    .decrypt		= ecb_decrypt,
    }, {
    .base.cra_name		= "cbc(twofish)",
    .base.cra_driver_name	= "cbc-twofish-avx",
    .base.cra_priority	= 400,
    .base.cra_blocksize	= TF_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct twofish_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= TF_MIN_KEY_SIZE,
    .max_keysize		= TF_MAX_KEY_SIZE,
    .ivsize			= TF_BLOCK_SIZE,
    .setkey			= twofish_setkey_skcipher,
    .encrypt		= cbc_encrypt,
    .decrypt		= cbc_decrypt,
    },
    };
#[no_mangle]
unsafe extern "C" fn twofish_init() -> int __init {
    static int __init twofish_init(void)
    {
    const char *feature_name;
    if (!cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM, &feature_name)) {
    pr_info("CPU feature '%s' is not supported.\n", feature_name);
    return -ENODEV;
    }
    return crypto_register_skciphers(twofish_algs,
    ARRAY_SIZE(twofish_algs));
    }
#[no_mangle]
unsafe extern "C" fn twofish_exit() -> void __exit {
    static void __exit twofish_exit(void)
    {
    crypto_unregister_skciphers(twofish_algs, ARRAY_SIZE(twofish_algs));
    }
    module_init(twofish_init);
    module_exit(twofish_exit);
    MODULE_DESCRIPTION("Twofish Cipher Algorithm, AVX optimized");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_CRYPTO("twofish");
