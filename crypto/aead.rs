//! Automatically rewritten from C to Rust
//! Source: crypto/aead.c
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
// AEAD: Authenticated Encryption with Associated Data
//
// This file provides API support for AEAD algorithms.
//
// Copyright (c) 2007-2015 Herbert Xu <herbert@gondor.apana.org.au>
//

    static int setkey_unaligned(struct crypto_aead *tfm, const u8 *key,
    unsigned int keylen)
    {
    let mut alignmask: c_ulong = crypto_aead_alignmask(tfm);
    int ret;
    u8 *buffer, *alignbuffer;
    unsigned long absize;
    absize = keylen + alignmask;
    buffer = kmalloc(absize, GFP_ATOMIC);
    if (!buffer)
    return -ENOMEM;
    alignbuffer = (u8 *)ALIGN((unsigned long)buffer, alignmask + 1);
    memcpy(alignbuffer, key, keylen);
    ret = crypto_aead_alg(tfm).setkey(tfm, alignbuffer, keylen);
    kfree_sensitive(buffer);
    return ret;
    }
    int crypto_aead_setkey(struct crypto_aead *tfm,
    const u8 *key, unsigned int keylen)
    {
    let mut alignmask: c_ulong = crypto_aead_alignmask(tfm);
    int err;
    if ((unsigned long)key & alignmask)
    err = setkey_unaligned(tfm, key, keylen);
    else
    err = crypto_aead_alg(tfm).setkey(tfm, key, keylen);
    if (unlikely(err)) {
    crypto_aead_set_flags(tfm, CRYPTO_TFM_NEED_KEY);
    return err;
    }
    crypto_aead_clear_flags(tfm, CRYPTO_TFM_NEED_KEY);
    return 0;
    }
    EXPORT_SYMBOL_GPL(crypto_aead_setkey);
