//! Automatically rewritten from C to Rust
//! Source: arch/arm64/crypto/aes-neonbs-glue.c
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
// Bit sliced AES using NEON instructions
//
// Copyright (C) 2016 - 2017 Linaro Ltd <ard.biesheuvel@linaro.org>
//

    MODULE_AUTHOR("Ard Biesheuvel <ard.biesheuvel@linaro.org>");
    MODULE_DESCRIPTION("Bit sliced AES using NEON instructions");
    MODULE_IMPORT_NS("CRYPTO_INTERNAL");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS_CRYPTO("ecb(aes)");
    MODULE_ALIAS_CRYPTO("cbc(aes)");
    MODULE_ALIAS_CRYPTO("ctr(aes)");
    MODULE_ALIAS_CRYPTO("xts(aes)");
    asmlinkage void aesbs_convert_key(u8 out[], u32 const rk[], int rounds);
    asmlinkage void aesbs_ecb_encrypt(u8 out[], u8 const in[], u8 const rk[],
    int rounds, int blocks);
    asmlinkage void aesbs_ecb_decrypt(u8 out[], u8 const in[], u8 const rk[],
    int rounds, int blocks);
    asmlinkage void aesbs_cbc_decrypt(u8 out[], u8 const in[], u8 const rk[],
    int rounds, int blocks, u8 iv[]);
    asmlinkage void aesbs_ctr_encrypt(u8 out[], u8 const in[], u8 const rk[],
    int rounds, int blocks, u8 iv[]);
    asmlinkage void aesbs_xts_encrypt(u8 out[], u8 const in[], u8 const rk[],
    int rounds, int blocks, u8 iv[]);
    asmlinkage void aesbs_xts_decrypt(u8 out[], u8 const in[], u8 const rk[],
    int rounds, int blocks, u8 iv[]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aesbs_ctx {
    pub 32]: *mut *mut *mut u8 rk[13  (8  AES_BLOCK_SIZE) +,
    pub rounds: c_int,
    pub __aligned(AES_BLOCK_SIZE): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aesbs_cbc_ctr_ctx {
    pub key: aesbs_ctx,
    pub enc: [u32; AES_MAX_KEYLENGTH_U32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aesbs_xts_ctx {
    pub key: aesbs_ctx,
    pub twkey: [u32; AES_MAX_KEYLENGTH_U32],
    pub cts: crypto_aes_ctx,
}

    static int aesbs_setkey(struct crypto_skcipher *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct aesbs_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct crypto_aes_ctx *rk;
    int err;
    rk = kmalloc_obj(*rk);
    if (!rk)
    return -ENOMEM;
    err = aes_expandkey(rk, in_key, key_len);
    if (err)
    goto out;
    ctx.rounds = 6 + key_len / 4;
    scoped_ksimd()
    aesbs_convert_key(ctx.rk, rk.key_enc, ctx.rounds);
    out:
    kfree_sensitive(rk);
    return err;
    }
    static int __ecb_crypt(struct skcipher_request *req,
    void (*fn)(u8 out[], u8 const in[], u8 const rk[],
    int rounds, int blocks))
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct aesbs_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk walk;
    int err;
    err = skcipher_walk_virt(&walk, req, false);
    while (walk.nbytes >= AES_BLOCK_SIZE) {
    let mut blocks: c_uint = walk.nbytes / AES_BLOCK_SIZE;
    if (walk.nbytes < walk.total)
    blocks = round_down(blocks,
    walk.stride / AES_BLOCK_SIZE);
    scoped_ksimd()
    fn(walk.dst.virt.addr, walk.src.virt.addr, ctx.rk,
    ctx.rounds, blocks);
    err = skcipher_walk_done(&walk,
    walk.nbytes - blocks * AES_BLOCK_SIZE);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ecb_encrypt(req: *mut skcipher_request) -> c_int {
    static int ecb_encrypt(struct skcipher_request *req)
    {
    return __ecb_crypt(req, aesbs_ecb_encrypt);
    }
#[no_mangle]
unsafe extern "C" fn ecb_decrypt(req: *mut skcipher_request) -> c_int {
    static int ecb_decrypt(struct skcipher_request *req)
    {
    return __ecb_crypt(req, aesbs_ecb_decrypt);
    }
    static int aesbs_cbc_ctr_setkey(struct crypto_skcipher *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct aesbs_cbc_ctr_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct crypto_aes_ctx *rk;
    int err;
    rk = kmalloc_obj(*rk);
    if (!rk)
    return -ENOMEM;
    err = aes_expandkey(rk, in_key, key_len);
    if (err)
    goto out;
    ctx.key.rounds = 6 + key_len / 4;
    memcpy(ctx.enc, rk.key_enc, sizeof(ctx.enc));
    scoped_ksimd()
    aesbs_convert_key(ctx.key.rk, rk.key_enc, ctx.key.rounds);
    out:
    kfree_sensitive(rk);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cbc_encrypt(req: *mut skcipher_request) -> c_int {
    static int cbc_encrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct aesbs_cbc_ctr_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk walk;
    int err;
    err = skcipher_walk_virt(&walk, req, false);
    while (walk.nbytes >= AES_BLOCK_SIZE) {
    let mut blocks: c_uint = walk.nbytes / AES_BLOCK_SIZE;
// fall back to the non-bitsliced NEON implementation
    scoped_ksimd()
    neon_aes_cbc_encrypt(walk.dst.virt.addr,
    walk.src.virt.addr,
    ctx.enc, ctx.key.rounds, blocks,
    walk.iv);
    err = skcipher_walk_done(&walk, walk.nbytes % AES_BLOCK_SIZE);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cbc_decrypt(req: *mut skcipher_request) -> c_int {
    static int cbc_decrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct aesbs_cbc_ctr_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk walk;
    int err;
    err = skcipher_walk_virt(&walk, req, false);
    while (walk.nbytes >= AES_BLOCK_SIZE) {
    let mut blocks: c_uint = walk.nbytes / AES_BLOCK_SIZE;
    if (walk.nbytes < walk.total)
    blocks = round_down(blocks,
    walk.stride / AES_BLOCK_SIZE);
    scoped_ksimd()
    aesbs_cbc_decrypt(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key.rk, ctx.key.rounds, blocks,
    walk.iv);
    err = skcipher_walk_done(&walk,
    walk.nbytes - blocks * AES_BLOCK_SIZE);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ctr_encrypt(req: *mut skcipher_request) -> c_int {
    static int ctr_encrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct aesbs_cbc_ctr_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk walk;
    int err;
    err = skcipher_walk_virt(&walk, req, false);
    while (walk.nbytes > 0) {
    let mut blocks: c_int = (walk.nbytes / AES_BLOCK_SIZE) & ~7;
    let mut nbytes: c_int = walk.nbytes % (8 * AES_BLOCK_SIZE);
    const u8 *src = walk.src.virt.addr;
    u8 *dst = walk.dst.virt.addr;
    scoped_ksimd() {
    if (blocks >= 8) {
    aesbs_ctr_encrypt(dst, src, ctx.key.rk,
    ctx.key.rounds, blocks,
    walk.iv);
    dst += blocks * AES_BLOCK_SIZE;
    src += blocks * AES_BLOCK_SIZE;
    }
    if (nbytes && walk.nbytes == walk.total) {
    u8 buf[AES_BLOCK_SIZE];
    u8 *d = dst;
    if (unlikely(nbytes < AES_BLOCK_SIZE))
    src = dst = memcpy(buf + sizeof(buf) -
    nbytes, src, nbytes);
    neon_aes_ctr_encrypt(dst, src, ctx.enc,
    ctx.key.rounds, nbytes,
    walk.iv);
    if (unlikely(nbytes < AES_BLOCK_SIZE))
    memcpy(d, dst, nbytes);
    nbytes = 0;
    }
    }
    err = skcipher_walk_done(&walk, nbytes);
    }
    return err;
    }
    static int aesbs_xts_setkey(struct crypto_skcipher *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct aesbs_xts_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct crypto_aes_ctx rk;
    int err;
    err = xts_verify_key(tfm, in_key, key_len);
    if (err)
    return err;
    key_len /= 2;
    err = aes_expandkey(&ctx.cts, in_key, key_len);
    if (err)
    return err;
    err = aes_expandkey(&rk, in_key + key_len, key_len);
    if (err)
    return err;
    memcpy(ctx.twkey, rk.key_enc, sizeof(ctx.twkey));
    return aesbs_setkey(tfm, in_key, key_len);
    }
    static int __xts_crypt(struct skcipher_request *req, bool encrypt,
    void (*fn)(u8 out[], u8 const in[], u8 const rk[],
    int rounds, int blocks, u8 iv[]))
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct aesbs_xts_ctx *ctx = crypto_skcipher_ctx(tfm);
    let mut tail: c_int = req.cryptlen % (8 * AES_BLOCK_SIZE);
    struct scatterlist sg_src[2], sg_dst[2];
    struct skcipher_request subreq;
    struct scatterlist *src, *dst;
    struct skcipher_walk walk;
    int nbytes, err;
    let mut first: c_int = 1;
    const u8 *in;
    u8 *out;
    if (req.cryptlen < AES_BLOCK_SIZE)
    return -EINVAL;
// ensure that the cts tail is covered by a single step
    if (unlikely(tail > 0 && tail < AES_BLOCK_SIZE)) {
    int xts_blocks = DIV_ROUND_UP(req.cryptlen,
    AES_BLOCK_SIZE) - 2;
    skcipher_request_set_tfm(&subreq, tfm);
    skcipher_request_set_callback(&subreq,
    skcipher_request_flags(req),
    core::ptr::null_mut(), core::ptr::null_mut());
    skcipher_request_set_crypt(&subreq, req.src, req.dst,
    xts_blocks * AES_BLOCK_SIZE,
    req.iv);
    req = &subreq;
    } else {
    tail = 0;
    }
    err = skcipher_walk_virt(&walk, req, false);
    if (err)
    return err;
    scoped_ksimd() {
    while (walk.nbytes >= AES_BLOCK_SIZE) {
    let mut blocks: c_int = (walk.nbytes / AES_BLOCK_SIZE) & ~7;
    out = walk.dst.virt.addr;
    in = walk.src.virt.addr;
    nbytes = walk.nbytes;
    if (blocks >= 8) {
    if (first == 1)
    neon_aes_ecb_encrypt(walk.iv, walk.iv,
    ctx.twkey,
    ctx.key.rounds, 1);
    first = 2;
    fn(out, in, ctx.key.rk, ctx.key.rounds, blocks,
    walk.iv);
    out += blocks * AES_BLOCK_SIZE;
    in += blocks * AES_BLOCK_SIZE;
    nbytes -= blocks * AES_BLOCK_SIZE;
    }
    if (walk.nbytes == walk.total && nbytes > 0) {
    if (encrypt)
    neon_aes_xts_encrypt(out, in, ctx.cts.key_enc,
    ctx.key.rounds, nbytes,
    ctx.twkey, walk.iv, first);
    else
    neon_aes_xts_decrypt(out, in, ctx.cts.key_dec,
    ctx.key.rounds, nbytes,
    ctx.twkey, walk.iv, first);
    nbytes = first = 0;
    }
    err = skcipher_walk_done(&walk, nbytes);
    }
    if (err || likely(!tail))
    return err;
// handle ciphertext stealing
    dst = src = scatterwalk_ffwd(sg_src, req.src, req.cryptlen);
    if (req.dst != req.src)
    dst = scatterwalk_ffwd(sg_dst, req.dst, req.cryptlen);
    skcipher_request_set_crypt(req, src, dst, AES_BLOCK_SIZE + tail,
    req.iv);
    err = skcipher_walk_virt(&walk, req, false);
    if (err)
    return err;
    out = walk.dst.virt.addr;
    in = walk.src.virt.addr;
    nbytes = walk.nbytes;
    if (encrypt)
    neon_aes_xts_encrypt(out, in, ctx.cts.key_enc,
    ctx.key.rounds, nbytes, ctx.twkey,
    walk.iv, first);
    else
    neon_aes_xts_decrypt(out, in, ctx.cts.key_dec,
    ctx.key.rounds, nbytes, ctx.twkey,
    walk.iv, first);
    }
    return skcipher_walk_done(&walk, 0);
    }
#[no_mangle]
unsafe extern "C" fn xts_encrypt(req: *mut skcipher_request) -> c_int {
    static int xts_encrypt(struct skcipher_request *req)
    {
    return __xts_crypt(req, true, aesbs_xts_encrypt);
    }
#[no_mangle]
unsafe extern "C" fn xts_decrypt(req: *mut skcipher_request) -> c_int {
    static int xts_decrypt(struct skcipher_request *req)
    {
    return __xts_crypt(req, false, aesbs_xts_decrypt);
    }
    static struct skcipher_alg aes_algs[] = { {
    .base.cra_name		= "ecb(aes)",
    .base.cra_driver_name	= "ecb-aes-neonbs",
    .base.cra_priority	= 250,
    .base.cra_blocksize	= AES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct aesbs_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= AES_MIN_KEY_SIZE,
    .max_keysize		= AES_MAX_KEY_SIZE,
    .walksize		= 8 * AES_BLOCK_SIZE,
    .setkey			= aesbs_setkey,
    .encrypt		= ecb_encrypt,
    .decrypt		= ecb_decrypt,
    }, {
    .base.cra_name		= "cbc(aes)",
    .base.cra_driver_name	= "cbc-aes-neonbs",
    .base.cra_priority	= 250,
    .base.cra_blocksize	= AES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct aesbs_cbc_ctr_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= AES_MIN_KEY_SIZE,
    .max_keysize		= AES_MAX_KEY_SIZE,
    .walksize		= 8 * AES_BLOCK_SIZE,
    .ivsize			= AES_BLOCK_SIZE,
    .setkey			= aesbs_cbc_ctr_setkey,
    .encrypt		= cbc_encrypt,
    .decrypt		= cbc_decrypt,
    }, {
    .base.cra_name		= "ctr(aes)",
    .base.cra_driver_name	= "ctr-aes-neonbs",
    .base.cra_priority	= 250,
    .base.cra_blocksize	= 1,
    .base.cra_ctxsize	= sizeof(struct aesbs_cbc_ctr_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= AES_MIN_KEY_SIZE,
    .max_keysize		= AES_MAX_KEY_SIZE,
    .chunksize		= AES_BLOCK_SIZE,
    .walksize		= 8 * AES_BLOCK_SIZE,
    .ivsize			= AES_BLOCK_SIZE,
    .setkey			= aesbs_cbc_ctr_setkey,
    .encrypt		= ctr_encrypt,
    .decrypt		= ctr_encrypt,
    }, {
    .base.cra_name		= "xts(aes)",
    .base.cra_driver_name	= "xts-aes-neonbs",
    .base.cra_priority	= 250,
    .base.cra_blocksize	= AES_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct aesbs_xts_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= 2 * AES_MIN_KEY_SIZE,
    .max_keysize		= 2 * AES_MAX_KEY_SIZE,
    .walksize		= 8 * AES_BLOCK_SIZE,
    .ivsize			= AES_BLOCK_SIZE,
    .setkey			= aesbs_xts_setkey,
    .encrypt		= xts_encrypt,
    .decrypt		= xts_decrypt,
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
    if (!cpu_have_named_feature(ASIMD))
    return -ENODEV;
    return crypto_register_skciphers(aes_algs, ARRAY_SIZE(aes_algs));
    }
    module_init(aes_init);
    module_exit(aes_exit);
