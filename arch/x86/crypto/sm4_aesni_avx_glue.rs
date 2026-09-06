//! Automatically rewritten from C to Rust
//! Source: arch/x86/crypto/sm4_aesni_avx_glue.c
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
// SM4 Cipher Algorithm, AES-NI/AVX optimized.
// as specified in
// https://tools.ietf.org/id/draft-ribose-cfrg-sm4-10.html
//
// Copyright (c) 2021, Alibaba Group.
// Copyright (c) 2021 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>
//

    asmlinkage void sm4_aesni_avx_crypt4(const u32 *rk, u8 *dst,
    const u8 *src, int nblocks);
    asmlinkage void sm4_aesni_avx_crypt8(const u32 *rk, u8 *dst,
    const u8 *src, int nblocks);
    asmlinkage void sm4_aesni_avx_ctr_enc_blk8(const u32 *rk, u8 *dst,
    const u8 *src, u8 *iv);
    asmlinkage void sm4_aesni_avx_cbc_dec_blk8(const u32 *rk, u8 *dst,
    const u8 *src, u8 *iv);
    static int sm4_skcipher_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int key_len)
    {
    struct sm4_ctx *ctx = crypto_skcipher_ctx(tfm);
    return sm4_expandkey(ctx, key, key_len);
    }
