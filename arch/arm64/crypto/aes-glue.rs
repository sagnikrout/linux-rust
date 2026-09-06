//! Automatically rewritten from C to Rust
//! Source: arch/arm64/crypto/aes-glue.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/arch/arm64/crypto/aes-glue.c - wrapper code for ARMv8 AES
//
// Copyright (C) 2013 - 2017 Linaro Ltd <ard.biesheuvel@linaro.org>
//

pub const PRIO: c_int = 300;

    MODULE_DESCRIPTION("AES-ECB/CBC/CTR/XTS/XCTR using ARMv8 Crypto Extensions");

pub const PRIO: c_int = 200;

    MODULE_DESCRIPTION("AES-ECB/CBC/CTR/XTS/XCTR using ARMv8 NEON");

    MODULE_ALIAS_CRYPTO("ecb(aes)");
    MODULE_ALIAS_CRYPTO("cbc(aes)");
    MODULE_ALIAS_CRYPTO("ctr(aes)");
    MODULE_ALIAS_CRYPTO("xts(aes)");
    MODULE_ALIAS_CRYPTO("xctr(aes)");

    MODULE_ALIAS_CRYPTO("cts(cbc(aes))");
    MODULE_ALIAS_CRYPTO("essiv(cbc(aes),sha256)");
    MODULE_AUTHOR("Ard Biesheuvel <ard.biesheuvel@linaro.org>");
    MODULE_IMPORT_NS("CRYPTO_INTERNAL");
    MODULE_LICENSE("GPL v2");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_aes_xts_ctx {
    pub key1: crypto_aes_ctx,
    pub key2: crypto_aes_ctx __aligned(8),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_aes_essiv_cbc_ctx {
    pub key1: crypto_aes_ctx,
    pub key2: crypto_aes_ctx __aligned(8),
}

    static int skcipher_aes_setkey(struct crypto_skcipher *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct crypto_aes_ctx *ctx = crypto_skcipher_ctx(tfm);
    return aes_expandkey(ctx, in_key, key_len);
    }
    static int __maybe_unused xts_set_key(struct crypto_skcipher *tfm,
    const u8 *in_key, unsigned int key_len)
    {
    struct crypto_aes_xts_ctx *ctx = crypto_skcipher_ctx(tfm);
    int ret;
    ret = xts_verify_key(tfm, in_key, key_len);
    if (ret)
    return ret;
    ret = aes_expandkey(&ctx.key1, in_key, key_len / 2);
    if (!ret)
    ret = aes_expandkey(&ctx.key2, &in_key[key_len / 2],
    key_len / 2);
    return ret;
    }
    static int __maybe_unused essiv_cbc_set_key(struct crypto_skcipher *tfm,
    const u8 *in_key,
    unsigned int key_len)
    {
    struct crypto_aes_essiv_cbc_ctx *ctx = crypto_skcipher_ctx(tfm);
    u8 digest[SHA256_DIGEST_SIZE];
    int ret;
    ret = aes_expandkey(&ctx.key1, in_key, key_len);
    if (ret)
    return ret;
    sha256(in_key, key_len, digest);
    return aes_expandkey(&ctx.key2, digest, sizeof(digest));
    }
#[no_mangle]
unsafe extern "C" fn ecb_encrypt(req: *mut skcipher_request) -> int __maybe_unused {
    static int __maybe_unused ecb_encrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct crypto_aes_ctx *ctx = crypto_skcipher_ctx(tfm);
    int err, rounds = 6 + ctx.key_length / 4;
    struct skcipher_walk walk;
    unsigned int blocks;
    err = skcipher_walk_virt(&walk, req, false);
    while ((blocks = (walk.nbytes / AES_BLOCK_SIZE))) {
    scoped_ksimd()
    aes_ecb_encrypt(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key_enc, rounds, blocks);
    err = skcipher_walk_done(&walk, walk.nbytes % AES_BLOCK_SIZE);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ecb_decrypt(req: *mut skcipher_request) -> int __maybe_unused {
    static int __maybe_unused ecb_decrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct crypto_aes_ctx *ctx = crypto_skcipher_ctx(tfm);
    int err, rounds = 6 + ctx.key_length / 4;
    struct skcipher_walk walk;
    unsigned int blocks;
    err = skcipher_walk_virt(&walk, req, false);
    while ((blocks = (walk.nbytes / AES_BLOCK_SIZE))) {
    scoped_ksimd()
    aes_ecb_decrypt(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key_dec, rounds, blocks);
    err = skcipher_walk_done(&walk, walk.nbytes % AES_BLOCK_SIZE);
    }
    return err;
    }
    static int cbc_encrypt_walk(struct skcipher_request *req,
    struct skcipher_walk *walk)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct crypto_aes_ctx *ctx = crypto_skcipher_ctx(tfm);
    let mut err: c_int = 0, rounds = 6 + ctx.key_length / 4;
    unsigned int blocks;
    while ((blocks = (walk.nbytes / AES_BLOCK_SIZE))) {
    scoped_ksimd()
    aes_cbc_encrypt(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key_enc, rounds, blocks, walk.iv);
    err = skcipher_walk_done(walk, walk.nbytes % AES_BLOCK_SIZE);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cbc_encrypt(req: *mut skcipher_request) -> int __maybe_unused {
    static int __maybe_unused cbc_encrypt(struct skcipher_request *req)
    {
    struct skcipher_walk walk;
    int err;
    err = skcipher_walk_virt(&walk, req, false);
    if (err)
    return err;
    return cbc_encrypt_walk(req, &walk);
    }
    static int cbc_decrypt_walk(struct skcipher_request *req,
    struct skcipher_walk *walk)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct crypto_aes_ctx *ctx = crypto_skcipher_ctx(tfm);
    let mut err: c_int = 0, rounds = 6 + ctx.key_length / 4;
    unsigned int blocks;
    while ((blocks = (walk.nbytes / AES_BLOCK_SIZE))) {
    scoped_ksimd()
    aes_cbc_decrypt(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key_dec, rounds, blocks, walk.iv);
    err = skcipher_walk_done(walk, walk.nbytes % AES_BLOCK_SIZE);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cbc_decrypt(req: *mut skcipher_request) -> int __maybe_unused {
    static int __maybe_unused cbc_decrypt(struct skcipher_request *req)
    {
    struct skcipher_walk walk;
    int err;
    err = skcipher_walk_virt(&walk, req, false);
    if (err)
    return err;
    return cbc_decrypt_walk(req, &walk);
    }
#[no_mangle]
unsafe extern "C" fn cts_cbc_encrypt(req: *mut skcipher_request) -> c_int {
    static int cts_cbc_encrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct crypto_aes_ctx *ctx = crypto_skcipher_ctx(tfm);
    int err, rounds = 6 + ctx.key_length / 4;
    let mut cbc_blocks: c_int = DIV_ROUND_UP(req.cryptlen, AES_BLOCK_SIZE) - 2;
    struct scatterlist *src = req.src, *dst = req.dst;
    struct scatterlist sg_src[2], sg_dst[2];
    struct skcipher_request subreq;
    struct skcipher_walk walk;
    skcipher_request_set_tfm(&subreq, tfm);
    skcipher_request_set_callback(&subreq, skcipher_request_flags(req),
    core::ptr::null_mut(), core::ptr::null_mut());
    if (req.cryptlen <= AES_BLOCK_SIZE) {
    if (req.cryptlen < AES_BLOCK_SIZE)
    return -EINVAL;
    cbc_blocks = 1;
    }
    if (cbc_blocks > 0) {
    skcipher_request_set_crypt(&subreq, req.src, req.dst,
    cbc_blocks * AES_BLOCK_SIZE,
    req.iv);
    err = skcipher_walk_virt(&walk, &subreq, false) ?:
    cbc_encrypt_walk(&subreq, &walk);
    if (err)
    return err;
    if (req.cryptlen == AES_BLOCK_SIZE)
    return 0;
    dst = src = scatterwalk_ffwd(sg_src, req.src, subreq.cryptlen);
    if (req.dst != req.src)
    dst = scatterwalk_ffwd(sg_dst, req.dst,
    subreq.cryptlen);
    }
// handle ciphertext stealing
    skcipher_request_set_crypt(&subreq, src, dst,
    req.cryptlen - cbc_blocks * AES_BLOCK_SIZE,
    req.iv);
    err = skcipher_walk_virt(&walk, &subreq, false);
    if (err)
    return err;
    scoped_ksimd()
    aes_cbc_cts_encrypt(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key_enc, rounds, walk.nbytes, walk.iv);
    return skcipher_walk_done(&walk, 0);
    }
#[no_mangle]
unsafe extern "C" fn cts_cbc_decrypt(req: *mut skcipher_request) -> c_int {
    static int cts_cbc_decrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct crypto_aes_ctx *ctx = crypto_skcipher_ctx(tfm);
    int err, rounds = 6 + ctx.key_length / 4;
    let mut cbc_blocks: c_int = DIV_ROUND_UP(req.cryptlen, AES_BLOCK_SIZE) - 2;
    struct scatterlist *src = req.src, *dst = req.dst;
    struct scatterlist sg_src[2], sg_dst[2];
    struct skcipher_request subreq;
    struct skcipher_walk walk;
    skcipher_request_set_tfm(&subreq, tfm);
    skcipher_request_set_callback(&subreq, skcipher_request_flags(req),
    core::ptr::null_mut(), core::ptr::null_mut());
    if (req.cryptlen <= AES_BLOCK_SIZE) {
    if (req.cryptlen < AES_BLOCK_SIZE)
    return -EINVAL;
    cbc_blocks = 1;
    }
    if (cbc_blocks > 0) {
    skcipher_request_set_crypt(&subreq, req.src, req.dst,
    cbc_blocks * AES_BLOCK_SIZE,
    req.iv);
    err = skcipher_walk_virt(&walk, &subreq, false) ?:
    cbc_decrypt_walk(&subreq, &walk);
    if (err)
    return err;
    if (req.cryptlen == AES_BLOCK_SIZE)
    return 0;
    dst = src = scatterwalk_ffwd(sg_src, req.src, subreq.cryptlen);
    if (req.dst != req.src)
    dst = scatterwalk_ffwd(sg_dst, req.dst,
    subreq.cryptlen);
    }
// handle ciphertext stealing
    skcipher_request_set_crypt(&subreq, src, dst,
    req.cryptlen - cbc_blocks * AES_BLOCK_SIZE,
    req.iv);
    err = skcipher_walk_virt(&walk, &subreq, false);
    if (err)
    return err;
    scoped_ksimd()
    aes_cbc_cts_decrypt(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key_dec, rounds, walk.nbytes, walk.iv);
    return skcipher_walk_done(&walk, 0);
    }
#[no_mangle]
unsafe extern "C" fn essiv_cbc_encrypt(req: *mut skcipher_request) -> int __maybe_unused {
    static int __maybe_unused essiv_cbc_encrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct crypto_aes_essiv_cbc_ctx *ctx = crypto_skcipher_ctx(tfm);
    int err, rounds = 6 + ctx.key1.key_length / 4;
    struct skcipher_walk walk;
    unsigned int blocks;
    err = skcipher_walk_virt(&walk, req, false);
    blocks = walk.nbytes / AES_BLOCK_SIZE;
    if (blocks) {
    scoped_ksimd()
    aes_essiv_cbc_encrypt(walk.dst.virt.addr,
    walk.src.virt.addr,
    ctx.key1.key_enc, rounds, blocks,
    req.iv, ctx.key2.key_enc);
    err = skcipher_walk_done(&walk, walk.nbytes % AES_BLOCK_SIZE);
    }
    return err ?: cbc_encrypt_walk(req, &walk);
    }
#[no_mangle]
unsafe extern "C" fn essiv_cbc_decrypt(req: *mut skcipher_request) -> int __maybe_unused {
    static int __maybe_unused essiv_cbc_decrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct crypto_aes_essiv_cbc_ctx *ctx = crypto_skcipher_ctx(tfm);
    int err, rounds = 6 + ctx.key1.key_length / 4;
    struct skcipher_walk walk;
    unsigned int blocks;
    err = skcipher_walk_virt(&walk, req, false);
    blocks = walk.nbytes / AES_BLOCK_SIZE;
    if (blocks) {
    scoped_ksimd()
    aes_essiv_cbc_decrypt(walk.dst.virt.addr,
    walk.src.virt.addr,
    ctx.key1.key_dec, rounds, blocks,
    req.iv, ctx.key2.key_enc);
    err = skcipher_walk_done(&walk, walk.nbytes % AES_BLOCK_SIZE);
    }
    return err ?: cbc_decrypt_walk(req, &walk);
    }
#[no_mangle]
unsafe extern "C" fn xctr_encrypt(req: *mut skcipher_request) -> int __maybe_unused {
    static int __maybe_unused xctr_encrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct crypto_aes_ctx *ctx = crypto_skcipher_ctx(tfm);
    int err, rounds = 6 + ctx.key_length / 4;
    struct skcipher_walk walk;
    let mut byte_ctr: c_uint = 0;
    err = skcipher_walk_virt(&walk, req, false);
    while (walk.nbytes > 0) {
    const u8 *src = walk.src.virt.addr;
    let mut nbytes: c_uint = walk.nbytes;
    u8 *dst = walk.dst.virt.addr;
    u8 buf[AES_BLOCK_SIZE];
//
// If given less than 16 bytes, we must copy the partial block
// into a temporary buffer of 16 bytes to avoid out of bounds
// reads and writes.  Furthermore, this code is somewhat unusual
// in that it expects the end of the data to be at the end of
// the temporary buffer, rather than the start of the data at
// the start of the temporary buffer.
//
    if (unlikely(nbytes < AES_BLOCK_SIZE))
    src = dst = memcpy(buf + sizeof(buf) - nbytes,
    src, nbytes);
#[no_mangle]
pub unsafe extern "C" fn if(walk.total: nbytes <) -> else {
    else if (nbytes < walk.total)
    nbytes &= ~(AES_BLOCK_SIZE - 1);
    scoped_ksimd()
    aes_xctr_encrypt(dst, src, ctx.key_enc, rounds, nbytes,
    walk.iv, byte_ctr);
    if (unlikely(nbytes < AES_BLOCK_SIZE))
    memcpy(walk.dst.virt.addr,
    buf + sizeof(buf) - nbytes, nbytes);
    byte_ctr += nbytes;
    err = skcipher_walk_done(&walk, walk.nbytes - nbytes);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ctr_encrypt(req: *mut skcipher_request) -> int __maybe_unused {
    static int __maybe_unused ctr_encrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct crypto_aes_ctx *ctx = crypto_skcipher_ctx(tfm);
    int err, rounds = 6 + ctx.key_length / 4;
    struct skcipher_walk walk;
    err = skcipher_walk_virt(&walk, req, false);
    while (walk.nbytes > 0) {
    const u8 *src = walk.src.virt.addr;
    let mut nbytes: c_uint = walk.nbytes;
    u8 *dst = walk.dst.virt.addr;
    u8 buf[AES_BLOCK_SIZE];
//
// If given less than 16 bytes, we must copy the partial block
// into a temporary buffer of 16 bytes to avoid out of bounds
// reads and writes.  Furthermore, this code is somewhat unusual
// in that it expects the end of the data to be at the end of
// the temporary buffer, rather than the start of the data at
// the start of the temporary buffer.
//
    if (unlikely(nbytes < AES_BLOCK_SIZE))
    src = dst = memcpy(buf + sizeof(buf) - nbytes,
    src, nbytes);
#[no_mangle]
pub unsafe extern "C" fn if(walk.total: nbytes <) -> else {
    else if (nbytes < walk.total)
    nbytes &= ~(AES_BLOCK_SIZE - 1);
    scoped_ksimd()
    aes_ctr_encrypt(dst, src, ctx.key_enc, rounds, nbytes,
    walk.iv);
    if (unlikely(nbytes < AES_BLOCK_SIZE))
    memcpy(walk.dst.virt.addr,
    buf + sizeof(buf) - nbytes, nbytes);
    err = skcipher_walk_done(&walk, walk.nbytes - nbytes);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn xts_encrypt(req: *mut skcipher_request) -> int __maybe_unused {
    static int __maybe_unused xts_encrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct crypto_aes_xts_ctx *ctx = crypto_skcipher_ctx(tfm);
    int err, first, rounds = 6 + ctx.key1.key_length / 4;
    let mut tail: c_int = req.cryptlen % AES_BLOCK_SIZE;
    struct scatterlist sg_src[2], sg_dst[2];
    struct skcipher_request subreq;
    struct scatterlist *src, *dst;
    struct skcipher_walk walk;
    if (req.cryptlen < AES_BLOCK_SIZE)
    return -EINVAL;
    err = skcipher_walk_virt(&walk, req, false);
    if (unlikely(tail > 0 && walk.nbytes < walk.total)) {
    int xts_blocks = DIV_ROUND_UP(req.cryptlen,
    AES_BLOCK_SIZE) - 2;
    skcipher_walk_abort(&walk);
    skcipher_request_set_tfm(&subreq, tfm);
    skcipher_request_set_callback(&subreq,
    skcipher_request_flags(req),
    core::ptr::null_mut(), core::ptr::null_mut());
    skcipher_request_set_crypt(&subreq, req.src, req.dst,
    xts_blocks * AES_BLOCK_SIZE,
    req.iv);
    req = &subreq;
    err = skcipher_walk_virt(&walk, req, false);
    } else {
    tail = 0;
    }
    scoped_ksimd() {
    for (first = 1; walk.nbytes >= AES_BLOCK_SIZE; first = 0) {
    let mut nbytes: c_int = walk.nbytes;
    if (walk.nbytes < walk.total)
    nbytes &= ~(AES_BLOCK_SIZE - 1);
    aes_xts_encrypt(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key1.key_enc, rounds, nbytes,
    ctx.key2.key_enc, walk.iv, first);
    err = skcipher_walk_done(&walk, walk.nbytes - nbytes);
    }
    if (err || likely(!tail))
    return err;
    dst = src = scatterwalk_ffwd(sg_src, req.src, req.cryptlen);
    if (req.dst != req.src)
    dst = scatterwalk_ffwd(sg_dst, req.dst, req.cryptlen);
    skcipher_request_set_crypt(req, src, dst, AES_BLOCK_SIZE + tail,
    req.iv);
    err = skcipher_walk_virt(&walk, &subreq, false);
    if (err)
    return err;
    aes_xts_encrypt(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key1.key_enc, rounds, walk.nbytes,
    ctx.key2.key_enc, walk.iv, first);
    }
    return skcipher_walk_done(&walk, 0);
    }
#[no_mangle]
unsafe extern "C" fn xts_decrypt(req: *mut skcipher_request) -> int __maybe_unused {
    static int __maybe_unused xts_decrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct crypto_aes_xts_ctx *ctx = crypto_skcipher_ctx(tfm);
    int err, first, rounds = 6 + ctx.key1.key_length / 4;
    let mut tail: c_int = req.cryptlen % AES_BLOCK_SIZE;
    struct scatterlist sg_src[2], sg_dst[2];
    struct skcipher_request subreq;
    struct scatterlist *src, *dst;
    struct skcipher_walk walk;
    if (req.cryptlen < AES_BLOCK_SIZE)
    return -EINVAL;
    err = skcipher_walk_virt(&walk, req, false);
    if (unlikely(tail > 0 && walk.nbytes < walk.total)) {
    int xts_blocks = DIV_ROUND_UP(req.cryptlen,
    AES_BLOCK_SIZE) - 2;
    skcipher_walk_abort(&walk);
    skcipher_request_set_tfm(&subreq, tfm);
    skcipher_request_set_callback(&subreq,
    skcipher_request_flags(req),
    core::ptr::null_mut(), core::ptr::null_mut());
    skcipher_request_set_crypt(&subreq, req.src, req.dst,
    xts_blocks * AES_BLOCK_SIZE,
    req.iv);
    req = &subreq;
    err = skcipher_walk_virt(&walk, req, false);
    } else {
    tail = 0;
    }
    scoped_ksimd() {
    for (first = 1; walk.nbytes >= AES_BLOCK_SIZE; first = 0) {
    let mut nbytes: c_int = walk.nbytes;
    if (walk.nbytes < walk.total)
    nbytes &= ~(AES_BLOCK_SIZE - 1);
    aes_xts_decrypt(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key1.key_dec, rounds, nbytes,
    ctx.key2.key_enc, walk.iv, first);
    err = skcipher_walk_done(&walk, walk.nbytes - nbytes);
    }
    if (err || likely(!tail))
    return err;
    dst = src = scatterwalk_ffwd(sg_src, req.src, req.cryptlen);
    if (req.dst != req.src)
    dst = scatterwalk_ffwd(sg_dst, req.dst, req.cryptlen);
    skcipher_request_set_crypt(req, src, dst, AES_BLOCK_SIZE + tail,
    req.iv);
    err = skcipher_walk_virt(&walk, &subreq, false);
    if (err)
    return err;
    aes_xts_decrypt(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key1.key_dec, rounds, walk.nbytes,
    ctx.key2.key_enc, walk.iv, first);
    }
    return skcipher_walk_done(&walk, 0);
    }
    static struct skcipher_alg aes_algs[] = { {

    .base = {
    .cra_name		= "ecb(aes)",
    .cra_driver_name	= "ecb-aes-" MODE,
    .cra_priority		= PRIO,
    .cra_blocksize		= AES_BLOCK_SIZE,
    .cra_ctxsize		= sizeof(struct crypto_aes_ctx),
    .cra_module		= THIS_MODULE,
    },
    .min_keysize	= AES_MIN_KEY_SIZE,
    .max_keysize	= AES_MAX_KEY_SIZE,
    .setkey		= skcipher_aes_setkey,
    .encrypt	= ecb_encrypt,
    .decrypt	= ecb_decrypt,
    }, {
    .base = {
    .cra_name		= "cbc(aes)",
    .cra_driver_name	= "cbc-aes-" MODE,
    .cra_priority		= PRIO,
    .cra_blocksize		= AES_BLOCK_SIZE,
    .cra_ctxsize		= sizeof(struct crypto_aes_ctx),
    .cra_module		= THIS_MODULE,
    },
    .min_keysize	= AES_MIN_KEY_SIZE,
    .max_keysize	= AES_MAX_KEY_SIZE,
    .ivsize		= AES_BLOCK_SIZE,
    .setkey		= skcipher_aes_setkey,
    .encrypt	= cbc_encrypt,
    .decrypt	= cbc_decrypt,
    }, {
    .base = {
    .cra_name		= "ctr(aes)",
    .cra_driver_name	= "ctr-aes-" MODE,
    .cra_priority		= PRIO,
    .cra_blocksize		= 1,
    .cra_ctxsize		= sizeof(struct crypto_aes_ctx),
    .cra_module		= THIS_MODULE,
    },
    .min_keysize	= AES_MIN_KEY_SIZE,
    .max_keysize	= AES_MAX_KEY_SIZE,
    .ivsize		= AES_BLOCK_SIZE,
    .chunksize	= AES_BLOCK_SIZE,
    .setkey		= skcipher_aes_setkey,
    .encrypt	= ctr_encrypt,
    .decrypt	= ctr_encrypt,
    }, {
    .base = {
    .cra_name		= "xctr(aes)",
    .cra_driver_name	= "xctr-aes-" MODE,
    .cra_priority		= PRIO,
    .cra_blocksize		= 1,
    .cra_ctxsize		= sizeof(struct crypto_aes_ctx),
    .cra_module		= THIS_MODULE,
    },
    .min_keysize	= AES_MIN_KEY_SIZE,
    .max_keysize	= AES_MAX_KEY_SIZE,
    .ivsize		= AES_BLOCK_SIZE,
    .chunksize	= AES_BLOCK_SIZE,
    .setkey		= skcipher_aes_setkey,
    .encrypt	= xctr_encrypt,
    .decrypt	= xctr_encrypt,
    }, {
    .base = {
    .cra_name		= "xts(aes)",
    .cra_driver_name	= "xts-aes-" MODE,
    .cra_priority		= PRIO,
    .cra_blocksize		= AES_BLOCK_SIZE,
    .cra_ctxsize		= sizeof(struct crypto_aes_xts_ctx),
    .cra_module		= THIS_MODULE,
    },
    .min_keysize	= 2 * AES_MIN_KEY_SIZE,
    .max_keysize	= 2 * AES_MAX_KEY_SIZE,
    .ivsize		= AES_BLOCK_SIZE,
    .walksize	= 2 * AES_BLOCK_SIZE,
    .setkey		= xts_set_key,
    .encrypt	= xts_encrypt,
    .decrypt	= xts_decrypt,
    }, {

    .base = {
    .cra_name		= "cts(cbc(aes))",
    .cra_driver_name	= "cts-cbc-aes-" MODE,
    .cra_priority		= PRIO,
    .cra_blocksize		= AES_BLOCK_SIZE,
    .cra_ctxsize		= sizeof(struct crypto_aes_ctx),
    .cra_module		= THIS_MODULE,
    },
    .min_keysize	= AES_MIN_KEY_SIZE,
    .max_keysize	= AES_MAX_KEY_SIZE,
    .ivsize		= AES_BLOCK_SIZE,
    .walksize	= 2 * AES_BLOCK_SIZE,
    .setkey		= skcipher_aes_setkey,
    .encrypt	= cts_cbc_encrypt,
    .decrypt	= cts_cbc_decrypt,
    }, {
    .base = {
    .cra_name		= "essiv(cbc(aes),sha256)",
    .cra_driver_name	= "essiv-cbc-aes-sha256-" MODE,
    .cra_priority		= PRIO + 1,
    .cra_blocksize		= AES_BLOCK_SIZE,
    .cra_ctxsize		= sizeof(struct crypto_aes_essiv_cbc_ctx),
    .cra_module		= THIS_MODULE,
    },
    .min_keysize	= AES_MIN_KEY_SIZE,
    .max_keysize	= AES_MAX_KEY_SIZE,
    .ivsize		= AES_BLOCK_SIZE,
    .setkey		= essiv_cbc_set_key,
    .encrypt	= essiv_cbc_encrypt,
    .decrypt	= essiv_cbc_decrypt,
    } };
#[no_mangle]
unsafe extern "C" fn aes_exit() {
    static void aes_exit(void)
    {
    crypto_unregister_skciphers(aes_algs, ARRAY_SIZE(aes_algs));
    }
#[no_mangle]
unsafe extern "C" fn aes_init() -> int __init {
    static int __init aes_init(void)
    {
    return crypto_register_skciphers(aes_algs, ARRAY_SIZE(aes_algs));
    }

    module_cpu_feature_match(AES, aes_init);

    module_init(aes_init);

    module_exit(aes_exit);
