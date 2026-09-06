//! Automatically rewritten from C to Rust
//! Source: arch/x86/crypto/aria_aesni_avx_glue.c
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
// Glue Code for the AVX/AES-NI/GFNI assembler implementation of the ARIA Cipher
//
// Copyright (c) 2022 Taehee Yoo <ap420073@gmail.com>
//

    asmlinkage void aria_aesni_avx_encrypt_16way(const void *ctx, u8 *dst,
    const u8 *src);
    EXPORT_SYMBOL_GPL(aria_aesni_avx_encrypt_16way);
    asmlinkage void aria_aesni_avx_decrypt_16way(const void *ctx, u8 *dst,
    const u8 *src);
    EXPORT_SYMBOL_GPL(aria_aesni_avx_decrypt_16way);
    asmlinkage void aria_aesni_avx_ctr_crypt_16way(const void *ctx, u8 *dst,
    const u8 *src,
    u8 *keystream, u8 *iv);
    EXPORT_SYMBOL_GPL(aria_aesni_avx_ctr_crypt_16way);
    asmlinkage void aria_aesni_avx_gfni_encrypt_16way(const void *ctx, u8 *dst,
    const u8 *src);
    EXPORT_SYMBOL_GPL(aria_aesni_avx_gfni_encrypt_16way);
    asmlinkage void aria_aesni_avx_gfni_decrypt_16way(const void *ctx, u8 *dst,
    const u8 *src);
    EXPORT_SYMBOL_GPL(aria_aesni_avx_gfni_decrypt_16way);
    asmlinkage void aria_aesni_avx_gfni_ctr_crypt_16way(const void *ctx, u8 *dst,
    const u8 *src,
    u8 *keystream, u8 *iv);
    EXPORT_SYMBOL_GPL(aria_aesni_avx_gfni_ctr_crypt_16way);
    static struct aria_avx_ops aria_ops;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aria_avx_request_ctx {
    pub keystream: [u8; ARIA_AESNI_PARALLEL_BLOCK_SIZE],
}