#[no_mangle]
pub unsafe extern "C" fn crypto_aead_setauthsize(tfm: *mut crypto_aead, authsize: c_uint) -> c_int {
    int crypto_aead_setauthsize(struct crypto_aead *tfm, unsigned int authsize)
    {
    int err;
    if ((!authsize && crypto_aead_maxauthsize(tfm)) ||
    authsize > crypto_aead_maxauthsize(tfm))
    return -EINVAL;
    if (crypto_aead_alg(tfm).setauthsize) {
    err = crypto_aead_alg(tfm).setauthsize(tfm, authsize);
    if (err)
    return err;
    }
    tfm.authsize = authsize;
    return 0;
    }
    EXPORT_SYMBOL_GPL(crypto_aead_setauthsize);
#[no_mangle]
pub unsafe extern "C" fn crypto_aead_encrypt(req: *mut aead_request) -> c_int {
    int crypto_aead_encrypt(struct aead_request *req)
    {
    struct crypto_aead *aead = crypto_aead_reqtfm(req);
    if (crypto_aead_get_flags(aead) & CRYPTO_TFM_NEED_KEY)
    return -ENOKEY;
    return crypto_aead_alg(aead).encrypt(req);
    }
    EXPORT_SYMBOL_GPL(crypto_aead_encrypt);
#[no_mangle]
pub unsafe extern "C" fn crypto_aead_decrypt(req: *mut aead_request) -> c_int {
    int crypto_aead_decrypt(struct aead_request *req)
    {
    struct crypto_aead *aead = crypto_aead_reqtfm(req);
    if (crypto_aead_get_flags(aead) & CRYPTO_TFM_NEED_KEY)
    return -ENOKEY;
    if (req.cryptlen < crypto_aead_authsize(aead))
    return -EINVAL;
    return crypto_aead_alg(aead).decrypt(req);
    }
    EXPORT_SYMBOL_GPL(crypto_aead_decrypt);
#[no_mangle]
unsafe extern "C" fn crypto_aead_exit_tfm(tfm: *mut crypto_tfm) {
    static void crypto_aead_exit_tfm(struct crypto_tfm *tfm)
    {
    struct crypto_aead *aead = __crypto_aead_cast(tfm);
    struct aead_alg *alg = crypto_aead_alg(aead);
    alg.exit(aead);
    }
#[no_mangle]
unsafe extern "C" fn crypto_aead_init_tfm(tfm: *mut crypto_tfm) -> c_int {
    static int crypto_aead_init_tfm(struct crypto_tfm *tfm)
    {
    struct crypto_aead *aead = __crypto_aead_cast(tfm);
    struct aead_alg *alg = crypto_aead_alg(aead);
    crypto_aead_set_flags(aead, CRYPTO_TFM_NEED_KEY);
    crypto_aead_set_reqsize(aead, crypto_tfm_alg_reqsize(tfm));
    aead.authsize = alg.maxauthsize;
    if (alg.exit)
    aead.base.exit = crypto_aead_exit_tfm;
    if (alg.init)
    return alg.init(aead);
    return 0;
    }
    static int __maybe_unused crypto_aead_report(
    struct sk_buff *skb, struct crypto_alg *alg)
    {
    struct aead_alg *aead = container_of(alg, struct aead_alg, base);
    struct crypto_report_aead raead = {
    .type = "aead",
    .geniv = "<none>",
    };
    raead.blocksize = alg.cra_blocksize;
    raead.maxauthsize = aead.maxauthsize;
    raead.ivsize = aead.ivsize;
    return nla_put(skb, CRYPTOCFGA_REPORT_AEAD, sizeof(raead), &raead);
    }
    static void __maybe_unused crypto_aead_show(struct seq_file *m,
    struct crypto_alg *alg)
    {
    struct aead_alg *aead = container_of(alg, struct aead_alg, base);
    seq_printf(m, "type         : aead\n");
    seq_printf(m, "async        : %s\n",
    str_yes_no(alg.cra_flags & CRYPTO_ALG_ASYNC));
    seq_printf(m, "blocksize    : %u\n", alg.cra_blocksize);
    seq_printf(m, "ivsize       : %u\n", aead.ivsize);
    seq_printf(m, "maxauthsize  : %u\n", aead.maxauthsize);
    seq_printf(m, "geniv        : <none>\n");
    }
#[no_mangle]
unsafe extern "C" fn crypto_aead_free_instance(inst: *mut crypto_instance) {
    static void crypto_aead_free_instance(struct crypto_instance *inst)
    {
    struct aead_instance *aead = aead_instance(inst);
    aead.free(aead);
    }
    static const struct crypto_type crypto_aead_type = {
    .extsize = crypto_alg_extsize,
    .init_tfm = crypto_aead_init_tfm,
    .free = crypto_aead_free_instance,

    .show = crypto_aead_show,

    .report = crypto_aead_report,

    .maskclear = ~CRYPTO_ALG_TYPE_MASK,
    .maskset = CRYPTO_ALG_TYPE_MASK,
    .type = CRYPTO_ALG_TYPE_AEAD,
    .tfmsize = offsetof(struct crypto_aead, base),
    .algsize = offsetof(struct aead_alg, base),
    };
    int crypto_grab_aead(struct crypto_aead_spawn *spawn,
    struct crypto_instance *inst,
    const char *name, u32 type, u32 mask)
    {
    spawn.base.frontend = &crypto_aead_type;
    return crypto_grab_spawn(&spawn.base, inst, name, type, mask);
    }
    EXPORT_SYMBOL_GPL(crypto_grab_aead);
    struct crypto_aead *crypto_alloc_aead(const char *alg_name, u32 type, u32 mask)
    {
    return crypto_alloc_tfm(alg_name, &crypto_aead_type, type, mask);
    }
    EXPORT_SYMBOL_GPL(crypto_alloc_aead);
    struct crypto_sync_aead *crypto_alloc_sync_aead(const char *alg_name, u32 type, u32 mask)
    {
    struct crypto_aead *tfm;
// Only sync algorithms are allowed.
    mask |= CRYPTO_ALG_ASYNC;
    type &= ~(CRYPTO_ALG_ASYNC);
    tfm = crypto_alloc_tfm(alg_name, &crypto_aead_type, type, mask);
    if (!IS_ERR(tfm) && WARN_ON(crypto_aead_reqsize(tfm) > MAX_SYNC_AEAD_REQSIZE)) {
    crypto_free_aead(tfm);
    return ERR_PTR(-EINVAL);
    }
    return (struct crypto_sync_aead *)tfm;
    }
    EXPORT_SYMBOL_GPL(crypto_alloc_sync_aead);
#[no_mangle]
pub unsafe extern "C" fn crypto_has_aead(alg_name: *const c_char, type: u32, mask: u32) -> c_int {
    int crypto_has_aead(const char *alg_name, u32 type, u32 mask)
    {
    return crypto_type_has_alg(alg_name, &crypto_aead_type, type, mask);
    }
    EXPORT_SYMBOL_GPL(crypto_has_aead);
#[no_mangle]
unsafe extern "C" fn aead_prepare_alg(alg: *mut aead_alg) -> c_int {
    static int aead_prepare_alg(struct aead_alg *alg)
    {
    struct crypto_alg *base = &alg.base;
    if (max3(alg.maxauthsize, alg.ivsize, alg.chunksize) >
    PAGE_SIZE / 8)
    return -EINVAL;
    if (!alg.chunksize)
    alg.chunksize = base.cra_blocksize;
    base.cra_type = &crypto_aead_type;
    base.cra_flags &= ~CRYPTO_ALG_TYPE_MASK;
    base.cra_flags |= CRYPTO_ALG_TYPE_AEAD;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_register_aead(alg: *mut aead_alg) -> c_int {
    int crypto_register_aead(struct aead_alg *alg)
    {
    struct crypto_alg *base = &alg.base;
    int err;
    err = aead_prepare_alg(alg);
    if (err)
    return err;
    return crypto_register_alg(base);
    }
    EXPORT_SYMBOL_GPL(crypto_register_aead);
#[no_mangle]
pub unsafe extern "C" fn crypto_unregister_aead(alg: *mut aead_alg) {
    void crypto_unregister_aead(struct aead_alg *alg)
    {
    crypto_unregister_alg(&alg.base);
    }
    EXPORT_SYMBOL_GPL(crypto_unregister_aead);
#[no_mangle]
pub unsafe extern "C" fn crypto_register_aeads(algs: *mut aead_alg, count: c_int) -> c_int {
    int crypto_register_aeads(struct aead_alg *algs, int count)
    {
    int i, ret;
    for (i = 0; i < count; i++) {
    ret = crypto_register_aead(&algs[i]);
    if (ret)
    goto err;
    }
    return 0;
    err:
    for (--i; i >= 0; --i)
    crypto_unregister_aead(&algs[i]);
    return ret;
    }
    EXPORT_SYMBOL_GPL(crypto_register_aeads);
#[no_mangle]
pub unsafe extern "C" fn crypto_unregister_aeads(algs: *mut aead_alg, count: c_int) {
    void crypto_unregister_aeads(struct aead_alg *algs, int count)
    {
    int i;
    for (i = count - 1; i >= 0; --i)
    crypto_unregister_aead(&algs[i]);
    }
    EXPORT_SYMBOL_GPL(crypto_unregister_aeads);
    int aead_register_instance(struct crypto_template *tmpl,
    struct aead_instance *inst)
    {
    int err;
    if (WARN_ON(!inst.free))
    return -EINVAL;
    err = aead_prepare_alg(&inst.alg);
    if (err)
    return err;
    return crypto_register_instance(tmpl, aead_crypto_instance(inst));
    }
    EXPORT_SYMBOL_GPL(aead_register_instance);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Authenticated Encryption with Associated Data (AEAD)");
