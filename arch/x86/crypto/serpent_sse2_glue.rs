//! Automatically rewritten from C to Rust
//! Source: arch/x86/crypto/serpent_sse2_glue.c
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
// Glue Code for SSE2 assembler versions of Serpent Cipher
//
// Copyright (c) 2011 Jussi Kivilinna <jussi.kivilinna@mbnet.fi>
//
// Glue code based on aesni-intel_glue.c by:
// Copyright (C) 2008, Intel Corp.
// Author: Huang Ying <ying.huang@intel.com>
//
// CBC & ECB parts based on code (crypto/cbc.c,ecb.c) by:
// Copyright (c) 2006 Herbert Xu <herbert@gondor.apana.org.au>
//

    static int serpent_setkey_skcipher(struct crypto_skcipher *tfm,
    const u8 *key, unsigned int keylen)
    {
    return __serpent_setkey(crypto_skcipher_ctx(tfm), key, keylen);
    }
#[no_mangle]
unsafe extern "C" fn serpent_decrypt_cbc_xway(ctx: *const c_void, dst: *mut u8, src: *const u8) {
    static void serpent_decrypt_cbc_xway(const void *ctx, u8 *dst, const u8 *src)
    {
    u8 buf[SERPENT_PARALLEL_BLOCKS - 1][SERPENT_BLOCK_SIZE];
    const u8 *s = src;
    if (dst == src)
    s = memcpy(buf, src, sizeof(buf));
    serpent_dec_blk_xway(ctx, dst, src);
    crypto_xor(dst + SERPENT_BLOCK_SIZE, s, sizeof(buf));
    }
#[no_mangle]
unsafe extern "C" fn ecb_encrypt(req: *mut skcipher_request) -> c_int {
    static int ecb_encrypt(struct skcipher_request *req)
    {
    ECB_WALK_START(req, SERPENT_BLOCK_SIZE, SERPENT_PARALLEL_BLOCKS);
    ECB_BLOCK(SERPENT_PARALLEL_BLOCKS, serpent_enc_blk_xway);
    ECB_BLOCK(1, __serpent_encrypt);
    ECB_WALK_END();
    }
#[no_mangle]
unsafe extern "C" fn ecb_decrypt(req: *mut skcipher_request) -> c_int {
    static int ecb_decrypt(struct skcipher_request *req)
    {
    ECB_WALK_START(req, SERPENT_BLOCK_SIZE, SERPENT_PARALLEL_BLOCKS);
    ECB_BLOCK(SERPENT_PARALLEL_BLOCKS, serpent_dec_blk_xway);
    ECB_BLOCK(1, __serpent_decrypt);
    ECB_WALK_END();
    }
#[no_mangle]
unsafe extern "C" fn cbc_encrypt(req: *mut skcipher_request) -> c_int {
    static int cbc_encrypt(struct skcipher_request *req)
    {
    CBC_WALK_START(req, SERPENT_BLOCK_SIZE, -1);
    CBC_ENC_BLOCK(__serpent_encrypt);
    CBC_WALK_END();
    }
#[no_mangle]
unsafe extern "C" fn cbc_decrypt(req: *mut skcipher_request) -> c_int {
    static int cbc_decrypt(struct skcipher_request *req)
    {
    CBC_WALK_START(req, SERPENT_BLOCK_SIZE, SERPENT_PARALLEL_BLOCKS);
    CBC_DEC_BLOCK(SERPENT_PARALLEL_BLOCKS, serpent_decrypt_cbc_xway);
    CBC_DEC_BLOCK(1, __serpent_decrypt);
    CBC_WALK_END();
    }
    static struct skcipher_alg serpent_algs[] = {
    {
    .base.cra_name		= "ecb(serpent)",
    .base.cra_driver_name	= "ecb-serpent-sse2",
    .base.cra_priority	= 400,
    .base.cra_blocksize	= SERPENT_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct serpent_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= SERPENT_MIN_KEY_SIZE,
    .max_keysize		= SERPENT_MAX_KEY_SIZE,
    .setkey			= serpent_setkey_skcipher,
    .encrypt		= ecb_encrypt,
    .decrypt		= ecb_decrypt,
    }, {
    .base.cra_name		= "cbc(serpent)",
    .base.cra_driver_name	= "cbc-serpent-sse2",
    .base.cra_priority	= 400,
    .base.cra_blocksize	= SERPENT_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct serpent_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= SERPENT_MIN_KEY_SIZE,
    .max_keysize		= SERPENT_MAX_KEY_SIZE,
    .ivsize			= SERPENT_BLOCK_SIZE,
    .setkey			= serpent_setkey_skcipher,
    .encrypt		= cbc_encrypt,
    .decrypt		= cbc_decrypt,
    },
    };
#[no_mangle]
unsafe extern "C" fn serpent_sse2_init() -> int __init {
    static int __init serpent_sse2_init(void)
    {
    if (!boot_cpu_has(X86_FEATURE_XMM2)) {
    printk(KERN_INFO "SSE2 instructions are not detected.\n");
    return -ENODEV;
    }
    return crypto_register_skciphers(serpent_algs,
    ARRAY_SIZE(serpent_algs));
    }
#[no_mangle]
unsafe extern "C" fn serpent_sse2_exit() -> void __exit {
    static void __exit serpent_sse2_exit(void)
    {
    crypto_unregister_skciphers(serpent_algs, ARRAY_SIZE(serpent_algs));
    }
    module_init(serpent_sse2_init);
    module_exit(serpent_sse2_exit);
    MODULE_DESCRIPTION("Serpent Cipher Algorithm, SSE2 optimized");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_CRYPTO("serpent");
