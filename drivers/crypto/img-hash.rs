//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/img-hash.c
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
// Copyright (c) 2014 Imagination Technologies
// Authors:  Will Thomas, James Hartley
//
// Interface structure taken from omap-sham driver
//

pub const CR_RESET: c_int = 0;
pub const CR_RESET_SET: c_int = 1;
pub const CR_RESET_UNSET: c_int = 0;
pub const CR_MESSAGE_LENGTH_H: c_uint = 0x4;
pub const CR_MESSAGE_LENGTH_L: c_uint = 0x8;
pub const CR_CONTROL: c_uint = 0xc;
pub const CR_CONTROL_BYTE_ORDER_3210: c_int = 0;
pub const CR_CONTROL_BYTE_ORDER_0123: c_int = 1;
pub const CR_CONTROL_BYTE_ORDER_2310: c_int = 2;
pub const CR_CONTROL_BYTE_ORDER_1032: c_int = 3;
pub const CR_CONTROL_BYTE_ORDER_SHIFT: c_int = 8;
pub const CR_CONTROL_ALGO_MD5: c_int = 0;
pub const CR_CONTROL_ALGO_SHA1: c_int = 1;
pub const CR_CONTROL_ALGO_SHA224: c_int = 2;
pub const CR_CONTROL_ALGO_SHA256: c_int = 3;
pub const CR_INTSTAT: c_uint = 0x10;
pub const CR_INTENAB: c_uint = 0x14;
pub const CR_INTCLEAR: c_uint = 0x18;

pub const CR_RESULT_QUEUE: c_uint = 0x1c;
pub const CR_RSD0: c_uint = 0x40;
pub const CR_CORE_REV: c_uint = 0x50;
pub const CR_CORE_DES1: c_uint = 0x60;
pub const CR_CORE_DES2: c_uint = 0x70;

