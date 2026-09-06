//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/hisilicon/hpre/hpre_crypto.c
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

    struct hpre_ctx;
pub const HPRE_CRYPTO_ALG_PRI: c_int = 1000;
pub const HPRE_ALIGN_SZ: c_int = 64;
pub const HPRE_BITS_2_BYTES_SHIFT: c_int = 3;
pub const HPRE_RSA_512BITS_KSZ: c_int = 64;
pub const HPRE_RSA_1536BITS_KSZ: c_int = 192;
pub const HPRE_CRT_PRMS: c_int = 5;
pub const HPRE_CRT_Q: c_int = 2;
pub const HPRE_CRT_P: c_int = 3;
pub const HPRE_CRT_INV: c_int = 4;
pub const HPRE_DH_G_FLAG: c_uint = 0x02;
pub const HPRE_TRY_SEND_TIMES: c_int = 100;

pub const HPRE_SQE_ALG_BITS: c_int = 5;
pub const HPRE_SQE_DONE_SHIFT: c_int = 30;
pub const HPRE_DH_MAX_P_SZ: c_int = 512;
pub const HPRE_DFX_SEC_TO_US: c_int = 1000000;
pub const HPRE_DFX_US_TO_NS: c_int = 1000;
pub const HPRE_ENABLE_HPCORE_SHIFT: c_int = 7;
// due to nist p521
pub const HPRE_ECC_MAX_KSZ: c_int = 66;
// size in bytes of the n prime
pub const HPRE_ECC_NIST_P192_N_SIZE: c_int = 24;
pub const HPRE_ECC_NIST_P256_N_SIZE: c_int = 32;
pub const HPRE_ECC_NIST_P384_N_SIZE: c_int = 48;
// size in bytes
pub const HPRE_ECC_HW256_KSZ_B: c_int = 32;
pub const HPRE_ECC_HW384_KSZ_B: c_int = 48;
// capability register mask of driver

    static DEFINE_MUTEX(hpre_algs_lock);
    static unsigned int hpre_available_devs;
    typedef void (*hpre_cb)(struct hpre_ctx *ctx, void *sqe);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpre_rsa_ctx {
// low address: e--->n
    pub pubkey: *mut c_char,
    pub dma_pubkey: dma_addr_t,
// low address: d--->n
    pub prikey: *mut c_char,
    pub dma_prikey: dma_addr_t,
// low address: dq->dp->q->p->qinv
    pub crt_prikey: *mut c_char,
    pub dma_crt_prikey: dma_addr_t,
    pub soft_tfm: *mut crypto_akcipher,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpre_dh_ctx {
//
// If base is g we compute the public key
// ya = g^xa mod p; [RFC2631 sec 2.1.1]
// else if base if the counterpart public key we
// compute the shared secret
// ZZ = yb^xa mod p; [RFC2631 sec 2.1.1]
// low address: d--->n, please refer to Hisilicon HPRE UM
//
    pub xa_p: *mut c_char,
    pub dma_xa_p: dma_addr_t,
    pub /: *mut *mut *mut char g; / m,
    pub dma_g: dma_addr_t,
    pub soft_tfm: *mut crypto_kpp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpre_ecdh_ctx {
// low address: p->a->k->b
    pub p: *mut c_uchar,
    pub dma_p: dma_addr_t,
// low address: x->y
    pub g: *mut c_uchar,
    pub dma_g: dma_addr_t,
    pub soft_tfm: *mut crypto_kpp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpre_ctx {
    pub qp: *mut hisi_qp,
    pub dev: *mut device,
    pub hpre: *mut hpre,
    pub key_sz: c_uint,
    pub crt_g2_mode: bool,
    union {
    pub rsa: hpre_rsa_ctx,
    pub dh: hpre_dh_ctx,
    pub ecdh: hpre_ecdh_ctx,
}

// for ecc algorithms
    unsigned int curve_id;
// for high performance core
    u8 enable_hpcore;
    bool fallback;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpre_asym_request {
    pub src: *mut c_char,
    pub dst: *mut c_char,
    pub req: hpre_sqe,
    pub ctx: *mut hpre_ctx,
    union {
    pub rsa: *mut akcipher_request,
    pub dh: *mut kpp_request,
    pub ecdh: *mut kpp_request,
    pub areq: },
    pub err: c_int,
    pub cb: hpre_cb,
    pub req_time: timespec64,
}

#[no_mangle]
pub unsafe extern "C" fn hpre_align_sz() -> c_uint {
    static inline unsigned int hpre_align_sz(void)
    {
    return ((crypto_dma_align() - 1) | (HPRE_ALIGN_SZ - 1)) + 1;
    }
#[no_mangle]
pub unsafe extern "C" fn hpre_align_pd() -> c_uint {
    static inline unsigned int hpre_align_pd(void)
    {
    return (hpre_align_sz() - 1) & ~(crypto_tfm_ctx_alignment() - 1);
    }
#[no_mangle]
unsafe extern "C" fn hpre_dfx_add_req_time(hpre_req: *mut hpre_asym_request) {
    static void hpre_dfx_add_req_time(struct hpre_asym_request *hpre_req)
    {
    struct hpre_ctx *ctx = hpre_req.ctx;
    struct hpre_dfx *dfx = ctx.hpre.debug.dfx;
    if (atomic64_read(&dfx[HPRE_OVERTIME_THRHLD].value))
    ktime_get_ts64(&hpre_req.req_time);
    }
    static int hpre_get_data_dma_addr(struct hpre_asym_request *hpre_req,
    struct scatterlist *data, unsigned int len,
    int is_src, dma_addr_t *tmp)
    {
    struct device *dev = hpre_req.ctx.dev;
    enum dma_data_direction dma_dir;
    if (is_src) {
    hpre_req.src = core::ptr::null_mut();
    dma_dir = DMA_TO_DEVICE;
    } else {
    hpre_req.dst = core::ptr::null_mut();
    dma_dir = DMA_FROM_DEVICE;
    }
// tmp = dma_map_single(dev, sg_virt(data), len, dma_dir);
    if (unlikely(dma_mapping_error(dev, *tmp))) {
    dev_err(dev, "dma map data err!\n");
    return -ENOMEM;
    }
    return 0;
    }
    static int hpre_prepare_dma_buf(struct hpre_asym_request *hpre_req,
    struct scatterlist *data, unsigned int len,
    int is_src, dma_addr_t *tmp)
    {
    struct hpre_ctx *ctx = hpre_req.ctx;
    struct device *dev = ctx.dev;
    void *ptr;
    int shift;
    shift = ctx.key_sz - len;
    if (unlikely(shift < 0))
    return -EINVAL;
    ptr = dma_alloc_coherent(dev, ctx.key_sz, tmp, GFP_ATOMIC);
    if (unlikely(!ptr))
    return -ENOMEM;
    if (is_src) {
    scatterwalk_map_and_copy(ptr + shift, data, 0, len, 0);
    hpre_req.src = ptr;
    } else {
    hpre_req.dst = ptr;
    }
    return 0;
    }
    static int hpre_hw_data_init(struct hpre_asym_request *hpre_req,
    struct scatterlist *data, unsigned int len,
    int is_src, int is_dh)
    {
    struct hpre_sqe *msg = &hpre_req.req;
    struct hpre_ctx *ctx = hpre_req.ctx;
    let mut tmp: dma_addr_t = 0;
    int ret;
// when the data is dh's source, we should format it
    if ((sg_is_last(data) && len == ctx.key_sz) &&
    ((is_dh && !is_src) || !is_dh))
    ret = hpre_get_data_dma_addr(hpre_req, data, len, is_src, &tmp);
    else
    ret = hpre_prepare_dma_buf(hpre_req, data, len, is_src, &tmp);
    if (unlikely(ret))
    return ret;
    if (is_src)
    msg.in = cpu_to_le64(tmp);
    else
    msg.out = cpu_to_le64(tmp);
    return 0;
    }
    static void hpre_hw_data_clr_all(struct hpre_ctx *ctx,
    struct hpre_asym_request *req,
    struct scatterlist *dst,
    struct scatterlist *src)
    {
    struct device *dev = ctx.dev;
    struct hpre_sqe *sqe = &req.req;
    dma_addr_t tmp;
    tmp = le64_to_cpu(sqe.in);
    if (unlikely(dma_mapping_error(dev, tmp)))
    return;
    if (src) {
    if (req.src)
    dma_free_coherent(dev, ctx.key_sz, req.src, tmp);
    else
    dma_unmap_single(dev, tmp, ctx.key_sz, DMA_TO_DEVICE);
    }
    tmp = le64_to_cpu(sqe.out);
    if (unlikely(dma_mapping_error(dev, tmp)))
    return;
    if (req.dst) {
    if (dst)
    scatterwalk_map_and_copy(req.dst, dst, 0,
    ctx.key_sz, 1);
    dma_free_coherent(dev, ctx.key_sz, req.dst, tmp);
    } else {
    dma_unmap_single(dev, tmp, ctx.key_sz, DMA_FROM_DEVICE);
    }
    }
    static int hpre_alg_res_post_hf(struct hpre_ctx *ctx, struct hpre_sqe *sqe,
    void **kreq)
    {
    unsigned int err, done, alg;
pub const HPRE_NO_HW_ERR: c_int = 0;
pub const HPRE_HW_TASK_DONE: c_int = 3;

// kreq = (void *)le64_to_cpu(sqe->tag);
    err = (le32_to_cpu(sqe.dw0) >> HPRE_SQE_ALG_BITS) &
    HREE_HW_ERR_MASK;
    done = (le32_to_cpu(sqe.dw0) >> HPRE_SQE_DONE_SHIFT) &
    HREE_SQE_DONE_MASK;
    if (likely(err == HPRE_NO_HW_ERR && done == HPRE_HW_TASK_DONE))
    return 0;
    alg = le32_to_cpu(sqe.dw0) & HREE_ALG_TYPE_MASK;
    dev_err_ratelimited(ctx.dev, "alg[0x%x] error: done[0x%x], etype[0x%x]\n",
    alg, done, err);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn hpre_ctx_clear(ctx: *mut hpre_ctx, is_clear_all: bool) {
    static void hpre_ctx_clear(struct hpre_ctx *ctx, bool is_clear_all)
    {
    if (is_clear_all)
    hisi_qm_free_qps(&ctx.qp, 1);
    ctx.crt_g2_mode = false;
    ctx.key_sz = 0;
    }
    static bool hpre_is_bd_timeout(struct hpre_asym_request *req,
    u64 overtime_thrhld)
    {
    struct timespec64 reply_time;
    u64 time_use_us;
    ktime_get_ts64(&reply_time);
    time_use_us = (reply_time.tv_sec - req.req_time.tv_sec) *
    HPRE_DFX_SEC_TO_US +
    (reply_time.tv_nsec - req.req_time.tv_nsec) /
    HPRE_DFX_US_TO_NS;
    if (time_use_us <= overtime_thrhld)
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn hpre_dh_cb(ctx: *mut hpre_ctx, resp: *mut c_void) {
    static void hpre_dh_cb(struct hpre_ctx *ctx, void *resp)
    {
    struct hpre_dfx *dfx = ctx.hpre.debug.dfx;
    struct hpre_asym_request *req;
    struct kpp_request *areq;
    u64 overtime_thrhld;
    int ret;
    ret = hpre_alg_res_post_hf(ctx, resp, (void **)&req);
    areq = req.areq.dh;
    areq.dst_len = ctx.key_sz;
    overtime_thrhld = atomic64_read(&dfx[HPRE_OVERTIME_THRHLD].value);
    if (overtime_thrhld && hpre_is_bd_timeout(req, overtime_thrhld))
    atomic64_inc(&dfx[HPRE_OVER_THRHLD_CNT].value);
    hpre_hw_data_clr_all(ctx, req, areq.dst, areq.src);
    kpp_request_complete(areq, ret);
    atomic64_inc(&dfx[HPRE_RECV_CNT].value);
    }
#[no_mangle]
unsafe extern "C" fn hpre_rsa_cb(ctx: *mut hpre_ctx, resp: *mut c_void) {
    static void hpre_rsa_cb(struct hpre_ctx *ctx, void *resp)
    {
    struct hpre_dfx *dfx = ctx.hpre.debug.dfx;
    struct hpre_asym_request *req;
    struct akcipher_request *areq;
    u64 overtime_thrhld;
    int ret;
    ret = hpre_alg_res_post_hf(ctx, resp, (void **)&req);
    overtime_thrhld = atomic64_read(&dfx[HPRE_OVERTIME_THRHLD].value);
    if (overtime_thrhld && hpre_is_bd_timeout(req, overtime_thrhld))
    atomic64_inc(&dfx[HPRE_OVER_THRHLD_CNT].value);
    areq = req.areq.rsa;
    areq.dst_len = ctx.key_sz;
    hpre_hw_data_clr_all(ctx, req, areq.dst, areq.src);
    akcipher_request_complete(areq, ret);
    atomic64_inc(&dfx[HPRE_RECV_CNT].value);
    }
#[no_mangle]
unsafe extern "C" fn hpre_alg_cb(qp: *mut hisi_qp, resp: *mut c_void) {
    static void hpre_alg_cb(struct hisi_qp *qp, void *resp)
    {
    struct hpre_asym_request *h_req;
    struct hpre_sqe *sqe = resp;
    h_req = (struct hpre_asym_request *)le64_to_cpu(sqe.tag);
    if (unlikely(!h_req)) {
    pr_err("Failed to get request, and qp_id is %u\n", qp.qp_id);
    return;
    }
    h_req.cb(h_req.ctx, resp);
    }
#[no_mangle]
unsafe extern "C" fn hpre_ctx_init(ctx: *mut hpre_ctx, type: u8) -> c_int {
    static int hpre_ctx_init(struct hpre_ctx *ctx, u8 type)
    {
    struct hisi_qp *qp;
    struct hpre *hpre;
    qp = hpre_create_qp(type);
    if (!qp) {
    ctx.qp = core::ptr::null_mut();
    return -ENODEV;
    }
    qp.req_cb = hpre_alg_cb;
    ctx.qp = qp;
    ctx.dev = &qp.qm.pdev.dev;
    hpre = container_of(ctx.qp.qm, struct hpre, qm);
    ctx.hpre = hpre;
    ctx.key_sz = 0;
    ctx.crt_g2_mode = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpre_msg_request_set(ctx: *mut hpre_ctx, req: *mut c_void, is_rsa: bool) -> c_int {
    static int hpre_msg_request_set(struct hpre_ctx *ctx, void *req, bool is_rsa)
    {
    struct hpre_asym_request *h_req;
    struct hpre_sqe *msg;
    void *tmp;
    if (is_rsa) {
    struct akcipher_request *akreq = req;
    if (akreq.dst_len < ctx.key_sz) {
    akreq.dst_len = ctx.key_sz;
    return -EOVERFLOW;
    }
    tmp = akcipher_request_ctx(akreq);
    h_req = PTR_ALIGN(tmp, hpre_align_sz());
    h_req.cb = hpre_rsa_cb;
    h_req.areq.rsa = akreq;
    msg = &h_req.req;
    memset(msg, 0, sizeof(*msg));
    } else {
    struct kpp_request *kreq = req;
    if (kreq.dst_len < ctx.key_sz) {
    kreq.dst_len = ctx.key_sz;
    return -EOVERFLOW;
    }
    tmp = kpp_request_ctx(kreq);
    h_req = PTR_ALIGN(tmp, hpre_align_sz());
    h_req.cb = hpre_dh_cb;
    h_req.areq.dh = kreq;
    msg = &h_req.req;
    memset(msg, 0, sizeof(*msg));
    msg.key = cpu_to_le64(ctx.dh.dma_xa_p);
    }
    msg.in = cpu_to_le64(DMA_MAPPING_ERROR);
    msg.out = cpu_to_le64(DMA_MAPPING_ERROR);
    msg.dw0 |= cpu_to_le32(0x1 << HPRE_SQE_DONE_SHIFT);
    msg.task_len1 = (ctx.key_sz >> HPRE_BITS_2_BYTES_SHIFT) - 1;
    h_req.ctx = ctx;
    hpre_dfx_add_req_time(h_req);
    msg.tag = cpu_to_le64((uintptr_t)h_req);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpre_send(ctx: *mut hpre_ctx, msg: *mut hpre_sqe) -> c_int {
    static int hpre_send(struct hpre_ctx *ctx, struct hpre_sqe *msg)
    {
    struct hpre_dfx *dfx = ctx.hpre.debug.dfx;
    let mut ctr: c_int = 0;
    int ret;
    do {
    atomic64_inc(&dfx[HPRE_SEND_CNT].value);
    ret = hisi_qp_send(ctx.qp, msg);
    if (ret != -EBUSY)
    break;
    atomic64_inc(&dfx[HPRE_SEND_BUSY_CNT].value);
    } while (ctr++ < HPRE_TRY_SEND_TIMES);
    if (likely(!ret))
    return ret;
    if (ret != -EBUSY)
    atomic64_inc(&dfx[HPRE_SEND_FAIL_CNT].value);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hpre_dh_compute_value(req: *mut kpp_request) -> c_int {
    static int hpre_dh_compute_value(struct kpp_request *req)
    {
    struct crypto_kpp *tfm = crypto_kpp_reqtfm(req);
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    void *tmp = kpp_request_ctx(req);
    struct hpre_asym_request *hpre_req = PTR_ALIGN(tmp, hpre_align_sz());
    struct hpre_sqe *msg = &hpre_req.req;
    int ret;
    ret = hpre_msg_request_set(ctx, req, false);
    if (unlikely(ret))
    return ret;
    if (req.src) {
    ret = hpre_hw_data_init(hpre_req, req.src, req.src_len, 1, 1);
    if (unlikely(ret))
    goto clear_all;
    } else {
    msg.in = cpu_to_le64(ctx.dh.dma_g);
    }
    ret = hpre_hw_data_init(hpre_req, req.dst, req.dst_len, 0, 1);
    if (unlikely(ret))
    goto clear_all;
    if (ctx.crt_g2_mode && !req.src)
    msg.dw0 = cpu_to_le32(le32_to_cpu(msg.dw0) | HPRE_ALG_DH_G2);
    else
    msg.dw0 = cpu_to_le32(le32_to_cpu(msg.dw0) | HPRE_ALG_DH);
// success
    ret = hpre_send(ctx, msg);
    if (likely(!ret))
    return -EINPROGRESS;
    clear_all:
    hpre_hw_data_clr_all(ctx, hpre_req, req.dst, req.src);
    return ret;
    }
    static struct kpp_request *hpre_dh_prepare_fb_req(struct kpp_request *req)
    {
    struct kpp_request *fb_req = kpp_request_ctx(req);
    struct crypto_kpp *tfm = crypto_kpp_reqtfm(req);
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    kpp_request_set_tfm(fb_req, ctx.dh.soft_tfm);
    kpp_request_set_callback(fb_req, req.base.flags, req.base.complete, req.base.data);
    kpp_request_set_input(fb_req, req.src, req.src_len);
    kpp_request_set_output(fb_req, req.dst, req.dst_len);
    return fb_req;
    }
#[no_mangle]
unsafe extern "C" fn hpre_dh_generate_public_key(req: *mut kpp_request) -> c_int {
    static int hpre_dh_generate_public_key(struct kpp_request *req)
    {
    struct crypto_kpp *tfm = crypto_kpp_reqtfm(req);
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    struct kpp_request *fb_req;
    if (ctx.fallback) {
    fb_req = hpre_dh_prepare_fb_req(req);
    return crypto_kpp_generate_public_key(fb_req);
    }
    return hpre_dh_compute_value(req);
    }
#[no_mangle]
unsafe extern "C" fn hpre_dh_compute_shared_secret(req: *mut kpp_request) -> c_int {
    static int hpre_dh_compute_shared_secret(struct kpp_request *req)
    {
    struct crypto_kpp *tfm = crypto_kpp_reqtfm(req);
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    struct kpp_request *fb_req;
    if (ctx.fallback) {
    fb_req = hpre_dh_prepare_fb_req(req);
    return crypto_kpp_compute_shared_secret(fb_req);
    }
    return hpre_dh_compute_value(req);
    }
#[no_mangle]
unsafe extern "C" fn hpre_is_dh_params_length_valid(key_sz: c_uint) -> c_int {
    static int hpre_is_dh_params_length_valid(unsigned int key_sz)
    {
pub const _HPRE_DH_GRP1: c_int = 768;
pub const _HPRE_DH_GRP2: c_int = 1024;
pub const _HPRE_DH_GRP5: c_int = 1536;
pub const _HPRE_DH_GRP14: c_int = 2048;
pub const _HPRE_DH_GRP15: c_int = 3072;
pub const _HPRE_DH_GRP16: c_int = 4096;
    switch (key_sz) {
    case _HPRE_DH_GRP1:
    case _HPRE_DH_GRP2:
    case _HPRE_DH_GRP5:
    case _HPRE_DH_GRP14:
    case _HPRE_DH_GRP15:
    case _HPRE_DH_GRP16:
    return 0;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn hpre_dh_set_params(ctx: *mut hpre_ctx, params: *mut dh) -> c_int {
    static int hpre_dh_set_params(struct hpre_ctx *ctx, struct dh *params)
    {
    struct device *dev = ctx.dev;
    unsigned int sz;
    sz = ctx.key_sz = params.p_size;
    ctx.dh.xa_p = dma_alloc_coherent(dev, sz << 1,
    &ctx.dh.dma_xa_p, GFP_KERNEL);
    if (!ctx.dh.xa_p)
    return -ENOMEM;
    memcpy(ctx.dh.xa_p + sz, params.p, sz);
// If g equals 2 don't copy it
    if (params.g_size == 1 && *(char *)params.g == HPRE_DH_G_FLAG) {
    ctx.crt_g2_mode = true;
    return 0;
    }
    ctx.dh.g = dma_alloc_coherent(dev, sz, &ctx.dh.dma_g, GFP_KERNEL);
    if (!ctx.dh.g) {
    dma_free_coherent(dev, sz << 1, ctx.dh.xa_p,
    ctx.dh.dma_xa_p);
    ctx.dh.xa_p = core::ptr::null_mut();
    return -ENOMEM;
    }
    memcpy(ctx.dh.g + (sz - params.g_size), params.g, params.g_size);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpre_dh_clear_ctx(ctx: *mut hpre_ctx, is_clear_all: bool) {
    static void hpre_dh_clear_ctx(struct hpre_ctx *ctx, bool is_clear_all)
    {
    struct device *dev = ctx.dev;
    let mut sz: c_uint = ctx.key_sz;
    if (!ctx.qp)
    return;
    if (ctx.dh.g) {
    dma_free_coherent(dev, sz, ctx.dh.g, ctx.dh.dma_g);
    ctx.dh.g = core::ptr::null_mut();
    }
    if (ctx.dh.xa_p) {
    memzero_explicit(ctx.dh.xa_p, sz);
    dma_free_coherent(dev, sz << 1, ctx.dh.xa_p,
    ctx.dh.dma_xa_p);
    ctx.dh.xa_p = core::ptr::null_mut();
    }
    hpre_ctx_clear(ctx, is_clear_all);
    }
    static int hpre_dh_set_secret(struct crypto_kpp *tfm, const void *buf,
    unsigned int len)
    {
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    struct dh params;
    int ret;
    if (crypto_dh_decode_key(buf, len, &params) < 0)
    return -EINVAL;
    if (!ctx.qp)
    goto set_soft_secret;
    if (hpre_is_dh_params_length_valid(params.p_size <<
    HPRE_BITS_2_BYTES_SHIFT))
    goto set_soft_secret;
// Free old secret if any
    hpre_dh_clear_ctx(ctx, false);
    ret = hpre_dh_set_params(ctx, &params);
    if (ret < 0)
    goto err_clear_ctx;
    memcpy(ctx.dh.xa_p + (ctx.key_sz - params.key_size), params.key,
    params.key_size);
    ctx.fallback = false;
    return 0;
    err_clear_ctx:
    hpre_dh_clear_ctx(ctx, false);
    return ret;
    set_soft_secret:
    ctx.fallback = true;
    return crypto_kpp_set_secret(ctx.dh.soft_tfm, buf, len);
    }
#[no_mangle]
unsafe extern "C" fn hpre_dh_max_size(tfm: *mut crypto_kpp) -> c_uint {
    static unsigned int hpre_dh_max_size(struct crypto_kpp *tfm)
    {
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    if (ctx.fallback)
    return crypto_kpp_maxsize(ctx.dh.soft_tfm);
    return ctx.key_sz;
    }
#[no_mangle]
unsafe extern "C" fn hpre_dh_init_tfm(tfm: *mut crypto_kpp) -> c_int {
    static int hpre_dh_init_tfm(struct crypto_kpp *tfm)
    {
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    const char *alg = kpp_alg_name(tfm);
    unsigned int reqsize;
    int ret;
    ctx.dh.soft_tfm = crypto_alloc_kpp(alg, 0, CRYPTO_ALG_NEED_FALLBACK);
    if (IS_ERR(ctx.dh.soft_tfm)) {
    pr_err("Failed to alloc dh tfm!\n");
    return PTR_ERR(ctx.dh.soft_tfm);
    }
    crypto_kpp_set_flags(ctx.dh.soft_tfm, crypto_kpp_get_flags(tfm));
    reqsize = max(sizeof(struct hpre_asym_request) + hpre_align_pd(),
    sizeof(struct kpp_request) + crypto_kpp_reqsize(ctx.dh.soft_tfm));
    kpp_set_reqsize(tfm, reqsize);
    ret = hpre_ctx_init(ctx, HPRE_V2_ALG_TYPE);
    if (ret && ret != -ENODEV) {
    crypto_free_kpp(ctx.dh.soft_tfm);
    return ret;
    } else if (ret == -ENODEV) {
    ctx.fallback = true;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpre_dh_exit_tfm(tfm: *mut crypto_kpp) {
    static void hpre_dh_exit_tfm(struct crypto_kpp *tfm)
    {
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    hpre_dh_clear_ctx(ctx, true);
    crypto_free_kpp(ctx.dh.soft_tfm);
    }
#[no_mangle]
unsafe extern "C" fn hpre_rsa_drop_leading_zeros(ptr: *const c_char, len: *mut usize) {
    static void hpre_rsa_drop_leading_zeros(const char **ptr, size_t *len)
    {
    while (!**ptr && *len) {
    (*ptr)++;
    (*len)--;
    }
    }
#[no_mangle]
unsafe extern "C" fn hpre_rsa_key_size_is_support(len: c_uint) -> bool {
    static bool hpre_rsa_key_size_is_support(unsigned int len)
    {
    let mut bits: c_uint = len << HPRE_BITS_2_BYTES_SHIFT;
pub const _RSA_1024BITS_KEY_WDTH: c_int = 1024;
pub const _RSA_2048BITS_KEY_WDTH: c_int = 2048;
pub const _RSA_3072BITS_KEY_WDTH: c_int = 3072;
pub const _RSA_4096BITS_KEY_WDTH: c_int = 4096;
    switch (bits) {
    case _RSA_1024BITS_KEY_WDTH:
    case _RSA_2048BITS_KEY_WDTH:
    case _RSA_3072BITS_KEY_WDTH:
    case _RSA_4096BITS_KEY_WDTH:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn hpre_rsa_enc(req: *mut akcipher_request) -> c_int {
    static int hpre_rsa_enc(struct akcipher_request *req)
    {
    struct crypto_akcipher *tfm = crypto_akcipher_reqtfm(req);
    struct hpre_ctx *ctx = akcipher_tfm_ctx(tfm);
    void *tmp = akcipher_request_ctx(req);
    struct hpre_asym_request *hpre_req = PTR_ALIGN(tmp, hpre_align_sz());
    struct hpre_sqe *msg = &hpre_req.req;
    int ret;
// For unsupported key size and unavailable devices, use soft tfm instead
    if (ctx.fallback) {
    akcipher_request_set_tfm(req, ctx.rsa.soft_tfm);
    ret = crypto_akcipher_encrypt(req);
    akcipher_request_set_tfm(req, tfm);
    return ret;
    }
    if (unlikely(!ctx.rsa.pubkey))
    return -EINVAL;
    ret = hpre_msg_request_set(ctx, req, true);
    if (unlikely(ret))
    return ret;
    msg.dw0 |= cpu_to_le32(HPRE_ALG_NC_NCRT);
    msg.key = cpu_to_le64(ctx.rsa.dma_pubkey);
    ret = hpre_hw_data_init(hpre_req, req.src, req.src_len, 1, 0);
    if (unlikely(ret))
    goto clear_all;
    ret = hpre_hw_data_init(hpre_req, req.dst, req.dst_len, 0, 0);
    if (unlikely(ret))
    goto clear_all;
// success
    ret = hpre_send(ctx, msg);
    if (likely(!ret))
    return -EINPROGRESS;
    clear_all:
    hpre_hw_data_clr_all(ctx, hpre_req, req.dst, req.src);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hpre_rsa_dec(req: *mut akcipher_request) -> c_int {
    static int hpre_rsa_dec(struct akcipher_request *req)
    {
    struct crypto_akcipher *tfm = crypto_akcipher_reqtfm(req);
    struct hpre_ctx *ctx = akcipher_tfm_ctx(tfm);
    void *tmp = akcipher_request_ctx(req);
    struct hpre_asym_request *hpre_req = PTR_ALIGN(tmp, hpre_align_sz());
    struct hpre_sqe *msg = &hpre_req.req;
    int ret;
// For unsupported key size and unavailable devices, use soft tfm instead
    if (ctx.fallback) {
    akcipher_request_set_tfm(req, ctx.rsa.soft_tfm);
    ret = crypto_akcipher_decrypt(req);
    akcipher_request_set_tfm(req, tfm);
    return ret;
    }
    if (unlikely(!ctx.rsa.prikey))
    return -EINVAL;
    ret = hpre_msg_request_set(ctx, req, true);
    if (unlikely(ret))
    return ret;
    if (ctx.crt_g2_mode) {
    msg.key = cpu_to_le64(ctx.rsa.dma_crt_prikey);
    msg.dw0 = cpu_to_le32(le32_to_cpu(msg.dw0) |
    HPRE_ALG_NC_CRT);
    } else {
    msg.key = cpu_to_le64(ctx.rsa.dma_prikey);
    msg.dw0 = cpu_to_le32(le32_to_cpu(msg.dw0) |
    HPRE_ALG_NC_NCRT);
    }
    ret = hpre_hw_data_init(hpre_req, req.src, req.src_len, 1, 0);
    if (unlikely(ret))
    goto clear_all;
    ret = hpre_hw_data_init(hpre_req, req.dst, req.dst_len, 0, 0);
    if (unlikely(ret))
    goto clear_all;
// success
    ret = hpre_send(ctx, msg);
    if (likely(!ret))
    return -EINPROGRESS;
    clear_all:
    hpre_hw_data_clr_all(ctx, hpre_req, req.dst, req.src);
    return ret;
    }
    static int hpre_rsa_set_n(struct hpre_ctx *ctx, const char *value,
    size_t vlen, bool private)
    {
    const char *ptr = value;
    hpre_rsa_drop_leading_zeros(&ptr, &vlen);
    ctx.key_sz = vlen;
// if invalid key size provided, we use software tfm
    if (!hpre_rsa_key_size_is_support(ctx.key_sz)) {
    ctx.fallback = true;
    return 0;
    }
    ctx.rsa.pubkey = dma_alloc_coherent(ctx.dev, vlen << 1,
    &ctx.rsa.dma_pubkey,
    GFP_KERNEL);
    if (!ctx.rsa.pubkey)
    return -ENOMEM;
    if (private) {
    ctx.rsa.prikey = dma_alloc_coherent(ctx.dev, vlen << 1,
    &ctx.rsa.dma_prikey,
    GFP_KERNEL);
    if (!ctx.rsa.prikey) {
    dma_free_coherent(ctx.dev, vlen << 1,
    ctx.rsa.pubkey,
    ctx.rsa.dma_pubkey);
    ctx.rsa.pubkey = core::ptr::null_mut();
    return -ENOMEM;
    }
    memcpy(ctx.rsa.prikey + vlen, ptr, vlen);
    }
    memcpy(ctx.rsa.pubkey + vlen, ptr, vlen);
// Using hardware HPRE to do RSA
    return 1;
    }
    static int hpre_rsa_set_e(struct hpre_ctx *ctx, const char *value,
    size_t vlen)
    {
    const char *ptr = value;
    hpre_rsa_drop_leading_zeros(&ptr, &vlen);
    if (!ctx.key_sz || !vlen || vlen > ctx.key_sz)
    return -EINVAL;
    memcpy(ctx.rsa.pubkey + ctx.key_sz - vlen, ptr, vlen);
    return 0;
    }
    static int hpre_rsa_set_d(struct hpre_ctx *ctx, const char *value,
    size_t vlen)
    {
    const char *ptr = value;
    hpre_rsa_drop_leading_zeros(&ptr, &vlen);
    if (!ctx.key_sz || !vlen || vlen > ctx.key_sz)
    return -EINVAL;
    memcpy(ctx.rsa.prikey + ctx.key_sz - vlen, ptr, vlen);
    return 0;
    }
    static int hpre_crt_para_get(char *para, size_t para_sz,
    const char *raw, size_t raw_sz)
    {
    const char *ptr = raw;
    let mut len: usize = raw_sz;
    hpre_rsa_drop_leading_zeros(&ptr, &len);
    if (!len || len > para_sz)
    return -EINVAL;
    memcpy(para + para_sz - len, ptr, len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpre_rsa_setkey_crt(ctx: *mut hpre_ctx, rsa_key: *mut rsa_key) -> c_int {
    static int hpre_rsa_setkey_crt(struct hpre_ctx *ctx, struct rsa_key *rsa_key)
    {
    let mut hlf_ksz: c_uint = ctx.key_sz >> 1;
    struct device *dev = ctx.dev;
    u64 offset;
    int ret;
    ctx.rsa.crt_prikey = dma_alloc_coherent(dev, hlf_ksz * HPRE_CRT_PRMS,
    &ctx.rsa.dma_crt_prikey,
    GFP_KERNEL);
    if (!ctx.rsa.crt_prikey)
    return -ENOMEM;
    ret = hpre_crt_para_get(ctx.rsa.crt_prikey, hlf_ksz,
    rsa_key.dq, rsa_key.dq_sz);
    if (ret)
    goto free_key;
    offset = hlf_ksz;
    ret = hpre_crt_para_get(ctx.rsa.crt_prikey + offset, hlf_ksz,
    rsa_key.dp, rsa_key.dp_sz);
    if (ret)
    goto free_key;
    offset = hlf_ksz * HPRE_CRT_Q;
    ret = hpre_crt_para_get(ctx.rsa.crt_prikey + offset, hlf_ksz,
    rsa_key.q, rsa_key.q_sz);
    if (ret)
    goto free_key;
    offset = hlf_ksz * HPRE_CRT_P;
    ret = hpre_crt_para_get(ctx.rsa.crt_prikey + offset, hlf_ksz,
    rsa_key.p, rsa_key.p_sz);
    if (ret)
    goto free_key;
    offset = hlf_ksz * HPRE_CRT_INV;
    ret = hpre_crt_para_get(ctx.rsa.crt_prikey + offset, hlf_ksz,
    rsa_key.qinv, rsa_key.qinv_sz);
    if (ret)
    goto free_key;
    ctx.crt_g2_mode = true;
    return 0;
    free_key:
    offset = hlf_ksz * HPRE_CRT_PRMS;
    memzero_explicit(ctx.rsa.crt_prikey, offset);
    dma_free_coherent(dev, hlf_ksz * HPRE_CRT_PRMS, ctx.rsa.crt_prikey,
    ctx.rsa.dma_crt_prikey);
    ctx.rsa.crt_prikey = core::ptr::null_mut();
    ctx.crt_g2_mode = false;
    return ret;
    }
// If it is clear all, all the resources of the QP will be cleaned.
#[no_mangle]
unsafe extern "C" fn hpre_rsa_clear_ctx(ctx: *mut hpre_ctx, is_clear_all: bool) {
    static void hpre_rsa_clear_ctx(struct hpre_ctx *ctx, bool is_clear_all)
    {
    let mut half_key_sz: c_uint = ctx.key_sz >> 1;
    struct device *dev = ctx.dev;
    if (!ctx.qp)
    return;
    if (ctx.rsa.pubkey) {
    dma_free_coherent(dev, ctx.key_sz << 1,
    ctx.rsa.pubkey, ctx.rsa.dma_pubkey);
    ctx.rsa.pubkey = core::ptr::null_mut();
    }
    if (ctx.rsa.crt_prikey) {
    memzero_explicit(ctx.rsa.crt_prikey,
    half_key_sz * HPRE_CRT_PRMS);
    dma_free_coherent(dev, half_key_sz * HPRE_CRT_PRMS,
    ctx.rsa.crt_prikey, ctx.rsa.dma_crt_prikey);
    ctx.rsa.crt_prikey = core::ptr::null_mut();
    }
    if (ctx.rsa.prikey) {
    memzero_explicit(ctx.rsa.prikey, ctx.key_sz);
    dma_free_coherent(dev, ctx.key_sz << 1, ctx.rsa.prikey,
    ctx.rsa.dma_prikey);
    ctx.rsa.prikey = core::ptr::null_mut();
    }
    hpre_ctx_clear(ctx, is_clear_all);
    }
//
// we should judge if it is CRT or not,
// CRT: return true,  N-CRT: return false .
//
#[no_mangle]
unsafe extern "C" fn hpre_is_crt_key(key: *mut rsa_key) -> bool {
    static bool hpre_is_crt_key(struct rsa_key *key)
    {
    u16 len = key.p_sz + key.q_sz + key.dp_sz + key.dq_sz +
    key.qinv_sz;
pub const LEN_OF_NCRT_PARA: c_int = 5;
// N-CRT less than 5 parameters
    return len > LEN_OF_NCRT_PARA;
    }
    static int hpre_rsa_setkey(struct hpre_ctx *ctx, const void *key,
    unsigned int keylen, bool private)
    {
    struct rsa_key rsa_key;
    int ret;
    hpre_rsa_clear_ctx(ctx, false);
    if (private)
    ret = rsa_parse_priv_key(&rsa_key, key, keylen);
    else
    ret = rsa_parse_pub_key(&rsa_key, key, keylen);
    if (ret < 0)
    return ret;
    ret = hpre_rsa_set_n(ctx, rsa_key.n, rsa_key.n_sz, private);
    if (ret <= 0)
    return ret;
    if (private) {
    ret = hpre_rsa_set_d(ctx, rsa_key.d, rsa_key.d_sz);
    if (ret < 0)
    goto free;
    if (hpre_is_crt_key(&rsa_key)) {
    ret = hpre_rsa_setkey_crt(ctx, &rsa_key);
    if (ret < 0)
    goto free;
    }
    }
    ret = hpre_rsa_set_e(ctx, rsa_key.e, rsa_key.e_sz);
    if (ret < 0)
    goto free;
    if ((private && !ctx.rsa.prikey) || !ctx.rsa.pubkey) {
    ret = -EINVAL;
    goto free;
    }
    ctx.fallback = false;
    return 0;
    free:
    hpre_rsa_clear_ctx(ctx, false);
    return ret;
    }
    static int hpre_rsa_setpubkey(struct crypto_akcipher *tfm, const void *key,
    unsigned int keylen)
    {
    struct hpre_ctx *ctx = akcipher_tfm_ctx(tfm);
    int ret;
    ret = crypto_akcipher_set_pub_key(ctx.rsa.soft_tfm, key, keylen);
    if (ret)
    return ret;
    if (!ctx.qp)
    return 0;
    return hpre_rsa_setkey(ctx, key, keylen, false);
    }
    static int hpre_rsa_setprivkey(struct crypto_akcipher *tfm, const void *key,
    unsigned int keylen)
    {
    struct hpre_ctx *ctx = akcipher_tfm_ctx(tfm);
    int ret;
    ret = crypto_akcipher_set_priv_key(ctx.rsa.soft_tfm, key, keylen);
    if (ret)
    return ret;
    if (!ctx.qp)
    return 0;
    return hpre_rsa_setkey(ctx, key, keylen, true);
    }
#[no_mangle]
unsafe extern "C" fn hpre_rsa_max_size(tfm: *mut crypto_akcipher) -> c_uint {
    static unsigned int hpre_rsa_max_size(struct crypto_akcipher *tfm)
    {
    struct hpre_ctx *ctx = akcipher_tfm_ctx(tfm);
// For unsupported key size and unavailable devices, use soft tfm instead
    if (ctx.fallback)
    return crypto_akcipher_maxsize(ctx.rsa.soft_tfm);
    return ctx.key_sz;
    }
#[no_mangle]
unsafe extern "C" fn hpre_rsa_init_tfm(tfm: *mut crypto_akcipher) -> c_int {
    static int hpre_rsa_init_tfm(struct crypto_akcipher *tfm)
    {
    struct hpre_ctx *ctx = akcipher_tfm_ctx(tfm);
    int ret;
    ctx.rsa.soft_tfm = crypto_alloc_akcipher("rsa-generic", 0, 0);
    if (IS_ERR(ctx.rsa.soft_tfm)) {
    pr_err("Can not alloc_akcipher!\n");
    return PTR_ERR(ctx.rsa.soft_tfm);
    }
    akcipher_set_reqsize(tfm, sizeof(struct hpre_asym_request) +
    hpre_align_pd());
    ret = hpre_ctx_init(ctx, HPRE_V2_ALG_TYPE);
    if (ret && ret != -ENODEV) {
    crypto_free_akcipher(ctx.rsa.soft_tfm);
    return ret;
    } else if (ret == -ENODEV) {
    ctx.fallback = true;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpre_rsa_exit_tfm(tfm: *mut crypto_akcipher) {
    static void hpre_rsa_exit_tfm(struct crypto_akcipher *tfm)
    {
    struct hpre_ctx *ctx = akcipher_tfm_ctx(tfm);
    hpre_rsa_clear_ctx(ctx, true);
    crypto_free_akcipher(ctx.rsa.soft_tfm);
    }
#[no_mangle]
unsafe extern "C" fn hpre_key_to_big_end(data: *mut u8, len: c_int) {
    static void hpre_key_to_big_end(u8 *data, int len)
    {
    int i, j;
    for (i = 0; i < len / 2; i++) {
    j = len - i - 1;
    swap(data[j], data[i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn hpre_ecc_clear_ctx(ctx: *mut hpre_ctx, is_clear_all: bool) {
    static void hpre_ecc_clear_ctx(struct hpre_ctx *ctx, bool is_clear_all)
    {
    struct device *dev = ctx.dev;
    let mut sz: c_uint = ctx.key_sz;
    let mut shift: c_uint = sz << 1;
    if (ctx.ecdh.p) {
// ecdh: p->a->k->b
    memzero_explicit(ctx.ecdh.p + shift, sz);
    dma_free_coherent(dev, sz << 3, ctx.ecdh.p, ctx.ecdh.dma_p);
    ctx.ecdh.p = core::ptr::null_mut();
    }
    hpre_ctx_clear(ctx, is_clear_all);
    }
//
// The bits of 192/224/256/384/521 are supported by HPRE,
// and convert the bits like:
// bits<=256, bits=256; 256<bits<=384, bits=384; 384<bits<=576, bits=576;
// If the parameter bit width is insufficient, then we fill in the
// high-order zeros by soft, so TASK_LENGTH1 is 0x3/0x5/0x8;
//
#[no_mangle]
unsafe extern "C" fn hpre_ecdh_supported_curve(id: c_ushort) -> c_uint {
    static unsigned int hpre_ecdh_supported_curve(unsigned short id)
    {
    switch (id) {
    case ECC_CURVE_NIST_P192:
    case ECC_CURVE_NIST_P256:
    return HPRE_ECC_HW256_KSZ_B;
    case ECC_CURVE_NIST_P384:
    return HPRE_ECC_HW384_KSZ_B;
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fill_curve_param(addr: *mut c_void, param: *mut u64, cur_sz: c_uint, ndigits: u8) {
    static void fill_curve_param(void *addr, u64 *param, unsigned int cur_sz, u8 ndigits)
    {
    let mut sz: c_uint = cur_sz - (ndigits - 1) * sizeof(u64);
    let mut i: u8 = 0;
    while (i < ndigits - 1) {
    memcpy(addr + sizeof(u64) * i, &param[i], sizeof(u64));
    i++;
    }
    memcpy(addr + sizeof(u64) * i, &param[ndigits - 1], sz);
    hpre_key_to_big_end((u8 *)addr, cur_sz);
    }
    static int hpre_ecdh_fill_curve(struct hpre_ctx *ctx, struct ecdh *params,
    unsigned int cur_sz)
    {
    let mut shifta: c_uint = ctx.key_sz << 1;
    let mut shiftb: c_uint = ctx.key_sz << 2;
    void *p = ctx.ecdh.p + ctx.key_sz - cur_sz;
    void *a = ctx.ecdh.p + shifta - cur_sz;
    void *b = ctx.ecdh.p + shiftb - cur_sz;
    void *x = ctx.ecdh.g + ctx.key_sz - cur_sz;
    void *y = ctx.ecdh.g + shifta - cur_sz;
    const struct ecc_curve *curve = ecc_get_curve(ctx.curve_id);
    char *n;
    if (unlikely(!curve))
    return -EINVAL;
    n = kzalloc(ctx.key_sz, GFP_KERNEL);
    if (!n)
    return -ENOMEM;
    fill_curve_param(p, curve.p, cur_sz, curve.g.ndigits);
    fill_curve_param(a, curve.a, cur_sz, curve.g.ndigits);
    fill_curve_param(b, curve.b, cur_sz, curve.g.ndigits);
    fill_curve_param(x, curve.g.x, cur_sz, curve.g.ndigits);
    fill_curve_param(y, curve.g.y, cur_sz, curve.g.ndigits);
    fill_curve_param(n, curve.n, cur_sz, curve.g.ndigits);
    if (params.key_size == cur_sz && memcmp(params.key, n, cur_sz) >= 0) {
    kfree(n);
    return -EINVAL;
    }
    kfree(n);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpre_ecdh_get_curvesz(id: c_ushort) -> c_uint {
    static unsigned int hpre_ecdh_get_curvesz(unsigned short id)
    {
    switch (id) {
    case ECC_CURVE_NIST_P192:
    return HPRE_ECC_NIST_P192_N_SIZE;
    case ECC_CURVE_NIST_P256:
    return HPRE_ECC_NIST_P256_N_SIZE;
    case ECC_CURVE_NIST_P384:
    return HPRE_ECC_NIST_P384_N_SIZE;
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpre_ecdh_set_param(ctx: *mut hpre_ctx, params: *mut ecdh) -> c_int {
    static int hpre_ecdh_set_param(struct hpre_ctx *ctx, struct ecdh *params)
    {
    struct device *dev = ctx.dev;
    unsigned int sz, shift, curve_sz;
    int ret;
    ctx.key_sz = hpre_ecdh_supported_curve(ctx.curve_id);
    if (!ctx.key_sz)
    return -EINVAL;
    curve_sz = hpre_ecdh_get_curvesz(ctx.curve_id);
    if (!curve_sz || params.key_size > curve_sz)
    return -EINVAL;
    sz = ctx.key_sz;
    if (!ctx.ecdh.p) {
    ctx.ecdh.p = dma_alloc_coherent(dev, sz << 3, &ctx.ecdh.dma_p,
    GFP_KERNEL);
    if (!ctx.ecdh.p)
    return -ENOMEM;
    }
    shift = sz << 2;
    ctx.ecdh.g = ctx.ecdh.p + shift;
    ctx.ecdh.dma_g = ctx.ecdh.dma_p + shift;
    ret = hpre_ecdh_fill_curve(ctx, params, curve_sz);
    if (ret) {
    dev_err(dev, "failed to fill curve_param, ret = %d!\n", ret);
    dma_free_coherent(dev, sz << 3, ctx.ecdh.p, ctx.ecdh.dma_p);
    ctx.ecdh.p = core::ptr::null_mut();
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpre_key_is_zero(key: *const c_char, key_sz: c_ushort) -> bool {
    static bool hpre_key_is_zero(const char *key, unsigned short key_sz)
    {
    int i;
    for (i = 0; i < key_sz; i++)
    if (key[i])
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ecdh_gen_privkey(ctx: *mut hpre_ctx, params: *mut ecdh) -> c_int {
    static int ecdh_gen_privkey(struct hpre_ctx *ctx, struct ecdh *params)
    {
    struct device *dev = ctx.dev;
    int ret;
    ret = crypto_stdrng_get_bytes(params.key, params.key_size);
    if (ret)
    dev_err(dev, "failed to get random bytes, ret = %d!\n", ret);
    return ret;
    }
    static int hpre_ecdh_set_secret(struct crypto_kpp *tfm, const void *buf,
    unsigned int len)
    {
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    unsigned int sz, sz_shift, curve_sz;
    struct device *dev = ctx.dev;
    char key[HPRE_ECC_MAX_KSZ];
    struct ecdh params;
    int ret;
    if (ctx.fallback)
    return crypto_kpp_set_secret(ctx.ecdh.soft_tfm, buf, len);
    if (crypto_ecdh_decode_key(buf, len, &params) < 0) {
    dev_err(dev, "failed to decode ecdh key!\n");
    return -EINVAL;
    }
// Use stdrng to generate private key
    if (!params.key || !params.key_size) {
    params.key = key;
    curve_sz = hpre_ecdh_get_curvesz(ctx.curve_id);
    if (!curve_sz) {
    dev_err(dev, "Invalid curve size!\n");
    return -EINVAL;
    }
    params.key_size = curve_sz - 1;
    ret = ecdh_gen_privkey(ctx, &params);
    if (ret)
    return ret;
    }
    if (hpre_key_is_zero(params.key, params.key_size)) {
    dev_err(dev, "Invalid hpre key!\n");
    return -EINVAL;
    }
    hpre_ecc_clear_ctx(ctx, false);
    ret = hpre_ecdh_set_param(ctx, &params);
    if (ret < 0) {
    dev_err(dev, "failed to set hpre param, ret = %d!\n", ret);
    return ret;
    }
    sz = ctx.key_sz;
    sz_shift = (sz << 1) + sz - params.key_size;
    memcpy(ctx.ecdh.p + sz_shift, params.key, params.key_size);
    return 0;
    }
    static void hpre_ecdh_hw_data_clr_all(struct hpre_ctx *ctx,
    struct hpre_asym_request *req,
    struct scatterlist *dst,
    struct scatterlist *src)
    {
    struct device *dev = ctx.dev;
    struct hpre_sqe *sqe = &req.req;
    dma_addr_t dma;
    dma = le64_to_cpu(sqe.in);
    if (unlikely(dma_mapping_error(dev, dma)))
    return;
    if (src && req.src)
    dma_free_coherent(dev, ctx.key_sz << 2, req.src, dma);
    dma = le64_to_cpu(sqe.out);
    if (unlikely(dma_mapping_error(dev, dma)))
    return;
    if (req.dst)
    dma_free_coherent(dev, ctx.key_sz << 1, req.dst, dma);
    if (dst)
    dma_unmap_single(dev, dma, ctx.key_sz << 1, DMA_FROM_DEVICE);
    }
#[no_mangle]
unsafe extern "C" fn hpre_ecdh_cb(ctx: *mut hpre_ctx, resp: *mut c_void) {
    static void hpre_ecdh_cb(struct hpre_ctx *ctx, void *resp)
    {
    let mut curve_sz: c_uint = hpre_ecdh_get_curvesz(ctx.curve_id);
    struct hpre_dfx *dfx = ctx.hpre.debug.dfx;
    struct hpre_asym_request *req = core::ptr::null_mut();
    struct kpp_request *areq;
    u64 overtime_thrhld;
    char *p;
    int ret;
    ret = hpre_alg_res_post_hf(ctx, resp, (void **)&req);
    areq = req.areq.ecdh;
    areq.dst_len = ctx.key_sz << 1;
    overtime_thrhld = atomic64_read(&dfx[HPRE_OVERTIME_THRHLD].value);
    if (overtime_thrhld && hpre_is_bd_timeout(req, overtime_thrhld))
    atomic64_inc(&dfx[HPRE_OVER_THRHLD_CNT].value);
// Do unmap before data processing
    hpre_ecdh_hw_data_clr_all(ctx, req, areq.dst, areq.src);
    p = sg_virt(areq.dst);
    memmove(p, p + ctx.key_sz - curve_sz, curve_sz);
    memmove(p + curve_sz, p + areq.dst_len - curve_sz, curve_sz);
    kpp_request_complete(areq, ret);
    atomic64_inc(&dfx[HPRE_RECV_CNT].value);
    }
    static int hpre_ecdh_msg_request_set(struct hpre_ctx *ctx,
    struct kpp_request *req)
    {
    struct hpre_asym_request *h_req;
    struct hpre_sqe *msg;
    void *tmp;
    if (req.dst_len < ctx.key_sz << 1) {
    req.dst_len = ctx.key_sz << 1;
    return -EINVAL;
    }
    tmp = kpp_request_ctx(req);
    h_req = PTR_ALIGN(tmp, hpre_align_sz());
    h_req.cb = hpre_ecdh_cb;
    h_req.areq.ecdh = req;
    msg = &h_req.req;
    memset(msg, 0, sizeof(*msg));
    msg.in = cpu_to_le64(DMA_MAPPING_ERROR);
    msg.out = cpu_to_le64(DMA_MAPPING_ERROR);
    msg.key = cpu_to_le64(ctx.ecdh.dma_p);
    msg.dw0 |= cpu_to_le32(0x1U << HPRE_SQE_DONE_SHIFT);
    msg.task_len1 = (ctx.key_sz >> HPRE_BITS_2_BYTES_SHIFT) - 1;
    h_req.ctx = ctx;
    hpre_dfx_add_req_time(h_req);
    msg.tag = cpu_to_le64((uintptr_t)h_req);
    return 0;
    }
    static int hpre_ecdh_src_data_init(struct hpre_asym_request *hpre_req,
    struct scatterlist *data, unsigned int len)
    {
    struct hpre_sqe *msg = &hpre_req.req;
    struct hpre_ctx *ctx = hpre_req.ctx;
    struct device *dev = ctx.dev;
    unsigned int tmpshift;
    let mut dma: dma_addr_t = 0;
    void *ptr;
    int shift;
// Src_data include gx and gy.
    shift = ctx.key_sz - (len >> 1);
    if (unlikely(shift < 0))
    return -EINVAL;
    ptr = dma_alloc_coherent(dev, ctx.key_sz << 2, &dma, GFP_KERNEL);
    if (unlikely(!ptr))
    return -ENOMEM;
    tmpshift = ctx.key_sz << 1;
    scatterwalk_map_and_copy(ptr + tmpshift, data, 0, len, 0);
    memcpy(ptr + shift, ptr + tmpshift, len >> 1);
    memcpy(ptr + ctx.key_sz + shift, ptr + tmpshift + (len >> 1), len >> 1);
    hpre_req.src = ptr;
    msg.in = cpu_to_le64(dma);
    return 0;
    }
    static int hpre_ecdh_dst_data_init(struct hpre_asym_request *hpre_req,
    struct scatterlist *data, unsigned int len)
    {
    struct hpre_sqe *msg = &hpre_req.req;
    struct hpre_ctx *ctx = hpre_req.ctx;
    struct device *dev = ctx.dev;
    dma_addr_t dma;
    if (unlikely(!data || !sg_is_last(data) || len != ctx.key_sz << 1)) {
    dev_err(dev, "data or data length is illegal!\n");
    return -EINVAL;
    }
    hpre_req.dst = core::ptr::null_mut();
    dma = dma_map_single(dev, sg_virt(data), len, DMA_FROM_DEVICE);
    if (unlikely(dma_mapping_error(dev, dma))) {
    dev_err(dev, "dma map data err!\n");
    return -ENOMEM;
    }
    msg.out = cpu_to_le64(dma);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpre_ecdh_compute_value(req: *mut kpp_request) -> c_int {
    static int hpre_ecdh_compute_value(struct kpp_request *req)
    {
    struct crypto_kpp *tfm = crypto_kpp_reqtfm(req);
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    struct device *dev = ctx.dev;
    void *tmp = kpp_request_ctx(req);
    struct hpre_asym_request *hpre_req = PTR_ALIGN(tmp, hpre_align_sz());
    struct hpre_sqe *msg = &hpre_req.req;
    int ret;
    ret = hpre_ecdh_msg_request_set(ctx, req);
    if (unlikely(ret)) {
    dev_err(dev, "failed to set ecdh request, ret = %d!\n", ret);
    return ret;
    }
    if (req.src) {
    ret = hpre_ecdh_src_data_init(hpre_req, req.src, req.src_len);
    if (unlikely(ret)) {
    dev_err(dev, "failed to init src data, ret = %d!\n", ret);
    goto clear_all;
    }
    } else {
    msg.in = cpu_to_le64(ctx.ecdh.dma_g);
    }
    ret = hpre_ecdh_dst_data_init(hpre_req, req.dst, req.dst_len);
    if (unlikely(ret)) {
    dev_err(dev, "failed to init dst data, ret = %d!\n", ret);
    goto clear_all;
    }
    msg.dw0 = cpu_to_le32(le32_to_cpu(msg.dw0) | HPRE_ALG_ECC_MUL);
    msg.resv1 = ctx.enable_hpcore << HPRE_ENABLE_HPCORE_SHIFT;
    ret = hpre_send(ctx, msg);
    if (likely(!ret))
    return -EINPROGRESS;
    clear_all:
    hpre_ecdh_hw_data_clr_all(ctx, hpre_req, req.dst, req.src);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hpre_ecdh_generate_public_key(req: *mut kpp_request) -> c_int {
    static int hpre_ecdh_generate_public_key(struct kpp_request *req)
    {
    struct crypto_kpp *tfm = crypto_kpp_reqtfm(req);
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    int ret;
    if (ctx.fallback) {
    kpp_request_set_tfm(req, ctx.ecdh.soft_tfm);
    ret = crypto_kpp_generate_public_key(req);
    kpp_request_set_tfm(req, tfm);
    return ret;
    }
    return hpre_ecdh_compute_value(req);
    }
#[no_mangle]
unsafe extern "C" fn hpre_ecdh_compute_shared_secret(req: *mut kpp_request) -> c_int {
    static int hpre_ecdh_compute_shared_secret(struct kpp_request *req)
    {
    struct crypto_kpp *tfm = crypto_kpp_reqtfm(req);
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    int ret;
    if (ctx.fallback) {
    kpp_request_set_tfm(req, ctx.ecdh.soft_tfm);
    ret = crypto_kpp_compute_shared_secret(req);
    kpp_request_set_tfm(req, tfm);
    return ret;
    }
    return hpre_ecdh_compute_value(req);
    }
#[no_mangle]
unsafe extern "C" fn hpre_ecdh_max_size(tfm: *mut crypto_kpp) -> c_uint {
    static unsigned int hpre_ecdh_max_size(struct crypto_kpp *tfm)
    {
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    if (ctx.fallback)
    return crypto_kpp_maxsize(ctx.ecdh.soft_tfm);
// max size is the pub_key_size, include x and y
    return ctx.key_sz << 1;
    }
#[no_mangle]
unsafe extern "C" fn hpre_ecdh_init_tfm(tfm: *mut crypto_kpp) -> c_int {
    static int hpre_ecdh_init_tfm(struct crypto_kpp *tfm)
    {
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    const char *alg = kpp_alg_name(tfm);
    int ret;
    ret = hpre_ctx_init(ctx, HPRE_V3_ECC_ALG_TYPE);
    if (!ret) {
    kpp_set_reqsize(tfm, sizeof(struct hpre_asym_request) + hpre_align_pd());
    return 0;
    } else if (ret && ret != -ENODEV) {
    return ret;
    }
    ctx.ecdh.soft_tfm = crypto_alloc_kpp(alg, 0, CRYPTO_ALG_NEED_FALLBACK);
    if (IS_ERR(ctx.ecdh.soft_tfm)) {
    pr_err("Failed to alloc %s tfm!\n", alg);
    return PTR_ERR(ctx.ecdh.soft_tfm);
    }
    crypto_kpp_set_flags(ctx.ecdh.soft_tfm, crypto_kpp_get_flags(tfm));
    ctx.fallback = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpre_ecdh_nist_p192_init_tfm(tfm: *mut crypto_kpp) -> c_int {
    static int hpre_ecdh_nist_p192_init_tfm(struct crypto_kpp *tfm)
    {
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    ctx.curve_id = ECC_CURVE_NIST_P192;
    return hpre_ecdh_init_tfm(tfm);
    }
#[no_mangle]
unsafe extern "C" fn hpre_ecdh_nist_p256_init_tfm(tfm: *mut crypto_kpp) -> c_int {
    static int hpre_ecdh_nist_p256_init_tfm(struct crypto_kpp *tfm)
    {
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    ctx.curve_id = ECC_CURVE_NIST_P256;
    ctx.enable_hpcore = 1;
    return hpre_ecdh_init_tfm(tfm);
    }
#[no_mangle]
unsafe extern "C" fn hpre_ecdh_nist_p384_init_tfm(tfm: *mut crypto_kpp) -> c_int {
    static int hpre_ecdh_nist_p384_init_tfm(struct crypto_kpp *tfm)
    {
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    ctx.curve_id = ECC_CURVE_NIST_P384;
    return hpre_ecdh_init_tfm(tfm);
    }
#[no_mangle]
unsafe extern "C" fn hpre_ecdh_exit_tfm(tfm: *mut crypto_kpp) {
    static void hpre_ecdh_exit_tfm(struct crypto_kpp *tfm)
    {
    struct hpre_ctx *ctx = kpp_tfm_ctx(tfm);
    if (ctx.fallback) {
    crypto_free_kpp(ctx.ecdh.soft_tfm);
    return;
    }
    hpre_ecc_clear_ctx(ctx, true);
    }
    static struct akcipher_alg rsa = {
    .encrypt = hpre_rsa_enc,
    .decrypt = hpre_rsa_dec,
    .set_pub_key = hpre_rsa_setpubkey,
    .set_priv_key = hpre_rsa_setprivkey,
    .max_size = hpre_rsa_max_size,
    .init = hpre_rsa_init_tfm,
    .exit = hpre_rsa_exit_tfm,
    .base = {
    .cra_ctxsize = sizeof(struct hpre_ctx),
    .cra_priority = HPRE_CRYPTO_ALG_PRI,
    .cra_name = "rsa",
    .cra_driver_name = "hpre-rsa",
    .cra_module = THIS_MODULE,
    .cra_flags = CRYPTO_ALG_NEED_FALLBACK,
    },
    };
    static struct kpp_alg dh = {
    .set_secret = hpre_dh_set_secret,
    .generate_public_key = hpre_dh_generate_public_key,
    .compute_shared_secret = hpre_dh_compute_shared_secret,
    .max_size = hpre_dh_max_size,
    .init = hpre_dh_init_tfm,
    .exit = hpre_dh_exit_tfm,
    .base = {
    .cra_ctxsize = sizeof(struct hpre_ctx),
    .cra_priority = HPRE_CRYPTO_ALG_PRI,
    .cra_name = "dh",
    .cra_driver_name = "hpre-dh",
    .cra_module = THIS_MODULE,
    .cra_flags = CRYPTO_ALG_NEED_FALLBACK,
    },
    };
    static struct kpp_alg ecdh_curves[] = {
    {
    .set_secret = hpre_ecdh_set_secret,
    .generate_public_key = hpre_ecdh_generate_public_key,
    .compute_shared_secret = hpre_ecdh_compute_shared_secret,
    .max_size = hpre_ecdh_max_size,
    .init = hpre_ecdh_nist_p192_init_tfm,
    .exit = hpre_ecdh_exit_tfm,
    .base = {
    .cra_ctxsize = sizeof(struct hpre_ctx),
    .cra_priority = HPRE_CRYPTO_ALG_PRI,
    .cra_name = "ecdh-nist-p192",
    .cra_driver_name = "hpre-ecdh-nist-p192",
    .cra_module = THIS_MODULE,
    .cra_flags = CRYPTO_ALG_NEED_FALLBACK,
    },
    }, {
    .set_secret = hpre_ecdh_set_secret,
    .generate_public_key = hpre_ecdh_generate_public_key,
    .compute_shared_secret = hpre_ecdh_compute_shared_secret,
    .max_size = hpre_ecdh_max_size,
    .init = hpre_ecdh_nist_p256_init_tfm,
    .exit = hpre_ecdh_exit_tfm,
    .base = {
    .cra_ctxsize = sizeof(struct hpre_ctx),
    .cra_priority = HPRE_CRYPTO_ALG_PRI,
    .cra_name = "ecdh-nist-p256",
    .cra_driver_name = "hpre-ecdh-nist-p256",
    .cra_module = THIS_MODULE,
    .cra_flags = CRYPTO_ALG_NEED_FALLBACK,
    },
    }, {
    .set_secret = hpre_ecdh_set_secret,
    .generate_public_key = hpre_ecdh_generate_public_key,
    .compute_shared_secret = hpre_ecdh_compute_shared_secret,
    .max_size = hpre_ecdh_max_size,
    .init = hpre_ecdh_nist_p384_init_tfm,
    .exit = hpre_ecdh_exit_tfm,
    .base = {
    .cra_ctxsize = sizeof(struct hpre_ctx),
    .cra_priority = HPRE_CRYPTO_ALG_PRI,
    .cra_name = "ecdh-nist-p384",
    .cra_driver_name = "hpre-ecdh-nist-p384",
    .cra_module = THIS_MODULE,
    .cra_flags = CRYPTO_ALG_NEED_FALLBACK,
    },
    }
    };
#[no_mangle]
unsafe extern "C" fn hpre_register_rsa(qm: *mut hisi_qm) -> c_int {
    static int hpre_register_rsa(struct hisi_qm *qm)
    {
    int ret;
    if (!hpre_check_alg_support(qm, HPRE_DRV_RSA_MASK_CAP))
    return 0;
    rsa.base.cra_flags = 0;
    ret = crypto_register_akcipher(&rsa);
    if (ret)
    dev_err(&qm.pdev.dev, "failed to register rsa (%d)!\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hpre_unregister_rsa(qm: *mut hisi_qm) {
    static void hpre_unregister_rsa(struct hisi_qm *qm)
    {
    if (!hpre_check_alg_support(qm, HPRE_DRV_RSA_MASK_CAP))
    return;
    crypto_unregister_akcipher(&rsa);
    }
#[no_mangle]
unsafe extern "C" fn hpre_register_dh(qm: *mut hisi_qm) -> c_int {
    static int hpre_register_dh(struct hisi_qm *qm)
    {
    int ret;
    if (!hpre_check_alg_support(qm, HPRE_DRV_DH_MASK_CAP))
    return 0;
    ret = crypto_register_kpp(&dh);
    if (ret)
    dev_err(&qm.pdev.dev, "failed to register dh (%d)!\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hpre_unregister_dh(qm: *mut hisi_qm) {
    static void hpre_unregister_dh(struct hisi_qm *qm)
    {
    if (!hpre_check_alg_support(qm, HPRE_DRV_DH_MASK_CAP))
    return;
    crypto_unregister_kpp(&dh);
    }
#[no_mangle]
unsafe extern "C" fn hpre_register_ecdh(qm: *mut hisi_qm) -> c_int {
    static int hpre_register_ecdh(struct hisi_qm *qm)
    {
    int ret, i;
    if (!hpre_check_alg_support(qm, HPRE_DRV_ECDH_MASK_CAP))
    return 0;
    for (i = 0; i < ARRAY_SIZE(ecdh_curves); i++) {
    ret = crypto_register_kpp(&ecdh_curves[i]);
    if (ret) {
    dev_err(&qm.pdev.dev, "failed to register %s (%d)!\n",
    ecdh_curves[i].base.cra_name, ret);
    goto unreg_kpp;
    }
    }
    return 0;
    unreg_kpp:
    for (--i; i >= 0; --i)
    crypto_unregister_kpp(&ecdh_curves[i]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hpre_unregister_ecdh(qm: *mut hisi_qm) {
    static void hpre_unregister_ecdh(struct hisi_qm *qm)
    {
    int i;
    if (!hpre_check_alg_support(qm, HPRE_DRV_ECDH_MASK_CAP))
    return;
    for (i = ARRAY_SIZE(ecdh_curves) - 1; i >= 0; --i)
    crypto_unregister_kpp(&ecdh_curves[i]);
    }
#[no_mangle]
pub unsafe extern "C" fn hpre_algs_register(qm: *mut hisi_qm) -> c_int {
    int hpre_algs_register(struct hisi_qm *qm)
    {
    let mut ret: c_int = 0;
    mutex_lock(&hpre_algs_lock);
    if (hpre_available_devs) {
    hpre_available_devs++;
    goto unlock;
    }
    ret = hpre_register_rsa(qm);
    if (ret)
    goto unlock;
    ret = hpre_register_dh(qm);
    if (ret)
    goto unreg_rsa;
    ret = hpre_register_ecdh(qm);
    if (ret)
    goto unreg_dh;
    hpre_available_devs++;
    mutex_unlock(&hpre_algs_lock);
    return ret;
    unreg_dh:
    hpre_unregister_dh(qm);
    unreg_rsa:
    hpre_unregister_rsa(qm);
    unlock:
    mutex_unlock(&hpre_algs_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hpre_algs_unregister(qm: *mut hisi_qm) {
    void hpre_algs_unregister(struct hisi_qm *qm)
    {
    mutex_lock(&hpre_algs_lock);
    if (--hpre_available_devs)
    goto unlock;
    hpre_unregister_ecdh(qm);
    hpre_unregister_dh(qm);
    hpre_unregister_rsa(qm);
    unlock:
    mutex_unlock(&hpre_algs_lock);
    }
