//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/intel/qat/qat_common/qat_asym_algs.c
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation

    static DEFINE_MUTEX(algs_lock);
    static unsigned int active_devs;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_rsa_input_params {
    union {
    struct {
    pub m: dma_addr_t,
    pub e: dma_addr_t,
    pub n: dma_addr_t,
    pub enc: },
    struct {
    pub c: dma_addr_t,
    pub d: dma_addr_t,
    pub n: dma_addr_t,
    pub dec: },
    struct {
    pub c: dma_addr_t,
    pub p: dma_addr_t,
    pub q: dma_addr_t,
    pub dp: dma_addr_t,
    pub dq: dma_addr_t,
    pub qinv: dma_addr_t,
    pub dec_crt: },
    pub in_tab: [u64; 8],
}

    } __packed __aligned(64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_rsa_output_params {
    union {
    struct {
    pub c: dma_addr_t,
    pub enc: },
    struct {
    pub m: dma_addr_t,
    pub dec: },
    pub out_tab: [u64; 8],
}

    } __packed __aligned(64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_rsa_ctx {
    pub n: *mut c_char,
    pub e: *mut c_char,
    pub d: *mut c_char,
    pub p: *mut c_char,
    pub q: *mut c_char,
    pub dp: *mut c_char,
    pub dq: *mut c_char,
    pub qinv: *mut c_char,
    pub dma_n: dma_addr_t,
    pub dma_e: dma_addr_t,
    pub dma_d: dma_addr_t,
    pub dma_p: dma_addr_t,
    pub dma_q: dma_addr_t,
    pub dma_dp: dma_addr_t,
    pub dma_dq: dma_addr_t,
    pub dma_qinv: dma_addr_t,
    pub key_sz: c_uint,
    pub crt_mode: bool,
    pub inst: *mut qat_crypto_instance,
    pub __aligned(64): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_dh_input_params {
    union {
    struct {
    pub b: dma_addr_t,
    pub xa: dma_addr_t,
    pub p: dma_addr_t,
    pub in: },
    struct {
    pub xa: dma_addr_t,
    pub p: dma_addr_t,
    pub in_g2: },
    pub in_tab: [u64; 8],
}

    } __packed __aligned(64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_dh_output_params {
    union {
    pub r: dma_addr_t,
    pub out_tab: [u64; 8],
}

    } __packed __aligned(64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_dh_ctx {
    pub g: *mut c_char,
    pub xa: *mut c_char,
    pub p: *mut c_char,
    pub dma_g: dma_addr_t,
    pub dma_xa: dma_addr_t,
    pub dma_p: dma_addr_t,
    pub p_size: c_uint,
    pub g2: bool,
    pub inst: *mut qat_crypto_instance,
    pub ftfm: *mut crypto_kpp,
    pub fallback: bool,
    pub __aligned(64): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_asym_request {
    union {
    pub rsa: qat_rsa_input_params,
    pub dh: qat_dh_input_params,
    pub in: },
    union {
    pub rsa: qat_rsa_output_params,
    pub dh: qat_dh_output_params,
    pub out: },
    pub phy_in: dma_addr_t,
    pub phy_out: dma_addr_t,
    pub src_align: *mut c_char,
    pub dst_align: *mut c_char,
    pub req: icp_qat_fw_pke_request,
    union {
    pub rsa: *mut qat_rsa_ctx,
    pub dh: *mut qat_dh_ctx,
    pub ctx: },
    union {
    pub rsa: *mut akcipher_request,
    pub dh: *mut kpp_request,
    pub areq: },
    pub err: c_int,
    pub resp): *mut *mut void (cb)(struct icp_qat_fw_pke_resp,
    pub alg_req: qat_alg_req,
    pub __aligned(64): },
    static int qat_alg_send_asym_message(struct qat_asym_request *qat_req,
    struct qat_crypto_instance *inst,
    struct crypto_async_request *base)
    {
    pub &qat_req->alg_req: *mut *mut qat_alg_req alg_req =,
    pub )&qat_req->req: *mut alg_req->fw_req = (u32,
    pub inst->pke_tx: alg_req->tx_ring =,
    pub base: alg_req->base =,
    pub &inst->backlog: alg_req->backlog =,
    pub qat_alg_send_message(alg_req): return,
    }
#[no_mangle]
unsafe extern "C" fn qat_dh_cb(resp: *mut icp_qat_fw_pke_resp) {
    static void qat_dh_cb(struct icp_qat_fw_pke_resp *resp)
    {
    pub long)resp->opaque: *mut *mut *mut qat_asym_request req = (void )(,
    pub req->areq.dh: *mut *mut kpp_request areq =,
    pub &GET_DEV(req->ctx.dh->inst->accel_dev): *mut *mut device dev =,
    int err = ICP_QAT_FW_PKE_RESP_PKE_STAT_GET(
    pub -EINVAL: err = (err == ICP_QAT_FW_COMN_STATUS_FLAG_OK) ? 0 :,
    if (areq.src) {
    dma_unmap_single(dev, req.in.dh.in.b, req.ctx.dh.p_size,
    }
    pub req->ctx.dh->p_size: areq->dst_len =,
    dma_unmap_single(dev, req.out.dh.r, req.ctx.dh.p_size,
    if (req.dst_align) {
    scatterwalk_map_and_copy(req.dst_align, areq.dst, 0,
    pub 1): areq->dst_len,,
    }
    dma_unmap_single(dev, req.phy_in, sizeof(struct qat_dh_input_params),
    dma_unmap_single(dev, req.phy_out,
    sizeof(struct qat_dh_output_params),
    pub err): kpp_request_complete(areq,,
    }
pub const PKE_DH_1536: c_uint = 0x390c1a49;
pub const PKE_DH_G2_1536: c_uint = 0x2e0b1a3e;
pub const PKE_DH_2048: c_uint = 0x4d0c1a60;
pub const PKE_DH_G2_2048: c_uint = 0x3e0b1a55;
pub const PKE_DH_3072: c_uint = 0x510c1a77;
pub const PKE_DH_G2_3072: c_uint = 0x3a0b1a6c;
pub const PKE_DH_4096: c_uint = 0x690c1a8e;
pub const PKE_DH_G2_4096: c_uint = 0x4a0b1a83;
#[no_mangle]
unsafe extern "C" fn qat_dh_fn_id(len: c_uint, g2: bool) -> c_ulong {
    static unsigned long qat_dh_fn_id(unsigned int len, bool g2)
    {
    pub 3: unsigned int bitslen = len <<,
    switch (bitslen) {
    case 1536:
    pub PKE_DH_1536: return g2 ? PKE_DH_G2_1536 :,
    case 2048:
    pub PKE_DH_2048: return g2 ? PKE_DH_G2_2048 :,
    case 3072:
    pub PKE_DH_3072: return g2 ? PKE_DH_G2_3072 :,
    case 4096:
    pub PKE_DH_4096: return g2 ? PKE_DH_G2_4096 :,
    default:
    pub 0: return,
    }
    }
#[no_mangle]
unsafe extern "C" fn qat_dh_compute_value(req: *mut kpp_request) -> c_int {
    static int qat_dh_compute_value(struct kpp_request *req)
    {
    pub crypto_kpp_reqtfm(req): *mut *mut crypto_kpp tfm =,
    pub kpp_tfm_ctx(tfm): *mut *mut qat_dh_ctx ctx =,
    pub ctx->inst: *mut *mut qat_crypto_instance inst =,
    pub &GET_DEV(inst->accel_dev): *mut *mut device dev =,
    struct qat_asym_request *qat_req =
    pub 64): PTR_ALIGN(kpp_request_ctx(req),,
    pub &qat_req->req: *mut *mut icp_qat_fw_pke_request msg =,
    pub qat_algs_alloc_flags(&req->base): gfp_t flags =,
    pub 0: int n_input_params =,
    pub vaddr: *mut u8,
    pub ret: c_int,
    if (unlikely(!ctx.xa))
    pub -EINVAL: return,
    if (req.dst_len < ctx.p_size) {
    pub ctx->p_size: req->dst_len =,
    pub -EOVERFLOW: return,
    }
    if (req.src_len > ctx.p_size)
    pub -EINVAL: return,
    pub sizeof(*msg)): *mut memset(msg, '\0',,
    ICP_QAT_FW_PKE_HDR_VALID_FLAG_SET(msg.pke_hdr,
    msg.pke_hdr.cd_pars.func_id = qat_dh_fn_id(ctx.p_size,
    pub ctx->g2): !req->src &&,
    if (unlikely(!msg.pke_hdr.cd_pars.func_id))
    pub -EINVAL: return,
    pub qat_dh_cb: qat_req->cb =,
    pub ctx: qat_req->ctx.dh =,
    pub req: qat_req->areq.dh =,
    pub ICP_QAT_FW_COMN_REQ_CPM_FW_PKE: msg->pke_hdr.service_type =,
    msg.pke_hdr.comn_req_flags =
    ICP_QAT_FW_COMN_FLAGS_BUILD(QAT_COMN_CD_FLD_TYPE_64BIT_ADR,
//
// If no source is provided use g as base
//
    if (req.src) {
    pub ctx->dma_xa: qat_req->in.dh.in.xa =,
    pub ctx->dma_p: qat_req->in.dh.in.p =,
    pub 3: n_input_params =,
    } else {
    if (ctx.g2) {
    pub ctx->dma_xa: qat_req->in.dh.in_g2.xa =,
    pub ctx->dma_p: qat_req->in.dh.in_g2.p =,
    pub 2: n_input_params =,
    } else {
    pub ctx->dma_g: qat_req->in.dh.in.b =,
    pub ctx->dma_xa: qat_req->in.dh.in.xa =,
    pub ctx->dma_p: qat_req->in.dh.in.p =,
    pub 3: n_input_params =,
    }
    }
    pub -ENOMEM: ret =,
    if (req.src) {
//
// src can be of any size in valid range, but HW expects it to
// be the same as modulo p so in case it is different we need
// to allocate a new buf and copy src data.
// In other case we just need to map the user provided buffer.
// Also need to make sure that it is in contiguous buffer.
//
    if (sg_is_last(req.src) && req.src_len == ctx.p_size) {
    pub NULL: qat_req->src_align =,
    pub sg_virt(req->src): vaddr =,
    } else {
    pub req->src_len: int shift = ctx->p_size -,
    pub flags): qat_req->src_align = kzalloc(ctx->p_size,,
    if (unlikely(!qat_req.src_align))
    pub ret: return,
    scatterwalk_map_and_copy(qat_req.src_align + shift,
    pub 0): req->src, 0, req->src_len,,
    pub qat_req->src_align: vaddr =,
    }
    qat_req.in.dh.in.b = dma_map_single(dev, vaddr, ctx.p_size,
    if (unlikely(dma_mapping_error(dev, qat_req.in.dh.in.b)))
    pub unmap_src: goto,
    }
//
// dst can be of any size in valid range, but HW expects it to be the
// same as modulo m so in case it is different we need to allocate a
// new buf and copy src data.
// In other case we just need to map the user provided buffer.
// Also need to make sure that it is in contiguous buffer.
//
    if (sg_is_last(req.dst) && req.dst_len == ctx.p_size) {
    pub NULL: qat_req->dst_align =,
    pub sg_virt(req->dst): vaddr =,
    } else {
    pub flags): qat_req->dst_align = kzalloc(ctx->p_size,,
    if (unlikely(!qat_req.dst_align))
    pub unmap_src: goto,
    pub qat_req->dst_align: vaddr =,
    }
    qat_req.out.dh.r = dma_map_single(dev, vaddr, ctx.p_size,
    if (unlikely(dma_mapping_error(dev, qat_req.out.dh.r)))
    pub unmap_dst: goto,
    pub 0: qat_req->in.dh.in_tab[n_input_params] =,
    pub 0: qat_req->out.dh.out_tab[1] =,
// Mapping in.in.b or in.in_g2.xa is the same
    qat_req.phy_in = dma_map_single(dev, &qat_req.in.dh,
    sizeof(struct qat_dh_input_params),
    if (unlikely(dma_mapping_error(dev, qat_req.phy_in)))
    pub unmap_dst: goto,
    qat_req.phy_out = dma_map_single(dev, &qat_req.out.dh,
    sizeof(struct qat_dh_output_params),
    if (unlikely(dma_mapping_error(dev, qat_req.phy_out)))
    pub unmap_in_params: goto,
    pub qat_req->phy_in: msg->pke_mid.src_data_addr =,
    pub qat_req->phy_out: msg->pke_mid.dest_data_addr =,
    pub long)qat_req: msg->pke_mid.opaque = (u64)(,
    pub n_input_params: msg->input_param_count =,
    pub 1: msg->output_param_count =,
    pub &req->base): ret = qat_alg_send_asym_message(qat_req, inst,,
    if (ret == -ENOSPC)
    pub unmap_all: goto,
    pub ret: return,
    unmap_all:
    if (!dma_mapping_error(dev, qat_req.phy_out))
    dma_unmap_single(dev, qat_req.phy_out,
    sizeof(struct qat_dh_output_params),
    unmap_in_params:
    if (!dma_mapping_error(dev, qat_req.phy_in))
    dma_unmap_single(dev, qat_req.phy_in,
    sizeof(struct qat_dh_input_params),
    unmap_dst:
    if (!dma_mapping_error(dev, qat_req.out.dh.r))
    dma_unmap_single(dev, qat_req.out.dh.r, ctx.p_size,
    unmap_src:
    if (req.src) {
    if (!dma_mapping_error(dev, qat_req.in.dh.in.b))
    dma_unmap_single(dev, qat_req.in.dh.in.b,
    ctx.p_size,
    }
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn qat_dh_generate_public_key(req: *mut kpp_request) -> c_int {
    static int qat_dh_generate_public_key(struct kpp_request *req)
    {
    pub kpp_request_ctx(req): *mut *mut kpp_request nreq =,
    pub crypto_kpp_reqtfm(req): *mut *mut crypto_kpp tfm =,
    pub kpp_tfm_ctx(tfm): *mut *mut qat_dh_ctx ctx =,
    if (ctx.fallback) {
    pub sizeof(*req)): *mut memcpy(nreq, req,,
    pub ctx->ftfm): kpp_request_set_tfm(nreq,,
    pub crypto_kpp_generate_public_key(nreq): return,
    }
    pub qat_dh_compute_value(req): return,
    }
#[no_mangle]
unsafe extern "C" fn qat_dh_compute_shared_secret(req: *mut kpp_request) -> c_int {
    static int qat_dh_compute_shared_secret(struct kpp_request *req)
    {
    pub kpp_request_ctx(req): *mut *mut kpp_request nreq =,
    pub crypto_kpp_reqtfm(req): *mut *mut crypto_kpp tfm =,
    pub kpp_tfm_ctx(tfm): *mut *mut qat_dh_ctx ctx =,
    if (ctx.fallback) {
    pub sizeof(*req)): *mut memcpy(nreq, req,,
    pub ctx->ftfm): kpp_request_set_tfm(nreq,,
    pub crypto_kpp_compute_shared_secret(nreq): return,
    }
    pub qat_dh_compute_value(req): return,
    }
#[no_mangle]
unsafe extern "C" fn qat_dh_check_params_length(p_len: c_uint) -> c_int {
    static int qat_dh_check_params_length(unsigned int p_len)
    {
    switch (p_len) {
    case 1536:
    case 2048:
    case 3072:
    case 4096:
    pub 0: return,
    }
    pub -EINVAL: return,
    }
#[no_mangle]
unsafe extern "C" fn qat_dh_set_params(ctx: *mut qat_dh_ctx, params: *mut dh) -> c_int {
    static int qat_dh_set_params(struct qat_dh_ctx *ctx, struct dh *params)
    {
    pub ctx->inst: *mut *mut qat_crypto_instance inst =,
    pub &GET_DEV(inst->accel_dev): *mut *mut device dev =,
    pub params->p_size: ctx->p_size =,
    pub GFP_KERNEL): ctx->p = dma_alloc_coherent(dev, ctx->p_size, &ctx->dma_p,,
    if (!ctx.p)
    pub -ENOMEM: return,
    pub ctx->p_size): memcpy(ctx->p, params->p,,
// If g equals 2 don't copy it
    if (params.g_size == 1 && *(char *)params.g == 0x02) {
    pub true: ctx->g2 =,
    pub 0: return,
    }
    pub GFP_KERNEL): ctx->g = dma_alloc_coherent(dev, ctx->p_size, &ctx->dma_g,,
    if (!ctx.g)
    pub -ENOMEM: return,
    memcpy(ctx.g + (ctx.p_size - params.g_size), params.g,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn qat_dh_clear_ctx(dev: *mut device, ctx: *mut qat_dh_ctx) {
    static void qat_dh_clear_ctx(struct device *dev, struct qat_dh_ctx *ctx)
    {
    if (ctx.g) {
    pub ctx->p_size): memset(ctx->g, 0,,
    pub ctx->dma_g): dma_free_coherent(dev, ctx->p_size, ctx->g,,
    pub NULL: ctx->g =,
    }
    if (ctx.xa) {
    pub ctx->p_size): memset(ctx->xa, 0,,
    pub ctx->dma_xa): dma_free_coherent(dev, ctx->p_size, ctx->xa,,
    pub NULL: ctx->xa =,
    }
    if (ctx.p) {
    pub ctx->p_size): memset(ctx->p, 0,,
    pub ctx->dma_p): dma_free_coherent(dev, ctx->p_size, ctx->p,,
    pub NULL: ctx->p =,
    }
    pub 0: ctx->p_size =,
    pub false: ctx->g2 =,
    }
    static int qat_dh_set_secret(struct crypto_kpp *tfm, const void *buf,
    unsigned int len)
    {
    pub kpp_tfm_ctx(tfm): *mut *mut qat_dh_ctx ctx =,
    pub &GET_DEV(ctx->inst->accel_dev): *mut *mut device dev =,
    pub params: dh,
    pub ret: c_int,
    if (crypto_dh_decode_key(buf, len, &params) < 0)
    pub -EINVAL: return,
    if (qat_dh_check_params_length(params.p_size << 3)) {
    pub true: ctx->fallback =,
    pub len): return crypto_kpp_set_secret(ctx->ftfm, buf,,
    }
    pub false: ctx->fallback =,
// Free old secret if any
    pub ctx): qat_dh_clear_ctx(dev,,
    pub &params): ret = qat_dh_set_params(ctx,,
    if (ret < 0)
    pub err_clear_ctx: goto,
    ctx.xa = dma_alloc_coherent(dev, ctx.p_size, &ctx.dma_xa,
    if (!ctx.xa) {
    pub -ENOMEM: ret =,
    pub err_clear_ctx: goto,
    }
    memcpy(ctx.xa + (ctx.p_size - params.key_size), params.key,
    pub 0: return,
    err_clear_ctx:
    pub ctx): qat_dh_clear_ctx(dev,,
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn qat_dh_max_size(tfm: *mut crypto_kpp) -> c_uint {
    static unsigned int qat_dh_max_size(struct crypto_kpp *tfm)
    {
    pub kpp_tfm_ctx(tfm): *mut *mut qat_dh_ctx ctx =,
    if (ctx.fallback)
    pub crypto_kpp_maxsize(ctx->ftfm): return,
    pub ctx->p_size: return,
    }
#[no_mangle]
unsafe extern "C" fn qat_dh_init_tfm(tfm: *mut crypto_kpp) -> c_int {
    static int qat_dh_init_tfm(struct crypto_kpp *tfm)
    {
    pub kpp_tfm_ctx(tfm): *mut *mut qat_dh_ctx ctx =,
    struct qat_crypto_instance *inst =
    pub kpp_alg_name(tfm): *const *const char alg =,
    pub reqsize: c_uint,
    if (!inst)
    pub -EINVAL: return,
    pub CRYPTO_ALG_NEED_FALLBACK): ctx->ftfm = crypto_alloc_kpp(alg, 0,,
    if (IS_ERR(ctx.ftfm))
    pub PTR_ERR(ctx->ftfm): return,
    pub crypto_kpp_get_flags(tfm)): crypto_kpp_set_flags(ctx->ftfm,,
    reqsize = max(sizeof(struct qat_asym_request) + 64,
    pub crypto_kpp_reqsize(ctx->ftfm)): sizeof(struct kpp_request) +,
    pub reqsize): kpp_set_reqsize(tfm,,
    pub 0: ctx->p_size =,
    pub false: ctx->g2 =,
    pub inst: ctx->inst =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn qat_dh_exit_tfm(tfm: *mut crypto_kpp) {
    static void qat_dh_exit_tfm(struct crypto_kpp *tfm)
    {
    pub kpp_tfm_ctx(tfm): *mut *mut qat_dh_ctx ctx =,
    pub &GET_DEV(ctx->inst->accel_dev): *mut *mut device dev =,
    if (ctx.ftfm)
    pub ctx): qat_dh_clear_ctx(dev,,
    }
#[no_mangle]
unsafe extern "C" fn qat_rsa_cb(resp: *mut icp_qat_fw_pke_resp) {
    static void qat_rsa_cb(struct icp_qat_fw_pke_resp *resp)
    {
    pub long)resp->opaque: *mut *mut *mut qat_asym_request req = (void )(,
    pub req->areq.rsa: *mut *mut akcipher_request areq =,
    pub &GET_DEV(req->ctx.rsa->inst->accel_dev): *mut *mut device dev =,
    int err = ICP_QAT_FW_PKE_RESP_PKE_STAT_GET(
    pub -EINVAL: err = (err == ICP_QAT_FW_COMN_STATUS_FLAG_OK) ? 0 :,
    dma_unmap_single(dev, req.in.rsa.enc.m, req.ctx.rsa.key_sz,
    pub req->ctx.rsa->key_sz: areq->dst_len =,
    dma_unmap_single(dev, req.out.rsa.enc.c, req.ctx.rsa.key_sz,
    if (req.dst_align) {
    scatterwalk_map_and_copy(req.dst_align, areq.dst, 0,
    pub 1): areq->dst_len,,
    }
    dma_unmap_single(dev, req.phy_in, sizeof(struct qat_rsa_input_params),
    dma_unmap_single(dev, req.phy_out,
    sizeof(struct qat_rsa_output_params),
    pub err): akcipher_request_complete(areq,,
    }
#[no_mangle]
pub unsafe extern "C" fn qat_alg_asym_callback(_resp: *mut c_void) {
    void qat_alg_asym_callback(void *_resp)
    {
    pub _resp: *mut *mut icp_qat_fw_pke_resp resp =,
    pub long)resp->opaque: *mut *mut *mut qat_asym_request areq = (void )(,
    pub areq->alg_req.backlog: *mut *mut qat_instance_backlog backlog =,
    }
pub const PKE_RSA_EP_512: c_uint = 0x1c161b21;
pub const PKE_RSA_EP_1024: c_uint = 0x35111bf7;
pub const PKE_RSA_EP_1536: c_uint = 0x4d111cdc;
pub const PKE_RSA_EP_2048: c_uint = 0x6e111dba;
pub const PKE_RSA_EP_3072: c_uint = 0x7d111ea3;
pub const PKE_RSA_EP_4096: c_uint = 0xa5101f7e;
#[no_mangle]
unsafe extern "C" fn qat_rsa_enc_fn_id(len: c_uint) -> c_ulong {
    static unsigned long qat_rsa_enc_fn_id(unsigned int len)
    {
    pub 3: unsigned int bitslen = len <<,
    switch (bitslen) {
    case 512:
    pub PKE_RSA_EP_512: return,
    case 1024:
    pub PKE_RSA_EP_1024: return,
    case 1536:
    pub PKE_RSA_EP_1536: return,
    case 2048:
    pub PKE_RSA_EP_2048: return,
    case 3072:
    pub PKE_RSA_EP_3072: return,
    case 4096:
    pub PKE_RSA_EP_4096: return,
    default:
    pub 0: return,
    }
    }
pub const PKE_RSA_DP1_512: c_uint = 0x1c161b3c;
pub const PKE_RSA_DP1_1024: c_uint = 0x35111c12;
pub const PKE_RSA_DP1_1536: c_uint = 0x4d111cf7;
pub const PKE_RSA_DP1_2048: c_uint = 0x6e111dda;
pub const PKE_RSA_DP1_3072: c_uint = 0x7d111ebe;
pub const PKE_RSA_DP1_4096: c_uint = 0xa5101f98;
#[no_mangle]
unsafe extern "C" fn qat_rsa_dec_fn_id(len: c_uint) -> c_ulong {
    static unsigned long qat_rsa_dec_fn_id(unsigned int len)
    {
    pub 3: unsigned int bitslen = len <<,
    switch (bitslen) {
    case 512:
    pub PKE_RSA_DP1_512: return,
    case 1024:
    pub PKE_RSA_DP1_1024: return,
    case 1536:
    pub PKE_RSA_DP1_1536: return,
    case 2048:
    pub PKE_RSA_DP1_2048: return,
    case 3072:
    pub PKE_RSA_DP1_3072: return,
    case 4096:
    pub PKE_RSA_DP1_4096: return,
    default:
    pub 0: return,
    }
    }
pub const PKE_RSA_DP2_512: c_uint = 0x1c131b57;
pub const PKE_RSA_DP2_1024: c_uint = 0x26131c2d;
pub const PKE_RSA_DP2_1536: c_uint = 0x45111d12;
pub const PKE_RSA_DP2_2048: c_uint = 0x59121dfa;
pub const PKE_RSA_DP2_3072: c_uint = 0x81121ed9;
pub const PKE_RSA_DP2_4096: c_uint = 0xb1111fb2;
#[no_mangle]
unsafe extern "C" fn qat_rsa_dec_fn_id_crt(len: c_uint) -> c_ulong {
    static unsigned long qat_rsa_dec_fn_id_crt(unsigned int len)
    {
    pub 3: unsigned int bitslen = len <<,
    switch (bitslen) {
    case 512:
    pub PKE_RSA_DP2_512: return,
    case 1024:
    pub PKE_RSA_DP2_1024: return,
    case 1536:
    pub PKE_RSA_DP2_1536: return,
    case 2048:
    pub PKE_RSA_DP2_2048: return,
    case 3072:
    pub PKE_RSA_DP2_3072: return,
    case 4096:
    pub PKE_RSA_DP2_4096: return,
    default:
    pub 0: return,
    }
    }
#[no_mangle]
unsafe extern "C" fn qat_rsa_enc(req: *mut akcipher_request) -> c_int {
    static int qat_rsa_enc(struct akcipher_request *req)
    {
    pub crypto_akcipher_reqtfm(req): *mut *mut crypto_akcipher tfm =,
    pub akcipher_tfm_ctx(tfm): *mut *mut qat_rsa_ctx ctx =,
    pub ctx->inst: *mut *mut qat_crypto_instance inst =,
    pub &GET_DEV(inst->accel_dev): *mut *mut device dev =,
    struct qat_asym_request *qat_req =
    pub 64): PTR_ALIGN(akcipher_request_ctx(req),,
    pub &qat_req->req: *mut *mut icp_qat_fw_pke_request msg =,
    pub qat_algs_alloc_flags(&req->base): gfp_t flags =,
    pub vaddr: *mut u8,
    pub ret: c_int,
    if (unlikely(!ctx.n || !ctx.e))
    pub -EINVAL: return,
    if (req.dst_len < ctx.key_sz) {
    pub ctx->key_sz: req->dst_len =,
    pub -EOVERFLOW: return,
    }
    if (req.src_len > ctx.key_sz)
    pub -EINVAL: return,
    pub sizeof(*msg)): *mut memset(msg, '\0',,
    ICP_QAT_FW_PKE_HDR_VALID_FLAG_SET(msg.pke_hdr,
    pub qat_rsa_enc_fn_id(ctx->key_sz): msg->pke_hdr.cd_pars.func_id =,
    if (unlikely(!msg.pke_hdr.cd_pars.func_id))
    pub -EINVAL: return,
    pub qat_rsa_cb: qat_req->cb =,
    pub ctx: qat_req->ctx.rsa =,
    pub req: qat_req->areq.rsa =,
    pub ICP_QAT_FW_COMN_REQ_CPM_FW_PKE: msg->pke_hdr.service_type =,
    msg.pke_hdr.comn_req_flags =
    ICP_QAT_FW_COMN_FLAGS_BUILD(QAT_COMN_CD_FLD_TYPE_64BIT_ADR,
    pub ctx->dma_e: qat_req->in.rsa.enc.e =,
    pub ctx->dma_n: qat_req->in.rsa.enc.n =,
    pub -ENOMEM: ret =,
//
// src can be of any size in valid range, but HW expects it to be the
// same as modulo n so in case it is different we need to allocate a
// new buf and copy src data.
// In other case we just need to map the user provided buffer.
// Also need to make sure that it is in contiguous buffer.
//
    if (sg_is_last(req.src) && req.src_len == ctx.key_sz) {
    pub NULL: qat_req->src_align =,
    pub sg_virt(req->src): vaddr =,
    } else {
    pub req->src_len: int shift = ctx->key_sz -,
    pub flags): qat_req->src_align = kzalloc(ctx->key_sz,,
    if (unlikely(!qat_req.src_align))
    pub ret: return,
    scatterwalk_map_and_copy(qat_req.src_align + shift, req.src,
    pub 0): 0, req->src_len,,
    pub qat_req->src_align: vaddr =,
    }
    qat_req.in.rsa.enc.m = dma_map_single(dev, vaddr, ctx.key_sz,
    if (unlikely(dma_mapping_error(dev, qat_req.in.rsa.enc.m)))
    pub unmap_src: goto,
    if (sg_is_last(req.dst) && req.dst_len == ctx.key_sz) {
    pub NULL: qat_req->dst_align =,
    pub sg_virt(req->dst): vaddr =,
    } else {
    pub flags): qat_req->dst_align = kzalloc(ctx->key_sz,,
    if (unlikely(!qat_req.dst_align))
    pub unmap_src: goto,
    pub qat_req->dst_align: vaddr =,
    }
    qat_req.out.rsa.enc.c = dma_map_single(dev, vaddr, ctx.key_sz,
    if (unlikely(dma_mapping_error(dev, qat_req.out.rsa.enc.c)))
    pub unmap_dst: goto,
    pub 0: qat_req->in.rsa.in_tab[3] =,
    pub 0: qat_req->out.rsa.out_tab[1] =,
    qat_req.phy_in = dma_map_single(dev, &qat_req.in.rsa,
    sizeof(struct qat_rsa_input_params),
    if (unlikely(dma_mapping_error(dev, qat_req.phy_in)))
    pub unmap_dst: goto,
    qat_req.phy_out = dma_map_single(dev, &qat_req.out.rsa,
    sizeof(struct qat_rsa_output_params),
    if (unlikely(dma_mapping_error(dev, qat_req.phy_out)))
    pub unmap_in_params: goto,
    pub qat_req->phy_in: msg->pke_mid.src_data_addr =,
    pub qat_req->phy_out: msg->pke_mid.dest_data_addr =,
    pub long)qat_req: msg->pke_mid.opaque = (u64)(,
    pub 3: msg->input_param_count =,
    pub 1: msg->output_param_count =,
    pub &req->base): ret = qat_alg_send_asym_message(qat_req, inst,,
    if (ret == -ENOSPC)
    pub unmap_all: goto,
    pub ret: return,
    unmap_all:
    if (!dma_mapping_error(dev, qat_req.phy_out))
    dma_unmap_single(dev, qat_req.phy_out,
    sizeof(struct qat_rsa_output_params),
    unmap_in_params:
    if (!dma_mapping_error(dev, qat_req.phy_in))
    dma_unmap_single(dev, qat_req.phy_in,
    sizeof(struct qat_rsa_input_params),
    unmap_dst:
    if (!dma_mapping_error(dev, qat_req.out.rsa.enc.c))
    dma_unmap_single(dev, qat_req.out.rsa.enc.c,
    pub DMA_FROM_DEVICE): ctx->key_sz,,
    unmap_src:
    if (!dma_mapping_error(dev, qat_req.in.rsa.enc.m))
    dma_unmap_single(dev, qat_req.in.rsa.enc.m, ctx.key_sz,
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn qat_rsa_dec(req: *mut akcipher_request) -> c_int {
    static int qat_rsa_dec(struct akcipher_request *req)
    {
    pub crypto_akcipher_reqtfm(req): *mut *mut crypto_akcipher tfm =,
    pub akcipher_tfm_ctx(tfm): *mut *mut qat_rsa_ctx ctx =,
    pub ctx->inst: *mut *mut qat_crypto_instance inst =,
    pub &GET_DEV(inst->accel_dev): *mut *mut device dev =,
    struct qat_asym_request *qat_req =
    pub 64): PTR_ALIGN(akcipher_request_ctx(req),,
    pub &qat_req->req: *mut *mut icp_qat_fw_pke_request msg =,
    pub qat_algs_alloc_flags(&req->base): gfp_t flags =,
    pub vaddr: *mut u8,
    pub ret: c_int,
    if (unlikely(!ctx.n || !ctx.d))
    pub -EINVAL: return,
    if (req.dst_len < ctx.key_sz) {
    pub ctx->key_sz: req->dst_len =,
    pub -EOVERFLOW: return,
    }
    if (req.src_len > ctx.key_sz)
    pub -EINVAL: return,
    pub sizeof(*msg)): *mut memset(msg, '\0',,
    ICP_QAT_FW_PKE_HDR_VALID_FLAG_SET(msg.pke_hdr,
    msg.pke_hdr.cd_pars.func_id = ctx.crt_mode ?
    qat_rsa_dec_fn_id_crt(ctx.key_sz) :
    if (unlikely(!msg.pke_hdr.cd_pars.func_id))
    pub -EINVAL: return,
    pub qat_rsa_cb: qat_req->cb =,
    pub ctx: qat_req->ctx.rsa =,
    pub req: qat_req->areq.rsa =,
    pub ICP_QAT_FW_COMN_REQ_CPM_FW_PKE: msg->pke_hdr.service_type =,
    msg.pke_hdr.comn_req_flags =
    ICP_QAT_FW_COMN_FLAGS_BUILD(QAT_COMN_CD_FLD_TYPE_64BIT_ADR,
    if (ctx.crt_mode) {
    pub ctx->dma_p: qat_req->in.rsa.dec_crt.p =,
    pub ctx->dma_q: qat_req->in.rsa.dec_crt.q =,
    pub ctx->dma_dp: qat_req->in.rsa.dec_crt.dp =,
    pub ctx->dma_dq: qat_req->in.rsa.dec_crt.dq =,
    pub ctx->dma_qinv: qat_req->in.rsa.dec_crt.qinv =,
    } else {
    pub ctx->dma_d: qat_req->in.rsa.dec.d =,
    pub ctx->dma_n: qat_req->in.rsa.dec.n =,
    }
    pub -ENOMEM: ret =,
//
// src can be of any size in valid range, but HW expects it to be the
// same as modulo n so in case it is different we need to allocate a
// new buf and copy src data.
// In other case we just need to map the user provided buffer.
// Also need to make sure that it is in contiguous buffer.
//
    if (sg_is_last(req.src) && req.src_len == ctx.key_sz) {
    pub NULL: qat_req->src_align =,
    pub sg_virt(req->src): vaddr =,
    } else {
    pub req->src_len: int shift = ctx->key_sz -,
    pub flags): qat_req->src_align = kzalloc(ctx->key_sz,,
    if (unlikely(!qat_req.src_align))
    pub ret: return,
    scatterwalk_map_and_copy(qat_req.src_align + shift, req.src,
    pub 0): 0, req->src_len,,
    pub qat_req->src_align: vaddr =,
    }
    qat_req.in.rsa.dec.c = dma_map_single(dev, vaddr, ctx.key_sz,
    if (unlikely(dma_mapping_error(dev, qat_req.in.rsa.dec.c)))
    pub unmap_src: goto,
    if (sg_is_last(req.dst) && req.dst_len == ctx.key_sz) {
    pub NULL: qat_req->dst_align =,
    pub sg_virt(req->dst): vaddr =,
    } else {
    pub flags): qat_req->dst_align = kzalloc(ctx->key_sz,,
    if (unlikely(!qat_req.dst_align))
    pub unmap_src: goto,
    pub qat_req->dst_align: vaddr =,
    }
    qat_req.out.rsa.dec.m = dma_map_single(dev, vaddr, ctx.key_sz,
    if (unlikely(dma_mapping_error(dev, qat_req.out.rsa.dec.m)))
    pub unmap_dst: goto,
    if (ctx.crt_mode)
    pub 0: qat_req->in.rsa.in_tab[6] =,
    else
    pub 0: qat_req->in.rsa.in_tab[3] =,
    pub 0: qat_req->out.rsa.out_tab[1] =,
    qat_req.phy_in = dma_map_single(dev, &qat_req.in.rsa,
    sizeof(struct qat_rsa_input_params),
    if (unlikely(dma_mapping_error(dev, qat_req.phy_in)))
    pub unmap_dst: goto,
    qat_req.phy_out = dma_map_single(dev, &qat_req.out.rsa,
    sizeof(struct qat_rsa_output_params),
    if (unlikely(dma_mapping_error(dev, qat_req.phy_out)))
    pub unmap_in_params: goto,
    pub qat_req->phy_in: msg->pke_mid.src_data_addr =,
    pub qat_req->phy_out: msg->pke_mid.dest_data_addr =,
    pub long)qat_req: msg->pke_mid.opaque = (u64)(,
    if (ctx.crt_mode)
    pub 6: msg->input_param_count =,
    else
    pub 3: msg->input_param_count =,
    pub 1: msg->output_param_count =,
    pub &req->base): ret = qat_alg_send_asym_message(qat_req, inst,,
    if (ret == -ENOSPC)
    pub unmap_all: goto,
    pub ret: return,
    unmap_all:
    if (!dma_mapping_error(dev, qat_req.phy_out))
    dma_unmap_single(dev, qat_req.phy_out,
    sizeof(struct qat_rsa_output_params),
    unmap_in_params:
    if (!dma_mapping_error(dev, qat_req.phy_in))
    dma_unmap_single(dev, qat_req.phy_in,
    sizeof(struct qat_rsa_input_params),
    unmap_dst:
    if (!dma_mapping_error(dev, qat_req.out.rsa.dec.m))
    dma_unmap_single(dev, qat_req.out.rsa.dec.m,
    pub DMA_FROM_DEVICE): ctx->key_sz,,
    unmap_src:
    if (!dma_mapping_error(dev, qat_req.in.rsa.dec.c))
    dma_unmap_single(dev, qat_req.in.rsa.dec.c, ctx.key_sz,
    pub ret: return,
    }
    static int qat_rsa_set_n(struct qat_rsa_ctx *ctx, const char *value,
    size_t vlen)
    {
    pub ctx->inst: *mut *mut qat_crypto_instance inst =,
    pub &GET_DEV(inst->accel_dev): *mut *mut device dev =,
    pub value: *const *const char ptr =,
    pub ret: c_int,
    while (!*ptr && vlen) {
    }
    pub vlen: ctx->key_sz =,
    pub -EINVAL: ret =,
// invalid key size provided
    if (!qat_rsa_enc_fn_id(ctx.key_sz))
    pub err: goto,
    pub -ENOMEM: ret =,
    pub GFP_KERNEL): ctx->n = dma_alloc_coherent(dev, ctx->key_sz, &ctx->dma_n,,
    if (!ctx.n)
    pub err: goto,
    pub ctx->key_sz): memcpy(ctx->n, ptr,,
    pub 0: return,
    err:
    pub 0: ctx->key_sz =,
    pub NULL: ctx->n =,
    pub ret: return,
    }
    static int qat_rsa_set_e(struct qat_rsa_ctx *ctx, const char *value,
    size_t vlen)
    {
    pub ctx->inst: *mut *mut qat_crypto_instance inst =,
    pub &GET_DEV(inst->accel_dev): *mut *mut device dev =,
    pub value: *const *const char ptr =,
    while (!*ptr && vlen) {
    }
    if (!ctx.key_sz || !vlen || vlen > ctx.key_sz) {
    pub NULL: ctx->e =,
    pub -EINVAL: return,
    }
    pub GFP_KERNEL): ctx->e = dma_alloc_coherent(dev, ctx->key_sz, &ctx->dma_e,,
    if (!ctx.e)
    pub -ENOMEM: return,
    pub vlen): memcpy(ctx->e + (ctx->key_sz - vlen), ptr,,
    pub 0: return,
    }
    static int qat_rsa_set_d(struct qat_rsa_ctx *ctx, const char *value,
    size_t vlen)
    {
    pub ctx->inst: *mut *mut qat_crypto_instance inst =,
    pub &GET_DEV(inst->accel_dev): *mut *mut device dev =,
    pub value: *const *const char ptr =,
    pub ret: c_int,
    while (!*ptr && vlen) {
    }
    pub -EINVAL: ret =,
    if (!ctx.key_sz || !vlen || vlen > ctx.key_sz)
    pub err: goto,
    pub -ENOMEM: ret =,
    pub GFP_KERNEL): ctx->d = dma_alloc_coherent(dev, ctx->key_sz, &ctx->dma_d,,
    if (!ctx.d)
    pub err: goto,
    pub vlen): memcpy(ctx->d + (ctx->key_sz - vlen), ptr,,
    pub 0: return,
    err:
    pub NULL: ctx->d =,
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn qat_rsa_drop_leading_zeros(ptr: *const c_char, len: *mut c_uint) {
    static void qat_rsa_drop_leading_zeros(const char **ptr, unsigned int *len)
    {
    while (!**ptr && *len) {
    }
    }
#[no_mangle]
unsafe extern "C" fn qat_rsa_setkey_crt(ctx: *mut qat_rsa_ctx, rsa_key: *mut rsa_key) {
    static void qat_rsa_setkey_crt(struct qat_rsa_ctx *ctx, struct rsa_key *rsa_key)
    {
    pub ctx->inst: *mut *mut qat_crypto_instance inst =,
    pub &GET_DEV(inst->accel_dev): *mut *mut device dev =,
    pub ptr: *const c_char,
    pub len: c_uint,
    pub 2: unsigned int half_key_sz = ctx->key_sz /,
// p
    pub rsa_key->p: ptr =,
    pub rsa_key->p_sz: len =,
    pub &len): qat_rsa_drop_leading_zeros(&ptr,,
    if (!len || len > half_key_sz)
    pub err: goto,
    pub GFP_KERNEL): ctx->p = dma_alloc_coherent(dev, half_key_sz, &ctx->dma_p,,
    if (!ctx.p)
    pub err: goto,
    pub len): memcpy(ctx->p + (half_key_sz - len), ptr,,
// q
    pub rsa_key->q: ptr =,
    pub rsa_key->q_sz: len =,
    pub &len): qat_rsa_drop_leading_zeros(&ptr,,
    if (!len || len > half_key_sz)
    pub free_p: goto,
    pub GFP_KERNEL): ctx->q = dma_alloc_coherent(dev, half_key_sz, &ctx->dma_q,,
    if (!ctx.q)
    pub free_p: goto,
    pub len): memcpy(ctx->q + (half_key_sz - len), ptr,,
// dp
    pub rsa_key->dp: ptr =,
    pub rsa_key->dp_sz: len =,
    pub &len): qat_rsa_drop_leading_zeros(&ptr,,
    if (!len || len > half_key_sz)
    pub free_q: goto,
    ctx.dp = dma_alloc_coherent(dev, half_key_sz, &ctx.dma_dp,
    if (!ctx.dp)
    pub free_q: goto,
    pub len): memcpy(ctx->dp + (half_key_sz - len), ptr,,
// dq
    pub rsa_key->dq: ptr =,
    pub rsa_key->dq_sz: len =,
    pub &len): qat_rsa_drop_leading_zeros(&ptr,,
    if (!len || len > half_key_sz)
    pub free_dp: goto,
    ctx.dq = dma_alloc_coherent(dev, half_key_sz, &ctx.dma_dq,
    if (!ctx.dq)
    pub free_dp: goto,
    pub len): memcpy(ctx->dq + (half_key_sz - len), ptr,,
// qinv
    pub rsa_key->qinv: ptr =,
    pub rsa_key->qinv_sz: len =,
    pub &len): qat_rsa_drop_leading_zeros(&ptr,,
    if (!len || len > half_key_sz)
    pub free_dq: goto,
    ctx.qinv = dma_alloc_coherent(dev, half_key_sz, &ctx.dma_qinv,
    if (!ctx.qinv)
    pub free_dq: goto,
    pub len): memcpy(ctx->qinv + (half_key_sz - len), ptr,,
    pub true: ctx->crt_mode =,
    free_dq:
    pub half_key_sz): memset(ctx->dq, '\0',,
    pub ctx->dma_dq): dma_free_coherent(dev, half_key_sz, ctx->dq,,
    pub NULL: ctx->dq =,
    free_dp:
    pub half_key_sz): memset(ctx->dp, '\0',,
    pub ctx->dma_dp): dma_free_coherent(dev, half_key_sz, ctx->dp,,
    pub NULL: ctx->dp =,
    free_q:
    pub half_key_sz): memset(ctx->q, '\0',,
    pub ctx->dma_q): dma_free_coherent(dev, half_key_sz, ctx->q,,
    pub NULL: ctx->q =,
    free_p:
    pub half_key_sz): memset(ctx->p, '\0',,
    pub ctx->dma_p): dma_free_coherent(dev, half_key_sz, ctx->p,,
    pub NULL: ctx->p =,
    err:
    pub false: ctx->crt_mode =,
    }
#[no_mangle]
unsafe extern "C" fn qat_rsa_clear_ctx(dev: *mut device, ctx: *mut qat_rsa_ctx) {
    static void qat_rsa_clear_ctx(struct device *dev, struct qat_rsa_ctx *ctx)
    {
    pub 2: unsigned int half_key_sz = ctx->key_sz /,
// Free the old key if any
    if (ctx.n)
    pub ctx->dma_n): dma_free_coherent(dev, ctx->key_sz, ctx->n,,
    if (ctx.e)
    pub ctx->dma_e): dma_free_coherent(dev, ctx->key_sz, ctx->e,,
    if (ctx.d) {
    pub ctx->key_sz): memset(ctx->d, '\0',,
    pub ctx->dma_d): dma_free_coherent(dev, ctx->key_sz, ctx->d,,
    }
    if (ctx.p) {
    pub half_key_sz): memset(ctx->p, '\0',,
    pub ctx->dma_p): dma_free_coherent(dev, half_key_sz, ctx->p,,
    }
    if (ctx.q) {
    pub half_key_sz): memset(ctx->q, '\0',,
    pub ctx->dma_q): dma_free_coherent(dev, half_key_sz, ctx->q,,
    }
    if (ctx.dp) {
    pub half_key_sz): memset(ctx->dp, '\0',,
    pub ctx->dma_dp): dma_free_coherent(dev, half_key_sz, ctx->dp,,
    }
    if (ctx.dq) {
    pub half_key_sz): memset(ctx->dq, '\0',,
    pub ctx->dma_dq): dma_free_coherent(dev, half_key_sz, ctx->dq,,
    }
    if (ctx.qinv) {
    pub half_key_sz): memset(ctx->qinv, '\0',,
    pub ctx->dma_qinv): dma_free_coherent(dev, half_key_sz, ctx->qinv,,
    }
    pub NULL: ctx->n =,
    pub NULL: ctx->e =,
    pub NULL: ctx->d =,
    pub NULL: ctx->p =,
    pub NULL: ctx->q =,
    pub NULL: ctx->dp =,
    pub NULL: ctx->dq =,
    pub NULL: ctx->qinv =,
    pub false: ctx->crt_mode =,
    pub 0: ctx->key_sz =,
    }
    static int qat_rsa_setkey(struct crypto_akcipher *tfm, const void *key,
    unsigned int keylen, bool private)
    {
    pub akcipher_tfm_ctx(tfm): *mut *mut qat_rsa_ctx ctx =,
    pub &GET_DEV(ctx->inst->accel_dev): *mut *mut device dev =,
    pub rsa_key: rsa_key,
    pub ret: c_int,
    pub ctx): qat_rsa_clear_ctx(dev,,
    if (private)
    pub keylen): ret = rsa_parse_priv_key(&rsa_key, key,,
    else
    pub keylen): ret = rsa_parse_pub_key(&rsa_key, key,,
    if (ret < 0)
    pub free: goto,
    pub rsa_key.n_sz): ret = qat_rsa_set_n(ctx, rsa_key.n,,
    if (ret < 0)
    pub free: goto,
    pub rsa_key.e_sz): ret = qat_rsa_set_e(ctx, rsa_key.e,,
    if (ret < 0)
    pub free: goto,
    if (private) {
    pub rsa_key.d_sz): ret = qat_rsa_set_d(ctx, rsa_key.d,,
    if (ret < 0)
    pub free: goto,
    pub &rsa_key): qat_rsa_setkey_crt(ctx,,
    }
    if (!ctx.n || !ctx.e) {
// invalid key provided
    pub -EINVAL: ret =,
    pub free: goto,
    }
    if (private && !ctx.d) {
// invalid private key provided
    pub -EINVAL: ret =,
    pub free: goto,
    }
    pub 0: return,
    free:
    pub ctx): qat_rsa_clear_ctx(dev,,
    pub ret: return,
    }
    static int qat_rsa_setpubkey(struct crypto_akcipher *tfm, const void *key,
    unsigned int keylen)
    {
    pub false): return qat_rsa_setkey(tfm, key, keylen,,
    }
    static int qat_rsa_setprivkey(struct crypto_akcipher *tfm, const void *key,
    unsigned int keylen)
    {
    pub true): return qat_rsa_setkey(tfm, key, keylen,,
    }
#[no_mangle]
unsafe extern "C" fn qat_rsa_max_size(tfm: *mut crypto_akcipher) -> c_uint {
    static unsigned int qat_rsa_max_size(struct crypto_akcipher *tfm)
    {
    pub akcipher_tfm_ctx(tfm): *mut *mut qat_rsa_ctx ctx =,
    pub ctx->key_sz: return,
    }
#[no_mangle]
unsafe extern "C" fn qat_rsa_init_tfm(tfm: *mut crypto_akcipher) -> c_int {
    static int qat_rsa_init_tfm(struct crypto_akcipher *tfm)
    {
    pub akcipher_tfm_ctx(tfm): *mut *mut qat_rsa_ctx ctx =,
    struct qat_crypto_instance *inst =
    if (!inst)
    pub -EINVAL: return,
    pub 64): akcipher_set_reqsize(tfm, sizeof(struct qat_asym_request) +,
    pub 0: ctx->key_sz =,
    pub inst: ctx->inst =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn qat_rsa_exit_tfm(tfm: *mut crypto_akcipher) {
    static void qat_rsa_exit_tfm(struct crypto_akcipher *tfm)
    {
    pub akcipher_tfm_ctx(tfm): *mut *mut qat_rsa_ctx ctx =,
    pub &GET_DEV(ctx->inst->accel_dev): *mut *mut device dev =,
    pub ctx): qat_rsa_clear_ctx(dev,,
    }
    static struct akcipher_alg rsa = {
    .encrypt = qat_rsa_enc,
    .decrypt = qat_rsa_dec,
    .set_pub_key = qat_rsa_setpubkey,
    .set_priv_key = qat_rsa_setprivkey,
    .max_size = qat_rsa_max_size,
    .init = qat_rsa_init_tfm,
    .exit = qat_rsa_exit_tfm,
    .base = {
    .cra_name = "rsa",
    .cra_driver_name = "qat-rsa",
    .cra_priority = 1000,
    .cra_module = THIS_MODULE,
    .cra_ctxsize = sizeof(struct qat_rsa_ctx),
    },
}

    static struct kpp_alg dh = {
    .set_secret = qat_dh_set_secret,
    .generate_public_key = qat_dh_generate_public_key,
    .compute_shared_secret = qat_dh_compute_shared_secret,
    .max_size = qat_dh_max_size,
    .init = qat_dh_init_tfm,
    .exit = qat_dh_exit_tfm,
    .base = {
    .cra_name = "dh",
    .cra_driver_name = "qat-dh",
    .cra_priority = 1000,
    .cra_module = THIS_MODULE,
    .cra_ctxsize = sizeof(struct qat_dh_ctx),
    .cra_flags = CRYPTO_ALG_NEED_FALLBACK,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn qat_asym_algs_register() -> c_int {
    int qat_asym_algs_register(void)
    {
    let mut ret: c_int = 0;
    mutex_lock(&algs_lock);
    if (++active_devs == 1) {
    rsa.base.cra_flags = 0;
    ret = crypto_register_akcipher(&rsa);
    if (ret)
    goto unlock;
    ret = crypto_register_kpp(&dh);
    }
    unlock:
    mutex_unlock(&algs_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn qat_asym_algs_unregister() {
    void qat_asym_algs_unregister(void)
    {
    mutex_lock(&algs_lock);
    if (--active_devs == 0) {
    crypto_unregister_akcipher(&rsa);
    crypto_unregister_kpp(&dh);
    }
    mutex_unlock(&algs_lock);
    }
