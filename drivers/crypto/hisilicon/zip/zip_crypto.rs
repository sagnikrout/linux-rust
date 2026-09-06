//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/hisilicon/zip/zip_crypto.c
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
// Copyright (c) 2019 HiSilicon Limited.

// hisi_zip_sqe dw3

// hisi_zip_sqe dw7

// hisi_zip_sqe dw8

// hisi_zip_sqe dw9

pub const HZIP_ALG_TYPE_DEFLATE: c_uint = 0x01;
pub const HZIP_ALG_TYPE_LZ4: c_uint = 0x04;

pub const HZIP_SGL: c_uint = 0x1;

pub const HZIP_16K_WINSZ: c_uint = 0x2;
pub const HZIP_ALG_PRIORITY: c_int = 300;
pub const HZIP_SGL_SGE_NR: c_int = 10;

    static DEFINE_MUTEX(zip_algs_lock);
    static unsigned int zip_available_devs;
    enum hisi_zip_alg_type {
    HZIP_ALG_TYPE_COMP = 0,
    HZIP_ALG_TYPE_DECOMP = 1,
    };
    enum {
    HZIP_QPC_COMP,
    HZIP_QPC_DECOMP,
    HZIP_CTX_Q_NUM
    };

    (!strcmp((alg_name), "deflate") ? HZIP_ALG_TYPE_DEFLATE :	\
    (!strcmp((alg_name), "lz4") ? HZIP_ALG_TYPE_LZ4 : 0))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_zip_req {
    pub req: *mut acomp_req,
    pub hw_src: *mut hisi_acc_hw_sgl,
    pub hw_dst: *mut hisi_acc_hw_sgl,
    pub dma_src: dma_addr_t,
    pub dma_dst: dma_addr_t,
    pub qp_ctx: *mut hisi_zip_qp_ctx,
    pub req_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_zip_req_q {
    pub q: *mut hisi_zip_req,
    pub req_bitmap: *mut c_ulong,
    pub req_lock: spinlock_t,
    pub size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_zip_qp_ctx {
    pub qp: *mut hisi_qp,
    pub req_q: hisi_zip_req_q,
    pub sgl_pool: *mut hisi_acc_sgl_pool,
    pub zip_dev: *mut hisi_zip,
    pub ctx: *mut hisi_zip_ctx,
    pub req_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_zip_sqe_ops {
    pub sqe_type: u8,
    pub req): *mut *mut *mut void (fill_addr)(struct hisi_zip_sqe sqe, struct hisi_zip_req,
    pub req): *mut *mut *mut void (fill_buf_size)(struct hisi_zip_sqe sqe, struct hisi_zip_req,
    pub buf_type): *mut *mut *mut void (fill_buf_type)(struct hisi_zip_sqe sqe, u8,
    pub req_type): *mut *mut *mut void (fill_req_type)(struct hisi_zip_sqe sqe, u8,
    pub win_size): *mut *mut *mut void (fill_win_size)(struct hisi_zip_sqe sqe, u8,
    pub req): *mut *mut *mut void (fill_tag)(struct hisi_zip_sqe sqe, struct hisi_zip_req,
    pub sqe_type): *mut *mut *mut void (fill_sqe_type)(struct hisi_zip_sqe sqe, u8,
    pub sqe): *mut *mut u32 (get_status)(struct hisi_zip_sqe,
    pub sqe): *mut *mut u32 (get_dstlen)(struct hisi_zip_sqe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_zip_ctx {
    pub qp_ctx: [hisi_zip_qp_ctx; HZIP_CTX_Q_NUM],
    pub ops: *const hisi_zip_sqe_ops,
    pub fallback: bool,
}

#[no_mangle]
unsafe extern "C" fn sgl_sge_nr_set(val: *const c_char, kp: *const kernel_param) -> c_int {
    static int sgl_sge_nr_set(const char *val, const struct kernel_param *kp)
    {
    int ret;
    u16 n;
    if (!val)
    return -EINVAL;
    ret = kstrtou16(val, 10, &n);
    if (ret || n == 0 || n > HISI_ACC_SGL_SGE_NR_MAX)
    return -EINVAL;
    return param_set_ushort(val, kp);
    }
    static const struct kernel_param_ops sgl_sge_nr_ops = {
    .set = sgl_sge_nr_set,
    .get = param_get_ushort,
    };
    let mut sgl_sge_nr: static u16 = HZIP_SGL_SGE_NR;
    module_param_cb(sgl_sge_nr, &sgl_sge_nr_ops, &sgl_sge_nr, 0444);
    MODULE_PARM_DESC(sgl_sge_nr, "Number of sge in sgl(1-255)");
#[no_mangle]
unsafe extern "C" fn hisi_zip_fallback_do_work(acomp_req: *mut acomp_req, is_decompress: bool) -> c_int {
    static int hisi_zip_fallback_do_work(struct acomp_req *acomp_req, bool is_decompress)
    {
    ACOMP_FBREQ_ON_STACK(fbreq, acomp_req);
    int ret;
    if (!is_decompress)
    ret = crypto_acomp_compress(fbreq);
    else
    ret = crypto_acomp_decompress(fbreq);
    if (ret) {
    pr_err("failed to do fallback work, ret=%d\n", ret);
    return ret;
    }
    acomp_req.dlen = fbreq.dlen;
    return ret;
    }
    static struct hisi_zip_req *hisi_zip_create_req(struct hisi_zip_qp_ctx *qp_ctx,
    struct acomp_req *req)
    {
    struct hisi_zip_req_q *req_q = &qp_ctx.req_q;
    struct hisi_zip_req *q = req_q.q;
    struct hisi_zip_req *req_cache;
    int req_id;
    spin_lock(&req_q.req_lock);
    req_id = find_first_zero_bit(req_q.req_bitmap, req_q.size);
    if (req_id >= req_q.size) {
    spin_unlock(&req_q.req_lock);
    dev_dbg(&qp_ctx.qp.qm.pdev.dev, "req cache is full!\n");
    return ERR_PTR(-EAGAIN);
    }
    set_bit(req_id, req_q.req_bitmap);
    spin_unlock(&req_q.req_lock);
    req_cache = q + req_id;
    req_cache.req_id = req_id;
    req_cache.req = req;
    req_cache.qp_ctx = qp_ctx;
    return req_cache;
    }
    static void hisi_zip_remove_req(struct hisi_zip_qp_ctx *qp_ctx,
    struct hisi_zip_req *req)
    {
    struct hisi_zip_req_q *req_q = &qp_ctx.req_q;
    spin_lock(&req_q.req_lock);
    clear_bit(req.req_id, req_q.req_bitmap);
    spin_unlock(&req_q.req_lock);
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_fill_addr(sqe: *mut hisi_zip_sqe, req: *mut hisi_zip_req) {
    static void hisi_zip_fill_addr(struct hisi_zip_sqe *sqe, struct hisi_zip_req *req)
    {
    sqe.source_addr_l = lower_32_bits(req.dma_src);
    sqe.source_addr_h = upper_32_bits(req.dma_src);
    sqe.dest_addr_l = lower_32_bits(req.dma_dst);
    sqe.dest_addr_h = upper_32_bits(req.dma_dst);
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_fill_buf_size(sqe: *mut hisi_zip_sqe, req: *mut hisi_zip_req) {
    static void hisi_zip_fill_buf_size(struct hisi_zip_sqe *sqe, struct hisi_zip_req *req)
    {
    struct acomp_req *a_req = req.req;
    sqe.input_data_length = a_req.slen;
    sqe.dest_avail_out = a_req.dlen;
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_fill_buf_type(sqe: *mut hisi_zip_sqe, buf_type: u8) {
    static void hisi_zip_fill_buf_type(struct hisi_zip_sqe *sqe, u8 buf_type)
    {
    u32 val;
    val = sqe.dw9 & ~HZIP_BUF_TYPE_M;
    val |= FIELD_PREP(HZIP_BUF_TYPE_M, buf_type);
    sqe.dw9 = val;
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_fill_req_type(sqe: *mut hisi_zip_sqe, req_type: u8) {
    static void hisi_zip_fill_req_type(struct hisi_zip_sqe *sqe, u8 req_type)
    {
    u32 val;
    val = sqe.dw9 & ~HZIP_REQ_TYPE_M;
    val |= FIELD_PREP(HZIP_REQ_TYPE_M, req_type);
    sqe.dw9 = val;
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_fill_win_size(sqe: *mut hisi_zip_sqe, win_size: u8) {
    static void hisi_zip_fill_win_size(struct hisi_zip_sqe *sqe, u8 win_size)
    {
    u32 val;
    val = sqe.dw9 & ~HZIP_WIN_SIZE_M;
    val |= FIELD_PREP(HZIP_WIN_SIZE_M, win_size);
    sqe.dw9 = val;
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_fill_tag(sqe: *mut hisi_zip_sqe, req: *mut hisi_zip_req) {
    static void hisi_zip_fill_tag(struct hisi_zip_sqe *sqe, struct hisi_zip_req *req)
    {
    sqe.dw26 = lower_32_bits((u64)req);
    sqe.dw27 = upper_32_bits((u64)req);
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_fill_sqe_type(sqe: *mut hisi_zip_sqe, sqe_type: u8) {
    static void hisi_zip_fill_sqe_type(struct hisi_zip_sqe *sqe, u8 sqe_type)
    {
    u32 val;
    val = sqe.dw7 & ~HZIP_SQE_TYPE_M;
    val |= FIELD_PREP(HZIP_SQE_TYPE_M, sqe_type);
    sqe.dw7 = val;
    }
    static void hisi_zip_fill_sqe(struct hisi_zip_ctx *ctx, struct hisi_zip_sqe *sqe,
    u8 req_type, struct hisi_zip_req *req)
    {
    const struct hisi_zip_sqe_ops *ops = ctx.ops;
    memset(sqe, 0, sizeof(struct hisi_zip_sqe));
    ops.fill_addr(sqe, req);
    ops.fill_buf_size(sqe, req);
    ops.fill_buf_type(sqe, HZIP_SGL);
    ops.fill_req_type(sqe, req_type);
    ops.fill_win_size(sqe, HZIP_16K_WINSZ);
    ops.fill_tag(sqe, req);
    ops.fill_sqe_type(sqe, ops.sqe_type);
    }
    static int hisi_zip_do_work(struct hisi_zip_qp_ctx *qp_ctx,
    struct hisi_zip_req *req)
    {
    struct hisi_acc_sgl_pool *pool = qp_ctx.sgl_pool;
    struct hisi_zip_dfx *dfx = &qp_ctx.zip_dev.dfx;
    struct acomp_req *a_req = req.req;
    struct hisi_qp *qp = qp_ctx.qp;
    struct device *dev = &qp.qm.pdev.dev;
    struct hisi_zip_sqe zip_sqe;
    int ret;
    if (unlikely(!a_req.src || !a_req.slen || !a_req.dst || !a_req.dlen))
    return -EINVAL;
    req.hw_src = hisi_acc_sg_buf_map_to_hw_sgl(dev, a_req.src, pool,
    req.req_id << 1, &req.dma_src,
    DMA_TO_DEVICE);
    if (IS_ERR(req.hw_src)) {
    dev_err(dev, "failed to map the src buffer to hw sgl (%ld)!\n",
    PTR_ERR(req.hw_src));
    return PTR_ERR(req.hw_src);
    }
    req.hw_dst = hisi_acc_sg_buf_map_to_hw_sgl(dev, a_req.dst, pool,
    (req.req_id << 1) + 1,
    &req.dma_dst, DMA_FROM_DEVICE);
    if (IS_ERR(req.hw_dst)) {
    ret = PTR_ERR(req.hw_dst);
    dev_err(dev, "failed to map the dst buffer to hw sgl (%d)!\n",
    ret);
    goto err_unmap_input;
    }
    hisi_zip_fill_sqe(qp_ctx.ctx, &zip_sqe, qp_ctx.req_type, req);
// send command to start a task
    atomic64_inc(&dfx.send_cnt);
    ret = hisi_qp_send(qp, &zip_sqe);
    if (unlikely(ret < 0)) {
    atomic64_inc(&dfx.send_busy_cnt);
    ret = -EAGAIN;
    dev_dbg_ratelimited(dev, "failed to send request!\n");
    goto err_unmap_output;
    }
    return -EINPROGRESS;
    err_unmap_output:
    hisi_acc_sg_buf_unmap(dev, a_req.dst, req.hw_dst, DMA_FROM_DEVICE);
    err_unmap_input:
    hisi_acc_sg_buf_unmap(dev, a_req.src, req.hw_src, DMA_TO_DEVICE);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_get_status(sqe: *mut hisi_zip_sqe) -> u32 {
    static u32 hisi_zip_get_status(struct hisi_zip_sqe *sqe)
    {
    return sqe.dw3 & HZIP_BD_STATUS_M;
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_get_dstlen(sqe: *mut hisi_zip_sqe) -> u32 {
    static u32 hisi_zip_get_dstlen(struct hisi_zip_sqe *sqe)
    {
    return sqe.produced;
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_acomp_cb(qp: *mut hisi_qp, data: *mut c_void) {
    static void hisi_zip_acomp_cb(struct hisi_qp *qp, void *data)
    {
    struct hisi_zip_sqe *sqe = data;
    struct hisi_zip_req *req = (struct hisi_zip_req *)GET_REQ_FROM_SQE(sqe);
    struct hisi_zip_qp_ctx *qp_ctx = req.qp_ctx;
    const struct hisi_zip_sqe_ops *ops = qp_ctx.ctx.ops;
    struct hisi_zip_dfx *dfx = &qp_ctx.zip_dev.dfx;
    struct device *dev = &qp.qm.pdev.dev;
    struct acomp_req *acomp_req = req.req;
    let mut err: c_int = 0;
    u32 status;
    atomic64_inc(&dfx.recv_cnt);
    status = ops.get_status(sqe);
    if (unlikely(status != 0 && status != HZIP_NC_ERR)) {
    dev_err(dev, "%scompress fail in qp%u: %u, output: %u\n",
    (qp.alg_type == 0) ? "" : "de", qp.qp_id, status,
    sqe.produced);
    atomic64_inc(&dfx.err_bd_cnt);
    err = -EIO;
    }
    hisi_acc_sg_buf_unmap(dev, acomp_req.dst, req.hw_dst, DMA_FROM_DEVICE);
    hisi_acc_sg_buf_unmap(dev, acomp_req.src, req.hw_src, DMA_TO_DEVICE);
    acomp_req.dlen = ops.get_dstlen(sqe);
    if (acomp_req.base.complete)
    acomp_request_complete(acomp_req, err);
    hisi_zip_remove_req(qp_ctx, req);
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_acompress(acomp_req: *mut acomp_req) -> c_int {
    static int hisi_zip_acompress(struct acomp_req *acomp_req)
    {
    struct hisi_zip_ctx *ctx = crypto_tfm_ctx(acomp_req.base.tfm);
    struct hisi_zip_qp_ctx *qp_ctx = &ctx.qp_ctx[HZIP_QPC_COMP];
    struct hisi_zip_req *req;
    struct device *dev;
    int ret;
    if (ctx.fallback)
    return hisi_zip_fallback_do_work(acomp_req, 0);
    dev = &qp_ctx.qp.qm.pdev.dev;
    req = hisi_zip_create_req(qp_ctx, acomp_req);
    if (IS_ERR(req))
    return PTR_ERR(req);
    ret = hisi_zip_do_work(qp_ctx, req);
    if (unlikely(ret != -EINPROGRESS)) {
    dev_info_ratelimited(dev, "failed to do compress (%d)!\n", ret);
    hisi_zip_remove_req(qp_ctx, req);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_adecompress(acomp_req: *mut acomp_req) -> c_int {
    static int hisi_zip_adecompress(struct acomp_req *acomp_req)
    {
    struct hisi_zip_ctx *ctx = crypto_tfm_ctx(acomp_req.base.tfm);
    struct hisi_zip_qp_ctx *qp_ctx = &ctx.qp_ctx[HZIP_QPC_DECOMP];
    struct hisi_zip_req *req;
    struct device *dev;
    int ret;
    if (ctx.fallback)
    return hisi_zip_fallback_do_work(acomp_req, 1);
    dev = &qp_ctx.qp.qm.pdev.dev;
    req = hisi_zip_create_req(qp_ctx, acomp_req);
    if (IS_ERR(req))
    return PTR_ERR(req);
    ret = hisi_zip_do_work(qp_ctx, req);
    if (unlikely(ret != -EINPROGRESS)) {
    dev_info_ratelimited(dev, "failed to do decompress (%d)!\n",
    ret);
    hisi_zip_remove_req(qp_ctx, req);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_decompress(acomp_req: *mut acomp_req) -> c_int {
    static int hisi_zip_decompress(struct acomp_req *acomp_req)
    {
    return hisi_zip_fallback_do_work(acomp_req, 1);
    }
    static const struct hisi_zip_sqe_ops hisi_zip_ops = {
    .sqe_type		= 0x3,
    .fill_addr		= hisi_zip_fill_addr,
    .fill_buf_size		= hisi_zip_fill_buf_size,
    .fill_buf_type		= hisi_zip_fill_buf_type,
    .fill_req_type		= hisi_zip_fill_req_type,
    .fill_win_size		= hisi_zip_fill_win_size,
    .fill_tag		= hisi_zip_fill_tag,
    .fill_sqe_type		= hisi_zip_fill_sqe_type,
    .get_status		= hisi_zip_get_status,
    .get_dstlen		= hisi_zip_get_dstlen,
    };
#[no_mangle]
unsafe extern "C" fn hisi_zip_ctx_init(hisi_zip_ctx: *mut hisi_zip_ctx, req_type: u8, node: c_int) -> c_int {
    static int hisi_zip_ctx_init(struct hisi_zip_ctx *hisi_zip_ctx, u8 req_type, int node)
    {
    struct hisi_qp *qps[HZIP_CTX_Q_NUM] = { core::ptr::null_mut() };
    struct hisi_zip_qp_ctx *qp_ctx;
    u8 alg_type[HZIP_CTX_Q_NUM];
    struct hisi_zip *hisi_zip;
    int ret, i;
// alg_type = 0 for compress, 1 for decompress in hw sqe
    for (i = 0; i < HZIP_CTX_Q_NUM; i++)
    alg_type[i] = i;
    ret = zip_create_qps(qps, HZIP_CTX_Q_NUM, node, alg_type);
    if (ret) {
    pr_err("failed to create zip qps (%d)!\n", ret);
    return -ENODEV;
    }
    hisi_zip = container_of(qps[0].qm, struct hisi_zip, qm);
    for (i = 0; i < HZIP_CTX_Q_NUM; i++) {
    qp_ctx = &hisi_zip_ctx.qp_ctx[i];
    qp_ctx.ctx = hisi_zip_ctx;
    qp_ctx.zip_dev = hisi_zip;
    qp_ctx.req_type = req_type;
    qp_ctx.qp = qps[i];
    }
    hisi_zip_ctx.ops = &hisi_zip_ops;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_ctx_exit(hisi_zip_ctx: *mut hisi_zip_ctx) {
    static void hisi_zip_ctx_exit(struct hisi_zip_ctx *hisi_zip_ctx)
    {
    struct hisi_qp *qps[HZIP_CTX_Q_NUM] = { core::ptr::null_mut() };
    int i;
    for (i = 0; i < HZIP_CTX_Q_NUM; i++)
    qps[i] = hisi_zip_ctx.qp_ctx[i].qp;
    hisi_qm_free_qps(qps, HZIP_CTX_Q_NUM);
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_create_req_q(ctx: *mut hisi_zip_ctx) -> c_int {
    static int hisi_zip_create_req_q(struct hisi_zip_ctx *ctx)
    {
    let mut q_depth: u16 = ctx.qp_ctx[0].qp.sq_depth;
    struct hisi_zip_req_q *req_q;
    int i, ret;
    for (i = 0; i < HZIP_CTX_Q_NUM; i++) {
    req_q = &ctx.qp_ctx[i].req_q;
    req_q.size = q_depth;
    req_q.req_bitmap = bitmap_zalloc(req_q.size, GFP_KERNEL);
    if (!req_q.req_bitmap) {
    ret = -ENOMEM;
    if (i == 0)
    return ret;
    goto err_free_comp_q;
    }
    spin_lock_init(&req_q.req_lock);
    req_q.q = kzalloc_objs(struct hisi_zip_req, req_q.size);
    if (!req_q.q) {
    ret = -ENOMEM;
    if (i == 0)
    goto err_free_comp_bitmap;
    else
    goto err_free_decomp_bitmap;
    }
    }
    return 0;
    err_free_decomp_bitmap:
    bitmap_free(ctx.qp_ctx[HZIP_QPC_DECOMP].req_q.req_bitmap);
    err_free_comp_q:
    kfree(ctx.qp_ctx[HZIP_QPC_COMP].req_q.q);
    err_free_comp_bitmap:
    bitmap_free(ctx.qp_ctx[HZIP_QPC_COMP].req_q.req_bitmap);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_release_req_q(ctx: *mut hisi_zip_ctx) {
    static void hisi_zip_release_req_q(struct hisi_zip_ctx *ctx)
    {
    int i;
    for (i = 0; i < HZIP_CTX_Q_NUM; i++) {
    kfree(ctx.qp_ctx[i].req_q.q);
    bitmap_free(ctx.qp_ctx[i].req_q.req_bitmap);
    }
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_create_sgl_pool(ctx: *mut hisi_zip_ctx) -> c_int {
    static int hisi_zip_create_sgl_pool(struct hisi_zip_ctx *ctx)
    {
    let mut q_depth: u16 = ctx.qp_ctx[0].qp.sq_depth;
    struct hisi_zip_qp_ctx *tmp;
    struct device *dev;
    int i;
    for (i = 0; i < HZIP_CTX_Q_NUM; i++) {
    tmp = &ctx.qp_ctx[i];
    dev = &tmp.qp.qm.pdev.dev;
    tmp.sgl_pool = hisi_acc_create_sgl_pool(dev, q_depth << 1,
    sgl_sge_nr);
    if (IS_ERR(tmp.sgl_pool)) {
    if (i == 1)
    goto err_free_sgl_pool0;
    return -ENOMEM;
    }
    }
    return 0;
    err_free_sgl_pool0:
    hisi_acc_free_sgl_pool(&ctx.qp_ctx[HZIP_QPC_COMP].qp.qm.pdev.dev,
    ctx.qp_ctx[HZIP_QPC_COMP].sgl_pool);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_release_sgl_pool(ctx: *mut hisi_zip_ctx) {
    static void hisi_zip_release_sgl_pool(struct hisi_zip_ctx *ctx)
    {
    int i;
    for (i = 0; i < HZIP_CTX_Q_NUM; i++)
    hisi_acc_free_sgl_pool(&ctx.qp_ctx[i].qp.qm.pdev.dev,
    ctx.qp_ctx[i].sgl_pool);
    }
    static void hisi_zip_set_acomp_cb(struct hisi_zip_ctx *ctx,
    void (*fn)(struct hisi_qp *, void *))
    {
    int i;
    for (i = 0; i < HZIP_CTX_Q_NUM; i++)
    ctx.qp_ctx[i].qp.req_cb = fn;
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_acomp_init(tfm: *mut crypto_acomp) -> c_int {
    static int hisi_zip_acomp_init(struct crypto_acomp *tfm)
    {
    const char *alg_name = crypto_tfm_alg_name(&tfm.base);
    struct hisi_zip_ctx *ctx = crypto_tfm_ctx(&tfm.base);
    struct device *dev;
    int ret;
    ret = hisi_zip_ctx_init(ctx, COMP_NAME_TO_TYPE(alg_name), tfm.base.node);
    if (ret) {
    pr_err("failed to init ctx (%d)!\n", ret);
    goto switch_to_soft;
    }
    dev = &ctx.qp_ctx[0].qp.qm.pdev.dev;
    ret = hisi_zip_create_req_q(ctx);
    if (ret) {
    dev_err(dev, "failed to create request queue (%d)!\n", ret);
    goto err_ctx_exit;
    }
    ret = hisi_zip_create_sgl_pool(ctx);
    if (ret) {
    dev_err(dev, "failed to create sgl pool (%d)!\n", ret);
    goto err_release_req_q;
    }
    hisi_zip_set_acomp_cb(ctx, hisi_zip_acomp_cb);
    return 0;
    err_release_req_q:
    hisi_zip_release_req_q(ctx);
    err_ctx_exit:
    hisi_zip_ctx_exit(ctx);
    switch_to_soft:
    ctx.fallback = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_acomp_exit(tfm: *mut crypto_acomp) {
    static void hisi_zip_acomp_exit(struct crypto_acomp *tfm)
    {
    struct hisi_zip_ctx *ctx = crypto_tfm_ctx(&tfm.base);
    if (ctx.fallback)
    return;
    hisi_zip_release_sgl_pool(ctx);
    hisi_zip_release_req_q(ctx);
    hisi_zip_ctx_exit(ctx);
    }
    static struct acomp_alg hisi_zip_acomp_deflate = {
    .init			= hisi_zip_acomp_init,
    .exit			= hisi_zip_acomp_exit,
    .compress		= hisi_zip_acompress,
    .decompress		= hisi_zip_adecompress,
    .base			= {
    .cra_name		= "deflate",
    .cra_driver_name	= "hisi-deflate-acomp",
    .cra_flags		= CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_module		= THIS_MODULE,
    .cra_priority		= HZIP_ALG_PRIORITY,
    .cra_ctxsize		= sizeof(struct hisi_zip_ctx),
    }
    };
#[no_mangle]
unsafe extern "C" fn hisi_zip_register_deflate(qm: *mut hisi_qm) -> c_int {
    static int hisi_zip_register_deflate(struct hisi_qm *qm)
    {
    int ret;
    if (!hisi_zip_alg_support(qm, HZIP_ALG_DEFLATE))
    return 0;
    ret = crypto_register_acomp(&hisi_zip_acomp_deflate);
    if (ret)
    dev_err(&qm.pdev.dev, "failed to register to deflate (%d)!\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_unregister_deflate(qm: *mut hisi_qm) {
    static void hisi_zip_unregister_deflate(struct hisi_qm *qm)
    {
    if (!hisi_zip_alg_support(qm, HZIP_ALG_DEFLATE))
    return;
    crypto_unregister_acomp(&hisi_zip_acomp_deflate);
    }
    static struct acomp_alg hisi_zip_acomp_lz4 = {
    .init			= hisi_zip_acomp_init,
    .exit			= hisi_zip_acomp_exit,
    .compress		= hisi_zip_acompress,
    .decompress		= hisi_zip_decompress,
    .base			= {
    .cra_name		= "lz4",
    .cra_driver_name	= "hisi-lz4-acomp",
    .cra_flags		= CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_module		= THIS_MODULE,
    .cra_priority		= HZIP_ALG_PRIORITY,
    .cra_ctxsize		= sizeof(struct hisi_zip_ctx),
    }
    };
#[no_mangle]
unsafe extern "C" fn hisi_zip_register_lz4(qm: *mut hisi_qm) -> c_int {
    static int hisi_zip_register_lz4(struct hisi_qm *qm)
    {
    int ret;
    if (!hisi_zip_alg_support(qm, HZIP_ALG_LZ4))
    return 0;
    ret = crypto_register_acomp(&hisi_zip_acomp_lz4);
    if (ret)
    dev_err(&qm.pdev.dev, "failed to register to LZ4 (%d)!\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hisi_zip_unregister_lz4(qm: *mut hisi_qm) {
    static void hisi_zip_unregister_lz4(struct hisi_qm *qm)
    {
    if (!hisi_zip_alg_support(qm, HZIP_ALG_LZ4))
    return;
    crypto_unregister_acomp(&hisi_zip_acomp_lz4);
    }
#[no_mangle]
pub unsafe extern "C" fn hisi_zip_register_to_crypto(qm: *mut hisi_qm) -> c_int {
    int hisi_zip_register_to_crypto(struct hisi_qm *qm)
    {
    let mut ret: c_int = 0;
    mutex_lock(&zip_algs_lock);
    if (zip_available_devs) {
    zip_available_devs++;
    goto unlock;
    }
    ret = hisi_zip_register_deflate(qm);
    if (ret)
    goto unlock;
    ret = hisi_zip_register_lz4(qm);
    if (ret)
    goto unreg_deflate;
    zip_available_devs++;
    mutex_unlock(&zip_algs_lock);
    return 0;
    unreg_deflate:
    hisi_zip_unregister_deflate(qm);
    unlock:
    mutex_unlock(&zip_algs_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hisi_zip_unregister_from_crypto(qm: *mut hisi_qm) {
    void hisi_zip_unregister_from_crypto(struct hisi_qm *qm)
    {
    mutex_lock(&zip_algs_lock);
    if (--zip_available_devs)
    goto unlock;
    hisi_zip_unregister_deflate(qm);
    hisi_zip_unregister_lz4(qm);
    unlock:
    mutex_unlock(&zip_algs_lock);
    }
