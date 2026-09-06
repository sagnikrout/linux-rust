//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/sahara.c
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
// Support for SAHARA cryptographic accelerator.
//
// Copyright (c) 2014 Steffen Trumtrar <s.trumtrar@pengutronix.de>
// Copyright (c) 2013 Vista Silicon S.L.
// Author: Javier Martin <javier.martin@vista-silicon.com>
//
// Based on omap-aes.c and tegra-aes.c
//

pub const SAHARA_VERSION_3: c_int = 3;
pub const SAHARA_VERSION_4: c_int = 4;
pub const SAHARA_TIMEOUT_MS: c_int = 1000;
pub const SAHARA_MAX_HW_DESC: c_int = 2;
pub const SAHARA_MAX_HW_LINK: c_int = 20;
pub const FLAGS_MODE_MASK: c_uint = 0x000f;

pub const SAHARA_HDR_BASE: c_uint = 0x00800000;
pub const SAHARA_HDR_SKHA_ALG_AES: c_int = 0;
pub const SAHARA_HDR_SKHA_MODE_ECB: c_int = 0;

pub const SAHARA_HDR_MDHA_SET_MODE_MD_KEY: c_uint = 0x20880000;
pub const SAHARA_HDR_MDHA_SET_MODE_HASH: c_uint = 0x208D0000;
pub const SAHARA_HDR_MDHA_HASH: c_uint = 0xA0850000;
pub const SAHARA_HDR_MDHA_STORE_DIGEST: c_uint = 0x20820000;
pub const SAHARA_HDR_MDHA_ALG_SHA1: c_int = 0;
pub const SAHARA_HDR_MDHA_ALG_MD5: c_int = 1;
pub const SAHARA_HDR_MDHA_ALG_SHA256: c_int = 2;
pub const SAHARA_HDR_MDHA_ALG_SHA224: c_int = 3;

pub const SAHARA_REG_VERSION: c_uint = 0x00;
pub const SAHARA_REG_DAR: c_uint = 0x04;
pub const SAHARA_REG_CONTROL: c_uint = 0x08;

pub const SAHARA_REG_CMD: c_uint = 0x0C;

pub const SAHARA_REG_STATUS: c_uint = 0x10;

pub const SAHARA_STATE_IDLE: c_int = 0;
pub const SAHARA_STATE_BUSY: c_int = 1;
pub const SAHARA_STATE_ERR: c_int = 2;
pub const SAHARA_STATE_FAULT: c_int = 3;
pub const SAHARA_STATE_COMPLETE: c_int = 4;

pub const SAHARA_REG_ERRSTATUS: c_uint = 0x14;

pub const SAHARA_ERRSOURCE_CHA: c_int = 14;
pub const SAHARA_ERRSOURCE_DMA: c_int = 15;

