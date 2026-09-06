//! Automatically rewritten from C to Rust
//! Source: crypto/seqiv.c
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
// seqiv: Sequence Number IV Generator
//
// This generator generates an IV based on a sequence number by xoring it
// with a salt.  This algorithm is mainly useful for CTR and similar modes.
//
// Copyright (c) 2007 Herbert Xu <herbert@gondor.apana.org.au>
//

#[no_mangle]
unsafe extern "C" fn seqiv_aead_encrypt_complete2(req: *mut aead_request, err: c_int) {
    static void seqiv_aead_encrypt_complete2(struct aead_request *req, int err)
    {
    struct aead_request *subreq = aead_request_ctx(req);
    struct crypto_aead *geniv;
    if (err == -EINPROGRESS || err == -EBUSY)
    return;
    if (err)
    goto out;
    geniv = crypto_aead_reqtfm(req);
    memcpy(req.iv, subreq.iv, crypto_aead_ivsize(geniv));
    out:
    kfree_sensitive(subreq.iv);
    }
#[no_mangle]
unsafe extern "C" fn seqiv_aead_encrypt_complete(data: *mut c_void, err: c_int) {
    static void seqiv_aead_encrypt_complete(void *data, int err)
    {
    struct aead_request *req = data;
    seqiv_aead_encrypt_complete2(req, err);
    aead_request_complete(req, err);
    }
#[no_mangle]
unsafe extern "C" fn seqiv_aead_encrypt(req: *mut aead_request) -> c_int {
    static int seqiv_aead_encrypt(struct aead_request *req)
    {
    struct crypto_aead *geniv = crypto_aead_reqtfm(req);
    struct aead_geniv_ctx *ctx = crypto_aead_ctx(geniv);
    struct aead_request *subreq = aead_request_ctx(req);
    crypto_completion_t compl;
    bool unaligned_info;
    void *data;
    u8 *info;
    let mut ivsize: c_uint = 8;
    int err;
    if (req.cryptlen < ivsize)
    return -EINVAL;
    aead_request_set_tfm(subreq, ctx.child);
    compl = req.base.complete;
    data = req.base.data;
    info = req.iv;
    if (req.src != req.dst)
    memcpy_sglist(req.dst, req.src,
    req.assoclen + req.cryptlen);
    unaligned_info = !IS_ALIGNED((unsigned long)info,
    crypto_aead_alignmask(geniv) + 1);
    if (unlikely(unaligned_info)) {
    info = kmemdup(req.iv, ivsize, req.base.flags &
    CRYPTO_TFM_REQ_MAY_SLEEP ? GFP_KERNEL :
    GFP_ATOMIC);
    if (!info)
    return -ENOMEM;
    compl = seqiv_aead_encrypt_complete;
    data = req;
    }
    aead_request_set_callback(subreq, req.base.flags, compl, data);
    aead_request_set_crypt(subreq, req.dst, req.dst,
    req.cryptlen - ivsize, info);
    aead_request_set_ad(subreq, req.assoclen + ivsize);
    crypto_xor(info, ctx.salt, ivsize);
    scatterwalk_map_and_copy(info, req.dst, req.assoclen, ivsize, 1);
    err = crypto_aead_encrypt(subreq);
    if (unlikely(unaligned_info))
    seqiv_aead_encrypt_complete2(req, err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn seqiv_aead_decrypt(req: *mut aead_request) -> c_int {
    static int seqiv_aead_decrypt(struct aead_request *req)
    {
    struct crypto_aead *geniv = crypto_aead_reqtfm(req);
    struct aead_geniv_ctx *ctx = crypto_aead_ctx(geniv);
    struct aead_request *subreq = aead_request_ctx(req);
    crypto_completion_t compl;
    void *data;
    let mut ivsize: c_uint = 8;
    if (req.cryptlen < ivsize + crypto_aead_authsize(geniv))
    return -EINVAL;
    aead_request_set_tfm(subreq, ctx.child);
    compl = req.base.complete;
    data = req.base.data;
    aead_request_set_callback(subreq, req.base.flags, compl, data);
    aead_request_set_crypt(subreq, req.src, req.dst,
    req.cryptlen - ivsize, req.iv);
    aead_request_set_ad(subreq, req.assoclen + ivsize);
    scatterwalk_map_and_copy(req.iv, req.src, req.assoclen, ivsize, 0);
    return crypto_aead_decrypt(subreq);
    }
#[no_mangle]
unsafe extern "C" fn seqiv_aead_create(tmpl: *mut crypto_template, tb: *mut rtattr) -> c_int {
    static int seqiv_aead_create(struct crypto_template *tmpl, struct rtattr **tb)
    {
    struct aead_instance *inst;
    int err;
    inst = aead_geniv_alloc(tmpl, tb);
    if (IS_ERR(inst))
    return PTR_ERR(inst);
    err = -EINVAL;
    if (inst.alg.ivsize != sizeof(u64))
    goto free_inst;
    inst.alg.encrypt = seqiv_aead_encrypt;
    inst.alg.decrypt = seqiv_aead_decrypt;
    inst.alg.init = aead_init_geniv;
    inst.alg.exit = aead_exit_geniv;
    inst.alg.base.cra_ctxsize = sizeof(struct aead_geniv_ctx);
    inst.alg.base.cra_ctxsize += inst.alg.ivsize;
    err = aead_register_instance(tmpl, inst);
    if (err) {
    free_inst:
    inst.free(inst);
    }
    return err;
    }
    static struct crypto_template seqiv_tmpl = {
    .name = "seqiv",
    .create = seqiv_aead_create,
    .module = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn seqiv_module_init() -> int __init {
    static int __init seqiv_module_init(void)
    {
    return crypto_register_template(&seqiv_tmpl);
    }
#[no_mangle]
unsafe extern "C" fn seqiv_module_exit() -> void __exit {
    static void __exit seqiv_module_exit(void)
    {
    crypto_unregister_template(&seqiv_tmpl);
    }
    module_init(seqiv_module_init);
    module_exit(seqiv_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Sequence Number IV Generator");
    MODULE_ALIAS_CRYPTO("seqiv");
