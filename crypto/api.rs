//! Automatically rewritten from C to Rust
//! Source: crypto/api.c
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
// Scatterlist Cryptographic API.
//
// Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
// Copyright (c) 2002 David S. Miller (davem@redhat.com)
// Copyright (c) 2005 Herbert Xu <herbert@gondor.apana.org.au>
//
// Portions derived from Cryptoapi, by Alexander Kjeldaas <astor@fast.no>
// and Nettle, by Niels Möller.
//

    LIST_HEAD(crypto_alg_list);
    EXPORT_SYMBOL_GPL(crypto_alg_list);
    DECLARE_RWSEM(crypto_alg_sem);
    EXPORT_SYMBOL_GPL(crypto_alg_sem);
    BLOCKING_NOTIFIER_HEAD(crypto_chain);
    EXPORT_SYMBOL_GPL(crypto_chain);

    DEFINE_STATIC_KEY_FALSE(__crypto_boot_test_finished);

    static struct crypto_alg *crypto_larval_wait(struct crypto_alg *alg,
    u32 type, u32 mask);
    static struct crypto_alg *crypto_alg_lookup(const char *name, u32 type,
    u32 mask);
    struct crypto_alg *crypto_mod_get(struct crypto_alg *alg)
    {
    return try_module_get(alg.cra_module) ? crypto_alg_get(alg) : core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(crypto_mod_get);
#[no_mangle]
pub unsafe extern "C" fn crypto_mod_put(alg: *mut crypto_alg) {
    void crypto_mod_put(struct crypto_alg *alg)
    {
    struct module *module = alg.cra_module;
    crypto_alg_put(alg);
    module_put(module);
    }
    EXPORT_SYMBOL_GPL(crypto_mod_put);
    static struct crypto_alg *__crypto_alg_lookup(const char *name, u32 type,
    u32 mask)
    __must_hold_shared(&crypto_alg_sem)
    {
    struct crypto_alg *q, *alg = core::ptr::null_mut();
    let mut best: c_int = -2;
    list_for_each_entry(q, &crypto_alg_list, cra_list) {
    int exact, fuzzy;
    if (crypto_is_moribund(q))
    continue;
    if ((q.cra_flags ^ type) & mask)
    continue;
    exact = !strcmp(q.cra_driver_name, name);
    fuzzy = !strcmp(q.cra_name, name);
    if (!exact && !(fuzzy && q.cra_priority > best))
    continue;
    if (unlikely(!crypto_mod_get(q)))
    continue;
    best = q.cra_priority;
    if (alg)
    crypto_mod_put(alg);
    alg = q;
    if (exact)
    break;
    }
    return alg;
    }
#[no_mangle]
unsafe extern "C" fn crypto_larval_destroy(alg: *mut crypto_alg) {
    static void crypto_larval_destroy(struct crypto_alg *alg)
    {
    struct crypto_larval *larval = (void *)alg;
    BUG_ON(!crypto_is_larval(alg));
    if (!IS_ERR_OR_NULL(larval.adult))
    crypto_mod_put(larval.adult);
    kfree(larval);
    }
    struct crypto_larval *crypto_larval_alloc(const char *name, u32 type, u32 mask)
    {
    struct crypto_larval *larval;
    larval = kzalloc_obj(*larval);
    if (!larval)
    return ERR_PTR(-ENOMEM);
    type &= ~CRYPTO_ALG_TYPE_MASK | (mask ?: CRYPTO_ALG_TYPE_MASK);
    larval.mask = mask;
    larval.alg.cra_flags = CRYPTO_ALG_LARVAL | type;
    larval.alg.cra_priority = -1;
    larval.alg.cra_destroy = crypto_larval_destroy;
    strscpy(larval.alg.cra_name, name);
    init_completion(&larval.completion);
    return larval;
    }
    EXPORT_SYMBOL_GPL(crypto_larval_alloc);
    static struct crypto_alg *crypto_larval_add(const char *name, u32 type,
    u32 mask)
    {
    struct crypto_alg *alg;
    struct crypto_larval *larval;
    larval = crypto_larval_alloc(name, type, mask);
    if (IS_ERR(larval))
    return ERR_CAST(larval);
    refcount_set(&larval.alg.cra_refcnt, 2);
    down_write(&crypto_alg_sem);
    alg = __crypto_alg_lookup(name, type, mask);
    if (!alg) {
    alg = &larval.alg;
    list_add(&alg.cra_list, &crypto_alg_list);
    }
    up_write(&crypto_alg_sem);
    if (alg != &larval.alg) {
    kfree(larval);
    if (crypto_is_larval(alg))
    alg = crypto_larval_wait(alg, type, mask);
    }
    return alg;
    }
#[no_mangle]
unsafe extern "C" fn crypto_larval_kill(larval: *mut crypto_larval) {
    static void crypto_larval_kill(struct crypto_larval *larval)
    {
    bool unlinked;
    down_write(&crypto_alg_sem);
    unlinked = list_empty(&larval.alg.cra_list);
    if (!unlinked)
    list_del_init(&larval.alg.cra_list);
    up_write(&crypto_alg_sem);
    if (unlinked)
    return;
    complete_all(&larval.completion);
    crypto_alg_put(&larval.alg);
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_schedule_test(larval: *mut crypto_larval) {
    void crypto_schedule_test(struct crypto_larval *larval)
    {
    int err;
    err = crypto_probing_notify(CRYPTO_MSG_ALG_REGISTER, larval.adult);
    WARN_ON_ONCE(err != NOTIFY_STOP);
    }
    EXPORT_SYMBOL_GPL(crypto_schedule_test);
#[no_mangle]
unsafe extern "C" fn crypto_start_test(larval: *mut crypto_larval) {
    static void crypto_start_test(struct crypto_larval *larval)
    {
    if (!crypto_is_test_larval(larval))
    return;
    if (larval.test_started)
    return;
    down_write(&crypto_alg_sem);
    if (larval.test_started) {
    up_write(&crypto_alg_sem);
    return;
    }
    larval.test_started = true;
    up_write(&crypto_alg_sem);
    crypto_schedule_test(larval);
    }
    static struct crypto_alg *crypto_larval_wait(struct crypto_alg *alg,
    u32 type, u32 mask)
    {
    struct crypto_larval *larval;
    long time_left;
    again:
    larval = container_of(alg, struct crypto_larval, alg);
    if (!crypto_boot_test_finished())
    crypto_start_test(larval);
    time_left = wait_for_completion_killable_timeout(
    &larval.completion, 60 * HZ);
    alg = larval.adult;
    if (time_left < 0)
    alg = ERR_PTR(-EINTR);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !time_left) -> else {
    if (crypto_is_test_larval(larval))
    crypto_larval_kill(larval);
    alg = ERR_PTR(-ETIMEDOUT);
    } else if (!alg || PTR_ERR(alg) == -EEXIST) {
    let mut err: c_int = alg ? -EEXIST : -EAGAIN;
//
// EEXIST is expected because two probes can be scheduled
// at the same time with one using alg_name and the other
// using driver_name.  Do a re-lookup but do not retry in
// case we hit a quirk like gcm_base(ctr(aes),...) which
// will never match.
//
    alg = &larval.alg;
    alg = crypto_alg_lookup(alg.cra_name, type, mask) ?:
    ERR_PTR(err);
    } else if (IS_ERR(alg))
    ;
    else if (crypto_is_test_larval(larval) &&
    !(alg.cra_flags & CRYPTO_ALG_TESTED))
    alg = ERR_PTR(-EAGAIN);
#[no_mangle]
pub unsafe extern "C" fn if(CRYPTO_ALG_FIPS_INTERNAL: alg->cra_flags &) -> else {
    else if (alg.cra_flags & CRYPTO_ALG_FIPS_INTERNAL)
    alg = ERR_PTR(-EAGAIN);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !crypto_mod_get(alg)) -> else {
    else if (!crypto_mod_get(alg))
    alg = ERR_PTR(-EAGAIN);
    crypto_mod_put(&larval.alg);
    if (!IS_ERR(alg) && crypto_is_larval(alg))
    goto again;
    return alg;
    }
    static struct crypto_alg *crypto_alg_lookup(const char *name, u32 type,
    u32 mask)
    {
    let mut fips: u32 = CRYPTO_ALG_FIPS_INTERNAL;
    struct crypto_alg *alg;
    let mut test: u32 = 0;
    if (!((type | mask) & CRYPTO_ALG_TESTED))
    test |= CRYPTO_ALG_TESTED;
    down_read(&crypto_alg_sem);
    alg = __crypto_alg_lookup(name, (type | test) & ~fips,
    (mask | test) & ~fips);
    if (alg) {
    if (((type | mask) ^ fips) & fips)
    mask |= fips;
    mask &= fips;
    if (!crypto_is_larval(alg) &&
    ((type ^ alg.cra_flags) & mask)) {
// Algorithm is disallowed in FIPS mode.
    crypto_mod_put(alg);
    alg = ERR_PTR(-ENOENT);
    }
    } else if (test) {
    alg = __crypto_alg_lookup(name, type, mask);
    if (alg && !crypto_is_larval(alg)) {
// Test failed
    crypto_mod_put(alg);
    alg = ERR_PTR(-ELIBBAD);
    }
    }
    up_read(&crypto_alg_sem);
    return alg;
    }
    static struct crypto_alg *crypto_larval_lookup(const char *name, u32 type,
    u32 mask)
    {
    struct crypto_alg *alg;
    if (!name)
    return ERR_PTR(-ENOENT);
    type &= ~(CRYPTO_ALG_LARVAL | CRYPTO_ALG_DEAD);
    mask &= ~(CRYPTO_ALG_LARVAL | CRYPTO_ALG_DEAD);
    alg = crypto_alg_lookup(name, type, mask);
    if (!alg && !(mask & CRYPTO_NOLOAD)) {
    request_module("crypto-%s", name);
    if (!((type ^ CRYPTO_ALG_NEED_FALLBACK) & mask &
    CRYPTO_ALG_NEED_FALLBACK))
    request_module("crypto-%s-all", name);
    alg = crypto_alg_lookup(name, type, mask);
    }
    if (!IS_ERR_OR_NULL(alg) && crypto_is_larval(alg))
    alg = crypto_larval_wait(alg, type, mask);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: alg) -> else {
    else if (alg)
    ;
#[no_mangle]
pub unsafe extern "C" fn if(CRYPTO_ALG_TESTED): !(mask &) -> else {
    else if (!(mask & CRYPTO_ALG_TESTED))
    alg = crypto_larval_add(name, type, mask);
    else
    alg = ERR_PTR(-ENOENT);
    return alg;
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_probing_notify(val: c_ulong, v: *mut c_void) -> c_int {
    int crypto_probing_notify(unsigned long val, void *v)
    {
    int ok;
    ok = blocking_notifier_call_chain(&crypto_chain, val, v);
    if (ok == NOTIFY_DONE) {
    request_module("cryptomgr");
    ok = blocking_notifier_call_chain(&crypto_chain, val, v);
    }
    return ok;
    }
    EXPORT_SYMBOL_GPL(crypto_probing_notify);
    struct crypto_alg *crypto_alg_mod_lookup(const char *name, u32 type, u32 mask)
    {
    struct crypto_alg *alg;
    struct crypto_alg *larval;
    int ok;
//
// If the internal flag is set for a cipher, require a caller to
// invoke the cipher with the internal flag to use that cipher.
// Also, if a caller wants to allocate a cipher that may or may
// not be an internal cipher, use type | CRYPTO_ALG_INTERNAL and
// !(mask & CRYPTO_ALG_INTERNAL).
//
    if (!((type | mask) & CRYPTO_ALG_INTERNAL))
    mask |= CRYPTO_ALG_INTERNAL;
    larval = crypto_larval_lookup(name, type, mask);
    if (IS_ERR(larval) || !crypto_is_larval(larval))
    return larval;
    ok = crypto_probing_notify(CRYPTO_MSG_ALG_REQUEST, larval);
    if (ok == NOTIFY_STOP)
    alg = crypto_larval_wait(larval, type, mask);
    else {
    crypto_mod_put(larval);
    alg = ERR_PTR(-ENOENT);
    }
    crypto_larval_kill(container_of(larval, struct crypto_larval, alg));
    return alg;
    }
    EXPORT_SYMBOL_GPL(crypto_alg_mod_lookup);
#[no_mangle]
unsafe extern "C" fn crypto_exit_ops(tfm: *mut crypto_tfm) {
    static void crypto_exit_ops(struct crypto_tfm *tfm)
    {
    const struct crypto_type *type = tfm.__crt_alg.cra_type;
    if (type && tfm.exit)
    tfm.exit(tfm);
    }
#[no_mangle]
unsafe extern "C" fn crypto_ctxsize(alg: *mut crypto_alg, type: u32, mask: u32) -> c_uint {
    static unsigned int crypto_ctxsize(struct crypto_alg *alg, u32 type, u32 mask)
    {
    const struct crypto_type *type_obj = alg.cra_type;
    unsigned int len;
    len = alg.cra_alignmask & ~(crypto_tfm_ctx_alignment() - 1);
    if (type_obj)
    return len + type_obj.ctxsize(alg, type, mask);
    switch (alg.cra_flags & CRYPTO_ALG_TYPE_MASK) {
    default:
    BUG();
    case CRYPTO_ALG_TYPE_CIPHER:
    len += crypto_cipher_ctxsize(alg);
    break;
    }
    return len;
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_shoot_alg(alg: *mut crypto_alg) {
    void crypto_shoot_alg(struct crypto_alg *alg)
    {
    down_write(&crypto_alg_sem);
    alg.cra_flags |= CRYPTO_ALG_DYING;
    up_write(&crypto_alg_sem);
    }
    EXPORT_SYMBOL_GPL(crypto_shoot_alg);
    struct crypto_tfm *__crypto_alloc_tfm(struct crypto_alg *alg, u32 type,
    u32 mask)
    {
    struct crypto_tfm *tfm;
    unsigned int tfm_size;
    let mut err: c_int = -ENOMEM;
    tfm_size = sizeof(*tfm) + crypto_ctxsize(alg, type, mask);
    tfm = kzalloc(tfm_size, GFP_KERNEL);
    if (tfm == core::ptr::null_mut())
    goto out_err;
    tfm.__crt_alg = alg;
    if (!tfm.exit && alg.cra_init && (err = alg.cra_init(tfm)))
    goto cra_init_failed;
    goto out;
    cra_init_failed:
    crypto_exit_ops(tfm);
    if (err == -EAGAIN)
    crypto_shoot_alg(alg);
    kfree(tfm);
    out_err:
    tfm = ERR_PTR(err);
    out:
    return tfm;
    }
    EXPORT_SYMBOL_GPL(__crypto_alloc_tfm);
//
// crypto_alloc_base - Locate algorithm and allocate transform
// @alg_name: Name of algorithm
// @type: Type of algorithm
// @mask: Mask for type comparison
//
// This function should not be used by new algorithm types.
// Please use crypto_alloc_tfm instead.
//
// crypto_alloc_base() will first attempt to locate an already loaded
// algorithm.  If that fails and the kernel supports dynamically loadable
// modules, it will then attempt to load a module of the same name or
// alias.  If that fails it will send a query to any loaded crypto manager
// to construct an algorithm on the fly.  A refcount is grabbed on the
// algorithm which is then associated with the new transform.
//
// The returned transform is of a non-determinate type.  Most people
// should use one of the more specific allocation functions such as
// crypto_alloc_skcipher().
//
// In case of error the return value is an error pointer.
//
    struct crypto_tfm *crypto_alloc_base(const char *alg_name, u32 type, u32 mask)
    {
    struct crypto_tfm *tfm;
    int err;
    for (;;) {
    struct crypto_alg *alg;
    alg = crypto_alg_mod_lookup(alg_name, type, mask);
    if (IS_ERR(alg)) {
    err = PTR_ERR(alg);
    goto err;
    }
    tfm = __crypto_alloc_tfm(alg, type, mask);
    if (!IS_ERR(tfm))
    return tfm;
    crypto_mod_put(alg);
    err = PTR_ERR(tfm);
    err:
    if (err != -EAGAIN)
    break;
    if (fatal_signal_pending(current)) {
    err = -EINTR;
    break;
    }
    }
    return ERR_PTR(err);
    }
    EXPORT_SYMBOL_GPL(crypto_alloc_base);
    void *crypto_create_tfm_node(struct crypto_alg *alg,
    const struct crypto_type *frontend,
    int node)
    {
    struct crypto_tfm *tfm;
    size_t size;
    char *mem;
    int err;
    size = frontend.tfmsize + sizeof(*tfm) + frontend.extsize(alg);
    mem = kzalloc_node(size, GFP_KERNEL, node);
    if (!mem)
    return ERR_PTR(-ENOMEM);
    tfm = (struct crypto_tfm *)(mem + frontend.tfmsize);
    tfm.__crt_alg = alg;
    tfm.node = node;
    tfm.fb = tfm;
    err = frontend.init_tfm(tfm);
    if (err)
    goto out_free_tfm;
    if (!tfm.exit && alg.cra_init && (err = alg.cra_init(tfm)))
    goto cra_init_failed;
    goto out;
    cra_init_failed:
    crypto_exit_ops(tfm);
    out_free_tfm:
    if (err == -EAGAIN)
    crypto_shoot_alg(alg);
    kfree(mem);
    mem = ERR_PTR(err);
    out:
    return mem;
    }
    EXPORT_SYMBOL_GPL(crypto_create_tfm_node);
    struct crypto_alg *crypto_find_alg(const char *alg_name,
    const struct crypto_type *frontend,
    u32 type, u32 mask)
    {
    if (frontend) {
    type &= frontend.maskclear;
    mask &= frontend.maskclear;
    type |= frontend.type;
    mask |= frontend.maskset;
    }
    return crypto_alg_mod_lookup(alg_name, type, mask);
    }
    EXPORT_SYMBOL_GPL(crypto_find_alg);
//
// crypto_alloc_tfm_node - Locate algorithm and allocate transform
// @alg_name: Name of algorithm
// @frontend: Frontend algorithm type
// @type: Type of algorithm
// @mask: Mask for type comparison
// @node: NUMA node in which users desire to put requests, if node is
// NUMA_NO_NODE, it means users have no special requirement.
//
// crypto_alloc_tfm() will first attempt to locate an already loaded
// algorithm.  If that fails and the kernel supports dynamically loadable
// modules, it will then attempt to load a module of the same name or
// alias.  If that fails it will send a query to any loaded crypto manager
// to construct an algorithm on the fly.  A refcount is grabbed on the
// algorithm which is then associated with the new transform.
//
// The returned transform is of a non-determinate type.  Most people
// should use one of the more specific allocation functions such as
// crypto_alloc_skcipher().
//
// In case of error the return value is an error pointer.
//
    void *crypto_alloc_tfm_node(const char *alg_name,
    const struct crypto_type *frontend, u32 type, u32 mask,
    int node)
    {
    void *tfm;
    int err;
    for (;;) {
    struct crypto_alg *alg;
    alg = crypto_find_alg(alg_name, frontend, type, mask);
    if (IS_ERR(alg)) {
    err = PTR_ERR(alg);
    goto err;
    }
    tfm = crypto_create_tfm_node(alg, frontend, node);
    if (!IS_ERR(tfm))
    return tfm;
    crypto_mod_put(alg);
    err = PTR_ERR(tfm);
    err:
    if (err != -EAGAIN)
    break;
    if (fatal_signal_pending(current)) {
    err = -EINTR;
    break;
    }
    }
    return ERR_PTR(err);
    }
    EXPORT_SYMBOL_GPL(crypto_alloc_tfm_node);
//
// crypto_destroy_tfm - Free crypto transform
// @mem: Start of tfm slab
// @tfm: Transform to free
//
// This function frees up the transform and any associated resources,
// then drops the refcount on the associated algorithm.
//
#[no_mangle]
pub unsafe extern "C" fn crypto_destroy_tfm(mem: *mut c_void, tfm: *mut crypto_tfm) {
    void crypto_destroy_tfm(void *mem, struct crypto_tfm *tfm)
    {
    struct crypto_alg *alg;
    if (IS_ERR_OR_NULL(mem))
    return;
    alg = tfm.__crt_alg;
    if (!tfm.exit && alg.cra_exit)
    alg.cra_exit(tfm);
    crypto_exit_ops(tfm);
    crypto_mod_put(alg);
    kfree_sensitive(mem);
    }
    EXPORT_SYMBOL_GPL(crypto_destroy_tfm);
#[no_mangle]
pub unsafe extern "C" fn crypto_has_alg(name: *const c_char, type: u32, mask: u32) -> c_int {
    int crypto_has_alg(const char *name, u32 type, u32 mask)
    {
    let mut ret: c_int = 0;
    struct crypto_alg *alg = crypto_alg_mod_lookup(name, type, mask);
    if (!IS_ERR(alg)) {
    crypto_mod_put(alg);
    ret = 1;
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(crypto_has_alg);
#[no_mangle]
pub unsafe extern "C" fn crypto_req_done(data: *mut c_void, err: c_int) {
    void crypto_req_done(void *data, int err)
    {
    struct crypto_wait *wait = data;
    if (err == -EINPROGRESS)
    return;
    wait.err = err;
    complete(&wait.completion);
    }
    EXPORT_SYMBOL_GPL(crypto_req_done);
#[no_mangle]
pub unsafe extern "C" fn crypto_destroy_alg(alg: *mut crypto_alg) {
    void crypto_destroy_alg(struct crypto_alg *alg)
    {
    if (alg.cra_type && alg.cra_type.destroy)
    alg.cra_type.destroy(alg);
    if (alg.cra_destroy)
    alg.cra_destroy(alg);
    }
    EXPORT_SYMBOL_GPL(crypto_destroy_alg);
    struct crypto_async_request *crypto_request_clone(
    struct crypto_async_request *req, size_t total, gfp_t gfp)
    {
    struct crypto_tfm *tfm = req.tfm;
    struct crypto_async_request *nreq;
    nreq = kmemdup(req, total, gfp);
    if (!nreq) {
    req.tfm = tfm.fb;
    return req;
    }
    nreq.flags &= ~CRYPTO_TFM_REQ_ON_STACK;
    return nreq;
    }
    EXPORT_SYMBOL_GPL(crypto_request_clone);
    MODULE_DESCRIPTION("Cryptographic core API");
    MODULE_LICENSE("GPL");
