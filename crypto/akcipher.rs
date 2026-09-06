//! Automatically rewritten from C to Rust
//! Source: crypto/akcipher.c
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
// Public Key Encryption
//
// Copyright (c) 2015, Intel Corporation
// Authors: Tadeusz Struk <tadeusz.struk@intel.com>
//

pub const CRYPTO_ALG_TYPE_AHASH_MASK: c_uint = 0x0000000e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_akcipher_sync_data {
    pub tfm: *mut crypto_akcipher,
    pub src: *const c_void,
    pub dst: *mut c_void,
    pub slen: c_uint,
    pub dlen: c_uint,
    pub req: *mut akcipher_request,
    pub cwait: crypto_wait,
    pub sg: scatterlist,
    pub buf: *mut u8,
}

    static int __maybe_unused crypto_akcipher_report(
    struct sk_buff *skb, struct crypto_alg *alg)
    {
    struct crypto_report_akcipher rakcipher = {
    .type = "akcipher",
    };
    return nla_put(skb, CRYPTOCFGA_REPORT_AKCIPHER,
    sizeof(rakcipher), &rakcipher);
    }
    static void __maybe_unused crypto_akcipher_show(struct seq_file *m,
    struct crypto_alg *alg)
    {
    seq_puts(m, "type         : akcipher\n");
    }