#[no_mangle]
unsafe extern "C" fn ecb_do_crypt(req: *mut skcipher_request, rkey: *const u32) -> c_int {
    static int ecb_do_crypt(struct skcipher_request *req, const u32 *rkey)
    {
    struct skcipher_walk walk;
    unsigned int nbytes;
    int err;
    err = skcipher_walk_virt(&walk, req, false);
    while ((nbytes = walk.nbytes) > 0) {
    const u8 *src = walk.src.virt.addr;
    u8 *dst = walk.dst.virt.addr;
    kernel_fpu_begin();
    while (nbytes >= SM4_CRYPT8_BLOCK_SIZE) {
    sm4_aesni_avx_crypt8(rkey, dst, src, 8);
    dst += SM4_CRYPT8_BLOCK_SIZE;
    src += SM4_CRYPT8_BLOCK_SIZE;
    nbytes -= SM4_CRYPT8_BLOCK_SIZE;
    }
    while (nbytes >= SM4_BLOCK_SIZE) {
    let mut nblocks: c_uint = min(nbytes >> 4, 4u);
    sm4_aesni_avx_crypt4(rkey, dst, src, nblocks);
    dst += nblocks * SM4_BLOCK_SIZE;
    src += nblocks * SM4_BLOCK_SIZE;
    nbytes -= nblocks * SM4_BLOCK_SIZE;
    }
    kernel_fpu_end();
    err = skcipher_walk_done(&walk, nbytes);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn sm4_avx_ecb_encrypt(req: *mut skcipher_request) -> c_int {
    int sm4_avx_ecb_encrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct sm4_ctx *ctx = crypto_skcipher_ctx(tfm);
    return ecb_do_crypt(req, ctx.rkey_enc);
    }
    EXPORT_SYMBOL_GPL(sm4_avx_ecb_encrypt);
#[no_mangle]
pub unsafe extern "C" fn sm4_avx_ecb_decrypt(req: *mut skcipher_request) -> c_int {
    int sm4_avx_ecb_decrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct sm4_ctx *ctx = crypto_skcipher_ctx(tfm);
    return ecb_do_crypt(req, ctx.rkey_dec);
    }
    EXPORT_SYMBOL_GPL(sm4_avx_ecb_decrypt);
#[no_mangle]
pub unsafe extern "C" fn sm4_cbc_encrypt(req: *mut skcipher_request) -> c_int {
    int sm4_cbc_encrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct sm4_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk walk;
    unsigned int nbytes;
    int err;
    err = skcipher_walk_virt(&walk, req, false);
    while ((nbytes = walk.nbytes) > 0) {
    const u8 *iv = walk.iv;
    const u8 *src = walk.src.virt.addr;
    u8 *dst = walk.dst.virt.addr;
    while (nbytes >= SM4_BLOCK_SIZE) {
    crypto_xor_cpy(dst, src, iv, SM4_BLOCK_SIZE);
    sm4_crypt_block(ctx.rkey_enc, dst, dst);
    iv = dst;
    src += SM4_BLOCK_SIZE;
    dst += SM4_BLOCK_SIZE;
    nbytes -= SM4_BLOCK_SIZE;
    }
    if (iv != walk.iv)
    memcpy(walk.iv, iv, SM4_BLOCK_SIZE);
    err = skcipher_walk_done(&walk, nbytes);
    }
    return err;
    }
    EXPORT_SYMBOL_GPL(sm4_cbc_encrypt);
    int sm4_avx_cbc_decrypt(struct skcipher_request *req,
    unsigned int bsize, sm4_crypt_func func)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct sm4_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk walk;
    unsigned int nbytes;
    int err;
    err = skcipher_walk_virt(&walk, req, false);
    while ((nbytes = walk.nbytes) > 0) {
    const u8 *src = walk.src.virt.addr;
    u8 *dst = walk.dst.virt.addr;
    kernel_fpu_begin();
    while (nbytes >= bsize) {
    func(ctx.rkey_dec, dst, src, walk.iv);
    dst += bsize;
    src += bsize;
    nbytes -= bsize;
    }
    while (nbytes >= SM4_BLOCK_SIZE) {
    u8 keystream[SM4_BLOCK_SIZE * 8];
    u8 iv[SM4_BLOCK_SIZE];
    let mut nblocks: c_uint = min(nbytes >> 4, 8u);
    int i;
    sm4_aesni_avx_crypt8(ctx.rkey_dec, keystream,
    src, nblocks);
    src += ((int)nblocks - 2) * SM4_BLOCK_SIZE;
    dst += (nblocks - 1) * SM4_BLOCK_SIZE;
    memcpy(iv, src + SM4_BLOCK_SIZE, SM4_BLOCK_SIZE);
    for (i = nblocks - 1; i > 0; i--) {
    crypto_xor_cpy(dst, src,
    &keystream[i * SM4_BLOCK_SIZE],
    SM4_BLOCK_SIZE);
    src -= SM4_BLOCK_SIZE;
    dst -= SM4_BLOCK_SIZE;
    }
    crypto_xor_cpy(dst, walk.iv, keystream, SM4_BLOCK_SIZE);
    memcpy(walk.iv, iv, SM4_BLOCK_SIZE);
    dst += nblocks * SM4_BLOCK_SIZE;
    src += (nblocks + 1) * SM4_BLOCK_SIZE;
    nbytes -= nblocks * SM4_BLOCK_SIZE;
    }
    kernel_fpu_end();
    err = skcipher_walk_done(&walk, nbytes);
    }
    return err;
    }
    EXPORT_SYMBOL_GPL(sm4_avx_cbc_decrypt);
#[no_mangle]
unsafe extern "C" fn cbc_decrypt(req: *mut skcipher_request) -> c_int {
    static int cbc_decrypt(struct skcipher_request *req)
    {
    return sm4_avx_cbc_decrypt(req, SM4_CRYPT8_BLOCK_SIZE,
    sm4_aesni_avx_cbc_dec_blk8);
    }
    int sm4_avx_ctr_crypt(struct skcipher_request *req,
    unsigned int bsize, sm4_crypt_func func)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct sm4_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk walk;
    unsigned int nbytes;
    int err;
    err = skcipher_walk_virt(&walk, req, false);
    while ((nbytes = walk.nbytes) > 0) {
    const u8 *src = walk.src.virt.addr;
    u8 *dst = walk.dst.virt.addr;
    kernel_fpu_begin();
    while (nbytes >= bsize) {
    func(ctx.rkey_enc, dst, src, walk.iv);
    dst += bsize;
    src += bsize;
    nbytes -= bsize;
    }
    while (nbytes >= SM4_BLOCK_SIZE) {
    u8 keystream[SM4_BLOCK_SIZE * 8];
    let mut nblocks: c_uint = min(nbytes >> 4, 8u);
    int i;
    for (i = 0; i < nblocks; i++) {
    memcpy(&keystream[i * SM4_BLOCK_SIZE],
    walk.iv, SM4_BLOCK_SIZE);
    crypto_inc(walk.iv, SM4_BLOCK_SIZE);
    }
    sm4_aesni_avx_crypt8(ctx.rkey_enc, keystream,
    keystream, nblocks);
    crypto_xor_cpy(dst, src, keystream,
    nblocks * SM4_BLOCK_SIZE);
    dst += nblocks * SM4_BLOCK_SIZE;
    src += nblocks * SM4_BLOCK_SIZE;
    nbytes -= nblocks * SM4_BLOCK_SIZE;
    }
    kernel_fpu_end();
// tail
    if (walk.nbytes == walk.total && nbytes > 0) {
    u8 keystream[SM4_BLOCK_SIZE];
    memcpy(keystream, walk.iv, SM4_BLOCK_SIZE);
    crypto_inc(walk.iv, SM4_BLOCK_SIZE);
    sm4_crypt_block(ctx.rkey_enc, keystream, keystream);
    crypto_xor_cpy(dst, src, keystream, nbytes);
    dst += nbytes;
    src += nbytes;
    nbytes = 0;
    }
    err = skcipher_walk_done(&walk, nbytes);
    }
    return err;
    }
    EXPORT_SYMBOL_GPL(sm4_avx_ctr_crypt);
#[no_mangle]
unsafe extern "C" fn ctr_crypt(req: *mut skcipher_request) -> c_int {
    static int ctr_crypt(struct skcipher_request *req)
    {
    return sm4_avx_ctr_crypt(req, SM4_CRYPT8_BLOCK_SIZE,
    sm4_aesni_avx_ctr_enc_blk8);
    }
    static struct skcipher_alg sm4_aesni_avx_skciphers[] = {
    {
    .base = {
    .cra_name		= "ecb(sm4)",
    .cra_driver_name	= "ecb-sm4-aesni-avx",
    .cra_priority		= 400,
    .cra_blocksize		= SM4_BLOCK_SIZE,
    .cra_ctxsize		= sizeof(struct sm4_ctx),
    .cra_module		= THIS_MODULE,
    },
    .min_keysize	= SM4_KEY_SIZE,
    .max_keysize	= SM4_KEY_SIZE,
    .walksize	= 8 * SM4_BLOCK_SIZE,
    .setkey		= sm4_skcipher_setkey,
    .encrypt	= sm4_avx_ecb_encrypt,
    .decrypt	= sm4_avx_ecb_decrypt,
    }, {
    .base = {
    .cra_name		= "cbc(sm4)",
    .cra_driver_name	= "cbc-sm4-aesni-avx",
    .cra_priority		= 400,
    .cra_blocksize		= SM4_BLOCK_SIZE,
    .cra_ctxsize		= sizeof(struct sm4_ctx),
    .cra_module		= THIS_MODULE,
    },
    .min_keysize	= SM4_KEY_SIZE,
    .max_keysize	= SM4_KEY_SIZE,
    .ivsize		= SM4_BLOCK_SIZE,
    .walksize	= 8 * SM4_BLOCK_SIZE,
    .setkey		= sm4_skcipher_setkey,
    .encrypt	= sm4_cbc_encrypt,
    .decrypt	= cbc_decrypt,
    }, {
    .base = {
    .cra_name		= "ctr(sm4)",
    .cra_driver_name	= "ctr-sm4-aesni-avx",
    .cra_priority		= 400,
    .cra_blocksize		= 1,
    .cra_ctxsize		= sizeof(struct sm4_ctx),
    .cra_module		= THIS_MODULE,
    },
    .min_keysize	= SM4_KEY_SIZE,
    .max_keysize	= SM4_KEY_SIZE,
    .ivsize		= SM4_BLOCK_SIZE,
    .chunksize	= SM4_BLOCK_SIZE,
    .walksize	= 8 * SM4_BLOCK_SIZE,
    .setkey		= sm4_skcipher_setkey,
    .encrypt	= ctr_crypt,
    .decrypt	= ctr_crypt,
    }
    };
#[no_mangle]
unsafe extern "C" fn sm4_init() -> int __init {
    static int __init sm4_init(void)
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
    return crypto_register_skciphers(sm4_aesni_avx_skciphers,
    ARRAY_SIZE(sm4_aesni_avx_skciphers));
    }
#[no_mangle]
unsafe extern "C" fn sm4_exit() -> void __exit {
    static void __exit sm4_exit(void)
    {
    crypto_unregister_skciphers(sm4_aesni_avx_skciphers,
    ARRAY_SIZE(sm4_aesni_avx_skciphers));
    }
    module_init(sm4_init);
    module_exit(sm4_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Tianjia Zhang <tianjia.zhang@linux.alibaba.com>");
    MODULE_DESCRIPTION("SM4 Cipher Algorithm, AES-NI/AVX optimized");
    MODULE_ALIAS_CRYPTO("sm4");
    MODULE_ALIAS_CRYPTO("sm4-aesni-avx");
