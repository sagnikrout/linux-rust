//! Automatically rewritten from C to Rust
//! Source: arch/s390/crypto/paes_s390.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Cryptographic API.
//
// s390 implementation of the AES Cipher Algorithm with protected keys.
//
// s390 Version:
// Copyright IBM Corp. 2017, 2025
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
// Harald Freudenberger <freude@de.ibm.com>
//

//
// Key blobs smaller/bigger than these defines are rejected
// by the common code even before the individual setkey function
// is called. As paes can handle different kinds of key blobs
// and padding is also possible, the limits need to be generous.
//
pub const PAES_MIN_KEYSIZE: c_int = 16;

    static bool pkey_clrkey_allowed;
    module_param_named(clrkey, pkey_clrkey_allowed, bool, 0444);
    MODULE_PARM_DESC(clrkey, "Allow clear key material (default N)");
    static u8 *ctrblk;
    static DEFINE_MUTEX(ctrblk_lock);
    static cpacf_mask_t km_functions, kmc_functions, kmctr_functions;
    static struct crypto_engine *paes_crypto_engine;
pub const MAX_QLEN: c_int = 10;
//
// protected key specific stuff
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct paes_protkey {
    pub type: u32,
    pub len: u32,
    pub protkey: [u8; PXTS_256_PROTKEY_SIZE],
}

