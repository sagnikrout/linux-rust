//! Automatically rewritten from C to Rust
//! Source: crypto/simd.c
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
// Shared crypto simd helpers
//
// Copyright (c) 2012 Jussi Kivilinna <jussi.kivilinna@mbnet.fi>
// Copyright (c) 2016 Herbert Xu <herbert@gondor.apana.org.au>
// Copyright (c) 2019 Google LLC
//
// Based on aesni-intel_glue.c by:
// Copyright (C) 2008, Intel Corp.
// Author: Huang Ying <ying.huang@intel.com>
//
// Shared crypto SIMD helpers.  These functions dynamically create and register
// an AEAD algorithm that wraps another, internal algorithm.  The wrapper
// ensures that the internal algorithm is only executed in a context where SIMD
// instructions are usable, i.e. where may_use_simd() returns true.  If SIMD is
// already usable, the wrapper directly calls the internal algorithm.  Otherwise
// it defers execution to a workqueue via cryptd.
//
// This is an alternative to the internal algorithm implementing a fallback for
// the !may_use_simd() case itself.
//
// Note that the wrapper algorithm is asynchronous, i.e. it has the
// CRYPTO_ALG_ASYNC flag set.  Therefore it won't be found by users who
// explicitly allocate a synchronous algorithm.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_aead_alg {
    pub ialg_name: *const c_char,
    pub alg: aead_alg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_aead_ctx {
    pub cryptd_tfm: *mut cryptd_aead,
}

    static int simd_aead_setkey(struct crypto_aead *tfm, const u8 *key,
    unsigned int key_len)
    {
    struct simd_aead_ctx *ctx = crypto_aead_ctx(tfm);
    struct crypto_aead *child = &ctx.cryptd_tfm.base;
    crypto_aead_clear_flags(child, CRYPTO_TFM_REQ_MASK);
    crypto_aead_set_flags(child, crypto_aead_get_flags(tfm) &
    CRYPTO_TFM_REQ_MASK);
    return crypto_aead_setkey(child, key, key_len);
    }
