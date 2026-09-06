//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/ccp/ccp-crypto-aes-galois.c
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
// AMD Cryptographic Coprocessor (CCP) AES GCM crypto API support
//
// Copyright (C) 2016,2017 Advanced Micro Devices, Inc.
//
// Author: Gary R Hook <gary.hook@amd.com>
//

#[no_mangle]
unsafe extern "C" fn ccp_aes_gcm_complete(async_req: *mut crypto_async_request, ret: c_int) -> c_int {
    static int ccp_aes_gcm_complete(struct crypto_async_request *async_req, int ret)
    {
    return ret;
    }
    static int ccp_aes_gcm_setkey(struct crypto_aead *tfm, const u8 *key,
    unsigned int key_len)
    {
    struct ccp_ctx *ctx = crypto_aead_ctx_dma(tfm);
    switch (key_len) {
    case AES_KEYSIZE_128:
    ctx.u.aes.type = CCP_AES_TYPE_128;
    break;
    case AES_KEYSIZE_192:
    ctx.u.aes.type = CCP_AES_TYPE_192;
    break;
    case AES_KEYSIZE_256:
    ctx.u.aes.type = CCP_AES_TYPE_256;
    break;
    default:
    return -EINVAL;
    }
    ctx.u.aes.mode = CCP_AES_MODE_GCM;
    ctx.u.aes.key_len = key_len;
    memcpy(ctx.u.aes.key, key, key_len);
    sg_init_one(&ctx.u.aes.key_sg, ctx.u.aes.key, key_len);
    return 0;
    }
    static int ccp_aes_gcm_setauthsize(struct crypto_aead *tfm,
    unsigned int authsize)
    {
    switch (authsize) {
    case 16:
    case 15:
    case 14:
    case 13:
    case 12:
    case 8:
    case 4:
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ccp_aes_gcm_crypt(req: *mut aead_request, encrypt: bool) -> c_int {
    static int ccp_aes_gcm_crypt(struct aead_request *req, bool encrypt)
    {
    struct crypto_aead *tfm = crypto_aead_reqtfm(req);
    struct ccp_ctx *ctx = crypto_aead_ctx_dma(tfm);
    struct ccp_aes_req_ctx *rctx = aead_request_ctx_dma(req);
    struct scatterlist *iv_sg = core::ptr::null_mut();
    let mut iv_len: c_uint = 0;
    int i;
    let mut ret: c_int = 0;
    if (!ctx.u.aes.key_len)
    return -EINVAL;
    if (ctx.u.aes.mode != CCP_AES_MODE_GCM)
    return -EINVAL;
    if (!req.iv)
    return -EINVAL;
//
// 5 parts:
// plaintext/ciphertext input
// AAD
// key
// IV
// Destination+tag buffer
//
// Prepare the IV: 12 bytes + an integer (counter)
    memcpy(rctx.iv, req.iv, GCM_AES_IV_SIZE);
    for (i = 0; i < 3; i++)
    rctx.iv[i + GCM_AES_IV_SIZE] = 0;
    rctx.iv[AES_BLOCK_SIZE - 1] = 1;
// Set up a scatterlist for the IV
    iv_sg = &rctx.iv_sg;
    iv_len = AES_BLOCK_SIZE;
    sg_init_one(iv_sg, rctx.iv, iv_len);
// The AAD + plaintext are concatenated in the src buffer
    memset(&rctx.cmd, 0, sizeof(rctx.cmd));
    INIT_LIST_HEAD(&rctx.cmd.entry);
    rctx.cmd.engine = CCP_ENGINE_AES;
    rctx.cmd.u.aes.authsize = crypto_aead_authsize(tfm);
    rctx.cmd.u.aes.type = ctx.u.aes.type;
    rctx.cmd.u.aes.mode = ctx.u.aes.mode;
    rctx.cmd.u.aes.action = encrypt;
    rctx.cmd.u.aes.key = &ctx.u.aes.key_sg;
    rctx.cmd.u.aes.key_len = ctx.u.aes.key_len;
    rctx.cmd.u.aes.iv = iv_sg;
    rctx.cmd.u.aes.iv_len = iv_len;
    rctx.cmd.u.aes.src = req.src;
    rctx.cmd.u.aes.src_len = req.cryptlen;
    rctx.cmd.u.aes.aad_len = req.assoclen;
// The cipher text + the tag are in the dst buffer
    rctx.cmd.u.aes.dst = req.dst;
    ret = ccp_crypto_enqueue_request(&req.base, &rctx.cmd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ccp_aes_gcm_encrypt(req: *mut aead_request) -> c_int {
    static int ccp_aes_gcm_encrypt(struct aead_request *req)
    {
    return ccp_aes_gcm_crypt(req, CCP_AES_ACTION_ENCRYPT);
    }
#[no_mangle]
unsafe extern "C" fn ccp_aes_gcm_decrypt(req: *mut aead_request) -> c_int {
    static int ccp_aes_gcm_decrypt(struct aead_request *req)
    {
    return ccp_aes_gcm_crypt(req, CCP_AES_ACTION_DECRYPT);
    }
#[no_mangle]
unsafe extern "C" fn ccp_aes_gcm_cra_init(tfm: *mut crypto_aead) -> c_int {
    static int ccp_aes_gcm_cra_init(struct crypto_aead *tfm)
    {
    struct ccp_ctx *ctx = crypto_aead_ctx_dma(tfm);
    ctx.complete = ccp_aes_gcm_complete;
    ctx.u.aes.key_len = 0;
    crypto_aead_set_reqsize_dma(tfm, sizeof(struct ccp_aes_req_ctx));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ccp_aes_gcm_cra_exit(tfm: *mut crypto_tfm) {
    static void ccp_aes_gcm_cra_exit(struct crypto_tfm *tfm)
    {
    }
    static struct aead_alg ccp_aes_gcm_defaults = {
    .setkey = ccp_aes_gcm_setkey,
    .setauthsize = ccp_aes_gcm_setauthsize,
    .encrypt = ccp_aes_gcm_encrypt,
    .decrypt = ccp_aes_gcm_decrypt,
    .init = ccp_aes_gcm_cra_init,
    .ivsize = GCM_AES_IV_SIZE,
    .maxauthsize = AES_BLOCK_SIZE,
    .base = {
    .cra_flags	= CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_ALLOCATES_MEMORY |
    CRYPTO_ALG_KERN_DRIVER_ONLY |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_blocksize	= AES_BLOCK_SIZE,
    .cra_ctxsize	= sizeof(struct ccp_ctx) + CRYPTO_DMA_PADDING,
    .cra_priority	= CCP_CRA_PRIORITY,
    .cra_exit	= ccp_aes_gcm_cra_exit,
    .cra_module	= THIS_MODULE,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_aes_aead_def {
    pub mode: enum ccp_aes_mode,
    pub version: c_uint,
    pub name: *const c_char,
    pub driver_name: *const c_char,
    pub blocksize: c_uint,
    pub ivsize: c_uint,
    pub alg_defaults: *mut aead_alg,
}

    static struct ccp_aes_aead_def aes_aead_algs[] = {
    {
    .mode		= CCP_AES_MODE_GHASH,
    .version	= CCP_VERSION(5, 0),
    .name		= "gcm(aes)",
    .driver_name	= "gcm-aes-ccp",
    .blocksize	= 1,
    .ivsize		= AES_BLOCK_SIZE,
    .alg_defaults	= &ccp_aes_gcm_defaults,
    },
    };
    static int ccp_register_aes_aead(struct list_head *head,
    const struct ccp_aes_aead_def *def)
    {
    struct ccp_crypto_aead *ccp_aead;
    struct aead_alg *alg;
    int ret;
    ccp_aead = kzalloc_obj(*ccp_aead);
    if (!ccp_aead)
    return -ENOMEM;
    INIT_LIST_HEAD(&ccp_aead.entry);
    ccp_aead.mode = def.mode;
// Copy the defaults and override as necessary
    alg = &ccp_aead.alg;
// alg = *def->alg_defaults;
    strscpy(alg.base.cra_name, def.name);
    strscpy(alg.base.cra_driver_name, def.driver_name);
    alg.base.cra_blocksize = def.blocksize;
    ret = crypto_register_aead(alg);
    if (ret) {
    pr_err("%s aead algorithm registration error (%d)\n",
    alg.base.cra_name, ret);
    kfree(ccp_aead);
    return ret;
    }
    list_add(&ccp_aead.entry, head);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ccp_register_aes_aeads(head: *mut list_head) -> c_int {
    int ccp_register_aes_aeads(struct list_head *head)
    {
    int i, ret;
    let mut ccpversion: c_uint = ccp_version();
    for (i = 0; i < ARRAY_SIZE(aes_aead_algs); i++) {
    if (aes_aead_algs[i].version > ccpversion)
    continue;
    ret = ccp_register_aes_aead(head, &aes_aead_algs[i]);
    if (ret)
    return ret;
    }
    return 0;
    }
