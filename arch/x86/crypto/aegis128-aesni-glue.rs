//! Automatically rewritten from C to Rust
//! Source: arch/x86/crypto/aegis128-aesni-glue.c
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
// The AEGIS-128 Authenticated-Encryption Algorithm
// Glue for AES-NI + SSE4.1 implementation
//
// Copyright (c) 2017-2018 Ondrej Mosnacek <omosnacek@gmail.com>
// Copyright (C) 2017-2018 Red Hat, Inc. All rights reserved.
//

pub const AEGIS128_BLOCK_ALIGN: c_int = 16;
pub const AEGIS128_BLOCK_SIZE: c_int = 16;
pub const AEGIS128_NONCE_SIZE: c_int = 16;
pub const AEGIS128_STATE_BLOCKS: c_int = 5;
pub const AEGIS128_KEY_SIZE: c_int = 16;
pub const AEGIS128_MIN_AUTH_SIZE: c_int = 8;
pub const AEGIS128_MAX_AUTH_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aegis_block {
    pub __aligned(AEGIS128_BLOCK_ALIGN): u8 bytes[AEGIS128_BLOCK_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aegis_state {
    pub blocks: [aegis_block; AEGIS128_STATE_BLOCKS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aegis_ctx {
    pub key: aegis_block,
}

    asmlinkage void aegis128_aesni_init(struct aegis_state *state,
    const struct aegis_block *key,
    const u8 iv[AEGIS128_NONCE_SIZE]);
    asmlinkage void aegis128_aesni_ad(struct aegis_state *state, const u8 *data,
    unsigned int len);
    asmlinkage void aegis128_aesni_enc(struct aegis_state *state, const u8 *src,
    u8 *dst, unsigned int len);
    asmlinkage void aegis128_aesni_dec(struct aegis_state *state, const u8 *src,
    u8 *dst, unsigned int len);
    asmlinkage void aegis128_aesni_enc_tail(struct aegis_state *state,
    const u8 *src, u8 *dst,
    unsigned int len);
    asmlinkage void aegis128_aesni_dec_tail(struct aegis_state *state,
    const u8 *src, u8 *dst,
    unsigned int len);
    asmlinkage void aegis128_aesni_final(struct aegis_state *state,
    struct aegis_block *tag_xor,
    unsigned int assoclen,
    unsigned int cryptlen);
    static void crypto_aegis128_aesni_process_ad(
    struct aegis_state *state, struct scatterlist *sg_src,
    unsigned int assoclen)
    {
    struct scatter_walk walk;
    struct aegis_block buf;
    let mut pos: c_uint = 0;
    scatterwalk_start(&walk, sg_src);
    while (assoclen != 0) {
    let mut size: c_uint = scatterwalk_next(&walk, assoclen);
    const u8 *src = walk.addr;
    let mut left: c_uint = size;
    if (pos + size >= AEGIS128_BLOCK_SIZE) {
    if (pos > 0) {
    let mut fill: c_uint = AEGIS128_BLOCK_SIZE - pos;
    memcpy(buf.bytes + pos, src, fill);
    aegis128_aesni_ad(state, buf.bytes,
    AEGIS128_BLOCK_SIZE);
    pos = 0;
    left -= fill;
    src += fill;
    }
    aegis128_aesni_ad(state, src,
    left & ~(AEGIS128_BLOCK_SIZE - 1));
    src += left & ~(AEGIS128_BLOCK_SIZE - 1);
    left &= AEGIS128_BLOCK_SIZE - 1;
    }
    memcpy(buf.bytes + pos, src, left);
    pos += left;
    assoclen -= size;
    scatterwalk_done_src(&walk, size);
    }
    if (pos > 0) {
    memset(buf.bytes + pos, 0, AEGIS128_BLOCK_SIZE - pos);
    aegis128_aesni_ad(state, buf.bytes, AEGIS128_BLOCK_SIZE);
    }
    }
    static __always_inline int
    crypto_aegis128_aesni_process_crypt(struct aegis_state *state,
    struct skcipher_walk *walk, bool enc)
    {
    let mut err: c_int = 0;
    while (walk.nbytes >= AEGIS128_BLOCK_SIZE) {
    if (enc)
    aegis128_aesni_enc(state, walk.src.virt.addr,
    walk.dst.virt.addr,
    round_down(walk.nbytes,
    AEGIS128_BLOCK_SIZE));
    else
    aegis128_aesni_dec(state, walk.src.virt.addr,
    walk.dst.virt.addr,
    round_down(walk.nbytes,
    AEGIS128_BLOCK_SIZE));
    kernel_fpu_end();
    err = skcipher_walk_done(walk,
    walk.nbytes % AEGIS128_BLOCK_SIZE);
    kernel_fpu_begin();
    }
    if (walk.nbytes) {
    if (enc)
    aegis128_aesni_enc_tail(state, walk.src.virt.addr,
    walk.dst.virt.addr,
    walk.nbytes);
    else
    aegis128_aesni_dec_tail(state, walk.src.virt.addr,
    walk.dst.virt.addr,
    walk.nbytes);
    kernel_fpu_end();
    err = skcipher_walk_done(walk, 0);
    kernel_fpu_begin();
    }
    return err;
    }
    static struct aegis_ctx *crypto_aegis128_aesni_ctx(struct crypto_aead *aead)
    {
    u8 *ctx = crypto_aead_ctx(aead);
    ctx = PTR_ALIGN(ctx, __alignof__(struct aegis_ctx));
    return (void *)ctx;
    }
    static int crypto_aegis128_aesni_setkey(struct crypto_aead *aead, const u8 *key,
    unsigned int keylen)
    {
    struct aegis_ctx *ctx = crypto_aegis128_aesni_ctx(aead);
    if (keylen != AEGIS128_KEY_SIZE)
    return -EINVAL;
    memcpy(ctx.key.bytes, key, AEGIS128_KEY_SIZE);
    return 0;
    }
    static int crypto_aegis128_aesni_setauthsize(struct crypto_aead *tfm,
    unsigned int authsize)
    {
    if (authsize > AEGIS128_MAX_AUTH_SIZE)
    return -EINVAL;
    if (authsize < AEGIS128_MIN_AUTH_SIZE)
    return -EINVAL;
    return 0;
    }
    static __always_inline int
    crypto_aegis128_aesni_crypt(struct aead_request *req,
    struct aegis_block *tag_xor,
    unsigned int cryptlen, bool enc)
    {
    struct crypto_aead *tfm = crypto_aead_reqtfm(req);
    struct aegis_ctx *ctx = crypto_aegis128_aesni_ctx(tfm);
    struct skcipher_walk walk;
    struct aegis_state state;
    int err;
    if (enc)
    err = skcipher_walk_aead_encrypt(&walk, req, false);
    else
    err = skcipher_walk_aead_decrypt(&walk, req, false);
    if (err)
    return err;
    kernel_fpu_begin();
    aegis128_aesni_init(&state, &ctx.key, req.iv);
    crypto_aegis128_aesni_process_ad(&state, req.src, req.assoclen);
    err = crypto_aegis128_aesni_process_crypt(&state, &walk, enc);
    if (err == 0)
    aegis128_aesni_final(&state, tag_xor, req.assoclen, cryptlen);
    kernel_fpu_end();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn crypto_aegis128_aesni_encrypt(req: *mut aead_request) -> c_int {
    static int crypto_aegis128_aesni_encrypt(struct aead_request *req)
    {
    struct crypto_aead *tfm = crypto_aead_reqtfm(req);
    let mut tag: aegis_block = {};
    let mut authsize: c_uint = crypto_aead_authsize(tfm);
    let mut cryptlen: c_uint = req.cryptlen;
    int err;
    err = crypto_aegis128_aesni_crypt(req, &tag, cryptlen, true);
    if (err)
    return err;
    scatterwalk_map_and_copy(tag.bytes, req.dst,
    req.assoclen + cryptlen, authsize, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_aegis128_aesni_decrypt(req: *mut aead_request) -> c_int {
    static int crypto_aegis128_aesni_decrypt(struct aead_request *req)
    {
    let mut zeros: static struct aegis_block = {};
    struct crypto_aead *tfm = crypto_aead_reqtfm(req);
    struct aegis_block tag;
    let mut authsize: c_uint = crypto_aead_authsize(tfm);
    let mut cryptlen: c_uint = req.cryptlen - authsize;
    int err;
    scatterwalk_map_and_copy(tag.bytes, req.src,
    req.assoclen + cryptlen, authsize, 0);
    err = crypto_aegis128_aesni_crypt(req, &tag, cryptlen, false);
    if (err)
    return err;
    return crypto_memneq(tag.bytes, zeros.bytes, authsize) ? -EBADMSG : 0;
    }
    static struct aead_alg crypto_aegis128_aesni_alg = {
    .setkey = crypto_aegis128_aesni_setkey,
    .setauthsize = crypto_aegis128_aesni_setauthsize,
    .encrypt = crypto_aegis128_aesni_encrypt,
    .decrypt = crypto_aegis128_aesni_decrypt,
    .ivsize = AEGIS128_NONCE_SIZE,
    .maxauthsize = AEGIS128_MAX_AUTH_SIZE,
    .chunksize = AEGIS128_BLOCK_SIZE,
    .base = {
    .cra_blocksize = 1,
    .cra_ctxsize = sizeof(struct aegis_ctx) +
    __alignof__(struct aegis_ctx),
    .cra_priority = 400,
    .cra_name = "aegis128",
    .cra_driver_name = "aegis128-aesni",
    .cra_module = THIS_MODULE,
    }
    };
#[no_mangle]
unsafe extern "C" fn crypto_aegis128_aesni_module_init() -> int __init {
    static int __init crypto_aegis128_aesni_module_init(void)
    {
    if (!boot_cpu_has(X86_FEATURE_XMM4_1) ||
    !boot_cpu_has(X86_FEATURE_AES) ||
    !cpu_has_xfeatures(XFEATURE_MASK_SSE, core::ptr::null_mut()))
    return -ENODEV;
    return crypto_register_aead(&crypto_aegis128_aesni_alg);
    }
#[no_mangle]
unsafe extern "C" fn crypto_aegis128_aesni_module_exit() -> void __exit {
    static void __exit crypto_aegis128_aesni_module_exit(void)
    {
    crypto_unregister_aead(&crypto_aegis128_aesni_alg);
    }
    module_init(crypto_aegis128_aesni_module_init);
    module_exit(crypto_aegis128_aesni_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Ondrej Mosnacek <omosnacek@gmail.com>");
    MODULE_DESCRIPTION("AEGIS-128 AEAD algorithm -- AESNI+SSE4.1 implementation");
    MODULE_ALIAS_CRYPTO("aegis128");
    MODULE_ALIAS_CRYPTO("aegis128-aesni");