#[no_mangle]
unsafe extern "C" fn crypto_akcipher_exit_tfm(tfm: *mut crypto_tfm) {
    static void crypto_akcipher_exit_tfm(struct crypto_tfm *tfm)
    {
    struct crypto_akcipher *akcipher = __crypto_akcipher_tfm(tfm);
    struct akcipher_alg *alg = crypto_akcipher_alg(akcipher);
    alg.exit(akcipher);
    }
#[no_mangle]
unsafe extern "C" fn crypto_akcipher_init_tfm(tfm: *mut crypto_tfm) -> c_int {
    static int crypto_akcipher_init_tfm(struct crypto_tfm *tfm)
    {
    struct crypto_akcipher *akcipher = __crypto_akcipher_tfm(tfm);
    struct akcipher_alg *alg = crypto_akcipher_alg(akcipher);
    if (alg.exit)
    akcipher.base.exit = crypto_akcipher_exit_tfm;
    if (alg.init)
    return alg.init(akcipher);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_akcipher_free_instance(inst: *mut crypto_instance) {
    static void crypto_akcipher_free_instance(struct crypto_instance *inst)
    {
    struct akcipher_instance *akcipher = akcipher_instance(inst);
    akcipher.free(akcipher);
    }
    static const struct crypto_type crypto_akcipher_type = {
    .extsize = crypto_alg_extsize,
    .init_tfm = crypto_akcipher_init_tfm,
    .free = crypto_akcipher_free_instance,

    .show = crypto_akcipher_show,

    .report = crypto_akcipher_report,

    .maskclear = ~CRYPTO_ALG_TYPE_MASK,
    .maskset = CRYPTO_ALG_TYPE_AHASH_MASK,
    .type = CRYPTO_ALG_TYPE_AKCIPHER,
    .tfmsize = offsetof(struct crypto_akcipher, base),
    .algsize = offsetof(struct akcipher_alg, base),
    };
    int crypto_grab_akcipher(struct crypto_akcipher_spawn *spawn,
    struct crypto_instance *inst,
    const char *name, u32 type, u32 mask)
    {
    spawn.base.frontend = &crypto_akcipher_type;
    return crypto_grab_spawn(&spawn.base, inst, name, type, mask);
    }
    EXPORT_SYMBOL_GPL(crypto_grab_akcipher);
    struct crypto_akcipher *crypto_alloc_akcipher(const char *alg_name, u32 type,
    u32 mask)
    {
    return crypto_alloc_tfm(alg_name, &crypto_akcipher_type, type, mask);
    }
    EXPORT_SYMBOL_GPL(crypto_alloc_akcipher);
#[no_mangle]
unsafe extern "C" fn akcipher_prepare_alg(alg: *mut akcipher_alg) {
    static void akcipher_prepare_alg(struct akcipher_alg *alg)
    {
    struct crypto_alg *base = &alg.base;
    base.cra_type = &crypto_akcipher_type;
    base.cra_flags &= ~CRYPTO_ALG_TYPE_MASK;
    base.cra_flags |= CRYPTO_ALG_TYPE_AKCIPHER;
    }
#[no_mangle]
unsafe extern "C" fn akcipher_default_op(req: *mut akcipher_request) -> c_int {
    static int akcipher_default_op(struct akcipher_request *req)
    {
    return -ENOSYS;
    }
    static int akcipher_default_set_key(struct crypto_akcipher *tfm,
    const void *key, unsigned int keylen)
    {
    return -ENOSYS;
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_register_akcipher(alg: *mut akcipher_alg) -> c_int {
    int crypto_register_akcipher(struct akcipher_alg *alg)
    {
    struct crypto_alg *base = &alg.base;
    if (!alg.encrypt)
    alg.encrypt = akcipher_default_op;
    if (!alg.decrypt)
    alg.decrypt = akcipher_default_op;
    if (!alg.set_priv_key)
    alg.set_priv_key = akcipher_default_set_key;
    akcipher_prepare_alg(alg);
    return crypto_register_alg(base);
    }
    EXPORT_SYMBOL_GPL(crypto_register_akcipher);
#[no_mangle]
pub unsafe extern "C" fn crypto_unregister_akcipher(alg: *mut akcipher_alg) {
    void crypto_unregister_akcipher(struct akcipher_alg *alg)
    {
    crypto_unregister_alg(&alg.base);
    }
    EXPORT_SYMBOL_GPL(crypto_unregister_akcipher);
    int akcipher_register_instance(struct crypto_template *tmpl,
    struct akcipher_instance *inst)
    {
    if (WARN_ON(!inst.free))
    return -EINVAL;
    akcipher_prepare_alg(&inst.alg);
    return crypto_register_instance(tmpl, akcipher_crypto_instance(inst));
    }
    EXPORT_SYMBOL_GPL(akcipher_register_instance);
#[no_mangle]
unsafe extern "C" fn crypto_akcipher_sync_prep(data: *mut crypto_akcipher_sync_data) -> c_int {
    static int crypto_akcipher_sync_prep(struct crypto_akcipher_sync_data *data)
    {
    let mut reqsize: c_uint = crypto_akcipher_reqsize(data.tfm);
    struct akcipher_request *req;
    struct scatterlist *sg;
    unsigned int mlen;
    unsigned int len;
    u8 *buf;
    mlen = max(data.slen, data.dlen);
    len = sizeof(*req) + reqsize + mlen;
    if (len < mlen)
    return -EOVERFLOW;
    req = kzalloc(len, GFP_KERNEL);
    if (!req)
    return -ENOMEM;
    data.req = req;
    akcipher_request_set_tfm(req, data.tfm);
    buf = (u8 *)(req + 1) + reqsize;
    data.buf = buf;
    memcpy(buf, data.src, data.slen);
    sg = &data.sg;
    sg_init_one(sg, buf, mlen);
    akcipher_request_set_crypt(req, sg, sg, data.slen, data.dlen);
    crypto_init_wait(&data.cwait);
    akcipher_request_set_callback(req, CRYPTO_TFM_REQ_MAY_SLEEP,
    crypto_req_done, &data.cwait);
    return 0;
    }
    static int crypto_akcipher_sync_post(struct crypto_akcipher_sync_data *data,
    int err)
    {
    err = crypto_wait_req(err, &data.cwait);
    memcpy(data.dst, data.buf, data.dlen);
    data.dlen = data.req.dst_len;
    kfree_sensitive(data.req);
    return err;
    }
    int crypto_akcipher_sync_encrypt(struct crypto_akcipher *tfm,
    const void *src, unsigned int slen,
    void *dst, unsigned int dlen)
    {
    struct crypto_akcipher_sync_data data = {
    .tfm = tfm,
    .src = src,
    .dst = dst,
    .slen = slen,
    .dlen = dlen,
    };
    return crypto_akcipher_sync_prep(&data) ?:
    crypto_akcipher_sync_post(&data,
    crypto_akcipher_encrypt(data.req));
    }
    EXPORT_SYMBOL_GPL(crypto_akcipher_sync_encrypt);
    int crypto_akcipher_sync_decrypt(struct crypto_akcipher *tfm,
    const void *src, unsigned int slen,
    void *dst, unsigned int dlen)
    {
    struct crypto_akcipher_sync_data data = {
    .tfm = tfm,
    .src = src,
    .dst = dst,
    .slen = slen,
    .dlen = dlen,
    };
    return crypto_akcipher_sync_prep(&data) ?:
    crypto_akcipher_sync_post(&data,
    crypto_akcipher_decrypt(data.req)) ?:
    data.dlen;
    }
    EXPORT_SYMBOL_GPL(crypto_akcipher_sync_decrypt);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Generic public key cipher type");
