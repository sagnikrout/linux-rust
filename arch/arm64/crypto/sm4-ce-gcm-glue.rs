//! Automatically rewritten from C to Rust
//! Source: arch/arm64/crypto/sm4-ce-gcm-glue.c
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
// SM4-GCM AEAD Algorithm using ARMv8 Crypto Extensions
// as specified in rfc8998
// https://datatracker.ietf.org/doc/html/rfc8998
//
// Copyright (C) 2022 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>
//

    asmlinkage void sm4_ce_pmull_ghash_setup(const u32 *rkey_enc, u8 *ghash_table);
    asmlinkage void pmull_ghash_update(const u8 *ghash_table, u8 *ghash,
    const u8 *src, unsigned int nblocks);
    asmlinkage void sm4_ce_pmull_gcm_enc(const u32 *rkey_enc, u8 *dst,
    const u8 *src, u8 *iv,
    unsigned int nbytes, u8 *ghash,
    const u8 *ghash_table, const u8 *lengths);
    asmlinkage void sm4_ce_pmull_gcm_dec(const u32 *rkey_enc, u8 *dst,
    const u8 *src, u8 *iv,
    unsigned int nbytes, u8 *ghash,
    const u8 *ghash_table, const u8 *lengths);
pub const GHASH_BLOCK_SIZE: c_int = 16;
pub const GCM_IV_SIZE: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sm4_gcm_ctx {
    pub key: sm4_ctx,
    pub 4]: *mut *mut u8 ghash_table[16,
}

    static int gcm_setkey(struct crypto_aead *tfm, const u8 *key,
    unsigned int key_len)
    {
    struct sm4_gcm_ctx *ctx = crypto_aead_ctx(tfm);
    if (key_len != SM4_KEY_SIZE)
    return -EINVAL;
    scoped_ksimd() {
    sm4_ce_expand_key(key, ctx.key.rkey_enc, ctx.key.rkey_dec,
    crypto_sm4_fk, crypto_sm4_ck);
    sm4_ce_pmull_ghash_setup(ctx.key.rkey_enc, ctx.ghash_table);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gcm_setauthsize(tfm: *mut crypto_aead, authsize: c_uint) -> c_int {
    static int gcm_setauthsize(struct crypto_aead *tfm, unsigned int authsize)
    {
    switch (authsize) {
    case 4:
    case 8:
    case 12 ... 16:
    return 0;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn gcm_calculate_auth_mac(req: *mut aead_request, ghash[]: u8) {
    static void gcm_calculate_auth_mac(struct aead_request *req, u8 ghash[])
    {
    struct crypto_aead *aead = crypto_aead_reqtfm(req);
    struct sm4_gcm_ctx *ctx = crypto_aead_ctx(aead);
    u8 __aligned(8) buffer[GHASH_BLOCK_SIZE];
    let mut assoclen: u32 = req.assoclen;
    struct scatter_walk walk;
    let mut buflen: c_uint = 0;
    scatterwalk_start(&walk, req.src);
    do {
    unsigned int n, orig_n;
    const u8 *p;
    orig_n = scatterwalk_next(&walk, assoclen);
    p = walk.addr;
    n = orig_n;
    if (n + buflen < GHASH_BLOCK_SIZE) {
    memcpy(&buffer[buflen], p, n);
    buflen += n;
    } else {
    unsigned int nblocks;
    if (buflen) {
    let mut l: c_uint = GHASH_BLOCK_SIZE - buflen;
    memcpy(&buffer[buflen], p, l);
    p += l;
    n -= l;
    pmull_ghash_update(ctx.ghash_table, ghash,
    buffer, 1);
    }
    nblocks = n / GHASH_BLOCK_SIZE;
    if (nblocks) {
    pmull_ghash_update(ctx.ghash_table, ghash,
    p, nblocks);
    p += nblocks * GHASH_BLOCK_SIZE;
    }
    buflen = n % GHASH_BLOCK_SIZE;
    if (buflen)
    memcpy(&buffer[0], p, buflen);
    }
    scatterwalk_done_src(&walk, orig_n);
    assoclen -= orig_n;
    } while (assoclen);
// padding with '0'
    if (buflen) {
    memset(&buffer[buflen], 0, GHASH_BLOCK_SIZE - buflen);
    pmull_ghash_update(ctx.ghash_table, ghash, buffer, 1);
    }
    }
    static int gcm_crypt(struct aead_request *req, struct skcipher_walk *walk,
    u8 ghash[], int err,
    void (*sm4_ce_pmull_gcm_crypt)(const u32 *rkey_enc,
    u8 *dst, const u8 *src, u8 *iv,
    unsigned int nbytes, u8 *ghash,
    const u8 *ghash_table, const u8 *lengths))
    {
    struct crypto_aead *aead = crypto_aead_reqtfm(req);
    struct sm4_gcm_ctx *ctx = crypto_aead_ctx(aead);
    u8 __aligned(8) iv[SM4_BLOCK_SIZE];
    be128 __aligned(8) lengths;
    memset(ghash, 0, SM4_BLOCK_SIZE);
    lengths.a = cpu_to_be64(req.assoclen * 8);
    lengths.b = cpu_to_be64(walk.total * 8);
    memcpy(iv, req.iv, GCM_IV_SIZE);
    put_unaligned_be32(2, iv + GCM_IV_SIZE);
    scoped_ksimd() {
    if (req.assoclen)
    gcm_calculate_auth_mac(req, ghash);
    do {
    let mut tail: c_uint = walk.nbytes % SM4_BLOCK_SIZE;
    const u8 *src = walk.src.virt.addr;
    u8 *dst = walk.dst.virt.addr;
    const u8 *l = core::ptr::null_mut();
    if (walk.nbytes == walk.total) {
    l = (const u8 *)&lengths;
    tail = 0;
    }
    sm4_ce_pmull_gcm_crypt(ctx.key.rkey_enc, dst, src, iv,
    walk.nbytes - tail, ghash,
    ctx.ghash_table, l);
    err = skcipher_walk_done(walk, tail);
    } while (walk.nbytes);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn gcm_encrypt(req: *mut aead_request) -> c_int {
    static int gcm_encrypt(struct aead_request *req)
    {
    struct crypto_aead *aead = crypto_aead_reqtfm(req);
    u8 __aligned(8) ghash[SM4_BLOCK_SIZE];
    struct skcipher_walk walk;
    int err;
    err = skcipher_walk_aead_encrypt(&walk, req, false);
    err = gcm_crypt(req, &walk, ghash, err, sm4_ce_pmull_gcm_enc);
    if (err)
    return err;
// copy authtag to end of dst
    scatterwalk_map_and_copy(ghash, req.dst, req.assoclen + req.cryptlen,
    crypto_aead_authsize(aead), 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gcm_decrypt(req: *mut aead_request) -> c_int {
    static int gcm_decrypt(struct aead_request *req)
    {
    struct crypto_aead *aead = crypto_aead_reqtfm(req);
    let mut authsize: c_uint = crypto_aead_authsize(aead);
    u8 __aligned(8) ghash[SM4_BLOCK_SIZE];
    u8 authtag[SM4_BLOCK_SIZE];
    struct skcipher_walk walk;
    int err;
    err = skcipher_walk_aead_decrypt(&walk, req, false);
    err = gcm_crypt(req, &walk, ghash, err, sm4_ce_pmull_gcm_dec);
    if (err)
    return err;
// compare calculated auth tag with the stored one
    scatterwalk_map_and_copy(authtag, req.src,
    req.assoclen + req.cryptlen - authsize,
    authsize, 0);
    if (crypto_memneq(authtag, ghash, authsize))
    return -EBADMSG;
    return 0;
    }
    static struct aead_alg sm4_gcm_alg = {
    .base = {
    .cra_name		= "gcm(sm4)",
    .cra_driver_name	= "gcm-sm4-ce",
    .cra_priority		= 400,
    .cra_blocksize		= 1,
    .cra_ctxsize		= sizeof(struct sm4_gcm_ctx),
    .cra_module		= THIS_MODULE,
    },
    .ivsize		= GCM_IV_SIZE,
    .chunksize	= SM4_BLOCK_SIZE,
    .maxauthsize	= SM4_BLOCK_SIZE,
    .setkey		= gcm_setkey,
    .setauthsize	= gcm_setauthsize,
    .encrypt	= gcm_encrypt,
    .decrypt	= gcm_decrypt,
    };
#[no_mangle]
unsafe extern "C" fn sm4_ce_gcm_init() -> int __init {
    static int __init sm4_ce_gcm_init(void)
    {
    if (!cpu_have_named_feature(PMULL))
    return -ENODEV;
    return crypto_register_aead(&sm4_gcm_alg);
    }
#[no_mangle]
unsafe extern "C" fn sm4_ce_gcm_exit() -> void __exit {
    static void __exit sm4_ce_gcm_exit(void)
    {
    crypto_unregister_aead(&sm4_gcm_alg);
    }
    static const struct cpu_feature __maybe_unused sm4_ce_gcm_cpu_feature[] = {
    { cpu_feature(PMULL) },
    {}
    };
    MODULE_DEVICE_TABLE(cpu, sm4_ce_gcm_cpu_feature);
    module_cpu_feature_match(SM4, sm4_ce_gcm_init);
    module_exit(sm4_ce_gcm_exit);
    MODULE_DESCRIPTION("Synchronous SM4 in GCM mode using ARMv8 Crypto Extensions");
    MODULE_ALIAS_CRYPTO("gcm(sm4)");
    MODULE_AUTHOR("Tianjia Zhang <tianjia.zhang@linux.alibaba.com>");
    MODULE_LICENSE("GPL v2");
