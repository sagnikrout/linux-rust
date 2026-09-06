//! Automatically rewritten from C to Rust
//! Source: crypto/acompress.c
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
// Asynchronous Compression operations
//
// Copyright (c) 2016, Intel Corporation
// Authors: Weigang Li <weigang.li@intel.com>
// Giovanni Cabiddu <giovanni.cabiddu@intel.com>
//

    struct crypto_scomp;
    enum {
    ACOMP_WALK_SLEEP = 1 << 0,
    ACOMP_WALK_SRC_LINEAR = 1 << 1,
    ACOMP_WALK_DST_LINEAR = 1 << 2,
    };
    static const struct crypto_type crypto_acomp_type;
    static void acomp_reqchain_done(void *data, int err);
    static inline struct acomp_alg *__crypto_acomp_alg(struct crypto_alg *alg)
    {
    return container_of(alg, struct acomp_alg, calg.base);
    }
    static inline struct acomp_alg *crypto_acomp_alg(struct crypto_acomp *tfm)
    {
    return __crypto_acomp_alg(crypto_acomp_tfm(tfm).__crt_alg);
    }
    static int __maybe_unused crypto_acomp_report(
    struct sk_buff *skb, struct crypto_alg *alg)
    {
    struct crypto_report_acomp racomp = {
    .type = "acomp",
    };
    return nla_put(skb, CRYPTOCFGA_REPORT_ACOMP, sizeof(racomp), &racomp);
    }
    static void __maybe_unused crypto_acomp_show(struct seq_file *m,
    struct crypto_alg *alg)
    {
    seq_puts(m, "type         : acomp\n");
    }