pub const SAHARA_REG_FADDR: c_uint = 0x18;
pub const SAHARA_REG_CDAR: c_uint = 0x1C;
pub const SAHARA_REG_IDAR: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sahara_hw_desc {
    pub hdr: u32,
    pub len1: u32,
    pub p1: u32,
    pub len2: u32,
    pub p2: u32,
    pub next: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sahara_hw_link {
    pub len: u32,
    pub p: u32,
    pub next: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sahara_ctx {
// AES-specific context
    pub keylen: c_int,
    pub key: [u8; AES_KEYSIZE_128],
    pub fallback: *mut crypto_skcipher,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sahara_aes_reqctx {
    pub mode: c_ulong,
    pub iv_out: [u8; AES_BLOCK_SIZE],
    pub end: skcipher_request fallback_req; // keep at the,
}

//
// struct sahara_sha_reqctx - private data per request
// @buf: holds data for requests smaller than block_size
// @rembuf: used to prepare one block_size-aligned request
// @context: hw-specific context for request. Digest is extracted from this
// @mode: specifies what type of hw-descriptor needs to be built
// @digest_size: length of digest for this request
// @context_size: length of hw-context for this request.
// Always digest_size + 4
// @buf_cnt: number of bytes saved in buf
// @sg_in_idx: number of hw links
// @in_sg: scatterlist for input data
// @in_sg_chain: scatterlists for chained input data
// @total: total number of bytes for transfer
// @last: is this the last block
// @first: is this the first block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sahara_sha_reqctx {
    pub buf: [u8; SAHARA_MAX_SHA_BLOCK_SIZE],
    pub rembuf: [u8; SAHARA_MAX_SHA_BLOCK_SIZE],
    pub 4]: u8 context[SHA256_DIGEST_SIZE +,
    pub mode: c_uint,
    pub digest_size: c_uint,
    pub context_size: c_uint,
    pub buf_cnt: c_uint,
    pub sg_in_idx: c_uint,
    pub in_sg: *mut scatterlist,
    pub in_sg_chain: [scatterlist; 2],
    pub total: usize,
    pub last: c_uint,
    pub first: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sahara_dev {
    pub device: *mut device,
    pub version: c_uint,
    pub regs_base: *mut void __iomem,
    pub clk_ipg: *mut clk,
    pub clk_ahb: *mut clk,
    pub dma_completion: completion,
    pub ctx: *mut sahara_ctx,
    pub flags: c_ulong,
    pub hw_desc: [*mut sahara_hw_desc; SAHARA_MAX_HW_DESC],
    pub hw_phys_desc: [dma_addr_t; SAHARA_MAX_HW_DESC],
    pub key_base: *mut u8,
    pub key_phys_base: dma_addr_t,
    pub iv_base: *mut u8,
    pub iv_phys_base: dma_addr_t,
    pub context_base: *mut u8,
    pub context_phys_base: dma_addr_t,
    pub hw_link: [*mut sahara_hw_link; SAHARA_MAX_HW_LINK],
    pub hw_phys_link: [dma_addr_t; SAHARA_MAX_HW_LINK],
    pub total: usize,
    pub in_sg: *mut scatterlist,
    pub nb_in_sg: c_int,
    pub out_sg: *mut scatterlist,
    pub nb_out_sg: c_int,
    pub engine: *mut crypto_engine,
}

    static struct sahara_dev *dev_ptr;
#[no_mangle]
pub unsafe extern "C" fn sahara_write(dev: *mut sahara_dev, data: u32, reg: u32) {
    static inline void sahara_write(struct sahara_dev *dev, u32 data, u32 reg)
    {
    writel(data, dev.regs_base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn sahara_read(dev: *mut sahara_dev, reg: u32) -> c_uint {
    static inline unsigned int sahara_read(struct sahara_dev *dev, u32 reg)
    {
    return readl(dev.regs_base + reg);
    }
#[no_mangle]
unsafe extern "C" fn sahara_aes_key_hdr(dev: *mut sahara_dev) -> u32 {
    static u32 sahara_aes_key_hdr(struct sahara_dev *dev)
    {
    u32 hdr = SAHARA_HDR_BASE | SAHARA_HDR_SKHA_ALG_AES |
    SAHARA_HDR_FORM_KEY | SAHARA_HDR_LLO |
    SAHARA_HDR_CHA_SKHA | SAHARA_HDR_PARITY_BIT;
    if (dev.flags & FLAGS_CBC) {
    hdr |= SAHARA_HDR_SKHA_MODE_CBC;
    hdr ^= SAHARA_HDR_PARITY_BIT;
    }
    if (dev.flags & FLAGS_ENCRYPT) {
    hdr |= SAHARA_HDR_SKHA_OP_ENC;
    hdr ^= SAHARA_HDR_PARITY_BIT;
    }
    return hdr;
    }
#[no_mangle]
unsafe extern "C" fn sahara_aes_data_link_hdr(dev: *mut sahara_dev) -> u32 {
    static u32 sahara_aes_data_link_hdr(struct sahara_dev *dev)
    {
    return SAHARA_HDR_BASE | SAHARA_HDR_FORM_DATA |
    SAHARA_HDR_CHA_SKHA | SAHARA_HDR_PARITY_BIT;
    }
    static const char *sahara_err_src[16] = {
    "No error",
    "Header error",
    "Descriptor length error",
    "Descriptor length or pointer error",
    "Link length error",
    "Link pointer error",
    "Input buffer error",
    "Output buffer error",
    "Output buffer starvation",
    "Internal state fault",
    "General descriptor problem",
    "Reserved",
    "Descriptor address error",
    "Link address error",
    "CHA error",
    "DMA error"
    };
    static const char *sahara_err_dmasize[4] = {
    "Byte transfer",
    "Half-word transfer",
    "Word transfer",
    "Reserved"
    };
    static const char *sahara_err_dmasrc[8] = {
    "No error",
    "AHB bus error",
    "Internal IP bus error",
    "Parity error",
    "DMA crosses 256 byte boundary",
    "DMA is busy",
    "Reserved",
    "DMA HW error"
    };
    static const char *sahara_cha_errsrc[12] = {
    "Input buffer non-empty",
    "Illegal address",
    "Illegal mode",
    "Illegal data size",
    "Illegal key size",
    "Write during processing",
    "CTX read during processing",
    "HW error",
    "Input buffer disabled/underflow",
    "Output buffer disabled/overflow",
    "DES key parity error",
    "Reserved"
    };
    static const char *sahara_cha_err[4] = { "No error", "SKHA", "MDHA", "RNG" };
#[no_mangle]
unsafe extern "C" fn sahara_decode_error(dev: *mut sahara_dev, error: c_uint) {
    static void sahara_decode_error(struct sahara_dev *dev, unsigned int error)
    {
    let mut source: u8 = SAHARA_ERRSTATUS_GET_SOURCE(error);
    let mut chasrc: u16 = ffs(SAHARA_ERRSTATUS_GET_CHASRC(error));
    dev_err(dev.device, "%s: Error Register = 0x%08x\n", __func__, error);
    dev_err(dev.device, "	- %s.\n", sahara_err_src[source]);
    if (source == SAHARA_ERRSOURCE_DMA) {
    if (error & SAHARA_ERRSTATUS_DMA_DIR)
    dev_err(dev.device, "		* DMA read.\n");
    else
    dev_err(dev.device, "		* DMA write.\n");
    dev_err(dev.device, "		* %s.\n",
    sahara_err_dmasize[SAHARA_ERRSTATUS_GET_DMASZ(error)]);
    dev_err(dev.device, "		* %s.\n",
    sahara_err_dmasrc[SAHARA_ERRSTATUS_GET_DMASRC(error)]);
    } else if (source == SAHARA_ERRSOURCE_CHA) {
    dev_err(dev.device, "		* %s.\n",
    sahara_cha_errsrc[chasrc]);
    dev_err(dev.device, "		* %s.\n",
    sahara_cha_err[SAHARA_ERRSTATUS_GET_CHAERR(error)]);
    }
    dev_err(dev.device, "\n");
    }
    static const char *sahara_state[4] = { "Idle", "Busy", "Error", "HW Fault" };
#[no_mangle]
unsafe extern "C" fn sahara_decode_status(dev: *mut sahara_dev, status: c_uint) {
    static void sahara_decode_status(struct sahara_dev *dev, unsigned int status)
    {
    u8 state;
    if (!__is_defined(DEBUG))
    return;
    state = SAHARA_STATUS_GET_STATE(status);
    dev_dbg(dev.device, "%s: Status Register = 0x%08x\n",
    __func__, status);
    dev_dbg(dev.device, "	- State = %d:\n", state);
    if (state & SAHARA_STATE_COMP_FLAG)
    dev_dbg(dev.device, "		* Descriptor completed. IRQ pending.\n");
    dev_dbg(dev.device, "		* %s.\n",
    sahara_state[state & ~SAHARA_STATE_COMP_FLAG]);
    if (status & SAHARA_STATUS_DAR_FULL)
    dev_dbg(dev.device, "	- DAR Full.\n");
    if (status & SAHARA_STATUS_ERROR)
    dev_dbg(dev.device, "	- Error.\n");
    if (status & SAHARA_STATUS_SECURE)
    dev_dbg(dev.device, "	- Secure.\n");
    if (status & SAHARA_STATUS_FAIL)
    dev_dbg(dev.device, "	- Fail.\n");
    if (status & SAHARA_STATUS_RNG_RESEED)
    dev_dbg(dev.device, "	- RNG Reseed Request.\n");
    if (status & SAHARA_STATUS_ACTIVE_RNG)
    dev_dbg(dev.device, "	- RNG Active.\n");
    if (status & SAHARA_STATUS_ACTIVE_MDHA)
    dev_dbg(dev.device, "	- MDHA Active.\n");
    if (status & SAHARA_STATUS_ACTIVE_SKHA)
    dev_dbg(dev.device, "	- SKHA Active.\n");
    if (status & SAHARA_STATUS_MODE_BATCH)
    dev_dbg(dev.device, "	- Batch Mode.\n");
#[no_mangle]
pub unsafe extern "C" fn if(SAHARA_STATUS_MODE_DEDICATED: status &) -> else {
    else if (status & SAHARA_STATUS_MODE_DEDICATED)
    dev_dbg(dev.device, "	- Dedicated Mode.\n");
#[no_mangle]
pub unsafe extern "C" fn if(SAHARA_STATUS_MODE_DEBUG: status &) -> else {
    else if (status & SAHARA_STATUS_MODE_DEBUG)
    dev_dbg(dev.device, "	- Debug Mode.\n");
    dev_dbg(dev.device, "	- Internal state = 0x%02x\n",
    SAHARA_STATUS_GET_ISTATE(status));
    dev_dbg(dev.device, "Current DAR: 0x%08x\n",
    sahara_read(dev, SAHARA_REG_CDAR));
    dev_dbg(dev.device, "Initial DAR: 0x%08x\n\n",
    sahara_read(dev, SAHARA_REG_IDAR));
    }
#[no_mangle]
unsafe extern "C" fn sahara_dump_descriptors(dev: *mut sahara_dev) {
    static void sahara_dump_descriptors(struct sahara_dev *dev)
    {
    int i;
    if (!__is_defined(DEBUG))
    return;
    for (i = 0; i < SAHARA_MAX_HW_DESC; i++) {
    dev_dbg(dev.device, "Descriptor (%d) (%pad):\n",
    i, &dev.hw_phys_desc[i]);
    dev_dbg(dev.device, "\thdr = 0x%08x\n", dev.hw_desc[i].hdr);
    dev_dbg(dev.device, "\tlen1 = %u\n", dev.hw_desc[i].len1);
    dev_dbg(dev.device, "\tp1 = 0x%08x\n", dev.hw_desc[i].p1);
    dev_dbg(dev.device, "\tlen2 = %u\n", dev.hw_desc[i].len2);
    dev_dbg(dev.device, "\tp2 = 0x%08x\n", dev.hw_desc[i].p2);
    dev_dbg(dev.device, "\tnext = 0x%08x\n",
    dev.hw_desc[i].next);
    }
    dev_dbg(dev.device, "\n");
    }
#[no_mangle]
unsafe extern "C" fn sahara_dump_links(dev: *mut sahara_dev) {
    static void sahara_dump_links(struct sahara_dev *dev)
    {
    int i;
    if (!__is_defined(DEBUG))
    return;
    for (i = 0; i < SAHARA_MAX_HW_LINK; i++) {
    dev_dbg(dev.device, "Link (%d) (%pad):\n",
    i, &dev.hw_phys_link[i]);
    dev_dbg(dev.device, "\tlen = %u\n", dev.hw_link[i].len);
    dev_dbg(dev.device, "\tp = 0x%08x\n", dev.hw_link[i].p);
    dev_dbg(dev.device, "\tnext = 0x%08x\n",
    dev.hw_link[i].next);
    }
    dev_dbg(dev.device, "\n");
    }
#[no_mangle]
unsafe extern "C" fn sahara_hw_descriptor_create(dev: *mut sahara_dev) -> c_int {
    static int sahara_hw_descriptor_create(struct sahara_dev *dev)
    {
    struct sahara_ctx *ctx = dev.ctx;
    struct scatterlist *sg;
    int ret;
    int i, j;
    let mut idx: c_int = 0;
    u32 len;
    memcpy(dev.key_base, ctx.key, ctx.keylen);
    if (dev.flags & FLAGS_CBC) {
    dev.hw_desc[idx].len1 = AES_BLOCK_SIZE;
    dev.hw_desc[idx].p1 = dev.iv_phys_base;
    } else {
    dev.hw_desc[idx].len1 = 0;
    dev.hw_desc[idx].p1 = 0;
    }
    dev.hw_desc[idx].len2 = ctx.keylen;
    dev.hw_desc[idx].p2 = dev.key_phys_base;
    dev.hw_desc[idx].next = dev.hw_phys_desc[1];
    dev.hw_desc[idx].hdr = sahara_aes_key_hdr(dev);
    idx++;
    dev.nb_in_sg = sg_nents_for_len(dev.in_sg, dev.total);
    if (dev.nb_in_sg < 0) {
    dev_err(dev.device, "Invalid numbers of src SG.\n");
    return dev.nb_in_sg;
    }
    dev.nb_out_sg = sg_nents_for_len(dev.out_sg, dev.total);
    if (dev.nb_out_sg < 0) {
    dev_err(dev.device, "Invalid numbers of dst SG.\n");
    return dev.nb_out_sg;
    }
    if ((dev.nb_in_sg + dev.nb_out_sg) > SAHARA_MAX_HW_LINK) {
    dev_err(dev.device, "not enough hw links (%d)\n",
    dev.nb_in_sg + dev.nb_out_sg);
    return -EINVAL;
    }
    ret = dma_map_sg(dev.device, dev.in_sg, dev.nb_in_sg,
    DMA_TO_DEVICE);
    if (!ret) {
    dev_err(dev.device, "couldn't map in sg\n");
    return -EINVAL;
    }
    ret = dma_map_sg(dev.device, dev.out_sg, dev.nb_out_sg,
    DMA_FROM_DEVICE);
    if (!ret) {
    dev_err(dev.device, "couldn't map out sg\n");
    goto unmap_in;
    }
// Create input links
    dev.hw_desc[idx].p1 = dev.hw_phys_link[0];
    sg = dev.in_sg;
    len = dev.total;
    for (i = 0; i < dev.nb_in_sg; i++) {
    dev.hw_link[i].len = min(len, sg.length);
    dev.hw_link[i].p = sg.dma_address;
    if (i == (dev.nb_in_sg - 1)) {
    dev.hw_link[i].next = 0;
    } else {
    len -= min(len, sg.length);
    dev.hw_link[i].next = dev.hw_phys_link[i + 1];
    sg = sg_next(sg);
    }
    }
// Create output links
    dev.hw_desc[idx].p2 = dev.hw_phys_link[i];
    sg = dev.out_sg;
    len = dev.total;
    for (j = i; j < dev.nb_out_sg + i; j++) {
    dev.hw_link[j].len = min(len, sg.length);
    dev.hw_link[j].p = sg.dma_address;
    if (j == (dev.nb_out_sg + i - 1)) {
    dev.hw_link[j].next = 0;
    } else {
    len -= min(len, sg.length);
    dev.hw_link[j].next = dev.hw_phys_link[j + 1];
    sg = sg_next(sg);
    }
    }
// Fill remaining fields of hw_desc[1]
    dev.hw_desc[idx].hdr = sahara_aes_data_link_hdr(dev);
    dev.hw_desc[idx].len1 = dev.total;
    dev.hw_desc[idx].len2 = dev.total;
    dev.hw_desc[idx].next = 0;
    sahara_dump_descriptors(dev);
    sahara_dump_links(dev);
    sahara_write(dev, dev.hw_phys_desc[0], SAHARA_REG_DAR);
    return 0;
    unmap_in:
    dma_unmap_sg(dev.device, dev.in_sg, dev.nb_in_sg,
    DMA_TO_DEVICE);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn sahara_aes_cbc_update_iv(req: *mut skcipher_request) {
    static void sahara_aes_cbc_update_iv(struct skcipher_request *req)
    {
    struct crypto_skcipher *skcipher = crypto_skcipher_reqtfm(req);
    struct sahara_aes_reqctx *rctx = skcipher_request_ctx(req);
    let mut ivsize: c_uint = crypto_skcipher_ivsize(skcipher);
// Update IV buffer to contain the last ciphertext block
    if (rctx.mode & FLAGS_ENCRYPT) {
    sg_pcopy_to_buffer(req.dst, sg_nents(req.dst), req.iv,
    ivsize, req.cryptlen - ivsize);
    } else {
    memcpy(req.iv, rctx.iv_out, ivsize);
    }
    }
#[no_mangle]
unsafe extern "C" fn sahara_aes_process(req: *mut skcipher_request) -> c_int {
    static int sahara_aes_process(struct skcipher_request *req)
    {
    struct crypto_skcipher *skcipher = crypto_skcipher_reqtfm(req);
    struct sahara_dev *dev = dev_ptr;
    struct sahara_ctx *ctx;
    struct sahara_aes_reqctx *rctx;
    int ret;
    unsigned long time_left;
// Request is ready to be dispatched by the device
    dev_dbg(dev.device,
    "dispatch request (nbytes=%d, src=%p, dst=%p)\n",
    req.cryptlen, req.src, req.dst);
// assign new request to device
    dev.total = req.cryptlen;
    dev.in_sg = req.src;
    dev.out_sg = req.dst;
    rctx = skcipher_request_ctx(req);
    ctx = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req));
    rctx.mode &= FLAGS_MODE_MASK;
    dev.flags = (dev.flags & ~FLAGS_MODE_MASK) | rctx.mode;
    if ((dev.flags & FLAGS_CBC) && req.iv) {
    let mut ivsize: c_uint = crypto_skcipher_ivsize(skcipher);
    memcpy(dev.iv_base, req.iv, ivsize);
    if (!(dev.flags & FLAGS_ENCRYPT)) {
    sg_pcopy_to_buffer(req.src, sg_nents(req.src),
    rctx.iv_out, ivsize,
    req.cryptlen - ivsize);
    }
    }
// assign new context to device
    dev.ctx = ctx;
    reinit_completion(&dev.dma_completion);
    ret = sahara_hw_descriptor_create(dev);
    if (ret)
    return -EINVAL;
    time_left = wait_for_completion_timeout(&dev.dma_completion,
    msecs_to_jiffies(SAHARA_TIMEOUT_MS));
    dma_unmap_sg(dev.device, dev.out_sg, dev.nb_out_sg,
    DMA_FROM_DEVICE);
    dma_unmap_sg(dev.device, dev.in_sg, dev.nb_in_sg,
    DMA_TO_DEVICE);
    if (!time_left) {
    dev_err(dev.device, "AES timeout\n");
    return -ETIMEDOUT;
    }
    if ((dev.flags & FLAGS_CBC) && req.iv)
    sahara_aes_cbc_update_iv(req);
    return 0;
    }
    static int sahara_aes_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int keylen)
    {
    struct sahara_ctx *ctx = crypto_skcipher_ctx(tfm);
    ctx.keylen = keylen;
// SAHARA only supports 128bit keys
    if (keylen == AES_KEYSIZE_128) {
    memcpy(ctx.key, key, keylen);
    return 0;
    }
    if (keylen != AES_KEYSIZE_192 && keylen != AES_KEYSIZE_256)
    return -EINVAL;
//
// The requested key size is not supported by HW, do a fallback.
//
    crypto_skcipher_clear_flags(ctx.fallback, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags(ctx.fallback, tfm.base.crt_flags &
    CRYPTO_TFM_REQ_MASK);
    return crypto_skcipher_setkey(ctx.fallback, key, keylen);
    }
#[no_mangle]
unsafe extern "C" fn sahara_aes_fallback(req: *mut skcipher_request, mode: c_ulong) -> c_int {
    static int sahara_aes_fallback(struct skcipher_request *req, unsigned long mode)
    {
    struct sahara_aes_reqctx *rctx = skcipher_request_ctx(req);
    struct sahara_ctx *ctx = crypto_skcipher_ctx(
    crypto_skcipher_reqtfm(req));
    skcipher_request_set_tfm(&rctx.fallback_req, ctx.fallback);
    skcipher_request_set_callback(&rctx.fallback_req,
    req.base.flags,
    req.base.complete,
    req.base.data);
    skcipher_request_set_crypt(&rctx.fallback_req, req.src,
    req.dst, req.cryptlen, req.iv);
    if (mode & FLAGS_ENCRYPT)
    return crypto_skcipher_encrypt(&rctx.fallback_req);
    return crypto_skcipher_decrypt(&rctx.fallback_req);
    }
#[no_mangle]
unsafe extern "C" fn sahara_aes_crypt(req: *mut skcipher_request, mode: c_ulong) -> c_int {
    static int sahara_aes_crypt(struct skcipher_request *req, unsigned long mode)
    {
    struct sahara_aes_reqctx *rctx = skcipher_request_ctx(req);
    struct sahara_ctx *ctx = crypto_skcipher_ctx(
    crypto_skcipher_reqtfm(req));
    struct sahara_dev *dev = dev_ptr;
    if (!req.cryptlen)
    return 0;
    if (unlikely(ctx.keylen != AES_KEYSIZE_128))
    return sahara_aes_fallback(req, mode);
    dev_dbg(dev.device, "nbytes: %d, enc: %d, cbc: %d\n",
    req.cryptlen, !!(mode & FLAGS_ENCRYPT), !!(mode & FLAGS_CBC));
    if (!IS_ALIGNED(req.cryptlen, AES_BLOCK_SIZE))
    return -EINVAL;
    rctx.mode = mode;
    return crypto_transfer_skcipher_request_to_engine(dev.engine, req);
    }
#[no_mangle]
unsafe extern "C" fn sahara_aes_ecb_encrypt(req: *mut skcipher_request) -> c_int {
    static int sahara_aes_ecb_encrypt(struct skcipher_request *req)
    {
    return sahara_aes_crypt(req, FLAGS_ENCRYPT);
    }
#[no_mangle]
unsafe extern "C" fn sahara_aes_ecb_decrypt(req: *mut skcipher_request) -> c_int {
    static int sahara_aes_ecb_decrypt(struct skcipher_request *req)
    {
    return sahara_aes_crypt(req, 0);
    }
#[no_mangle]
unsafe extern "C" fn sahara_aes_cbc_encrypt(req: *mut skcipher_request) -> c_int {
    static int sahara_aes_cbc_encrypt(struct skcipher_request *req)
    {
    return sahara_aes_crypt(req, FLAGS_ENCRYPT | FLAGS_CBC);
    }
#[no_mangle]
unsafe extern "C" fn sahara_aes_cbc_decrypt(req: *mut skcipher_request) -> c_int {
    static int sahara_aes_cbc_decrypt(struct skcipher_request *req)
    {
    return sahara_aes_crypt(req, FLAGS_CBC);
    }
#[no_mangle]
unsafe extern "C" fn sahara_aes_init_tfm(tfm: *mut crypto_skcipher) -> c_int {
    static int sahara_aes_init_tfm(struct crypto_skcipher *tfm)
    {
    const char *name = crypto_tfm_alg_name(&tfm.base);
    struct sahara_ctx *ctx = crypto_skcipher_ctx(tfm);
    ctx.fallback = crypto_alloc_skcipher(name, 0,
    CRYPTO_ALG_NEED_FALLBACK);
    if (IS_ERR(ctx.fallback)) {
    pr_err("Error allocating fallback algo %s\n", name);
    return PTR_ERR(ctx.fallback);
    }
    crypto_skcipher_set_reqsize(tfm, sizeof(struct sahara_aes_reqctx) +
    crypto_skcipher_reqsize(ctx.fallback));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sahara_aes_exit_tfm(tfm: *mut crypto_skcipher) {
    static void sahara_aes_exit_tfm(struct crypto_skcipher *tfm)
    {
    struct sahara_ctx *ctx = crypto_skcipher_ctx(tfm);
    crypto_free_skcipher(ctx.fallback);
    }
    static u32 sahara_sha_init_hdr(struct sahara_dev *dev,
    struct sahara_sha_reqctx *rctx)
    {
    let mut hdr: u32 = 0;
    hdr = rctx.mode;
    if (rctx.first) {
    hdr |= SAHARA_HDR_MDHA_SET_MODE_HASH;
    hdr |= SAHARA_HDR_MDHA_INIT;
    } else {
    hdr |= SAHARA_HDR_MDHA_SET_MODE_MD_KEY;
    }
    if (rctx.last)
    hdr |= SAHARA_HDR_MDHA_PDATA;
    if (hweight_long(hdr) % 2 == 0)
    hdr |= SAHARA_HDR_PARITY_BIT;
    return hdr;
    }
    static int sahara_sha_hw_links_create(struct sahara_dev *dev,
    struct sahara_sha_reqctx *rctx,
    int start)
    {
    struct scatterlist *sg;
    unsigned int len;
    unsigned int i;
    int ret;
    dev.in_sg = rctx.in_sg;
    dev.nb_in_sg = sg_nents_for_len(dev.in_sg, rctx.total);
    if (dev.nb_in_sg < 0) {
    dev_err(dev.device, "Invalid numbers of src SG.\n");
    return dev.nb_in_sg;
    }
    if ((dev.nb_in_sg) > SAHARA_MAX_HW_LINK) {
    dev_err(dev.device, "not enough hw links (%d)\n",
    dev.nb_in_sg + dev.nb_out_sg);
    return -EINVAL;
    }
    sg = dev.in_sg;
    ret = dma_map_sg(dev.device, dev.in_sg, dev.nb_in_sg, DMA_TO_DEVICE);
    if (!ret)
    return -EFAULT;
    len = rctx.total;
    for (i = start; i < dev.nb_in_sg + start; i++) {
    dev.hw_link[i].len = min(len, sg.length);
    dev.hw_link[i].p = sg.dma_address;
    if (i == (dev.nb_in_sg + start - 1)) {
    dev.hw_link[i].next = 0;
    } else {
    len -= min(len, sg.length);
    dev.hw_link[i].next = dev.hw_phys_link[i + 1];
    sg = sg_next(sg);
    }
    }
    return i;
    }
    static int sahara_sha_hw_data_descriptor_create(struct sahara_dev *dev,
    struct sahara_sha_reqctx *rctx,
    struct ahash_request *req,
    int index)
    {
    unsigned result_len;
    let mut i: c_int = index;
    if (rctx.first)
// Create initial descriptor: #8
    dev.hw_desc[index].hdr = sahara_sha_init_hdr(dev, rctx);
    else
// Create hash descriptor: #10. Must follow #6.
    dev.hw_desc[index].hdr = SAHARA_HDR_MDHA_HASH;
    dev.hw_desc[index].len1 = rctx.total;
    if (dev.hw_desc[index].len1 == 0) {
// if len1 is 0, p1 must be 0, too
    dev.hw_desc[index].p1 = 0;
    rctx.sg_in_idx = 0;
    } else {
// Create input links
    dev.hw_desc[index].p1 = dev.hw_phys_link[index];
    i = sahara_sha_hw_links_create(dev, rctx, index);
    rctx.sg_in_idx = index;
    if (i < 0)
    return i;
    }
    dev.hw_desc[index].p2 = dev.hw_phys_link[i];
// Save the context for the next operation
    result_len = rctx.context_size;
    dev.hw_link[i].p = dev.context_phys_base;
    dev.hw_link[i].len = result_len;
    dev.hw_desc[index].len2 = result_len;
    dev.hw_link[i].next = 0;
    return 0;
    }
//
// Load descriptor aka #6
//
// To load a previously saved context back to the MDHA unit
//
// p1: Saved Context
// p2: NULL
//
    static int sahara_sha_hw_context_descriptor_create(struct sahara_dev *dev,
    struct sahara_sha_reqctx *rctx,
    struct ahash_request *req,
    int index)
    {
    dev.hw_desc[index].hdr = sahara_sha_init_hdr(dev, rctx);
    dev.hw_desc[index].len1 = rctx.context_size;
    dev.hw_desc[index].p1 = dev.hw_phys_link[index];
    dev.hw_desc[index].len2 = 0;
    dev.hw_desc[index].p2 = 0;
    dev.hw_link[index].len = rctx.context_size;
    dev.hw_link[index].p = dev.context_phys_base;
    dev.hw_link[index].next = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sahara_sha_prepare_request(req: *mut ahash_request) -> c_int {
    static int sahara_sha_prepare_request(struct ahash_request *req)
    {
    struct crypto_ahash *tfm = crypto_ahash_reqtfm(req);
    struct sahara_sha_reqctx *rctx = ahash_request_ctx(req);
    unsigned int hash_later;
    unsigned int block_size;
    unsigned int len;
    block_size = crypto_tfm_alg_blocksize(crypto_ahash_tfm(tfm));
// append bytes from previous operation
    len = rctx.buf_cnt + req.nbytes;
// only the last transfer can be padded in hardware
    if (!rctx.last && (len < block_size)) {
// to few data, save for next operation
    scatterwalk_map_and_copy(rctx.buf + rctx.buf_cnt, req.src,
    0, req.nbytes, 0);
    rctx.buf_cnt += req.nbytes;
    return 0;
    }
// add data from previous operation first
    if (rctx.buf_cnt)
    memcpy(rctx.rembuf, rctx.buf, rctx.buf_cnt);
// data must always be a multiple of block_size
    hash_later = rctx.last ? 0 : len & (block_size - 1);
    if (hash_later) {
    let mut offset: c_uint = req.nbytes - hash_later;
// Save remaining bytes for later use
    scatterwalk_map_and_copy(rctx.buf, req.src, offset,
    hash_later, 0);
    }
    rctx.total = len - hash_later;
// have data from previous operation and current
    if (rctx.buf_cnt && req.nbytes) {
    sg_init_table(rctx.in_sg_chain, 2);
    sg_set_buf(rctx.in_sg_chain, rctx.rembuf, rctx.buf_cnt);
    sg_chain(rctx.in_sg_chain, 2, req.src);
    rctx.in_sg = rctx.in_sg_chain;
// only data from previous operation
    } else if (rctx.buf_cnt) {
    rctx.in_sg = rctx.in_sg_chain;
    sg_init_one(rctx.in_sg, rctx.rembuf, rctx.buf_cnt);
// no data from previous operation
    } else {
    rctx.in_sg = req.src;
    }
// on next call, we only have the remaining data in the buffer
    rctx.buf_cnt = hash_later;
    return -EINPROGRESS;
    }
#[no_mangle]
unsafe extern "C" fn sahara_sha_process(req: *mut ahash_request) -> c_int {
    static int sahara_sha_process(struct ahash_request *req)
    {
    struct sahara_dev *dev = dev_ptr;
    struct sahara_sha_reqctx *rctx = ahash_request_ctx(req);
    int ret;
    unsigned long time_left;
    ret = sahara_sha_prepare_request(req);
    if (!ret)
    return ret;
    if (rctx.first) {
    ret = sahara_sha_hw_data_descriptor_create(dev, rctx, req, 0);
    if (ret)
    return ret;
    dev.hw_desc[0].next = 0;
    rctx.first = 0;
    } else {
    memcpy(dev.context_base, rctx.context, rctx.context_size);
    sahara_sha_hw_context_descriptor_create(dev, rctx, req, 0);
    dev.hw_desc[0].next = dev.hw_phys_desc[1];
    ret = sahara_sha_hw_data_descriptor_create(dev, rctx, req, 1);
    if (ret)
    return ret;
    dev.hw_desc[1].next = 0;
    }
    sahara_dump_descriptors(dev);
    sahara_dump_links(dev);
    reinit_completion(&dev.dma_completion);
    sahara_write(dev, dev.hw_phys_desc[0], SAHARA_REG_DAR);
    time_left = wait_for_completion_timeout(&dev.dma_completion,
    msecs_to_jiffies(SAHARA_TIMEOUT_MS));
    if (rctx.sg_in_idx)
    dma_unmap_sg(dev.device, dev.in_sg, dev.nb_in_sg,
    DMA_TO_DEVICE);
    if (!time_left) {
    dev_err(dev.device, "SHA timeout\n");
    return -ETIMEDOUT;
    }
    memcpy(rctx.context, dev.context_base, rctx.context_size);
    if (req.result && rctx.last)
    memcpy(req.result, rctx.context, rctx.digest_size);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sahara_do_one_request(engine: *mut crypto_engine, areq: *mut c_void) -> c_int {
    static int sahara_do_one_request(struct crypto_engine *engine, void *areq)
    {
    struct crypto_async_request *async_req = areq;
    int err;
    if (crypto_tfm_alg_type(async_req.tfm) == CRYPTO_ALG_TYPE_AHASH) {
    struct ahash_request *req = ahash_request_cast(async_req);
    err = sahara_sha_process(req);
    local_bh_disable();
    crypto_finalize_hash_request(engine, req, err);
    local_bh_enable();
    } else {
    struct skcipher_request *req = skcipher_request_cast(async_req);
    err = sahara_aes_process(skcipher_request_cast(async_req));
    local_bh_disable();
    crypto_finalize_skcipher_request(engine, req, err);
    local_bh_enable();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sahara_sha_enqueue(req: *mut ahash_request, last: c_int) -> c_int {
    static int sahara_sha_enqueue(struct ahash_request *req, int last)
    {
    struct sahara_sha_reqctx *rctx = ahash_request_ctx(req);
    struct sahara_dev *dev = dev_ptr;
    if (!req.nbytes && !last)
    return 0;
    rctx.last = last;
    return crypto_transfer_hash_request_to_engine(dev.engine, req);
    }
#[no_mangle]
unsafe extern "C" fn sahara_sha_init(req: *mut ahash_request) -> c_int {
    static int sahara_sha_init(struct ahash_request *req)
    {
    struct crypto_ahash *tfm = crypto_ahash_reqtfm(req);
    struct sahara_sha_reqctx *rctx = ahash_request_ctx(req);
    memset(rctx, 0, sizeof(*rctx));
    switch (crypto_ahash_digestsize(tfm)) {
    case SHA1_DIGEST_SIZE:
    rctx.mode |= SAHARA_HDR_MDHA_ALG_SHA1;
    rctx.digest_size = SHA1_DIGEST_SIZE;
    break;
    case SHA256_DIGEST_SIZE:
    rctx.mode |= SAHARA_HDR_MDHA_ALG_SHA256;
    rctx.digest_size = SHA256_DIGEST_SIZE;
    break;
    default:
    return -EINVAL;
    }
    rctx.context_size = rctx.digest_size + 4;
    rctx.first = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sahara_sha_update(req: *mut ahash_request) -> c_int {
    static int sahara_sha_update(struct ahash_request *req)
    {
    return sahara_sha_enqueue(req, 0);
    }
#[no_mangle]
unsafe extern "C" fn sahara_sha_final(req: *mut ahash_request) -> c_int {
    static int sahara_sha_final(struct ahash_request *req)
    {
    req.nbytes = 0;
    return sahara_sha_enqueue(req, 1);
    }
#[no_mangle]
unsafe extern "C" fn sahara_sha_finup(req: *mut ahash_request) -> c_int {
    static int sahara_sha_finup(struct ahash_request *req)
    {
    return sahara_sha_enqueue(req, 1);
    }
#[no_mangle]
unsafe extern "C" fn sahara_sha_digest(req: *mut ahash_request) -> c_int {
    static int sahara_sha_digest(struct ahash_request *req)
    {
    sahara_sha_init(req);
    return sahara_sha_finup(req);
    }
#[no_mangle]
unsafe extern "C" fn sahara_sha_export(req: *mut ahash_request, out: *mut c_void) -> c_int {
    static int sahara_sha_export(struct ahash_request *req, void *out)
    {
    struct sahara_sha_reqctx *rctx = ahash_request_ctx(req);
    memcpy(out, rctx, sizeof(struct sahara_sha_reqctx));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sahara_sha_import(req: *mut ahash_request, in: *const c_void) -> c_int {
    static int sahara_sha_import(struct ahash_request *req, const void *in)
    {
    struct sahara_sha_reqctx *rctx = ahash_request_ctx(req);
    memcpy(rctx, in, sizeof(struct sahara_sha_reqctx));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sahara_sha_cra_init(tfm: *mut crypto_tfm) -> c_int {
    static int sahara_sha_cra_init(struct crypto_tfm *tfm)
    {
    crypto_ahash_set_reqsize(__crypto_ahash_cast(tfm),
    sizeof(struct sahara_sha_reqctx));
    return 0;
    }
    static struct skcipher_engine_alg aes_algs[] = {
    {
    .base = {
    .base.cra_name		= "ecb(aes)",
    .base.cra_driver_name	= "sahara-ecb-aes",
    .base.cra_priority	= 300,
    .base.cra_flags		= CRYPTO_ALG_ASYNC | CRYPTO_ALG_NEED_FALLBACK,
    .base.cra_blocksize	= AES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct sahara_ctx),
    .base.cra_alignmask	= 0x0,
    .base.cra_module	= THIS_MODULE,
    .init			= sahara_aes_init_tfm,
    .exit			= sahara_aes_exit_tfm,
    .min_keysize		= AES_MIN_KEY_SIZE,
    .max_keysize		= AES_MAX_KEY_SIZE,
    .setkey			= sahara_aes_setkey,
    .encrypt		= sahara_aes_ecb_encrypt,
    .decrypt		= sahara_aes_ecb_decrypt,
    },
    .op = {
    .do_one_request = sahara_do_one_request,
    },
    }, {
    .base = {
    .base.cra_name		= "cbc(aes)",
    .base.cra_driver_name	= "sahara-cbc-aes",
    .base.cra_priority	= 300,
    .base.cra_flags		= CRYPTO_ALG_ASYNC | CRYPTO_ALG_NEED_FALLBACK,
    .base.cra_blocksize	= AES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct sahara_ctx),
    .base.cra_alignmask	= 0x0,
    .base.cra_module	= THIS_MODULE,
    .init			= sahara_aes_init_tfm,
    .exit			= sahara_aes_exit_tfm,
    .min_keysize		= AES_MIN_KEY_SIZE,
    .max_keysize		= AES_MAX_KEY_SIZE,
    .ivsize			= AES_BLOCK_SIZE,
    .setkey			= sahara_aes_setkey,
    .encrypt		= sahara_aes_cbc_encrypt,
    .decrypt		= sahara_aes_cbc_decrypt,
    },
    .op = {
    .do_one_request = sahara_do_one_request,
    },
    }
    };
    static struct ahash_engine_alg sha_v3_algs[] = {
    {
    .base = {
    .init		= sahara_sha_init,
    .update		= sahara_sha_update,
    .final		= sahara_sha_final,
    .finup		= sahara_sha_finup,
    .digest		= sahara_sha_digest,
    .export		= sahara_sha_export,
    .import		= sahara_sha_import,
    .halg.digestsize	= SHA1_DIGEST_SIZE,
    .halg.statesize         = sizeof(struct sahara_sha_reqctx),
    .halg.base	= {
    .cra_name		= "sha1",
    .cra_driver_name	= "sahara-sha1",
    .cra_priority		= 300,
    .cra_flags		= CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_blocksize		= SHA1_BLOCK_SIZE,
    .cra_ctxsize		= sizeof(struct sahara_ctx),
    .cra_alignmask		= 0,
    .cra_module		= THIS_MODULE,
    .cra_init		= sahara_sha_cra_init,
    }
    },
    .op = {
    .do_one_request = sahara_do_one_request,
    },
    },
    };
    static struct ahash_engine_alg sha_v4_algs[] = {
    {
    .base = {
    .init		= sahara_sha_init,
    .update		= sahara_sha_update,
    .final		= sahara_sha_final,
    .finup		= sahara_sha_finup,
    .digest		= sahara_sha_digest,
    .export		= sahara_sha_export,
    .import		= sahara_sha_import,
    .halg.digestsize	= SHA256_DIGEST_SIZE,
    .halg.statesize         = sizeof(struct sahara_sha_reqctx),
    .halg.base	= {
    .cra_name		= "sha256",
    .cra_driver_name	= "sahara-sha256",
    .cra_priority		= 300,
    .cra_flags		= CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_blocksize		= SHA256_BLOCK_SIZE,
    .cra_ctxsize		= sizeof(struct sahara_ctx),
    .cra_alignmask		= 0,
    .cra_module		= THIS_MODULE,
    .cra_init		= sahara_sha_cra_init,
    }
    },
    .op = {
    .do_one_request = sahara_do_one_request,
    },
    },
    };
#[no_mangle]
unsafe extern "C" fn sahara_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t sahara_irq_handler(int irq, void *data)
    {
    struct sahara_dev *dev = data;
    let mut stat: c_uint = sahara_read(dev, SAHARA_REG_STATUS);
    let mut err: c_uint = sahara_read(dev, SAHARA_REG_ERRSTATUS);
    sahara_write(dev, SAHARA_CMD_CLEAR_INT | SAHARA_CMD_CLEAR_ERR,
    SAHARA_REG_CMD);
    sahara_decode_status(dev, stat);
    if (SAHARA_STATUS_GET_STATE(stat) == SAHARA_STATE_BUSY)
    return IRQ_NONE;
    if (SAHARA_STATUS_GET_STATE(stat) != SAHARA_STATE_COMPLETE)
    sahara_decode_error(dev, err);
    complete(&dev.dma_completion);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sahara_register_algs(dev: *mut sahara_dev) -> c_int {
    static int sahara_register_algs(struct sahara_dev *dev)
    {
    int err;
    err = crypto_engine_register_skciphers(aes_algs, ARRAY_SIZE(aes_algs));
    if (err)
    return err;
    err = crypto_engine_register_ahashes(sha_v3_algs,
    ARRAY_SIZE(sha_v3_algs));
    if (err)
    goto err_aes_algs;
    if (dev.version > SAHARA_VERSION_3) {
    err = crypto_engine_register_ahashes(sha_v4_algs,
    ARRAY_SIZE(sha_v4_algs));
    if (err)
    goto err_sha_v3_algs;
    }
    return 0;
    err_sha_v3_algs:
    crypto_engine_unregister_ahashes(sha_v3_algs, ARRAY_SIZE(sha_v3_algs));
    err_aes_algs:
    crypto_engine_unregister_skciphers(aes_algs, ARRAY_SIZE(aes_algs));
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sahara_unregister_algs(dev: *mut sahara_dev) {
    static void sahara_unregister_algs(struct sahara_dev *dev)
    {
    crypto_engine_unregister_skciphers(aes_algs, ARRAY_SIZE(aes_algs));
    crypto_engine_unregister_ahashes(sha_v3_algs, ARRAY_SIZE(sha_v3_algs));
    if (dev.version > SAHARA_VERSION_3)
    crypto_engine_unregister_ahashes(sha_v4_algs,
    ARRAY_SIZE(sha_v4_algs));
    }
    static const struct of_device_id sahara_dt_ids[] = {
    { .compatible = "fsl,imx53-sahara" },
    { .compatible = "fsl,imx27-sahara" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, sahara_dt_ids);
#[no_mangle]
unsafe extern "C" fn sahara_probe(pdev: *mut platform_device) -> c_int {
    static int sahara_probe(struct platform_device *pdev)
    {
    struct sahara_dev *dev;
    u32 version;
    int irq;
    int err;
    int i;
    dev = devm_kzalloc(&pdev.dev, sizeof(*dev), GFP_KERNEL);
    if (!dev)
    return -ENOMEM;
    dev.device = &pdev.dev;
    platform_set_drvdata(pdev, dev);
// Get the base address
    dev.regs_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(dev.regs_base))
    return PTR_ERR(dev.regs_base);
// Get the IRQ
    irq = platform_get_irq(pdev,  0);
    if (irq < 0)
    return irq;
    err = devm_request_irq(&pdev.dev, irq, sahara_irq_handler,
    0, dev_name(&pdev.dev), dev);
    if (err)
    return err;
// clocks
    dev.clk_ipg = devm_clk_get_enabled(&pdev.dev, "ipg");
    if (IS_ERR(dev.clk_ipg))
    return dev_err_probe(&pdev.dev, PTR_ERR(dev.clk_ipg),
    "Could not get ipg clock\n");
    dev.clk_ahb = devm_clk_get_enabled(&pdev.dev, "ahb");
    if (IS_ERR(dev.clk_ahb))
    return dev_err_probe(&pdev.dev, PTR_ERR(dev.clk_ahb),
    "Could not get ahb clock\n");
// Allocate HW descriptors
    dev.hw_desc[0] = dmam_alloc_coherent(&pdev.dev,
    SAHARA_MAX_HW_DESC * sizeof(struct sahara_hw_desc),
    &dev.hw_phys_desc[0], GFP_KERNEL);
    if (!dev.hw_desc[0])
    return -ENOMEM;
    dev.hw_desc[1] = dev.hw_desc[0] + 1;
    dev.hw_phys_desc[1] = dev.hw_phys_desc[0] +
    sizeof(struct sahara_hw_desc);
// Allocate space for iv and key
    dev.key_base = dmam_alloc_coherent(&pdev.dev, 2 * AES_KEYSIZE_128,
    &dev.key_phys_base, GFP_KERNEL);
    if (!dev.key_base)
    return -ENOMEM;
    dev.iv_base = dev.key_base + AES_KEYSIZE_128;
    dev.iv_phys_base = dev.key_phys_base + AES_KEYSIZE_128;
// Allocate space for context: largest digest + message length field
    dev.context_base = dmam_alloc_coherent(&pdev.dev,
    SHA256_DIGEST_SIZE + 4,
    &dev.context_phys_base, GFP_KERNEL);
    if (!dev.context_base)
    return -ENOMEM;
// Allocate space for HW links
    dev.hw_link[0] = dmam_alloc_coherent(&pdev.dev,
    SAHARA_MAX_HW_LINK * sizeof(struct sahara_hw_link),
    &dev.hw_phys_link[0], GFP_KERNEL);
    if (!dev.hw_link[0])
    return -ENOMEM;
    for (i = 1; i < SAHARA_MAX_HW_LINK; i++) {
    dev.hw_phys_link[i] = dev.hw_phys_link[i - 1] +
    sizeof(struct sahara_hw_link);
    dev.hw_link[i] = dev.hw_link[i - 1] + 1;
    }
    dev_ptr = dev;
    dev.engine = crypto_engine_alloc_init(&pdev.dev, true);
    if (!dev.engine)
    return -ENOMEM;
    err = crypto_engine_start(dev.engine);
    if (err) {
    crypto_engine_exit(dev.engine);
    return dev_err_probe(&pdev.dev, err,
    "Could not start crypto engine\n");
    }
    init_completion(&dev.dma_completion);
    version = sahara_read(dev, SAHARA_REG_VERSION);
    if (of_device_is_compatible(pdev.dev.of_node, "fsl,imx27-sahara")) {
    if (version != SAHARA_VERSION_3)
    err = -ENODEV;
    } else if (of_device_is_compatible(pdev.dev.of_node,
    "fsl,imx53-sahara")) {
    if (((version >> 8) & 0xff) != SAHARA_VERSION_4)
    err = -ENODEV;
    version = (version >> 8) & 0xff;
    }
    if (err == -ENODEV) {
    dev_err_probe(&pdev.dev, err,
    "SAHARA version %d not supported\n", version);
    goto err_algs;
    }
    dev.version = version;
    sahara_write(dev, SAHARA_CMD_RESET | SAHARA_CMD_MODE_BATCH,
    SAHARA_REG_CMD);
    sahara_write(dev, SAHARA_CONTROL_SET_THROTTLE(0) |
    SAHARA_CONTROL_SET_MAXBURST(8) |
    SAHARA_CONTROL_RNG_AUTORSD |
    SAHARA_CONTROL_ENABLE_INT,
    SAHARA_REG_CONTROL);
    err = sahara_register_algs(dev);
    if (err)
    goto err_algs;
    dev_info(&pdev.dev, "SAHARA version %d initialized\n", version);
    return 0;
    err_algs:
    crypto_engine_exit(dev.engine);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sahara_remove(pdev: *mut platform_device) {
    static void sahara_remove(struct platform_device *pdev)
    {
    struct sahara_dev *dev = platform_get_drvdata(pdev);
    crypto_engine_exit(dev.engine);
    sahara_unregister_algs(dev);
    }
    static struct platform_driver sahara_driver = {
    .probe		= sahara_probe,
    .remove		= sahara_remove,
    .driver		= {
    .name	= SAHARA_NAME,
    .of_match_table = sahara_dt_ids,
    },
    };
    module_platform_driver(sahara_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Javier Martin <javier.martin@vista-silicon.com>");
    MODULE_AUTHOR("Steffen Trumtrar <s.trumtrar@pengutronix.de>");
    MODULE_DESCRIPTION("SAHARA2 HW crypto accelerator");