pub const PK_STATE_NO_KEY: c_int = 0;
pub const PK_STATE_CONVERT_IN_PROGRESS: c_int = 1;
pub const PK_STATE_VALID: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_paes_ctx {
// source key material used to derive a protected key from
    pub keybuf: [u8; PAES_MAX_KEYSIZE],
    pub keylen: c_uint,
// cpacf function code to use with this protected key type
    pub fc: c_long,
// nr of requests enqueued via crypto engine which use this tfm ctx
    pub via_engine_ctr: core::sync::atomic::AtomicI32,
// spinlock to atomic read/update all the following fields
    pub pk_lock: spinlock_t,
// see PK_STATE* defines above, < 0 holds convert failure rc
    pub pk_state: c_int,
// if state is valid, pk holds the protected key
    pub pk: paes_protkey,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_pxts_ctx {
// source key material used to derive a protected key from
    pub PAES_MAX_KEYSIZE]: *mut *mut u8 keybuf[2,
    pub keylen: c_uint,
// cpacf function code to use with this protected key type
    pub fc: c_long,
// nr of requests enqueued via crypto engine which use this tfm ctx
    pub via_engine_ctr: core::sync::atomic::AtomicI32,
// spinlock to atomic read/update all the following fields
    pub pk_lock: spinlock_t,
// see PK_STATE* defines above, < 0 holds convert failure rc
    pub pk_state: c_int,
// if state is valid, pk[] hold(s) the protected key(s)
    pub pk: [paes_protkey; 2],
}

//
// make_clrkey_token() - wrap the raw key ck with pkey clearkey token
// information.
// @returns the size of the clearkey token
//
#[no_mangle]
pub unsafe extern "C" fn make_clrkey_token(ck: *const u8, cklen: usize, dest: *mut u8) -> u32 {
    static inline u32 make_clrkey_token(const u8 *ck, size_t cklen, u8 *dest)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clrkey_token {
    pub type: u8,
    pub res0: [u8; 3],
    pub version: u8,
    pub res1: [u8; 3],
    pub keytype: u32,
    pub len: u32,
    pub key: [u8; ],
    pub )dest: *mut *mut } __packed token = (struct clrkey_token,
    pub 0x00: token->type =,
    pub 0x02: token->version =,
    pub 3: token->keytype = (cklen - 8) >>,
    pub cklen: token->len =,
    pub cklen): memcpy(token->key, ck,,
    pub cklen: *mut *mut return sizeof(token) +,
    }
//
// paes_ctx_setkey() - Set key value into context, maybe construct
// a clear key token digestible by pkey from a clear key value.
//
    static inline int paes_ctx_setkey(struct s390_paes_ctx *ctx,
    const u8 *key, unsigned int keylen)
    {
    if (keylen > sizeof(ctx.keybuf))
    pub -EINVAL: return,
    switch (keylen) {
    case 16:
    case 24:
    case 32:
// clear key value, prepare pkey clear key token in keybuf
    pub sizeof(ctx->keybuf)): memset(ctx->keybuf, 0,,
    pub ctx->keybuf): ctx->keylen = make_clrkey_token(key, keylen,,
    default:
// other key material, let pkey handle this
    pub keylen): memcpy(ctx->keybuf, key,,
    pub keylen: ctx->keylen =,
    }
    pub 0: return,
    }
//
// pxts_ctx_setkey() - Set key value into context, maybe construct
// a clear key token digestible by pkey from a clear key value.
//
    static inline int pxts_ctx_setkey(struct s390_pxts_ctx *ctx,
    const u8 *key, unsigned int keylen)
    {
    pub 2: size_t cklen = keylen /,
    if (keylen > sizeof(ctx.keybuf))
    pub -EINVAL: return,
    switch (keylen) {
    case 32:
    case 64:
// clear key value, prepare pkey clear key tokens in keybuf
    pub sizeof(ctx->keybuf)): memset(ctx->keybuf, 0,,
    pub ctx->keybuf): ctx->keylen = make_clrkey_token(key, cklen,,
    ctx.keylen += make_clrkey_token(key + cklen, cklen,
    pub ctx->keylen): ctx->keybuf +,
    default:
// other key material, let pkey handle this
    pub keylen): memcpy(ctx->keybuf, key,,
    pub keylen: ctx->keylen =,
    }
    pub 0: return,
    }
//
// Convert the raw key material into a protected key via PKEY api.
// This function may sleep - don't call in non-sleeping context.
//
    static inline int convert_key(const u8 *key, unsigned int keylen,
    struct paes_protkey *pk, bool tested)
    {
    pub PKEY_XFLAG_NOMEMALLOC: u32 xflags =,
    pub i: int rc,,
    if (tested && !pkey_clrkey_allowed)
    pub PKEY_XFLAG_NOCLEARKEY: xflags |=,
    pub sizeof(pk->protkey): pk->len =,
//
// In case of a busy card retry with increasing delay
// of 200, 400, 800 and 1600 ms - in total 3 s.
//
    pub {: for (rc = -EIO, i = 0; rc && i < 5; i++),
    if (rc == -EBUSY && msleep_interruptible((1 << i) * 100)) {
    pub -EINTR: rc =,
    pub out: goto,
    }
    rc = pkey_key2protkey(key, keylen,
    pk.protkey, &pk.len, &pk.type,
    }
    out:
    pub rc): pr_debug("rc=%d\n",,
    pub rc: return,
    }
//
// (Re-)Convert the raw key material from the ctx into a protected key
// via convert_key() function. Update the pk_state, pk_type, pk_len
// and the protected key in the tfm context.
// Please note this function may be invoked concurrently with the very
// same tfm context. The pk_lock spinlock in the context ensures an
// atomic update of the pk and the pk state but does not guarantee any
// order of update. So a fresh converted valid protected key may get
// updated with an 'old' expired key value. As the cpacf instructions
// detect this, refuse to operate with an invalid key and the calling
// code triggers a (re-)conversion this does no harm. This may lead to
// unnecessary additional conversion but never to invalid data on en-
// or decrypt operations.
//
#[no_mangle]
unsafe extern "C" fn paes_convert_key(ctx: *mut s390_paes_ctx, tested: bool) -> c_int {
    static int paes_convert_key(struct s390_paes_ctx *ctx, bool tested)
    {
    pub pk: paes_protkey,
    pub rc: c_int,
    pub PK_STATE_CONVERT_IN_PROGRESS: ctx->pk_state =,
    pub tested): rc = convert_key(ctx->keybuf, ctx->keylen, &pk,,
// update context
    if (rc) {
    pub rc: ctx->pk_state =,
    } else {
    pub PK_STATE_VALID: ctx->pk_state =,
    pub pk: ctx->pk =,
    }
    pub sizeof(pk)): memzero_explicit(&pk,,
    pub rc): pr_debug("rc=%d\n",,
    pub rc: return,
    }
//
// (Re-)Convert the raw xts key material from the ctx into a
// protected key via convert_key() function. Update the pk_state,
// pk_type, pk_len and the protected key in the tfm context.
// See also comments on function paes_convert_key.
//
#[no_mangle]
unsafe extern "C" fn pxts_convert_key(ctx: *mut s390_pxts_ctx, tested: bool) -> c_int {
    static int pxts_convert_key(struct s390_pxts_ctx *ctx, bool tested)
    {
    pub pk1: paes_protkey pk0,,
    pub split_keylen: usize,
    pub rc: c_int,
    pub PK_STATE_CONVERT_IN_PROGRESS: ctx->pk_state =,
    pub tested): rc = convert_key(ctx->keybuf, ctx->keylen, &pk0,,
    if (rc)
    pub out: goto,
    switch (pk0.type) {
    case PKEY_KEYTYPE_AES_128:
    case PKEY_KEYTYPE_AES_256:
// second keytoken required
    if (ctx.keylen % 2) {
    pub -EINVAL: rc =,
    pub out: goto,
    }
    pub 2: split_keylen = ctx->keylen /,
    rc = convert_key(ctx.keybuf + split_keylen,
    pub tested): split_keylen, &pk1,,
    if (rc)
    pub out: goto,
    if (pk0.type != pk1.type) {
    pub -EINVAL: rc =,
    pub out: goto,
    }
    case PKEY_KEYTYPE_AES_XTS_128:
    case PKEY_KEYTYPE_AES_XTS_256:
// single key
    pub 0: pk1.type =,
    default:
// unsupported protected keytype
    pub -EINVAL: rc =,
    pub out: goto,
    }
    out:
// update context
    if (rc) {
    pub rc: ctx->pk_state =,
    } else {
    pub PK_STATE_VALID: ctx->pk_state =,
    pub pk0: ctx->pk[0] =,
    pub pk1: ctx->pk[1] =,
    }
    pub sizeof(pk0)): memzero_explicit(&pk0,,
    pub sizeof(pk1)): memzero_explicit(&pk1,,
    pub rc): pr_debug("rc=%d\n",,
    pub rc: return,
    }
//
// PAES ECB implementation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecb_param {
    pub key: [u8; PAES_256_PROTKEY_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_pecb_req_ctx {
    pub modifier: c_ulong,
    pub walk: skcipher_walk,
    pub param_init_done: bool,
    pub param: ecb_param,
}

    static int ecb_paes_setkey(struct crypto_skcipher *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct s390_paes_ctx *ctx = crypto_skcipher_ctx(tfm);
    let mut tested: bool = crypto_skcipher_tested(tfm);
    long fc;
    int rc;
// set raw key into context
    rc = paes_ctx_setkey(ctx, in_key, key_len);
    if (rc)
    goto out;
// convert key into protected key
    rc = paes_convert_key(ctx, tested);
    if (rc)
    goto out;
// Pick the correct function code based on the protected key type
    switch (ctx.pk.type) {
    case PKEY_KEYTYPE_AES_128:
    fc = CPACF_KM_PAES_128;
    break;
    case PKEY_KEYTYPE_AES_192:
    fc = CPACF_KM_PAES_192;
    break;
    case PKEY_KEYTYPE_AES_256:
    fc = CPACF_KM_PAES_256;
    break;
    default:
    fc = 0;
    break;
    }
    ctx.fc = (fc && cpacf_test_func(&km_functions, fc)) ? fc : 0;
    rc = fc ? 0 : -EINVAL;
    out:
    pr_debug("rc=%d\n", rc);
    return rc;
    }
    static int ecb_paes_do_crypt(struct s390_paes_ctx *ctx,
    struct s390_pecb_req_ctx *req_ctx,
    bool tested, bool maysleep)
    {
    struct ecb_param *param = &req_ctx.param;
    struct skcipher_walk *walk = &req_ctx.walk;
    unsigned int nbytes, n, k;
    int pk_state, rc = 0;
    if (!req_ctx.param_init_done) {
// fetch and check protected key state
    spin_lock_bh(&ctx.pk_lock);
    pk_state = ctx.pk_state;
    switch (pk_state) {
    case PK_STATE_NO_KEY:
    rc = -ENOKEY;
    break;
    case PK_STATE_CONVERT_IN_PROGRESS:
    rc = -EKEYEXPIRED;
    break;
    case PK_STATE_VALID:
    memcpy(param.key, ctx.pk.protkey, sizeof(param.key));
    req_ctx.param_init_done = true;
    break;
    default:
    rc = pk_state < 0 ? pk_state : -EIO;
    break;
    }
    spin_unlock_bh(&ctx.pk_lock);
    }
    if (rc)
    goto out;
//
// Note that in case of partial processing or failure the walk
// is NOT unmapped here. So a follow up task may reuse the walk
// or in case of unrecoverable failure needs to unmap it.
//
    while ((nbytes = walk.nbytes) != 0) {
// only use complete blocks
    n = nbytes & ~(AES_BLOCK_SIZE - 1);
    k = cpacf_km(ctx.fc | req_ctx.modifier, param,
    walk.dst.virt.addr, walk.src.virt.addr, n);
    if (k)
    rc = skcipher_walk_done(walk, nbytes - k);
    if (k < n) {
    if (!maysleep) {
    rc = -EKEYEXPIRED;
    goto out;
    }
    rc = paes_convert_key(ctx, tested);
    if (rc)
    goto out;
    spin_lock_bh(&ctx.pk_lock);
    memcpy(param.key, ctx.pk.protkey, sizeof(param.key));
    spin_unlock_bh(&ctx.pk_lock);
    }
    }
    out:
    pr_debug("rc=%d\n", rc);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ecb_paes_crypt(req: *mut skcipher_request, modifier: c_ulong) -> c_int {
    static int ecb_paes_crypt(struct skcipher_request *req, unsigned long modifier)
    {
    struct s390_pecb_req_ctx *req_ctx = skcipher_request_ctx(req);
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct s390_paes_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk *walk = &req_ctx.walk;
    let mut tested: bool = crypto_skcipher_tested(tfm);
    int rc;
//
// Attempt synchronous encryption first. If it fails, schedule the request
// asynchronously via the crypto engine. To preserve execution order,
// once a request is queued to the engine, further requests using the same
// tfm will also be routed through the engine.
//
    rc = skcipher_walk_virt(walk, req, false);
    if (rc)
    goto out;
    req_ctx.modifier = modifier;
    req_ctx.param_init_done = false;
// Try synchronous operation if no active engine usage
    if (!atomic_read(&ctx.via_engine_ctr)) {
    rc = ecb_paes_do_crypt(ctx, req_ctx, tested, false);
    if (rc == 0)
    goto out;
    }
//
// If sync operation failed or key expired or there are already
// requests enqueued via engine, fallback to async. Mark tfm as
// using engine to serialize requests.
//
    if (rc == 0 || rc == -EKEYEXPIRED) {
    atomic_inc(&ctx.via_engine_ctr);
    rc = crypto_transfer_skcipher_request_to_engine(paes_crypto_engine, req);
    if (rc != -EINPROGRESS)
    atomic_dec(&ctx.via_engine_ctr);
    }
    if (rc != -EINPROGRESS)
    skcipher_walk_done(walk, rc);
    out:
    if (rc != -EINPROGRESS)
    memzero_explicit(&req_ctx.param, sizeof(req_ctx.param));
    pr_debug("rc=%d\n", rc);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ecb_paes_encrypt(req: *mut skcipher_request) -> c_int {
    static int ecb_paes_encrypt(struct skcipher_request *req)
    {
    return ecb_paes_crypt(req, 0);
    }
#[no_mangle]
unsafe extern "C" fn ecb_paes_decrypt(req: *mut skcipher_request) -> c_int {
    static int ecb_paes_decrypt(struct skcipher_request *req)
    {
    return ecb_paes_crypt(req, CPACF_DECRYPT);
    }
#[no_mangle]
unsafe extern "C" fn ecb_paes_init(tfm: *mut crypto_skcipher) -> c_int {
    static int ecb_paes_init(struct crypto_skcipher *tfm)
    {
    struct s390_paes_ctx *ctx = crypto_skcipher_ctx(tfm);
    memset(ctx, 0, sizeof(*ctx));
    spin_lock_init(&ctx.pk_lock);
    crypto_skcipher_set_reqsize(tfm, sizeof(struct s390_pecb_req_ctx));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ecb_paes_exit(tfm: *mut crypto_skcipher) {
    static void ecb_paes_exit(struct crypto_skcipher *tfm)
    {
    struct s390_paes_ctx *ctx = crypto_skcipher_ctx(tfm);
    memzero_explicit(ctx, sizeof(*ctx));
    }
#[no_mangle]
unsafe extern "C" fn ecb_paes_do_one_request(engine: *mut crypto_engine, areq: *mut c_void) -> c_int {
    static int ecb_paes_do_one_request(struct crypto_engine *engine, void *areq)
    {
    struct skcipher_request *req = skcipher_request_cast(areq);
    struct s390_pecb_req_ctx *req_ctx = skcipher_request_ctx(req);
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct s390_paes_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk *walk = &req_ctx.walk;
    let mut tested: bool = crypto_skcipher_tested(tfm);
    int rc;
// walk has already been prepared
    rc = ecb_paes_do_crypt(ctx, req_ctx, tested, true);
    if (rc == -EKEYEXPIRED) {
    return pkey_handle_expired();
    } else if (rc) {
    skcipher_walk_done(walk, rc);
    }
    memzero_explicit(&req_ctx.param, sizeof(req_ctx.param));
    pr_debug("request complete with rc=%d\n", rc);
    local_bh_disable();
    atomic_dec(&ctx.via_engine_ctr);
    crypto_finalize_skcipher_request(engine, req, rc);
    local_bh_enable();
    return rc;
    }
    static struct skcipher_engine_alg ecb_paes_alg = {
    .base = {
    .base.cra_name	      = "ecb(paes)",
    .base.cra_driver_name = "ecb-paes-s390",
    .base.cra_priority    = 401,	/* combo: aes + ecb + 1 */
    .base.cra_blocksize   = AES_BLOCK_SIZE,
    .base.cra_ctxsize     = sizeof(struct s390_paes_ctx),
    .base.cra_module      = THIS_MODULE,
    .base.cra_list	      = LIST_HEAD_INIT(ecb_paes_alg.base.base.cra_list),
    .init		      = ecb_paes_init,
    .exit		      = ecb_paes_exit,
    .min_keysize	      = PAES_MIN_KEYSIZE,
    .max_keysize	      = PAES_MAX_KEYSIZE,
    .setkey		      = ecb_paes_setkey,
    .encrypt	      = ecb_paes_encrypt,
    .decrypt	      = ecb_paes_decrypt,
    },
    .op = {
    .do_one_request	      = ecb_paes_do_one_request,
    },
    };
//
// PAES CBC implementation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cbc_param {
    pub iv: [u8; AES_BLOCK_SIZE],
    pub key: [u8; PAES_256_PROTKEY_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_pcbc_req_ctx {
    pub modifier: c_ulong,
    pub walk: skcipher_walk,
    pub param_init_done: bool,
    pub param: cbc_param,
}

    static int cbc_paes_setkey(struct crypto_skcipher *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct s390_paes_ctx *ctx = crypto_skcipher_ctx(tfm);
    let mut tested: bool = crypto_skcipher_tested(tfm);
    long fc;
    int rc;
// set raw key into context
    rc = paes_ctx_setkey(ctx, in_key, key_len);
    if (rc)
    goto out;
// convert raw key into protected key
    rc = paes_convert_key(ctx, tested);
    if (rc)
    goto out;
// Pick the correct function code based on the protected key type
    switch (ctx.pk.type) {
    case PKEY_KEYTYPE_AES_128:
    fc = CPACF_KMC_PAES_128;
    break;
    case PKEY_KEYTYPE_AES_192:
    fc = CPACF_KMC_PAES_192;
    break;
    case PKEY_KEYTYPE_AES_256:
    fc = CPACF_KMC_PAES_256;
    break;
    default:
    fc = 0;
    break;
    }
    ctx.fc = (fc && cpacf_test_func(&kmc_functions, fc)) ? fc : 0;
    rc = fc ? 0 : -EINVAL;
    out:
    pr_debug("rc=%d\n", rc);
    return rc;
    }
    static int cbc_paes_do_crypt(struct s390_paes_ctx *ctx,
    struct s390_pcbc_req_ctx *req_ctx,
    bool tested, bool maysleep)
    {
    struct cbc_param *param = &req_ctx.param;
    struct skcipher_walk *walk = &req_ctx.walk;
    unsigned int nbytes, n, k;
    int pk_state, rc = 0;
    if (!req_ctx.param_init_done) {
// fetch and check protected key state
    spin_lock_bh(&ctx.pk_lock);
    pk_state = ctx.pk_state;
    switch (pk_state) {
    case PK_STATE_NO_KEY:
    rc = -ENOKEY;
    break;
    case PK_STATE_CONVERT_IN_PROGRESS:
    rc = -EKEYEXPIRED;
    break;
    case PK_STATE_VALID:
    memcpy(param.key, ctx.pk.protkey, sizeof(param.key));
    req_ctx.param_init_done = true;
    break;
    default:
    rc = pk_state < 0 ? pk_state : -EIO;
    break;
    }
    spin_unlock_bh(&ctx.pk_lock);
    }
    if (rc)
    goto out;
    memcpy(param.iv, walk.iv, AES_BLOCK_SIZE);
//
// Note that in case of partial processing or failure the walk
// is NOT unmapped here. So a follow up task may reuse the walk
// or in case of unrecoverable failure needs to unmap it.
//
    while ((nbytes = walk.nbytes) != 0) {
// only use complete blocks
    n = nbytes & ~(AES_BLOCK_SIZE - 1);
    k = cpacf_kmc(ctx.fc | req_ctx.modifier, param,
    walk.dst.virt.addr, walk.src.virt.addr, n);
    if (k) {
    memcpy(walk.iv, param.iv, AES_BLOCK_SIZE);
    rc = skcipher_walk_done(walk, nbytes - k);
    }
    if (k < n) {
    if (!maysleep) {
    rc = -EKEYEXPIRED;
    goto out;
    }
    rc = paes_convert_key(ctx, tested);
    if (rc)
    goto out;
    spin_lock_bh(&ctx.pk_lock);
    memcpy(param.key, ctx.pk.protkey, sizeof(param.key));
    spin_unlock_bh(&ctx.pk_lock);
    }
    }
    out:
    pr_debug("rc=%d\n", rc);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn cbc_paes_crypt(req: *mut skcipher_request, modifier: c_ulong) -> c_int {
    static int cbc_paes_crypt(struct skcipher_request *req, unsigned long modifier)
    {
    struct s390_pcbc_req_ctx *req_ctx = skcipher_request_ctx(req);
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct s390_paes_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk *walk = &req_ctx.walk;
    let mut tested: bool = crypto_skcipher_tested(tfm);
    int rc;
//
// Attempt synchronous encryption first. If it fails, schedule the request
// asynchronously via the crypto engine. To preserve execution order,
// once a request is queued to the engine, further requests using the same
// tfm will also be routed through the engine.
//
    rc = skcipher_walk_virt(walk, req, false);
    if (rc)
    goto out;
    req_ctx.modifier = modifier;
    req_ctx.param_init_done = false;
// Try synchronous operation if no active engine usage
    if (!atomic_read(&ctx.via_engine_ctr)) {
    rc = cbc_paes_do_crypt(ctx, req_ctx, tested, false);
    if (rc == 0)
    goto out;
    }
//
// If sync operation failed or key expired or there are already
// requests enqueued via engine, fallback to async. Mark tfm as
// using engine to serialize requests.
//
    if (rc == 0 || rc == -EKEYEXPIRED) {
    atomic_inc(&ctx.via_engine_ctr);
    rc = crypto_transfer_skcipher_request_to_engine(paes_crypto_engine, req);
    if (rc != -EINPROGRESS)
    atomic_dec(&ctx.via_engine_ctr);
    }
    if (rc != -EINPROGRESS)
    skcipher_walk_done(walk, rc);
    out:
    if (rc != -EINPROGRESS)
    memzero_explicit(&req_ctx.param, sizeof(req_ctx.param));
    pr_debug("rc=%d\n", rc);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn cbc_paes_encrypt(req: *mut skcipher_request) -> c_int {
    static int cbc_paes_encrypt(struct skcipher_request *req)
    {
    return cbc_paes_crypt(req, 0);
    }
#[no_mangle]
unsafe extern "C" fn cbc_paes_decrypt(req: *mut skcipher_request) -> c_int {
    static int cbc_paes_decrypt(struct skcipher_request *req)
    {
    return cbc_paes_crypt(req, CPACF_DECRYPT);
    }
#[no_mangle]
unsafe extern "C" fn cbc_paes_init(tfm: *mut crypto_skcipher) -> c_int {
    static int cbc_paes_init(struct crypto_skcipher *tfm)
    {
    struct s390_paes_ctx *ctx = crypto_skcipher_ctx(tfm);
    memset(ctx, 0, sizeof(*ctx));
    spin_lock_init(&ctx.pk_lock);
    crypto_skcipher_set_reqsize(tfm, sizeof(struct s390_pcbc_req_ctx));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cbc_paes_exit(tfm: *mut crypto_skcipher) {
    static void cbc_paes_exit(struct crypto_skcipher *tfm)
    {
    struct s390_paes_ctx *ctx = crypto_skcipher_ctx(tfm);
    memzero_explicit(ctx, sizeof(*ctx));
    }
#[no_mangle]
unsafe extern "C" fn cbc_paes_do_one_request(engine: *mut crypto_engine, areq: *mut c_void) -> c_int {
    static int cbc_paes_do_one_request(struct crypto_engine *engine, void *areq)
    {
    struct skcipher_request *req = skcipher_request_cast(areq);
    struct s390_pcbc_req_ctx *req_ctx = skcipher_request_ctx(req);
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct s390_paes_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk *walk = &req_ctx.walk;
    let mut tested: bool = crypto_skcipher_tested(tfm);
    int rc;
// walk has already been prepared
    rc = cbc_paes_do_crypt(ctx, req_ctx, tested, true);
    if (rc == -EKEYEXPIRED) {
    return pkey_handle_expired();
    } else if (rc) {
    skcipher_walk_done(walk, rc);
    }
    memzero_explicit(&req_ctx.param, sizeof(req_ctx.param));
    pr_debug("request complete with rc=%d\n", rc);
    local_bh_disable();
    atomic_dec(&ctx.via_engine_ctr);
    crypto_finalize_skcipher_request(engine, req, rc);
    local_bh_enable();
    return rc;
    }
    static struct skcipher_engine_alg cbc_paes_alg = {
    .base = {
    .base.cra_name	      = "cbc(paes)",
    .base.cra_driver_name = "cbc-paes-s390",
    .base.cra_priority    = 402,	/* cbc-paes-s390 + 1 */
    .base.cra_blocksize   = AES_BLOCK_SIZE,
    .base.cra_ctxsize     = sizeof(struct s390_paes_ctx),
    .base.cra_module      = THIS_MODULE,
    .base.cra_list	      = LIST_HEAD_INIT(cbc_paes_alg.base.base.cra_list),
    .init		      = cbc_paes_init,
    .exit		      = cbc_paes_exit,
    .min_keysize	      = PAES_MIN_KEYSIZE,
    .max_keysize	      = PAES_MAX_KEYSIZE,
    .ivsize		      = AES_BLOCK_SIZE,
    .setkey		      = cbc_paes_setkey,
    .encrypt	      = cbc_paes_encrypt,
    .decrypt	      = cbc_paes_decrypt,
    },
    .op = {
    .do_one_request	      = cbc_paes_do_one_request,
    },
    };
//
// PAES CTR implementation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctr_param {
    pub key: [u8; PAES_256_PROTKEY_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_pctr_req_ctx {
    pub modifier: c_ulong,
    pub walk: skcipher_walk,
    pub param_init_done: bool,
    pub param: ctr_param,
}

    static int ctr_paes_setkey(struct crypto_skcipher *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct s390_paes_ctx *ctx = crypto_skcipher_ctx(tfm);
    let mut tested: bool = crypto_skcipher_tested(tfm);
    long fc;
    int rc;
// set raw key into context
    rc = paes_ctx_setkey(ctx, in_key, key_len);
    if (rc)
    goto out;
// convert raw key into protected key
    rc = paes_convert_key(ctx, tested);
    if (rc)
    goto out;
// Pick the correct function code based on the protected key type
    switch (ctx.pk.type) {
    case PKEY_KEYTYPE_AES_128:
    fc = CPACF_KMCTR_PAES_128;
    break;
    case PKEY_KEYTYPE_AES_192:
    fc = CPACF_KMCTR_PAES_192;
    break;
    case PKEY_KEYTYPE_AES_256:
    fc = CPACF_KMCTR_PAES_256;
    break;
    default:
    fc = 0;
    break;
    }
    ctx.fc = (fc && cpacf_test_func(&kmctr_functions, fc)) ? fc : 0;
    rc = fc ? 0 : -EINVAL;
    out:
    pr_debug("rc=%d\n", rc);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn __ctrblk_init(ctrptr: *mut u8, iv: *mut u8, nbytes: c_uint) -> c_uint {
    static inline unsigned int __ctrblk_init(u8 *ctrptr, u8 *iv, unsigned int nbytes)
    {
    unsigned int i, n;
// only use complete blocks, max. PAGE_SIZE
    memcpy(ctrptr, iv, AES_BLOCK_SIZE);
    n = (nbytes > PAGE_SIZE) ? PAGE_SIZE : nbytes & ~(AES_BLOCK_SIZE - 1);
    for (i = (n / AES_BLOCK_SIZE) - 1; i > 0; i--) {
    memcpy(ctrptr + AES_BLOCK_SIZE, ctrptr, AES_BLOCK_SIZE);
    crypto_inc(ctrptr + AES_BLOCK_SIZE, AES_BLOCK_SIZE);
    ctrptr += AES_BLOCK_SIZE;
    }
    return n;
    }
    static int ctr_paes_do_crypt(struct s390_paes_ctx *ctx,
    struct s390_pctr_req_ctx *req_ctx,
    bool tested, bool maysleep)
    {
    struct ctr_param *param = &req_ctx.param;
    struct skcipher_walk *walk = &req_ctx.walk;
    u8 buf[AES_BLOCK_SIZE], *ctrptr;
    unsigned int nbytes, n, k;
    int pk_state, locked, rc = 0;
    if (!req_ctx.param_init_done) {
// fetch and check protected key state
    spin_lock_bh(&ctx.pk_lock);
    pk_state = ctx.pk_state;
    switch (pk_state) {
    case PK_STATE_NO_KEY:
    rc = -ENOKEY;
    break;
    case PK_STATE_CONVERT_IN_PROGRESS:
    rc = -EKEYEXPIRED;
    break;
    case PK_STATE_VALID:
    memcpy(param.key, ctx.pk.protkey, sizeof(param.key));
    req_ctx.param_init_done = true;
    break;
    default:
    rc = pk_state < 0 ? pk_state : -EIO;
    break;
    }
    spin_unlock_bh(&ctx.pk_lock);
    }
    if (rc)
    goto out;
    locked = mutex_trylock(&ctrblk_lock);
//
// Note that in case of partial processing or failure the walk
// is NOT unmapped here. So a follow up task may reuse the walk
// or in case of unrecoverable failure needs to unmap it.
//
    while ((nbytes = walk.nbytes) >= AES_BLOCK_SIZE) {
    n = AES_BLOCK_SIZE;
    if (nbytes >= 2 * AES_BLOCK_SIZE && locked)
    n = __ctrblk_init(ctrblk, walk.iv, nbytes);
    ctrptr = (n > AES_BLOCK_SIZE) ? ctrblk : walk.iv;
    k = cpacf_kmctr(ctx.fc, param, walk.dst.virt.addr,
    walk.src.virt.addr, n, ctrptr);
    if (k) {
    if (ctrptr == ctrblk)
    memcpy(walk.iv, ctrptr + k - AES_BLOCK_SIZE,
    AES_BLOCK_SIZE);
    crypto_inc(walk.iv, AES_BLOCK_SIZE);
    rc = skcipher_walk_done(walk, nbytes - k);
    }
    if (k < n) {
    if (!maysleep) {
    if (locked)
    mutex_unlock(&ctrblk_lock);
    rc = -EKEYEXPIRED;
    goto out;
    }
    rc = paes_convert_key(ctx, tested);
    if (rc) {
    if (locked)
    mutex_unlock(&ctrblk_lock);
    goto out;
    }
    spin_lock_bh(&ctx.pk_lock);
    memcpy(param.key, ctx.pk.protkey, sizeof(param.key));
    spin_unlock_bh(&ctx.pk_lock);
    }
    }
    if (locked)
    mutex_unlock(&ctrblk_lock);
// final block may be < AES_BLOCK_SIZE, copy only nbytes
    if (nbytes) {
    memset(buf, 0, AES_BLOCK_SIZE);
    memcpy(buf, walk.src.virt.addr, nbytes);
    while (1) {
    if (cpacf_kmctr(ctx.fc, param, buf,
    buf, AES_BLOCK_SIZE,
    walk.iv) == AES_BLOCK_SIZE)
    break;
    if (!maysleep) {
    rc = -EKEYEXPIRED;
    goto out;
    }
    rc = paes_convert_key(ctx, tested);
    if (rc)
    goto out;
    spin_lock_bh(&ctx.pk_lock);
    memcpy(param.key, ctx.pk.protkey, sizeof(param.key));
    spin_unlock_bh(&ctx.pk_lock);
    }
    memcpy(walk.dst.virt.addr, buf, nbytes);
    crypto_inc(walk.iv, AES_BLOCK_SIZE);
    rc = skcipher_walk_done(walk, 0);
    }
    out:
    pr_debug("rc=%d\n", rc);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ctr_paes_crypt(req: *mut skcipher_request) -> c_int {
    static int ctr_paes_crypt(struct skcipher_request *req)
    {
    struct s390_pctr_req_ctx *req_ctx = skcipher_request_ctx(req);
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct s390_paes_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk *walk = &req_ctx.walk;
    let mut tested: bool = crypto_skcipher_tested(tfm);
    int rc;
//
// Attempt synchronous encryption first. If it fails, schedule the request
// asynchronously via the crypto engine. To preserve execution order,
// once a request is queued to the engine, further requests using the same
// tfm will also be routed through the engine.
//
    rc = skcipher_walk_virt(walk, req, false);
    if (rc)
    goto out;
    req_ctx.param_init_done = false;
// Try synchronous operation if no active engine usage
    if (!atomic_read(&ctx.via_engine_ctr)) {
    rc = ctr_paes_do_crypt(ctx, req_ctx, tested, false);
    if (rc == 0)
    goto out;
    }
//
// If sync operation failed or key expired or there are already
// requests enqueued via engine, fallback to async. Mark tfm as
// using engine to serialize requests.
//
    if (rc == 0 || rc == -EKEYEXPIRED) {
    atomic_inc(&ctx.via_engine_ctr);
    rc = crypto_transfer_skcipher_request_to_engine(paes_crypto_engine, req);
    if (rc != -EINPROGRESS)
    atomic_dec(&ctx.via_engine_ctr);
    }
    if (rc != -EINPROGRESS)
    skcipher_walk_done(walk, rc);
    out:
    if (rc != -EINPROGRESS)
    memzero_explicit(&req_ctx.param, sizeof(req_ctx.param));
    pr_debug("rc=%d\n", rc);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ctr_paes_init(tfm: *mut crypto_skcipher) -> c_int {
    static int ctr_paes_init(struct crypto_skcipher *tfm)
    {
    struct s390_paes_ctx *ctx = crypto_skcipher_ctx(tfm);
    memset(ctx, 0, sizeof(*ctx));
    spin_lock_init(&ctx.pk_lock);
    crypto_skcipher_set_reqsize(tfm, sizeof(struct s390_pctr_req_ctx));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ctr_paes_exit(tfm: *mut crypto_skcipher) {
    static void ctr_paes_exit(struct crypto_skcipher *tfm)
    {
    struct s390_paes_ctx *ctx = crypto_skcipher_ctx(tfm);
    memzero_explicit(ctx, sizeof(*ctx));
    }
#[no_mangle]
unsafe extern "C" fn ctr_paes_do_one_request(engine: *mut crypto_engine, areq: *mut c_void) -> c_int {
    static int ctr_paes_do_one_request(struct crypto_engine *engine, void *areq)
    {
    struct skcipher_request *req = skcipher_request_cast(areq);
    struct s390_pctr_req_ctx *req_ctx = skcipher_request_ctx(req);
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct s390_paes_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk *walk = &req_ctx.walk;
    let mut tested: bool = crypto_skcipher_tested(tfm);
    int rc;
// walk has already been prepared
    rc = ctr_paes_do_crypt(ctx, req_ctx, tested, true);
    if (rc == -EKEYEXPIRED) {
    return pkey_handle_expired();
    } else if (rc) {
    skcipher_walk_done(walk, rc);
    }
    memzero_explicit(&req_ctx.param, sizeof(req_ctx.param));
    pr_debug("request complete with rc=%d\n", rc);
    local_bh_disable();
    atomic_dec(&ctx.via_engine_ctr);
    crypto_finalize_skcipher_request(engine, req, rc);
    local_bh_enable();
    return rc;
    }
    static struct skcipher_engine_alg ctr_paes_alg = {
    .base = {
    .base.cra_name	      =	"ctr(paes)",
    .base.cra_driver_name =	"ctr-paes-s390",
    .base.cra_priority    =	402,	/* ecb-paes-s390 + 1 */
    .base.cra_blocksize   =	1,
    .base.cra_ctxsize     =	sizeof(struct s390_paes_ctx),
    .base.cra_module      =	THIS_MODULE,
    .base.cra_list	      =	LIST_HEAD_INIT(ctr_paes_alg.base.base.cra_list),
    .init		      =	ctr_paes_init,
    .exit		      =	ctr_paes_exit,
    .min_keysize	      =	PAES_MIN_KEYSIZE,
    .max_keysize	      =	PAES_MAX_KEYSIZE,
    .ivsize		      =	AES_BLOCK_SIZE,
    .setkey		      =	ctr_paes_setkey,
    .encrypt	      =	ctr_paes_crypt,
    .decrypt	      =	ctr_paes_crypt,
    .chunksize	      =	AES_BLOCK_SIZE,
    },
    .op = {
    .do_one_request	      = ctr_paes_do_one_request,
    },
    };
//
// PAES XTS implementation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xts_full_km_param {
    pub key: [u8; 64],
    pub tweak: [u8; 16],
    pub nap: [u8; 16],
    pub wkvp: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xts_km_param {
    pub key: [u8; PAES_256_PROTKEY_SIZE],
    pub init: [u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xts_pcc_param {
    pub key: [u8; PAES_256_PROTKEY_SIZE],
    pub tweak: [u8; 16],
    pub block: [u8; 16],
    pub bit: [u8; 16],
    pub xts: [u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_pxts_req_ctx {
    pub modifier: c_ulong,
    pub walk: skcipher_walk,
    pub param_init_done: bool,
    union {
    pub full_km_param: xts_full_km_param,
    pub km_param: xts_km_param,
    pub param: },
}

    static int xts_paes_setkey(struct crypto_skcipher *tfm, const u8 *in_key,
    unsigned int in_keylen)
    {
    struct s390_pxts_ctx *ctx = crypto_skcipher_ctx(tfm);
    let mut tested: bool = crypto_skcipher_tested(tfm);
    u8 ckey[2 * AES_MAX_KEY_SIZE];
    unsigned int ckey_len;
    long fc;
    int rc;
    if ((in_keylen == 32 || in_keylen == 64) &&
    xts_verify_key(tfm, in_key, in_keylen))
    return -EINVAL;
// set raw key into context
    rc = pxts_ctx_setkey(ctx, in_key, in_keylen);
    if (rc)
    goto out;
// convert raw key(s) into protected key(s)
    rc = pxts_convert_key(ctx, tested);
    if (rc)
    goto out;
//
// xts_verify_key verifies the key length is not odd and makes
// sure that the two keys are not the same. This can be done
// on the two protected keys as well - but not for full xts keys.
//
    if (ctx.pk[0].type == PKEY_KEYTYPE_AES_128 ||
    ctx.pk[0].type == PKEY_KEYTYPE_AES_256) {
    ckey_len = (ctx.pk[0].type == PKEY_KEYTYPE_AES_128) ?
    AES_KEYSIZE_128 : AES_KEYSIZE_256;
    memcpy(ckey, ctx.pk[0].protkey, ckey_len);
    memcpy(ckey + ckey_len, ctx.pk[1].protkey, ckey_len);
    rc = xts_verify_key(tfm, ckey, 2 * ckey_len);
    memzero_explicit(ckey, sizeof(ckey));
    if (rc)
    goto out;
    }
// Pick the correct function code based on the protected key type
    switch (ctx.pk[0].type) {
    case PKEY_KEYTYPE_AES_128:
    fc = CPACF_KM_PXTS_128;
    break;
    case PKEY_KEYTYPE_AES_256:
    fc = CPACF_KM_PXTS_256;
    break;
    case PKEY_KEYTYPE_AES_XTS_128:
    fc = CPACF_KM_PXTS_128_FULL;
    break;
    case PKEY_KEYTYPE_AES_XTS_256:
    fc = CPACF_KM_PXTS_256_FULL;
    break;
    default:
    fc = 0;
    break;
    }
    ctx.fc = (fc && cpacf_test_func(&km_functions, fc)) ? fc : 0;
    rc = fc ? 0 : -EINVAL;
    out:
    pr_debug("rc=%d\n", rc);
    return rc;
    }
    static int xts_paes_do_crypt_fullkey(struct s390_pxts_ctx *ctx,
    struct s390_pxts_req_ctx *req_ctx,
    bool tested, bool maysleep)
    {
    struct xts_full_km_param *param = &req_ctx.param.full_km_param;
    struct skcipher_walk *walk = &req_ctx.walk;
    unsigned int keylen, offset, nbytes, n, k;
    let mut rc: c_int = 0;
//
// The calling function xts_paes_do_crypt() ensures the
// protected key state is always PK_STATE_VALID when this
// function is invoked.
//
    keylen = (ctx.pk[0].type == PKEY_KEYTYPE_AES_XTS_128) ? 32 : 64;
    offset = (ctx.pk[0].type == PKEY_KEYTYPE_AES_XTS_128) ? 32 : 0;
    if (!req_ctx.param_init_done) {
    memset(param, 0, sizeof(*param));
    spin_lock_bh(&ctx.pk_lock);
    memcpy(param.key + offset, ctx.pk[0].protkey, keylen);
    memcpy(param.wkvp, ctx.pk[0].protkey + keylen, sizeof(param.wkvp));
    spin_unlock_bh(&ctx.pk_lock);
    memcpy(param.tweak, walk.iv, sizeof(param.tweak));
    param.nap[0] = 0x01; /* initial alpha power (1, little-endian) */
    req_ctx.param_init_done = true;
    }
//
// Note that in case of partial processing or failure the walk
// is NOT unmapped here. So a follow up task may reuse the walk
// or in case of unrecoverable failure needs to unmap it.
//
    while ((nbytes = walk.nbytes) != 0) {
// only use complete blocks
    n = nbytes & ~(AES_BLOCK_SIZE - 1);
    k = cpacf_km(ctx.fc | req_ctx.modifier, param.key + offset,
    walk.dst.virt.addr, walk.src.virt.addr, n);
    if (k)
    rc = skcipher_walk_done(walk, nbytes - k);
    if (k < n) {
    if (!maysleep) {
    rc = -EKEYEXPIRED;
    goto out;
    }
    rc = pxts_convert_key(ctx, tested);
    if (rc)
    goto out;
    spin_lock_bh(&ctx.pk_lock);
    memcpy(param.key + offset, ctx.pk[0].protkey, keylen);
    memcpy(param.wkvp, ctx.pk[0].protkey + keylen, sizeof(param.wkvp));
    spin_unlock_bh(&ctx.pk_lock);
    }
    }
    out:
    pr_debug("rc=%d\n", rc);
    return rc;
    }
    static inline int __xts_2keys_prep_param(struct s390_pxts_ctx *ctx,
    struct xts_km_param *param,
    struct skcipher_walk *walk,
    unsigned int keylen,
    unsigned int offset,
    bool tested, bool maysleep)
    {
    struct xts_pcc_param pcc_param;
    let mut cc: c_ulong = 1;
    let mut rc: c_int = 0;
    while (cc) {
    memset(&pcc_param, 0, sizeof(pcc_param));
    memcpy(pcc_param.tweak, walk.iv, sizeof(pcc_param.tweak));
    spin_lock_bh(&ctx.pk_lock);
    memcpy(pcc_param.key + offset, ctx.pk[1].protkey, keylen);
    memcpy(param.key + offset, ctx.pk[0].protkey, keylen);
    spin_unlock_bh(&ctx.pk_lock);
    cc = cpacf_pcc(ctx.fc, pcc_param.key + offset);
    if (cc) {
    if (!maysleep) {
    rc = -EKEYEXPIRED;
    break;
    }
    rc = pxts_convert_key(ctx, tested);
    if (rc)
    break;
    continue;
    }
    memcpy(param.init, pcc_param.xts, 16);
    }
    memzero_explicit(pcc_param.key, sizeof(pcc_param.key));
    return rc;
    }
    static int xts_paes_do_crypt_2keys(struct s390_pxts_ctx *ctx,
    struct s390_pxts_req_ctx *req_ctx,
    bool tested, bool maysleep)
    {
    struct xts_km_param *param = &req_ctx.param.km_param;
    struct skcipher_walk *walk = &req_ctx.walk;
    unsigned int keylen, offset, nbytes, n, k;
    let mut rc: c_int = 0;
//
// The calling function xts_paes_do_crypt() ensures the
// protected key state is always PK_STATE_VALID when this
// function is invoked.
//
    keylen = (ctx.pk[0].type == PKEY_KEYTYPE_AES_128) ? 48 : 64;
    offset = (ctx.pk[0].type == PKEY_KEYTYPE_AES_128) ? 16 : 0;
    if (!req_ctx.param_init_done) {
    rc = __xts_2keys_prep_param(ctx, param, walk,
    keylen, offset, tested, maysleep);
    if (rc)
    goto out;
    req_ctx.param_init_done = true;
    }
//
// Note that in case of partial processing or failure the walk
// is NOT unmapped here. So a follow up task may reuse the walk
// or in case of unrecoverable failure needs to unmap it.
//
    while ((nbytes = walk.nbytes) != 0) {
// only use complete blocks
    n = nbytes & ~(AES_BLOCK_SIZE - 1);
    k = cpacf_km(ctx.fc | req_ctx.modifier, param.key + offset,
    walk.dst.virt.addr, walk.src.virt.addr, n);
    if (k)
    rc = skcipher_walk_done(walk, nbytes - k);
    if (k < n) {
    if (!maysleep) {
    rc = -EKEYEXPIRED;
    goto out;
    }
    rc = pxts_convert_key(ctx, tested);
    if (rc)
    goto out;
    spin_lock_bh(&ctx.pk_lock);
    memcpy(param.key + offset, ctx.pk[0].protkey, keylen);
    spin_unlock_bh(&ctx.pk_lock);
    }
    }
    out:
    pr_debug("rc=%d\n", rc);
    return rc;
    }
    static int xts_paes_do_crypt(struct s390_pxts_ctx *ctx,
    struct s390_pxts_req_ctx *req_ctx,
    bool tested, bool maysleep)
    {
    int pk_state, rc = 0;
// fetch and check protected key state
    spin_lock_bh(&ctx.pk_lock);
    pk_state = ctx.pk_state;
    switch (pk_state) {
    case PK_STATE_NO_KEY:
    rc = -ENOKEY;
    break;
    case PK_STATE_CONVERT_IN_PROGRESS:
    rc = -EKEYEXPIRED;
    break;
    case PK_STATE_VALID:
    break;
    default:
    rc = pk_state < 0 ? pk_state : -EIO;
    break;
    }
    spin_unlock_bh(&ctx.pk_lock);
    if (rc)
    goto out;
// Call the 'real' crypt function based on the xts prot key type.
    switch (ctx.fc) {
    case CPACF_KM_PXTS_128:
    case CPACF_KM_PXTS_256:
    rc = xts_paes_do_crypt_2keys(ctx, req_ctx, tested, maysleep);
    break;
    case CPACF_KM_PXTS_128_FULL:
    case CPACF_KM_PXTS_256_FULL:
    rc = xts_paes_do_crypt_fullkey(ctx, req_ctx, tested, maysleep);
    break;
    default:
    rc = -EINVAL;
    }
    out:
    pr_debug("rc=%d\n", rc);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn xts_paes_crypt(req: *mut skcipher_request, modifier: c_ulong) -> c_int {
    static inline int xts_paes_crypt(struct skcipher_request *req, unsigned long modifier)
    {
    struct s390_pxts_req_ctx *req_ctx = skcipher_request_ctx(req);
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct s390_pxts_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk *walk = &req_ctx.walk;
    let mut tested: bool = crypto_skcipher_tested(tfm);
    int rc;
//
// Attempt synchronous encryption first. If it fails, schedule the request
// asynchronously via the crypto engine. To preserve execution order,
// once a request is queued to the engine, further requests using the same
// tfm will also be routed through the engine.
//
    rc = skcipher_walk_virt(walk, req, false);
    if (rc)
    goto out;
    req_ctx.modifier = modifier;
    req_ctx.param_init_done = false;
// Try synchronous operation if no active engine usage
    if (!atomic_read(&ctx.via_engine_ctr)) {
    rc = xts_paes_do_crypt(ctx, req_ctx, tested, false);
    if (rc == 0)
    goto out;
    }
//
// If sync operation failed or key expired or there are already
// requests enqueued via engine, fallback to async. Mark tfm as
// using engine to serialize requests.
//
    if (rc == 0 || rc == -EKEYEXPIRED) {
    atomic_inc(&ctx.via_engine_ctr);
    rc = crypto_transfer_skcipher_request_to_engine(paes_crypto_engine, req);
    if (rc != -EINPROGRESS)
    atomic_dec(&ctx.via_engine_ctr);
    }
    if (rc != -EINPROGRESS)
    skcipher_walk_done(walk, rc);
    out:
    if (rc != -EINPROGRESS)
    memzero_explicit(&req_ctx.param, sizeof(req_ctx.param));
    pr_debug("rc=%d\n", rc);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn xts_paes_encrypt(req: *mut skcipher_request) -> c_int {
    static int xts_paes_encrypt(struct skcipher_request *req)
    {
    return xts_paes_crypt(req, 0);
    }
#[no_mangle]
unsafe extern "C" fn xts_paes_decrypt(req: *mut skcipher_request) -> c_int {
    static int xts_paes_decrypt(struct skcipher_request *req)
    {
    return xts_paes_crypt(req, CPACF_DECRYPT);
    }
#[no_mangle]
unsafe extern "C" fn xts_paes_init(tfm: *mut crypto_skcipher) -> c_int {
    static int xts_paes_init(struct crypto_skcipher *tfm)
    {
    struct s390_pxts_ctx *ctx = crypto_skcipher_ctx(tfm);
    memset(ctx, 0, sizeof(*ctx));
    spin_lock_init(&ctx.pk_lock);
    crypto_skcipher_set_reqsize(tfm, sizeof(struct s390_pxts_req_ctx));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xts_paes_exit(tfm: *mut crypto_skcipher) {
    static void xts_paes_exit(struct crypto_skcipher *tfm)
    {
    struct s390_pxts_ctx *ctx = crypto_skcipher_ctx(tfm);
    memzero_explicit(ctx, sizeof(*ctx));
    }
#[no_mangle]
unsafe extern "C" fn xts_paes_do_one_request(engine: *mut crypto_engine, areq: *mut c_void) -> c_int {
    static int xts_paes_do_one_request(struct crypto_engine *engine, void *areq)
    {
    struct skcipher_request *req = skcipher_request_cast(areq);
    struct s390_pxts_req_ctx *req_ctx = skcipher_request_ctx(req);
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct s390_pxts_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk *walk = &req_ctx.walk;
    let mut tested: bool = crypto_skcipher_tested(tfm);
    int rc;
// walk has already been prepared
    rc = xts_paes_do_crypt(ctx, req_ctx, tested, true);
    if (rc == -EKEYEXPIRED) {
    return pkey_handle_expired();
    } else if (rc) {
    skcipher_walk_done(walk, rc);
    }
    memzero_explicit(&req_ctx.param, sizeof(req_ctx.param));
    pr_debug("request complete with rc=%d\n", rc);
    local_bh_disable();
    atomic_dec(&ctx.via_engine_ctr);
    crypto_finalize_skcipher_request(engine, req, rc);
    local_bh_enable();
    return rc;
    }
    static struct skcipher_engine_alg xts_paes_alg = {
    .base = {
    .base.cra_name	      =	"xts(paes)",
    .base.cra_driver_name =	"xts-paes-s390",
    .base.cra_priority    =	402,	/* ecb-paes-s390 + 1 */
    .base.cra_blocksize   =	AES_BLOCK_SIZE,
    .base.cra_ctxsize     =	sizeof(struct s390_pxts_ctx),
    .base.cra_module      =	THIS_MODULE,
    .base.cra_list	      =	LIST_HEAD_INIT(xts_paes_alg.base.base.cra_list),
    .init		      =	xts_paes_init,
    .exit		      =	xts_paes_exit,
    .min_keysize	      =	2 * PAES_MIN_KEYSIZE,
    .max_keysize	      =	2 * PAES_MAX_KEYSIZE,
    .ivsize		      =	AES_BLOCK_SIZE,
    .setkey		      =	xts_paes_setkey,
    .encrypt	      =	xts_paes_encrypt,
    .decrypt	      =	xts_paes_decrypt,
    },
    .op = {
    .do_one_request	      = xts_paes_do_one_request,
    },
    };
//
// alg register, unregister, module init, exit
//
    static struct miscdevice paes_dev = {
    .name	= "paes",
    .minor	= MISC_DYNAMIC_MINOR,
    };
#[no_mangle]
pub unsafe extern "C" fn __crypto_unregister_skcipher(alg: *mut skcipher_engine_alg) {
    static inline void __crypto_unregister_skcipher(struct skcipher_engine_alg *alg)
    {
    if (!list_empty(&alg.base.base.cra_list))
    crypto_engine_unregister_skcipher(alg);
    }
#[no_mangle]
unsafe extern "C" fn paes_s390_fini() {
    static void paes_s390_fini(void)
    {
    if (paes_crypto_engine) {
    crypto_engine_stop(paes_crypto_engine);
    crypto_engine_exit(paes_crypto_engine);
    }
    __crypto_unregister_skcipher(&ctr_paes_alg);
    __crypto_unregister_skcipher(&xts_paes_alg);
    __crypto_unregister_skcipher(&cbc_paes_alg);
    __crypto_unregister_skcipher(&ecb_paes_alg);
    if (ctrblk)
    free_page((unsigned long)ctrblk);
    misc_deregister(&paes_dev);
    }
#[no_mangle]
unsafe extern "C" fn paes_s390_init() -> int __init {
    static int __init paes_s390_init(void)
    {
    int rc;
// register a simple paes pseudo misc device
    rc = misc_register(&paes_dev);
    if (rc)
    return rc;
// with this pseudo devie alloc and start a crypto engine
    paes_crypto_engine =
    crypto_engine_alloc_init_and_set(paes_dev.this_device,
    true, false, MAX_QLEN);
    if (!paes_crypto_engine) {
    rc = -ENOMEM;
    goto out_err;
    }
    rc = crypto_engine_start(paes_crypto_engine);
    if (rc) {
    crypto_engine_exit(paes_crypto_engine);
    paes_crypto_engine = core::ptr::null_mut();
    goto out_err;
    }
// Query available functions for KM, KMC and KMCTR
    cpacf_query(CPACF_KM, &km_functions);
    cpacf_query(CPACF_KMC, &kmc_functions);
    cpacf_query(CPACF_KMCTR, &kmctr_functions);
    if (cpacf_test_func(&km_functions, CPACF_KM_PAES_128) ||
    cpacf_test_func(&km_functions, CPACF_KM_PAES_192) ||
    cpacf_test_func(&km_functions, CPACF_KM_PAES_256)) {
    rc = crypto_engine_register_skcipher(&ecb_paes_alg);
    if (rc)
    goto out_err;
    pr_debug("%s registered\n", ecb_paes_alg.base.base.cra_driver_name);
    }
    if (cpacf_test_func(&kmc_functions, CPACF_KMC_PAES_128) ||
    cpacf_test_func(&kmc_functions, CPACF_KMC_PAES_192) ||
    cpacf_test_func(&kmc_functions, CPACF_KMC_PAES_256)) {
    rc = crypto_engine_register_skcipher(&cbc_paes_alg);
    if (rc)
    goto out_err;
    pr_debug("%s registered\n", cbc_paes_alg.base.base.cra_driver_name);
    }
    if (cpacf_test_func(&km_functions, CPACF_KM_PXTS_128) ||
    cpacf_test_func(&km_functions, CPACF_KM_PXTS_256)) {
    rc = crypto_engine_register_skcipher(&xts_paes_alg);
    if (rc)
    goto out_err;
    pr_debug("%s registered\n", xts_paes_alg.base.base.cra_driver_name);
    }
    if (cpacf_test_func(&kmctr_functions, CPACF_KMCTR_PAES_128) ||
    cpacf_test_func(&kmctr_functions, CPACF_KMCTR_PAES_192) ||
    cpacf_test_func(&kmctr_functions, CPACF_KMCTR_PAES_256)) {
    ctrblk = (u8 *)__get_free_page(GFP_KERNEL);
    if (!ctrblk) {
    rc = -ENOMEM;
    goto out_err;
    }
    rc = crypto_engine_register_skcipher(&ctr_paes_alg);
    if (rc)
    goto out_err;
    pr_debug("%s registered\n", ctr_paes_alg.base.base.cra_driver_name);
    }
    return 0;
    out_err:
    paes_s390_fini();
    return rc;
    }
    module_init(paes_s390_init);
    module_exit(paes_s390_fini);
    MODULE_ALIAS_CRYPTO("ecb(paes)");
    MODULE_ALIAS_CRYPTO("cbc(paes)");
    MODULE_ALIAS_CRYPTO("ctr(paes)");
    MODULE_ALIAS_CRYPTO("xts(paes)");
    MODULE_DESCRIPTION("Rijndael (AES) Cipher Algorithm with protected keys");
    MODULE_LICENSE("GPL");
