//! Automatically rewritten from C to Rust
//! Source: crypto/chacha20poly1305.c
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
// ChaCha20-Poly1305 AEAD, RFC7539
//
// Copyright (C) 2015 Martin Willi
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chachapoly_instance_ctx {
    pub chacha: crypto_skcipher_spawn,
    pub saltlen: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chachapoly_ctx {
    pub chacha: *mut crypto_skcipher,
// key bytes we use for the ChaCha20 IV
    pub saltlen: c_uint,
    pub __counted_by(saltlen): u8 salt[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chacha_req {
    pub iv: [u8; CHACHA_IV_SIZE],
    pub src: [scatterlist; 1],
    pub /: *mut *mut skcipher_request req; / must be last member,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chachapoly_req_ctx {
    pub src: [scatterlist; 2],
    pub dst: [scatterlist; 2],
// the key we generate for Poly1305 using Chacha20
    pub key: [u8; POLY1305_KEY_SIZE],
// calculated Poly1305 tag
    pub tag: [u8; POLY1305_DIGEST_SIZE],
// length of data to en/decrypt, without ICV
    pub cryptlen: c_uint,
// Actual AD, excluding IV
    pub assoclen: c_uint,
// request flags, with MAY_SLEEP cleared if needed
    pub flags: u32,
    union {
    pub chacha: chacha_req,
    pub u: },
}

    static inline void async_done_continue(struct aead_request *req, int err,
    int (*cont)(struct aead_request *))
    {
    if (!err) {
    struct chachapoly_req_ctx *rctx = aead_request_ctx(req);
    rctx.flags &= ~CRYPTO_TFM_REQ_MAY_SLEEP;
    err = cont(req);
    }
    if (err != -EINPROGRESS && err != -EBUSY)
    aead_request_complete(req, err);
    }
#[no_mangle]
unsafe extern "C" fn chacha_iv(iv: *mut u8, req: *mut aead_request, icb: u32) {
    static void chacha_iv(u8 *iv, struct aead_request *req, u32 icb)
    {
    struct chachapoly_ctx *ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    let mut leicb: __le32 = cpu_to_le32(icb);
    memcpy(iv, &leicb, sizeof(leicb));
    memcpy(iv + sizeof(leicb), ctx.salt, ctx.saltlen);
    memcpy(iv + sizeof(leicb) + ctx.saltlen, req.iv,
    CHACHA_IV_SIZE - sizeof(leicb) - ctx.saltlen);
    }
#[no_mangle]
unsafe extern "C" fn poly_verify_tag(req: *mut aead_request) -> c_int {
    static int poly_verify_tag(struct aead_request *req)
    {
    struct chachapoly_req_ctx *rctx = aead_request_ctx(req);
    u8 tag[sizeof(rctx.tag)];
    scatterwalk_map_and_copy(tag, req.src,
    req.assoclen + rctx.cryptlen,
    sizeof(tag), 0);
    if (crypto_memneq(tag, rctx.tag, sizeof(tag)))
    return -EBADMSG;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chacha_decrypt_done(data: *mut c_void, err: c_int) {
    static void chacha_decrypt_done(void *data, int err)
    {
    async_done_continue(data, err, poly_verify_tag);
    }
#[no_mangle]
unsafe extern "C" fn chacha_decrypt(req: *mut aead_request) -> c_int {
    static int chacha_decrypt(struct aead_request *req)
    {
    struct chachapoly_ctx *ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    struct chachapoly_req_ctx *rctx = aead_request_ctx(req);
    struct chacha_req *creq = &rctx.u.chacha;
    struct scatterlist *src, *dst;
    int err;
    if (rctx.cryptlen == 0)
    goto skip;
    chacha_iv(creq.iv, req, 1);
    src = scatterwalk_ffwd(rctx.src, req.src, req.assoclen);
    dst = src;
    if (req.src != req.dst)
    dst = scatterwalk_ffwd(rctx.dst, req.dst, req.assoclen);
    skcipher_request_set_callback(&creq.req, rctx.flags,
    chacha_decrypt_done, req);
    skcipher_request_set_tfm(&creq.req, ctx.chacha);
    skcipher_request_set_crypt(&creq.req, src, dst,
    rctx.cryptlen, creq.iv);
    err = crypto_skcipher_decrypt(&creq.req);
    if (err)
    return err;
    skip:
    return poly_verify_tag(req);
    }
#[no_mangle]
unsafe extern "C" fn poly_hash(req: *mut aead_request) -> c_int {
    static int poly_hash(struct aead_request *req)
    {
    struct chachapoly_req_ctx *rctx = aead_request_ctx(req);
    const void *zp = page_address(ZERO_PAGE(0));
    struct scatterlist *sg = req.src;
    struct poly1305_desc_ctx desc;
    struct scatter_walk walk;
    struct {
    union {
    struct {
    __le64 assoclen;
    __le64 cryptlen;
    };
    u8 u8[16];
    };
    } tail;
    unsigned int padlen;
    unsigned int total;
    if (sg != req.dst)
    memcpy_sglist(req.dst, sg, req.assoclen);
    if (rctx.cryptlen == req.cryptlen) /* encrypting */
    sg = req.dst;
    poly1305_init(&desc, rctx.key);
    scatterwalk_start(&walk, sg);
    total = rctx.assoclen;
    while (total) {
    let mut n: c_uint = scatterwalk_next(&walk, total);
    poly1305_update(&desc, walk.addr, n);
    scatterwalk_done_src(&walk, n);
    total -= n;
    }
    padlen = -rctx.assoclen % POLY1305_BLOCK_SIZE;
    poly1305_update(&desc, zp, padlen);
    scatterwalk_skip(&walk, req.assoclen - rctx.assoclen);
    total = rctx.cryptlen;
    while (total) {
    let mut n: c_uint = scatterwalk_next(&walk, total);
    poly1305_update(&desc, walk.addr, n);
    scatterwalk_done_src(&walk, n);
    total -= n;
    }
    padlen = -rctx.cryptlen % POLY1305_BLOCK_SIZE;
    poly1305_update(&desc, zp, padlen);
    tail.assoclen = cpu_to_le64(rctx.assoclen);
    tail.cryptlen = cpu_to_le64(rctx.cryptlen);
    poly1305_update(&desc, tail.u8, sizeof(tail));
    memzero_explicit(&tail, sizeof(tail));
    poly1305_final(&desc, rctx.tag);
    if (rctx.cryptlen != req.cryptlen)
    return chacha_decrypt(req);
    memcpy_to_scatterwalk(&walk, rctx.tag, sizeof(rctx.tag));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn poly_genkey_done(data: *mut c_void, err: c_int) {
    static void poly_genkey_done(void *data, int err)
    {
    async_done_continue(data, err, poly_hash);
    }
#[no_mangle]
unsafe extern "C" fn poly_genkey(req: *mut aead_request) -> c_int {
    static int poly_genkey(struct aead_request *req)
    {
    struct crypto_aead *tfm = crypto_aead_reqtfm(req);
    struct chachapoly_ctx *ctx = crypto_aead_ctx(tfm);
    struct chachapoly_req_ctx *rctx = aead_request_ctx(req);
    struct chacha_req *creq = &rctx.u.chacha;
    int err;
    rctx.assoclen = req.assoclen;
    if (crypto_aead_ivsize(tfm) == 8) {
    if (rctx.assoclen < 8)
    return -EINVAL;
    rctx.assoclen -= 8;
    }
    memset(rctx.key, 0, sizeof(rctx.key));
    sg_init_one(creq.src, rctx.key, sizeof(rctx.key));
    chacha_iv(creq.iv, req, 0);
    skcipher_request_set_callback(&creq.req, rctx.flags,
    poly_genkey_done, req);
    skcipher_request_set_tfm(&creq.req, ctx.chacha);
    skcipher_request_set_crypt(&creq.req, creq.src, creq.src,
    POLY1305_KEY_SIZE, creq.iv);
    err = crypto_skcipher_decrypt(&creq.req);
    if (err)
    return err;
    return poly_hash(req);
    }
#[no_mangle]
unsafe extern "C" fn chacha_encrypt_done(data: *mut c_void, err: c_int) {
    static void chacha_encrypt_done(void *data, int err)
    {
    async_done_continue(data, err, poly_genkey);
    }
#[no_mangle]
unsafe extern "C" fn chacha_encrypt(req: *mut aead_request) -> c_int {
    static int chacha_encrypt(struct aead_request *req)
    {
    struct chachapoly_ctx *ctx = crypto_aead_ctx(crypto_aead_reqtfm(req));
    struct chachapoly_req_ctx *rctx = aead_request_ctx(req);
    struct chacha_req *creq = &rctx.u.chacha;
    struct scatterlist *src, *dst;
    int err;
    if (req.cryptlen == 0)
    goto skip;
    chacha_iv(creq.iv, req, 1);
    src = scatterwalk_ffwd(rctx.src, req.src, req.assoclen);
    dst = src;
    if (req.src != req.dst)
    dst = scatterwalk_ffwd(rctx.dst, req.dst, req.assoclen);
    skcipher_request_set_callback(&creq.req, rctx.flags,
    chacha_encrypt_done, req);
    skcipher_request_set_tfm(&creq.req, ctx.chacha);
    skcipher_request_set_crypt(&creq.req, src, dst,
    req.cryptlen, creq.iv);
    err = crypto_skcipher_encrypt(&creq.req);
    if (err)
    return err;
    skip:
    return poly_genkey(req);
    }
#[no_mangle]
unsafe extern "C" fn chachapoly_encrypt(req: *mut aead_request) -> c_int {
    static int chachapoly_encrypt(struct aead_request *req)
    {
    struct chachapoly_req_ctx *rctx = aead_request_ctx(req);
    rctx.cryptlen = req.cryptlen;
    rctx.flags = aead_request_flags(req);
// encrypt call chain:
// - chacha_encrypt/done()
// - poly_genkey/done()
// - poly_hash()
//
    return chacha_encrypt(req);
    }
#[no_mangle]
unsafe extern "C" fn chachapoly_decrypt(req: *mut aead_request) -> c_int {
    static int chachapoly_decrypt(struct aead_request *req)
    {
    struct chachapoly_req_ctx *rctx = aead_request_ctx(req);
    rctx.cryptlen = req.cryptlen - POLY1305_DIGEST_SIZE;
    rctx.flags = aead_request_flags(req);
// decrypt call chain:
// - poly_genkey/done()
// - poly_hash()
// - chacha_decrypt/done()
// - poly_verify_tag()
//
    return poly_genkey(req);
    }
    static int chachapoly_setkey(struct crypto_aead *aead, const u8 *key,
    unsigned int keylen)
    {
    struct chachapoly_ctx *ctx = crypto_aead_ctx(aead);
    if (keylen != ctx.saltlen + CHACHA_KEY_SIZE)
    return -EINVAL;
    keylen -= ctx.saltlen;
    memcpy(ctx.salt, key + keylen, ctx.saltlen);
    crypto_skcipher_clear_flags(ctx.chacha, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags(ctx.chacha, crypto_aead_get_flags(aead) &
    CRYPTO_TFM_REQ_MASK);
    return crypto_skcipher_setkey(ctx.chacha, key, keylen);
    }
    static int chachapoly_setauthsize(struct crypto_aead *tfm,
    unsigned int authsize)
    {
    if (authsize != POLY1305_DIGEST_SIZE)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chachapoly_init(tfm: *mut crypto_aead) -> c_int {
    static int chachapoly_init(struct crypto_aead *tfm)
    {
    struct aead_instance *inst = aead_alg_instance(tfm);
    struct chachapoly_instance_ctx *ictx = aead_instance_ctx(inst);
    struct chachapoly_ctx *ctx = crypto_aead_ctx(tfm);
    struct crypto_skcipher *chacha;
    unsigned long align;
    chacha = crypto_spawn_skcipher(&ictx.chacha);
    if (IS_ERR(chacha))
    return PTR_ERR(chacha);
    ctx.chacha = chacha;
    ctx.saltlen = ictx.saltlen;
    align = crypto_aead_alignmask(tfm);
    align &= ~(crypto_tfm_ctx_alignment() - 1);
    crypto_aead_set_reqsize(
    tfm,
    align + offsetof(struct chachapoly_req_ctx, u) +
    offsetof(struct chacha_req, req) +
    sizeof(struct skcipher_request) +
    crypto_skcipher_reqsize(chacha));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chachapoly_exit(tfm: *mut crypto_aead) {
    static void chachapoly_exit(struct crypto_aead *tfm)
    {
    struct chachapoly_ctx *ctx = crypto_aead_ctx(tfm);
    crypto_free_skcipher(ctx.chacha);
    }
#[no_mangle]
unsafe extern "C" fn chachapoly_free(inst: *mut aead_instance) {
    static void chachapoly_free(struct aead_instance *inst)
    {
    struct chachapoly_instance_ctx *ctx = aead_instance_ctx(inst);
    crypto_drop_skcipher(&ctx.chacha);
    kfree(inst);
    }
    static int chachapoly_create(struct crypto_template *tmpl, struct rtattr **tb,
    const char *name, unsigned int ivsize)
    {
    u32 mask;
    struct aead_instance *inst;
    struct chachapoly_instance_ctx *ctx;
    struct skcipher_alg_common *chacha;
    const char *poly_name;
    int err;
    if (ivsize > CHACHAPOLY_IV_SIZE)
    return -EINVAL;
    err = crypto_check_attr_type(tb, CRYPTO_ALG_TYPE_AEAD, &mask);
    if (err)
    return err;
    inst = kzalloc(sizeof(*inst) + sizeof(*ctx), GFP_KERNEL);
    if (!inst)
    return -ENOMEM;
    ctx = aead_instance_ctx(inst);
    ctx.saltlen = CHACHAPOLY_IV_SIZE - ivsize;
    err = crypto_grab_skcipher(&ctx.chacha, aead_crypto_instance(inst),
    crypto_attr_alg_name(tb[1]), 0, mask);
    if (err)
    goto err_free_inst;
    chacha = crypto_spawn_skcipher_alg_common(&ctx.chacha);
    poly_name = crypto_attr_alg_name(tb[2]);
    if (IS_ERR(poly_name)) {
    err = PTR_ERR(poly_name);
    goto err_free_inst;
    }
    err = -EINVAL;
    if (strcmp(poly_name, "poly1305") &&
    strcmp(poly_name, "poly1305-generic"))
    goto err_free_inst;
// Need 16-byte IV size, including Initial Block Counter value
    if (chacha.ivsize != CHACHA_IV_SIZE)
    goto err_free_inst;
// Not a stream cipher?
    if (chacha.base.cra_blocksize != 1)
    goto err_free_inst;
    err = -ENAMETOOLONG;
    if (snprintf(inst.alg.base.cra_name, CRYPTO_MAX_ALG_NAME,
    "%s(%s,poly1305)", name,
    chacha.base.cra_name) >= CRYPTO_MAX_ALG_NAME)
    goto err_free_inst;
    if (snprintf(inst.alg.base.cra_driver_name, CRYPTO_MAX_ALG_NAME,
    "%s(%s,poly1305-generic)", name,
    chacha.base.cra_driver_name) >= CRYPTO_MAX_ALG_NAME)
    goto err_free_inst;
    inst.alg.base.cra_priority = chacha.base.cra_priority;
    inst.alg.base.cra_blocksize = 1;
    inst.alg.base.cra_alignmask = chacha.base.cra_alignmask;
    inst.alg.base.cra_ctxsize = sizeof(struct chachapoly_ctx) +
    ctx.saltlen;
    inst.alg.ivsize = ivsize;
    inst.alg.chunksize = chacha.chunksize;
    inst.alg.maxauthsize = POLY1305_DIGEST_SIZE;
    inst.alg.init = chachapoly_init;
    inst.alg.exit = chachapoly_exit;
    inst.alg.encrypt = chachapoly_encrypt;
    inst.alg.decrypt = chachapoly_decrypt;
    inst.alg.setkey = chachapoly_setkey;
    inst.alg.setauthsize = chachapoly_setauthsize;
    inst.free = chachapoly_free;
    err = aead_register_instance(tmpl, inst);
    if (err) {
    err_free_inst:
    chachapoly_free(inst);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rfc7539_create(tmpl: *mut crypto_template, tb: *mut rtattr) -> c_int {
    static int rfc7539_create(struct crypto_template *tmpl, struct rtattr **tb)
    {
    return chachapoly_create(tmpl, tb, "rfc7539", 12);
    }
#[no_mangle]
unsafe extern "C" fn rfc7539esp_create(tmpl: *mut crypto_template, tb: *mut rtattr) -> c_int {
    static int rfc7539esp_create(struct crypto_template *tmpl, struct rtattr **tb)
    {
    return chachapoly_create(tmpl, tb, "rfc7539esp", 8);
    }
    static struct crypto_template rfc7539_tmpls[] = {
    {
    .name = "rfc7539",
    .create = rfc7539_create,
    .module = THIS_MODULE,
    }, {
    .name = "rfc7539esp",
    .create = rfc7539esp_create,
    .module = THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn chacha20poly1305_module_init() -> int __init {
    static int __init chacha20poly1305_module_init(void)
    {
    return crypto_register_templates(rfc7539_tmpls,
    ARRAY_SIZE(rfc7539_tmpls));
    }
#[no_mangle]
unsafe extern "C" fn chacha20poly1305_module_exit() -> void __exit {
    static void __exit chacha20poly1305_module_exit(void)
    {
    crypto_unregister_templates(rfc7539_tmpls,
    ARRAY_SIZE(rfc7539_tmpls));
    }
    module_init(chacha20poly1305_module_init);
    module_exit(chacha20poly1305_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Martin Willi <martin@strongswan.org>");
    MODULE_DESCRIPTION("ChaCha20-Poly1305 AEAD");
    MODULE_ALIAS_CRYPTO("rfc7539");
    MODULE_ALIAS_CRYPTO("rfc7539esp");
