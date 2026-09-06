//! Automatically rewritten from C to Rust
//! Source: crypto/kpp.c
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
// Key-agreement Protocol Primitives (KPP)
//
// Copyright (c) 2016, Intel Corporation
// Authors: Salvatore Benedetto <salvatore.benedetto@intel.com>
//

    static int __maybe_unused crypto_kpp_report(
    struct sk_buff *skb, struct crypto_alg *alg)
    {
    struct crypto_report_kpp rkpp = {
    .type = "kpp",
    };
    return nla_put(skb, CRYPTOCFGA_REPORT_KPP, sizeof(rkpp), &rkpp);
    }
    static void __maybe_unused crypto_kpp_show(struct seq_file *m,
    struct crypto_alg *alg)
    {
    seq_puts(m, "type         : kpp\n");
    }
#[no_mangle]
unsafe extern "C" fn crypto_kpp_exit_tfm(tfm: *mut crypto_tfm) {
    static void crypto_kpp_exit_tfm(struct crypto_tfm *tfm)
    {
    struct crypto_kpp *kpp = __crypto_kpp_tfm(tfm);
    struct kpp_alg *alg = crypto_kpp_alg(kpp);
    alg.exit(kpp);
    }
#[no_mangle]
unsafe extern "C" fn crypto_kpp_init_tfm(tfm: *mut crypto_tfm) -> c_int {
    static int crypto_kpp_init_tfm(struct crypto_tfm *tfm)
    {
    struct crypto_kpp *kpp = __crypto_kpp_tfm(tfm);
    struct kpp_alg *alg = crypto_kpp_alg(kpp);
    if (alg.exit)
    kpp.base.exit = crypto_kpp_exit_tfm;
    if (alg.init)
    return alg.init(kpp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_kpp_free_instance(inst: *mut crypto_instance) {
    static void crypto_kpp_free_instance(struct crypto_instance *inst)
    {
    struct kpp_instance *kpp = kpp_instance(inst);
    kpp.free(kpp);
    }
    static const struct crypto_type crypto_kpp_type = {
    .extsize = crypto_alg_extsize,
    .init_tfm = crypto_kpp_init_tfm,
    .free = crypto_kpp_free_instance,

    .show = crypto_kpp_show,

    .report = crypto_kpp_report,

    .maskclear = ~CRYPTO_ALG_TYPE_MASK,
    .maskset = CRYPTO_ALG_TYPE_MASK,
    .type = CRYPTO_ALG_TYPE_KPP,
    .tfmsize = offsetof(struct crypto_kpp, base),
    .algsize = offsetof(struct kpp_alg, base),
    };
    struct crypto_kpp *crypto_alloc_kpp(const char *alg_name, u32 type, u32 mask)
    {
    return crypto_alloc_tfm(alg_name, &crypto_kpp_type, type, mask);
    }
    EXPORT_SYMBOL_GPL(crypto_alloc_kpp);
    int crypto_grab_kpp(struct crypto_kpp_spawn *spawn,
    struct crypto_instance *inst,
    const char *name, u32 type, u32 mask)
    {
    spawn.base.frontend = &crypto_kpp_type;
    return crypto_grab_spawn(&spawn.base, inst, name, type, mask);
    }
    EXPORT_SYMBOL_GPL(crypto_grab_kpp);
#[no_mangle]
pub unsafe extern "C" fn crypto_has_kpp(alg_name: *const c_char, type: u32, mask: u32) -> c_int {
    int crypto_has_kpp(const char *alg_name, u32 type, u32 mask)
    {
    return crypto_type_has_alg(alg_name, &crypto_kpp_type, type, mask);
    }
    EXPORT_SYMBOL_GPL(crypto_has_kpp);
#[no_mangle]
unsafe extern "C" fn kpp_prepare_alg(alg: *mut kpp_alg) {
    static void kpp_prepare_alg(struct kpp_alg *alg)
    {
    struct crypto_alg *base = &alg.base;
    base.cra_type = &crypto_kpp_type;
    base.cra_flags &= ~CRYPTO_ALG_TYPE_MASK;
    base.cra_flags |= CRYPTO_ALG_TYPE_KPP;
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_register_kpp(alg: *mut kpp_alg) -> c_int {
    int crypto_register_kpp(struct kpp_alg *alg)
    {
    struct crypto_alg *base = &alg.base;
    kpp_prepare_alg(alg);
    return crypto_register_alg(base);
    }
    EXPORT_SYMBOL_GPL(crypto_register_kpp);
#[no_mangle]
pub unsafe extern "C" fn crypto_unregister_kpp(alg: *mut kpp_alg) {
    void crypto_unregister_kpp(struct kpp_alg *alg)
    {
    crypto_unregister_alg(&alg.base);
    }
    EXPORT_SYMBOL_GPL(crypto_unregister_kpp);
    int kpp_register_instance(struct crypto_template *tmpl,
    struct kpp_instance *inst)
    {
    if (WARN_ON(!inst.free))
    return -EINVAL;
    kpp_prepare_alg(&inst.alg);
    return crypto_register_instance(tmpl, kpp_crypto_instance(inst));
    }
    EXPORT_SYMBOL_GPL(kpp_register_instance);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Key-agreement Protocol Primitives");
