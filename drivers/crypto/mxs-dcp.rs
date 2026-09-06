//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/mxs-dcp.c
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
// Freescale i.MX23/i.MX28 Data Co-Processor driver
//
// Copyright (C) 2013 Marek Vasut <marex@denx.de>
//

pub const DCP_MAX_CHANS: c_int = 4;

pub const DCP_SHA_PAY_SZ: c_int = 64;
pub const DCP_ALIGNMENT: c_int = 64;
//
// Null hashes to align with hw behavior on imx6sl and ull
// these are flipped for consistency with hw output
//
    static const uint8_t sha1_null_hash[] =
    "\x09\x07\xd8\xaf\x90\x18\x60\x95\xef\xbf"
    "\x55\x32\x0d\x4b\x6b\x5e\xee\xa3\x39\xda";
    static const uint8_t sha256_null_hash[] =
    "\x55\xb8\x52\x78\x1b\x99\x95\xa4"
    "\x4c\x93\x9b\x64\xe4\x41\xae\x27"
    "\x24\xb9\x6f\x99\xc8\xf4\xfb\x9a"
    "\x14\x1c\xfc\x98\x42\xc4\xb0\xe3";
// DCP DMA descriptor.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcp_dma_desc {
    pub next_cmd_addr: u32,
    pub control0: u32,
    pub control1: u32,
    pub source: u32,
    pub destination: u32,
    pub size: u32,
    pub payload: u32,
    pub status: u32,
}

