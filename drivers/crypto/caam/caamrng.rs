//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/caam/caamrng.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// caam - Freescale FSL CAAM support for hw_random
//
// Copyright 2011 Freescale Semiconductor, Inc.
// Copyright 2018-2019, 2023 NXP
//
// Based on caamalg.c crypto API driver.
//

pub const CAAM_RNG_MAX_FIFO_STORE_SIZE: c_int = 16;
//
// Length of used descriptors, see caam_init_desc()
//

    CAAM_CMD_SZ +				\
    CAAM_CMD_SZ + CAAM_PTR_SZ_MAX)
// rng per-device context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_rng_ctx {
    pub rng: hwrng,
    pub jrdev: *mut device,
    pub ctrldev: *mut device,
    pub desc_async: *mut c_void,
    pub desc_sync: *mut c_void,
    pub worker: work_struct,
    pub fifo: kfifo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_rng_job_ctx {
    pub done: *mut completion,
    pub err: *mut c_int,
}

    static struct caam_rng_ctx *to_caam_rng_ctx(struct hwrng *r)
    {
    return (struct caam_rng_ctx *)r.priv;
    }
    static void caam_rng_done(struct device *jrdev, u32 *desc, u32 err,
    void *context)
    {
    struct caam_rng_job_ctx *jctx = context;
    if (err)
// jctx->err = caam_jr_strstatus(jrdev, err);
    complete(jctx.done);
    }
    static u32 *caam_init_desc(u32 *desc, dma_addr_t dst_dma)
    {
    init_job_desc(desc, 0);	/* + 1 cmd_sz */
// Generate random bytes: + 1 cmd_sz
    append_operation(desc, OP_ALG_ALGSEL_RNG | OP_TYPE_CLASS1_ALG |
    OP_ALG_PR_ON);
// Store bytes: + 1 cmd_sz + caam_ptr_sz
    append_fifo_store(desc, dst_dma,
    CAAM_RNG_MAX_FIFO_STORE_SIZE, FIFOST_TYPE_RNGSTORE);
    print_hex_dump_debug("rng job desc@: ", DUMP_PREFIX_ADDRESS,
    16, 4, desc, desc_bytes(desc), 1);
    return desc;
    }
    static int caam_rng_read_one(struct device *jrdev,
    void *dst, int len,
    void *desc,
    struct completion *done)
    {
    dma_addr_t dst_dma;
    int err, ret = 0;
    struct caam_rng_job_ctx jctx = {
    .done = done,
    .err  = &ret,
    };
    len = CAAM_RNG_MAX_FIFO_STORE_SIZE;
    dst_dma = dma_map_single(jrdev, dst, len, DMA_FROM_DEVICE);
    if (dma_mapping_error(jrdev, dst_dma)) {
    dev_err(jrdev, "unable to map destination memory\n");
    return -ENOMEM;
    }
    init_completion(done);
    err = caam_jr_enqueue(jrdev,
    caam_init_desc(desc, dst_dma),
    caam_rng_done, &jctx);
    if (err == -EINPROGRESS) {
    wait_for_completion(done);
    err = 0;
    }
    dma_unmap_single(jrdev, dst_dma, len, DMA_FROM_DEVICE);
    return err ?: (ret ?: len);
    }
