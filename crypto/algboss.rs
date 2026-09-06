//! Automatically rewritten from C to Rust
//! Source: crypto/algboss.c
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
// Create default crypto algorithm instances.
//
// Copyright (c) 2006 Herbert Xu <herbert@gondor.apana.org.au>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cryptomgr_param {
    pub 2]: *mut *mut rtattr tb[CRYPTO_MAX_ATTRS +,
    struct {
    pub attr: rtattr,
    pub data: crypto_attr_type,
    pub type: },
    struct {
    pub attr: rtattr,
    pub data: crypto_attr_alg,
    pub attrs: [}; CRYPTO_MAX_ATTRS],
    pub template: [c_char; CRYPTO_MAX_ALG_NAME],
    pub larval: *mut crypto_larval,
    pub otype: u32,
    pub omask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_test_param {
    pub driver: [c_char; CRYPTO_MAX_ALG_NAME],
    pub alg: [c_char; CRYPTO_MAX_ALG_NAME],
    pub type: u32,
}

#[no_mangle]
unsafe extern "C" fn cryptomgr_probe(data: *mut c_void) -> c_int {
    static int cryptomgr_probe(void *data)
    {
    struct cryptomgr_param *param = data;
    struct crypto_template *tmpl;
    let mut err: c_int = -ENOENT;
    tmpl = crypto_lookup_template(param.template);
    if (!tmpl)
    goto out;
    do {
    err = tmpl.create(tmpl, param.tb);
    } while (err == -EAGAIN && !signal_pending(current));
    crypto_tmpl_put(tmpl);
    out:
    param.larval.adult = ERR_PTR(err);
    param.larval.alg.cra_flags |= CRYPTO_ALG_DEAD;
    complete_all(&param.larval.completion);
    crypto_alg_put(&param.larval.alg);
    kfree(param);
    module_put_and_kthread_exit(0);
    }
#[no_mangle]
unsafe extern "C" fn cryptomgr_schedule_probe(larval: *mut crypto_larval) -> c_int {
    static int cryptomgr_schedule_probe(struct crypto_larval *larval)
    {
    struct task_struct *thread;
    struct cryptomgr_param *param;
    const char *name = larval.alg.cra_name;
    const char *p;
    unsigned int len;
    int i;
    if (!try_module_get(THIS_MODULE))
    goto err;
    param = kzalloc_obj(*param);
    if (!param)
    goto err_put_module;
    for (p = name; isalnum(*p) || *p == '-' || *p == '_'; p++)
    ;
    len = p - name;
    if (!len || *p != '(')
    goto err_free_param;
    memcpy(param.template, name, len);
    i = 0;
    for (;;) {
    name = ++p;
    for (; isalnum(*p) || *p == '-' || *p == '_'; p++)
    ;
    if (*p == '(') {
    let mut recursion: c_int = 0;
    for (;;) {
    if (!*++p)
    goto err_free_param;
    if (*p == '(')
    recursion++;
#[no_mangle]
pub unsafe extern "C" fn if(!recursion--: *mut *mut p == ')' &&) -> else {
    else if (*p == ')' && !recursion--)
    break;
    }
    p++;
    }
    len = p - name;
    if (!len)
    goto err_free_param;
    param.attrs[i].attr.rta_len = sizeof(param.attrs[i]);
    param.attrs[i].attr.rta_type = CRYPTOA_ALG;
    memcpy(param.attrs[i].data.name, name, len);
    param.tb[i + 1] = &param.attrs[i].attr;
    i++;
    if (i >= CRYPTO_MAX_ATTRS)
    goto err_free_param;
    if (*p == ')')
    break;
    if (*p != ',')
    goto err_free_param;
    }
    param.tb[i + 1] = core::ptr::null_mut();
    param.type.attr.rta_len = sizeof(param.type);
    param.type.attr.rta_type = CRYPTOA_TYPE;
    param.type.data.type = larval.alg.cra_flags & ~CRYPTO_ALG_TESTED;
    param.type.data.mask = larval.mask & ~CRYPTO_ALG_TESTED;
    param.tb[0] = &param.type.attr;
    param.otype = larval.alg.cra_flags;
    param.omask = larval.mask;
    crypto_alg_get(&larval.alg);
    param.larval = larval;
    thread = kthread_run(cryptomgr_probe, param, "cryptomgr_probe");
    if (IS_ERR(thread))
    goto err_put_larval;
    return NOTIFY_STOP;
    err_put_larval:
    crypto_alg_put(&larval.alg);
    err_free_param:
    kfree(param);
    err_put_module:
    module_put(THIS_MODULE);
    err:
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn cryptomgr_test(data: *mut c_void) -> c_int {
    static int cryptomgr_test(void *data)
    {
    struct crypto_test_param *param = data;
    let mut type: u32 = param.type;
    int err;
    err = alg_test(param.driver, param.alg, type, CRYPTO_ALG_TESTED);
    crypto_alg_tested(param.driver, err);
    kfree(param);
    module_put_and_kthread_exit(0);
    }
#[no_mangle]
unsafe extern "C" fn cryptomgr_schedule_test(alg: *mut crypto_alg) -> c_int {
    static int cryptomgr_schedule_test(struct crypto_alg *alg)
    {
    struct task_struct *thread;
    struct crypto_test_param *param;
    if (!IS_ENABLED(CONFIG_CRYPTO_SELFTESTS))
    return NOTIFY_DONE;
    if (!try_module_get(THIS_MODULE))
    goto err;
    param = kzalloc_obj(*param);
    if (!param)
    goto err_put_module;
    memcpy(param.driver, alg.cra_driver_name, sizeof(param.driver));
    memcpy(param.alg, alg.cra_name, sizeof(param.alg));
    param.type = alg.cra_flags;
    thread = kthread_run(cryptomgr_test, param, "cryptomgr_test");
    if (IS_ERR(thread))
    goto err_free_param;
    return NOTIFY_STOP;
    err_free_param:
    kfree(param);
    err_put_module:
    module_put(THIS_MODULE);
    err:
    return NOTIFY_OK;
    }
    static int cryptomgr_notify(struct notifier_block *this, unsigned long msg,
    void *data)
    {
    switch (msg) {
    case CRYPTO_MSG_ALG_REQUEST:
    return cryptomgr_schedule_probe(data);
    case CRYPTO_MSG_ALG_REGISTER:
    return cryptomgr_schedule_test(data);
    case CRYPTO_MSG_ALG_LOADED:
    break;
    }
    return NOTIFY_DONE;
    }
    static struct notifier_block cryptomgr_notifier = {
    .notifier_call = cryptomgr_notify,
    };
#[no_mangle]
unsafe extern "C" fn cryptomgr_init() -> int __init {
    static int __init cryptomgr_init(void)
    {
    return crypto_register_notifier(&cryptomgr_notifier);
    }
#[no_mangle]
unsafe extern "C" fn cryptomgr_exit() -> void __exit {
    static void __exit cryptomgr_exit(void)
    {
    let mut err: c_int = crypto_unregister_notifier(&cryptomgr_notifier);
    BUG_ON(err);
    }
    module_init(cryptomgr_init);
    module_exit(cryptomgr_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Crypto Algorithm Manager");
