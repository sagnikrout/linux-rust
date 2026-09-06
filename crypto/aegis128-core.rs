//! Automatically rewritten from C to Rust
//! Source: crypto/aegis128-core.c
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
//
// Copyright (c) 2017-2018 Ondrej Mosnacek <omosnacek@gmail.com>
// Copyright (C) 2017-2018 Red Hat, Inc. All rights reserved.
//

pub const AEGIS128_NONCE_SIZE: c_int = 16;
pub const AEGIS128_STATE_BLOCKS: c_int = 5;
pub const AEGIS128_KEY_SIZE: c_int = 16;
pub const AEGIS128_MIN_AUTH_SIZE: c_int = 8;
pub const AEGIS128_MAX_AUTH_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aegis_state {
    pub blocks: [union aegis_block; AEGIS128_STATE_BLOCKS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aegis_ctx {
    pub key: union aegis_block,
}

    static __ro_after_init DEFINE_STATIC_KEY_FALSE(have_simd);
    static const union aegis_block crypto_aegis_const[2] = {
    { .words64 = {
    cpu_to_le64(U64_C(0x0d08050302010100)),
    cpu_to_le64(U64_C(0x6279e99059372215)),
    } },
    { .words64 = {
    cpu_to_le64(U64_C(0xf12fc26d55183ddb)),
    cpu_to_le64(U64_C(0xdd28b57342311120)),
    } },
    };
#[no_mangle]
unsafe extern "C" fn aegis128_do_simd() -> bool {
    static bool aegis128_do_simd(void)
    {

    if (static_branch_likely(&have_simd))
    return crypto_simd_usable();

    return false;
    }
#[no_mangle]
unsafe extern "C" fn crypto_aegis128_update(state: *mut aegis_state) {
    static void crypto_aegis128_update(struct aegis_state *state)
    {
    union aegis_block tmp;
    unsigned int i;
    tmp = state.blocks[AEGIS128_STATE_BLOCKS - 1];
    for (i = AEGIS128_STATE_BLOCKS - 1; i > 0; i--)
    crypto_aegis_aesenc(&state.blocks[i], &state.blocks[i - 1],
    &state.blocks[i]);
    crypto_aegis_aesenc(&state.blocks[0], &tmp, &state.blocks[0]);
    }
    static void crypto_aegis128_update_a(struct aegis_state *state,
    const union aegis_block *msg,
    bool do_simd)
    {
    if (IS_ENABLED(CONFIG_CRYPTO_AEGIS128_SIMD) && do_simd) {
    crypto_aegis128_update_simd(state, msg);
    return;
    }
    crypto_aegis128_update(state);
    crypto_aegis_block_xor(&state.blocks[0], msg);
    }
    static void crypto_aegis128_update_u(struct aegis_state *state, const void *msg,
    bool do_simd)
    {
    if (IS_ENABLED(CONFIG_CRYPTO_AEGIS128_SIMD) && do_simd) {
    crypto_aegis128_update_simd(state, msg);
    return;
    }
    crypto_aegis128_update(state);
    crypto_xor(state.blocks[0].bytes, msg, AEGIS_BLOCK_SIZE);
    }
    static void crypto_aegis128_init(struct aegis_state *state,
    const union aegis_block *key,
    const u8 *iv)
    {
    union aegis_block key_iv;
    unsigned int i;
    key_iv = *key;
    crypto_xor(key_iv.bytes, iv, AEGIS_BLOCK_SIZE);
    state.blocks[0] = key_iv;
    state.blocks[1] = crypto_aegis_const[1];
    state.blocks[2] = crypto_aegis_const[0];
    state.blocks[3] = *key;
    state.blocks[4] = *key;
    crypto_aegis_block_xor(&state.blocks[3], &crypto_aegis_const[0]);
    crypto_aegis_block_xor(&state.blocks[4], &crypto_aegis_const[1]);
    for (i = 0; i < 5; i++) {
    crypto_aegis128_update_a(state, key, false);
    crypto_aegis128_update_a(state, &key_iv, false);
    }
    }
    static void crypto_aegis128_ad(struct aegis_state *state,
    const u8 *src, unsigned int size,
    bool do_simd)
    {
    if (AEGIS_ALIGNED(src)) {
    const union aegis_block *src_blk =
    (const union aegis_block *)src;
    while (size >= AEGIS_BLOCK_SIZE) {
    crypto_aegis128_update_a(state, src_blk, do_simd);
    size -= AEGIS_BLOCK_SIZE;
    src_blk++;
    }
    } else {
    while (size >= AEGIS_BLOCK_SIZE) {
    crypto_aegis128_update_u(state, src, do_simd);
    size -= AEGIS_BLOCK_SIZE;
    src += AEGIS_BLOCK_SIZE;
    }
    }
    }
    static void crypto_aegis128_wipe_chunk(struct aegis_state *state, u8 *dst,
    const u8 *src, unsigned int size)
    {
    memzero_explicit(dst, size);
    }
    static void crypto_aegis128_encrypt_chunk(struct aegis_state *state, u8 *dst,
    const u8 *src, unsigned int size)
    {
    union aegis_block tmp;
    if (AEGIS_ALIGNED(src) && AEGIS_ALIGNED(dst)) {
    while (size >= AEGIS_BLOCK_SIZE) {
    union aegis_block *dst_blk =
    (union aegis_block *)dst;
    const union aegis_block *src_blk =
    (const union aegis_block *)src;
    tmp = state.blocks[2];
    crypto_aegis_block_and(&tmp, &state.blocks[3]);
    crypto_aegis_block_xor(&tmp, &state.blocks[4]);
    crypto_aegis_block_xor(&tmp, &state.blocks[1]);
    crypto_aegis_block_xor(&tmp, src_blk);
    crypto_aegis128_update_a(state, src_blk, false);
// dst_blk = tmp;
    size -= AEGIS_BLOCK_SIZE;
    src += AEGIS_BLOCK_SIZE;
    dst += AEGIS_BLOCK_SIZE;
    }
    } else {
    while (size >= AEGIS_BLOCK_SIZE) {
    tmp = state.blocks[2];
    crypto_aegis_block_and(&tmp, &state.blocks[3]);
    crypto_aegis_block_xor(&tmp, &state.blocks[4]);
    crypto_aegis_block_xor(&tmp, &state.blocks[1]);
    crypto_xor(tmp.bytes, src, AEGIS_BLOCK_SIZE);
    crypto_aegis128_update_u(state, src, false);
    memcpy(dst, tmp.bytes, AEGIS_BLOCK_SIZE);
    size -= AEGIS_BLOCK_SIZE;
    src += AEGIS_BLOCK_SIZE;
    dst += AEGIS_BLOCK_SIZE;
    }
    }
    if (size > 0) {
    let mut msg: union aegis_block = {};
    memcpy(msg.bytes, src, size);
    tmp = state.blocks[2];
    crypto_aegis_block_and(&tmp, &state.blocks[3]);
    crypto_aegis_block_xor(&tmp, &state.blocks[4]);
    crypto_aegis_block_xor(&tmp, &state.blocks[1]);
    crypto_aegis128_update_a(state, &msg, false);
    crypto_aegis_block_xor(&msg, &tmp);
    memcpy(dst, msg.bytes, size);
    }
    }
    static void crypto_aegis128_decrypt_chunk(struct aegis_state *state, u8 *dst,
    const u8 *src, unsigned int size)
    {
    union aegis_block tmp;
    if (AEGIS_ALIGNED(src) && AEGIS_ALIGNED(dst)) {
    while (size >= AEGIS_BLOCK_SIZE) {
    union aegis_block *dst_blk =
    (union aegis_block *)dst;
    const union aegis_block *src_blk =
    (const union aegis_block *)src;
    tmp = state.blocks[2];
    crypto_aegis_block_and(&tmp, &state.blocks[3]);
    crypto_aegis_block_xor(&tmp, &state.blocks[4]);
    crypto_aegis_block_xor(&tmp, &state.blocks[1]);
    crypto_aegis_block_xor(&tmp, src_blk);
    crypto_aegis128_update_a(state, &tmp, false);
// dst_blk = tmp;
    size -= AEGIS_BLOCK_SIZE;
    src += AEGIS_BLOCK_SIZE;
    dst += AEGIS_BLOCK_SIZE;
    }
    } else {
    while (size >= AEGIS_BLOCK_SIZE) {
    tmp = state.blocks[2];
    crypto_aegis_block_and(&tmp, &state.blocks[3]);
    crypto_aegis_block_xor(&tmp, &state.blocks[4]);
    crypto_aegis_block_xor(&tmp, &state.blocks[1]);
    crypto_xor(tmp.bytes, src, AEGIS_BLOCK_SIZE);
    crypto_aegis128_update_a(state, &tmp, false);
    memcpy(dst, tmp.bytes, AEGIS_BLOCK_SIZE);
    size -= AEGIS_BLOCK_SIZE;
    src += AEGIS_BLOCK_SIZE;
    dst += AEGIS_BLOCK_SIZE;
    }
    }
    if (size > 0) {
    let mut msg: union aegis_block = {};
    memcpy(msg.bytes, src, size);
    tmp = state.blocks[2];
    crypto_aegis_block_and(&tmp, &state.blocks[3]);
    crypto_aegis_block_xor(&tmp, &state.blocks[4]);
    crypto_aegis_block_xor(&tmp, &state.blocks[1]);
    crypto_aegis_block_xor(&msg, &tmp);
    memset(msg.bytes + size, 0, AEGIS_BLOCK_SIZE - size);
    crypto_aegis128_update_a(state, &msg, false);
    memcpy(dst, msg.bytes, size);
    }
    }
    static void crypto_aegis128_process_ad(struct aegis_state *state,
    struct scatterlist *sg_src,
    unsigned int assoclen,
    bool do_simd)
    {
    struct scatter_walk walk;
    union aegis_block buf;
    let mut pos: c_uint = 0;
    scatterwalk_start(&walk, sg_src);
    while (assoclen != 0) {
    let mut size: c_uint = scatterwalk_next(&walk, assoclen);
    const u8 *src = walk.addr;
    let mut left: c_uint = size;
    if (pos + size >= AEGIS_BLOCK_SIZE) {
    if (pos > 0) {
    let mut fill: c_uint = AEGIS_BLOCK_SIZE - pos;
    memcpy(buf.bytes + pos, src, fill);
    crypto_aegis128_update_a(state, &buf, do_simd);
    pos = 0;
    left -= fill;
    src += fill;
    }
    crypto_aegis128_ad(state, src, left, do_simd);
    src += left & ~(AEGIS_BLOCK_SIZE - 1);
    left &= AEGIS_BLOCK_SIZE - 1;
    }
    memcpy(buf.bytes + pos, src, left);
    pos += left;
    assoclen -= size;
    scatterwalk_done_src(&walk, size);
    }
    if (pos > 0) {
    memset(buf.bytes + pos, 0, AEGIS_BLOCK_SIZE - pos);
    crypto_aegis128_update_a(state, &buf, do_simd);
    }
    }
    static __always_inline
    int crypto_aegis128_process_crypt(struct aegis_state *state,
    struct skcipher_walk *walk,
    void (*crypt)(struct aegis_state *state,
    u8 *dst,
    const u8 *src,
    unsigned int size))
    {
    let mut err: c_int = 0;
    while (walk.nbytes) {
    let mut nbytes: c_uint = walk.nbytes;
    if (nbytes < walk.total)
    nbytes = round_down(nbytes, walk.stride);
    crypt(state, walk.dst.virt.addr, walk.src.virt.addr, nbytes);
    err = skcipher_walk_done(walk, walk.nbytes - nbytes);
    }
    return err;
    }
    static void crypto_aegis128_final(struct aegis_state *state,
    union aegis_block *tag_xor,
    u64 assoclen, u64 cryptlen)
    {
    let mut assocbits: u64 = assoclen * 8;
    let mut cryptbits: u64 = cryptlen * 8;
    union aegis_block tmp;
    unsigned int i;
    tmp.words64[0] = cpu_to_le64(assocbits);
    tmp.words64[1] = cpu_to_le64(cryptbits);
    crypto_aegis_block_xor(&tmp, &state.blocks[3]);
    for (i = 0; i < 7; i++)
    crypto_aegis128_update_a(state, &tmp, false);
    for (i = 0; i < AEGIS128_STATE_BLOCKS; i++)
    crypto_aegis_block_xor(tag_xor, &state.blocks[i]);
    }
    static int crypto_aegis128_setkey(struct crypto_aead *aead, const u8 *key,
    unsigned int keylen)
    {
    struct aegis_ctx *ctx = crypto_aead_ctx(aead);
    if (keylen != AEGIS128_KEY_SIZE)
    return -EINVAL;
    memcpy(ctx.key.bytes, key, AEGIS128_KEY_SIZE);
    return 0;
    }
    static int crypto_aegis128_setauthsize(struct crypto_aead *tfm,
    unsigned int authsize)
    {
    if (authsize > AEGIS128_MAX_AUTH_SIZE)
    return -EINVAL;
    if (authsize < AEGIS128_MIN_AUTH_SIZE)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_aegis128_encrypt_generic(req: *mut aead_request) -> c_int {
    static int crypto_aegis128_encrypt_generic(struct aead_request *req)
    {
    struct crypto_aead *tfm = crypto_aead_reqtfm(req);
    let mut tag: union aegis_block = {};
    let mut authsize: c_uint = crypto_aead_authsize(tfm);
    struct aegis_ctx *ctx = crypto_aead_ctx(tfm);
    let mut cryptlen: c_uint = req.cryptlen;
    struct skcipher_walk walk;
    struct aegis_state state;
    skcipher_walk_aead_encrypt(&walk, req, false);
    crypto_aegis128_init(&state, &ctx.key, req.iv);
    crypto_aegis128_process_ad(&state, req.src, req.assoclen, false);
    crypto_aegis128_process_crypt(&state, &walk,
    crypto_aegis128_encrypt_chunk);
    crypto_aegis128_final(&state, &tag, req.assoclen, cryptlen);
    scatterwalk_map_and_copy(tag.bytes, req.dst, req.assoclen + cryptlen,
    authsize, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_aegis128_decrypt_generic(req: *mut aead_request) -> c_int {
    static int crypto_aegis128_decrypt_generic(struct aead_request *req)
    {
    static const u8 zeros[AEGIS128_MAX_AUTH_SIZE] = {};
    struct crypto_aead *tfm = crypto_aead_reqtfm(req);
    union aegis_block tag;
    let mut authsize: c_uint = crypto_aead_authsize(tfm);
    let mut cryptlen: c_uint = req.cryptlen - authsize;
    struct aegis_ctx *ctx = crypto_aead_ctx(tfm);
    struct skcipher_walk walk;
    struct aegis_state state;
    scatterwalk_map_and_copy(tag.bytes, req.src, req.assoclen + cryptlen,
    authsize, 0);
    skcipher_walk_aead_decrypt(&walk, req, false);
    crypto_aegis128_init(&state, &ctx.key, req.iv);
    crypto_aegis128_process_ad(&state, req.src, req.assoclen, false);
    crypto_aegis128_process_crypt(&state, &walk,
    crypto_aegis128_decrypt_chunk);
    crypto_aegis128_final(&state, &tag, req.assoclen, cryptlen);
    if (unlikely(crypto_memneq(tag.bytes, zeros, authsize))) {
//
// From Chapter 4. 'Security Analysis' of the AEGIS spec [0]
//
// "3. If verification fails, the decrypted plaintext and the
// wrong authentication tag should not be given as output."
//
// [0] https://competitions.cr.yp.to/round3/aegisv11.pdf
//
    skcipher_walk_aead_decrypt(&walk, req, false);
    crypto_aegis128_process_crypt(core::ptr::null_mut(), &walk,
    crypto_aegis128_wipe_chunk);
    memzero_explicit(&tag, sizeof(tag));
    return -EBADMSG;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_aegis128_encrypt_simd(req: *mut aead_request) -> c_int {
    static int crypto_aegis128_encrypt_simd(struct aead_request *req)
    {
    struct crypto_aead *tfm = crypto_aead_reqtfm(req);
    let mut tag: union aegis_block = {};
    let mut authsize: c_uint = crypto_aead_authsize(tfm);
    struct aegis_ctx *ctx = crypto_aead_ctx(tfm);
    let mut cryptlen: c_uint = req.cryptlen;
    struct skcipher_walk walk;
    struct aegis_state state;
    if (!aegis128_do_simd())
    return crypto_aegis128_encrypt_generic(req);
    skcipher_walk_aead_encrypt(&walk, req, false);
    crypto_aegis128_init_simd(&state, &ctx.key, req.iv);
    crypto_aegis128_process_ad(&state, req.src, req.assoclen, true);
    crypto_aegis128_process_crypt(&state, &walk,
    crypto_aegis128_encrypt_chunk_simd);
    crypto_aegis128_final_simd(&state, &tag, req.assoclen, cryptlen, 0);
    scatterwalk_map_and_copy(tag.bytes, req.dst, req.assoclen + cryptlen,
    authsize, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_aegis128_decrypt_simd(req: *mut aead_request) -> c_int {
    static int crypto_aegis128_decrypt_simd(struct aead_request *req)
    {
    struct crypto_aead *tfm = crypto_aead_reqtfm(req);
    union aegis_block tag;
    let mut authsize: c_uint = crypto_aead_authsize(tfm);
    let mut cryptlen: c_uint = req.cryptlen - authsize;
    struct aegis_ctx *ctx = crypto_aead_ctx(tfm);
    struct skcipher_walk walk;
    struct aegis_state state;
    if (!aegis128_do_simd())
    return crypto_aegis128_decrypt_generic(req);
    scatterwalk_map_and_copy(tag.bytes, req.src, req.assoclen + cryptlen,
    authsize, 0);
    skcipher_walk_aead_decrypt(&walk, req, false);
    crypto_aegis128_init_simd(&state, &ctx.key, req.iv);
    crypto_aegis128_process_ad(&state, req.src, req.assoclen, true);
    crypto_aegis128_process_crypt(&state, &walk,
    crypto_aegis128_decrypt_chunk_simd);
    if (unlikely(crypto_aegis128_final_simd(&state, &tag, req.assoclen,
    cryptlen, authsize))) {
    skcipher_walk_aead_decrypt(&walk, req, false);
    crypto_aegis128_process_crypt(core::ptr::null_mut(), &walk,
    crypto_aegis128_wipe_chunk);
    return -EBADMSG;
    }
    return 0;
    }
    static struct aead_alg crypto_aegis128_alg_generic = {
    .setkey			= crypto_aegis128_setkey,
    .setauthsize		= crypto_aegis128_setauthsize,
    .encrypt		= crypto_aegis128_encrypt_generic,
    .decrypt		= crypto_aegis128_decrypt_generic,
    .ivsize			= AEGIS128_NONCE_SIZE,
    .maxauthsize		= AEGIS128_MAX_AUTH_SIZE,
    .chunksize		= AEGIS_BLOCK_SIZE,
    .base.cra_blocksize	= 1,
    .base.cra_ctxsize	= sizeof(struct aegis_ctx),
    .base.cra_priority	= 100,
    .base.cra_name		= "aegis128",
    .base.cra_driver_name	= "aegis128-generic",
    .base.cra_module	= THIS_MODULE,
    };
    static struct aead_alg crypto_aegis128_alg_simd = {
    .setkey			= crypto_aegis128_setkey,
    .setauthsize		= crypto_aegis128_setauthsize,
    .encrypt		= crypto_aegis128_encrypt_simd,
    .decrypt		= crypto_aegis128_decrypt_simd,
    .ivsize			= AEGIS128_NONCE_SIZE,
    .maxauthsize		= AEGIS128_MAX_AUTH_SIZE,
    .chunksize		= AEGIS_BLOCK_SIZE,
    .base.cra_blocksize	= 1,
    .base.cra_ctxsize	= sizeof(struct aegis_ctx),
    .base.cra_priority	= 200,
    .base.cra_name		= "aegis128",
    .base.cra_driver_name	= "aegis128-simd",
    .base.cra_module	= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn crypto_aegis128_module_init() -> int __init {
    static int __init crypto_aegis128_module_init(void)
    {
    int ret;
    ret = crypto_register_aead(&crypto_aegis128_alg_generic);
    if (ret)
    return ret;
    if (IS_ENABLED(CONFIG_CRYPTO_AEGIS128_SIMD) &&
    crypto_aegis128_have_simd()) {
    ret = crypto_register_aead(&crypto_aegis128_alg_simd);
    if (ret) {
    crypto_unregister_aead(&crypto_aegis128_alg_generic);
    return ret;
    }
    static_branch_enable(&have_simd);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_aegis128_module_exit() -> void __exit {
    static void __exit crypto_aegis128_module_exit(void)
    {
    if (IS_ENABLED(CONFIG_CRYPTO_AEGIS128_SIMD) &&
    crypto_aegis128_have_simd())
    crypto_unregister_aead(&crypto_aegis128_alg_simd);
    crypto_unregister_aead(&crypto_aegis128_alg_generic);
    }
    module_init(crypto_aegis128_module_init);
    module_exit(crypto_aegis128_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Ondrej Mosnacek <omosnacek@gmail.com>");
    MODULE_DESCRIPTION("AEGIS-128 AEAD algorithm");
    MODULE_ALIAS_CRYPTO("aegis128");
    MODULE_ALIAS_CRYPTO("aegis128-generic");
    MODULE_ALIAS_CRYPTO("aegis128-simd");
