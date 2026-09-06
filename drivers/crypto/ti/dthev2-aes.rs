//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/ti/dthev2-aes.c
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
// K3 DTHE V2 crypto accelerator driver
//
// Copyright (C) Texas Instruments 2025 - https://www.ti.com
// Author: T Pratham <t-pratham@ti.com>
//

// Registers
// AES Engine
pub const DTHE_P_AES_BASE: c_uint = 0x7000;
pub const DTHE_P_AES_KEY1_0: c_uint = 0x0038;
pub const DTHE_P_AES_KEY1_1: c_uint = 0x003C;
pub const DTHE_P_AES_KEY1_2: c_uint = 0x0030;
pub const DTHE_P_AES_KEY1_3: c_uint = 0x0034;
pub const DTHE_P_AES_KEY1_4: c_uint = 0x0028;
pub const DTHE_P_AES_KEY1_5: c_uint = 0x002C;
pub const DTHE_P_AES_KEY1_6: c_uint = 0x0020;
pub const DTHE_P_AES_KEY1_7: c_uint = 0x0024;
pub const DTHE_P_AES_KEY2_0: c_uint = 0x0018;
pub const DTHE_P_AES_KEY2_1: c_uint = 0x001C;
pub const DTHE_P_AES_KEY2_2: c_uint = 0x0010;
pub const DTHE_P_AES_KEY2_3: c_uint = 0x0014;
pub const DTHE_P_AES_KEY2_4: c_uint = 0x0008;
pub const DTHE_P_AES_KEY2_5: c_uint = 0x000C;
pub const DTHE_P_AES_KEY2_6: c_uint = 0x0000;
pub const DTHE_P_AES_KEY2_7: c_uint = 0x0004;
pub const DTHE_P_AES_IV_IN_0: c_uint = 0x0040;
pub const DTHE_P_AES_IV_IN_1: c_uint = 0x0044;
pub const DTHE_P_AES_IV_IN_2: c_uint = 0x0048;
pub const DTHE_P_AES_IV_IN_3: c_uint = 0x004C;
pub const DTHE_P_AES_CTRL: c_uint = 0x0050;
pub const DTHE_P_AES_C_LENGTH_0: c_uint = 0x0054;
pub const DTHE_P_AES_C_LENGTH_1: c_uint = 0x0058;
pub const DTHE_P_AES_AUTH_LENGTH: c_uint = 0x005C;
pub const DTHE_P_AES_DATA_IN_OUT: c_uint = 0x0060;
pub const DTHE_P_AES_TAG_OUT: c_uint = 0x0070;
pub const DTHE_P_AES_SYSCONFIG: c_uint = 0x0084;
pub const DTHE_P_AES_IRQSTATUS: c_uint = 0x008C;
pub const DTHE_P_AES_IRQENABLE: c_uint = 0x0090;
// Register write values and macros
    enum aes_ctrl_mode_masks {
    AES_CTRL_ECB_MASK = 0x00,
    AES_CTRL_CBC_MASK = BIT(5),
    AES_CTRL_CTR_MASK = BIT(6),
    AES_CTRL_XTS_MASK = BIT(12) | BIT(11),
    AES_CTRL_GCM_MASK = BIT(17) | BIT(16) | BIT(6),
    AES_CTRL_CCM_MASK = BIT(18) | BIT(6),
    };

// Misc

