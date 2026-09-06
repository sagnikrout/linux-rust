//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/ccp/ccp-crypto-aes-xts.c
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
// AMD Cryptographic Coprocessor (CCP) AES XTS crypto API support
//
// Copyright (C) 2013,2017 Advanced Micro Devices, Inc.
//
// Author: Gary R Hook <gary.hook@amd.com>
// Author: Tom Lendacky <thomas.lendacky@amd.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_aes_xts_def {
    pub name: *const c_char,
    pub drv_name: *const c_char,
}

    static const struct ccp_aes_xts_def aes_xts_algs[] = {
    {
    .name		= "xts(aes)",
    .drv_name	= "xts-aes-ccp",
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccp_unit_size_map {
    pub size: c_uint,
    pub value: u32,
}

    static struct ccp_unit_size_map xts_unit_sizes[] = {
    {
    .size   = 16,
    .value	= CCP_XTS_AES_UNIT_SIZE_16,
    },
    {
    .size   = 512,
    .value	= CCP_XTS_AES_UNIT_SIZE_512,
    },
    {
    .size   = 1024,
    .value	= CCP_XTS_AES_UNIT_SIZE_1024,
    },
    {
    .size   = 2048,
    .value	= CCP_XTS_AES_UNIT_SIZE_2048,
    },
    {
    .size   = 4096,
    .value	= CCP_XTS_AES_UNIT_SIZE_4096,
    },
    };
#[no_mangle]
unsafe extern "C" fn ccp_aes_xts_complete(async_req: *mut crypto_async_request, ret: c_int) -> c_int {
    static int ccp_aes_xts_complete(struct crypto_async_request *async_req, int ret)
    {
    struct skcipher_request *req = skcipher_request_cast(async_req);
    struct ccp_aes_req_ctx *rctx = skcipher_request_ctx_dma(req);
    if (ret)
    return ret;
    memcpy(req.iv, rctx.iv, AES_BLOCK_SIZE);
    return 0;
    }
    static int ccp_aes_xts_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int key_len)
    {
    struct ccp_ctx *ctx = crypto_skcipher_ctx_dma(tfm);
    let mut ccpversion: c_uint = ccp_version();
    int ret;
    ret = xts_verify_key(tfm, key, key_len);
    if (ret)
    return ret;
// Version 3 devices support 128-bit keys; version 5 devices can
// accommodate 128- and 256-bit keys.
//
    switch (key_len) {
    case AES_KEYSIZE_128 * 2:
    memcpy(ctx.u.aes.key, key, key_len);
    break;
    case AES_KEYSIZE_256 * 2:
    if (ccpversion > CCP_VERSION(3, 0))
    memcpy(ctx.u.aes.key, key, key_len);
    break;
    }
    ctx.u.aes.key_len = key_len / 2;
    sg_init_one(&ctx.u.aes.key_sg, ctx.u.aes.key, key_len);
    return crypto_skcipher_setkey(ctx.u.aes.tfm_skcipher, key, key_len);
    }
    static int ccp_aes_xts_crypt(struct skcipher_request *req,
    unsigned int encrypt)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct ccp_ctx *ctx = crypto_skcipher_ctx_dma(tfm);
    struct ccp_aes_req_ctx *rctx = skcipher_request_ctx_dma(req);
    let mut ccpversion: c_uint = ccp_version();
    let mut fallback: c_uint = 0;
    unsigned int unit;
    u32 unit_size;
    int ret;
    if (!ctx.u.aes.key_len)
    return -EINVAL;
    if (!req.iv)
    return -EINVAL;
// Check conditions under which the CCP can fulfill a request. The
// device can handle input plaintext of a length that is a multiple
// of the unit_size, bug the crypto implementation only supports
// the unit_size being equal to the input length. This limits the
// number of scenarios we can handle.
//
    unit_size = CCP_XTS_AES_UNIT_SIZE__LAST;
    for (unit = 0; unit < ARRAY_SIZE(xts_unit_sizes); unit++) {
    if (req.cryptlen == xts_unit_sizes[unit].size) {
    unit_size = unit;
    break;
    }
    }
// The CCP has restrictions on block sizes. Also, a version 3 device
// only supports AES-128 operations; version 5 CCPs support both
// AES-128 and -256 operations.
//
    if (unit_size == CCP_XTS_AES_UNIT_SIZE__LAST)
    fallback = 1;
    if ((ccpversion < CCP_VERSION(5, 0)) &&
    (ctx.u.aes.key_len != AES_KEYSIZE_128))
    fallback = 1;
    if ((ctx.u.aes.key_len != AES_KEYSIZE_128) &&
    (ctx.u.aes.key_len != AES_KEYSIZE_256))
    fallback = 1;
    if (fallback) {
// Use the fallback to process the request for any
// unsupported unit sizes or key sizes
//
    skcipher_request_set_tfm(&rctx.fallback_req,
    ctx.u.aes.tfm_skcipher);
    skcipher_request_set_callback(&rctx.fallback_req,
    req.base.flags,
    req.base.complete,
    req.base.data);
    skcipher_request_set_crypt(&rctx.fallback_req, req.src,
    req.dst, req.cryptlen, req.iv);
    ret = encrypt ? crypto_skcipher_encrypt(&rctx.fallback_req) :
    crypto_skcipher_decrypt(&rctx.fallback_req);
    return ret;
    }
    memcpy(rctx.iv, req.iv, AES_BLOCK_SIZE);
    sg_init_one(&rctx.iv_sg, rctx.iv, AES_BLOCK_SIZE);
    memset(&rctx.cmd, 0, sizeof(rctx.cmd));
    INIT_LIST_HEAD(&rctx.cmd.entry);
    rctx.cmd.engine = CCP_ENGINE_XTS_AES_128;
    rctx.cmd.u.xts.type = CCP_AES_TYPE_128;
    rctx.cmd.u.xts.action = (encrypt) ? CCP_AES_ACTION_ENCRYPT
    : CCP_AES_ACTION_DECRYPT;
    rctx.cmd.u.xts.unit_size = unit_size;
    rctx.cmd.u.xts.key = &ctx.u.aes.key_sg;
    rctx.cmd.u.xts.key_len = ctx.u.aes.key_len;
    rctx.cmd.u.xts.iv = &rctx.iv_sg;
    rctx.cmd.u.xts.iv_len = AES_BLOCK_SIZE;
    rctx.cmd.u.xts.src = req.src;
    rctx.cmd.u.xts.src_len = req.cryptlen;
    rctx.cmd.u.xts.dst = req.dst;
    ret = ccp_crypto_enqueue_request(&req.base, &rctx.cmd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ccp_aes_xts_encrypt(req: *mut skcipher_request) -> c_int {
    static int ccp_aes_xts_encrypt(struct skcipher_request *req)
    {
    return ccp_aes_xts_crypt(req, 1);
    }
#[no_mangle]
unsafe extern "C" fn ccp_aes_xts_decrypt(req: *mut skcipher_request) -> c_int {
    static int ccp_aes_xts_decrypt(struct skcipher_request *req)
    {
    return ccp_aes_xts_crypt(req, 0);
    }
#[no_mangle]
unsafe extern "C" fn ccp_aes_xts_init_tfm(tfm: *mut crypto_skcipher) -> c_int {
    static int ccp_aes_xts_init_tfm(struct crypto_skcipher *tfm)
    {
    struct ccp_ctx *ctx = crypto_skcipher_ctx_dma(tfm);
    struct crypto_skcipher *fallback_tfm;
    ctx.complete = ccp_aes_xts_complete;
    ctx.u.aes.key_len = 0;
    fallback_tfm = crypto_alloc_skcipher("xts(aes)", 0,
    CRYPTO_ALG_NEED_FALLBACK);
    if (IS_ERR(fallback_tfm)) {
    pr_warn("could not load fallback driver xts(aes)\n");
    return PTR_ERR(fallback_tfm);
    }
    ctx.u.aes.tfm_skcipher = fallback_tfm;
    crypto_skcipher_set_reqsize_dma(tfm,
    sizeof(struct ccp_aes_req_ctx) +
    crypto_skcipher_reqsize(fallback_tfm));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ccp_aes_xts_exit_tfm(tfm: *mut crypto_skcipher) {
    static void ccp_aes_xts_exit_tfm(struct crypto_skcipher *tfm)
    {
    struct ccp_ctx *ctx = crypto_skcipher_ctx_dma(tfm);
    crypto_free_skcipher(ctx.u.aes.tfm_skcipher);
    }
    static int ccp_register_aes_xts_alg(struct list_head *head,
    const struct ccp_aes_xts_def *def)
    {
    struct ccp_crypto_skcipher_alg *ccp_alg;
    struct skcipher_alg *alg;
    int ret;
    ccp_alg = kzalloc_obj(*ccp_alg);
    if (!ccp_alg)
    return -ENOMEM;
    INIT_LIST_HEAD(&ccp_alg.entry);
    alg = &ccp_alg.alg;
    strscpy(alg.base.cra_name, def.name);
    strscpy(alg.base.cra_driver_name, def.drv_name);
    alg.base.cra_flags	= CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_ALLOCATES_MEMORY |
    CRYPTO_ALG_KERN_DRIVER_ONLY |
    CRYPTO_ALG_NEED_FALLBACK;
    alg.base.cra_blocksize	= AES_BLOCK_SIZE;
    alg.base.cra_ctxsize	= sizeof(struct ccp_ctx) +
    crypto_dma_padding();
    alg.base.cra_priority	= CCP_CRA_PRIORITY;
    alg.base.cra_module	= THIS_MODULE;
    alg.setkey		= ccp_aes_xts_setkey;
    alg.encrypt		= ccp_aes_xts_encrypt;
    alg.decrypt		= ccp_aes_xts_decrypt;
    alg.min_keysize	= AES_MIN_KEY_SIZE * 2;
    alg.max_keysize	= AES_MAX_KEY_SIZE * 2;
    alg.ivsize		= AES_BLOCK_SIZE;
    alg.init		= ccp_aes_xts_init_tfm;
    alg.exit		= ccp_aes_xts_exit_tfm;
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
pub unsafe extern "C" fn ccp_register_aes_xts_algs(head: *mut list_head) -> c_int {
    int ccp_register_aes_xts_algs(struct list_head *head)
    {
    int i, ret;
    for (i = 0; i < ARRAY_SIZE(aes_xts_algs); i++) {
    ret = ccp_register_aes_xts_alg(head, &aes_xts_algs[i]);
    if (ret)
    return ret;
    }
    return 0;
    }
