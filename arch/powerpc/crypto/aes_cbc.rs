//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/crypto/aes_cbc.c
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
// AES CBC routines supporting VMX instructions on the Power 8
//
// Copyright (C) 2015 International Business Machines Inc.
//
// Author: Marcelo Henrique Cerri <mhcerri@br.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p8_aes_cbc_ctx {
    pub fallback: *mut crypto_skcipher,
    pub enc_key: p8_aes_key,
    pub dec_key: p8_aes_key,
}

#[no_mangle]
unsafe extern "C" fn p8_aes_cbc_init(tfm: *mut crypto_skcipher) -> c_int {
    static int p8_aes_cbc_init(struct crypto_skcipher *tfm)
    {
    struct p8_aes_cbc_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct crypto_skcipher *fallback;
    fallback = crypto_alloc_skcipher("cbc(aes)", 0,
    CRYPTO_ALG_NEED_FALLBACK |
    CRYPTO_ALG_ASYNC);
    if (IS_ERR(fallback)) {
    pr_err("Failed to allocate cbc(aes) fallback: %ld\n",
    PTR_ERR(fallback));
    return PTR_ERR(fallback);
    }
    crypto_skcipher_set_reqsize(tfm, sizeof(struct skcipher_request) +
    crypto_skcipher_reqsize(fallback));
    ctx.fallback = fallback;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn p8_aes_cbc_exit(tfm: *mut crypto_skcipher) {
    static void p8_aes_cbc_exit(struct crypto_skcipher *tfm)
    {
    struct p8_aes_cbc_ctx *ctx = crypto_skcipher_ctx(tfm);
    crypto_free_skcipher(ctx.fallback);
    }
    static int p8_aes_cbc_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int keylen)
    {
    struct p8_aes_cbc_ctx *ctx = crypto_skcipher_ctx(tfm);
    int ret;
    preempt_disable();
    pagefault_disable();
    enable_kernel_vsx();
    ret = aes_p8_set_encrypt_key(key, keylen * 8, &ctx.enc_key);
    ret |= aes_p8_set_decrypt_key(key, keylen * 8, &ctx.dec_key);
    disable_kernel_vsx();
    pagefault_enable();
    preempt_enable();
    ret |= crypto_skcipher_setkey(ctx.fallback, key, keylen);
    return ret ? -EINVAL : 0;
    }
#[no_mangle]
unsafe extern "C" fn p8_aes_cbc_crypt(req: *mut skcipher_request, enc: bool) -> c_int {
    static int p8_aes_cbc_crypt(struct skcipher_request *req, bool enc)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    const struct p8_aes_cbc_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk walk;
    unsigned int nbytes;
    int ret;
    if (!crypto_simd_usable()) {
    struct skcipher_request *subreq = skcipher_request_ctx(req);
// subreq = *req;
    skcipher_request_set_tfm(subreq, ctx.fallback);
    return enc ? crypto_skcipher_encrypt(subreq) :
    crypto_skcipher_decrypt(subreq);
    }
    ret = skcipher_walk_virt(&walk, req, false);
    while ((nbytes = walk.nbytes) != 0) {
    preempt_disable();
    pagefault_disable();
    enable_kernel_vsx();
    aes_p8_cbc_encrypt(walk.src.virt.addr,
    walk.dst.virt.addr,
    round_down(nbytes, AES_BLOCK_SIZE),
    enc ? &ctx.enc_key : &ctx.dec_key,
    walk.iv, enc);
    disable_kernel_vsx();
    pagefault_enable();
    preempt_enable();
    ret = skcipher_walk_done(&walk, nbytes % AES_BLOCK_SIZE);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn p8_aes_cbc_encrypt(req: *mut skcipher_request) -> c_int {
    static int p8_aes_cbc_encrypt(struct skcipher_request *req)
    {
    return p8_aes_cbc_crypt(req, true);
    }
#[no_mangle]
unsafe extern "C" fn p8_aes_cbc_decrypt(req: *mut skcipher_request) -> c_int {
    static int p8_aes_cbc_decrypt(struct skcipher_request *req)
    {
    return p8_aes_cbc_crypt(req, false);
    }
    struct skcipher_alg p8_aes_cbc_alg = {
    .base.cra_name = "cbc(aes)",
    .base.cra_driver_name = "p8_aes_cbc",
    .base.cra_module = THIS_MODULE,
    .base.cra_priority = 2000,
    .base.cra_flags = CRYPTO_ALG_NEED_FALLBACK,
    .base.cra_blocksize = AES_BLOCK_SIZE,
    .base.cra_ctxsize = sizeof(struct p8_aes_cbc_ctx),
    .setkey = p8_aes_cbc_setkey,
    .encrypt = p8_aes_cbc_encrypt,
    .decrypt = p8_aes_cbc_decrypt,
    .init = p8_aes_cbc_init,
    .exit = p8_aes_cbc_exit,
    .min_keysize = AES_MIN_KEY_SIZE,
    .max_keysize = AES_MAX_KEY_SIZE,
    .ivsize = AES_BLOCK_SIZE,
    };
