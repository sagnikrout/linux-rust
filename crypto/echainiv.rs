//! Automatically rewritten from C to Rust
//! Source: crypto/echainiv.c
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
// echainiv: Encrypted Chain IV Generator
//
// This generator generates an IV based on a sequence number by multiplying
// it with a salt and then encrypting it with the same key as used to encrypt
// the plain text.  This algorithm requires that the block size be equal
// to the IV size.  It is mainly useful for CBC.
//
// This generator can only be used by algorithms where authentication
// is performed after encryption (i.e., authenc).
//
// Copyright (c) 2015 Herbert Xu <herbert@gondor.apana.org.au>
//

#[no_mangle]
unsafe extern "C" fn echainiv_encrypt(req: *mut aead_request) -> c_int {
    static int echainiv_encrypt(struct aead_request *req)
    {
    struct crypto_aead *geniv = crypto_aead_reqtfm(req);
    struct aead_geniv_ctx *ctx = crypto_aead_ctx(geniv);
    struct aead_request *subreq = aead_request_ctx(req);
    __be64 nseqno;
    u64 seqno;
    u8 *info;
    let mut ivsize: c_uint = crypto_aead_ivsize(geniv);
    if (req.cryptlen < ivsize)
    return -EINVAL;
    aead_request_set_tfm(subreq, ctx.child);
    info = req.iv;
    if (req.src != req.dst)
    memcpy_sglist(req.dst, req.src,
    req.assoclen + req.cryptlen);
    aead_request_set_callback(subreq, req.base.flags,
    req.base.complete, req.base.data);
    aead_request_set_crypt(subreq, req.dst, req.dst,
    req.cryptlen, info);
    aead_request_set_ad(subreq, req.assoclen);
    memcpy(&nseqno, info + ivsize - 8, 8);
    seqno = be64_to_cpu(nseqno);
    memset(info, 0, ivsize);
    scatterwalk_map_and_copy(info, req.dst, req.assoclen, ivsize, 1);
    do {
    u64 a;
    memcpy(&a, ctx.salt + ivsize - 8, 8);
    a |= 1;
    a *= seqno;
    memcpy(info + ivsize - 8, &a, 8);
    } while ((ivsize -= 8));
    return crypto_aead_encrypt(subreq);
    }
#[no_mangle]
unsafe extern "C" fn echainiv_decrypt(req: *mut aead_request) -> c_int {
    static int echainiv_decrypt(struct aead_request *req)
    {
    struct crypto_aead *geniv = crypto_aead_reqtfm(req);
    struct aead_geniv_ctx *ctx = crypto_aead_ctx(geniv);
    struct aead_request *subreq = aead_request_ctx(req);
    crypto_completion_t compl;
    void *data;
    let mut ivsize: c_uint = crypto_aead_ivsize(geniv);
    if (req.cryptlen < ivsize)
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
    static int echainiv_aead_create(struct crypto_template *tmpl,
    struct rtattr **tb)
    {
    struct aead_instance *inst;
    int err;
    inst = aead_geniv_alloc(tmpl, tb);
    if (IS_ERR(inst))
    return PTR_ERR(inst);
    err = -EINVAL;
    if (inst.alg.ivsize & (sizeof(u64) - 1) || !inst.alg.ivsize)
    goto free_inst;
    inst.alg.encrypt = echainiv_encrypt;
    inst.alg.decrypt = echainiv_decrypt;
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
    static struct crypto_template echainiv_tmpl = {
    .name = "echainiv",
    .create = echainiv_aead_create,
    .module = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn echainiv_module_init() -> int __init {
    static int __init echainiv_module_init(void)
    {
    return crypto_register_template(&echainiv_tmpl);
    }
#[no_mangle]
unsafe extern "C" fn echainiv_module_exit() -> void __exit {
    static void __exit echainiv_module_exit(void)
    {
    crypto_unregister_template(&echainiv_tmpl);
    }
    module_init(echainiv_module_init);
    module_exit(echainiv_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Encrypted Chain IV Generator");
    MODULE_ALIAS_CRYPTO("echainiv");
