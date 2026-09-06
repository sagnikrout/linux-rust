//! Automatically rewritten from C to Rust
//! Source: crypto/rng.c
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
// Cryptographic API.
//
// RNG operations.
//
// Copyright (c) 2008 Neil Horman <nhorman@tuxdriver.com>
// Copyright (c) 2015 Herbert Xu <herbert@gondor.apana.org.au>
//

    static DEFINE_MUTEX(crypto_default_rng_lock);
    static struct crypto_rng *crypto_default_rng;
    static int crypto_default_rng_refcnt;
#[no_mangle]
pub unsafe extern "C" fn crypto_rng_reset(tfm: *mut crypto_rng, seed: *const u8, slen: c_uint) -> c_int {
    int crypto_rng_reset(struct crypto_rng *tfm, const u8 *seed, unsigned int slen)
    {
    u8 *buf = core::ptr::null_mut();
    int err;
    if (!seed && slen) {
    buf = kmalloc(slen, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    err = get_random_bytes_wait(buf, slen);
    if (err)
    goto out;
    seed = buf;
    }
    err = crypto_rng_alg(tfm).seed(tfm, seed, slen);
    out:
    kfree_sensitive(buf);
    return err;
    }
    EXPORT_SYMBOL_GPL(crypto_rng_reset);
#[no_mangle]
unsafe extern "C" fn crypto_rng_init_tfm(tfm: *mut crypto_tfm) -> c_int {
    static int crypto_rng_init_tfm(struct crypto_tfm *tfm)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn seedsize(alg: *mut crypto_alg) -> c_uint {
    static unsigned int seedsize(struct crypto_alg *alg)
    {
    struct rng_alg *ralg = container_of(alg, struct rng_alg, base);
    return ralg.seedsize;
    }
    static int __maybe_unused crypto_rng_report(
    struct sk_buff *skb, struct crypto_alg *alg)
    {
    struct crypto_report_rng rrng = {
    .type = "rng",
    };
    rrng.seedsize = seedsize(alg);
    return nla_put(skb, CRYPTOCFGA_REPORT_RNG, sizeof(rrng), &rrng);
    }
    static void __maybe_unused crypto_rng_show(struct seq_file *m,
    struct crypto_alg *alg)
    {
    seq_printf(m, "type         : rng\n");
    seq_printf(m, "seedsize     : %u\n", seedsize(alg));
    }
    static const struct crypto_type crypto_rng_type = {
    .extsize = crypto_alg_extsize,
    .init_tfm = crypto_rng_init_tfm,

    .show = crypto_rng_show,

    .report = crypto_rng_report,

    .maskclear = ~CRYPTO_ALG_TYPE_MASK,
    .maskset = CRYPTO_ALG_TYPE_MASK,
    .type = CRYPTO_ALG_TYPE_RNG,
    .tfmsize = offsetof(struct crypto_rng, base),
    .algsize = offsetof(struct rng_alg, base),
    };
    struct crypto_rng *crypto_alloc_rng(const char *alg_name, u32 type, u32 mask)
    {
    return crypto_alloc_tfm(alg_name, &crypto_rng_type, type, mask);
    }
    EXPORT_SYMBOL_GPL(crypto_alloc_rng);
#[no_mangle]
unsafe extern "C" fn crypto_get_default_rng() -> c_int {
    static int crypto_get_default_rng(void)
    {
    struct crypto_rng *rng;
    int err;
    mutex_lock(&crypto_default_rng_lock);
    if (!crypto_default_rng) {
    rng = crypto_alloc_rng("stdrng", 0, 0);
    err = PTR_ERR(rng);
    if (IS_ERR(rng))
    goto unlock;
    err = crypto_rng_reset(rng, core::ptr::null_mut(), crypto_rng_seedsize(rng));
    if (err) {
    crypto_free_rng(rng);
    goto unlock;
    }
    crypto_default_rng = rng;
    }
    crypto_default_rng_refcnt++;
    err = 0;
    unlock:
    mutex_unlock(&crypto_default_rng_lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn crypto_put_default_rng() {
    static void crypto_put_default_rng(void)
    {
    mutex_lock(&crypto_default_rng_lock);
    crypto_default_rng_refcnt--;
    mutex_unlock(&crypto_default_rng_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn __crypto_stdrng_get_bytes(buf: *mut c_void, len: c_uint) -> c_int {
    int __crypto_stdrng_get_bytes(void *buf, unsigned int len)
    {
    int err;
    err = crypto_get_default_rng();
    if (err)
    return err;
    err = crypto_rng_get_bytes(crypto_default_rng, buf, len);
    crypto_put_default_rng();
    return err;
    }
    EXPORT_SYMBOL_GPL(__crypto_stdrng_get_bytes);

#[no_mangle]
pub unsafe extern "C" fn crypto_del_default_rng() -> c_int {
    int crypto_del_default_rng(void)
    {
    let mut err: c_int = -EBUSY;
    mutex_lock(&crypto_default_rng_lock);
    if (crypto_default_rng_refcnt)
    goto out;
    crypto_free_rng(crypto_default_rng);
    crypto_default_rng = core::ptr::null_mut();
    err = 0;
    out:
    mutex_unlock(&crypto_default_rng_lock);
    return err;
    }
    EXPORT_SYMBOL_GPL(crypto_del_default_rng);

    static void rng_default_set_ent(struct crypto_rng *tfm, const u8 *data,
    unsigned int len)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_register_rng(alg: *mut rng_alg) -> c_int {
    int crypto_register_rng(struct rng_alg *alg)
    {
    struct crypto_alg *base = &alg.base;
    if (alg.seedsize > PAGE_SIZE / 8)
    return -EINVAL;
    base.cra_type = &crypto_rng_type;
    base.cra_flags &= ~CRYPTO_ALG_TYPE_MASK;
    base.cra_flags |= CRYPTO_ALG_TYPE_RNG;
    if (!alg.set_ent)
    alg.set_ent = rng_default_set_ent;
    return crypto_register_alg(base);
    }
    EXPORT_SYMBOL_GPL(crypto_register_rng);
#[no_mangle]
pub unsafe extern "C" fn crypto_unregister_rng(alg: *mut rng_alg) {
    void crypto_unregister_rng(struct rng_alg *alg)
    {
    crypto_unregister_alg(&alg.base);
    }
    EXPORT_SYMBOL_GPL(crypto_unregister_rng);
#[no_mangle]
pub unsafe extern "C" fn crypto_register_rngs(algs: *mut rng_alg, count: c_int) -> c_int {
    int crypto_register_rngs(struct rng_alg *algs, int count)
    {
    int i, ret;
    for (i = 0; i < count; i++) {
    ret = crypto_register_rng(algs + i);
    if (ret) {
    crypto_unregister_rngs(algs, i);
    return ret;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(crypto_register_rngs);
#[no_mangle]
pub unsafe extern "C" fn crypto_unregister_rngs(algs: *mut rng_alg, count: c_int) {
    void crypto_unregister_rngs(struct rng_alg *algs, int count)
    {
    int i;
    for (i = count - 1; i >= 0; --i)
    crypto_unregister_rng(algs + i);
    }
    EXPORT_SYMBOL_GPL(crypto_unregister_rngs);
#[no_mangle]
unsafe extern "C" fn rng_exit() -> void __exit {
    static void __exit rng_exit(void)
    {
    int err;
    err = crypto_del_default_rng();
    if (err)
    pr_err("Failed delete default RNG: %d\n", err);
    }
    module_exit(rng_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Random Number Generator");