// Coherent aligned block for bounce buffering.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcp_coherent_block {
    pub aes_in_buf: [u8; DCP_BUF_SZ],
    pub aes_out_buf: [u8; DCP_BUF_SZ],
    pub sha_in_buf: [u8; DCP_BUF_SZ],
    pub sha_out_buf: [u8; DCP_SHA_PAY_SZ],
    pub AES_KEYSIZE_128]: *mut *mut uint8_t aes_key[2,
    pub desc: [dcp_dma_desc; DCP_MAX_CHANS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcp {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub caps: u32,
    pub coh: *mut dcp_coherent_block,
    pub completion: [completion; DCP_MAX_CHANS],
    pub lock: [spinlock_t; DCP_MAX_CHANS],
    pub thread: [*mut task_struct; DCP_MAX_CHANS],
    pub queue: [crypto_queue; DCP_MAX_CHANS],
    pub dcp_clk: *mut clk,
}

    enum dcp_chan {
    DCP_CHAN_HASH_SHA	= 0,
    DCP_CHAN_CRYPTO		= 2,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcp_async_ctx {
// Common context
    pub chan: enum dcp_chan,
    pub fill: u32,
// SHA Hash-specific context
    pub mutex: mutex,
    pub alg: u32,
    pub hot:1: c_uint,
// Crypto-specific context
    pub fallback: *mut crypto_skcipher,
    pub key_len: c_uint,
    pub key: [u8; AES_KEYSIZE_128],
    pub key_referenced: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcp_aes_req_ctx {
    pub enc:1: c_uint,
    pub ecb:1: c_uint,
    pub end: skcipher_request fallback_req; // keep at the,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcp_sha_req_ctx {
    pub init:1: c_uint,
    pub fini:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcp_export_state {
    pub req_ctx: dcp_sha_req_ctx,
    pub async_ctx: dcp_async_ctx,
}

//
// There can even be only one instance of the MXS DCP due to the
// design of Linux Crypto API.
//
    static struct dcp *global_sdcp;
// DCP register layout.
pub const MXS_DCP_CTRL: c_uint = 0x00;

pub const MXS_DCP_STAT: c_uint = 0x10;
pub const MXS_DCP_STAT_CLR: c_uint = 0x18;
pub const MXS_DCP_STAT_IRQ_MASK: c_uint = 0xf;
pub const MXS_DCP_CHANNELCTRL: c_uint = 0x20;
pub const MXS_DCP_CHANNELCTRL_ENABLE_CHANNEL_MASK: c_uint = 0xff;
pub const MXS_DCP_CAPABILITY1: c_uint = 0x40;

pub const MXS_DCP_CONTEXT: c_uint = 0x50;

// DMA descriptor bits.

pub const MXS_DCP_CONTROL1_KEY_SELECT_SHIFT: c_int = 8;
#[no_mangle]
unsafe extern "C" fn mxs_dcp_start_dma(actx: *mut dcp_async_ctx) -> c_int {
    static int mxs_dcp_start_dma(struct dcp_async_ctx *actx)
    {
    int dma_err;
    struct dcp *sdcp = global_sdcp;
    let mut chan: c_int = actx.chan;
    uint32_t stat;
    unsigned long ret;
    struct dcp_dma_desc *desc = &sdcp.coh.desc[actx.chan];
    dma_addr_t desc_phys = dma_map_single(sdcp.dev, desc, sizeof(*desc),
    DMA_TO_DEVICE);
    dma_err = dma_mapping_error(sdcp.dev, desc_phys);
    if (dma_err)
    return dma_err;
    reinit_completion(&sdcp.completion[chan]);
// Clear status register.
    writel(0xffffffff, sdcp.base + MXS_DCP_CH_N_STAT_CLR(chan));
// Load the DMA descriptor.
    writel(desc_phys, sdcp.base + MXS_DCP_CH_N_CMDPTR(chan));
// Increment the semaphore to start the DMA transfer.
    writel(1, sdcp.base + MXS_DCP_CH_N_SEMA(chan));
    ret = wait_for_completion_timeout(&sdcp.completion[chan],
    msecs_to_jiffies(1000));
    if (!ret) {
    dev_err(sdcp.dev, "Channel %i timeout (DCP_STAT=0x%08x)\n",
    chan, readl(sdcp.base + MXS_DCP_STAT));
    return -ETIMEDOUT;
    }
    stat = readl(sdcp.base + MXS_DCP_CH_N_STAT(chan));
    if (stat & 0xff) {
    dev_err(sdcp.dev, "Channel %i error (CH_STAT=0x%08x)\n",
    chan, stat);
    return -EINVAL;
    }
    dma_unmap_single(sdcp.dev, desc_phys, sizeof(*desc), DMA_TO_DEVICE);
    return 0;
    }
//
// Encryption (AES128)
//
    static int mxs_dcp_run_aes(struct dcp_async_ctx *actx,
    struct skcipher_request *req, int init)
    {
    dma_addr_t key_phys, src_phys, dst_phys;
    struct dcp *sdcp = global_sdcp;
    struct dcp_dma_desc *desc = &sdcp.coh.desc[actx.chan];
    struct dcp_aes_req_ctx *rctx = skcipher_request_ctx(req);
    let mut key_referenced: bool = actx.key_referenced;
    int ret;
    if (key_referenced)
    key_phys = dma_map_single(sdcp.dev, sdcp.coh.aes_key + AES_KEYSIZE_128,
    AES_KEYSIZE_128, DMA_TO_DEVICE);
    else
    key_phys = dma_map_single(sdcp.dev, sdcp.coh.aes_key,
    2 * AES_KEYSIZE_128, DMA_TO_DEVICE);
    ret = dma_mapping_error(sdcp.dev, key_phys);
    if (ret)
    return ret;
    src_phys = dma_map_single(sdcp.dev, sdcp.coh.aes_in_buf,
    DCP_BUF_SZ, DMA_TO_DEVICE);
    ret = dma_mapping_error(sdcp.dev, src_phys);
    if (ret)
    goto err_src;
    dst_phys = dma_map_single(sdcp.dev, sdcp.coh.aes_out_buf,
    DCP_BUF_SZ, DMA_FROM_DEVICE);
    ret = dma_mapping_error(sdcp.dev, dst_phys);
    if (ret)
    goto err_dst;
    if (actx.fill % AES_BLOCK_SIZE) {
    dev_err(sdcp.dev, "Invalid block size!\n");
    ret = -EINVAL;
    goto aes_done_run;
    }
// Fill in the DMA descriptor.
    desc.control0 = MXS_DCP_CONTROL0_DECR_SEMAPHORE |
    MXS_DCP_CONTROL0_INTERRUPT |
    MXS_DCP_CONTROL0_ENABLE_CIPHER;
    if (!key_referenced)
// Payload contains the key.
    desc.control0 |= MXS_DCP_CONTROL0_PAYLOAD_KEY;
#[no_mangle]
pub unsafe extern "C" fn if(DCP_PAES_KEY_OTP: actx->key[0] ==) -> else {
    else if (actx.key[0] == DCP_PAES_KEY_OTP)
// Set OTP key bit to select the key via KEY_SELECT.
    desc.control0 |= MXS_DCP_CONTROL0_OTP_KEY;
    if (rctx.enc)
    desc.control0 |= MXS_DCP_CONTROL0_CIPHER_ENCRYPT;
    if (init)
    desc.control0 |= MXS_DCP_CONTROL0_CIPHER_INIT;
    desc.control1 = MXS_DCP_CONTROL1_CIPHER_SELECT_AES128;
    if (rctx.ecb)
    desc.control1 |= MXS_DCP_CONTROL1_CIPHER_MODE_ECB;
    else
    desc.control1 |= MXS_DCP_CONTROL1_CIPHER_MODE_CBC;
    if (key_referenced)
    desc.control1 |= sdcp.coh.aes_key[0] << MXS_DCP_CONTROL1_KEY_SELECT_SHIFT;
    desc.next_cmd_addr = 0;
    desc.source = src_phys;
    desc.destination = dst_phys;
    desc.size = actx.fill;
    desc.payload = key_phys;
    desc.status = 0;
    ret = mxs_dcp_start_dma(actx);
    aes_done_run:
    dma_unmap_single(sdcp.dev, dst_phys, DCP_BUF_SZ, DMA_FROM_DEVICE);
    err_dst:
    dma_unmap_single(sdcp.dev, src_phys, DCP_BUF_SZ, DMA_TO_DEVICE);
    err_src:
    if (key_referenced)
    dma_unmap_single(sdcp.dev, key_phys, AES_KEYSIZE_128,
    DMA_TO_DEVICE);
    else
    dma_unmap_single(sdcp.dev, key_phys, 2 * AES_KEYSIZE_128,
    DMA_TO_DEVICE);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mxs_dcp_aes_block_crypt(arq: *mut crypto_async_request) -> c_int {
    static int mxs_dcp_aes_block_crypt(struct crypto_async_request *arq)
    {
    struct dcp *sdcp = global_sdcp;
    struct skcipher_request *req = skcipher_request_cast(arq);
    struct dcp_async_ctx *actx = crypto_tfm_ctx(arq.tfm);
    struct dcp_aes_req_ctx *rctx = skcipher_request_ctx(req);
    struct scatterlist *dst = req.dst;
    struct scatterlist *src = req.src;
    let mut dst_nents: c_int = sg_nents(dst);
    let mut out_off: c_int = DCP_BUF_SZ;
    uint8_t *in_buf = sdcp.coh.aes_in_buf;
    uint8_t *out_buf = sdcp.coh.aes_out_buf;
    let mut dst_off: u32 = 0;
    uint8_t *src_buf = core::ptr::null_mut();
    let mut last_out_len: u32 = 0;
    uint8_t *key = sdcp.coh.aes_key;
    let mut ret: c_int = 0;
    unsigned int i, len, clen, tlen = 0;
    let mut init: c_int = 0;
    let mut limit_hit: bool = false;
    actx.fill = 0;
// Copy the key from the temporary location.
    memcpy(key, actx.key, actx.key_len);
    if (!rctx.ecb) {
// Copy the CBC IV just past the key.
    memcpy(key + AES_KEYSIZE_128, req.iv, AES_KEYSIZE_128);
// CBC needs the INIT set.
    init = 1;
    } else {
    memset(key + AES_KEYSIZE_128, 0, AES_KEYSIZE_128);
    }
    for_each_sg(req.src, src, sg_nents(req.src), i) {
    src_buf = sg_virt(src);
    len = src.length;
    tlen += len;
    limit_hit = tlen > req.cryptlen;
    if (limit_hit)
    len = req.cryptlen - (tlen - len);
    do {
    if (actx.fill + len > out_off)
    clen = out_off - actx.fill;
    else
    clen = len;
    memcpy(in_buf + actx.fill, src_buf, clen);
    len -= clen;
    src_buf += clen;
    actx.fill += clen;
//
// If we filled the buffer or this is the last SG,
// submit the buffer.
//
    if (actx.fill == out_off || sg_is_last(src) ||
    limit_hit) {
    ret = mxs_dcp_run_aes(actx, req, init);
    if (ret)
    return ret;
    init = 0;
    sg_pcopy_from_buffer(dst, dst_nents, out_buf,
    actx.fill, dst_off);
    dst_off += actx.fill;
    last_out_len = actx.fill;
    actx.fill = 0;
    }
    } while (len);
    if (limit_hit)
    break;
    }
// Copy the IV for CBC for chaining
    if (!rctx.ecb) {
    if (rctx.enc)
    memcpy(req.iv, out_buf+(last_out_len-AES_BLOCK_SIZE),
    AES_BLOCK_SIZE);
    else
    memcpy(req.iv, in_buf+(last_out_len-AES_BLOCK_SIZE),
    AES_BLOCK_SIZE);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dcp_chan_thread_aes(data: *mut c_void) -> c_int {
    static int dcp_chan_thread_aes(void *data)
    {
    struct dcp *sdcp = global_sdcp;
    let mut chan: c_int = DCP_CHAN_CRYPTO;
    struct crypto_async_request *backlog;
    struct crypto_async_request *arq;
    int ret;
    while (!kthread_should_stop()) {
    set_current_state(TASK_INTERRUPTIBLE);
    spin_lock(&sdcp.lock[chan]);
    backlog = crypto_get_backlog(&sdcp.queue[chan]);
    arq = crypto_dequeue_request(&sdcp.queue[chan]);
    spin_unlock(&sdcp.lock[chan]);
    if (!backlog && !arq) {
    schedule();
    continue;
    }
    set_current_state(TASK_RUNNING);
    if (backlog)
    crypto_request_complete(backlog, -EINPROGRESS);
    if (arq) {
    ret = mxs_dcp_aes_block_crypt(arq);
    crypto_request_complete(arq, ret);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxs_dcp_block_fallback(req: *mut skcipher_request, enc: c_int) -> c_int {
    static int mxs_dcp_block_fallback(struct skcipher_request *req, int enc)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct dcp_aes_req_ctx *rctx = skcipher_request_ctx(req);
    struct dcp_async_ctx *ctx = crypto_skcipher_ctx(tfm);
    int ret;
    skcipher_request_set_tfm(&rctx.fallback_req, ctx.fallback);
    skcipher_request_set_callback(&rctx.fallback_req, req.base.flags,
    req.base.complete, req.base.data);
    skcipher_request_set_crypt(&rctx.fallback_req, req.src, req.dst,
    req.cryptlen, req.iv);
    if (enc)
    ret = crypto_skcipher_encrypt(&rctx.fallback_req);
    else
    ret = crypto_skcipher_decrypt(&rctx.fallback_req);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mxs_dcp_aes_enqueue(req: *mut skcipher_request, enc: c_int, ecb: c_int) -> c_int {
    static int mxs_dcp_aes_enqueue(struct skcipher_request *req, int enc, int ecb)
    {
    struct dcp *sdcp = global_sdcp;
    struct crypto_async_request *arq = &req.base;
    struct dcp_async_ctx *actx = crypto_tfm_ctx(arq.tfm);
    struct dcp_aes_req_ctx *rctx = skcipher_request_ctx(req);
    int ret;
    if (unlikely(actx.key_len != AES_KEYSIZE_128 && !actx.key_referenced))
    return mxs_dcp_block_fallback(req, enc);
    rctx.enc = enc;
    rctx.ecb = ecb;
    actx.chan = DCP_CHAN_CRYPTO;
    spin_lock(&sdcp.lock[actx.chan]);
    ret = crypto_enqueue_request(&sdcp.queue[actx.chan], &req.base);
    spin_unlock(&sdcp.lock[actx.chan]);
    wake_up_process(sdcp.thread[actx.chan]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mxs_dcp_aes_ecb_decrypt(req: *mut skcipher_request) -> c_int {
    static int mxs_dcp_aes_ecb_decrypt(struct skcipher_request *req)
    {
    return mxs_dcp_aes_enqueue(req, 0, 1);
    }
#[no_mangle]
unsafe extern "C" fn mxs_dcp_aes_ecb_encrypt(req: *mut skcipher_request) -> c_int {
    static int mxs_dcp_aes_ecb_encrypt(struct skcipher_request *req)
    {
    return mxs_dcp_aes_enqueue(req, 1, 1);
    }
#[no_mangle]
unsafe extern "C" fn mxs_dcp_aes_cbc_decrypt(req: *mut skcipher_request) -> c_int {
    static int mxs_dcp_aes_cbc_decrypt(struct skcipher_request *req)
    {
    return mxs_dcp_aes_enqueue(req, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn mxs_dcp_aes_cbc_encrypt(req: *mut skcipher_request) -> c_int {
    static int mxs_dcp_aes_cbc_encrypt(struct skcipher_request *req)
    {
    return mxs_dcp_aes_enqueue(req, 1, 0);
    }
    static int mxs_dcp_aes_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int len)
    {
    struct dcp_async_ctx *actx = crypto_skcipher_ctx(tfm);
//
// AES 128 is supposed by the hardware, store key into temporary
// buffer and exit. We must use the temporary buffer here, since
// there can still be an operation in progress.
//
    actx.key_len = len;
    actx.key_referenced = false;
    if (len == AES_KEYSIZE_128) {
    memcpy(actx.key, key, len);
    return 0;
    }
//
// If the requested AES key size is not supported by the hardware,
// but is supported by in-kernel software implementation, we use
// software fallback.
//
    crypto_skcipher_clear_flags(actx.fallback, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags(actx.fallback,
    tfm.base.crt_flags & CRYPTO_TFM_REQ_MASK);
    return crypto_skcipher_setkey(actx.fallback, key, len);
    }
    static int mxs_dcp_aes_setrefkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int len)
    {
    struct dcp_async_ctx *actx = crypto_skcipher_ctx(tfm);
    if (len != DCP_PAES_KEYSIZE)
    return -EINVAL;
    switch (key[0]) {
    case DCP_PAES_KEY_SLOT0:
    case DCP_PAES_KEY_SLOT1:
    case DCP_PAES_KEY_SLOT2:
    case DCP_PAES_KEY_SLOT3:
    case DCP_PAES_KEY_UNIQUE:
    case DCP_PAES_KEY_OTP:
    memcpy(actx.key, key, len);
    actx.key_len = len;
    actx.key_referenced = true;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxs_dcp_aes_fallback_init_tfm(tfm: *mut crypto_skcipher) -> c_int {
    static int mxs_dcp_aes_fallback_init_tfm(struct crypto_skcipher *tfm)
    {
    const char *name = crypto_tfm_alg_name(crypto_skcipher_tfm(tfm));
    struct dcp_async_ctx *actx = crypto_skcipher_ctx(tfm);
    struct crypto_skcipher *blk;
    blk = crypto_alloc_skcipher(name, 0, CRYPTO_ALG_NEED_FALLBACK);
    if (IS_ERR(blk))
    return PTR_ERR(blk);
    actx.fallback = blk;
    crypto_skcipher_set_reqsize(tfm, sizeof(struct dcp_aes_req_ctx) +
    crypto_skcipher_reqsize(blk));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxs_dcp_aes_fallback_exit_tfm(tfm: *mut crypto_skcipher) {
    static void mxs_dcp_aes_fallback_exit_tfm(struct crypto_skcipher *tfm)
    {
    struct dcp_async_ctx *actx = crypto_skcipher_ctx(tfm);
    crypto_free_skcipher(actx.fallback);
    }
#[no_mangle]
unsafe extern "C" fn mxs_dcp_paes_init_tfm(tfm: *mut crypto_skcipher) -> c_int {
    static int mxs_dcp_paes_init_tfm(struct crypto_skcipher *tfm)
    {
    crypto_skcipher_set_reqsize(tfm, sizeof(struct dcp_aes_req_ctx));
    return 0;
    }
//
// Hashing (SHA1/SHA256)
//
#[no_mangle]
unsafe extern "C" fn mxs_dcp_run_sha(req: *mut ahash_request) -> c_int {
    static int mxs_dcp_run_sha(struct ahash_request *req)
    {
    struct dcp *sdcp = global_sdcp;
    int ret;
    struct crypto_ahash *tfm = crypto_ahash_reqtfm(req);
    struct dcp_async_ctx *actx = crypto_ahash_ctx(tfm);
    struct dcp_sha_req_ctx *rctx = ahash_request_ctx(req);
    struct dcp_dma_desc *desc = &sdcp.coh.desc[actx.chan];
    let mut digest_phys: dma_addr_t = 0;
    dma_addr_t buf_phys = dma_map_single(sdcp.dev, sdcp.coh.sha_in_buf,
    DCP_BUF_SZ, DMA_TO_DEVICE);
    ret = dma_mapping_error(sdcp.dev, buf_phys);
    if (ret)
    return ret;
// Fill in the DMA descriptor.
    desc.control0 = MXS_DCP_CONTROL0_DECR_SEMAPHORE |
    MXS_DCP_CONTROL0_INTERRUPT |
    MXS_DCP_CONTROL0_ENABLE_HASH;
    if (rctx.init)
    desc.control0 |= MXS_DCP_CONTROL0_HASH_INIT;
    desc.control1 = actx.alg;
    desc.next_cmd_addr = 0;
    desc.source = buf_phys;
    desc.destination = 0;
    desc.size = actx.fill;
    desc.payload = 0;
    desc.status = 0;
//
// Align driver with hw behavior when generating null hashes
//
    if (rctx.init && rctx.fini && desc.size == 0) {
    struct hash_alg_common *halg = crypto_hash_alg_common(tfm);
    const uint8_t *sha_buf =
    (actx.alg == MXS_DCP_CONTROL1_HASH_SELECT_SHA1) ?
    sha1_null_hash : sha256_null_hash;
    memcpy(sdcp.coh.sha_out_buf, sha_buf, halg.digestsize);
    ret = 0;
    goto done_run;
    }
// Set HASH_TERM bit for last transfer block.
    if (rctx.fini) {
    digest_phys = dma_map_single(sdcp.dev, sdcp.coh.sha_out_buf,
    DCP_SHA_PAY_SZ, DMA_FROM_DEVICE);
    ret = dma_mapping_error(sdcp.dev, digest_phys);
    if (ret)
    goto done_run;
    desc.control0 |= MXS_DCP_CONTROL0_HASH_TERM;
    desc.payload = digest_phys;
    }
    ret = mxs_dcp_start_dma(actx);
    if (rctx.fini)
    dma_unmap_single(sdcp.dev, digest_phys, DCP_SHA_PAY_SZ,
    DMA_FROM_DEVICE);
    done_run:
    dma_unmap_single(sdcp.dev, buf_phys, DCP_BUF_SZ, DMA_TO_DEVICE);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dcp_sha_req_to_buf(arq: *mut crypto_async_request) -> c_int {
    static int dcp_sha_req_to_buf(struct crypto_async_request *arq)
    {
    struct dcp *sdcp = global_sdcp;
    struct ahash_request *req = ahash_request_cast(arq);
    struct crypto_ahash *tfm = crypto_ahash_reqtfm(req);
    struct dcp_async_ctx *actx = crypto_ahash_ctx(tfm);
    struct dcp_sha_req_ctx *rctx = ahash_request_ctx(req);
    struct hash_alg_common *halg = crypto_hash_alg_common(tfm);
    uint8_t *in_buf = sdcp.coh.sha_in_buf;
    uint8_t *out_buf = sdcp.coh.sha_out_buf;
    struct scatterlist *src;
    unsigned int i, len, clen, oft = 0;
    int ret;
    let mut fin: c_int = rctx.fini;
    if (fin)
    rctx.fini = 0;
    src = req.src;
    len = req.nbytes;
    while (len) {
    if (actx.fill + len > DCP_BUF_SZ)
    clen = DCP_BUF_SZ - actx.fill;
    else
    clen = len;
    scatterwalk_map_and_copy(in_buf + actx.fill, src, oft, clen,
    0);
    len -= clen;
    oft += clen;
    actx.fill += clen;
//
// If we filled the buffer and still have some
// more data, submit the buffer.
//
    if (len && actx.fill == DCP_BUF_SZ) {
    ret = mxs_dcp_run_sha(req);
    if (ret)
    return ret;
    actx.fill = 0;
    rctx.init = 0;
    }
    }
    if (fin) {
    rctx.fini = 1;
// Submit whatever is left.
    if (!req.result)
    return -EINVAL;
    ret = mxs_dcp_run_sha(req);
    if (ret)
    return ret;
    actx.fill = 0;
// For some reason the result is flipped
    for (i = 0; i < halg.digestsize; i++)
    req.result[i] = out_buf[halg.digestsize - i - 1];
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dcp_chan_thread_sha(data: *mut c_void) -> c_int {
    static int dcp_chan_thread_sha(void *data)
    {
    struct dcp *sdcp = global_sdcp;
    let mut chan: c_int = DCP_CHAN_HASH_SHA;
    struct crypto_async_request *backlog;
    struct crypto_async_request *arq;
    int ret;
    while (!kthread_should_stop()) {
    set_current_state(TASK_INTERRUPTIBLE);
    spin_lock(&sdcp.lock[chan]);
    backlog = crypto_get_backlog(&sdcp.queue[chan]);
    arq = crypto_dequeue_request(&sdcp.queue[chan]);
    spin_unlock(&sdcp.lock[chan]);
    if (!backlog && !arq) {
    schedule();
    continue;
    }
    set_current_state(TASK_RUNNING);
    if (backlog)
    crypto_request_complete(backlog, -EINPROGRESS);
    if (arq) {
    ret = dcp_sha_req_to_buf(arq);
    crypto_request_complete(arq, ret);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dcp_sha_init(req: *mut ahash_request) -> c_int {
    static int dcp_sha_init(struct ahash_request *req)
    {
    struct crypto_ahash *tfm = crypto_ahash_reqtfm(req);
    struct dcp_async_ctx *actx = crypto_ahash_ctx(tfm);
    struct hash_alg_common *halg = crypto_hash_alg_common(tfm);
//
// Start hashing session. The code below only inits the
// hashing session context, nothing more.
//
    memset(actx, 0, sizeof(*actx));
    if (strcmp(halg.base.cra_name, "sha1") == 0)
    actx.alg = MXS_DCP_CONTROL1_HASH_SELECT_SHA1;
    else
    actx.alg = MXS_DCP_CONTROL1_HASH_SELECT_SHA256;
    actx.fill = 0;
    actx.hot = 0;
    actx.chan = DCP_CHAN_HASH_SHA;
    mutex_init(&actx.mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dcp_sha_update_fx(req: *mut ahash_request, fini: c_int) -> c_int {
    static int dcp_sha_update_fx(struct ahash_request *req, int fini)
    {
    struct dcp *sdcp = global_sdcp;
    struct dcp_sha_req_ctx *rctx = ahash_request_ctx(req);
    struct crypto_ahash *tfm = crypto_ahash_reqtfm(req);
    struct dcp_async_ctx *actx = crypto_ahash_ctx(tfm);
    int ret;
//
// Ignore requests that have no data in them and are not
// the trailing requests in the stream of requests.
//
    if (!req.nbytes && !fini)
    return 0;
    mutex_lock(&actx.mutex);
    rctx.fini = fini;
    if (!actx.hot) {
    actx.hot = 1;
    rctx.init = 1;
    }
    spin_lock(&sdcp.lock[actx.chan]);
    ret = crypto_enqueue_request(&sdcp.queue[actx.chan], &req.base);
    spin_unlock(&sdcp.lock[actx.chan]);
    wake_up_process(sdcp.thread[actx.chan]);
    mutex_unlock(&actx.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dcp_sha_update(req: *mut ahash_request) -> c_int {
    static int dcp_sha_update(struct ahash_request *req)
    {
    return dcp_sha_update_fx(req, 0);
    }
#[no_mangle]
unsafe extern "C" fn dcp_sha_final(req: *mut ahash_request) -> c_int {
    static int dcp_sha_final(struct ahash_request *req)
    {
    ahash_request_set_crypt(req, core::ptr::null_mut(), req.result, 0);
    req.nbytes = 0;
    return dcp_sha_update_fx(req, 1);
    }
#[no_mangle]
unsafe extern "C" fn dcp_sha_finup(req: *mut ahash_request) -> c_int {
    static int dcp_sha_finup(struct ahash_request *req)
    {
    return dcp_sha_update_fx(req, 1);
    }
#[no_mangle]
unsafe extern "C" fn dcp_sha_digest(req: *mut ahash_request) -> c_int {
    static int dcp_sha_digest(struct ahash_request *req)
    {
    int ret;
    ret = dcp_sha_init(req);
    if (ret)
    return ret;
    return dcp_sha_finup(req);
    }
#[no_mangle]
unsafe extern "C" fn dcp_sha_import(req: *mut ahash_request, in: *const c_void) -> c_int {
    static int dcp_sha_import(struct ahash_request *req, const void *in)
    {
    struct dcp_sha_req_ctx *rctx = ahash_request_ctx(req);
    struct crypto_ahash *tfm = crypto_ahash_reqtfm(req);
    struct dcp_async_ctx *actx = crypto_ahash_ctx(tfm);
    const struct dcp_export_state *export = in;
    memset(rctx, 0, sizeof(struct dcp_sha_req_ctx));
    memset(actx, 0, sizeof(struct dcp_async_ctx));
    memcpy(rctx, &export.req_ctx, sizeof(struct dcp_sha_req_ctx));
    memcpy(actx, &export.async_ctx, sizeof(struct dcp_async_ctx));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dcp_sha_export(req: *mut ahash_request, out: *mut c_void) -> c_int {
    static int dcp_sha_export(struct ahash_request *req, void *out)
    {
    struct dcp_sha_req_ctx *rctx_state = ahash_request_ctx(req);
    struct crypto_ahash *tfm = crypto_ahash_reqtfm(req);
    struct dcp_async_ctx *actx_state = crypto_ahash_ctx(tfm);
    struct dcp_export_state *export = out;
    memcpy(&export.req_ctx, rctx_state, sizeof(struct dcp_sha_req_ctx));
    memcpy(&export.async_ctx, actx_state, sizeof(struct dcp_async_ctx));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dcp_sha_cra_init(tfm: *mut crypto_tfm) -> c_int {
    static int dcp_sha_cra_init(struct crypto_tfm *tfm)
    {
    crypto_ahash_set_reqsize(__crypto_ahash_cast(tfm),
    sizeof(struct dcp_sha_req_ctx));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dcp_sha_cra_exit(tfm: *mut crypto_tfm) {
    static void dcp_sha_cra_exit(struct crypto_tfm *tfm)
    {
    }
// AES 128 ECB and AES 128 CBC
    static struct skcipher_alg dcp_aes_algs[] = {
    {
    .base.cra_name		= "ecb(aes)",
    .base.cra_driver_name	= "ecb-aes-dcp",
    .base.cra_priority	= 400,
    .base.cra_alignmask	= 15,
    .base.cra_flags		= CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_NEED_FALLBACK,
    .base.cra_blocksize	= AES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct dcp_async_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= AES_MIN_KEY_SIZE,
    .max_keysize		= AES_MAX_KEY_SIZE,
    .setkey			= mxs_dcp_aes_setkey,
    .encrypt		= mxs_dcp_aes_ecb_encrypt,
    .decrypt		= mxs_dcp_aes_ecb_decrypt,
    .init			= mxs_dcp_aes_fallback_init_tfm,
    .exit			= mxs_dcp_aes_fallback_exit_tfm,
    }, {
    .base.cra_name		= "cbc(aes)",
    .base.cra_driver_name	= "cbc-aes-dcp",
    .base.cra_priority	= 400,
    .base.cra_alignmask	= 15,
    .base.cra_flags		= CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_NEED_FALLBACK,
    .base.cra_blocksize	= AES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct dcp_async_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= AES_MIN_KEY_SIZE,
    .max_keysize		= AES_MAX_KEY_SIZE,
    .setkey			= mxs_dcp_aes_setkey,
    .encrypt		= mxs_dcp_aes_cbc_encrypt,
    .decrypt		= mxs_dcp_aes_cbc_decrypt,
    .ivsize			= AES_BLOCK_SIZE,
    .init			= mxs_dcp_aes_fallback_init_tfm,
    .exit			= mxs_dcp_aes_fallback_exit_tfm,
    }, {
    .base.cra_name		= "ecb(paes)",
    .base.cra_driver_name	= "ecb-paes-dcp",
    .base.cra_priority	= 401,
    .base.cra_alignmask	= 15,
    .base.cra_flags		= CRYPTO_ALG_ASYNC | CRYPTO_ALG_INTERNAL,
    .base.cra_blocksize	= AES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct dcp_async_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= DCP_PAES_KEYSIZE,
    .max_keysize		= DCP_PAES_KEYSIZE,
    .setkey			= mxs_dcp_aes_setrefkey,
    .encrypt		= mxs_dcp_aes_ecb_encrypt,
    .decrypt		= mxs_dcp_aes_ecb_decrypt,
    .init			= mxs_dcp_paes_init_tfm,
    }, {
    .base.cra_name		= "cbc(paes)",
    .base.cra_driver_name	= "cbc-paes-dcp",
    .base.cra_priority	= 401,
    .base.cra_alignmask	= 15,
    .base.cra_flags		= CRYPTO_ALG_ASYNC | CRYPTO_ALG_INTERNAL,
    .base.cra_blocksize	= AES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct dcp_async_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= DCP_PAES_KEYSIZE,
    .max_keysize		= DCP_PAES_KEYSIZE,
    .setkey			= mxs_dcp_aes_setrefkey,
    .encrypt		= mxs_dcp_aes_cbc_encrypt,
    .decrypt		= mxs_dcp_aes_cbc_decrypt,
    .ivsize			= AES_BLOCK_SIZE,
    .init			= mxs_dcp_paes_init_tfm,
    },
    };
// SHA1
    static struct ahash_alg dcp_sha1_alg = {
    .init	= dcp_sha_init,
    .update	= dcp_sha_update,
    .final	= dcp_sha_final,
    .finup	= dcp_sha_finup,
    .digest	= dcp_sha_digest,
    .import = dcp_sha_import,
    .export = dcp_sha_export,
    .halg	= {
    .digestsize	= SHA1_DIGEST_SIZE,
    .statesize	= sizeof(struct dcp_export_state),
    .base		= {
    .cra_name		= "sha1",
    .cra_driver_name	= "sha1-dcp",
    .cra_priority		= 400,
    .cra_flags		= CRYPTO_ALG_ASYNC,
    .cra_blocksize		= SHA1_BLOCK_SIZE,
    .cra_ctxsize		= sizeof(struct dcp_async_ctx),
    .cra_module		= THIS_MODULE,
    .cra_init		= dcp_sha_cra_init,
    .cra_exit		= dcp_sha_cra_exit,
    },
    },
    };
// SHA256
    static struct ahash_alg dcp_sha256_alg = {
    .init	= dcp_sha_init,
    .update	= dcp_sha_update,
    .final	= dcp_sha_final,
    .finup	= dcp_sha_finup,
    .digest	= dcp_sha_digest,
    .import = dcp_sha_import,
    .export = dcp_sha_export,
    .halg	= {
    .digestsize	= SHA256_DIGEST_SIZE,
    .statesize	= sizeof(struct dcp_export_state),
    .base		= {
    .cra_name		= "sha256",
    .cra_driver_name	= "sha256-dcp",
    .cra_priority		= 400,
    .cra_flags		= CRYPTO_ALG_ASYNC,
    .cra_blocksize		= SHA256_BLOCK_SIZE,
    .cra_ctxsize		= sizeof(struct dcp_async_ctx),
    .cra_module		= THIS_MODULE,
    .cra_init		= dcp_sha_cra_init,
    .cra_exit		= dcp_sha_cra_exit,
    },
    },
    };
#[no_mangle]
unsafe extern "C" fn mxs_dcp_irq(irq: c_int, context: *mut c_void) -> irqreturn_t {
    static irqreturn_t mxs_dcp_irq(int irq, void *context)
    {
    struct dcp *sdcp = context;
    uint32_t stat;
    int i;
    stat = readl(sdcp.base + MXS_DCP_STAT);
    stat &= MXS_DCP_STAT_IRQ_MASK;
    if (!stat)
    return IRQ_NONE;
// Clear the interrupts.
    writel(stat, sdcp.base + MXS_DCP_STAT_CLR);
// Complete the DMA requests that finished.
    for (i = 0; i < DCP_MAX_CHANS; i++)
    if (stat & (1 << i))
    complete(&sdcp.completion[i]);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mxs_dcp_probe(pdev: *mut platform_device) -> c_int {
    static int mxs_dcp_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct dcp *sdcp = core::ptr::null_mut();
    int i, ret;
    int dcp_vmi_irq, dcp_irq;
    if (global_sdcp) {
    dev_err(dev, "Only one DCP instance allowed!\n");
    return -ENODEV;
    }
    dcp_vmi_irq = platform_get_irq(pdev, 0);
    if (dcp_vmi_irq < 0)
    return dcp_vmi_irq;
    dcp_irq = platform_get_irq(pdev, 1);
    if (dcp_irq < 0)
    return dcp_irq;
    sdcp = devm_kzalloc(dev, sizeof(*sdcp), GFP_KERNEL);
    if (!sdcp)
    return -ENOMEM;
    sdcp.dev = dev;
    sdcp.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(sdcp.base))
    return PTR_ERR(sdcp.base);
    ret = devm_request_irq(dev, dcp_vmi_irq, mxs_dcp_irq, 0,
    "dcp-vmi-irq", sdcp);
    if (ret)
    return ret;
    ret = devm_request_irq(dev, dcp_irq, mxs_dcp_irq, 0,
    "dcp-irq", sdcp);
    if (ret)
    return ret;
// Allocate coherent helper block.
    sdcp.coh = devm_kzalloc(dev, sizeof(*sdcp.coh) + DCP_ALIGNMENT,
    GFP_KERNEL);
    if (!sdcp.coh)
    return -ENOMEM;
// Re-align the structure so it fits the DCP constraints.
    sdcp.coh = PTR_ALIGN(sdcp.coh, DCP_ALIGNMENT);
// DCP clock is optional, only used on some SOCs
    sdcp.dcp_clk = devm_clk_get_optional_enabled(dev, "dcp");
    if (IS_ERR(sdcp.dcp_clk))
    return PTR_ERR(sdcp.dcp_clk);
// Restart the DCP block.
    ret = stmp_reset_block(sdcp.base);
    if (ret) {
    dev_err(dev, "Failed reset\n");
    return ret;
    }
// Initialize control register.
    writel(MXS_DCP_CTRL_GATHER_RESIDUAL_WRITES |
    MXS_DCP_CTRL_ENABLE_CONTEXT_CACHING | 0xf,
    sdcp.base + MXS_DCP_CTRL);
// Enable all DCP DMA channels.
    writel(MXS_DCP_CHANNELCTRL_ENABLE_CHANNEL_MASK,
    sdcp.base + MXS_DCP_CHANNELCTRL);
//
// We do not enable context switching. Give the context buffer a
// pointer to an illegal address so if context switching is
// inadvertantly enabled, the DCP will return an error instead of
// trashing good memory. The DCP DMA cannot access ROM, so any ROM
// address will do.
//
    writel(0xffff0000, sdcp.base + MXS_DCP_CONTEXT);
    for (i = 0; i < DCP_MAX_CHANS; i++)
    writel(0xffffffff, sdcp.base + MXS_DCP_CH_N_STAT_CLR(i));
    writel(0xffffffff, sdcp.base + MXS_DCP_STAT_CLR);
    global_sdcp = sdcp;
    platform_set_drvdata(pdev, sdcp);
    for (i = 0; i < DCP_MAX_CHANS; i++) {
    spin_lock_init(&sdcp.lock[i]);
    init_completion(&sdcp.completion[i]);
    crypto_init_queue(&sdcp.queue[i], 50);
    }
// Create the SHA and AES handler threads.
    sdcp.thread[DCP_CHAN_HASH_SHA] = kthread_run(dcp_chan_thread_sha,
    core::ptr::null_mut(), "mxs_dcp_chan/sha");
    if (IS_ERR(sdcp.thread[DCP_CHAN_HASH_SHA])) {
    dev_err(dev, "Error starting SHA thread!\n");
    ret = PTR_ERR(sdcp.thread[DCP_CHAN_HASH_SHA]);
    return ret;
    }
    sdcp.thread[DCP_CHAN_CRYPTO] = kthread_run(dcp_chan_thread_aes,
    core::ptr::null_mut(), "mxs_dcp_chan/aes");
    if (IS_ERR(sdcp.thread[DCP_CHAN_CRYPTO])) {
    dev_err(dev, "Error starting SHA thread!\n");
    ret = PTR_ERR(sdcp.thread[DCP_CHAN_CRYPTO]);
    goto err_destroy_sha_thread;
    }
// Register the various crypto algorithms.
    sdcp.caps = readl(sdcp.base + MXS_DCP_CAPABILITY1);
    if (sdcp.caps & MXS_DCP_CAPABILITY1_AES128) {
    ret = crypto_register_skciphers(dcp_aes_algs,
    ARRAY_SIZE(dcp_aes_algs));
    if (ret) {
// Failed to register algorithm.
    dev_err(dev, "Failed to register AES crypto!\n");
    goto err_destroy_aes_thread;
    }
    }
    if (sdcp.caps & MXS_DCP_CAPABILITY1_SHA1) {
    ret = crypto_register_ahash(&dcp_sha1_alg);
    if (ret) {
    dev_err(dev, "Failed to register %s hash!\n",
    dcp_sha1_alg.halg.base.cra_name);
    goto err_unregister_aes;
    }
    }
    if (sdcp.caps & MXS_DCP_CAPABILITY1_SHA256) {
    ret = crypto_register_ahash(&dcp_sha256_alg);
    if (ret) {
    dev_err(dev, "Failed to register %s hash!\n",
    dcp_sha256_alg.halg.base.cra_name);
    goto err_unregister_sha1;
    }
    }
    return 0;
    err_unregister_sha1:
    if (sdcp.caps & MXS_DCP_CAPABILITY1_SHA1)
    crypto_unregister_ahash(&dcp_sha1_alg);
    err_unregister_aes:
    if (sdcp.caps & MXS_DCP_CAPABILITY1_AES128)
    crypto_unregister_skciphers(dcp_aes_algs, ARRAY_SIZE(dcp_aes_algs));
    err_destroy_aes_thread:
    kthread_stop(sdcp.thread[DCP_CHAN_CRYPTO]);
    err_destroy_sha_thread:
    kthread_stop(sdcp.thread[DCP_CHAN_HASH_SHA]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mxs_dcp_remove(pdev: *mut platform_device) {
    static void mxs_dcp_remove(struct platform_device *pdev)
    {
    struct dcp *sdcp = platform_get_drvdata(pdev);
    if (sdcp.caps & MXS_DCP_CAPABILITY1_SHA256)
    crypto_unregister_ahash(&dcp_sha256_alg);
    if (sdcp.caps & MXS_DCP_CAPABILITY1_SHA1)
    crypto_unregister_ahash(&dcp_sha1_alg);
    if (sdcp.caps & MXS_DCP_CAPABILITY1_AES128)
    crypto_unregister_skciphers(dcp_aes_algs, ARRAY_SIZE(dcp_aes_algs));
    kthread_stop(sdcp.thread[DCP_CHAN_HASH_SHA]);
    kthread_stop(sdcp.thread[DCP_CHAN_CRYPTO]);
    platform_set_drvdata(pdev, core::ptr::null_mut());
    global_sdcp = core::ptr::null_mut();
    }
    static const struct of_device_id mxs_dcp_dt_ids[] = {
    { .compatible = "fsl,imx23-dcp", .data = core::ptr::null_mut(), },
    { .compatible = "fsl,imx28-dcp", .data = core::ptr::null_mut(), },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, mxs_dcp_dt_ids);
    static struct platform_driver mxs_dcp_driver = {
    .probe	= mxs_dcp_probe,
    .remove = mxs_dcp_remove,
    .driver	= {
    .name		= "mxs-dcp",
    .of_match_table	= mxs_dcp_dt_ids,
    },
    };
    module_platform_driver(mxs_dcp_driver);
    MODULE_AUTHOR("Marek Vasut <marex@denx.de>");
    MODULE_DESCRIPTION("Freescale MXS DCP Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:mxs-dcp");
