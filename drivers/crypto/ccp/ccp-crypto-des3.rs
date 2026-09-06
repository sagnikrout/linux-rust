//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/ccp/ccp-crypto-des3.c
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
// AMD Cryptographic Coprocessor (CCP) DES3 crypto API support
//
// Copyright (C) 2016,2017 Advanced Micro Devices, Inc.
//
// Author: Gary R Hook <ghook@amd.com>
//

#[no_mangle]
unsafe extern "C" fn ccp_des3_complete(async_req: *mut crypto_async_request, ret: c_int) -> c_int {
    static int ccp_des3_complete(struct crypto_async_request *async_req, int ret)
    {
    struct skcipher_request *req = skcipher_request_cast(async_req);
    struct ccp_ctx *ctx = crypto_skcipher_ctx_dma(
    crypto_skcipher_reqtfm(req));
    struct ccp_des3_req_ctx *rctx = skcipher_request_ctx_dma(req);
    if (ret)
    return ret;
    if (ctx.u.des3.mode != CCP_DES3_MODE_ECB)
    memcpy(req.iv, rctx.iv, DES3_EDE_BLOCK_SIZE);
    return 0;
    }
    static int ccp_des3_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int key_len)
    {
    struct ccp_crypto_skcipher_alg *alg = ccp_crypto_skcipher_alg(tfm);
    struct ccp_ctx *ctx = crypto_skcipher_ctx_dma(tfm);
    int err;
    err = verify_skcipher_des3_key(tfm, key);
    if (err)
    return err;
// It's not clear that there is any support for a keysize of 112.
// If needed, the caller should make K1 == K3
//
    ctx.u.des3.type = CCP_DES3_TYPE_168;
    ctx.u.des3.mode = alg.mode;
    ctx.u.des3.key_len = key_len;
    memcpy(ctx.u.des3.key, key, key_len);
    sg_init_one(&ctx.u.des3.key_sg, ctx.u.des3.key, key_len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ccp_des3_crypt(req: *mut skcipher_request, encrypt: bool) -> c_int {
    static int ccp_des3_crypt(struct skcipher_request *req, bool encrypt)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct ccp_ctx *ctx = crypto_skcipher_ctx_dma(tfm);
    struct ccp_des3_req_ctx *rctx = skcipher_request_ctx_dma(req);
    struct scatterlist *iv_sg = core::ptr::null_mut();
    let mut iv_len: c_uint = 0;
    if (!ctx.u.des3.key_len)
    return -EINVAL;
    if (((ctx.u.des3.mode == CCP_DES3_MODE_ECB) ||
    (ctx.u.des3.mode == CCP_DES3_MODE_CBC)) &&
    (req.cryptlen & (DES3_EDE_BLOCK_SIZE - 1)))
    return -EINVAL;
    if (ctx.u.des3.mode != CCP_DES3_MODE_ECB) {
    if (!req.iv)
    return -EINVAL;
    memcpy(rctx.iv, req.iv, DES3_EDE_BLOCK_SIZE);
    iv_sg = &rctx.iv_sg;
    iv_len = DES3_EDE_BLOCK_SIZE;
    sg_init_one(iv_sg, rctx.iv, iv_len);
    }
    memset(&rctx.cmd, 0, sizeof(rctx.cmd));
    INIT_LIST_HEAD(&rctx.cmd.entry);
    rctx.cmd.engine = CCP_ENGINE_DES3;
    rctx.cmd.u.des3.type = ctx.u.des3.type;
    rctx.cmd.u.des3.mode = ctx.u.des3.mode;
    rctx.cmd.u.des3.action = (encrypt)
    ? CCP_DES3_ACTION_ENCRYPT
    : CCP_DES3_ACTION_DECRYPT;
    rctx.cmd.u.des3.key = &ctx.u.des3.key_sg;
    rctx.cmd.u.des3.key_len = ctx.u.des3.key_len;
    rctx.cmd.u.des3.iv = iv_sg;
    rctx.cmd.u.des3.iv_len = iv_len;
    rctx.cmd.u.des3.src = req.src;
    rctx.cmd.u.des3.src_len = req.cryptlen;
    rctx.cmd.u.des3.dst = req.dst;
    return ccp_crypto_enqueue_request(&req.base, &rctx.cmd);
    }
#[no_mangle]
unsafe extern "C" fn ccp_des3_encrypt(req: *mut skcipher_request) -> c_int {
    static int ccp_des3_encrypt(struct skcipher_request *req)
    {
    return ccp_des3_crypt(req, true);
    }
#[no_mangle]
unsafe extern "C" fn ccp_des3_decrypt(req: *mut skcipher_request) -> c_int {
    static int ccp_des3_decrypt(struct skcipher_request *req)
    {
    return ccp_des3_crypt(req, false);
    }
#[no_mangle]
unsafe extern "C" fn ccp_des3_init_tfm(tfm: *mut crypto_skcipher) -> c_int {
    static int ccp_des3_init_tfm(struct crypto_skcipher *tfm)
    {
    struct ccp_ctx *ctx = crypto_skcipher_ctx_dma(tfm);
    ctx.complete = ccp_des3_complete;
    ctx.u.des3.key_len = 0;
    crypto_skcipher_set_reqsize_dma(tfm, sizeof(struct ccp_des3_req_ctx));
    return 0;
    }
    static const struct skcipher_alg ccp_des3_defaults = {
    .setkey			= ccp_des3_setkey,
    .encrypt		= ccp_des3_encrypt,
    .decrypt		= ccp_des3_decrypt,
    .min_keysize		= DES3_EDE_KEY_SIZE,
    .max_keysize		= DES3_EDE_KEY_SIZE,
    .init			= ccp_des3_init_tfm,
    .base.cra_flags		= CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_ALLOCATES_MEMORY |
    CRYPTO_ALG_KERN_DRIVER_ONLY |
    CRYPTO_ALG_NEED_FALLBACK,
    .base.cra_blocksize	= DES3_EDE_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct ccp_ctx) + CRYPTO_DMA_PADDING,
    .base.cra_priority	= CCP_CRA_PRIORITY,
    .base.cra_module	= THIS_MODULE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_des3_def {
    pub mode: enum ccp_des3_mode,
    pub version: c_uint,
    pub name: *const c_char,
    pub driver_name: *const c_char,
    pub blocksize: c_uint,
    pub ivsize: c_uint,
    pub alg_defaults: *const skcipher_alg,
}

    static const struct ccp_des3_def des3_algs[] = {
    {
    .mode		= CCP_DES3_MODE_ECB,
    .version	= CCP_VERSION(5, 0),
    .name		= "ecb(des3_ede)",
    .driver_name	= "ecb-des3-ccp",
    .blocksize	= DES3_EDE_BLOCK_SIZE,
    .ivsize		= 0,
    .alg_defaults	= &ccp_des3_defaults,
    },
    {
    .mode		= CCP_DES3_MODE_CBC,
    .version	= CCP_VERSION(5, 0),
    .name		= "cbc(des3_ede)",
    .driver_name	= "cbc-des3-ccp",
    .blocksize	= DES3_EDE_BLOCK_SIZE,
    .ivsize		= DES3_EDE_BLOCK_SIZE,
    .alg_defaults	= &ccp_des3_defaults,
    },
    };
    static int ccp_register_des3_alg(struct list_head *head,
    const struct ccp_des3_def *def)
    {
    struct ccp_crypto_skcipher_alg *ccp_alg;
    struct skcipher_alg *alg;
    int ret;
    ccp_alg = kzalloc_obj(*ccp_alg);
    if (!ccp_alg)
    return -ENOMEM;
    INIT_LIST_HEAD(&ccp_alg.entry);
    ccp_alg.mode = def.mode;
// Copy the defaults and override as necessary
    alg = &ccp_alg.alg;
// alg = *def->alg_defaults;
    strscpy(alg.base.cra_name, def.name);
    strscpy(alg.base.cra_driver_name, def.driver_name);
    alg.base.cra_blocksize = def.blocksize;
    alg.ivsize = def.ivsize;
    ret = crypto_register_skcipher(alg);
    if (ret) {
    pr_err("%s skcipher algorithm registration error (%d)\n",
    alg.base.cra_name, ret);
    kfree(ccp_alg);
    return ret;
    }
    list_add(&ccp_alg.entry, head);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ccp_register_des3_algs(head: *mut list_head) -> c_int {
    int ccp_register_des3_algs(struct list_head *head)
    {
    int i, ret;
    let mut ccpversion: c_uint = ccp_version();
    for (i = 0; i < ARRAY_SIZE(des3_algs); i++) {
    if (des3_algs[i].version > ccpversion)
    continue;
    ret = ccp_register_des3_alg(head, &des3_algs[i]);
    if (ret)
    return ret;
    }
    return 0;
    }