#[no_mangle]
unsafe extern "C" fn dthe_cipher_init_tfm(tfm: *mut crypto_skcipher) -> c_int {
    static int dthe_cipher_init_tfm(struct crypto_skcipher *tfm)
    {
    struct dthe_tfm_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct dthe_data *dev_data = dthe_get_dev(ctx);
    ctx.dev_data = dev_data;
    ctx.keylen = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dthe_cipher_init_tfm_fallback(tfm: *mut crypto_skcipher) -> c_int {
    static int dthe_cipher_init_tfm_fallback(struct crypto_skcipher *tfm)
    {
    struct dthe_tfm_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct dthe_data *dev_data = dthe_get_dev(ctx);
    const char *alg_name = crypto_tfm_alg_name(crypto_skcipher_tfm(tfm));
    ctx.dev_data = dev_data;
    ctx.keylen = 0;
    ctx.skcipher_fb = crypto_alloc_sync_skcipher(alg_name, 0,
    CRYPTO_ALG_NEED_FALLBACK);
    if (IS_ERR(ctx.skcipher_fb)) {
    dev_err(dev_data.dev, "fallback driver %s couldn't be loaded\n",
    alg_name);
    return PTR_ERR(ctx.skcipher_fb);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dthe_cipher_exit_tfm(tfm: *mut crypto_skcipher) {
    static void dthe_cipher_exit_tfm(struct crypto_skcipher *tfm)
    {
    struct dthe_tfm_ctx *ctx = crypto_skcipher_ctx(tfm);
    crypto_free_sync_skcipher(ctx.skcipher_fb);
    }
#[no_mangle]
unsafe extern "C" fn dthe_aes_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: c_uint) -> c_int {
    static int dthe_aes_setkey(struct crypto_skcipher *tfm, const u8 *key, unsigned int keylen)
    {
    struct dthe_tfm_ctx *ctx = crypto_skcipher_ctx(tfm);
    if (keylen != AES_KEYSIZE_128 && keylen != AES_KEYSIZE_192 && keylen != AES_KEYSIZE_256)
    return -EINVAL;
    ctx.keylen = keylen;
    memcpy(ctx.key, key, keylen);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dthe_aes_ecb_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: c_uint) -> c_int {
    static int dthe_aes_ecb_setkey(struct crypto_skcipher *tfm, const u8 *key, unsigned int keylen)
    {
    struct dthe_tfm_ctx *ctx = crypto_skcipher_ctx(tfm);
    ctx.aes_mode = DTHE_AES_ECB;
    return dthe_aes_setkey(tfm, key, keylen);
    }
#[no_mangle]
unsafe extern "C" fn dthe_aes_cbc_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: c_uint) -> c_int {
    static int dthe_aes_cbc_setkey(struct crypto_skcipher *tfm, const u8 *key, unsigned int keylen)
    {
    struct dthe_tfm_ctx *ctx = crypto_skcipher_ctx(tfm);
    ctx.aes_mode = DTHE_AES_CBC;
    return dthe_aes_setkey(tfm, key, keylen);
    }
#[no_mangle]
unsafe extern "C" fn dthe_aes_ctr_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: c_uint) -> c_int {
    static int dthe_aes_ctr_setkey(struct crypto_skcipher *tfm, const u8 *key, unsigned int keylen)
    {
    struct dthe_tfm_ctx *ctx = crypto_skcipher_ctx(tfm);
    let mut ret: c_int = dthe_aes_setkey(tfm, key, keylen);
    if (ret)
    return ret;
    ctx.aes_mode = DTHE_AES_CTR;
    crypto_sync_skcipher_clear_flags(ctx.skcipher_fb, CRYPTO_TFM_REQ_MASK);
    crypto_sync_skcipher_set_flags(ctx.skcipher_fb,
    crypto_skcipher_get_flags(tfm) &
    CRYPTO_TFM_REQ_MASK);
    return crypto_sync_skcipher_setkey(ctx.skcipher_fb, key, keylen);
    }
#[no_mangle]
unsafe extern "C" fn dthe_aes_xts_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: c_uint) -> c_int {
    static int dthe_aes_xts_setkey(struct crypto_skcipher *tfm, const u8 *key, unsigned int keylen)
    {
    struct dthe_tfm_ctx *ctx = crypto_skcipher_ctx(tfm);
    if (keylen != 2 * AES_KEYSIZE_128 &&
    keylen != 2 * AES_KEYSIZE_192 &&
    keylen != 2 * AES_KEYSIZE_256)
    return -EINVAL;
    ctx.aes_mode = DTHE_AES_XTS;
    ctx.keylen = keylen / 2;
    memcpy(ctx.key, key, keylen);
    crypto_sync_skcipher_clear_flags(ctx.skcipher_fb, CRYPTO_TFM_REQ_MASK);
    crypto_sync_skcipher_set_flags(ctx.skcipher_fb,
    crypto_skcipher_get_flags(tfm) &
    CRYPTO_TFM_REQ_MASK);
    return crypto_sync_skcipher_setkey(ctx.skcipher_fb, key, keylen);
    }
    static void dthe_aes_set_ctrl_key(struct dthe_tfm_ctx *ctx,
    struct dthe_aes_req_ctx *rctx,
    u32 *iv_in)
    {
    struct dthe_data *dev_data = dthe_get_dev(ctx);
    void __iomem *aes_base_reg = dev_data.regs + DTHE_P_AES_BASE;
    let mut ctrl_val: u32 = 0;
    writel_relaxed(ctx.key[0], aes_base_reg + DTHE_P_AES_KEY1_0);
    writel_relaxed(ctx.key[1], aes_base_reg + DTHE_P_AES_KEY1_1);
    writel_relaxed(ctx.key[2], aes_base_reg + DTHE_P_AES_KEY1_2);
    writel_relaxed(ctx.key[3], aes_base_reg + DTHE_P_AES_KEY1_3);
    if (ctx.keylen > AES_KEYSIZE_128) {
    writel_relaxed(ctx.key[4], aes_base_reg + DTHE_P_AES_KEY1_4);
    writel_relaxed(ctx.key[5], aes_base_reg + DTHE_P_AES_KEY1_5);
    }
    if (ctx.keylen == AES_KEYSIZE_256) {
    writel_relaxed(ctx.key[6], aes_base_reg + DTHE_P_AES_KEY1_6);
    writel_relaxed(ctx.key[7], aes_base_reg + DTHE_P_AES_KEY1_7);
    }
    if (ctx.aes_mode == DTHE_AES_XTS) {
    let mut key2_offset: usize = ctx.keylen / sizeof(u32);
    writel_relaxed(ctx.key[key2_offset + 0], aes_base_reg + DTHE_P_AES_KEY2_0);
    writel_relaxed(ctx.key[key2_offset + 1], aes_base_reg + DTHE_P_AES_KEY2_1);
    writel_relaxed(ctx.key[key2_offset + 2], aes_base_reg + DTHE_P_AES_KEY2_2);
    writel_relaxed(ctx.key[key2_offset + 3], aes_base_reg + DTHE_P_AES_KEY2_3);
    if (ctx.keylen > AES_KEYSIZE_128) {
    writel_relaxed(ctx.key[key2_offset + 4], aes_base_reg + DTHE_P_AES_KEY2_4);
    writel_relaxed(ctx.key[key2_offset + 5], aes_base_reg + DTHE_P_AES_KEY2_5);
    }
    if (ctx.keylen == AES_KEYSIZE_256) {
    writel_relaxed(ctx.key[key2_offset + 6], aes_base_reg + DTHE_P_AES_KEY2_6);
    writel_relaxed(ctx.key[key2_offset + 7], aes_base_reg + DTHE_P_AES_KEY2_7);
    }
    }
    if (rctx.enc)
    ctrl_val |= DTHE_AES_CTRL_DIR_ENC;
    if (ctx.keylen == AES_KEYSIZE_128)
    ctrl_val |= DTHE_AES_CTRL_KEYSIZE_16B;
#[no_mangle]
pub unsafe extern "C" fn if(AES_KEYSIZE_192: ctx->keylen ==) -> else {
    else if (ctx.keylen == AES_KEYSIZE_192)
    ctrl_val |= DTHE_AES_CTRL_KEYSIZE_24B;
    else
    ctrl_val |= DTHE_AES_CTRL_KEYSIZE_32B;
// Write AES mode
    ctrl_val &= DTHE_AES_CTRL_MODE_CLEAR_MASK;
    switch (ctx.aes_mode) {
    case DTHE_AES_ECB:
    ctrl_val |= AES_CTRL_ECB_MASK;
    break;
    case DTHE_AES_CBC:
    ctrl_val |= AES_CTRL_CBC_MASK;
    break;
    case DTHE_AES_CTR:
    ctrl_val |= AES_CTRL_CTR_MASK;
    ctrl_val |= DTHE_AES_CTRL_CTR_WIDTH_128B;
    break;
    case DTHE_AES_XTS:
    ctrl_val |= AES_CTRL_XTS_MASK;
    break;
    case DTHE_AES_GCM:
    ctrl_val |= AES_CTRL_GCM_MASK;
    break;
    case DTHE_AES_CCM:
    ctrl_val |= AES_CTRL_CCM_MASK;
    ctrl_val |= FIELD_PREP(DTHE_AES_CTRL_CCM_L_FIELD_MASK,
    (iv_in[0] & DTHE_AES_CCM_L_FROM_IV_MASK));
    ctrl_val |= FIELD_PREP(DTHE_AES_CTRL_CCM_M_FIELD_MASK,
    ((ctx.authsize - 2) >> 1) & DTHE_AES_CCM_M_BITS);
    break;
    }
    if (iv_in) {
    ctrl_val |= DTHE_AES_CTRL_SAVE_CTX_SET;
    for (int i = 0; i < AES_IV_WORDS; ++i)
    writel_relaxed(iv_in[i],
    aes_base_reg + DTHE_P_AES_IV_IN_0 + (DTHE_REG_SIZE * i));
    }
    writel_relaxed(ctrl_val, aes_base_reg + DTHE_P_AES_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn dthe_aes_do_fallback(req: *mut skcipher_request) -> c_int {
    static int dthe_aes_do_fallback(struct skcipher_request *req)
    {
    struct dthe_tfm_ctx *ctx = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req));
    struct dthe_aes_req_ctx *rctx = skcipher_request_ctx(req);
    SYNC_SKCIPHER_REQUEST_ON_STACK(subreq, ctx.skcipher_fb);
    skcipher_request_set_callback(subreq, skcipher_request_flags(req),
    req.base.complete, req.base.data);
    skcipher_request_set_crypt(subreq, req.src, req.dst,
    req.cryptlen, req.iv);
    return rctx.enc ? crypto_skcipher_encrypt(subreq) :
    crypto_skcipher_decrypt(subreq);
    }
#[no_mangle]
unsafe extern "C" fn dthe_aes_dma_in_callback(data: *mut c_void) {
    static void dthe_aes_dma_in_callback(void *data)
    {
    struct skcipher_request *req = (struct skcipher_request *)data;
    struct dthe_aes_req_ctx *rctx = skcipher_request_ctx(req);
    complete(&rctx.aes_compl);
    }
#[no_mangle]
unsafe extern "C" fn dthe_aes_run(engine: *mut crypto_engine, areq: *mut c_void) -> c_int {
    static int dthe_aes_run(struct crypto_engine *engine, void *areq)
    {
    struct skcipher_request *req = container_of(areq, struct skcipher_request, base);
    struct dthe_tfm_ctx *ctx = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req));
    struct dthe_data *dev_data = dthe_get_dev(ctx);
    struct dthe_aes_req_ctx *rctx = skcipher_request_ctx(req);
    let mut len: c_uint = req.cryptlen;
    struct scatterlist *src = req.src;
    struct scatterlist *dst = req.dst;
    let mut src_nents: c_int = sg_nents_for_len(src, len);
    let mut dst_nents: c_int = sg_nents_for_len(dst, len);
    int src_mapped_nents;
    int dst_mapped_nents;
    bool diff_dst;
    enum dma_data_direction src_dir, dst_dir;
    struct device *tx_dev, *rx_dev;
    struct dma_async_tx_descriptor *desc_in, *desc_out;
    int ret;
    void __iomem *aes_base_reg = dev_data.regs + DTHE_P_AES_BASE;
    let mut aes_irqenable_val: u32 = readl_relaxed(aes_base_reg + DTHE_P_AES_IRQENABLE);
    let mut aes_sysconfig_val: u32 = readl_relaxed(aes_base_reg + DTHE_P_AES_SYSCONFIG);
    aes_sysconfig_val |= DTHE_AES_SYSCONFIG_DMA_DATA_IN_OUT_EN;
    writel_relaxed(aes_sysconfig_val, aes_base_reg + DTHE_P_AES_SYSCONFIG);
    aes_irqenable_val |= DTHE_AES_IRQENABLE_EN_ALL;
    writel_relaxed(aes_irqenable_val, aes_base_reg + DTHE_P_AES_IRQENABLE);
    if (src == dst) {
    diff_dst = false;
    src_dir = DMA_BIDIRECTIONAL;
    dst_dir = DMA_BIDIRECTIONAL;
    } else {
    diff_dst = true;
    src_dir = DMA_TO_DEVICE;
    dst_dir  = DMA_FROM_DEVICE;
    }
//
// CTR mode can operate on any input length, but the hardware
// requires input length to be a multiple of the block size.
// We need to handle the padding in the driver.
//
    if (ctx.aes_mode == DTHE_AES_CTR && req.cryptlen % AES_BLOCK_SIZE) {
    let mut pad_size: c_uint = AES_BLOCK_SIZE - (req.cryptlen % AES_BLOCK_SIZE);
    u8 *pad_buf = rctx.padding;
    struct scatterlist *sg;
    len += pad_size;
    src_nents++;
    dst_nents++;
    src = kmalloc_objs(*src, src_nents, GFP_ATOMIC);
    if (!src) {
    ret = -ENOMEM;
    goto aes_ctr_src_alloc_err;
    }
    sg_init_table(src, src_nents);
    sg = dthe_copy_sg(src, req.src, req.cryptlen);
    memzero_explicit(pad_buf, AES_BLOCK_SIZE);
    sg_set_buf(sg, pad_buf, pad_size);
    if (diff_dst) {
    dst = kmalloc_objs(*dst, dst_nents, GFP_ATOMIC);
    if (!dst) {
    ret = -ENOMEM;
    goto aes_ctr_dst_alloc_err;
    }
    sg_init_table(dst, dst_nents);
    sg = dthe_copy_sg(dst, req.dst, req.cryptlen);
    sg_set_buf(sg, pad_buf, pad_size);
    } else {
    dst = src;
    }
    }
    tx_dev = dmaengine_get_dma_device(dev_data.dma_aes_tx);
    rx_dev = dmaengine_get_dma_device(dev_data.dma_aes_rx);
    src_mapped_nents = dma_map_sg(tx_dev, src, src_nents, src_dir);
    if (src_mapped_nents == 0) {
    ret = -EINVAL;
    goto aes_map_src_err;
    }
    if (!diff_dst) {
    dst_mapped_nents = src_mapped_nents;
    } else {
    dst_mapped_nents = dma_map_sg(rx_dev, dst, dst_nents, dst_dir);
    if (dst_mapped_nents == 0) {
    ret = -EINVAL;
    goto aes_map_dst_err;
    }
    }
    desc_in = dmaengine_prep_slave_sg(dev_data.dma_aes_rx, dst, dst_mapped_nents,
    DMA_DEV_TO_MEM, DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!desc_in) {
    dev_err(dev_data.dev, "IN prep_slave_sg() failed\n");
    ret = -EINVAL;
    goto aes_prep_err;
    }
    desc_out = dmaengine_prep_slave_sg(dev_data.dma_aes_tx, src, src_mapped_nents,
    DMA_MEM_TO_DEV, DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!desc_out) {
    dev_err(dev_data.dev, "OUT prep_slave_sg() failed\n");
    ret = -EINVAL;
    goto aes_prep_err;
    }
    desc_in.callback = dthe_aes_dma_in_callback;
    desc_in.callback_param = req;
    init_completion(&rctx.aes_compl);
    if (ctx.aes_mode == DTHE_AES_ECB)
    dthe_aes_set_ctrl_key(ctx, rctx, core::ptr::null_mut());
    else
    dthe_aes_set_ctrl_key(ctx, rctx, (u32 *)req.iv);
    writel_relaxed(lower_32_bits(len), aes_base_reg + DTHE_P_AES_C_LENGTH_0);
    writel_relaxed(upper_32_bits(len), aes_base_reg + DTHE_P_AES_C_LENGTH_1);
    dmaengine_submit(desc_in);
    dmaengine_submit(desc_out);
    dma_async_issue_pending(dev_data.dma_aes_rx);
    dma_async_issue_pending(dev_data.dma_aes_tx);
// Need to do a timeout to ensure finalise gets called if DMA callback fails for any reason
    ret = wait_for_completion_timeout(&rctx.aes_compl, msecs_to_jiffies(DTHE_DMA_TIMEOUT_MS));
    if (!ret) {
    ret = -ETIMEDOUT;
    dmaengine_terminate_sync(dev_data.dma_aes_rx);
    dmaengine_terminate_sync(dev_data.dma_aes_tx);
    for (int i = 0; i < AES_BLOCK_WORDS; ++i)
    readl_relaxed(aes_base_reg + DTHE_P_AES_DATA_IN_OUT + (DTHE_REG_SIZE * i));
    } else {
    ret = 0;
    }
// For modes other than ECB, read IV_OUT
    if (ctx.aes_mode != DTHE_AES_ECB) {
    u32 *iv_out = (u32 *)req.iv;
    for (int i = 0; i < AES_IV_WORDS; ++i)
    iv_out[i] = readl_relaxed(aes_base_reg +
    DTHE_P_AES_IV_IN_0 +
    (DTHE_REG_SIZE * i));
    }
    aes_prep_err:
    if (dst_dir != DMA_BIDIRECTIONAL)
    dma_unmap_sg(rx_dev, dst, dst_nents, dst_dir);
    aes_map_dst_err:
    dma_unmap_sg(tx_dev, src, src_nents, src_dir);
    aes_map_src_err:
    if (ctx.aes_mode == DTHE_AES_CTR && req.cryptlen % AES_BLOCK_SIZE) {
    memzero_explicit(rctx.padding, AES_BLOCK_SIZE);
    if (diff_dst)
    kfree(dst);
    aes_ctr_dst_alloc_err:
    kfree(src);
    aes_ctr_src_alloc_err:
//
// Fallback to software if ENOMEM
//
    if (ret == -ENOMEM)
    ret = dthe_aes_do_fallback(req);
    }
    local_bh_disable();
    crypto_finalize_skcipher_request(dev_data.engine, req, ret);
    local_bh_enable();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dthe_aes_crypt(req: *mut skcipher_request) -> c_int {
    static int dthe_aes_crypt(struct skcipher_request *req)
    {
    struct dthe_tfm_ctx *ctx = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req));
    struct dthe_data *dev_data = dthe_get_dev(ctx);
    struct crypto_engine *engine;
//
// If data is not a multiple of AES_BLOCK_SIZE:
// - need to return -EINVAL for ECB, CBC as they are block ciphers
// - need to fallback to software as H/W doesn't support Ciphertext Stealing for XTS
// - do nothing for CTR
//
    if (req.cryptlen % AES_BLOCK_SIZE) {
    if (ctx.aes_mode == DTHE_AES_XTS)
    return dthe_aes_do_fallback(req);
    if (ctx.aes_mode != DTHE_AES_CTR)
    return -EINVAL;
    }
//
// If data length input is zero, no need to do any operation.
// Except for XTS mode, where data length should be non-zero.
//
    if (req.cryptlen == 0) {
    if (ctx.aes_mode == DTHE_AES_XTS)
    return -EINVAL;
    return 0;
    }
    engine = dev_data.engine;
    return crypto_transfer_skcipher_request_to_engine(engine, req);
    }
#[no_mangle]
unsafe extern "C" fn dthe_aes_encrypt(req: *mut skcipher_request) -> c_int {
    static int dthe_aes_encrypt(struct skcipher_request *req)
    {
    struct dthe_aes_req_ctx *rctx = skcipher_request_ctx(req);
    rctx.enc = 1;
    return dthe_aes_crypt(req);
    }
#[no_mangle]
unsafe extern "C" fn dthe_aes_decrypt(req: *mut skcipher_request) -> c_int {
    static int dthe_aes_decrypt(struct skcipher_request *req)
    {
    struct dthe_aes_req_ctx *rctx = skcipher_request_ctx(req);
    rctx.enc = 0;
    return dthe_aes_crypt(req);
    }
#[no_mangle]
unsafe extern "C" fn dthe_aead_init_tfm(tfm: *mut crypto_aead) -> c_int {
    static int dthe_aead_init_tfm(struct crypto_aead *tfm)
    {
    struct dthe_tfm_ctx *ctx = crypto_aead_ctx(tfm);
    struct dthe_data *dev_data = dthe_get_dev(ctx);
    ctx.dev_data = dev_data;
    const char *alg_name = crypto_tfm_alg_name(crypto_aead_tfm(tfm));
    ctx.aead_fb = crypto_alloc_sync_aead(alg_name, 0,
    CRYPTO_ALG_NEED_FALLBACK);
    if (IS_ERR(ctx.aead_fb)) {
    dev_err(dev_data.dev, "fallback driver %s couldn't be loaded\n",
    alg_name);
    return PTR_ERR(ctx.aead_fb);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dthe_aead_exit_tfm(tfm: *mut crypto_aead) {
    static void dthe_aead_exit_tfm(struct crypto_aead *tfm)
    {
    struct dthe_tfm_ctx *ctx = crypto_aead_ctx(tfm);
    crypto_free_sync_aead(ctx.aead_fb);
    }
//
// dthe_aead_prep_aad - Prepare AAD scatterlist from input request
// @sg: Input scatterlist containing AAD
// @assoclen: Length of AAD
// @pad_buf: Buffer to hold AAD padding if needed
//
// Description:
// Creates a scatterlist containing only the AAD portion with padding
// to align to AES_BLOCK_SIZE. This simplifies DMA handling by allowing
// AAD to be sent separately via TX-only DMA.
//
// Return:
// Pointer to the AAD scatterlist, or ERR_PTR(error) on failure.
// The calling function needs to free the returned scatterlist when done.
//
    static struct scatterlist *dthe_aead_prep_aad(struct scatterlist *sg,
    unsigned int assoclen,
    u8 *pad_buf)
    {
    struct scatterlist *aad_sg;
    struct scatterlist *to_sg;
    int aad_nents;
    if (assoclen == 0)
    return core::ptr::null_mut();
    aad_nents = sg_nents_for_len(sg, assoclen);
    if (assoclen % AES_BLOCK_SIZE)
    aad_nents++;
    aad_sg = kmalloc_objs(struct scatterlist, aad_nents, GFP_ATOMIC);
    if (!aad_sg)
    return ERR_PTR(-ENOMEM);
    sg_init_table(aad_sg, aad_nents);
    to_sg = dthe_copy_sg(aad_sg, sg, assoclen);
    if (assoclen % AES_BLOCK_SIZE) {
    let mut pad_len: c_uint = AES_BLOCK_SIZE - (assoclen % AES_BLOCK_SIZE);
    memset(pad_buf, 0, pad_len);
    sg_set_buf(to_sg, pad_buf, pad_len);
    }
    return aad_sg;
    }
//
// dthe_aead_prep_crypt - Prepare crypt scatterlist from req->src/req->dst
// @sg: Input req->src/req->dst scatterlist
// @assoclen: Length of AAD (to skip)
// @cryptlen: Length of ciphertext/plaintext (minus the size of TAG in decryption)
// @pad_buf: Zeroed buffer to hold crypt padding if needed
//
// Description:
// Creates a scatterlist containing only the ciphertext/plaintext portion
// (skipping AAD) with padding to align to AES_BLOCK_SIZE.
//
// Return:
// Pointer to the ciphertext scatterlist, or ERR_PTR(error) on failure.
// The calling function needs to free the returned scatterlist when done.
//
    static struct scatterlist *dthe_aead_prep_crypt(struct scatterlist *sg,
    unsigned int assoclen,
    unsigned int cryptlen,
    u8 *pad_buf)
    {
    struct scatterlist *out_sg[1];
    struct scatterlist *crypt_sg;
    struct scatterlist *to_sg;
    size_t split_sizes[1] = {cryptlen};
    int out_mapped_nents[1];
    int crypt_nents;
    int err;
    if (cryptlen == 0)
    return core::ptr::null_mut();
// Skip AAD, extract ciphertext portion
    err = sg_split(sg, 0, assoclen, 1, split_sizes, out_sg, out_mapped_nents, GFP_ATOMIC);
    if (err)
    goto dthe_aead_prep_crypt_split_err;
    crypt_nents = sg_nents_for_len(out_sg[0], cryptlen);
    if (cryptlen % AES_BLOCK_SIZE)
    crypt_nents++;
    crypt_sg = kmalloc_objs(struct scatterlist, crypt_nents, GFP_ATOMIC);
    if (!crypt_sg) {
    err = -ENOMEM;
    goto dthe_aead_prep_crypt_mem_err;
    }
    sg_init_table(crypt_sg, crypt_nents);
    to_sg = dthe_copy_sg(crypt_sg, out_sg[0], cryptlen);
    if (cryptlen % AES_BLOCK_SIZE) {
    let mut pad_len: c_uint = AES_BLOCK_SIZE - (cryptlen % AES_BLOCK_SIZE);
    sg_set_buf(to_sg, pad_buf, pad_len);
    }
    dthe_aead_prep_crypt_mem_err:
    kfree(out_sg[0]);
    dthe_aead_prep_crypt_split_err:
    if (err)
    return ERR_PTR(err);
    return crypt_sg;
    }
#[no_mangle]
unsafe extern "C" fn dthe_aead_read_tag(ctx: *mut dthe_tfm_ctx, tag: *mut u32) -> c_int {
    static int dthe_aead_read_tag(struct dthe_tfm_ctx *ctx, u32 *tag)
    {
    struct dthe_data *dev_data = dthe_get_dev(ctx);
    void __iomem *aes_base_reg = dev_data.regs + DTHE_P_AES_BASE;
    u32 val;
    int ret;
    ret = readl_relaxed_poll_timeout(aes_base_reg + DTHE_P_AES_CTRL, val,
    (val & DTHE_AES_CTRL_SAVED_CTX_READY),
    0, POLL_TIMEOUT_INTERVAL);
    if (ret)
    return ret;
    for (int i = 0; i < AES_BLOCK_WORDS; ++i)
    tag[i] = readl_relaxed(aes_base_reg +
    DTHE_P_AES_TAG_OUT +
    DTHE_REG_SIZE * i);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dthe_aead_enc_get_tag(req: *mut aead_request) -> c_int {
    static int dthe_aead_enc_get_tag(struct aead_request *req)
    {
    struct dthe_tfm_ctx *ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    u32 tag[AES_BLOCK_WORDS];
    int nents;
    int ret;
    ret = dthe_aead_read_tag(ctx, tag);
    if (ret)
    return ret;
    nents = sg_nents_for_len(req.dst, req.cryptlen + req.assoclen + ctx.authsize);
    sg_pcopy_from_buffer(req.dst, nents, tag, ctx.authsize,
    req.assoclen + req.cryptlen);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dthe_aead_dec_verify_tag(req: *mut aead_request) -> c_int {
    static int dthe_aead_dec_verify_tag(struct aead_request *req)
    {
    struct dthe_tfm_ctx *ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    u32 tag_out[AES_BLOCK_WORDS];
    u32 tag_in[AES_BLOCK_WORDS];
    int nents;
    int ret;
    ret = dthe_aead_read_tag(ctx, tag_out);
    if (ret)
    return ret;
    nents = sg_nents_for_len(req.src, req.assoclen + req.cryptlen);
    sg_pcopy_to_buffer(req.src, nents, tag_in, ctx.authsize,
    req.assoclen + req.cryptlen - ctx.authsize);
    if (crypto_memneq(tag_in, tag_out, ctx.authsize))
    return -EBADMSG;
    else
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dthe_aead_setkey(tfm: *mut crypto_aead, key: *const u8, keylen: c_uint) -> c_int {
    static int dthe_aead_setkey(struct crypto_aead *tfm, const u8 *key, unsigned int keylen)
    {
    struct dthe_tfm_ctx *ctx = crypto_aead_ctx(tfm);
    if (keylen != AES_KEYSIZE_128 && keylen != AES_KEYSIZE_192 && keylen != AES_KEYSIZE_256)
    return -EINVAL;
    crypto_sync_aead_clear_flags(ctx.aead_fb, CRYPTO_TFM_REQ_MASK);
    crypto_sync_aead_set_flags(ctx.aead_fb,
    crypto_aead_get_flags(tfm) &
    CRYPTO_TFM_REQ_MASK);
    return crypto_sync_aead_setkey(ctx.aead_fb, key, keylen);
    }
#[no_mangle]
unsafe extern "C" fn dthe_gcm_aes_setkey(tfm: *mut crypto_aead, key: *const u8, keylen: c_uint) -> c_int {
    static int dthe_gcm_aes_setkey(struct crypto_aead *tfm, const u8 *key, unsigned int keylen)
    {
    struct dthe_tfm_ctx *ctx = crypto_aead_ctx(tfm);
    int ret;
    ret = dthe_aead_setkey(tfm, key, keylen);
    if (ret)
    return ret;
    ctx.aes_mode = DTHE_AES_GCM;
    ctx.keylen = keylen;
    memcpy(ctx.key, key, keylen);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dthe_ccm_aes_setkey(tfm: *mut crypto_aead, key: *const u8, keylen: c_uint) -> c_int {
    static int dthe_ccm_aes_setkey(struct crypto_aead *tfm, const u8 *key, unsigned int keylen)
    {
    struct dthe_tfm_ctx *ctx = crypto_aead_ctx(tfm);
    int ret;
    ret = dthe_aead_setkey(tfm, key, keylen);
    if (ret)
    return ret;
    ctx.aes_mode = DTHE_AES_CCM;
    ctx.keylen = keylen;
    memcpy(ctx.key, key, keylen);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dthe_aead_setauthsize(tfm: *mut crypto_aead, authsize: c_uint) -> c_int {
    static int dthe_aead_setauthsize(struct crypto_aead *tfm, unsigned int authsize)
    {
    struct dthe_tfm_ctx *ctx = crypto_aead_ctx(tfm);
// Invalid auth size will be handled by crypto_aead_setauthsize()
    ctx.authsize = authsize;
    return crypto_sync_aead_setauthsize(ctx.aead_fb, authsize);
    }
#[no_mangle]
unsafe extern "C" fn dthe_aead_do_fallback(req: *mut aead_request) -> c_int {
    static int dthe_aead_do_fallback(struct aead_request *req)
    {
    struct dthe_tfm_ctx *ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    struct dthe_aes_req_ctx *rctx = aead_request_ctx(req);
    SYNC_AEAD_REQUEST_ON_STACK(subreq, ctx.aead_fb);
    aead_request_set_callback(subreq, req.base.flags,
    req.base.complete, req.base.data);
    aead_request_set_crypt(subreq, req.src, req.dst, req.cryptlen, req.iv);
    aead_request_set_ad(subreq, req.assoclen);
    return rctx.enc ? crypto_aead_encrypt(subreq) :
    crypto_aead_decrypt(subreq);
    }
#[no_mangle]
unsafe extern "C" fn dthe_aead_dma_in_callback(data: *mut c_void) {
    static void dthe_aead_dma_in_callback(void *data)
    {
    struct aead_request *req = (struct aead_request *)data;
    struct dthe_aes_req_ctx *rctx = aead_request_ctx(req);
    complete(&rctx.aes_compl);
    }
#[no_mangle]
unsafe extern "C" fn dthe_aead_run(engine: *mut crypto_engine, areq: *mut c_void) -> c_int {
    static int dthe_aead_run(struct crypto_engine *engine, void *areq)
    {
    struct aead_request *req = container_of(areq, struct aead_request, base);
    struct dthe_tfm_ctx *ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    struct dthe_aes_req_ctx *rctx = aead_request_ctx(req);
    struct dthe_data *dev_data = dthe_get_dev(ctx);
    let mut cryptlen: c_uint = req.cryptlen;
    let mut assoclen: c_uint = req.assoclen;
    let mut authsize: c_uint = ctx.authsize;
    unsigned int unpadded_cryptlen;
    struct scatterlist *src = core::ptr::null_mut();
    struct scatterlist *dst = core::ptr::null_mut();
    struct scatterlist *aad_sg = core::ptr::null_mut();
    u32 iv_in[AES_IV_WORDS];
    let mut aad_nents: c_int = 0;
    let mut src_nents: c_int = 0;
    let mut dst_nents: c_int = 0;
    let mut aad_mapped_nents: c_int = 0;
    let mut src_mapped_nents: c_int = 0;
    let mut dst_mapped_nents: c_int = 0;
    u8 *src_assoc_padbuf = rctx.padding;
    u8 *src_crypt_padbuf = rctx.padding + AES_BLOCK_SIZE;
    u8 *dst_crypt_padbuf = rctx.padding + AES_BLOCK_SIZE;
    bool diff_dst;
    enum dma_data_direction aad_dir, src_dir, dst_dir;
    struct device *tx_dev, *rx_dev;
    struct dma_async_tx_descriptor *desc_in, *desc_out, *desc_aad_out;
    int ret;
    int err;
    void __iomem *aes_base_reg = dev_data.regs + DTHE_P_AES_BASE;
    let mut aes_irqenable_val: u32 = readl_relaxed(aes_base_reg + DTHE_P_AES_IRQENABLE);
    let mut aes_sysconfig_val: u32 = readl_relaxed(aes_base_reg + DTHE_P_AES_SYSCONFIG);
    aes_sysconfig_val |= DTHE_AES_SYSCONFIG_DMA_DATA_IN_OUT_EN;
    writel_relaxed(aes_sysconfig_val, aes_base_reg + DTHE_P_AES_SYSCONFIG);
    aes_irqenable_val |= DTHE_AES_IRQENABLE_EN_ALL;
    writel_relaxed(aes_irqenable_val, aes_base_reg + DTHE_P_AES_IRQENABLE);
// In decryption, the last authsize bytes are the TAG
    if (!rctx.enc)
    cryptlen -= authsize;
    unpadded_cryptlen = cryptlen;
    memset(src_assoc_padbuf, 0, AES_BLOCK_SIZE);
    memset(src_crypt_padbuf, 0, AES_BLOCK_SIZE);
    memset(dst_crypt_padbuf, 0, AES_BLOCK_SIZE);
    tx_dev = dmaengine_get_dma_device(dev_data.dma_aes_tx);
    rx_dev = dmaengine_get_dma_device(dev_data.dma_aes_rx);
    if (req.src == req.dst) {
    diff_dst = false;
    src_dir = DMA_BIDIRECTIONAL;
    dst_dir = DMA_BIDIRECTIONAL;
    } else {
    diff_dst = true;
    src_dir = DMA_TO_DEVICE;
    dst_dir = DMA_FROM_DEVICE;
    }
    aad_dir = DMA_TO_DEVICE;
// Prep AAD scatterlist (always from req->src)
    aad_sg = dthe_aead_prep_aad(req.src, req.assoclen, src_assoc_padbuf);
    if (IS_ERR(aad_sg)) {
    ret = PTR_ERR(aad_sg);
    goto aead_prep_aad_err;
    }
// Prep ciphertext src scatterlist
    src = dthe_aead_prep_crypt(req.src, req.assoclen, cryptlen, src_crypt_padbuf);
    if (IS_ERR(src)) {
    ret = PTR_ERR(src);
    goto aead_prep_src_err;
    }
// Prep ciphertext dst scatterlist (only if separate dst)
    if (diff_dst) {
    dst = dthe_aead_prep_crypt(req.dst, req.assoclen, unpadded_cryptlen,
    dst_crypt_padbuf);
    if (IS_ERR(dst)) {
    ret = PTR_ERR(dst);
    goto aead_prep_dst_err;
    }
    } else {
    dst = src;
    }
// Calculate padded lengths for nents calculations
    if (req.assoclen % AES_BLOCK_SIZE)
    assoclen += AES_BLOCK_SIZE - (req.assoclen % AES_BLOCK_SIZE);
    if (cryptlen % AES_BLOCK_SIZE)
    cryptlen += AES_BLOCK_SIZE - (cryptlen % AES_BLOCK_SIZE);
    if (assoclen != 0) {
// Map AAD for TX only
    aad_nents = sg_nents_for_len(aad_sg, assoclen);
    aad_mapped_nents = dma_map_sg(tx_dev, aad_sg, aad_nents, aad_dir);
    if (aad_mapped_nents == 0) {
    dev_err(dev_data.dev, "Failed to map AAD for TX\n");
    ret = -EINVAL;
    goto aead_dma_map_aad_err;
    }
// Prepare DMA descriptors for AAD TX
    desc_aad_out = dmaengine_prep_slave_sg(dev_data.dma_aes_tx, aad_sg,
    aad_mapped_nents, DMA_MEM_TO_DEV,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!desc_aad_out) {
    dev_err(dev_data.dev, "AAD TX prep_slave_sg() failed\n");
    ret = -EINVAL;
    goto aead_dma_prep_aad_err;
    }
    }
    if (cryptlen != 0) {
// Map ciphertext src for TX (BIDIRECTIONAL if in-place)
    src_nents = sg_nents_for_len(src, cryptlen);
    src_mapped_nents = dma_map_sg(tx_dev, src, src_nents, src_dir);
    if (src_mapped_nents == 0) {
    dev_err(dev_data.dev, "Failed to map ciphertext src for TX\n");
    ret = -EINVAL;
    goto aead_dma_prep_aad_err;
    }
// Prepare DMA descriptors for ciphertext TX
    desc_out = dmaengine_prep_slave_sg(dev_data.dma_aes_tx, src,
    src_mapped_nents, DMA_MEM_TO_DEV,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!desc_out) {
    dev_err(dev_data.dev, "Ciphertext TX prep_slave_sg() failed\n");
    ret = -EINVAL;
    goto aead_dma_prep_src_err;
    }
// Map ciphertext dst for RX (only if separate dst)
    if (diff_dst) {
    dst_nents = sg_nents_for_len(dst, cryptlen);
    dst_mapped_nents = dma_map_sg(rx_dev, dst, dst_nents, dst_dir);
    if (dst_mapped_nents == 0) {
    dev_err(dev_data.dev, "Failed to map ciphertext dst for RX\n");
    ret = -EINVAL;
    goto aead_dma_prep_src_err;
    }
    } else {
    dst_nents = src_nents;
    dst_mapped_nents = src_mapped_nents;
    }
// Prepare DMA descriptor for ciphertext RX
    desc_in = dmaengine_prep_slave_sg(dev_data.dma_aes_rx, dst,
    dst_mapped_nents, DMA_DEV_TO_MEM,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!desc_in) {
    dev_err(dev_data.dev, "Ciphertext RX prep_slave_sg() failed\n");
    ret = -EINVAL;
    goto aead_dma_prep_dst_err;
    }
    desc_in.callback = dthe_aead_dma_in_callback;
    desc_in.callback_param = req;
    } else if (assoclen != 0) {
// AAD-only operation
    desc_aad_out.callback = dthe_aead_dma_in_callback;
    desc_aad_out.callback_param = req;
    }
    init_completion(&rctx.aes_compl);
//
// HACK: There is an unknown hw issue where if the previous operation had alen = 0 and
// plen != 0, the current operation's tag calculation is incorrect in the case where
// plen = 0 and alen != 0 currently. This is a workaround for now which somehow works;
// by resetting the context by writing a 1 to the C_LENGTH_0 and AUTH_LENGTH registers.
//
    if (cryptlen == 0) {
    writel_relaxed(1, aes_base_reg + DTHE_P_AES_C_LENGTH_0);
    writel_relaxed(1, aes_base_reg + DTHE_P_AES_AUTH_LENGTH);
    }
    if (ctx.aes_mode == DTHE_AES_GCM) {
    if (req.iv) {
    memcpy(iv_in, req.iv, GCM_AES_IV_SIZE);
    } else {
    iv_in[0] = 0;
    iv_in[1] = 0;
    iv_in[2] = 0;
    }
    iv_in[3] = 0x01000000;
    } else {
    memcpy(iv_in, req.iv, AES_IV_SIZE);
    }
// Clear key2 to reset previous GHASH intermediate data
    for (int i = 0; i < AES_KEYSIZE_256 / sizeof(u32); ++i)
    writel_relaxed(0, aes_base_reg + DTHE_P_AES_KEY2_6 + DTHE_REG_SIZE * i);
    dthe_aes_set_ctrl_key(ctx, rctx, iv_in);
    writel_relaxed(lower_32_bits(unpadded_cryptlen), aes_base_reg + DTHE_P_AES_C_LENGTH_0);
    writel_relaxed(upper_32_bits(unpadded_cryptlen), aes_base_reg + DTHE_P_AES_C_LENGTH_1);
    writel_relaxed(req.assoclen, aes_base_reg + DTHE_P_AES_AUTH_LENGTH);
// Submit DMA descriptors: AAD TX, ciphertext TX, ciphertext RX
    if (assoclen != 0)
    dmaengine_submit(desc_aad_out);
    if (cryptlen != 0) {
    dmaengine_submit(desc_out);
    dmaengine_submit(desc_in);
    }
    if (cryptlen != 0)
    dma_async_issue_pending(dev_data.dma_aes_rx);
    dma_async_issue_pending(dev_data.dma_aes_tx);
// Need to do timeout to ensure finalise gets called if DMA callback fails for any reason
    ret = wait_for_completion_timeout(&rctx.aes_compl, msecs_to_jiffies(DTHE_DMA_TIMEOUT_MS));
    if (!ret) {
    ret = -ETIMEDOUT;
    if (cryptlen != 0)
    dmaengine_terminate_sync(dev_data.dma_aes_rx);
    dmaengine_terminate_sync(dev_data.dma_aes_tx);
    for (int i = 0; i < AES_BLOCK_WORDS; ++i)
    readl_relaxed(aes_base_reg + DTHE_P_AES_DATA_IN_OUT + DTHE_REG_SIZE * i);
    } else {
    ret = 0;
    }
    if (cryptlen != 0)
    dma_sync_sg_for_cpu(rx_dev, dst, dst_nents, dst_dir);
    if (rctx.enc)
    err = dthe_aead_enc_get_tag(req);
    else
    err = dthe_aead_dec_verify_tag(req);
    ret = (ret) ? ret : err;
    aead_dma_prep_dst_err:
    if (diff_dst && cryptlen != 0)
    dma_unmap_sg(rx_dev, dst, dst_nents, dst_dir);
    aead_dma_prep_src_err:
    if (cryptlen != 0)
    dma_unmap_sg(tx_dev, src, src_nents, src_dir);
    aead_dma_prep_aad_err:
    if (assoclen != 0)
    dma_unmap_sg(tx_dev, aad_sg, aad_nents, aad_dir);
    aead_dma_map_aad_err:
    if (diff_dst && cryptlen != 0)
    kfree(dst);
    aead_prep_dst_err:
    if (cryptlen != 0)
    kfree(src);
    aead_prep_src_err:
    if (assoclen != 0)
    kfree(aad_sg);
    aead_prep_aad_err:
    memzero_explicit(rctx.padding, 2 * AES_BLOCK_SIZE);
    if (ret)
    ret = dthe_aead_do_fallback(req);
    local_bh_disable();
    crypto_finalize_aead_request(engine, req, ret);
    local_bh_enable();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dthe_aead_crypt(req: *mut aead_request) -> c_int {
    static int dthe_aead_crypt(struct aead_request *req)
    {
    struct dthe_tfm_ctx *ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    struct dthe_aes_req_ctx *rctx = aead_request_ctx(req);
    struct dthe_data *dev_data = dthe_get_dev(ctx);
    struct crypto_engine *engine;
    let mut cryptlen: c_uint = req.cryptlen;
    let mut is_zero_ctr: bool = true;
// In decryption, last authsize bytes are the TAG
    if (!rctx.enc)
    cryptlen -= ctx.authsize;
    if (ctx.aes_mode == DTHE_AES_CCM) {
//
// For CCM Mode, the 128-bit IV contains the following:
// | 0 .. 2 | 3 .. 7 | 8 .. (127-8*L) | (128-8*L) .. 127 |
// |   L-1  |  Zero  |     Nonce      |      Counter     |
// L needs to be between 2-8 (inclusive), i.e. 1 <= (L-1) <= 7
// and the next 5 bits need to be zeroes. Else return -EINVAL
//
    u8 *iv = req.iv;
    let mut L: u8 = iv[0];
// variable L stores L-1 here
    if (L < 1 || L > 7)
    return -EINVAL;
//
// DTHEv2 HW can only work with zero initial counter in CCM mode.
// Check if the initial counter value is zero or not
//
    for (int i = 0; i < L + 1; ++i) {
    if (iv[AES_IV_SIZE - 1 - i] != 0) {
    is_zero_ctr = false;
    break;
    }
    }
    }
//
// Need to fallback to software in the following cases due to HW restrictions:
// - Both AAD and plaintext/ciphertext are zero length
// - For AES-GCM, AAD length is more than 2^32 - 1 bytes
// - For AES-CCM, AAD length is more than 2^16 - 2^8 bytes
// - For AES-CCM, plaintext/ciphertext length is more than 2^61 - 1 bytes
// - For AES-CCM, AAD length is non-zero but plaintext/ciphertext length is zero
// - For AES-CCM, the initial counter (last L+1 bytes of IV) is not all zeroes
//
// PS: req->cryptlen is currently unsigned int type, which causes the second and fourth
// cases above tautologically false. If req->cryptlen is to be changed to a 64-bit
// type, the check for these would also need to be added below.
//
    if ((req.assoclen == 0 && cryptlen == 0) ||
    (ctx.aes_mode == DTHE_AES_CCM && req.assoclen > DTHE_AES_CCM_AAD_MAXLEN) ||
    (ctx.aes_mode == DTHE_AES_CCM && cryptlen == 0) ||
    (ctx.aes_mode == DTHE_AES_CCM && !is_zero_ctr))
    return dthe_aead_do_fallback(req);
    engine = dev_data.engine;
    return crypto_transfer_aead_request_to_engine(engine, req);
    }
#[no_mangle]
unsafe extern "C" fn dthe_aead_encrypt(req: *mut aead_request) -> c_int {
    static int dthe_aead_encrypt(struct aead_request *req)
    {
    struct dthe_aes_req_ctx *rctx = aead_request_ctx(req);
    rctx.enc = 1;
    return dthe_aead_crypt(req);
    }
#[no_mangle]
unsafe extern "C" fn dthe_aead_decrypt(req: *mut aead_request) -> c_int {
    static int dthe_aead_decrypt(struct aead_request *req)
    {
    struct dthe_aes_req_ctx *rctx = aead_request_ctx(req);
    rctx.enc = 0;
    return dthe_aead_crypt(req);
    }
    static struct skcipher_engine_alg cipher_algs[] = {
    {
    .base.init			= dthe_cipher_init_tfm,
    .base.setkey			= dthe_aes_ecb_setkey,
    .base.encrypt			= dthe_aes_encrypt,
    .base.decrypt			= dthe_aes_decrypt,
    .base.min_keysize		= AES_MIN_KEY_SIZE,
    .base.max_keysize		= AES_MAX_KEY_SIZE,
    .base.base = {
    .cra_name		= "ecb(aes)",
    .cra_driver_name	= "ecb-aes-dthev2",
    .cra_priority		= 299,
    .cra_flags		= CRYPTO_ALG_TYPE_SKCIPHER |
    CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_KERN_DRIVER_ONLY,
    .cra_alignmask		= AES_BLOCK_SIZE - 1,
    .cra_blocksize		= AES_BLOCK_SIZE,
    .cra_ctxsize		= sizeof(struct dthe_tfm_ctx),
    .cra_reqsize		= sizeof(struct dthe_aes_req_ctx),
    .cra_module		= THIS_MODULE,
    },
    .op.do_one_request = dthe_aes_run,
    }, /* ECB AES */
    {
    .base.init			= dthe_cipher_init_tfm,
    .base.setkey			= dthe_aes_cbc_setkey,
    .base.encrypt			= dthe_aes_encrypt,
    .base.decrypt			= dthe_aes_decrypt,
    .base.min_keysize		= AES_MIN_KEY_SIZE,
    .base.max_keysize		= AES_MAX_KEY_SIZE,
    .base.ivsize			= AES_IV_SIZE,
    .base.base = {
    .cra_name		= "cbc(aes)",
    .cra_driver_name	= "cbc-aes-dthev2",
    .cra_priority		= 299,
    .cra_flags		= CRYPTO_ALG_TYPE_SKCIPHER |
    CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_KERN_DRIVER_ONLY,
    .cra_alignmask		= AES_BLOCK_SIZE - 1,
    .cra_blocksize		= AES_BLOCK_SIZE,
    .cra_ctxsize		= sizeof(struct dthe_tfm_ctx),
    .cra_reqsize		= sizeof(struct dthe_aes_req_ctx),
    .cra_module		= THIS_MODULE,
    },
    .op.do_one_request = dthe_aes_run,
    }, /* CBC AES */
    {
    .base.init			= dthe_cipher_init_tfm_fallback,
    .base.exit			= dthe_cipher_exit_tfm,
    .base.setkey			= dthe_aes_ctr_setkey,
    .base.encrypt			= dthe_aes_encrypt,
    .base.decrypt			= dthe_aes_decrypt,
    .base.min_keysize		= AES_MIN_KEY_SIZE,
    .base.max_keysize		= AES_MAX_KEY_SIZE,
    .base.ivsize			= AES_IV_SIZE,
    .base.chunksize			= AES_BLOCK_SIZE,
    .base.base = {
    .cra_name		= "ctr(aes)",
    .cra_driver_name	= "ctr-aes-dthev2",
    .cra_priority		= 299,
    .cra_flags		= CRYPTO_ALG_TYPE_SKCIPHER |
    CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_KERN_DRIVER_ONLY |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_blocksize		= 1,
    .cra_ctxsize		= sizeof(struct dthe_tfm_ctx),
    .cra_reqsize		= sizeof(struct dthe_aes_req_ctx),
    .cra_module		= THIS_MODULE,
    },
    .op.do_one_request = dthe_aes_run,
    }, /* CTR AES */
    {
    .base.init			= dthe_cipher_init_tfm_fallback,
    .base.exit			= dthe_cipher_exit_tfm,
    .base.setkey			= dthe_aes_xts_setkey,
    .base.encrypt			= dthe_aes_encrypt,
    .base.decrypt			= dthe_aes_decrypt,
    .base.min_keysize		= AES_MIN_KEY_SIZE * 2,
    .base.max_keysize		= AES_MAX_KEY_SIZE * 2,
    .base.ivsize			= AES_IV_SIZE,
    .base.base = {
    .cra_name		= "xts(aes)",
    .cra_driver_name	= "xts-aes-dthev2",
    .cra_priority		= 299,
    .cra_flags		= CRYPTO_ALG_TYPE_SKCIPHER |
    CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_KERN_DRIVER_ONLY |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_alignmask		= AES_BLOCK_SIZE - 1,
    .cra_blocksize		= AES_BLOCK_SIZE,
    .cra_ctxsize		= sizeof(struct dthe_tfm_ctx),
    .cra_reqsize		= sizeof(struct dthe_aes_req_ctx),
    .cra_module		= THIS_MODULE,
    },
    .op.do_one_request = dthe_aes_run,
    }, /* XTS AES */
    };
    static struct aead_engine_alg aead_algs[] = {
    {
    .base.init			= dthe_aead_init_tfm,
    .base.exit			= dthe_aead_exit_tfm,
    .base.setkey			= dthe_gcm_aes_setkey,
    .base.setauthsize		= dthe_aead_setauthsize,
    .base.maxauthsize		= AES_BLOCK_SIZE,
    .base.encrypt			= dthe_aead_encrypt,
    .base.decrypt			= dthe_aead_decrypt,
    .base.chunksize			= AES_BLOCK_SIZE,
    .base.ivsize			= GCM_AES_IV_SIZE,
    .base.base = {
    .cra_name		= "gcm(aes)",
    .cra_driver_name	= "gcm-aes-dthev2",
    .cra_priority		= 299,
    .cra_flags		= CRYPTO_ALG_TYPE_AEAD |
    CRYPTO_ALG_KERN_DRIVER_ONLY |
    CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_blocksize		= 1,
    .cra_ctxsize		= sizeof(struct dthe_tfm_ctx),
    .cra_reqsize		= sizeof(struct dthe_aes_req_ctx),
    .cra_module		= THIS_MODULE,
    },
    .op.do_one_request = dthe_aead_run,
    }, /* GCM AES */
    {
    .base.init			= dthe_aead_init_tfm,
    .base.exit			= dthe_aead_exit_tfm,
    .base.setkey			= dthe_ccm_aes_setkey,
    .base.setauthsize		= dthe_aead_setauthsize,
    .base.maxauthsize		= AES_BLOCK_SIZE,
    .base.encrypt			= dthe_aead_encrypt,
    .base.decrypt			= dthe_aead_decrypt,
    .base.chunksize			= AES_BLOCK_SIZE,
    .base.ivsize			= AES_IV_SIZE,
    .base.base = {
    .cra_name		= "ccm(aes)",
    .cra_driver_name	= "ccm-aes-dthev2",
    .cra_priority		= 299,
    .cra_flags		= CRYPTO_ALG_TYPE_AEAD |
    CRYPTO_ALG_KERN_DRIVER_ONLY |
    CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_blocksize		= 1,
    .cra_ctxsize		= sizeof(struct dthe_tfm_ctx),
    .cra_reqsize		= sizeof(struct dthe_aes_req_ctx),
    .cra_module		= THIS_MODULE,
    },
    .op.do_one_request = dthe_aead_run,
    }, /* CCM AES */
    };
#[no_mangle]
pub unsafe extern "C" fn dthe_register_aes_algs() -> c_int {
    int dthe_register_aes_algs(void)
    {
    let mut ret: c_int = 0;
    ret = crypto_engine_register_skciphers(cipher_algs, ARRAY_SIZE(cipher_algs));
    if (ret)
    return ret;
    ret = crypto_engine_register_aeads(aead_algs, ARRAY_SIZE(aead_algs));
    if (ret)
    crypto_engine_unregister_skciphers(cipher_algs, ARRAY_SIZE(cipher_algs));
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dthe_unregister_aes_algs() {
    void dthe_unregister_aes_algs(void)
    {
    crypto_engine_unregister_skciphers(cipher_algs, ARRAY_SIZE(cipher_algs));
    crypto_engine_unregister_aeads(aead_algs, ARRAY_SIZE(aead_algs));
    }