#[no_mangle]
unsafe extern "C" fn ecb_do_encrypt(req: *mut skcipher_request, rkey: *const u32) -> c_int {
    static int ecb_do_encrypt(struct skcipher_request *req, const u32 *rkey)
    {
    ECB_WALK_START(req, ARIA_BLOCK_SIZE, ARIA_AESNI_PARALLEL_BLOCKS);
    ECB_BLOCK(ARIA_AESNI_PARALLEL_BLOCKS, aria_ops.aria_encrypt_16way);
    ECB_BLOCK(1, aria_encrypt);
    ECB_WALK_END();
    }
#[no_mangle]
unsafe extern "C" fn ecb_do_decrypt(req: *mut skcipher_request, rkey: *const u32) -> c_int {
    static int ecb_do_decrypt(struct skcipher_request *req, const u32 *rkey)
    {
    ECB_WALK_START(req, ARIA_BLOCK_SIZE, ARIA_AESNI_PARALLEL_BLOCKS);
    ECB_BLOCK(ARIA_AESNI_PARALLEL_BLOCKS, aria_ops.aria_decrypt_16way);
    ECB_BLOCK(1, aria_decrypt);
    ECB_WALK_END();
    }
#[no_mangle]
unsafe extern "C" fn aria_avx_ecb_encrypt(req: *mut skcipher_request) -> c_int {
    static int aria_avx_ecb_encrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct aria_ctx *ctx = crypto_skcipher_ctx(tfm);
    return ecb_do_encrypt(req, ctx.enc_key[0]);
    }
#[no_mangle]
unsafe extern "C" fn aria_avx_ecb_decrypt(req: *mut skcipher_request) -> c_int {
    static int aria_avx_ecb_decrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct aria_ctx *ctx = crypto_skcipher_ctx(tfm);
    return ecb_do_decrypt(req, ctx.dec_key[0]);
    }
    static int aria_avx_set_key(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int keylen)
    {
    return aria_set_key(&tfm.base, key, keylen);
    }
#[no_mangle]
unsafe extern "C" fn aria_avx_ctr_encrypt(req: *mut skcipher_request) -> c_int {
    static int aria_avx_ctr_encrypt(struct skcipher_request *req)
    {
    struct aria_avx_request_ctx *req_ctx = skcipher_request_ctx(req);
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct aria_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk walk;
    unsigned int nbytes;
    int err;
    err = skcipher_walk_virt(&walk, req, false);
    while ((nbytes = walk.nbytes) > 0) {
    const u8 *src = walk.src.virt.addr;
    u8 *dst = walk.dst.virt.addr;
    while (nbytes >= ARIA_AESNI_PARALLEL_BLOCK_SIZE) {
    kernel_fpu_begin();
    aria_ops.aria_ctr_crypt_16way(ctx, dst, src,
    &req_ctx.keystream[0],
    walk.iv);
    kernel_fpu_end();
    dst += ARIA_AESNI_PARALLEL_BLOCK_SIZE;
    src += ARIA_AESNI_PARALLEL_BLOCK_SIZE;
    nbytes -= ARIA_AESNI_PARALLEL_BLOCK_SIZE;
    }
    while (nbytes >= ARIA_BLOCK_SIZE) {
    memcpy(&req_ctx.keystream[0], walk.iv, ARIA_BLOCK_SIZE);
    crypto_inc(walk.iv, ARIA_BLOCK_SIZE);
    aria_encrypt(ctx, &req_ctx.keystream[0],
    &req_ctx.keystream[0]);
    crypto_xor_cpy(dst, src, &req_ctx.keystream[0],
    ARIA_BLOCK_SIZE);
    dst += ARIA_BLOCK_SIZE;
    src += ARIA_BLOCK_SIZE;
    nbytes -= ARIA_BLOCK_SIZE;
    }
    if (walk.nbytes == walk.total && nbytes > 0) {
    memcpy(&req_ctx.keystream[0], walk.iv,
    ARIA_BLOCK_SIZE);
    crypto_inc(walk.iv, ARIA_BLOCK_SIZE);
    aria_encrypt(ctx, &req_ctx.keystream[0],
    &req_ctx.keystream[0]);
    crypto_xor_cpy(dst, src, &req_ctx.keystream[0],
    nbytes);
    dst += nbytes;
    src += nbytes;
    nbytes = 0;
    }
    err = skcipher_walk_done(&walk, nbytes);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn aria_avx_init_tfm(tfm: *mut crypto_skcipher) -> c_int {
    static int aria_avx_init_tfm(struct crypto_skcipher *tfm)
    {
    crypto_skcipher_set_reqsize(tfm, sizeof(struct aria_avx_request_ctx));
    return 0;
    }
    static struct skcipher_alg aria_algs[] = {
    {
    .base.cra_name		= "ecb(aria)",
    .base.cra_driver_name	= "ecb-aria-avx",
    .base.cra_priority	= 400,
    .base.cra_blocksize	= ARIA_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct aria_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= ARIA_MIN_KEY_SIZE,
    .max_keysize		= ARIA_MAX_KEY_SIZE,
    .setkey			= aria_avx_set_key,
    .encrypt		= aria_avx_ecb_encrypt,
    .decrypt		= aria_avx_ecb_decrypt,
    }, {
    .base.cra_name		= "ctr(aria)",
    .base.cra_driver_name	= "ctr-aria-avx",
    .base.cra_priority	= 400,
    .base.cra_blocksize	= 1,
    .base.cra_ctxsize	= sizeof(struct aria_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= ARIA_MIN_KEY_SIZE,
    .max_keysize		= ARIA_MAX_KEY_SIZE,
    .ivsize			= ARIA_BLOCK_SIZE,
    .chunksize		= ARIA_BLOCK_SIZE,
    .walksize		= 16 * ARIA_BLOCK_SIZE,
    .setkey			= aria_avx_set_key,
    .encrypt		= aria_avx_ctr_encrypt,
    .decrypt		= aria_avx_ctr_encrypt,
    .init			= aria_avx_init_tfm,
    }
    };
#[no_mangle]
unsafe extern "C" fn aria_avx_init() -> int __init {
    static int __init aria_avx_init(void)
    {
    const char *feature_name;
    if (!boot_cpu_has(X86_FEATURE_AVX) ||
    !boot_cpu_has(X86_FEATURE_AES) ||
    !boot_cpu_has(X86_FEATURE_OSXSAVE)) {
    pr_info("AVX or AES-NI instructions are not detected.\n");
    return -ENODEV;
    }
    if (!cpu_has_xfeatures(XFEATURE_MASK_SSE | XFEATURE_MASK_YMM,
    &feature_name)) {
    pr_info("CPU feature '%s' is not supported.\n", feature_name);
    return -ENODEV;
    }
    if (boot_cpu_has(X86_FEATURE_GFNI)) {
    aria_ops.aria_encrypt_16way = aria_aesni_avx_gfni_encrypt_16way;
    aria_ops.aria_decrypt_16way = aria_aesni_avx_gfni_decrypt_16way;
    aria_ops.aria_ctr_crypt_16way = aria_aesni_avx_gfni_ctr_crypt_16way;
    } else {
    aria_ops.aria_encrypt_16way = aria_aesni_avx_encrypt_16way;
    aria_ops.aria_decrypt_16way = aria_aesni_avx_decrypt_16way;
    aria_ops.aria_ctr_crypt_16way = aria_aesni_avx_ctr_crypt_16way;
    }
    return crypto_register_skciphers(aria_algs, ARRAY_SIZE(aria_algs));
    }
#[no_mangle]
unsafe extern "C" fn aria_avx_exit() -> void __exit {
    static void __exit aria_avx_exit(void)
    {
    crypto_unregister_skciphers(aria_algs, ARRAY_SIZE(aria_algs));
    }
    module_init(aria_avx_init);
    module_exit(aria_avx_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Taehee Yoo <ap420073@gmail.com>");
    MODULE_DESCRIPTION("ARIA Cipher Algorithm, AVX/AES-NI/GFNI optimized");
    MODULE_ALIAS_CRYPTO("aria");
    MODULE_ALIAS_CRYPTO("aria-aesni-avx");
