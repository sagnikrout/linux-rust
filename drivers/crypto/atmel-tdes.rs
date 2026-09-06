//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/atmel-tdes.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Cryptographic API.
//
// Support for ATMEL DES/TDES HW acceleration.
//
// Copyright (c) 2012 Eukréa Electromatique - ATMEL
// Author: Nicolas Royer <nicolas@eukrea.com>
//
// Some ideas are from omap-aes.c drivers.
//

pub const ATMEL_TDES_PRIORITY: c_int = 300;
// TDES flags
// Reserve bits [17:16], [13:12], [2:0] for AES Mode Register

pub const ATMEL_TDES_QUEUE_LENGTH: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_tdes_caps {
    pub has_dma: bool,
}

    struct atmel_tdes_dev;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_tdes_ctx {
    pub dd: *mut atmel_tdes_dev,
    pub keylen: c_int,
    pub sizeof(u32)]: u32 key[DES3_EDE_KEY_SIZE /,
    pub flags: c_ulong,
    pub block_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_tdes_reqctx {
    pub mode: c_ulong,
    pub lastc: [u8; DES_BLOCK_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_tdes_dma {
    pub chan: *mut dma_chan,
    pub dma_conf: dma_slave_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_tdes_dev {
    pub list: list_head,
    pub phys_base: c_ulong,
    pub io_base: *mut void __iomem,
    pub ctx: *mut atmel_tdes_ctx,
    pub dev: *mut device,
    pub iclk: *mut clk,
    pub irq: c_int,
    pub flags: c_ulong,
    pub lock: spinlock_t,
    pub queue: crypto_queue,
    pub done_task: tasklet_struct,
    pub queue_task: tasklet_struct,
    pub req: *mut skcipher_request,
    pub total: usize,
    pub in_sg: *mut scatterlist,
    pub nb_in_sg: c_uint,
    pub in_offset: usize,
    pub out_sg: *mut scatterlist,
    pub nb_out_sg: c_uint,
    pub out_offset: usize,
    pub buflen: usize,
    pub dma_size: usize,
    pub buf_in: *mut c_void,
    pub dma_in: c_int,
    pub dma_addr_in: dma_addr_t,
    pub dma_lch_in: atmel_tdes_dma,
    pub buf_out: *mut c_void,
    pub dma_out: c_int,
    pub dma_addr_out: dma_addr_t,
    pub dma_lch_out: atmel_tdes_dma,
    pub caps: atmel_tdes_caps,
    pub hw_version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_tdes_drv {
    pub dev_list: list_head,
    pub lock: spinlock_t,
}

    static struct atmel_tdes_drv atmel_tdes = {
    .dev_list = LIST_HEAD_INIT(atmel_tdes.dev_list),
    .lock = __SPIN_LOCK_UNLOCKED(atmel_tdes.lock),
    };
    static int atmel_tdes_sg_copy(struct scatterlist **sg, size_t *offset,
    void *buf, size_t buflen, size_t total, int out)
    {
    size_t count, off = 0;
    while (buflen && total) {
    count = min((*sg).length - *offset, total);
    count = min(count, buflen);
    if (!count)
    return off;
    scatterwalk_map_and_copy(buf + off, *sg, *offset, count, out);
    off += count;
    buflen -= count;
// offset += count;
    total -= count;
    if (*offset == (*sg).length) {
// sg = sg_next(*sg);
    if (*sg)
// offset = 0;
    else
    total = 0;
    }
    }
    return off;
    }
#[no_mangle]
pub unsafe extern "C" fn atmel_tdes_read(dd: *mut atmel_tdes_dev, offset: u32) -> u32 {
    static inline u32 atmel_tdes_read(struct atmel_tdes_dev *dd, u32 offset)
    {
    return readl_relaxed(dd.io_base + offset);
    }
    static inline void atmel_tdes_write(struct atmel_tdes_dev *dd,
    u32 offset, u32 value)
    {
    writel_relaxed(value, dd.io_base + offset);
    }
    static void atmel_tdes_write_n(struct atmel_tdes_dev *dd, u32 offset,
    const u32 *value, int count)
    {
    for (; count--; value++, offset += 4)
    atmel_tdes_write(dd, offset, *value);
    }
    static struct atmel_tdes_dev *atmel_tdes_dev_alloc(void)
    {
    struct atmel_tdes_dev *tdes_dd;
    spin_lock_bh(&atmel_tdes.lock);
// One TDES IP per SoC.
    tdes_dd = list_first_entry_or_null(&atmel_tdes.dev_list,
    struct atmel_tdes_dev, list);
    spin_unlock_bh(&atmel_tdes.lock);
    return tdes_dd;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_hw_init(dd: *mut atmel_tdes_dev) -> c_int {
    static int atmel_tdes_hw_init(struct atmel_tdes_dev *dd)
    {
    int err;
    err = clk_prepare_enable(dd.iclk);
    if (err)
    return err;
    if (!(dd.flags & TDES_FLAGS_INIT)) {
    atmel_tdes_write(dd, TDES_CR, TDES_CR_SWRST);
    dd.flags |= TDES_FLAGS_INIT;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn atmel_tdes_get_version(dd: *mut atmel_tdes_dev) -> c_uint {
    static inline unsigned int atmel_tdes_get_version(struct atmel_tdes_dev *dd)
    {
    return atmel_tdes_read(dd, TDES_HW_VERSION) & 0x00000fff;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_hw_version_init(dd: *mut atmel_tdes_dev) -> c_int {
    static int atmel_tdes_hw_version_init(struct atmel_tdes_dev *dd)
    {
    int err;
    err = atmel_tdes_hw_init(dd);
    if (err)
    return err;
    dd.hw_version = atmel_tdes_get_version(dd);
    dev_info(dd.dev,
    "version: 0x%x\n", dd.hw_version);
    clk_disable_unprepare(dd.iclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_dma_callback(data: *mut c_void) {
    static void atmel_tdes_dma_callback(void *data)
    {
    struct atmel_tdes_dev *dd = data;
// dma_lch_out - completed
    tasklet_schedule(&dd.done_task);
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_write_ctrl(dd: *mut atmel_tdes_dev) -> c_int {
    static int atmel_tdes_write_ctrl(struct atmel_tdes_dev *dd)
    {
    int err;
    let mut valmr: u32 = TDES_MR_SMOD_PDC;
    err = atmel_tdes_hw_init(dd);
    if (err)
    return err;
    if (!dd.caps.has_dma)
    atmel_tdes_write(dd, TDES_PTCR,
    TDES_PTCR_TXTDIS | TDES_PTCR_RXTDIS);
// MR register must be set before IV registers
    if (dd.ctx.keylen > (DES_KEY_SIZE << 1)) {
    valmr |= TDES_MR_KEYMOD_3KEY;
    valmr |= TDES_MR_TDESMOD_TDES;
    } else if (dd.ctx.keylen > DES_KEY_SIZE) {
    valmr |= TDES_MR_KEYMOD_2KEY;
    valmr |= TDES_MR_TDESMOD_TDES;
    } else {
    valmr |= TDES_MR_TDESMOD_DES;
    }
    valmr |= dd.flags & TDES_FLAGS_MODE_MASK;
    atmel_tdes_write(dd, TDES_MR, valmr);
    atmel_tdes_write_n(dd, TDES_KEY1W1R, dd.ctx.key,
    dd.ctx.keylen >> 2);
    if (dd.req.iv && (valmr & TDES_MR_OPMOD_MASK) != TDES_MR_OPMOD_ECB)
    atmel_tdes_write_n(dd, TDES_IV1R, (void *)dd.req.iv, 2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_crypt_pdc_stop(dd: *mut atmel_tdes_dev) -> c_int {
    static int atmel_tdes_crypt_pdc_stop(struct atmel_tdes_dev *dd)
    {
    size_t count;
    atmel_tdes_write(dd, TDES_PTCR, TDES_PTCR_TXTDIS|TDES_PTCR_RXTDIS);
    if (dd.flags & TDES_FLAGS_FAST) {
    dma_unmap_sg(dd.dev, dd.out_sg, 1, DMA_FROM_DEVICE);
    dma_unmap_sg(dd.dev, dd.in_sg, 1, DMA_TO_DEVICE);
    } else {
    dma_sync_single_for_cpu(dd.dev, dd.dma_addr_out,
    dd.dma_size, DMA_FROM_DEVICE);
    count = atmel_tdes_sg_copy(&dd.out_sg, &dd.out_offset,
    dd.buf_out, dd.buflen, dd.dma_size, 1);
    if (count != dd.dma_size) {
    dev_dbg(dd.dev, "not all data converted: %zu\n", count);
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_buff_init(dd: *mut atmel_tdes_dev) -> c_int {
    static int atmel_tdes_buff_init(struct atmel_tdes_dev *dd)
    {
    let mut err: c_int = -ENOMEM;
    dd.buf_in = (void *)__get_free_page(GFP_KERNEL);
    dd.buf_out = (void *)__get_free_page(GFP_KERNEL);
    dd.buflen = PAGE_SIZE;
    dd.buflen &= ~(DES_BLOCK_SIZE - 1);
    if (!dd.buf_in || !dd.buf_out) {
    dev_dbg(dd.dev, "unable to alloc pages.\n");
    goto err_alloc;
    }
// MAP here
    dd.dma_addr_in = dma_map_single(dd.dev, dd.buf_in,
    dd.buflen, DMA_TO_DEVICE);
    err = dma_mapping_error(dd.dev, dd.dma_addr_in);
    if (err) {
    dev_dbg(dd.dev, "dma %zd bytes error\n", dd.buflen);
    goto err_map_in;
    }
    dd.dma_addr_out = dma_map_single(dd.dev, dd.buf_out,
    dd.buflen, DMA_FROM_DEVICE);
    err = dma_mapping_error(dd.dev, dd.dma_addr_out);
    if (err) {
    dev_dbg(dd.dev, "dma %zd bytes error\n", dd.buflen);
    goto err_map_out;
    }
    return 0;
    err_map_out:
    dma_unmap_single(dd.dev, dd.dma_addr_in, dd.buflen,
    DMA_TO_DEVICE);
    err_map_in:
    err_alloc:
    free_page((unsigned long)dd.buf_out);
    free_page((unsigned long)dd.buf_in);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_buff_cleanup(dd: *mut atmel_tdes_dev) {
    static void atmel_tdes_buff_cleanup(struct atmel_tdes_dev *dd)
    {
    dma_unmap_single(dd.dev, dd.dma_addr_out, dd.buflen,
    DMA_FROM_DEVICE);
    dma_unmap_single(dd.dev, dd.dma_addr_in, dd.buflen,
    DMA_TO_DEVICE);
    free_page((unsigned long)dd.buf_out);
    free_page((unsigned long)dd.buf_in);
    }
    static int atmel_tdes_crypt_pdc(struct atmel_tdes_dev *dd,
    dma_addr_t dma_addr_in,
    dma_addr_t dma_addr_out, int length)
    {
    int len32;
    dd.dma_size = length;
    if (!(dd.flags & TDES_FLAGS_FAST)) {
    dma_sync_single_for_device(dd.dev, dma_addr_in, length,
    DMA_TO_DEVICE);
    }
    len32 = DIV_ROUND_UP(length, sizeof(u32));
    atmel_tdes_write(dd, TDES_PTCR, TDES_PTCR_TXTDIS|TDES_PTCR_RXTDIS);
    atmel_tdes_write(dd, TDES_TPR, dma_addr_in);
    atmel_tdes_write(dd, TDES_TCR, len32);
    atmel_tdes_write(dd, TDES_RPR, dma_addr_out);
    atmel_tdes_write(dd, TDES_RCR, len32);
// Enable Interrupt
    atmel_tdes_write(dd, TDES_IER, TDES_INT_ENDRX);
// Start DMA transfer
    atmel_tdes_write(dd, TDES_PTCR, TDES_PTCR_TXTEN | TDES_PTCR_RXTEN);
    return 0;
    }
    static int atmel_tdes_crypt_dma(struct atmel_tdes_dev *dd,
    dma_addr_t dma_addr_in,
    dma_addr_t dma_addr_out, int length)
    {
    struct scatterlist sg[2];
    struct dma_async_tx_descriptor	*in_desc, *out_desc;
    enum dma_slave_buswidth addr_width;
    dd.dma_size = length;
    if (!(dd.flags & TDES_FLAGS_FAST)) {
    dma_sync_single_for_device(dd.dev, dma_addr_in, length,
    DMA_TO_DEVICE);
    }
    addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    dd.dma_lch_in.dma_conf.dst_addr_width = addr_width;
    dd.dma_lch_out.dma_conf.src_addr_width = addr_width;
    dmaengine_slave_config(dd.dma_lch_in.chan, &dd.dma_lch_in.dma_conf);
    dmaengine_slave_config(dd.dma_lch_out.chan, &dd.dma_lch_out.dma_conf);
    dd.flags |= TDES_FLAGS_DMA;
    sg_init_table(&sg[0], 1);
    sg_dma_address(&sg[0]) = dma_addr_in;
    sg_dma_len(&sg[0]) = length;
    sg_init_table(&sg[1], 1);
    sg_dma_address(&sg[1]) = dma_addr_out;
    sg_dma_len(&sg[1]) = length;
    in_desc = dmaengine_prep_slave_sg(dd.dma_lch_in.chan, &sg[0],
    1, DMA_MEM_TO_DEV,
    DMA_PREP_INTERRUPT  |  DMA_CTRL_ACK);
    if (!in_desc)
    return -EINVAL;
    out_desc = dmaengine_prep_slave_sg(dd.dma_lch_out.chan, &sg[1],
    1, DMA_DEV_TO_MEM,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!out_desc)
    return -EINVAL;
    out_desc.callback = atmel_tdes_dma_callback;
    out_desc.callback_param = dd;
    dmaengine_submit(out_desc);
    dma_async_issue_pending(dd.dma_lch_out.chan);
    dmaengine_submit(in_desc);
    dma_async_issue_pending(dd.dma_lch_in.chan);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_crypt_start(dd: *mut atmel_tdes_dev) -> c_int {
    static int atmel_tdes_crypt_start(struct atmel_tdes_dev *dd)
    {
    bool fast;
    int err;
    size_t count;
    dma_addr_t addr_in, addr_out;
    fast = !dd.in_offset && !dd.out_offset &&
    dd.in_sg.length == dd.out_sg.length &&
    IS_ALIGNED(dd.in_sg.offset, sizeof(u32)) &&
    IS_ALIGNED(dd.out_sg.offset, sizeof(u32)) &&
    IS_ALIGNED(dd.in_sg.length, dd.ctx.block_size);
    if (fast) {
    count = min(dd.total, dd.in_sg.length);
    err = dma_map_sg(dd.dev, dd.in_sg, 1, DMA_TO_DEVICE);
    if (!err) {
    dev_dbg(dd.dev, "dma_map_sg() error\n");
    return -EINVAL;
    }
    err = dma_map_sg(dd.dev, dd.out_sg, 1,
    DMA_FROM_DEVICE);
    if (!err) {
    dev_dbg(dd.dev, "dma_map_sg() error\n");
    dma_unmap_sg(dd.dev, dd.in_sg, 1,
    DMA_TO_DEVICE);
    return -EINVAL;
    }
    addr_in = sg_dma_address(dd.in_sg);
    addr_out = sg_dma_address(dd.out_sg);
    dd.flags |= TDES_FLAGS_FAST;
    } else {
// use cache buffers
    count = atmel_tdes_sg_copy(&dd.in_sg, &dd.in_offset,
    dd.buf_in, dd.buflen, dd.total, 0);
    addr_in = dd.dma_addr_in;
    addr_out = dd.dma_addr_out;
    dd.flags &= ~TDES_FLAGS_FAST;
    }
    dd.total -= count;
    if (dd.caps.has_dma)
    err = atmel_tdes_crypt_dma(dd, addr_in, addr_out, count);
    else
    err = atmel_tdes_crypt_pdc(dd, addr_in, addr_out, count);
    if (err && (dd.flags & TDES_FLAGS_FAST)) {
    dma_unmap_sg(dd.dev, dd.in_sg, 1, DMA_TO_DEVICE);
    dma_unmap_sg(dd.dev, dd.out_sg, 1, DMA_FROM_DEVICE);
    }
    return err;
    }
    static void
    atmel_tdes_set_iv_as_last_ciphertext_block(struct atmel_tdes_dev *dd)
    {
    struct skcipher_request *req = dd.req;
    struct atmel_tdes_reqctx *rctx = skcipher_request_ctx(req);
    struct crypto_skcipher *skcipher = crypto_skcipher_reqtfm(req);
    let mut ivsize: c_uint = crypto_skcipher_ivsize(skcipher);
    if (req.cryptlen < ivsize)
    return;
    if (rctx.mode & TDES_FLAGS_ENCRYPT)
    scatterwalk_map_and_copy(req.iv, req.dst,
    req.cryptlen - ivsize, ivsize, 0);
    else
    memcpy(req.iv, rctx.lastc, ivsize);
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_finish_req(dd: *mut atmel_tdes_dev, err: c_int) {
    static void atmel_tdes_finish_req(struct atmel_tdes_dev *dd, int err)
    {
    struct skcipher_request *req = dd.req;
    struct atmel_tdes_reqctx *rctx = skcipher_request_ctx(req);
    clk_disable_unprepare(dd.iclk);
    dd.flags &= ~TDES_FLAGS_BUSY;
    if (!err && (rctx.mode & TDES_FLAGS_OPMODE_MASK) != TDES_FLAGS_ECB)
    atmel_tdes_set_iv_as_last_ciphertext_block(dd);
    skcipher_request_complete(req, err);
    }
    static int atmel_tdes_handle_queue(struct atmel_tdes_dev *dd,
    struct skcipher_request *req)
    {
    struct crypto_async_request *async_req, *backlog;
    struct atmel_tdes_ctx *ctx;
    struct atmel_tdes_reqctx *rctx;
    unsigned long flags;
    int err, ret = 0;
    spin_lock_irqsave(&dd.lock, flags);
    if (req)
    ret = crypto_enqueue_request(&dd.queue, &req.base);
    if (dd.flags & TDES_FLAGS_BUSY) {
    spin_unlock_irqrestore(&dd.lock, flags);
    return ret;
    }
    backlog = crypto_get_backlog(&dd.queue);
    async_req = crypto_dequeue_request(&dd.queue);
    if (async_req)
    dd.flags |= TDES_FLAGS_BUSY;
    spin_unlock_irqrestore(&dd.lock, flags);
    if (!async_req)
    return ret;
    if (backlog)
    crypto_request_complete(backlog, -EINPROGRESS);
    req = skcipher_request_cast(async_req);
// assign new request to device
    dd.req = req;
    dd.total = req.cryptlen;
    dd.in_offset = 0;
    dd.in_sg = req.src;
    dd.out_offset = 0;
    dd.out_sg = req.dst;
    rctx = skcipher_request_ctx(req);
    ctx = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req));
    rctx.mode &= TDES_FLAGS_MODE_MASK;
    dd.flags = (dd.flags & ~TDES_FLAGS_MODE_MASK) | rctx.mode;
    dd.ctx = ctx;
    err = atmel_tdes_write_ctrl(dd);
    if (!err)
    err = atmel_tdes_crypt_start(dd);
    if (err) {
// des_task will not finish it, so do it here
    atmel_tdes_finish_req(dd, err);
    tasklet_schedule(&dd.queue_task);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_crypt_dma_stop(dd: *mut atmel_tdes_dev) -> c_int {
    static int atmel_tdes_crypt_dma_stop(struct atmel_tdes_dev *dd)
    {
    size_t count;
    if  (dd.flags & TDES_FLAGS_FAST) {
    dma_unmap_sg(dd.dev, dd.out_sg, 1, DMA_FROM_DEVICE);
    dma_unmap_sg(dd.dev, dd.in_sg, 1, DMA_TO_DEVICE);
    } else {
    dma_sync_single_for_cpu(dd.dev, dd.dma_addr_out, dd.dma_size,
    DMA_FROM_DEVICE);
    count = atmel_tdes_sg_copy(&dd.out_sg, &dd.out_offset,
    dd.buf_out, dd.buflen,
    dd.dma_size, 1);
    if (count != dd.dma_size) {
    dev_dbg(dd.dev, "not all data converted: %zu\n", count);
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_crypt(req: *mut skcipher_request, mode: c_ulong) -> c_int {
    static int atmel_tdes_crypt(struct skcipher_request *req, unsigned long mode)
    {
    struct crypto_skcipher *skcipher = crypto_skcipher_reqtfm(req);
    struct atmel_tdes_ctx *ctx = crypto_skcipher_ctx(skcipher);
    struct atmel_tdes_reqctx *rctx = skcipher_request_ctx(req);
    struct device *dev = ctx.dd.dev;
    if (!req.cryptlen)
    return 0;
    if (!IS_ALIGNED(req.cryptlen, DES_BLOCK_SIZE)) {
    dev_dbg(dev, "request size is not exact amount of DES blocks\n");
    return -EINVAL;
    }
    ctx.block_size = DES_BLOCK_SIZE;
    rctx.mode = mode;
    if ((mode & TDES_FLAGS_OPMODE_MASK) != TDES_FLAGS_ECB &&
    !(mode & TDES_FLAGS_ENCRYPT)) {
    let mut ivsize: c_uint = crypto_skcipher_ivsize(skcipher);
    if (req.cryptlen >= ivsize)
    scatterwalk_map_and_copy(rctx.lastc, req.src,
    req.cryptlen - ivsize,
    ivsize, 0);
    }
    return atmel_tdes_handle_queue(ctx.dd, req);
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_dma_init(dd: *mut atmel_tdes_dev) -> c_int {
    static int atmel_tdes_dma_init(struct atmel_tdes_dev *dd)
    {
    int ret;
// Try to grab 2 DMA channels
    dd.dma_lch_in.chan = dma_request_chan(dd.dev, "tx");
    if (IS_ERR(dd.dma_lch_in.chan)) {
    ret = PTR_ERR(dd.dma_lch_in.chan);
    goto err_dma_in;
    }
    dd.dma_lch_in.dma_conf.dst_addr = dd.phys_base +
    TDES_IDATA1R;
    dd.dma_lch_in.dma_conf.src_maxburst = 1;
    dd.dma_lch_in.dma_conf.src_addr_width =
    DMA_SLAVE_BUSWIDTH_4_BYTES;
    dd.dma_lch_in.dma_conf.dst_maxburst = 1;
    dd.dma_lch_in.dma_conf.dst_addr_width =
    DMA_SLAVE_BUSWIDTH_4_BYTES;
    dd.dma_lch_in.dma_conf.device_fc = false;
    dd.dma_lch_out.chan = dma_request_chan(dd.dev, "rx");
    if (IS_ERR(dd.dma_lch_out.chan)) {
    ret = PTR_ERR(dd.dma_lch_out.chan);
    goto err_dma_out;
    }
    dd.dma_lch_out.dma_conf.src_addr = dd.phys_base +
    TDES_ODATA1R;
    dd.dma_lch_out.dma_conf.src_maxburst = 1;
    dd.dma_lch_out.dma_conf.src_addr_width =
    DMA_SLAVE_BUSWIDTH_4_BYTES;
    dd.dma_lch_out.dma_conf.dst_maxburst = 1;
    dd.dma_lch_out.dma_conf.dst_addr_width =
    DMA_SLAVE_BUSWIDTH_4_BYTES;
    dd.dma_lch_out.dma_conf.device_fc = false;
    return 0;
    err_dma_out:
    dma_release_channel(dd.dma_lch_in.chan);
    err_dma_in:
    dev_err(dd.dev, "no DMA channel available\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_dma_cleanup(dd: *mut atmel_tdes_dev) {
    static void atmel_tdes_dma_cleanup(struct atmel_tdes_dev *dd)
    {
    dma_release_channel(dd.dma_lch_in.chan);
    dma_release_channel(dd.dma_lch_out.chan);
    }
    static int atmel_des_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int keylen)
    {
    struct atmel_tdes_ctx *ctx = crypto_skcipher_ctx(tfm);
    int err;
    err = verify_skcipher_des_key(tfm, key);
    if (err)
    return err;
    memcpy(ctx.key, key, keylen);
    ctx.keylen = keylen;
    return 0;
    }
    static int atmel_tdes_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int keylen)
    {
    struct atmel_tdes_ctx *ctx = crypto_skcipher_ctx(tfm);
    int err;
    err = verify_skcipher_des3_key(tfm, key);
    if (err)
    return err;
    memcpy(ctx.key, key, keylen);
    ctx.keylen = keylen;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_ecb_encrypt(req: *mut skcipher_request) -> c_int {
    static int atmel_tdes_ecb_encrypt(struct skcipher_request *req)
    {
    return atmel_tdes_crypt(req, TDES_FLAGS_ECB | TDES_FLAGS_ENCRYPT);
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_ecb_decrypt(req: *mut skcipher_request) -> c_int {
    static int atmel_tdes_ecb_decrypt(struct skcipher_request *req)
    {
    return atmel_tdes_crypt(req, TDES_FLAGS_ECB);
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_cbc_encrypt(req: *mut skcipher_request) -> c_int {
    static int atmel_tdes_cbc_encrypt(struct skcipher_request *req)
    {
    return atmel_tdes_crypt(req, TDES_FLAGS_CBC | TDES_FLAGS_ENCRYPT);
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_cbc_decrypt(req: *mut skcipher_request) -> c_int {
    static int atmel_tdes_cbc_decrypt(struct skcipher_request *req)
    {
    return atmel_tdes_crypt(req, TDES_FLAGS_CBC);
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_init_tfm(tfm: *mut crypto_skcipher) -> c_int {
    static int atmel_tdes_init_tfm(struct crypto_skcipher *tfm)
    {
    struct atmel_tdes_ctx *ctx = crypto_skcipher_ctx(tfm);
    ctx.dd = atmel_tdes_dev_alloc();
    if (!ctx.dd)
    return -ENODEV;
    crypto_skcipher_set_reqsize(tfm, sizeof(struct atmel_tdes_reqctx));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_skcipher_alg_init(alg: *mut skcipher_alg) {
    static void atmel_tdes_skcipher_alg_init(struct skcipher_alg *alg)
    {
    alg.base.cra_priority = ATMEL_TDES_PRIORITY;
    alg.base.cra_flags = CRYPTO_ALG_ASYNC | CRYPTO_ALG_KERN_DRIVER_ONLY;
    alg.base.cra_ctxsize = sizeof(struct atmel_tdes_ctx);
    alg.base.cra_module = THIS_MODULE;
    alg.init = atmel_tdes_init_tfm;
    }
    static struct skcipher_alg tdes_algs[] = {
    {
    .base.cra_name		= "ecb(des)",
    .base.cra_driver_name	= "atmel-ecb-des",
    .base.cra_blocksize	= DES_BLOCK_SIZE,
    .base.cra_alignmask	= 0x7,
    .min_keysize		= DES_KEY_SIZE,
    .max_keysize		= DES_KEY_SIZE,
    .setkey			= atmel_des_setkey,
    .encrypt		= atmel_tdes_ecb_encrypt,
    .decrypt		= atmel_tdes_ecb_decrypt,
    },
    {
    .base.cra_name		= "cbc(des)",
    .base.cra_driver_name	= "atmel-cbc-des",
    .base.cra_blocksize	= DES_BLOCK_SIZE,
    .base.cra_alignmask	= 0x7,
    .min_keysize		= DES_KEY_SIZE,
    .max_keysize		= DES_KEY_SIZE,
    .ivsize			= DES_BLOCK_SIZE,
    .setkey			= atmel_des_setkey,
    .encrypt		= atmel_tdes_cbc_encrypt,
    .decrypt		= atmel_tdes_cbc_decrypt,
    },
    {
    .base.cra_name		= "ecb(des3_ede)",
    .base.cra_driver_name	= "atmel-ecb-tdes",
    .base.cra_blocksize	= DES_BLOCK_SIZE,
    .base.cra_alignmask	= 0x7,
    .min_keysize		= DES3_EDE_KEY_SIZE,
    .max_keysize		= DES3_EDE_KEY_SIZE,
    .setkey			= atmel_tdes_setkey,
    .encrypt		= atmel_tdes_ecb_encrypt,
    .decrypt		= atmel_tdes_ecb_decrypt,
    },
    {
    .base.cra_name		= "cbc(des3_ede)",
    .base.cra_driver_name	= "atmel-cbc-tdes",
    .base.cra_blocksize	= DES_BLOCK_SIZE,
    .base.cra_alignmask	= 0x7,
    .min_keysize		= DES3_EDE_KEY_SIZE,
    .max_keysize		= DES3_EDE_KEY_SIZE,
    .setkey			= atmel_tdes_setkey,
    .encrypt		= atmel_tdes_cbc_encrypt,
    .decrypt		= atmel_tdes_cbc_decrypt,
    .ivsize			= DES_BLOCK_SIZE,
    },
    };
#[no_mangle]
unsafe extern "C" fn atmel_tdes_queue_task(data: c_ulong) {
    static void atmel_tdes_queue_task(unsigned long data)
    {
    struct atmel_tdes_dev *dd = (struct atmel_tdes_dev *)data;
    atmel_tdes_handle_queue(dd, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_done_task(data: c_ulong) {
    static void atmel_tdes_done_task(unsigned long data)
    {
    struct atmel_tdes_dev *dd = (struct atmel_tdes_dev *) data;
    int err;
    if (!(dd.flags & TDES_FLAGS_DMA))
    err = atmel_tdes_crypt_pdc_stop(dd);
    else
    err = atmel_tdes_crypt_dma_stop(dd);
    if (dd.total && !err) {
    if (dd.flags & TDES_FLAGS_FAST) {
    dd.in_sg = sg_next(dd.in_sg);
    dd.out_sg = sg_next(dd.out_sg);
    if (!dd.in_sg || !dd.out_sg)
    err = -EINVAL;
    }
    if (!err)
    err = atmel_tdes_crypt_start(dd);
    if (!err)
    return; /* DMA started. Not finishing. */
    }
    atmel_tdes_finish_req(dd, err);
    atmel_tdes_handle_queue(dd, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t atmel_tdes_irq(int irq, void *dev_id)
    {
    struct atmel_tdes_dev *tdes_dd = dev_id;
    u32 reg;
    reg = atmel_tdes_read(tdes_dd, TDES_ISR);
    if (reg & atmel_tdes_read(tdes_dd, TDES_IMR)) {
    atmel_tdes_write(tdes_dd, TDES_IDR, reg);
    if (TDES_FLAGS_BUSY & tdes_dd.flags)
    tasklet_schedule(&tdes_dd.done_task);
    else
    dev_warn(tdes_dd.dev, "TDES interrupt when no active requests.\n");
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_register_algs(dd: *mut atmel_tdes_dev) -> c_int {
    static int atmel_tdes_register_algs(struct atmel_tdes_dev *dd)
    {
    int err, i;
    for (i = 0; i < ARRAY_SIZE(tdes_algs); i++) {
    atmel_tdes_skcipher_alg_init(&tdes_algs[i]);
    err = crypto_register_skcipher(&tdes_algs[i]);
    if (err) {
    crypto_unregister_skciphers(tdes_algs, i);
    return err;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_get_cap(dd: *mut atmel_tdes_dev) {
    static void atmel_tdes_get_cap(struct atmel_tdes_dev *dd)
    {
    dd.caps.has_dma = 0;
// keep only major version number
    switch (dd.hw_version & 0xf00) {
    case 0x800:
    case 0x700:
    dd.caps.has_dma = 1;
    break;
    case 0x600:
    break;
    default:
    dev_warn(dd.dev,
    "Unmanaged tdes version, set minimum capabilities\n");
    break;
    }
    }
    static const struct of_device_id atmel_tdes_dt_ids[] = {
    { .compatible = "atmel,at91sam9g46-tdes" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, atmel_tdes_dt_ids);
#[no_mangle]
unsafe extern "C" fn atmel_tdes_probe(pdev: *mut platform_device) -> c_int {
    static int atmel_tdes_probe(struct platform_device *pdev)
    {
    struct atmel_tdes_dev *tdes_dd;
    struct device *dev = &pdev.dev;
    struct resource *tdes_res;
    int err;
    tdes_dd = devm_kmalloc(&pdev.dev, sizeof(*tdes_dd), GFP_KERNEL);
    if (!tdes_dd)
    return -ENOMEM;
    tdes_dd.dev = dev;
    platform_set_drvdata(pdev, tdes_dd);
    INIT_LIST_HEAD(&tdes_dd.list);
    spin_lock_init(&tdes_dd.lock);
    tasklet_init(&tdes_dd.done_task, atmel_tdes_done_task,
    (unsigned long)tdes_dd);
    tasklet_init(&tdes_dd.queue_task, atmel_tdes_queue_task,
    (unsigned long)tdes_dd);
    crypto_init_queue(&tdes_dd.queue, ATMEL_TDES_QUEUE_LENGTH);
    tdes_dd.io_base = devm_platform_get_and_ioremap_resource(pdev, 0, &tdes_res);
    if (IS_ERR(tdes_dd.io_base)) {
    err = PTR_ERR(tdes_dd.io_base);
    goto err_tasklet_kill;
    }
    tdes_dd.phys_base = tdes_res.start;
// Get the IRQ
    tdes_dd.irq = platform_get_irq(pdev,  0);
    if (tdes_dd.irq < 0) {
    err = tdes_dd.irq;
    goto err_tasklet_kill;
    }
    err = devm_request_irq(&pdev.dev, tdes_dd.irq, atmel_tdes_irq,
    IRQF_SHARED, "atmel-tdes", tdes_dd);
    if (err)
    goto err_tasklet_kill;
// Initializing the clock
    tdes_dd.iclk = devm_clk_get(&pdev.dev, "tdes_clk");
    if (IS_ERR(tdes_dd.iclk)) {
    dev_err(dev, "clock initialization failed.\n");
    err = PTR_ERR(tdes_dd.iclk);
    goto err_tasklet_kill;
    }
    err = atmel_tdes_hw_version_init(tdes_dd);
    if (err)
    goto err_tasklet_kill;
    atmel_tdes_get_cap(tdes_dd);
    err = atmel_tdes_buff_init(tdes_dd);
    if (err)
    goto err_tasklet_kill;
    if (tdes_dd.caps.has_dma) {
    err = atmel_tdes_dma_init(tdes_dd);
    if (err)
    goto err_buff_cleanup;
    dev_info(dev, "using %s, %s for DMA transfers\n",
    dma_chan_name(tdes_dd.dma_lch_in.chan),
    dma_chan_name(tdes_dd.dma_lch_out.chan));
    }
    spin_lock(&atmel_tdes.lock);
    list_add_tail(&tdes_dd.list, &atmel_tdes.dev_list);
    spin_unlock(&atmel_tdes.lock);
    err = atmel_tdes_register_algs(tdes_dd);
    if (err)
    goto err_algs;
    dev_info(dev, "Atmel DES/TDES\n");
    return 0;
    err_algs:
    spin_lock(&atmel_tdes.lock);
    list_del(&tdes_dd.list);
    spin_unlock(&atmel_tdes.lock);
    if (tdes_dd.caps.has_dma)
    atmel_tdes_dma_cleanup(tdes_dd);
    err_buff_cleanup:
    atmel_tdes_buff_cleanup(tdes_dd);
    err_tasklet_kill:
    tasklet_kill(&tdes_dd.done_task);
    tasklet_kill(&tdes_dd.queue_task);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn atmel_tdes_remove(pdev: *mut platform_device) {
    static void atmel_tdes_remove(struct platform_device *pdev)
    {
    struct atmel_tdes_dev *tdes_dd = platform_get_drvdata(pdev);
    spin_lock(&atmel_tdes.lock);
    list_del(&tdes_dd.list);
    spin_unlock(&atmel_tdes.lock);
    crypto_unregister_skciphers(tdes_algs, ARRAY_SIZE(tdes_algs));
    tasklet_kill(&tdes_dd.done_task);
    tasklet_kill(&tdes_dd.queue_task);
    if (tdes_dd.caps.has_dma)
    atmel_tdes_dma_cleanup(tdes_dd);
    atmel_tdes_buff_cleanup(tdes_dd);
    }
    static struct platform_driver atmel_tdes_driver = {
    .probe		= atmel_tdes_probe,
    .remove		= atmel_tdes_remove,
    .driver		= {
    .name	= "atmel_tdes",
    .of_match_table = atmel_tdes_dt_ids,
    },
    };
    module_platform_driver(atmel_tdes_driver);
    MODULE_DESCRIPTION("Atmel DES/TDES hw acceleration support.");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Nicolas Royer - Eukréa Electromatique");
