//! Automatically rewritten from C to Rust
//! Source: crypto/tea.c
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
// Cryptographic API.
//
// TEA, XTEA, and XETA crypto algorithms
//
// The TEA and Xtended TEA algorithms were developed by David Wheeler
// and Roger Needham at the Computer Laboratory of Cambridge University.
//
// Due to the order of evaluation in XTEA many people have incorrectly
// implemented it.  XETA (XTEA in the wrong order), exists for
// compatibility with these implementations.
//
// Copyright (c) 2004 Aaron Grothe ajgrothe@yahoo.com
//

pub const TEA_KEY_SIZE: c_int = 16;
pub const TEA_BLOCK_SIZE: c_int = 8;
pub const TEA_ROUNDS: c_int = 32;
pub const TEA_DELTA: c_uint = 0x9e3779b9;
pub const XTEA_KEY_SIZE: c_int = 16;
pub const XTEA_BLOCK_SIZE: c_int = 8;
pub const XTEA_ROUNDS: c_int = 32;
pub const XTEA_DELTA: c_uint = 0x9e3779b9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tea_ctx {
    pub KEY: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xtea_ctx {
    pub KEY: [u32; 4],
}

    static int tea_setkey(struct crypto_tfm *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct tea_ctx *ctx = crypto_tfm_ctx(tfm);
    ctx.KEY[0] = get_unaligned_le32(&in_key[0]);
    ctx.KEY[1] = get_unaligned_le32(&in_key[4]);
    ctx.KEY[2] = get_unaligned_le32(&in_key[8]);
    ctx.KEY[3] = get_unaligned_le32(&in_key[12]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tea_encrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    static void tea_encrypt(struct crypto_tfm *tfm, u8 *dst, const u8 *src)
    {
    u32 y, z, n, sum = 0;
    u32 k0, k1, k2, k3;
    struct tea_ctx *ctx = crypto_tfm_ctx(tfm);
    y = get_unaligned_le32(&src[0]);
    z = get_unaligned_le32(&src[4]);
    k0 = ctx.KEY[0];
    k1 = ctx.KEY[1];
    k2 = ctx.KEY[2];
    k3 = ctx.KEY[3];
    n = TEA_ROUNDS;
    while (n-- > 0) {
    sum += TEA_DELTA;
    y += ((z << 4) + k0) ^ (z + sum) ^ ((z >> 5) + k1);
    z += ((y << 4) + k2) ^ (y + sum) ^ ((y >> 5) + k3);
    }
    put_unaligned_le32(y, &dst[0]);
    put_unaligned_le32(z, &dst[4]);
    }
#[no_mangle]
unsafe extern "C" fn tea_decrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    static void tea_decrypt(struct crypto_tfm *tfm, u8 *dst, const u8 *src)
    {
    u32 y, z, n, sum;
    u32 k0, k1, k2, k3;
    struct tea_ctx *ctx = crypto_tfm_ctx(tfm);
    y = get_unaligned_le32(&src[0]);
    z = get_unaligned_le32(&src[4]);
    k0 = ctx.KEY[0];
    k1 = ctx.KEY[1];
    k2 = ctx.KEY[2];
    k3 = ctx.KEY[3];
    sum = TEA_DELTA << 5;
    n = TEA_ROUNDS;
    while (n-- > 0) {
    z -= ((y << 4) + k2) ^ (y + sum) ^ ((y >> 5) + k3);
    y -= ((z << 4) + k0) ^ (z + sum) ^ ((z >> 5) + k1);
    sum -= TEA_DELTA;
    }
    put_unaligned_le32(y, &dst[0]);
    put_unaligned_le32(z, &dst[4]);
    }
    static int xtea_setkey(struct crypto_tfm *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct xtea_ctx *ctx = crypto_tfm_ctx(tfm);
    ctx.KEY[0] = get_unaligned_le32(&in_key[0]);
    ctx.KEY[1] = get_unaligned_le32(&in_key[4]);
    ctx.KEY[2] = get_unaligned_le32(&in_key[8]);
    ctx.KEY[3] = get_unaligned_le32(&in_key[12]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xtea_encrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    static void xtea_encrypt(struct crypto_tfm *tfm, u8 *dst, const u8 *src)
    {
    u32 y, z, sum = 0;
    let mut limit: u32 = XTEA_DELTA * XTEA_ROUNDS;
    struct xtea_ctx *ctx = crypto_tfm_ctx(tfm);
    y = get_unaligned_le32(&src[0]);
    z = get_unaligned_le32(&src[4]);
    while (sum != limit) {
    y += ((z << 4 ^ z >> 5) + z) ^ (sum + ctx.KEY[sum&3]);
    sum += XTEA_DELTA;
    z += ((y << 4 ^ y >> 5) + y) ^ (sum + ctx.KEY[sum>>11 &3]);
    }
    put_unaligned_le32(y, &dst[0]);
    put_unaligned_le32(z, &dst[4]);
    }
#[no_mangle]
unsafe extern "C" fn xtea_decrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    static void xtea_decrypt(struct crypto_tfm *tfm, u8 *dst, const u8 *src)
    {
    u32 y, z, sum;
    struct tea_ctx *ctx = crypto_tfm_ctx(tfm);
    y = get_unaligned_le32(&src[0]);
    z = get_unaligned_le32(&src[4]);
    sum = XTEA_DELTA * XTEA_ROUNDS;
    while (sum) {
    z -= ((y << 4 ^ y >> 5) + y) ^ (sum + ctx.KEY[sum>>11 & 3]);
    sum -= XTEA_DELTA;
    y -= ((z << 4 ^ z >> 5) + z) ^ (sum + ctx.KEY[sum & 3]);
    }
    put_unaligned_le32(y, &dst[0]);
    put_unaligned_le32(z, &dst[4]);
    }
#[no_mangle]
unsafe extern "C" fn xeta_encrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    static void xeta_encrypt(struct crypto_tfm *tfm, u8 *dst, const u8 *src)
    {
    u32 y, z, sum = 0;
    let mut limit: u32 = XTEA_DELTA * XTEA_ROUNDS;
    struct xtea_ctx *ctx = crypto_tfm_ctx(tfm);
    y = get_unaligned_le32(&src[0]);
    z = get_unaligned_le32(&src[4]);
    while (sum != limit) {
    y += (z << 4 ^ z >> 5) + (z ^ sum) + ctx.KEY[sum&3];
    sum += XTEA_DELTA;
    z += (y << 4 ^ y >> 5) + (y ^ sum) + ctx.KEY[sum>>11 &3];
    }
    put_unaligned_le32(y, &dst[0]);
    put_unaligned_le32(z, &dst[4]);
    }
#[no_mangle]
unsafe extern "C" fn xeta_decrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    static void xeta_decrypt(struct crypto_tfm *tfm, u8 *dst, const u8 *src)
    {
    u32 y, z, sum;
    struct tea_ctx *ctx = crypto_tfm_ctx(tfm);
    y = get_unaligned_le32(&src[0]);
    z = get_unaligned_le32(&src[4]);
    sum = XTEA_DELTA * XTEA_ROUNDS;
    while (sum) {
    z -= (y << 4 ^ y >> 5) + (y ^ sum) + ctx.KEY[sum>>11 & 3];
    sum -= XTEA_DELTA;
    y -= (z << 4 ^ z >> 5) + (z ^ sum) + ctx.KEY[sum & 3];
    }
    put_unaligned_le32(y, &dst[0]);
    put_unaligned_le32(z, &dst[4]);
    }
    static struct crypto_alg tea_algs[3] = { {
    .cra_name		=	"tea",
    .cra_driver_name	=	"tea-generic",
    .cra_flags		=	CRYPTO_ALG_TYPE_CIPHER,
    .cra_blocksize		=	TEA_BLOCK_SIZE,
    .cra_ctxsize		=	sizeof (struct tea_ctx),
    .cra_module		=	THIS_MODULE,
    .cra_u			=	{ .cipher = {
    .cia_min_keysize	=	TEA_KEY_SIZE,
    .cia_max_keysize	=	TEA_KEY_SIZE,
    .cia_setkey		= 	tea_setkey,
    .cia_encrypt		=	tea_encrypt,
    .cia_decrypt		=	tea_decrypt } }
    }, {
    .cra_name		=	"xtea",
    .cra_driver_name	=	"xtea-generic",
    .cra_flags		=	CRYPTO_ALG_TYPE_CIPHER,
    .cra_blocksize		=	XTEA_BLOCK_SIZE,
    .cra_ctxsize		=	sizeof (struct xtea_ctx),
    .cra_module		=	THIS_MODULE,
    .cra_u			=	{ .cipher = {
    .cia_min_keysize	=	XTEA_KEY_SIZE,
    .cia_max_keysize	=	XTEA_KEY_SIZE,
    .cia_setkey		= 	xtea_setkey,
    .cia_encrypt		=	xtea_encrypt,
    .cia_decrypt		=	xtea_decrypt } }
    }, {
    .cra_name		=	"xeta",
    .cra_driver_name	=	"xeta-generic",
    .cra_flags		=	CRYPTO_ALG_TYPE_CIPHER,
    .cra_blocksize		=	XTEA_BLOCK_SIZE,
    .cra_ctxsize		=	sizeof (struct xtea_ctx),
    .cra_module		=	THIS_MODULE,
    .cra_u			=	{ .cipher = {
    .cia_min_keysize	=	XTEA_KEY_SIZE,
    .cia_max_keysize	=	XTEA_KEY_SIZE,
    .cia_setkey		= 	xtea_setkey,
    .cia_encrypt		=	xeta_encrypt,
    .cia_decrypt		=	xeta_decrypt } }
    } };
#[no_mangle]
unsafe extern "C" fn tea_mod_init() -> int __init {
    static int __init tea_mod_init(void)
    {
    return crypto_register_algs(tea_algs, ARRAY_SIZE(tea_algs));
    }
#[no_mangle]
unsafe extern "C" fn tea_mod_fini() -> void __exit {
    static void __exit tea_mod_fini(void)
    {
    crypto_unregister_algs(tea_algs, ARRAY_SIZE(tea_algs));
    }
    MODULE_ALIAS_CRYPTO("tea");
    MODULE_ALIAS_CRYPTO("xtea");
    MODULE_ALIAS_CRYPTO("xeta");
    module_init(tea_mod_init);
    module_exit(tea_mod_fini);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("TEA, XTEA & XETA Cryptographic Algorithms");