pub const IMG_HASH_QUEUE_LENGTH: c_int = 20;
pub const IMG_HASH_DMA_BURST: c_int = 4;
pub const IMG_HASH_DMA_THRESHOLD: c_int = 64;

    struct img_hash_dev;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_hash_request_ctx {
    pub hdev: *mut img_hash_dev,
    pub __aligned(sizeof(u32)): u8 digest[SHA256_DIGEST_SIZE],
    pub flags: c_ulong,
    pub digsize: usize,
    pub dma_addr: dma_addr_t,
    pub dma_ct: usize,
// sg root
    pub sgfirst: *mut scatterlist,
// walk state
    pub sg: *mut scatterlist,
    pub nents: usize,
    pub offset: usize,
    pub total: c_uint,
    pub sent: usize,
    pub op: c_ulong,
    pub bufcnt: usize,
    pub fallback_req: ahash_request,
// Zero length buffer must remain last member of struct
    pub __aligned(sizeof(u32)): u8 buffer[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_hash_ctx {
    pub hdev: *mut img_hash_dev,
    pub flags: c_ulong,
    pub fallback: *mut crypto_ahash,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_hash_dev {
    pub list: list_head,
    pub dev: *mut device,
    pub hash_clk: *mut clk,
    pub sys_clk: *mut clk,
    pub io_base: *mut void __iomem,
    pub bus_addr: phys_addr_t,
    pub cpu_addr: *mut void __iomem,
    pub lock: spinlock_t,
    pub err: c_int,
    pub done_task: tasklet_struct,
    pub dma_task: tasklet_struct,
    pub flags: c_ulong,
    pub queue: crypto_queue,
    pub req: *mut ahash_request,
    pub dma_lch: *mut dma_chan,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_hash_drv {
    pub dev_list: list_head,
    pub lock: spinlock_t,
}

    static struct img_hash_drv img_hash = {
    .dev_list = LIST_HEAD_INIT(img_hash.dev_list),
    .lock = __SPIN_LOCK_UNLOCKED(img_hash.lock),
    };
#[no_mangle]
pub unsafe extern "C" fn img_hash_read(hdev: *mut img_hash_dev, offset: u32) -> u32 {
    static inline u32 img_hash_read(struct img_hash_dev *hdev, u32 offset)
    {
    return readl_relaxed(hdev.io_base + offset);
    }
    static inline void img_hash_write(struct img_hash_dev *hdev,
    u32 offset, u32 value)
    {
    writel_relaxed(value, hdev.io_base + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn img_hash_read_result_queue(hdev: *mut img_hash_dev) -> __be32 {
    static inline __be32 img_hash_read_result_queue(struct img_hash_dev *hdev)
    {
    return cpu_to_be32(img_hash_read(hdev, CR_RESULT_QUEUE));
    }
#[no_mangle]
unsafe extern "C" fn img_hash_start(hdev: *mut img_hash_dev, dma: bool) {
    static void img_hash_start(struct img_hash_dev *hdev, bool dma)
    {
    struct img_hash_request_ctx *ctx = ahash_request_ctx(hdev.req);
    let mut cr: u32 = IMG_HASH_BYTE_ORDER << CR_CONTROL_BYTE_ORDER_SHIFT;
    if (ctx.flags & DRIVER_FLAGS_MD5)
    cr |= CR_CONTROL_ALGO_MD5;
#[no_mangle]
pub unsafe extern "C" fn if(DRIVER_FLAGS_SHA1: ctx->flags &) -> else {
    else if (ctx.flags & DRIVER_FLAGS_SHA1)
    cr |= CR_CONTROL_ALGO_SHA1;
#[no_mangle]
pub unsafe extern "C" fn if(DRIVER_FLAGS_SHA224: ctx->flags &) -> else {
    else if (ctx.flags & DRIVER_FLAGS_SHA224)
    cr |= CR_CONTROL_ALGO_SHA224;
#[no_mangle]
pub unsafe extern "C" fn if(DRIVER_FLAGS_SHA256: ctx->flags &) -> else {
    else if (ctx.flags & DRIVER_FLAGS_SHA256)
    cr |= CR_CONTROL_ALGO_SHA256;
    dev_dbg(hdev.dev, "Starting hash process\n");
    img_hash_write(hdev, CR_CONTROL, cr);
//
// The hardware block requires two cycles between writing the control
// register and writing the first word of data in non DMA mode, to
// ensure the first data write is not grouped in burst with the control
// register write a read is issued to 'flush' the bus.
//
    if (!dma)
    img_hash_read(hdev, CR_CONTROL);
    }
    static int img_hash_xmit_cpu(struct img_hash_dev *hdev, const u8 *buf,
    size_t length, int final)
    {
    u32 count, len32;
    const u32 *buffer = (const u32 *)buf;
    dev_dbg(hdev.dev, "xmit_cpu:  length: %zu bytes\n", length);
    if (final)
    hdev.flags |= DRIVER_FLAGS_FINAL;
    len32 = DIV_ROUND_UP(length, sizeof(u32));
    for (count = 0; count < len32; count++)
    writel_relaxed(buffer[count], hdev.cpu_addr);
    return -EINPROGRESS;
    }
#[no_mangle]
unsafe extern "C" fn img_hash_dma_callback(data: *mut c_void) {
    static void img_hash_dma_callback(void *data)
    {
    struct img_hash_dev *hdev = data;
    struct img_hash_request_ctx *ctx = ahash_request_ctx(hdev.req);
    if (ctx.bufcnt) {
    img_hash_xmit_cpu(hdev, ctx.buffer, ctx.bufcnt, 0);
    ctx.bufcnt = 0;
    }
    if (ctx.sg)
    tasklet_schedule(&hdev.dma_task);
    }
#[no_mangle]
unsafe extern "C" fn img_hash_xmit_dma(hdev: *mut img_hash_dev, sg: *mut scatterlist) -> c_int {
    static int img_hash_xmit_dma(struct img_hash_dev *hdev, struct scatterlist *sg)
    {
    struct dma_async_tx_descriptor *desc;
    struct img_hash_request_ctx *ctx = ahash_request_ctx(hdev.req);
    ctx.dma_ct = dma_map_sg(hdev.dev, sg, 1, DMA_TO_DEVICE);
    if (ctx.dma_ct == 0) {
    dev_err(hdev.dev, "Invalid DMA sg\n");
    hdev.err = -EINVAL;
    return -EINVAL;
    }
    desc = dmaengine_prep_slave_sg(hdev.dma_lch,
    sg,
    ctx.dma_ct,
    DMA_MEM_TO_DEV,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!desc) {
    dev_err(hdev.dev, "Null DMA descriptor\n");
    hdev.err = -EINVAL;
    dma_unmap_sg(hdev.dev, sg, 1, DMA_TO_DEVICE);
    return -EINVAL;
    }
    desc.callback = img_hash_dma_callback;
    desc.callback_param = hdev;
    dmaengine_submit(desc);
    dma_async_issue_pending(hdev.dma_lch);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_hash_write_via_cpu(hdev: *mut img_hash_dev) -> c_int {
    static int img_hash_write_via_cpu(struct img_hash_dev *hdev)
    {
    struct img_hash_request_ctx *ctx = ahash_request_ctx(hdev.req);
    ctx.bufcnt = sg_copy_to_buffer(hdev.req.src, sg_nents(ctx.sg),
    ctx.buffer, hdev.req.nbytes);
    ctx.total = hdev.req.nbytes;
    ctx.bufcnt = 0;
    hdev.flags |= (DRIVER_FLAGS_CPU | DRIVER_FLAGS_FINAL);
    img_hash_start(hdev, false);
    return img_hash_xmit_cpu(hdev, ctx.buffer, ctx.total, 1);
    }
#[no_mangle]
unsafe extern "C" fn img_hash_finish(req: *mut ahash_request) -> c_int {
    static int img_hash_finish(struct ahash_request *req)
    {
    struct img_hash_request_ctx *ctx = ahash_request_ctx(req);
    if (!req.result)
    return -EINVAL;
    memcpy(req.result, ctx.digest, ctx.digsize);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_hash_copy_hash(req: *mut ahash_request) {
    static void img_hash_copy_hash(struct ahash_request *req)
    {
    struct img_hash_request_ctx *ctx = ahash_request_ctx(req);
    __be32 *hash = (__be32 *)ctx.digest;
    int i;
    for (i = (ctx.digsize / sizeof(*hash)) - 1; i >= 0; i--)
    hash[i] = img_hash_read_result_queue(ctx.hdev);
    }
#[no_mangle]
unsafe extern "C" fn img_hash_finish_req(req: *mut ahash_request, err: c_int) {
    static void img_hash_finish_req(struct ahash_request *req, int err)
    {
    struct img_hash_request_ctx *ctx = ahash_request_ctx(req);
    struct img_hash_dev *hdev =  ctx.hdev;
    if (!err) {
    img_hash_copy_hash(req);
    if (DRIVER_FLAGS_FINAL & hdev.flags)
    err = img_hash_finish(req);
    } else {
    dev_warn(hdev.dev, "Hash failed with error %d\n", err);
    ctx.flags |= DRIVER_FLAGS_ERROR;
    }
    hdev.flags &= ~(DRIVER_FLAGS_DMA_READY | DRIVER_FLAGS_OUTPUT_READY |
    DRIVER_FLAGS_CPU | DRIVER_FLAGS_BUSY | DRIVER_FLAGS_FINAL);
    if (req.base.complete)
    ahash_request_complete(req, err);
    }
#[no_mangle]
unsafe extern "C" fn img_hash_write_via_dma(hdev: *mut img_hash_dev) -> c_int {
    static int img_hash_write_via_dma(struct img_hash_dev *hdev)
    {
    struct img_hash_request_ctx *ctx = ahash_request_ctx(hdev.req);
    img_hash_start(hdev, true);
    dev_dbg(hdev.dev, "xmit dma size: %d\n", ctx.total);
    if (!ctx.total)
    hdev.flags |= DRIVER_FLAGS_FINAL;
    hdev.flags |= DRIVER_FLAGS_DMA_ACTIVE | DRIVER_FLAGS_FINAL;
    tasklet_schedule(&hdev.dma_task);
    return -EINPROGRESS;
    }
#[no_mangle]
unsafe extern "C" fn img_hash_dma_init(hdev: *mut img_hash_dev) -> c_int {
    static int img_hash_dma_init(struct img_hash_dev *hdev)
    {
    struct dma_slave_config dma_conf;
    int err;
    hdev.dma_lch = dma_request_chan(hdev.dev, "tx");
    if (IS_ERR(hdev.dma_lch)) {
    dev_err(hdev.dev, "Couldn't acquire a slave DMA channel.\n");
    return PTR_ERR(hdev.dma_lch);
    }
    dma_conf.direction = DMA_MEM_TO_DEV;
    dma_conf.dst_addr = hdev.bus_addr;
    dma_conf.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    dma_conf.dst_maxburst = IMG_HASH_DMA_BURST;
    dma_conf.device_fc = false;
    err = dmaengine_slave_config(hdev.dma_lch,  &dma_conf);
    if (err) {
    dev_err(hdev.dev, "Couldn't configure DMA slave.\n");
    dma_release_channel(hdev.dma_lch);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_hash_dma_task(d: c_ulong) {
    static void img_hash_dma_task(unsigned long d)
    {
    struct img_hash_dev *hdev = (struct img_hash_dev *)d;
    struct img_hash_request_ctx *ctx;
    u8 *addr;
    size_t nbytes, bleft, wsend, len, tbc;
    struct scatterlist tsg;
    if (!hdev.req)
    return;
    ctx = ahash_request_ctx(hdev.req);
    if (!ctx.sg)
    return;
    addr = sg_virt(ctx.sg);
    nbytes = ctx.sg.length - ctx.offset;
//
// The hash accelerator does not support a data valid mask. This means
// that if each dma (i.e. per page) is not a multiple of 4 bytes, the
// padding bytes in the last word written by that dma would erroneously
// be included in the hash. To avoid this we round down the transfer,
// and add the excess to the start of the next dma. It does not matter
// that the final dma may not be a multiple of 4 bytes as the hashing
// block is programmed to accept the correct number of bytes.
//
    bleft = nbytes % 4;
    wsend = (nbytes / 4);
    if (wsend) {
    sg_init_one(&tsg, addr + ctx.offset, wsend * 4);
    if (img_hash_xmit_dma(hdev, &tsg)) {
    dev_err(hdev.dev, "DMA failed, falling back to CPU");
    ctx.flags |= DRIVER_FLAGS_CPU;
    hdev.err = 0;
    img_hash_xmit_cpu(hdev, addr + ctx.offset,
    wsend * 4, 0);
    ctx.sent += wsend * 4;
    wsend = 0;
    } else {
    ctx.sent += wsend * 4;
    }
    }
    if (bleft) {
    ctx.bufcnt = sg_pcopy_to_buffer(ctx.sgfirst, ctx.nents,
    ctx.buffer, bleft, ctx.sent);
    tbc = 0;
    ctx.sg = sg_next(ctx.sg);
    while (ctx.sg && (ctx.bufcnt < 4)) {
    len = ctx.sg.length;
    if (likely(len > (4 - ctx.bufcnt)))
    len = 4 - ctx.bufcnt;
    tbc = sg_pcopy_to_buffer(ctx.sgfirst, ctx.nents,
    ctx.buffer + ctx.bufcnt, len,
    ctx.sent + ctx.bufcnt);
    ctx.bufcnt += tbc;
    if (tbc >= ctx.sg.length) {
    ctx.sg = sg_next(ctx.sg);
    tbc = 0;
    }
    }
    ctx.sent += ctx.bufcnt;
    ctx.offset = tbc;
    if (!wsend)
    img_hash_dma_callback(hdev);
    } else {
    ctx.offset = 0;
    ctx.sg = sg_next(ctx.sg);
    }
    }
#[no_mangle]
unsafe extern "C" fn img_hash_write_via_dma_stop(hdev: *mut img_hash_dev) -> c_int {
    static int img_hash_write_via_dma_stop(struct img_hash_dev *hdev)
    {
    struct img_hash_request_ctx *ctx = ahash_request_ctx(hdev.req);
    if (ctx.flags & DRIVER_FLAGS_SG)
    dma_unmap_sg(hdev.dev, ctx.sg, 1, DMA_TO_DEVICE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_hash_process_data(hdev: *mut img_hash_dev) -> c_int {
    static int img_hash_process_data(struct img_hash_dev *hdev)
    {
    struct ahash_request *req = hdev.req;
    struct img_hash_request_ctx *ctx = ahash_request_ctx(req);
    let mut err: c_int = 0;
    ctx.bufcnt = 0;
    if (req.nbytes >= IMG_HASH_DMA_THRESHOLD) {
    dev_dbg(hdev.dev, "process data request(%d bytes) using DMA\n",
    req.nbytes);
    err = img_hash_write_via_dma(hdev);
    } else {
    dev_dbg(hdev.dev, "process data request(%d bytes) using CPU\n",
    req.nbytes);
    err = img_hash_write_via_cpu(hdev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn img_hash_hw_init(hdev: *mut img_hash_dev) -> c_int {
    static int img_hash_hw_init(struct img_hash_dev *hdev)
    {
    unsigned long long nbits;
    u32 u, l;
    img_hash_write(hdev, CR_RESET, CR_RESET_SET);
    img_hash_write(hdev, CR_RESET, CR_RESET_UNSET);
    img_hash_write(hdev, CR_INTENAB, CR_INT_NEW_RESULTS_SET);
    nbits = (u64)hdev.req.nbytes << 3;
    u = nbits >> 32;
    l = nbits;
    img_hash_write(hdev, CR_MESSAGE_LENGTH_H, u);
    img_hash_write(hdev, CR_MESSAGE_LENGTH_L, l);
    if (!(DRIVER_FLAGS_INIT & hdev.flags)) {
    hdev.flags |= DRIVER_FLAGS_INIT;
    hdev.err = 0;
    }
    dev_dbg(hdev.dev, "hw initialized, nbits: %llx\n", nbits);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_hash_init(req: *mut ahash_request) -> c_int {
    static int img_hash_init(struct ahash_request *req)
    {
    struct crypto_ahash *tfm = crypto_ahash_reqtfm(req);
    struct img_hash_request_ctx *rctx = ahash_request_ctx(req);
    struct img_hash_ctx *ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&rctx.fallback_req, ctx.fallback);
    ahash_request_set_callback(&rctx.fallback_req,
    req.base.flags & CRYPTO_TFM_REQ_MAY_SLEEP,
    req.base.complete, req.base.data);
    return crypto_ahash_init(&rctx.fallback_req);
    }
    static int img_hash_handle_queue(struct img_hash_dev *hdev,
    struct ahash_request *req)
    {
    struct crypto_async_request *async_req, *backlog;
    struct img_hash_request_ctx *ctx;
    unsigned long flags;
    let mut err: c_int = 0, res = 0;
    spin_lock_irqsave(&hdev.lock, flags);
    if (req)
    res = ahash_enqueue_request(&hdev.queue, req);
    if (DRIVER_FLAGS_BUSY & hdev.flags) {
    spin_unlock_irqrestore(&hdev.lock, flags);
    return res;
    }
    backlog = crypto_get_backlog(&hdev.queue);
    async_req = crypto_dequeue_request(&hdev.queue);
    if (async_req)
    hdev.flags |= DRIVER_FLAGS_BUSY;
    spin_unlock_irqrestore(&hdev.lock, flags);
    if (!async_req)
    return res;
    if (backlog)
    crypto_request_complete(backlog, -EINPROGRESS);
    req = ahash_request_cast(async_req);
    hdev.req = req;
    ctx = ahash_request_ctx(req);
    dev_info(hdev.dev, "processing req, op: %lu, bytes: %d\n",
    ctx.op, req.nbytes);
    err = img_hash_hw_init(hdev);
    if (!err)
    err = img_hash_process_data(hdev);
    if (err != -EINPROGRESS) {
// done_task will not finish so do it here
    img_hash_finish_req(req, err);
    }
    return res;
    }
#[no_mangle]
unsafe extern "C" fn img_hash_update(req: *mut ahash_request) -> c_int {
    static int img_hash_update(struct ahash_request *req)
    {
    struct img_hash_request_ctx *rctx = ahash_request_ctx(req);
    struct crypto_ahash *tfm = crypto_ahash_reqtfm(req);
    struct img_hash_ctx *ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&rctx.fallback_req, ctx.fallback);
    ahash_request_set_callback(&rctx.fallback_req,
    req.base.flags & CRYPTO_TFM_REQ_MAY_SLEEP,
    req.base.complete, req.base.data);
    ahash_request_set_crypt(&rctx.fallback_req, req.src, core::ptr::null_mut(), req.nbytes);
    return crypto_ahash_update(&rctx.fallback_req);
    }
#[no_mangle]
unsafe extern "C" fn img_hash_final(req: *mut ahash_request) -> c_int {
    static int img_hash_final(struct ahash_request *req)
    {
    struct img_hash_request_ctx *rctx = ahash_request_ctx(req);
    struct crypto_ahash *tfm = crypto_ahash_reqtfm(req);
    struct img_hash_ctx *ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&rctx.fallback_req, ctx.fallback);
    ahash_request_set_callback(&rctx.fallback_req,
    req.base.flags & CRYPTO_TFM_REQ_MAY_SLEEP,
    req.base.complete, req.base.data);
    ahash_request_set_crypt(&rctx.fallback_req, core::ptr::null_mut(), req.result, 0);
    return crypto_ahash_final(&rctx.fallback_req);
    }
#[no_mangle]
unsafe extern "C" fn img_hash_finup(req: *mut ahash_request) -> c_int {
    static int img_hash_finup(struct ahash_request *req)
    {
    struct img_hash_request_ctx *rctx = ahash_request_ctx(req);
    struct crypto_ahash *tfm = crypto_ahash_reqtfm(req);
    struct img_hash_ctx *ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&rctx.fallback_req, ctx.fallback);
    ahash_request_set_callback(&rctx.fallback_req,
    req.base.flags & CRYPTO_TFM_REQ_MAY_SLEEP,
    req.base.complete, req.base.data);
    ahash_request_set_crypt(&rctx.fallback_req, req.src, req.result,
    req.nbytes);
    return crypto_ahash_finup(&rctx.fallback_req);
    }
#[no_mangle]
unsafe extern "C" fn img_hash_import(req: *mut ahash_request, in: *const c_void) -> c_int {
    static int img_hash_import(struct ahash_request *req, const void *in)
    {
    struct img_hash_request_ctx *rctx = ahash_request_ctx(req);
    struct crypto_ahash *tfm = crypto_ahash_reqtfm(req);
    struct img_hash_ctx *ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&rctx.fallback_req, ctx.fallback);
    ahash_request_set_callback(&rctx.fallback_req,
    req.base.flags & CRYPTO_TFM_REQ_MAY_SLEEP,
    req.base.complete, req.base.data);
    return crypto_ahash_import(&rctx.fallback_req, in);
    }
#[no_mangle]
unsafe extern "C" fn img_hash_export(req: *mut ahash_request, out: *mut c_void) -> c_int {
    static int img_hash_export(struct ahash_request *req, void *out)
    {
    struct img_hash_request_ctx *rctx = ahash_request_ctx(req);
    struct crypto_ahash *tfm = crypto_ahash_reqtfm(req);
    struct img_hash_ctx *ctx = crypto_ahash_ctx(tfm);
    ahash_request_set_tfm(&rctx.fallback_req, ctx.fallback);
    ahash_request_set_callback(&rctx.fallback_req,
    req.base.flags & CRYPTO_TFM_REQ_MAY_SLEEP,
    req.base.complete, req.base.data);
    return crypto_ahash_export(&rctx.fallback_req, out);
    }
#[no_mangle]
unsafe extern "C" fn img_hash_digest(req: *mut ahash_request) -> c_int {
    static int img_hash_digest(struct ahash_request *req)
    {
    struct crypto_ahash *tfm = crypto_ahash_reqtfm(req);
    struct img_hash_ctx *tctx = crypto_ahash_ctx(tfm);
    struct img_hash_request_ctx *ctx = ahash_request_ctx(req);
    spin_lock(&img_hash.lock);
    if (!tctx.hdev)
    tctx.hdev = list_first_entry_or_null(&img_hash.dev_list,
    struct img_hash_dev, list);
    ctx.hdev = tctx.hdev;
    spin_unlock(&img_hash.lock);
    ctx.flags = 0;
    ctx.digsize = crypto_ahash_digestsize(tfm);
    switch (ctx.digsize) {
    case SHA1_DIGEST_SIZE:
    ctx.flags |= DRIVER_FLAGS_SHA1;
    break;
    case SHA256_DIGEST_SIZE:
    ctx.flags |= DRIVER_FLAGS_SHA256;
    break;
    case SHA224_DIGEST_SIZE:
    ctx.flags |= DRIVER_FLAGS_SHA224;
    break;
    case MD5_DIGEST_SIZE:
    ctx.flags |= DRIVER_FLAGS_MD5;
    break;
    default:
    return -EINVAL;
    }
    ctx.bufcnt = 0;
    ctx.offset = 0;
    ctx.sent = 0;
    ctx.total = req.nbytes;
    ctx.sg = req.src;
    ctx.sgfirst = req.src;
    ctx.nents = sg_nents(ctx.sg);
    return img_hash_handle_queue(ctx.hdev, req);
    }
#[no_mangle]
unsafe extern "C" fn img_hash_cra_init(tfm: *mut crypto_tfm, alg_name: *const c_char) -> c_int {
    static int img_hash_cra_init(struct crypto_tfm *tfm, const char *alg_name)
    {
    struct img_hash_ctx *ctx = crypto_tfm_ctx(tfm);
    ctx.fallback = crypto_alloc_ahash(alg_name, 0,
    CRYPTO_ALG_NEED_FALLBACK);
    if (IS_ERR(ctx.fallback)) {
    pr_err("img_hash: Could not load fallback driver.\n");
    return PTR_ERR(ctx.fallback);
    }
    crypto_ahash_set_reqsize(__crypto_ahash_cast(tfm),
    sizeof(struct img_hash_request_ctx) +
    crypto_ahash_reqsize(ctx.fallback) +
    IMG_HASH_DMA_THRESHOLD);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_hash_cra_md5_init(tfm: *mut crypto_tfm) -> c_int {
    static int img_hash_cra_md5_init(struct crypto_tfm *tfm)
    {
    return img_hash_cra_init(tfm, "md5-lib");
    }
#[no_mangle]
unsafe extern "C" fn img_hash_cra_sha1_init(tfm: *mut crypto_tfm) -> c_int {
    static int img_hash_cra_sha1_init(struct crypto_tfm *tfm)
    {
    return img_hash_cra_init(tfm, "sha1-lib");
    }
#[no_mangle]
unsafe extern "C" fn img_hash_cra_sha224_init(tfm: *mut crypto_tfm) -> c_int {
    static int img_hash_cra_sha224_init(struct crypto_tfm *tfm)
    {
    return img_hash_cra_init(tfm, "sha224-lib");
    }
#[no_mangle]
unsafe extern "C" fn img_hash_cra_sha256_init(tfm: *mut crypto_tfm) -> c_int {
    static int img_hash_cra_sha256_init(struct crypto_tfm *tfm)
    {
    return img_hash_cra_init(tfm, "sha256-lib");
    }
#[no_mangle]
unsafe extern "C" fn img_hash_cra_exit(tfm: *mut crypto_tfm) {
    static void img_hash_cra_exit(struct crypto_tfm *tfm)
    {
    struct img_hash_ctx *tctx = crypto_tfm_ctx(tfm);
    crypto_free_ahash(tctx.fallback);
    }
#[no_mangle]
unsafe extern "C" fn img_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t img_irq_handler(int irq, void *dev_id)
    {
    struct img_hash_dev *hdev = dev_id;
    u32 reg;
    reg = img_hash_read(hdev, CR_INTSTAT);
    img_hash_write(hdev, CR_INTCLEAR, reg);
    if (reg & CR_INT_NEW_RESULTS_SET) {
    dev_dbg(hdev.dev, "IRQ CR_INT_NEW_RESULTS_SET\n");
    if (DRIVER_FLAGS_BUSY & hdev.flags) {
    hdev.flags |= DRIVER_FLAGS_OUTPUT_READY;
    if (!(DRIVER_FLAGS_CPU & hdev.flags))
    hdev.flags |= DRIVER_FLAGS_DMA_READY;
    tasklet_schedule(&hdev.done_task);
    } else {
    dev_warn(hdev.dev,
    "HASH interrupt when no active requests.\n");
    }
    } else if (reg & CR_INT_RESULTS_AVAILABLE) {
    dev_warn(hdev.dev,
    "IRQ triggered before the hash had completed\n");
    } else if (reg & CR_INT_RESULT_READ_ERR) {
    dev_warn(hdev.dev,
    "Attempt to read from an empty result queue\n");
    } else if (reg & CR_INT_MESSAGE_WRITE_ERROR) {
    dev_warn(hdev.dev,
    "Data written before the hardware was configured\n");
    }
    return IRQ_HANDLED;
    }
    static struct ahash_alg img_algs[] = {
    {
    .init = img_hash_init,
    .update = img_hash_update,
    .final = img_hash_final,
    .finup = img_hash_finup,
    .export = img_hash_export,
    .import = img_hash_import,
    .digest = img_hash_digest,
    .halg = {
    .digestsize = MD5_DIGEST_SIZE,
    .statesize = sizeof(struct md5_state),
    .base = {
    .cra_name = "md5",
    .cra_driver_name = "img-md5",
    .cra_priority = 300,
    .cra_flags =
    CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_blocksize = MD5_HMAC_BLOCK_SIZE,
    .cra_ctxsize = sizeof(struct img_hash_ctx),
    .cra_init = img_hash_cra_md5_init,
    .cra_exit = img_hash_cra_exit,
    .cra_module = THIS_MODULE,
    }
    }
    },
    {
    .init = img_hash_init,
    .update = img_hash_update,
    .final = img_hash_final,
    .finup = img_hash_finup,
    .export = img_hash_export,
    .import = img_hash_import,
    .digest = img_hash_digest,
    .halg = {
    .digestsize = SHA1_DIGEST_SIZE,
    .statesize = sizeof(struct sha1_state),
    .base = {
    .cra_name = "sha1",
    .cra_driver_name = "img-sha1",
    .cra_priority = 300,
    .cra_flags =
    CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_blocksize = SHA1_BLOCK_SIZE,
    .cra_ctxsize = sizeof(struct img_hash_ctx),
    .cra_init = img_hash_cra_sha1_init,
    .cra_exit = img_hash_cra_exit,
    .cra_module = THIS_MODULE,
    }
    }
    },
    {
    .init = img_hash_init,
    .update = img_hash_update,
    .final = img_hash_final,
    .finup = img_hash_finup,
    .export = img_hash_export,
    .import = img_hash_import,
    .digest = img_hash_digest,
    .halg = {
    .digestsize = SHA224_DIGEST_SIZE,
    .statesize = sizeof(struct sha256_state),
    .base = {
    .cra_name = "sha224",
    .cra_driver_name = "img-sha224",
    .cra_priority = 300,
    .cra_flags =
    CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_blocksize = SHA224_BLOCK_SIZE,
    .cra_ctxsize = sizeof(struct img_hash_ctx),
    .cra_init = img_hash_cra_sha224_init,
    .cra_exit = img_hash_cra_exit,
    .cra_module = THIS_MODULE,
    }
    }
    },
    {
    .init = img_hash_init,
    .update = img_hash_update,
    .final = img_hash_final,
    .finup = img_hash_finup,
    .export = img_hash_export,
    .import = img_hash_import,
    .digest = img_hash_digest,
    .halg = {
    .digestsize = SHA256_DIGEST_SIZE,
    .statesize = sizeof(struct sha256_state),
    .base = {
    .cra_name = "sha256",
    .cra_driver_name = "img-sha256",
    .cra_priority = 300,
    .cra_flags =
    CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_blocksize = SHA256_BLOCK_SIZE,
    .cra_ctxsize = sizeof(struct img_hash_ctx),
    .cra_init = img_hash_cra_sha256_init,
    .cra_exit = img_hash_cra_exit,
    .cra_module = THIS_MODULE,
    }
    }
    }
    };
#[no_mangle]
unsafe extern "C" fn img_register_algs(hdev: *mut img_hash_dev) -> c_int {
    static int img_register_algs(struct img_hash_dev *hdev)
    {
    int i, err;
    for (i = 0; i < ARRAY_SIZE(img_algs); i++) {
    err = crypto_register_ahash(&img_algs[i]);
    if (err) {
    crypto_unregister_ahashes(img_algs, i);
    return err;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_unregister_algs(hdev: *mut img_hash_dev) {
    static void img_unregister_algs(struct img_hash_dev *hdev)
    {
    crypto_unregister_ahashes(img_algs, ARRAY_SIZE(img_algs));
    }
#[no_mangle]
unsafe extern "C" fn img_hash_done_task(data: c_ulong) {
    static void img_hash_done_task(unsigned long data)
    {
    struct img_hash_dev *hdev = (struct img_hash_dev *)data;
    let mut err: c_int = 0;
    if (hdev.err == -EINVAL) {
    err = hdev.err;
    goto finish;
    }
    if (!(DRIVER_FLAGS_BUSY & hdev.flags)) {
    img_hash_handle_queue(hdev, core::ptr::null_mut());
    return;
    }
    if (DRIVER_FLAGS_CPU & hdev.flags) {
    if (DRIVER_FLAGS_OUTPUT_READY & hdev.flags) {
    hdev.flags &= ~DRIVER_FLAGS_OUTPUT_READY;
    goto finish;
    }
    } else if (DRIVER_FLAGS_DMA_READY & hdev.flags) {
    if (DRIVER_FLAGS_DMA_ACTIVE & hdev.flags) {
    hdev.flags &= ~DRIVER_FLAGS_DMA_ACTIVE;
    img_hash_write_via_dma_stop(hdev);
    if (hdev.err) {
    err = hdev.err;
    goto finish;
    }
    }
    if (DRIVER_FLAGS_OUTPUT_READY & hdev.flags) {
    hdev.flags &= ~(DRIVER_FLAGS_DMA_READY |
    DRIVER_FLAGS_OUTPUT_READY);
    goto finish;
    }
    }
    return;
    finish:
    img_hash_finish_req(hdev.req, err);
    }
    static const struct of_device_id img_hash_match[] __maybe_unused = {
    { .compatible = "img,hash-accelerator" },
    {}
    };
    MODULE_DEVICE_TABLE(of, img_hash_match);
#[no_mangle]
unsafe extern "C" fn img_hash_probe(pdev: *mut platform_device) -> c_int {
    static int img_hash_probe(struct platform_device *pdev)
    {
    struct img_hash_dev *hdev;
    struct device *dev = &pdev.dev;
    struct resource *hash_res;
    int	irq;
    int err;
    hdev = devm_kzalloc(dev, sizeof(*hdev), GFP_KERNEL);
    if (hdev == core::ptr::null_mut())
    return -ENOMEM;
    spin_lock_init(&hdev.lock);
    hdev.dev = dev;
    platform_set_drvdata(pdev, hdev);
    INIT_LIST_HEAD(&hdev.list);
    tasklet_init(&hdev.done_task, img_hash_done_task, (unsigned long)hdev);
    tasklet_init(&hdev.dma_task, img_hash_dma_task, (unsigned long)hdev);
    crypto_init_queue(&hdev.queue, IMG_HASH_QUEUE_LENGTH);
// Register bank
    hdev.io_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(hdev.io_base)) {
    err = PTR_ERR(hdev.io_base);
    goto res_err;
    }
// Write port (DMA or CPU)
    hdev.cpu_addr = devm_platform_get_and_ioremap_resource(pdev, 1, &hash_res);
    if (IS_ERR(hdev.cpu_addr)) {
    err = PTR_ERR(hdev.cpu_addr);
    goto res_err;
    }
    hdev.bus_addr = hash_res.start;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    err = irq;
    goto res_err;
    }
    err = devm_request_irq(dev, irq, img_irq_handler, 0,
    dev_name(dev), hdev);
    if (err)
    goto res_err;
    dev_dbg(dev, "using IRQ channel %d\n", irq);
    hdev.hash_clk = devm_clk_get_enabled(&pdev.dev, "hash");
    if (IS_ERR(hdev.hash_clk)) {
    dev_err(dev, "clock initialization failed.\n");
    err = PTR_ERR(hdev.hash_clk);
    goto res_err;
    }
    hdev.sys_clk = devm_clk_get_enabled(&pdev.dev, "sys");
    if (IS_ERR(hdev.sys_clk)) {
    dev_err(dev, "clock initialization failed.\n");
    err = PTR_ERR(hdev.sys_clk);
    goto res_err;
    }
    err = img_hash_dma_init(hdev);
    if (err)
    goto res_err;
    dev_dbg(dev, "using %s for DMA transfers\n",
    dma_chan_name(hdev.dma_lch));
    spin_lock(&img_hash.lock);
    list_add_tail(&hdev.list, &img_hash.dev_list);
    spin_unlock(&img_hash.lock);
    err = img_register_algs(hdev);
    if (err)
    goto err_algs;
    dev_info(dev, "Img MD5/SHA1/SHA224/SHA256 Hardware accelerator initialized\n");
    return 0;
    err_algs:
    spin_lock(&img_hash.lock);
    list_del(&hdev.list);
    spin_unlock(&img_hash.lock);
    dma_release_channel(hdev.dma_lch);
    res_err:
    tasklet_kill(&hdev.done_task);
    tasklet_kill(&hdev.dma_task);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn img_hash_remove(pdev: *mut platform_device) {
    static void img_hash_remove(struct platform_device *pdev)
    {
    struct img_hash_dev *hdev;
    hdev = platform_get_drvdata(pdev);
    spin_lock(&img_hash.lock);
    list_del(&hdev.list);
    spin_unlock(&img_hash.lock);
    img_unregister_algs(hdev);
    tasklet_kill(&hdev.done_task);
    tasklet_kill(&hdev.dma_task);
    dma_release_channel(hdev.dma_lch);
    }

#[no_mangle]
unsafe extern "C" fn img_hash_suspend(dev: *mut device) -> c_int {
    static int img_hash_suspend(struct device *dev)
    {
    struct img_hash_dev *hdev = dev_get_drvdata(dev);
    clk_disable_unprepare(hdev.hash_clk);
    clk_disable_unprepare(hdev.sys_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn img_hash_resume(dev: *mut device) -> c_int {
    static int img_hash_resume(struct device *dev)
    {
    struct img_hash_dev *hdev = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(hdev.hash_clk);
    if (ret)
    return ret;
    ret = clk_prepare_enable(hdev.sys_clk);
    if (ret) {
    clk_disable_unprepare(hdev.hash_clk);
    return ret;
    }
    return 0;
    }

    static const struct dev_pm_ops img_hash_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(img_hash_suspend, img_hash_resume)
    };
    static struct platform_driver img_hash_driver = {
    .probe		= img_hash_probe,
    .remove		= img_hash_remove,
    .driver		= {
    .name	= "img-hash-accelerator",
    .pm	= &img_hash_pm_ops,
    .of_match_table	= img_hash_match,
    }
    };
    module_platform_driver(img_hash_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Imgtec SHA1/224/256 & MD5 hw accelerator driver");
    MODULE_AUTHOR("Will Thomas.");
    MODULE_AUTHOR("James Hartley <james.hartley@imgtec.com>");
