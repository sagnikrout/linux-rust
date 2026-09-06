//! Automatically rewritten from C to Rust
//! Source: crypto/scompress.c
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
// Synchronous Compression operations
//
// Copyright 2015 LG Electronics Inc.
// Copyright (c) 2016, Intel Corporation
// Author: Giovanni Cabiddu <giovanni.cabiddu@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scomp_scratch {
    pub lock: spinlock_t,
    union {
    pub __guarded_by(&lock): *mut *mut void src,
    pub __guarded_by(&lock): unsigned long saddr,
}

    };
    static DEFINE_PER_CPU(struct scomp_scratch, scomp_scratch) = {
    .lock = __SPIN_LOCK_UNLOCKED(scomp_scratch.lock),
    };
    static const struct crypto_type crypto_scomp_type;
    static DEFINE_MUTEX(scomp_lock);
    static int scomp_scratch_users __guarded_by(&scomp_lock);
    static cpumask_t scomp_scratch_want;
    static void scomp_scratch_workfn(struct work_struct *work);
    static DECLARE_WORK(scomp_scratch_work, scomp_scratch_workfn);
    static int __maybe_unused crypto_scomp_report(
    struct sk_buff *skb, struct crypto_alg *alg)
    {
    struct crypto_report_comp rscomp = {
    .type = "scomp",
    };
    return nla_put(skb, CRYPTOCFGA_REPORT_COMPRESS,
    sizeof(rscomp), &rscomp);
    }
    static void __maybe_unused crypto_scomp_show(struct seq_file *m,
    struct crypto_alg *alg)
    {
    seq_puts(m, "type         : scomp\n");
    }
