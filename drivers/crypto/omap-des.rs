//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/omap-des.c
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
// Support for OMAP DES and Triple DES HW acceleration.
//
// Copyright (c) 2013 Texas Instruments Incorporated
// Author: Joel Fernandes <joelf@ti.com>
//

pub const DST_MAXBURST: c_int = 2;

    ((x ^ 0x01) * 0x04))

pub const FLAGS_MODE_MASK: c_uint = 0x000f;

pub const DEFAULT_AUTOSUSPEND_DELAY: c_int = 1000;
pub const FLAGS_IN_DATA_ST_SHIFT: c_int = 8;
pub const FLAGS_OUT_DATA_ST_SHIFT: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_des_ctx {
    pub dd: *mut omap_des_dev,
    pub keylen: c_int,
    pub sizeof(u32)]: *mut *mut __le32 key[(3  DES_KEY_SIZE) /,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_des_reqctx {
    pub mode: c_ulong,
}

pub const OMAP_DES_QUEUE_LENGTH: c_int = 1;
pub const OMAP_DES_CACHE_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_des_algs_info {
    pub algs_list: *mut skcipher_engine_alg,
    pub size: c_uint,
    pub registered: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_des_pdata {
    pub algs_info: *mut omap_des_algs_info,
    pub algs_info_size: c_uint,
    pub length): *mut *mut *mut void (trigger)(struct omap_des_dev dd, int,
    pub key_ofs: u32,
    pub iv_ofs: u32,
    pub ctrl_ofs: u32,
    pub data_ofs: u32,
    pub rev_ofs: u32,
    pub mask_ofs: u32,
    pub irq_enable_ofs: u32,
    pub irq_status_ofs: u32,
    pub dma_enable_in: u32,
    pub dma_enable_out: u32,
    pub dma_start: u32,
    pub major_mask: u32,
    pub major_shift: u32,
    pub minor_mask: u32,
    pub minor_shift: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_des_dev {
    pub list: list_head,
    pub phys_base: c_ulong,
    pub io_base: *mut void __iomem,
    pub ctx: *mut omap_des_ctx,
    pub dev: *mut device,
    pub flags: c_ulong,
    pub err: c_int,
    pub done_task: work_struct,
    pub req: *mut skcipher_request,
    pub engine: *mut crypto_engine,
//
// total is used by PIO mode for book keeping so introduce
// variable total_save as need it to calc page_order
//
    pub total: usize,
    pub total_save: usize,
    pub in_sg: *mut scatterlist,
    pub out_sg: *mut scatterlist,
// Buffers for copying for unaligned cases
    pub in_sgl: scatterlist,
    pub out_sgl: scatterlist,
    pub orig_out: *mut scatterlist,
    pub in_sg_offset: c_uint,
    pub out_sg_offset: c_uint,
    pub dma_lch_in: *mut dma_chan,
    pub dma_lch_out: *mut dma_chan,
    pub in_sg_len: c_int,
    pub out_sg_len: c_int,
    pub pio_only: c_int,
    pub pdata: *const omap_des_pdata,
}

// keep registered devices data here
    static LIST_HEAD(dev_list);
    static DEFINE_SPINLOCK(list_lock);

    ({                                                              \
    int _read_ret;                                          \
    _read_ret = __raw_readl(dd.io_base + offset);          \
    pr_err("omap_des_read(" #offset "=%#x)= %#x\n",       \
    offset, _read_ret);                            \
    _read_ret;                                              \
    })

#[no_mangle]
pub unsafe extern "C" fn omap_des_read(dd: *mut omap_des_dev, offset: u32) -> u32 {
    static inline u32 omap_des_read(struct omap_des_dev *dd, u32 offset)
    {
    return __raw_readl(dd.io_base + offset);
    }

    do {                                                            \
    pr_err("omap_des_write(" #offset "=%#x) value=%#x\n", \
    offset, value);                                \
    __raw_writel(value, dd.io_base + offset);              \
    } while (0)

    static inline void omap_des_write(struct omap_des_dev *dd, u32 offset,
    u32 value)
    {
    __raw_writel(value, dd.io_base + offset);
    }

    static inline void omap_des_write_mask(struct omap_des_dev *dd, u32 offset,
    u32 value, u32 mask)
    {
    u32 val;
    val = omap_des_read(dd, offset);
    val &= ~mask;
    val |= value;
    omap_des_write(dd, offset, val);
    }
    static void omap_des_write_n(struct omap_des_dev *dd, u32 offset,
    u32 *value, int count)
    {
    for (; count--; value++, offset += 4)
    omap_des_write(dd, offset, *value);
    }
#[no_mangle]
unsafe extern "C" fn omap_des_hw_init(dd: *mut omap_des_dev) -> c_int {
    static int omap_des_hw_init(struct omap_des_dev *dd)
    {
    int err;
//
// clocks are enabled when request starts and disabled when finished.
// It may be long delays between requests.
// Device might go to off mode to save power.
//
    err = pm_runtime_resume_and_get(dd.dev);
    if (err < 0) {
    dev_err(dd.dev, "%s: failed to get_sync(%d)\n", __func__, err);
    return err;
    }
    if (!(dd.flags & FLAGS_INIT)) {
    dd.flags |= FLAGS_INIT;
    dd.err = 0;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_des_write_ctrl(dd: *mut omap_des_dev) -> c_int {
    static int omap_des_write_ctrl(struct omap_des_dev *dd)
    {
    unsigned int key32;
    int i, err;
    let mut val: u32 = 0, mask = 0;
    err = omap_des_hw_init(dd);
    if (err)
    return err;
    key32 = dd.ctx.keylen / sizeof(u32);
// it seems a key should always be set even if it has not changed
    for (i = 0; i < key32; i++) {
    omap_des_write(dd, DES_REG_KEY(dd, i),
    __le32_to_cpu(dd.ctx.key[i]));
    }
    if ((dd.flags & FLAGS_CBC) && dd.req.iv)
    omap_des_write_n(dd, DES_REG_IV(dd, 0), (void *)dd.req.iv, 2);
    if (dd.flags & FLAGS_CBC)
    val |= DES_REG_CTRL_CBC;
    if (dd.flags & FLAGS_ENCRYPT)
    val |= DES_REG_CTRL_DIRECTION;
    if (key32 == 6)
    val |= DES_REG_CTRL_TDES;
    mask |= DES_REG_CTRL_CBC | DES_REG_CTRL_DIRECTION | DES_REG_CTRL_TDES;
    omap_des_write_mask(dd, DES_REG_CTRL(dd), val, mask);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_des_dma_trigger_omap4(dd: *mut omap_des_dev, length: c_int) {
    static void omap_des_dma_trigger_omap4(struct omap_des_dev *dd, int length)
    {
    u32 mask, val;
    omap_des_write(dd, DES_REG_LENGTH_N(0), length);
    val = dd.pdata.dma_start;
    if (dd.dma_lch_out != core::ptr::null_mut())
    val |= dd.pdata.dma_enable_out;
    if (dd.dma_lch_in != core::ptr::null_mut())
    val |= dd.pdata.dma_enable_in;
    mask = dd.pdata.dma_enable_out | dd.pdata.dma_enable_in |
    dd.pdata.dma_start;
    omap_des_write_mask(dd, DES_REG_MASK(dd), val, mask);
    }
#[no_mangle]
unsafe extern "C" fn omap_des_dma_stop(dd: *mut omap_des_dev) {
    static void omap_des_dma_stop(struct omap_des_dev *dd)
    {
    u32 mask;
    mask = dd.pdata.dma_enable_out | dd.pdata.dma_enable_in |
    dd.pdata.dma_start;
    omap_des_write_mask(dd, DES_REG_MASK(dd), 0, mask);
    }
    static struct omap_des_dev *omap_des_find_dev(struct omap_des_ctx *ctx)
    {
    struct omap_des_dev *dd = core::ptr::null_mut(), *tmp;
    spin_lock_bh(&list_lock);
    if (!ctx.dd) {
    list_for_each_entry(tmp, &dev_list, list) {
// FIXME: take fist available des core
    dd = tmp;
    break;
    }
    ctx.dd = dd;
    } else {
// already found before
    dd = ctx.dd;
    }
    spin_unlock_bh(&list_lock);
    return dd;
    }
#[no_mangle]
unsafe extern "C" fn omap_des_dma_out_callback(data: *mut c_void) {
    static void omap_des_dma_out_callback(void *data)
    {
    struct omap_des_dev *dd = data;
// dma_lch_out - completed
    queue_work(system_bh_wq, &dd.done_task);
    }
#[no_mangle]
unsafe extern "C" fn omap_des_dma_init(dd: *mut omap_des_dev) -> c_int {
    static int omap_des_dma_init(struct omap_des_dev *dd)
    {
    int err;
    dd.dma_lch_out = core::ptr::null_mut();
    dd.dma_lch_in = core::ptr::null_mut();
    dd.dma_lch_in = dma_request_chan(dd.dev, "rx");
    if (IS_ERR(dd.dma_lch_in)) {
    dev_err(dd.dev, "Unable to request in DMA channel\n");
    return PTR_ERR(dd.dma_lch_in);
    }
    dd.dma_lch_out = dma_request_chan(dd.dev, "tx");
    if (IS_ERR(dd.dma_lch_out)) {
    dev_err(dd.dev, "Unable to request out DMA channel\n");
    err = PTR_ERR(dd.dma_lch_out);
    goto err_dma_out;
    }
    return 0;
    err_dma_out:
    dma_release_channel(dd.dma_lch_in);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn omap_des_dma_cleanup(dd: *mut omap_des_dev) {
    static void omap_des_dma_cleanup(struct omap_des_dev *dd)
    {
    if (dd.pio_only)
    return;
    dma_release_channel(dd.dma_lch_out);
    dma_release_channel(dd.dma_lch_in);
    }
    static int omap_des_crypt_dma(struct crypto_tfm *tfm,
    struct scatterlist *in_sg, struct scatterlist *out_sg,
    int in_sg_len, int out_sg_len)
    {
    struct omap_des_ctx *ctx = crypto_tfm_ctx(tfm);
    struct omap_des_dev *dd = ctx.dd;
    struct dma_async_tx_descriptor *tx_in, *tx_out;
    struct dma_slave_config cfg;
    int ret;
    if (dd.pio_only) {
    dd.in_sg_offset = 0;
    dd.out_sg_offset = 0;
// Enable DATAIN interrupt and let it take
    care of the rest */
    omap_des_write(dd, DES_REG_IRQ_ENABLE(dd), 0x2);
    return 0;
    }
    dma_sync_sg_for_device(dd.dev, dd.in_sg, in_sg_len, DMA_TO_DEVICE);
    memset(&cfg, 0, sizeof(cfg));
    cfg.src_addr = dd.phys_base + DES_REG_DATA_N(dd, 0);
    cfg.dst_addr = dd.phys_base + DES_REG_DATA_N(dd, 0);
    cfg.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    cfg.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    cfg.src_maxburst = DST_MAXBURST;
    cfg.dst_maxburst = DST_MAXBURST;
// IN
    ret = dmaengine_slave_config(dd.dma_lch_in, &cfg);
    if (ret) {
    dev_err(dd.dev, "can't configure IN dmaengine slave: %d\n",
    ret);
    return ret;
    }
    tx_in = dmaengine_prep_slave_sg(dd.dma_lch_in, in_sg, in_sg_len,
    DMA_MEM_TO_DEV,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!tx_in) {
    dev_err(dd.dev, "IN prep_slave_sg() failed\n");
    return -EINVAL;
    }
// No callback necessary
    tx_in.callback_param = dd;
// OUT
    ret = dmaengine_slave_config(dd.dma_lch_out, &cfg);
    if (ret) {
    dev_err(dd.dev, "can't configure OUT dmaengine slave: %d\n",
    ret);
    return ret;
    }
    tx_out = dmaengine_prep_slave_sg(dd.dma_lch_out, out_sg, out_sg_len,
    DMA_DEV_TO_MEM,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!tx_out) {
    dev_err(dd.dev, "OUT prep_slave_sg() failed\n");
    return -EINVAL;
    }
    tx_out.callback = omap_des_dma_out_callback;
    tx_out.callback_param = dd;
    dmaengine_submit(tx_in);
    dmaengine_submit(tx_out);
    dma_async_issue_pending(dd.dma_lch_in);
    dma_async_issue_pending(dd.dma_lch_out);
// start DMA
    dd.pdata.trigger(dd, dd.total);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_des_crypt_dma_start(dd: *mut omap_des_dev) -> c_int {
    static int omap_des_crypt_dma_start(struct omap_des_dev *dd)
    {
    struct crypto_tfm *tfm = crypto_skcipher_tfm(
    crypto_skcipher_reqtfm(dd.req));
    int err;
    pr_debug("total: %zd\n", dd.total);
    if (!dd.pio_only) {
    err = dma_map_sg(dd.dev, dd.in_sg, dd.in_sg_len,
    DMA_TO_DEVICE);
    if (!err) {
    dev_err(dd.dev, "dma_map_sg() error\n");
    return -EINVAL;
    }
    err = dma_map_sg(dd.dev, dd.out_sg, dd.out_sg_len,
    DMA_FROM_DEVICE);
    if (!err) {
    dev_err(dd.dev, "dma_map_sg() error\n");
    return -EINVAL;
    }
    }
    err = omap_des_crypt_dma(tfm, dd.in_sg, dd.out_sg, dd.in_sg_len,
    dd.out_sg_len);
    if (err && !dd.pio_only) {
    dma_unmap_sg(dd.dev, dd.in_sg, dd.in_sg_len, DMA_TO_DEVICE);
    dma_unmap_sg(dd.dev, dd.out_sg, dd.out_sg_len,
    DMA_FROM_DEVICE);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn omap_des_finish_req(dd: *mut omap_des_dev, err: c_int) {
    static void omap_des_finish_req(struct omap_des_dev *dd, int err)
    {
    struct skcipher_request *req = dd.req;
    pr_debug("err: %d\n", err);
    crypto_finalize_skcipher_request(dd.engine, req, err);
    pm_runtime_put_autosuspend(dd.dev);
    }
#[no_mangle]
unsafe extern "C" fn omap_des_crypt_dma_stop(dd: *mut omap_des_dev) -> c_int {
    static int omap_des_crypt_dma_stop(struct omap_des_dev *dd)
    {
    pr_debug("total: %zd\n", dd.total);
    omap_des_dma_stop(dd);
    dmaengine_terminate_all(dd.dma_lch_in);
    dmaengine_terminate_all(dd.dma_lch_out);
    return 0;
    }
    static int omap_des_handle_queue(struct omap_des_dev *dd,
    struct skcipher_request *req)
    {
    if (req)
    return crypto_transfer_skcipher_request_to_engine(dd.engine, req);
    return 0;
    }
    static int omap_des_prepare_req(struct skcipher_request *req,
    struct omap_des_dev *dd)
    {
    struct omap_des_ctx *ctx = crypto_skcipher_ctx(
    crypto_skcipher_reqtfm(req));
    struct omap_des_reqctx *rctx;
    int ret;
    u16 flags;
// assign new request to device
    dd.req = req;
    dd.total = req.cryptlen;
    dd.total_save = req.cryptlen;
    dd.in_sg = req.src;
    dd.out_sg = req.dst;
    dd.orig_out = req.dst;
    flags = OMAP_CRYPTO_COPY_DATA;
    if (req.src == req.dst)
    flags |= OMAP_CRYPTO_FORCE_COPY;
    ret = omap_crypto_align_sg(&dd.in_sg, dd.total, DES_BLOCK_SIZE,
    &dd.in_sgl, flags,
    FLAGS_IN_DATA_ST_SHIFT, &dd.flags);
    if (ret)
    return ret;
    ret = omap_crypto_align_sg(&dd.out_sg, dd.total, DES_BLOCK_SIZE,
    &dd.out_sgl, 0,
    FLAGS_OUT_DATA_ST_SHIFT, &dd.flags);
    if (ret)
    return ret;
    dd.in_sg_len = sg_nents_for_len(dd.in_sg, dd.total);
    if (dd.in_sg_len < 0)
    return dd.in_sg_len;
    dd.out_sg_len = sg_nents_for_len(dd.out_sg, dd.total);
    if (dd.out_sg_len < 0)
    return dd.out_sg_len;
    rctx = skcipher_request_ctx(req);
    ctx = crypto_skcipher_ctx(crypto_skcipher_reqtfm(req));
    rctx.mode &= FLAGS_MODE_MASK;
    dd.flags = (dd.flags & ~FLAGS_MODE_MASK) | rctx.mode;
    dd.ctx = ctx;
    ctx.dd = dd;
    return omap_des_write_ctrl(dd);
    }
    static int omap_des_crypt_req(struct crypto_engine *engine,
    void *areq)
    {
    struct skcipher_request *req = container_of(areq, struct skcipher_request, base);
    struct omap_des_ctx *ctx = crypto_skcipher_ctx(
    crypto_skcipher_reqtfm(req));
    struct omap_des_dev *dd = omap_des_find_dev(ctx);
    if (!dd)
    return -ENODEV;
    return omap_des_prepare_req(req, dd) ?:
    omap_des_crypt_dma_start(dd);
    }
#[no_mangle]
unsafe extern "C" fn omap_des_done_task(t: *mut work_struct) {
    static void omap_des_done_task(struct work_struct *t)
    {
    struct omap_des_dev *dd = from_work(dd, t, done_task);
    int i;
    pr_debug("enter done_task\n");
    if (!dd.pio_only) {
    dma_sync_sg_for_device(dd.dev, dd.out_sg, dd.out_sg_len,
    DMA_FROM_DEVICE);
    dma_unmap_sg(dd.dev, dd.in_sg, dd.in_sg_len, DMA_TO_DEVICE);
    dma_unmap_sg(dd.dev, dd.out_sg, dd.out_sg_len,
    DMA_FROM_DEVICE);
    omap_des_crypt_dma_stop(dd);
    }
    omap_crypto_cleanup(&dd.in_sgl, core::ptr::null_mut(), 0, dd.total_save,
    FLAGS_IN_DATA_ST_SHIFT, dd.flags);
    omap_crypto_cleanup(&dd.out_sgl, dd.orig_out, 0, dd.total_save,
    FLAGS_OUT_DATA_ST_SHIFT, dd.flags);
    if ((dd.flags & FLAGS_CBC) && dd.req.iv)
    for (i = 0; i < 2; i++)
    ((u32 *)dd.req.iv)[i] =
    omap_des_read(dd, DES_REG_IV(dd, i));
    omap_des_finish_req(dd, 0);
    pr_debug("exit\n");
    }
#[no_mangle]
unsafe extern "C" fn omap_des_crypt(req: *mut skcipher_request, mode: c_ulong) -> c_int {
    static int omap_des_crypt(struct skcipher_request *req, unsigned long mode)
    {
    struct omap_des_ctx *ctx = crypto_skcipher_ctx(
    crypto_skcipher_reqtfm(req));
    struct omap_des_reqctx *rctx = skcipher_request_ctx(req);
    struct omap_des_dev *dd;
    pr_debug("nbytes: %d, enc: %d, cbc: %d\n", req.cryptlen,
    !!(mode & FLAGS_ENCRYPT),
    !!(mode & FLAGS_CBC));
    if (!req.cryptlen)
    return 0;
    if (!IS_ALIGNED(req.cryptlen, DES_BLOCK_SIZE))
    return -EINVAL;
    dd = omap_des_find_dev(ctx);
    if (!dd)
    return -ENODEV;
    rctx.mode = mode;
    return omap_des_handle_queue(dd, req);
    }
// ********************** ALG API ************************************
    static int omap_des_setkey(struct crypto_skcipher *cipher, const u8 *key,
    unsigned int keylen)
    {
    struct omap_des_ctx *ctx = crypto_skcipher_ctx(cipher);
    int err;
    pr_debug("enter, keylen: %d\n", keylen);
    err = verify_skcipher_des_key(cipher, key);
    if (err)
    return err;
    memcpy(ctx.key, key, keylen);
    ctx.keylen = keylen;
    return 0;
    }
    static int omap_des3_setkey(struct crypto_skcipher *cipher, const u8 *key,
    unsigned int keylen)
    {
    struct omap_des_ctx *ctx = crypto_skcipher_ctx(cipher);
    int err;
    pr_debug("enter, keylen: %d\n", keylen);
    err = verify_skcipher_des3_key(cipher, key);
    if (err)
    return err;
    memcpy(ctx.key, key, keylen);
    ctx.keylen = keylen;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_des_ecb_encrypt(req: *mut skcipher_request) -> c_int {
    static int omap_des_ecb_encrypt(struct skcipher_request *req)
    {
    return omap_des_crypt(req, FLAGS_ENCRYPT);
    }
#[no_mangle]
unsafe extern "C" fn omap_des_ecb_decrypt(req: *mut skcipher_request) -> c_int {
    static int omap_des_ecb_decrypt(struct skcipher_request *req)
    {
    return omap_des_crypt(req, 0);
    }
#[no_mangle]
unsafe extern "C" fn omap_des_cbc_encrypt(req: *mut skcipher_request) -> c_int {
    static int omap_des_cbc_encrypt(struct skcipher_request *req)
    {
    return omap_des_crypt(req, FLAGS_ENCRYPT | FLAGS_CBC);
    }
#[no_mangle]
unsafe extern "C" fn omap_des_cbc_decrypt(req: *mut skcipher_request) -> c_int {
    static int omap_des_cbc_decrypt(struct skcipher_request *req)
    {
    return omap_des_crypt(req, FLAGS_CBC);
    }
#[no_mangle]
unsafe extern "C" fn omap_des_init_tfm(tfm: *mut crypto_skcipher) -> c_int {
    static int omap_des_init_tfm(struct crypto_skcipher *tfm)
    {
    pr_debug("enter\n");
    crypto_skcipher_set_reqsize(tfm, sizeof(struct omap_des_reqctx));
    return 0;
    }
// ********************** ALGS ************************************
    static struct skcipher_engine_alg algs_ecb_cbc[] = {
    {
    .base = {
    .base.cra_name		= "ecb(des)",
    .base.cra_driver_name	= "ecb-des-omap",
    .base.cra_priority	= 300,
    .base.cra_flags		= CRYPTO_ALG_KERN_DRIVER_ONLY |
    CRYPTO_ALG_ASYNC,
    .base.cra_blocksize	= DES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct omap_des_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= DES_KEY_SIZE,
    .max_keysize		= DES_KEY_SIZE,
    .setkey			= omap_des_setkey,
    .encrypt		= omap_des_ecb_encrypt,
    .decrypt		= omap_des_ecb_decrypt,
    .init			= omap_des_init_tfm,
    },
    .op.do_one_request = omap_des_crypt_req,
    },
    {
    .base = {
    .base.cra_name		= "cbc(des)",
    .base.cra_driver_name	= "cbc-des-omap",
    .base.cra_priority	= 300,
    .base.cra_flags		= CRYPTO_ALG_KERN_DRIVER_ONLY |
    CRYPTO_ALG_ASYNC,
    .base.cra_blocksize	= DES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct omap_des_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= DES_KEY_SIZE,
    .max_keysize		= DES_KEY_SIZE,
    .ivsize			= DES_BLOCK_SIZE,
    .setkey			= omap_des_setkey,
    .encrypt		= omap_des_cbc_encrypt,
    .decrypt		= omap_des_cbc_decrypt,
    .init			= omap_des_init_tfm,
    },
    .op.do_one_request = omap_des_crypt_req,
    },
    {
    .base = {
    .base.cra_name		= "ecb(des3_ede)",
    .base.cra_driver_name	= "ecb-des3-omap",
    .base.cra_priority	= 300,
    .base.cra_flags		= CRYPTO_ALG_KERN_DRIVER_ONLY |
    CRYPTO_ALG_ASYNC,
    .base.cra_blocksize	= DES3_EDE_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct omap_des_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= DES3_EDE_KEY_SIZE,
    .max_keysize		= DES3_EDE_KEY_SIZE,
    .setkey			= omap_des3_setkey,
    .encrypt		= omap_des_ecb_encrypt,
    .decrypt		= omap_des_ecb_decrypt,
    .init			= omap_des_init_tfm,
    },
    .op.do_one_request = omap_des_crypt_req,
    },
    {
    .base = {
    .base.cra_name		= "cbc(des3_ede)",
    .base.cra_driver_name	= "cbc-des3-omap",
    .base.cra_priority	= 300,
    .base.cra_flags		= CRYPTO_ALG_KERN_DRIVER_ONLY |
    CRYPTO_ALG_ASYNC,
    .base.cra_blocksize	= DES3_EDE_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct omap_des_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= DES3_EDE_KEY_SIZE,
    .max_keysize		= DES3_EDE_KEY_SIZE,
    .ivsize			= DES3_EDE_BLOCK_SIZE,
    .setkey			= omap_des3_setkey,
    .encrypt		= omap_des_cbc_encrypt,
    .decrypt		= omap_des_cbc_decrypt,
    .init			= omap_des_init_tfm,
    },
    .op.do_one_request = omap_des_crypt_req,
    }
    };
    static struct omap_des_algs_info omap_des_algs_info_ecb_cbc[] = {
    {
    .algs_list	= algs_ecb_cbc,
    .size		= ARRAY_SIZE(algs_ecb_cbc),
    },
    };
    static const struct omap_des_pdata omap_des_pdata_omap4 = {
    .algs_info	= omap_des_algs_info_ecb_cbc,
    .algs_info_size	= ARRAY_SIZE(omap_des_algs_info_ecb_cbc),
    .trigger	= omap_des_dma_trigger_omap4,
    .key_ofs	= 0x14,
    .iv_ofs		= 0x18,
    .ctrl_ofs	= 0x20,
    .data_ofs	= 0x28,
    .rev_ofs	= 0x30,
    .mask_ofs	= 0x34,
    .irq_status_ofs = 0x3c,
    .irq_enable_ofs = 0x40,
    .dma_enable_in	= BIT(5),
    .dma_enable_out	= BIT(6),
    .major_mask	= 0x0700,
    .major_shift	= 8,
    .minor_mask	= 0x003f,
    .minor_shift	= 0,
    };
#[no_mangle]
unsafe extern "C" fn omap_des_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t omap_des_irq(int irq, void *dev_id)
    {
    struct omap_des_dev *dd = dev_id;
    u32 status, i;
    u32 *src, *dst;
    status = omap_des_read(dd, DES_REG_IRQ_STATUS(dd));
    if (status & DES_REG_IRQ_DATA_IN) {
    omap_des_write(dd, DES_REG_IRQ_ENABLE(dd), 0x0);
    BUG_ON(!dd.in_sg);
    BUG_ON(dd.in_sg_offset > dd.in_sg.length);
    src = sg_virt(dd.in_sg) + dd.in_sg_offset;
    for (i = 0; i < DES_BLOCK_WORDS; i++) {
    omap_des_write(dd, DES_REG_DATA_N(dd, i), *src);
    dd.in_sg_offset += 4;
    if (dd.in_sg_offset == dd.in_sg.length) {
    dd.in_sg = sg_next(dd.in_sg);
    if (dd.in_sg) {
    dd.in_sg_offset = 0;
    src = sg_virt(dd.in_sg);
    }
    } else {
    src++;
    }
    }
// Clear IRQ status
    status &= ~DES_REG_IRQ_DATA_IN;
    omap_des_write(dd, DES_REG_IRQ_STATUS(dd), status);
// Enable DATA_OUT interrupt
    omap_des_write(dd, DES_REG_IRQ_ENABLE(dd), 0x4);
    } else if (status & DES_REG_IRQ_DATA_OUT) {
    omap_des_write(dd, DES_REG_IRQ_ENABLE(dd), 0x0);
    BUG_ON(!dd.out_sg);
    BUG_ON(dd.out_sg_offset > dd.out_sg.length);
    dst = sg_virt(dd.out_sg) + dd.out_sg_offset;
    for (i = 0; i < DES_BLOCK_WORDS; i++) {
// dst = omap_des_read(dd, DES_REG_DATA_N(dd, i));
    dd.out_sg_offset += 4;
    if (dd.out_sg_offset == dd.out_sg.length) {
    dd.out_sg = sg_next(dd.out_sg);
    if (dd.out_sg) {
    dd.out_sg_offset = 0;
    dst = sg_virt(dd.out_sg);
    }
    } else {
    dst++;
    }
    }
    BUG_ON(dd.total < DES_BLOCK_SIZE);
    dd.total -= DES_BLOCK_SIZE;
// Clear IRQ status
    status &= ~DES_REG_IRQ_DATA_OUT;
    omap_des_write(dd, DES_REG_IRQ_STATUS(dd), status);
    if (!dd.total)
// All bytes read!
    queue_work(system_bh_wq, &dd.done_task);
    else
// Enable DATA_IN interrupt for next block
    omap_des_write(dd, DES_REG_IRQ_ENABLE(dd), 0x2);
    }
    return IRQ_HANDLED;
    }
    static const struct of_device_id omap_des_of_match[] = {
    {
    .compatible	= "ti,omap4-des",
    .data		= &omap_des_pdata_omap4,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, omap_des_of_match);

    static int omap_des_get_of(struct omap_des_dev *dd,
    struct platform_device *pdev)
    {
    dd.pdata = of_device_get_match_data(&pdev.dev);
    if (!dd.pdata) {
    dev_err(&pdev.dev, "no compatible OF match\n");
    return -EINVAL;
    }
    return 0;
    }

    static int omap_des_get_of(struct omap_des_dev *dd,
    struct platform_device *pdev)
    {
    return -EINVAL;
    }

    static int omap_des_get_pdev(struct omap_des_dev *dd,
    struct platform_device *pdev)
    {
// non-DT devices get pdata from pdev
    dd.pdata = pdev.dev.platform_data;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_des_unregister_algs(pdata: *const omap_des_pdata) {
    static void omap_des_unregister_algs(const struct omap_des_pdata *pdata)
    {
    struct omap_des_algs_info *alg_info;
    int i;
    for (i = pdata.algs_info_size - 1; i >= 0; i--) {
    alg_info = &pdata.algs_info[i];
    crypto_engine_unregister_skciphers(alg_info.algs_list,
    alg_info.registered);
    alg_info.registered = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn omap_des_probe(pdev: *mut platform_device) -> c_int {
    static int omap_des_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct omap_des_dev *dd;
    struct skcipher_engine_alg *algp;
    struct resource *res;
    let mut err: c_int = -ENOMEM, i, j, irq = -1;
    u32 reg;
    dd = devm_kzalloc(dev, sizeof(struct omap_des_dev), GFP_KERNEL);
    if (dd == core::ptr::null_mut()) {
    dev_err(dev, "unable to alloc data struct.\n");
    goto err_data;
    }
    dd.dev = dev;
    platform_set_drvdata(pdev, dd);
    err = (dev.of_node) ? omap_des_get_of(dd, pdev) :
    omap_des_get_pdev(dd, pdev);
    if (err)
    goto err_res;
    dd.io_base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(dd.io_base)) {
    err = PTR_ERR(dd.io_base);
    goto err_res;
    }
    dd.phys_base = res.start;
    pm_runtime_use_autosuspend(dev);
    pm_runtime_set_autosuspend_delay(dev, DEFAULT_AUTOSUSPEND_DELAY);
    pm_runtime_enable(dev);
    err = pm_runtime_resume_and_get(dev);
    if (err < 0) {
    dev_err(dd.dev, "%s: failed to get_sync(%d)\n", __func__, err);
    goto err_get;
    }
    omap_des_dma_stop(dd);
    reg = omap_des_read(dd, DES_REG_REV(dd));
    pm_runtime_put_sync(dev);
    dev_info(dev, "OMAP DES hw accel rev: %u.%u\n",
    (reg & dd.pdata.major_mask) >> dd.pdata.major_shift,
    (reg & dd.pdata.minor_mask) >> dd.pdata.minor_shift);
    INIT_WORK(&dd.done_task, omap_des_done_task);
    err = omap_des_dma_init(dd);
    if (err == -EPROBE_DEFER) {
    goto err_irq;
    } else if (err && DES_REG_IRQ_STATUS(dd) && DES_REG_IRQ_ENABLE(dd)) {
    dd.pio_only = 1;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    err = irq;
    goto err_irq;
    }
    err = devm_request_irq(dev, irq, omap_des_irq, 0,
    dev_name(dev), dd);
    if (err)
    goto err_irq;
    }
    INIT_LIST_HEAD(&dd.list);
    spin_lock_bh(&list_lock);
    list_add_tail(&dd.list, &dev_list);
    spin_unlock_bh(&list_lock);
// Initialize des crypto engine
    dd.engine = crypto_engine_alloc_init(dev, 1);
    if (!dd.engine) {
    err = -ENOMEM;
    goto err_engine;
    }
    err = crypto_engine_start(dd.engine);
    if (err)
    goto err_engine;
    for (i = 0; i < dd.pdata.algs_info_size; i++) {
    for (j = 0; j < dd.pdata.algs_info[i].size; j++) {
    algp = &dd.pdata.algs_info[i].algs_list[j];
    pr_debug("reg alg: %s\n", algp.base.base.cra_name);
    err = crypto_engine_register_skcipher(algp);
    if (err)
    goto err_algs;
    dd.pdata.algs_info[i].registered++;
    }
    }
    return 0;
    err_algs:
    omap_des_unregister_algs(dd.pdata);
    err_engine:
    if (dd.engine)
    crypto_engine_exit(dd.engine);
    omap_des_dma_cleanup(dd);
    err_irq:
    cancel_work_sync(&dd.done_task);
    err_get:
    pm_runtime_disable(dev);
    err_res:
    dd = core::ptr::null_mut();
    err_data:
    dev_err(dev, "initialization failed.\n");
    return err;
    }
#[no_mangle]
unsafe extern "C" fn omap_des_remove(pdev: *mut platform_device) {
    static void omap_des_remove(struct platform_device *pdev)
    {
    struct omap_des_dev *dd = platform_get_drvdata(pdev);
    spin_lock_bh(&list_lock);
    list_del(&dd.list);
    spin_unlock_bh(&list_lock);
    omap_des_unregister_algs(dd.pdata);
    cancel_work_sync(&dd.done_task);
    omap_des_dma_cleanup(dd);
    pm_runtime_disable(dd.dev);
    }

#[no_mangle]
unsafe extern "C" fn omap_des_suspend(dev: *mut device) -> c_int {
    static int omap_des_suspend(struct device *dev)
    {
    pm_runtime_put_sync(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_des_resume(dev: *mut device) -> c_int {
    static int omap_des_resume(struct device *dev)
    {
    int err;
    err = pm_runtime_resume_and_get(dev);
    if (err < 0) {
    dev_err(dev, "%s: failed to get_sync(%d)\n", __func__, err);
    return err;
    }
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(omap_des_pm_ops, omap_des_suspend, omap_des_resume);
    static struct platform_driver omap_des_driver = {
    .probe	= omap_des_probe,
    .remove = omap_des_remove,
    .driver	= {
    .name	= "omap-des",
    .pm	= &omap_des_pm_ops,
    .of_match_table	= omap_des_of_match,
    },
    };
    module_platform_driver(omap_des_driver);
    MODULE_DESCRIPTION("OMAP DES hw acceleration support.");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Joel Fernandes <joelf@ti.com>");