#[no_mangle]
unsafe extern "C" fn crypto_acomp_exit_tfm(tfm: *mut crypto_tfm) {
    static void crypto_acomp_exit_tfm(struct crypto_tfm *tfm)
    {
    struct crypto_acomp *acomp = __crypto_acomp_tfm(tfm);
    struct acomp_alg *alg = crypto_acomp_alg(acomp);
    if (alg.exit)
    alg.exit(acomp);
    if (acomp_is_async(acomp))
    crypto_free_acomp(crypto_acomp_fb(acomp));
    }
#[no_mangle]
unsafe extern "C" fn crypto_acomp_init_tfm(tfm: *mut crypto_tfm) -> c_int {
    static int crypto_acomp_init_tfm(struct crypto_tfm *tfm)
    {
    struct crypto_acomp *acomp = __crypto_acomp_tfm(tfm);
    struct acomp_alg *alg = crypto_acomp_alg(acomp);
    struct crypto_acomp *fb = core::ptr::null_mut();
    int err;
    if (tfm.__crt_alg.cra_type != &crypto_acomp_type)
    return crypto_init_scomp_ops_async(tfm);
    if (acomp_is_async(acomp)) {
    fb = crypto_alloc_acomp(crypto_acomp_alg_name(acomp), 0,
    CRYPTO_ALG_ASYNC);
    if (IS_ERR(fb))
    return PTR_ERR(fb);
    err = -EINVAL;
    if (crypto_acomp_reqsize(fb) > MAX_SYNC_COMP_REQSIZE)
    goto out_free_fb;
    tfm.fb = crypto_acomp_tfm(fb);
    }
    acomp.compress = alg.compress;
    acomp.decompress = alg.decompress;
    acomp.reqsize = alg.base.cra_reqsize;
    acomp.base.exit = crypto_acomp_exit_tfm;
    if (!alg.init)
    return 0;
    err = alg.init(acomp);
    if (err)
    goto out_free_fb;
    return 0;
    out_free_fb:
    crypto_free_acomp(fb);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn crypto_acomp_extsize(alg: *mut crypto_alg) -> c_uint {
    static unsigned int crypto_acomp_extsize(struct crypto_alg *alg)
    {
    let mut extsize: c_int = crypto_alg_extsize(alg);
    if (alg.cra_type != &crypto_acomp_type)
    extsize += sizeof(struct crypto_scomp *);
    return extsize;
    }
    static const struct crypto_type crypto_acomp_type = {
    .extsize = crypto_acomp_extsize,
    .init_tfm = crypto_acomp_init_tfm,

    .show = crypto_acomp_show,

    .report = crypto_acomp_report,

    .maskclear = ~CRYPTO_ALG_TYPE_MASK,
    .maskset = CRYPTO_ALG_TYPE_ACOMPRESS_MASK,
    .type = CRYPTO_ALG_TYPE_ACOMPRESS,
    .tfmsize = offsetof(struct crypto_acomp, base),
    .algsize = offsetof(struct acomp_alg, base),
    };
    struct crypto_acomp *crypto_alloc_acomp(const char *alg_name, u32 type,
    u32 mask)
    {
    return crypto_alloc_tfm(alg_name, &crypto_acomp_type, type, mask);
    }
    EXPORT_SYMBOL_GPL(crypto_alloc_acomp);
    struct crypto_acomp *crypto_alloc_acomp_node(const char *alg_name, u32 type,
    u32 mask, int node)
    {
    return crypto_alloc_tfm_node(alg_name, &crypto_acomp_type, type, mask,
    node);
    }
    EXPORT_SYMBOL_GPL(crypto_alloc_acomp_node);
#[no_mangle]
unsafe extern "C" fn acomp_save_req(req: *mut acomp_req, cplt: crypto_completion_t) {
    static void acomp_save_req(struct acomp_req *req, crypto_completion_t cplt)
    {
    struct acomp_req_chain *state = &req.chain;
    state.compl = req.base.complete;
    state.data = req.base.data;
    req.base.complete = cplt;
    req.base.data = req;
    }
#[no_mangle]
unsafe extern "C" fn acomp_restore_req(req: *mut acomp_req) {
    static void acomp_restore_req(struct acomp_req *req)
    {
    req.base.complete = req.chain.compl;
    req.base.data = req.chain.data;
    }
#[no_mangle]
unsafe extern "C" fn acomp_reqchain_virt(req: *mut acomp_req) {
    static void acomp_reqchain_virt(struct acomp_req *req)
    {
    struct acomp_req_chain *state = &req.chain;
    let mut slen: c_uint = req.slen;
    let mut dlen: c_uint = req.dlen;
    if (state.flags & CRYPTO_ACOMP_REQ_SRC_VIRT)
    acomp_request_set_src_dma(req, state.src, slen);
    if (state.flags & CRYPTO_ACOMP_REQ_DST_VIRT)
    acomp_request_set_dst_dma(req, state.dst, dlen);
    }
#[no_mangle]
unsafe extern "C" fn acomp_virt_to_sg(req: *mut acomp_req) {
    static void acomp_virt_to_sg(struct acomp_req *req)
    {
    struct acomp_req_chain *state = &req.chain;
    state.flags = req.base.flags & (CRYPTO_ACOMP_REQ_SRC_VIRT |
    CRYPTO_ACOMP_REQ_DST_VIRT);
    if (acomp_request_src_isvirt(req)) {
    let mut slen: c_uint = req.slen;
    const u8 *svirt = req.svirt;
    state.src = svirt;
    sg_init_one(&state.ssg, svirt, slen);
    acomp_request_set_src_sg(req, &state.ssg, slen);
    }
    if (acomp_request_dst_isvirt(req)) {
    let mut dlen: c_uint = req.dlen;
    u8 *dvirt = req.dvirt;
    state.dst = dvirt;
    sg_init_one(&state.dsg, dvirt, dlen);
    acomp_request_set_dst_sg(req, &state.dsg, dlen);
    }
    }
#[no_mangle]
unsafe extern "C" fn acomp_do_nondma(req: *mut acomp_req, comp: bool) -> c_int {
    static int acomp_do_nondma(struct acomp_req *req, bool comp)
    {
    ACOMP_FBREQ_ON_STACK(fbreq, req);
    int err;
    if (comp)
    err = crypto_acomp_compress(fbreq);
    else
    err = crypto_acomp_decompress(fbreq);
    req.dlen = fbreq.dlen;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn acomp_do_one_req(req: *mut acomp_req, comp: bool) -> c_int {
    static int acomp_do_one_req(struct acomp_req *req, bool comp)
    {
    if (acomp_request_isnondma(req))
    return acomp_do_nondma(req, comp);
    acomp_virt_to_sg(req);
    return comp ? crypto_acomp_reqtfm(req).compress(req) :
    crypto_acomp_reqtfm(req).decompress(req);
    }
#[no_mangle]
unsafe extern "C" fn acomp_reqchain_finish(req: *mut acomp_req, err: c_int) -> c_int {
    static int acomp_reqchain_finish(struct acomp_req *req, int err)
    {
    acomp_reqchain_virt(req);
    acomp_restore_req(req);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn acomp_reqchain_done(data: *mut c_void, err: c_int) {
    static void acomp_reqchain_done(void *data, int err)
    {
    struct acomp_req *req = data;
    crypto_completion_t compl;
    compl = req.chain.compl;
    data = req.chain.data;
    if (err == -EINPROGRESS)
    goto notify;
    err = acomp_reqchain_finish(req, err);
    notify:
    compl(data, err);
    }
#[no_mangle]
unsafe extern "C" fn acomp_do_req_chain(req: *mut acomp_req, comp: bool) -> c_int {
    static int acomp_do_req_chain(struct acomp_req *req, bool comp)
    {
    int err;
    acomp_save_req(req, acomp_reqchain_done);
    err = acomp_do_one_req(req, comp);
    if (err == -EBUSY || err == -EINPROGRESS)
    return err;
    return acomp_reqchain_finish(req, err);
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_acomp_compress(req: *mut acomp_req) -> c_int {
    int crypto_acomp_compress(struct acomp_req *req)
    {
    struct crypto_acomp *tfm = crypto_acomp_reqtfm(req);
    if (acomp_req_on_stack(req) && acomp_is_async(tfm))
    return -EAGAIN;
    if (crypto_acomp_req_virt(tfm) || acomp_request_issg(req))
    return crypto_acomp_reqtfm(req).compress(req);
    return acomp_do_req_chain(req, true);
    }
    EXPORT_SYMBOL_GPL(crypto_acomp_compress);
#[no_mangle]
pub unsafe extern "C" fn crypto_acomp_decompress(req: *mut acomp_req) -> c_int {
    int crypto_acomp_decompress(struct acomp_req *req)
    {
    struct crypto_acomp *tfm = crypto_acomp_reqtfm(req);
    if (acomp_req_on_stack(req) && acomp_is_async(tfm))
    return -EAGAIN;
    if (crypto_acomp_req_virt(tfm) || acomp_request_issg(req))
    return crypto_acomp_reqtfm(req).decompress(req);
    return acomp_do_req_chain(req, false);
    }
    EXPORT_SYMBOL_GPL(crypto_acomp_decompress);
#[no_mangle]
pub unsafe extern "C" fn comp_prepare_alg(alg: *mut comp_alg_common) {
    void comp_prepare_alg(struct comp_alg_common *alg)
    {
    struct crypto_alg *base = &alg.base;
    base.cra_flags &= ~CRYPTO_ALG_TYPE_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_register_acomp(alg: *mut acomp_alg) -> c_int {
    int crypto_register_acomp(struct acomp_alg *alg)
    {
    struct crypto_alg *base = &alg.calg.base;
    comp_prepare_alg(&alg.calg);
    base.cra_type = &crypto_acomp_type;
    base.cra_flags |= CRYPTO_ALG_TYPE_ACOMPRESS;
    return crypto_register_alg(base);
    }
    EXPORT_SYMBOL_GPL(crypto_register_acomp);
#[no_mangle]
pub unsafe extern "C" fn crypto_unregister_acomp(alg: *mut acomp_alg) {
    void crypto_unregister_acomp(struct acomp_alg *alg)
    {
    crypto_unregister_alg(&alg.base);
    }
    EXPORT_SYMBOL_GPL(crypto_unregister_acomp);
#[no_mangle]
pub unsafe extern "C" fn crypto_register_acomps(algs: *mut acomp_alg, count: c_int) -> c_int {
    int crypto_register_acomps(struct acomp_alg *algs, int count)
    {
    int i, ret;
    for (i = 0; i < count; i++) {
    ret = crypto_register_acomp(&algs[i]);
    if (ret) {
    crypto_unregister_acomps(algs, i);
    return ret;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(crypto_register_acomps);
#[no_mangle]
pub unsafe extern "C" fn crypto_unregister_acomps(algs: *mut acomp_alg, count: c_int) {
    void crypto_unregister_acomps(struct acomp_alg *algs, int count)
    {
    int i;
    for (i = count - 1; i >= 0; --i)
    crypto_unregister_acomp(&algs[i]);
    }
    EXPORT_SYMBOL_GPL(crypto_unregister_acomps);
#[no_mangle]
unsafe extern "C" fn acomp_stream_workfn(work: *mut work_struct) {
    static void acomp_stream_workfn(struct work_struct *work)
    {
    struct crypto_acomp_streams *s =
    container_of(work, struct crypto_acomp_streams, stream_work);
    struct crypto_acomp_stream __percpu *streams = s.streams;
    int cpu;
    for_each_cpu(cpu, &s.stream_want) {
    struct crypto_acomp_stream *ps;
    void *ctx;
    ps = per_cpu_ptr(streams, cpu);
    if (ps.ctx)
    continue;
    ctx = s.alloc_ctx();
    if (IS_ERR(ctx))
    break;
    spin_lock_bh(&ps.lock);
    ps.ctx = ctx;
    spin_unlock_bh(&ps.lock);
    cpumask_clear_cpu(cpu, &s.stream_want);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_acomp_free_streams(s: *mut crypto_acomp_streams) {
    void crypto_acomp_free_streams(struct crypto_acomp_streams *s)
    {
    struct crypto_acomp_stream __percpu *streams = s.streams;
    void (*free_ctx)(void *);
    int i;
    s.streams = core::ptr::null_mut();
    if (!streams)
    return;
    cancel_work_sync(&s.stream_work);
    free_ctx = s.free_ctx;
    for_each_possible_cpu(i) {
    struct crypto_acomp_stream *ps = per_cpu_ptr(streams, i);
    if (!ps.ctx)
    continue;
    free_ctx(ps.ctx);
    }
    free_percpu(streams);
    }
    EXPORT_SYMBOL_GPL(crypto_acomp_free_streams);
#[no_mangle]
pub unsafe extern "C" fn crypto_acomp_alloc_streams(s: *mut crypto_acomp_streams) -> c_int {
    int crypto_acomp_alloc_streams(struct crypto_acomp_streams *s)
    {
    struct crypto_acomp_stream __percpu *streams;
    struct crypto_acomp_stream *ps;
    unsigned int i;
    void *ctx;
    if (s.streams)
    return 0;
    streams = alloc_percpu(struct crypto_acomp_stream);
    if (!streams)
    return -ENOMEM;
    ctx = s.alloc_ctx();
    if (IS_ERR(ctx)) {
    free_percpu(streams);
    return PTR_ERR(ctx);
    }
    i = cpumask_first(cpu_possible_mask);
    ps = per_cpu_ptr(streams, i);
    ps.ctx = ctx;
    for_each_possible_cpu(i) {
    ps = per_cpu_ptr(streams, i);
    spin_lock_init(&ps.lock);
    }
    s.streams = streams;
    INIT_WORK(&s.stream_work, acomp_stream_workfn);
    return 0;
    }
    EXPORT_SYMBOL_GPL(crypto_acomp_alloc_streams);
    struct crypto_acomp_stream *_crypto_acomp_lock_stream_bh(
    struct crypto_acomp_streams *s)
    {
    struct crypto_acomp_stream __percpu *streams = s.streams;
    let mut cpu: c_int = raw_smp_processor_id();
    struct crypto_acomp_stream *ps;
    ps = per_cpu_ptr(streams, cpu);
    spin_lock_bh(&ps.lock);
    if (likely(ps.ctx))
    return ps;
    spin_unlock(&ps.lock);
    cpumask_set_cpu(cpu, &s.stream_want);
    schedule_work(&s.stream_work);
    ps = per_cpu_ptr(streams, cpumask_first(cpu_possible_mask));
    spin_lock(&ps.lock);
    return ps;
    }
    EXPORT_SYMBOL_GPL(_crypto_acomp_lock_stream_bh);
#[no_mangle]
pub unsafe extern "C" fn acomp_walk_done_src(walk: *mut acomp_walk, used: c_int) {
    void acomp_walk_done_src(struct acomp_walk *walk, int used)
    {
    walk.slen -= used;
    if ((walk.flags & ACOMP_WALK_SRC_LINEAR))
    scatterwalk_advance(&walk.in, used);
    else
    scatterwalk_done_src(&walk.in, used);
    if ((walk.flags & ACOMP_WALK_SLEEP))
    cond_resched();
    }
    EXPORT_SYMBOL_GPL(acomp_walk_done_src);
#[no_mangle]
pub unsafe extern "C" fn acomp_walk_done_dst(walk: *mut acomp_walk, used: c_int) {
    void acomp_walk_done_dst(struct acomp_walk *walk, int used)
    {
    walk.dlen -= used;
    if ((walk.flags & ACOMP_WALK_DST_LINEAR))
    scatterwalk_advance(&walk.out, used);
    else
    scatterwalk_done_dst(&walk.out, used);
    if ((walk.flags & ACOMP_WALK_SLEEP))
    cond_resched();
    }
    EXPORT_SYMBOL_GPL(acomp_walk_done_dst);
#[no_mangle]
pub unsafe extern "C" fn acomp_walk_next_src(walk: *mut acomp_walk) -> c_int {
    int acomp_walk_next_src(struct acomp_walk *walk)
    {
    let mut slen: c_uint = walk.slen;
    let mut max: c_uint = UINT_MAX;
    if (!preempt_model_preemptible() && (walk.flags & ACOMP_WALK_SLEEP))
    max = PAGE_SIZE;
    if ((walk.flags & ACOMP_WALK_SRC_LINEAR)) {
    walk.in.__addr = (void *)(((u8 *)walk.in.sg) +
    walk.in.offset);
    return min(slen, max);
    }
    return slen ? scatterwalk_next(&walk.in, slen) : 0;
    }
    EXPORT_SYMBOL_GPL(acomp_walk_next_src);
#[no_mangle]
pub unsafe extern "C" fn acomp_walk_next_dst(walk: *mut acomp_walk) -> c_int {
    int acomp_walk_next_dst(struct acomp_walk *walk)
    {
    let mut dlen: c_uint = walk.dlen;
    let mut max: c_uint = UINT_MAX;
    if (!preempt_model_preemptible() && (walk.flags & ACOMP_WALK_SLEEP))
    max = PAGE_SIZE;
    if ((walk.flags & ACOMP_WALK_DST_LINEAR)) {
    walk.out.__addr = (void *)(((u8 *)walk.out.sg) +
    walk.out.offset);
    return min(dlen, max);
    }
    return dlen ? scatterwalk_next(&walk.out, dlen) : 0;
    }
    EXPORT_SYMBOL_GPL(acomp_walk_next_dst);
    int acomp_walk_virt(struct acomp_walk *__restrict walk,
    struct acomp_req *__restrict req, bool atomic)
    {
    struct scatterlist *src = req.src;
    struct scatterlist *dst = req.dst;
    walk.slen = req.slen;
    walk.dlen = req.dlen;
    if (!walk.slen || !walk.dlen)
    return -EINVAL;
    walk.flags = 0;
    if ((req.base.flags & CRYPTO_TFM_REQ_MAY_SLEEP) && !atomic)
    walk.flags |= ACOMP_WALK_SLEEP;
    if ((req.base.flags & CRYPTO_ACOMP_REQ_SRC_VIRT))
    walk.flags |= ACOMP_WALK_SRC_LINEAR;
    if ((req.base.flags & CRYPTO_ACOMP_REQ_DST_VIRT))
    walk.flags |= ACOMP_WALK_DST_LINEAR;
    if ((walk.flags & ACOMP_WALK_SRC_LINEAR)) {
    walk.in.sg = (void *)req.svirt;
    walk.in.offset = 0;
    } else
    scatterwalk_start(&walk.in, src);
    if ((walk.flags & ACOMP_WALK_DST_LINEAR)) {
    walk.out.sg = (void *)req.dvirt;
    walk.out.offset = 0;
    } else
    scatterwalk_start(&walk.out, dst);
    return 0;
    }
    EXPORT_SYMBOL_GPL(acomp_walk_virt);
    struct acomp_req *acomp_request_clone(struct acomp_req *req,
    size_t total, gfp_t gfp)
    {
    struct crypto_tfm *tfm = req.base.tfm;
    struct acomp_req *nreq;
    size_t len;
    len = sizeof(*req) +
    crypto_acomp_reqsize(crypto_acomp_reqtfm(req));
    len = ALIGN(len, CRYPTO_MINALIGN);
    nreq = kzalloc(len, gfp);
    if (!nreq) {
    req.base.tfm = tfm.fb;
    return req;
    }
    memcpy(nreq, req, sizeof(*req));
    nreq.base.flags &= ~CRYPTO_TFM_REQ_ON_STACK;
    if (req.src == &req.chain.ssg)
    nreq.src = &nreq.chain.ssg;
    if (req.dst == &req.chain.dsg)
    nreq.dst = &nreq.chain.dsg;
    return nreq;
    }
    EXPORT_SYMBOL_GPL(acomp_request_clone);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Asynchronous compression type");