#[no_mangle]
unsafe extern "C" fn caam_rng_fill_async(ctx: *mut caam_rng_ctx) {
    static void caam_rng_fill_async(struct caam_rng_ctx *ctx)
    {
    struct scatterlist sg[1];
    struct completion done;
    int len, nents;
    sg_init_table(sg, ARRAY_SIZE(sg));
    nents = kfifo_dma_in_prepare(&ctx.fifo, sg, ARRAY_SIZE(sg),
    CAAM_RNG_MAX_FIFO_STORE_SIZE);
    if (!nents)
    return;
    len = caam_rng_read_one(ctx.jrdev, sg_virt(&sg[0]),
    sg[0].length,
    ctx.desc_async,
    &done);
    if (len < 0)
    return;
    kfifo_dma_in_finish(&ctx.fifo, len);
    }
#[no_mangle]
unsafe extern "C" fn caam_rng_worker(work: *mut work_struct) {
    static void caam_rng_worker(struct work_struct *work)
    {
    struct caam_rng_ctx *ctx = container_of(work, struct caam_rng_ctx,
    worker);
    caam_rng_fill_async(ctx);
    }
#[no_mangle]
unsafe extern "C" fn caam_read(rng: *mut hwrng, dst: *mut c_void, max: usize, wait: bool) -> c_int {
    static int caam_read(struct hwrng *rng, void *dst, size_t max, bool wait)
    {
    struct caam_rng_ctx *ctx = to_caam_rng_ctx(rng);
    int out;
    if (wait) {
    struct completion done;
    return caam_rng_read_one(ctx.jrdev, dst, max,
    ctx.desc_sync, &done);
    }
    out = kfifo_out(&ctx.fifo, dst, max);
    if (kfifo_is_empty(&ctx.fifo))
    schedule_work(&ctx.worker);
    return out;
    }
#[no_mangle]
unsafe extern "C" fn caam_cleanup(rng: *mut hwrng) {
    static void caam_cleanup(struct hwrng *rng)
    {
    struct caam_rng_ctx *ctx = to_caam_rng_ctx(rng);
    flush_work(&ctx.worker);
    caam_jr_free(ctx.jrdev);
    kfifo_free(&ctx.fifo);
    }

#[no_mangle]
pub unsafe extern "C" fn test_len(rng: *mut hwrng, len: usize, wait: bool) {
    static inline void test_len(struct hwrng *rng, size_t len, bool wait)
    {
    u8 *buf;
    int read_len;
    struct caam_rng_ctx *ctx = to_caam_rng_ctx(rng);
    struct device *dev = ctx.ctrldev;
    buf = kcalloc(CAAM_RNG_MAX_FIFO_STORE_SIZE, sizeof(u8), GFP_KERNEL);
    if (!buf) {
    return;
    }
    while (len > 0) {
    read_len = rng.read(rng, buf, len, wait);
    if (read_len < 0 || (read_len == 0 && wait)) {
    dev_err(dev, "RNG Read FAILED received %d bytes\n",
    read_len);
    kfree(buf);
    return;
    }
    print_hex_dump_debug("random bytes@: ",
    DUMP_PREFIX_ADDRESS, 16, 4,
    buf, read_len, 1);
    len = len - read_len;
    }
    kfree(buf);
    }
#[no_mangle]
pub unsafe extern "C" fn test_mode_once(rng: *mut hwrng, wait: bool) {
    static inline void test_mode_once(struct hwrng *rng, bool wait)
    {
    test_len(rng, 32, wait);
    test_len(rng, 64, wait);
    test_len(rng, 128, wait);
    }
#[no_mangle]
unsafe extern "C" fn self_test(rng: *mut hwrng) {
    static void self_test(struct hwrng *rng)
    {
    pr_info("Executing RNG SELF-TEST with wait\n");
    test_mode_once(rng, true);
    }

#[no_mangle]
unsafe extern "C" fn caam_init(rng: *mut hwrng) -> c_int {
    static int caam_init(struct hwrng *rng)
    {
    struct caam_rng_ctx *ctx = to_caam_rng_ctx(rng);
    int err;
    ctx.desc_sync = devm_kzalloc(ctx.ctrldev, CAAM_RNG_DESC_LEN,
    GFP_KERNEL);
    if (!ctx.desc_sync)
    return -ENOMEM;
    ctx.desc_async = devm_kzalloc(ctx.ctrldev, CAAM_RNG_DESC_LEN,
    GFP_KERNEL);
    if (!ctx.desc_async)
    return -ENOMEM;
    if (kfifo_alloc(&ctx.fifo, ALIGN(CAAM_RNG_MAX_FIFO_STORE_SIZE,
    dma_get_cache_alignment()),
    GFP_KERNEL))
    return -ENOMEM;
    INIT_WORK(&ctx.worker, caam_rng_worker);
    ctx.jrdev = caam_jr_alloc();
    err = PTR_ERR_OR_ZERO(ctx.jrdev);
    if (err) {
    kfifo_free(&ctx.fifo);
    pr_err("Job Ring Device allocation for transform failed\n");
    return err;
    }
//
// Fill async buffer to have early randomness data for
// hw_random
//
    caam_rng_fill_async(ctx);
    return 0;
    }
    int caam_rng_init(struct device *ctrldev);
#[no_mangle]
pub unsafe extern "C" fn caam_rng_exit(ctrldev: *mut device) {
    void caam_rng_exit(struct device *ctrldev)
    {
    devres_release_group(ctrldev, caam_rng_init);
    }
#[no_mangle]
pub unsafe extern "C" fn caam_rng_init(ctrldev: *mut device) -> c_int {
    int caam_rng_init(struct device *ctrldev)
    {
    struct caam_rng_ctx *ctx;
    u32 rng_inst;
    struct caam_drv_private *priv = dev_get_drvdata(ctrldev);
    int ret;
// Check for an instantiated RNG before registration
    if (priv.era < 10)
    rng_inst = (rd_reg32(&priv.jr[0].perfmon.cha_num_ls) &
    CHA_ID_LS_RNG_MASK) >> CHA_ID_LS_RNG_SHIFT;
    else
    rng_inst = rd_reg32(&priv.jr[0].vreg.rng) & CHA_VER_NUM_MASK;
    if (!rng_inst)
    return 0;
    if (!devres_open_group(ctrldev, caam_rng_init, GFP_KERNEL))
    return -ENOMEM;
    ctx = devm_kzalloc(ctrldev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctx.ctrldev = ctrldev;
    ctx.rng.name    = "rng-caam";
    ctx.rng.init    = caam_init;
    ctx.rng.cleanup = caam_cleanup;
    ctx.rng.read    = caam_read;
    ctx.rng.priv    = (unsigned long)ctx;
    dev_info(ctrldev, "registering rng-caam\n");
    ret = devm_hwrng_register(ctrldev, &ctx.rng);
    if (ret) {
    caam_rng_exit(ctrldev);
    return ret;
    }

    self_test(&ctx.rng);

    devres_close_group(ctrldev, caam_rng_init);
    return 0;
    }