#[no_mangle]
unsafe extern "C" fn crypto_scomp_free_scratches() {
    static void crypto_scomp_free_scratches(void)
    __context_unsafe(/* frees @scratch */)
    {
    struct scomp_scratch *scratch;
    int i;
    for_each_possible_cpu(i) {
    scratch = per_cpu_ptr(&scomp_scratch, i);
    free_page(scratch.saddr);
    scratch.src = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn scomp_alloc_scratch(scratch: *mut scomp_scratch, cpu: c_int) -> c_int {
    static int scomp_alloc_scratch(struct scomp_scratch *scratch, int cpu)
    {
    let mut node: c_int = cpu_to_node(cpu);
    struct page *page;
    page = alloc_pages_node(node, GFP_KERNEL, 0);
    if (!page)
    return -ENOMEM;
    spin_lock_bh(&scratch.lock);
    scratch.src = page_address(page);
    spin_unlock_bh(&scratch.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn scomp_scratch_workfn(work: *mut work_struct) {
    static void scomp_scratch_workfn(struct work_struct *work)
    {
    int cpu;
    for_each_cpu(cpu, &scomp_scratch_want) {
    struct scomp_scratch *scratch;
    scratch = per_cpu_ptr(&scomp_scratch, cpu);
    if (context_unsafe(scratch.src))
    continue;
    if (scomp_alloc_scratch(scratch, cpu))
    break;
    cpumask_clear_cpu(cpu, &scomp_scratch_want);
    }
    }
#[no_mangle]
unsafe extern "C" fn crypto_scomp_alloc_scratches() -> c_int {
    static int crypto_scomp_alloc_scratches(void)
    __context_unsafe(/* allocates @scratch */)
    {
    let mut i: c_uint = cpumask_first(cpu_possible_mask);
    struct scomp_scratch *scratch;
    scratch = per_cpu_ptr(&scomp_scratch, i);
    return scomp_alloc_scratch(scratch, i);
    }
#[no_mangle]
unsafe extern "C" fn crypto_scomp_init_tfm(tfm: *mut crypto_tfm) -> c_int {
    static int crypto_scomp_init_tfm(struct crypto_tfm *tfm)
    {
    struct scomp_alg *alg = crypto_scomp_alg(__crypto_scomp_tfm(tfm));
    let mut ret: c_int = 0;
    mutex_lock(&scomp_lock);
    ret = crypto_acomp_alloc_streams(&alg.streams);
    if (ret)
    goto unlock;
    if (!scomp_scratch_users++) {
    ret = crypto_scomp_alloc_scratches();
    if (ret)
    scomp_scratch_users--;
    }
    unlock:
    mutex_unlock(&scomp_lock);
    return ret;
    }

    static struct scomp_scratch *_scomp_lock_scratch(void) __acquires_ret
    {
    let mut cpu: c_int = raw_smp_processor_id();
    struct scomp_scratch *scratch;
    scratch = per_cpu_ptr(&scomp_scratch, cpu);
    spin_lock(&scratch.lock);
    if (likely(scratch.src))
    return scratch;
    spin_unlock(&scratch.lock);
    cpumask_set_cpu(cpu, &scomp_scratch_want);
    schedule_work(&scomp_scratch_work);
    scratch = per_cpu_ptr(&scomp_scratch, cpumask_first(cpu_possible_mask));
    spin_lock(&scratch.lock);
    return scratch;
    }
#[no_mangle]
pub unsafe extern "C" fn scomp_unlock_scratch(scratch: *mut scomp_scratch) {
    static inline void scomp_unlock_scratch(struct scomp_scratch *scratch)
    __releases(&scratch.lock)
    {
    spin_unlock(&scratch.lock);
    }
#[no_mangle]
unsafe extern "C" fn scomp_acomp_comp_decomp(req: *mut acomp_req, dir: c_int) -> c_int {
    static int scomp_acomp_comp_decomp(struct acomp_req *req, int dir)
    {
    struct crypto_acomp *tfm = crypto_acomp_reqtfm(req);
    struct crypto_scomp **tfm_ctx = acomp_tfm_ctx(tfm);
    let mut src_isvirt: bool = acomp_request_src_isvirt(req);
    let mut dst_isvirt: bool = acomp_request_dst_isvirt(req);
    struct crypto_scomp *scomp = *tfm_ctx;
    let mut slen: c_uint = req.slen;
    let mut dlen: c_uint = req.dlen;
    struct page *spage, *dpage;
    unsigned int n;
    const u8 *src;
    size_t soff;
    size_t doff;
    u8 *dst;
    int ret;
    if (!req.src || !slen)
    return -EINVAL;
    if (!req.dst || !dlen)
    return -EINVAL;
    if (dst_isvirt)
    dst = req.dvirt;
    else {
    if (dlen <= req.dst.length) {
    dpage = sg_page(req.dst);
    doff = req.dst.offset;
    } else
    return -ENOSYS;
    dpage += doff / PAGE_SIZE;
    doff = offset_in_page(doff);
    n = (dlen - 1) / PAGE_SIZE;
    n += (offset_in_page(dlen - 1) + doff) / PAGE_SIZE;
    if (PageHighMem(dpage + n) &&
    size_add(doff, dlen) > PAGE_SIZE)
    return -ENOSYS;
    dst = kmap_local_page(dpage) + doff;
    }
    if (src_isvirt)
    src = req.svirt;
    else {
    src = core::ptr::null_mut();
    do {
    if (slen <= req.src.length) {
    spage = sg_page(req.src);
    soff = req.src.offset;
    } else
    break;
    spage = spage + soff / PAGE_SIZE;
    soff = offset_in_page(soff);
    n = (slen - 1) / PAGE_SIZE;
    n += (offset_in_page(slen - 1) + soff) / PAGE_SIZE;
    if (PageHighMem(spage + n) &&
    size_add(soff, slen) > PAGE_SIZE)
    break;
    src = kmap_local_page(spage) + soff;
    } while (0);
    }
    struct crypto_acomp_stream *stream = crypto_acomp_lock_stream_bh(&crypto_scomp_alg(scomp).streams);
    if (!src_isvirt && !src) {
    struct scomp_scratch *scratch = scomp_lock_scratch();
    const u8 *src = scratch.src;
    memcpy_from_sglist(scratch.src, req.src, 0, slen);
    if (dir)
    ret = crypto_scomp_compress(scomp, src, slen,
    dst, &dlen, stream.ctx);
    else
    ret = crypto_scomp_decompress(scomp, src, slen,
    dst, &dlen, stream.ctx);
    scomp_unlock_scratch(scratch);
    } else if (dir)
    ret = crypto_scomp_compress(scomp, src, slen,
    dst, &dlen, stream.ctx);
    else
    ret = crypto_scomp_decompress(scomp, src, slen,
    dst, &dlen, stream.ctx);
    crypto_acomp_unlock_stream_bh(stream);
    req.dlen = dlen;
    if (!src_isvirt && src)
    kunmap_local(src);
    if (!dst_isvirt) {
    kunmap_local(dst);
    dlen += doff;
    for (;;) {
    flush_dcache_page(dpage);
    if (dlen <= PAGE_SIZE)
    break;
    dlen -= PAGE_SIZE;
    dpage++;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn scomp_acomp_compress(req: *mut acomp_req) -> c_int {
    static int scomp_acomp_compress(struct acomp_req *req)
    {
    return scomp_acomp_comp_decomp(req, 1);
    }
#[no_mangle]
unsafe extern "C" fn scomp_acomp_decompress(req: *mut acomp_req) -> c_int {
    static int scomp_acomp_decompress(struct acomp_req *req)
    {
    return scomp_acomp_comp_decomp(req, 0);
    }
#[no_mangle]
unsafe extern "C" fn crypto_exit_scomp_ops_async(tfm: *mut crypto_tfm) {
    static void crypto_exit_scomp_ops_async(struct crypto_tfm *tfm)
    {
    struct crypto_scomp **ctx = crypto_tfm_ctx(tfm);
    crypto_free_scomp(*ctx);
    flush_work(&scomp_scratch_work);
    mutex_lock(&scomp_lock);
    if (!--scomp_scratch_users)
    crypto_scomp_free_scratches();
    mutex_unlock(&scomp_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_init_scomp_ops_async(tfm: *mut crypto_tfm) -> c_int {
    int crypto_init_scomp_ops_async(struct crypto_tfm *tfm)
    {
    struct crypto_alg *calg = tfm.__crt_alg;
    struct crypto_acomp *crt = __crypto_acomp_tfm(tfm);
    struct crypto_scomp **ctx = crypto_tfm_ctx(tfm);
    struct crypto_scomp *scomp;
    if (!crypto_mod_get(calg))
    return -EAGAIN;
    scomp = crypto_create_tfm(calg, &crypto_scomp_type);
    if (IS_ERR(scomp)) {
    crypto_mod_put(calg);
    return PTR_ERR(scomp);
    }
// ctx = scomp;
    tfm.exit = crypto_exit_scomp_ops_async;
    crt.compress = scomp_acomp_compress;
    crt.decompress = scomp_acomp_decompress;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_scomp_destroy(alg: *mut crypto_alg) {
    static void crypto_scomp_destroy(struct crypto_alg *alg)
    {
    struct scomp_alg *scomp = __crypto_scomp_alg(alg);
    crypto_acomp_free_streams(&scomp.streams);
    }
    static const struct crypto_type crypto_scomp_type = {
    .extsize = crypto_alg_extsize,
    .init_tfm = crypto_scomp_init_tfm,
    .destroy = crypto_scomp_destroy,

    .show = crypto_scomp_show,

    .report = crypto_scomp_report,

    .maskclear = ~CRYPTO_ALG_TYPE_MASK,
    .maskset = CRYPTO_ALG_TYPE_MASK,
    .type = CRYPTO_ALG_TYPE_SCOMPRESS,
    .tfmsize = offsetof(struct crypto_scomp, base),
    .algsize = offsetof(struct scomp_alg, base),
    };
#[no_mangle]
unsafe extern "C" fn scomp_prepare_alg(alg: *mut scomp_alg) {
    static void scomp_prepare_alg(struct scomp_alg *alg)
    {
    struct crypto_alg *base = &alg.calg.base;
    comp_prepare_alg(&alg.calg);
    base.cra_flags |= CRYPTO_ALG_REQ_VIRT;
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_register_scomp(alg: *mut scomp_alg) -> c_int {
    int crypto_register_scomp(struct scomp_alg *alg)
    {
    struct crypto_alg *base = &alg.calg.base;
    scomp_prepare_alg(alg);
    base.cra_type = &crypto_scomp_type;
    base.cra_flags |= CRYPTO_ALG_TYPE_SCOMPRESS;
    return crypto_register_alg(base);
    }
    EXPORT_SYMBOL_GPL(crypto_register_scomp);
#[no_mangle]
pub unsafe extern "C" fn crypto_unregister_scomp(alg: *mut scomp_alg) {
    void crypto_unregister_scomp(struct scomp_alg *alg)
    {
    crypto_unregister_alg(&alg.base);
    }
    EXPORT_SYMBOL_GPL(crypto_unregister_scomp);
#[no_mangle]
pub unsafe extern "C" fn crypto_register_scomps(algs: *mut scomp_alg, count: c_int) -> c_int {
    int crypto_register_scomps(struct scomp_alg *algs, int count)
    {
    int i, ret;
    for (i = 0; i < count; i++) {
    ret = crypto_register_scomp(&algs[i]);
    if (ret) {
    crypto_unregister_scomps(algs, i);
    return ret;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(crypto_register_scomps);
#[no_mangle]
pub unsafe extern "C" fn crypto_unregister_scomps(algs: *mut scomp_alg, count: c_int) {
    void crypto_unregister_scomps(struct scomp_alg *algs, int count)
    {
    int i;
    for (i = count - 1; i >= 0; --i)
    crypto_unregister_scomp(&algs[i]);
    }
    EXPORT_SYMBOL_GPL(crypto_unregister_scomps);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Synchronous compression type");
