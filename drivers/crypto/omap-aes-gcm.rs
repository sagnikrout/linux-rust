//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/omap-aes-gcm.c
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
// Cryptographic API.
//
// Support for OMAP AES GCM HW acceleration.
//
// Copyright (c) 2016 Texas Instruments Incorporated
//

    static int omap_aes_gcm_handle_queue(struct omap_aes_dev *dd,
    struct aead_request *req);
#[no_mangle]
unsafe extern "C" fn omap_aes_gcm_finish_req(dd: *mut omap_aes_dev, ret: c_int) {
    static void omap_aes_gcm_finish_req(struct omap_aes_dev *dd, int ret)
    {
    struct aead_request *req = dd.aead_req;
    dd.in_sg = core::ptr::null_mut();
    dd.out_sg = core::ptr::null_mut();
    crypto_finalize_aead_request(dd.engine, req, ret);
    pm_runtime_put_autosuspend(dd.dev);
    }
#[no_mangle]
unsafe extern "C" fn omap_aes_gcm_done_task(dd: *mut omap_aes_dev) {
    static void omap_aes_gcm_done_task(struct omap_aes_dev *dd)
    {
    u8 *tag;
    int alen, clen, i, ret = 0, nsg;
    struct omap_aes_reqctx *rctx;
    alen = ALIGN(dd.assoc_len, AES_BLOCK_SIZE);
    clen = ALIGN(dd.total, AES_BLOCK_SIZE);
    rctx = aead_request_ctx(dd.aead_req);
    nsg = !!(dd.assoc_len && dd.total);
    dma_sync_sg_for_device(dd.dev, dd.out_sg, dd.out_sg_len,
    DMA_FROM_DEVICE);
    dma_unmap_sg(dd.dev, dd.in_sg, dd.in_sg_len, DMA_TO_DEVICE);
    dma_unmap_sg(dd.dev, dd.out_sg, dd.out_sg_len, DMA_FROM_DEVICE);
    omap_aes_crypt_dma_stop(dd);
    omap_crypto_cleanup(dd.out_sg, dd.orig_out,
    dd.aead_req.assoclen, dd.total,
    FLAGS_OUT_DATA_ST_SHIFT, dd.flags);
    if (dd.flags & FLAGS_ENCRYPT)
    scatterwalk_map_and_copy(rctx.auth_tag,
    dd.aead_req.dst,
    dd.total + dd.aead_req.assoclen,
    dd.authsize, 1);
    omap_crypto_cleanup(&dd.in_sgl[0], core::ptr::null_mut(), 0, alen,
    FLAGS_ASSOC_DATA_ST_SHIFT, dd.flags);
    omap_crypto_cleanup(&dd.in_sgl[nsg], core::ptr::null_mut(), 0, clen,
    FLAGS_IN_DATA_ST_SHIFT, dd.flags);
    if (!(dd.flags & FLAGS_ENCRYPT)) {
    tag = (u8 *)rctx.auth_tag;
    for (i = 0; i < dd.authsize; i++) {
    if (tag[i]) {
    ret = -EBADMSG;
    }
    }
    }
    omap_aes_gcm_finish_req(dd, ret);
    }
    static int omap_aes_gcm_copy_buffers(struct omap_aes_dev *dd,
    struct aead_request *req)
    {
    int alen, clen, cryptlen, assoclen, ret;
    struct crypto_aead *aead = crypto_aead_reqtfm(req);
    let mut authlen: c_uint = crypto_aead_authsize(aead);
    struct scatterlist *tmp, sg_arr[2];
    int nsg;
    u16 flags;
    assoclen = req.assoclen;
    cryptlen = req.cryptlen;
    if (dd.flags & FLAGS_RFC4106_GCM)
    assoclen -= 8;
    if (!(dd.flags & FLAGS_ENCRYPT))
    cryptlen -= authlen;
    alen = ALIGN(assoclen, AES_BLOCK_SIZE);
    clen = ALIGN(cryptlen, AES_BLOCK_SIZE);
    nsg = !!(assoclen && cryptlen);
    omap_aes_clear_copy_flags(dd);
    sg_init_table(dd.in_sgl, nsg + 1);
    if (assoclen) {
    tmp = req.src;
    ret = omap_crypto_align_sg(&tmp, assoclen,
    AES_BLOCK_SIZE, dd.in_sgl,
    OMAP_CRYPTO_COPY_DATA |
    OMAP_CRYPTO_ZERO_BUF |
    OMAP_CRYPTO_FORCE_SINGLE_ENTRY,
    FLAGS_ASSOC_DATA_ST_SHIFT,
    &dd.flags);
    if (ret)
    return ret;
    }
    if (cryptlen) {
    tmp = scatterwalk_ffwd(sg_arr, req.src, req.assoclen);
    if (nsg)
    sg_unmark_end(dd.in_sgl);
    ret = omap_crypto_align_sg(&tmp, cryptlen,
    AES_BLOCK_SIZE, &dd.in_sgl[nsg],
    OMAP_CRYPTO_COPY_DATA |
    OMAP_CRYPTO_ZERO_BUF |
    OMAP_CRYPTO_FORCE_SINGLE_ENTRY,
    FLAGS_IN_DATA_ST_SHIFT,
    &dd.flags);
    if (ret)
    return ret;
    }
    dd.in_sg = dd.in_sgl;
    dd.total = cryptlen;
    dd.assoc_len = assoclen;
    dd.authsize = authlen;
    dd.out_sg = req.dst;
    dd.orig_out = req.dst;
    dd.out_sg = scatterwalk_ffwd(sg_arr, req.dst, req.assoclen);
    flags = 0;
    if (req.src == req.dst || dd.out_sg == sg_arr)
    flags |= OMAP_CRYPTO_FORCE_COPY;
    if (cryptlen) {
    ret = omap_crypto_align_sg(&dd.out_sg, cryptlen,
    AES_BLOCK_SIZE, &dd.out_sgl,
    flags,
    FLAGS_OUT_DATA_ST_SHIFT, &dd.flags);
    if (ret)
    return ret;
    }
    dd.in_sg_len = sg_nents_for_len(dd.in_sg, alen + clen);
    dd.out_sg_len = sg_nents_for_len(dd.out_sg, clen);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_encrypt_iv(req: *mut aead_request, tag: *mut u32, iv: *mut u32) -> c_int {
    static int do_encrypt_iv(struct aead_request *req, u32 *tag, u32 *iv)
    {
    struct omap_aes_gcm_ctx *ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    aes_encrypt(&ctx.akey, (u8 *)tag, (const u8 *)iv);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn omap_aes_gcm_dma_out_callback(data: *mut c_void) {
    void omap_aes_gcm_dma_out_callback(void *data)
    {
    struct omap_aes_dev *dd = data;
    struct omap_aes_reqctx *rctx;
    int i, val;
    u32 *auth_tag, tag[4];
    if (!(dd.flags & FLAGS_ENCRYPT))
    scatterwalk_map_and_copy(tag, dd.aead_req.src,
    dd.total + dd.aead_req.assoclen,
    dd.authsize, 0);
    rctx = aead_request_ctx(dd.aead_req);
    auth_tag = (u32 *)rctx.auth_tag;
    for (i = 0; i < 4; i++) {
    val = omap_aes_read(dd, AES_REG_TAG_N(dd, i));
    auth_tag[i] = val ^ auth_tag[i];
    if (!(dd.flags & FLAGS_ENCRYPT))
    auth_tag[i] = auth_tag[i] ^ tag[i];
    }
    omap_aes_gcm_done_task(dd);
    }
    static int omap_aes_gcm_handle_queue(struct omap_aes_dev *dd,
    struct aead_request *req)
    {
    if (req)
    return crypto_transfer_aead_request_to_engine(dd.engine, req);
    return 0;
    }
    static int omap_aes_gcm_prepare_req(struct aead_request *req,
    struct omap_aes_dev *dd)
    {
    struct omap_aes_reqctx *rctx = aead_request_ctx(req);
    struct omap_aes_gcm_ctx *ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    int err;
    dd.aead_req = req;
    rctx.mode &= FLAGS_MODE_MASK;
    dd.flags = (dd.flags & ~FLAGS_MODE_MASK) | rctx.mode;
    err = omap_aes_gcm_copy_buffers(dd, req);
    if (err)
    return err;
    dd.ctx = &ctx.octx;
    return omap_aes_write_ctrl(dd);
    }
#[no_mangle]
unsafe extern "C" fn omap_aes_gcm_crypt(req: *mut aead_request, mode: c_ulong) -> c_int {
    static int omap_aes_gcm_crypt(struct aead_request *req, unsigned long mode)
    {
    struct omap_aes_reqctx *rctx = aead_request_ctx(req);
    struct crypto_aead *aead = crypto_aead_reqtfm(req);
    let mut authlen: c_uint = crypto_aead_authsize(aead);
    struct omap_aes_dev *dd;
    let mut counter: __be32 = cpu_to_be32(1);
    int err, assoclen;
    memset(rctx.auth_tag, 0, sizeof(rctx.auth_tag));
    memcpy(rctx.iv + GCM_AES_IV_SIZE, &counter, 4);
    err = do_encrypt_iv(req, (u32 *)rctx.auth_tag, (u32 *)rctx.iv);
    if (err)
    return err;
    if (mode & FLAGS_RFC4106_GCM)
    assoclen = req.assoclen - 8;
    else
    assoclen = req.assoclen;
    if (assoclen + req.cryptlen == 0) {
    scatterwalk_map_and_copy(rctx.auth_tag, req.dst, 0, authlen,
    1);
    return 0;
    }
    dd = omap_aes_find_dev(rctx);
    if (!dd)
    return -ENODEV;
    rctx.mode = mode;
    return omap_aes_gcm_handle_queue(dd, req);
    }
#[no_mangle]
pub unsafe extern "C" fn omap_aes_gcm_encrypt(req: *mut aead_request) -> c_int {
    int omap_aes_gcm_encrypt(struct aead_request *req)
    {
    struct omap_aes_reqctx *rctx = aead_request_ctx(req);
    memcpy(rctx.iv, req.iv, GCM_AES_IV_SIZE);
    return omap_aes_gcm_crypt(req, FLAGS_ENCRYPT | FLAGS_GCM);
    }
#[no_mangle]
pub unsafe extern "C" fn omap_aes_gcm_decrypt(req: *mut aead_request) -> c_int {
    int omap_aes_gcm_decrypt(struct aead_request *req)
    {
    struct omap_aes_reqctx *rctx = aead_request_ctx(req);
    memcpy(rctx.iv, req.iv, GCM_AES_IV_SIZE);
    return omap_aes_gcm_crypt(req, FLAGS_GCM);
    }
#[no_mangle]
pub unsafe extern "C" fn omap_aes_4106gcm_encrypt(req: *mut aead_request) -> c_int {
    int omap_aes_4106gcm_encrypt(struct aead_request *req)
    {
    struct omap_aes_gcm_ctx *ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    struct omap_aes_reqctx *rctx = aead_request_ctx(req);
    memcpy(rctx.iv, ctx.octx.nonce, 4);
    memcpy(rctx.iv + 4, req.iv, 8);
    return crypto_ipsec_check_assoclen(req.assoclen) ?:
    omap_aes_gcm_crypt(req, FLAGS_ENCRYPT | FLAGS_GCM |
    FLAGS_RFC4106_GCM);
    }
#[no_mangle]
pub unsafe extern "C" fn omap_aes_4106gcm_decrypt(req: *mut aead_request) -> c_int {
    int omap_aes_4106gcm_decrypt(struct aead_request *req)
    {
    struct omap_aes_gcm_ctx *ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    struct omap_aes_reqctx *rctx = aead_request_ctx(req);
    memcpy(rctx.iv, ctx.octx.nonce, 4);
    memcpy(rctx.iv + 4, req.iv, 8);
    return crypto_ipsec_check_assoclen(req.assoclen) ?:
    omap_aes_gcm_crypt(req, FLAGS_GCM | FLAGS_RFC4106_GCM);
    }
    int omap_aes_gcm_setkey(struct crypto_aead *tfm, const u8 *key,
    unsigned int keylen)
    {
    struct omap_aes_gcm_ctx *ctx = crypto_aead_ctx(tfm);
    int ret;
    ret = aes_prepareenckey(&ctx.akey, key, keylen);
    if (ret)
    return ret;
    memcpy(ctx.octx.key, key, keylen);
    ctx.octx.keylen = keylen;
    return 0;
    }
    int omap_aes_4106gcm_setkey(struct crypto_aead *tfm, const u8 *key,
    unsigned int keylen)
    {
    struct omap_aes_gcm_ctx *ctx = crypto_aead_ctx(tfm);
    int ret;
    if (keylen < 4)
    return -EINVAL;
    keylen -= 4;
    ret = aes_prepareenckey(&ctx.akey, key, keylen);
    if (ret)
    return ret;
    memcpy(ctx.octx.key, key, keylen);
    memcpy(ctx.octx.nonce, key + keylen, 4);
    ctx.octx.keylen = keylen;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn omap_aes_gcm_setauthsize(tfm: *mut crypto_aead, authsize: c_uint) -> c_int {
    int omap_aes_gcm_setauthsize(struct crypto_aead *tfm, unsigned int authsize)
    {
    return crypto_gcm_check_authsize(authsize);
    }
    int omap_aes_4106gcm_setauthsize(struct crypto_aead *parent,
    unsigned int authsize)
    {
    return crypto_rfc4106_check_authsize(authsize);
    }
#[no_mangle]
pub unsafe extern "C" fn omap_aes_gcm_crypt_req(engine: *mut crypto_engine, areq: *mut c_void) -> c_int {
    int omap_aes_gcm_crypt_req(struct crypto_engine *engine, void *areq)
    {
    struct aead_request *req = container_of(areq, struct aead_request,
    base);
    struct omap_aes_reqctx *rctx = aead_request_ctx(req);
    struct omap_aes_dev *dd = rctx.dd;
    int ret;
    if (!dd)
    return -ENODEV;
    ret = omap_aes_gcm_prepare_req(req, dd);
    if (ret)
    return ret;
    if (dd.in_sg_len)
    ret = omap_aes_crypt_dma_start(dd);
    else
    omap_aes_gcm_dma_out_callback(dd);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn omap_aes_gcm_cra_init(tfm: *mut crypto_aead) -> c_int {
    int omap_aes_gcm_cra_init(struct crypto_aead *tfm)
    {
    crypto_aead_set_reqsize(tfm, sizeof(struct omap_aes_reqctx));
    return 0;
    }
