//! Automatically rewritten from C to Rust
//! Source: crypto/sig.c
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
// Public Key Signature Algorithm
//
// Copyright (c) 2023 Herbert Xu <herbert@gondor.apana.org.au>
//

#[no_mangle]
unsafe extern "C" fn crypto_sig_exit_tfm(tfm: *mut crypto_tfm) {
    static void crypto_sig_exit_tfm(struct crypto_tfm *tfm)
    {
    struct crypto_sig *sig = __crypto_sig_tfm(tfm);
    struct sig_alg *alg = crypto_sig_alg(sig);
    alg.exit(sig);
    }
#[no_mangle]
unsafe extern "C" fn crypto_sig_init_tfm(tfm: *mut crypto_tfm) -> c_int {
    static int crypto_sig_init_tfm(struct crypto_tfm *tfm)
    {
    struct crypto_sig *sig = __crypto_sig_tfm(tfm);
    struct sig_alg *alg = crypto_sig_alg(sig);
    if (alg.exit)
    sig.base.exit = crypto_sig_exit_tfm;
    if (alg.init)
    return alg.init(sig);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_sig_free_instance(inst: *mut crypto_instance) {
    static void crypto_sig_free_instance(struct crypto_instance *inst)
    {
    struct sig_instance *sig = sig_instance(inst);
    sig.free(sig);
    }
    static void __maybe_unused crypto_sig_show(struct seq_file *m,
    struct crypto_alg *alg)
    {
    seq_puts(m, "type         : sig\n");
    }
    static int __maybe_unused crypto_sig_report(struct sk_buff *skb,
    struct crypto_alg *alg)
    {
    struct crypto_report_sig rsig = {
    .type = "sig",
    };
    return nla_put(skb, CRYPTOCFGA_REPORT_SIG, sizeof(rsig), &rsig);
    }
    static const struct crypto_type crypto_sig_type = {
    .extsize = crypto_alg_extsize,
    .init_tfm = crypto_sig_init_tfm,
    .free = crypto_sig_free_instance,

    .show = crypto_sig_show,

    .report = crypto_sig_report,

    .maskclear = ~CRYPTO_ALG_TYPE_MASK,
    .maskset = CRYPTO_ALG_TYPE_MASK,
    .type = CRYPTO_ALG_TYPE_SIG,
    .tfmsize = offsetof(struct crypto_sig, base),
    .algsize = offsetof(struct sig_alg, base),
    };
    struct crypto_sig *crypto_alloc_sig(const char *alg_name, u32 type, u32 mask)
    {
    return crypto_alloc_tfm(alg_name, &crypto_sig_type, type, mask);
    }
    EXPORT_SYMBOL_GPL(crypto_alloc_sig);
    static int sig_default_sign(struct crypto_sig *tfm,
    const void *src, unsigned int slen,
    void *dst, unsigned int dlen)
    {
    return -ENOSYS;
    }
    static int sig_default_verify(struct crypto_sig *tfm,
    const void *src, unsigned int slen,
    const void *dst, unsigned int dlen)
    {
    return -ENOSYS;
    }
    static int sig_default_set_key(struct crypto_sig *tfm,
    const void *key, unsigned int keylen)
    {
    return -ENOSYS;
    }
#[no_mangle]
unsafe extern "C" fn sig_default_size(tfm: *mut crypto_sig) -> c_uint {
    static unsigned int sig_default_size(struct crypto_sig *tfm)
    {
    return DIV_ROUND_UP_POW2(crypto_sig_keysize(tfm), BITS_PER_BYTE);
    }
#[no_mangle]
unsafe extern "C" fn sig_prepare_alg(alg: *mut sig_alg) -> c_int {
    static int sig_prepare_alg(struct sig_alg *alg)
    {
    struct crypto_alg *base = &alg.base;
    if (!alg.sign)
    alg.sign = sig_default_sign;
    if (!alg.verify)
    alg.verify = sig_default_verify;
    if (!alg.set_priv_key)
    alg.set_priv_key = sig_default_set_key;
    if (!alg.set_pub_key)
    return -EINVAL;
    if (!alg.key_size)
    return -EINVAL;
    if (!alg.max_size)
    alg.max_size = sig_default_size;
    if (!alg.digest_size)
    alg.digest_size = sig_default_size;
    base.cra_type = &crypto_sig_type;
    base.cra_flags &= ~CRYPTO_ALG_TYPE_MASK;
    base.cra_flags |= CRYPTO_ALG_TYPE_SIG;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_register_sig(alg: *mut sig_alg) -> c_int {
    int crypto_register_sig(struct sig_alg *alg)
    {
    struct crypto_alg *base = &alg.base;
    int err;
    err = sig_prepare_alg(alg);
    if (err)
    return err;
    return crypto_register_alg(base);
    }
    EXPORT_SYMBOL_GPL(crypto_register_sig);
#[no_mangle]
pub unsafe extern "C" fn crypto_unregister_sig(alg: *mut sig_alg) {
    void crypto_unregister_sig(struct sig_alg *alg)
    {
    crypto_unregister_alg(&alg.base);
    }
    EXPORT_SYMBOL_GPL(crypto_unregister_sig);
    int sig_register_instance(struct crypto_template *tmpl,
    struct sig_instance *inst)
    {
    int err;
    if (WARN_ON(!inst.free))
    return -EINVAL;
    err = sig_prepare_alg(&inst.alg);
    if (err)
    return err;
    return crypto_register_instance(tmpl, sig_crypto_instance(inst));
    }
    EXPORT_SYMBOL_GPL(sig_register_instance);
    int crypto_grab_sig(struct crypto_sig_spawn *spawn,
    struct crypto_instance *inst,
    const char *name, u32 type, u32 mask)
    {
    spawn.base.frontend = &crypto_sig_type;
    return crypto_grab_spawn(&spawn.base, inst, name, type, mask);
    }
    EXPORT_SYMBOL_GPL(crypto_grab_sig);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Public Key Signature Algorithms");