#[no_mangle]
unsafe extern "C" fn simd_aead_setauthsize(tfm: *mut crypto_aead, authsize: c_uint) -> c_int {
    static int simd_aead_setauthsize(struct crypto_aead *tfm, unsigned int authsize)
    {
    struct simd_aead_ctx *ctx = crypto_aead_ctx(tfm);
    struct crypto_aead *child = &ctx.cryptd_tfm.base;
    return crypto_aead_setauthsize(child, authsize);
    }
#[no_mangle]
unsafe extern "C" fn simd_aead_encrypt(req: *mut aead_request) -> c_int {
    static int simd_aead_encrypt(struct aead_request *req)
    {
    struct crypto_aead *tfm = crypto_aead_reqtfm(req);
    struct simd_aead_ctx *ctx = crypto_aead_ctx(tfm);
    struct aead_request *subreq;
    struct crypto_aead *child;
    subreq = aead_request_ctx(req);
// subreq = *req;
    if (!crypto_simd_usable() ||
    (in_atomic() && cryptd_aead_queued(ctx.cryptd_tfm)))
    child = &ctx.cryptd_tfm.base;
    else
    child = cryptd_aead_child(ctx.cryptd_tfm);
    aead_request_set_tfm(subreq, child);
    return crypto_aead_encrypt(subreq);
    }
#[no_mangle]
unsafe extern "C" fn simd_aead_decrypt(req: *mut aead_request) -> c_int {
    static int simd_aead_decrypt(struct aead_request *req)
    {
    struct crypto_aead *tfm = crypto_aead_reqtfm(req);
    struct simd_aead_ctx *ctx = crypto_aead_ctx(tfm);
    struct aead_request *subreq;
    struct crypto_aead *child;
    subreq = aead_request_ctx(req);
// subreq = *req;
    if (!crypto_simd_usable() ||
    (in_atomic() && cryptd_aead_queued(ctx.cryptd_tfm)))
    child = &ctx.cryptd_tfm.base;
    else
    child = cryptd_aead_child(ctx.cryptd_tfm);
    aead_request_set_tfm(subreq, child);
    return crypto_aead_decrypt(subreq);
    }
#[no_mangle]
unsafe extern "C" fn simd_aead_exit(tfm: *mut crypto_aead) {
    static void simd_aead_exit(struct crypto_aead *tfm)
    {
    struct simd_aead_ctx *ctx = crypto_aead_ctx(tfm);
    cryptd_free_aead(ctx.cryptd_tfm);
    }
#[no_mangle]
unsafe extern "C" fn simd_aead_init(tfm: *mut crypto_aead) -> c_int {
    static int simd_aead_init(struct crypto_aead *tfm)
    {
    struct simd_aead_ctx *ctx = crypto_aead_ctx(tfm);
    struct cryptd_aead *cryptd_tfm;
    struct simd_aead_alg *salg;
    struct aead_alg *alg;
    unsigned reqsize;
    alg = crypto_aead_alg(tfm);
    salg = container_of(alg, struct simd_aead_alg, alg);
    cryptd_tfm = cryptd_alloc_aead(salg.ialg_name, CRYPTO_ALG_INTERNAL,
    CRYPTO_ALG_INTERNAL);
    if (IS_ERR(cryptd_tfm))
    return PTR_ERR(cryptd_tfm);
    ctx.cryptd_tfm = cryptd_tfm;
    reqsize = max(crypto_aead_reqsize(cryptd_aead_child(cryptd_tfm)),
    crypto_aead_reqsize(&cryptd_tfm.base));
    reqsize += sizeof(struct aead_request);
    crypto_aead_set_reqsize(tfm, reqsize);
    return 0;
    }
    static struct simd_aead_alg *simd_aead_create_compat(struct aead_alg *ialg,
    const char *algname,
    const char *drvname,
    const char *basename)
    {
    struct simd_aead_alg *salg;
    struct aead_alg *alg;
    int err;
    salg = kzalloc_obj(*salg);
    if (!salg) {
    salg = ERR_PTR(-ENOMEM);
    goto out;
    }
    salg.ialg_name = basename;
    alg = &salg.alg;
    err = -ENAMETOOLONG;
    if (snprintf(alg.base.cra_name, CRYPTO_MAX_ALG_NAME, "%s", algname) >=
    CRYPTO_MAX_ALG_NAME)
    goto out_free_salg;
    if (snprintf(alg.base.cra_driver_name, CRYPTO_MAX_ALG_NAME, "%s",
    drvname) >= CRYPTO_MAX_ALG_NAME)
    goto out_free_salg;
    alg.base.cra_flags = CRYPTO_ALG_ASYNC |
    (ialg.base.cra_flags & CRYPTO_ALG_INHERITED_FLAGS);
    alg.base.cra_priority = ialg.base.cra_priority;
    alg.base.cra_blocksize = ialg.base.cra_blocksize;
    alg.base.cra_alignmask = ialg.base.cra_alignmask;
    alg.base.cra_module = ialg.base.cra_module;
    alg.base.cra_ctxsize = sizeof(struct simd_aead_ctx);
    alg.ivsize = ialg.ivsize;
    alg.maxauthsize = ialg.maxauthsize;
    alg.chunksize = ialg.chunksize;
    alg.init = simd_aead_init;
    alg.exit = simd_aead_exit;
    alg.setkey = simd_aead_setkey;
    alg.setauthsize = simd_aead_setauthsize;
    alg.encrypt = simd_aead_encrypt;
    alg.decrypt = simd_aead_decrypt;
    err = crypto_register_aead(alg);
    if (err)
    goto out_free_salg;
    out:
    return salg;
    out_free_salg:
    kfree(salg);
    salg = ERR_PTR(err);
    goto out;
    }
#[no_mangle]
unsafe extern "C" fn simd_aead_free(salg: *mut simd_aead_alg) {
    static void simd_aead_free(struct simd_aead_alg *salg)
    {
    crypto_unregister_aead(&salg.alg);
    kfree(salg);
    }
    int simd_register_aeads_compat(struct aead_alg *algs, int count,
    struct simd_aead_alg **simd_algs)
    {
    int err;
    int i;
    const char *algname;
    const char *drvname;
    const char *basename;
    struct simd_aead_alg *simd;
    for (i = 0; i < count; i++) {
    if (WARN_ON(strncmp(algs[i].base.cra_name, "__", 2) ||
    strncmp(algs[i].base.cra_driver_name, "__", 2)))
    return -EINVAL;
    }
    err = crypto_register_aeads(algs, count);
    if (err)
    return err;
    for (i = 0; i < count; i++) {
    algname = algs[i].base.cra_name + 2;
    drvname = algs[i].base.cra_driver_name + 2;
    basename = algs[i].base.cra_driver_name;
    simd = simd_aead_create_compat(algs + i, algname, drvname, basename);
    err = PTR_ERR(simd);
    if (IS_ERR(simd))
    goto err_unregister;
    simd_algs[i] = simd;
    }
    return 0;
    err_unregister:
    simd_unregister_aeads(algs, count, simd_algs);
    return err;
    }
    EXPORT_SYMBOL_GPL(simd_register_aeads_compat);
    void simd_unregister_aeads(struct aead_alg *algs, int count,
    struct simd_aead_alg **simd_algs)
    {
    int i;
    crypto_unregister_aeads(algs, count);
    for (i = 0; i < count; i++) {
    if (simd_algs[i]) {
    simd_aead_free(simd_algs[i]);
    simd_algs[i] = core::ptr::null_mut();
    }
    }
    }
    EXPORT_SYMBOL_GPL(simd_unregister_aeads);
    MODULE_DESCRIPTION("Shared crypto SIMD helpers");
    MODULE_LICENSE("GPL");
