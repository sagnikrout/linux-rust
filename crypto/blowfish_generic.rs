//! Automatically rewritten from C to Rust
//! Source: crypto/blowfish_generic.c
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
// Blowfish Cipher Algorithm, by Bruce Schneier.
// http://www.counterpane.com/blowfish.html
//
// Adapted from Kerneli implementation.
//
// Copyright (c) Herbert Valerio Riedel <hvr@hvrlab.org>
// Copyright (c) Kyle McMartin <kyle@debian.org>
// Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
//

//
// Round loop unrolling macros, S is a pointer to a S-Box array
// organized in 4 unsigned longs at a row.
//

    S[512 + GET32_2(x)]) + S[768 + GET32_3(x)])

#[no_mangle]
unsafe extern "C" fn bf_encrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    static void bf_encrypt(struct crypto_tfm *tfm, u8 *dst, const u8 *src)
    {
    struct bf_ctx *ctx = crypto_tfm_ctx(tfm);
    const u32 *P = ctx.p;
    const u32 *S = ctx.s;
    let mut yl: u32 = get_unaligned_be32(src);
    let mut yr: u32 = get_unaligned_be32(src + 4);
    ROUND(yr, yl, 0);
    ROUND(yl, yr, 1);
    ROUND(yr, yl, 2);
    ROUND(yl, yr, 3);
    ROUND(yr, yl, 4);
    ROUND(yl, yr, 5);
    ROUND(yr, yl, 6);
    ROUND(yl, yr, 7);
    ROUND(yr, yl, 8);
    ROUND(yl, yr, 9);
    ROUND(yr, yl, 10);
    ROUND(yl, yr, 11);
    ROUND(yr, yl, 12);
    ROUND(yl, yr, 13);
    ROUND(yr, yl, 14);
    ROUND(yl, yr, 15);
    yl ^= P[16];
    yr ^= P[17];
    put_unaligned_be32(yr, dst);
    put_unaligned_be32(yl, dst + 4);
    }
#[no_mangle]
unsafe extern "C" fn bf_decrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    static void bf_decrypt(struct crypto_tfm *tfm, u8 *dst, const u8 *src)
    {
    struct bf_ctx *ctx = crypto_tfm_ctx(tfm);
    const u32 *P = ctx.p;
    const u32 *S = ctx.s;
    let mut yl: u32 = get_unaligned_be32(src);
    let mut yr: u32 = get_unaligned_be32(src + 4);
    ROUND(yr, yl, 17);
    ROUND(yl, yr, 16);
    ROUND(yr, yl, 15);
    ROUND(yl, yr, 14);
    ROUND(yr, yl, 13);
    ROUND(yl, yr, 12);
    ROUND(yr, yl, 11);
    ROUND(yl, yr, 10);
    ROUND(yr, yl, 9);
    ROUND(yl, yr, 8);
    ROUND(yr, yl, 7);
    ROUND(yl, yr, 6);
    ROUND(yr, yl, 5);
    ROUND(yl, yr, 4);
    ROUND(yr, yl, 3);
    ROUND(yl, yr, 2);
    yl ^= P[1];
    yr ^= P[0];
    put_unaligned_be32(yr, dst);
    put_unaligned_be32(yl, dst + 4);
    }
    static struct crypto_alg alg = {
    .cra_name		=	"blowfish",
    .cra_driver_name	=	"blowfish-generic",
    .cra_priority		=	100,
    .cra_flags		=	CRYPTO_ALG_TYPE_CIPHER,
    .cra_blocksize		=	BF_BLOCK_SIZE,
    .cra_ctxsize		=	sizeof(struct bf_ctx),
    .cra_module		=	THIS_MODULE,
    .cra_u			=	{ .cipher = {
    .cia_min_keysize	=	BF_MIN_KEY_SIZE,
    .cia_max_keysize	=	BF_MAX_KEY_SIZE,
    .cia_setkey		=	blowfish_setkey,
    .cia_encrypt		=	bf_encrypt,
    .cia_decrypt		=	bf_decrypt } }
    };
#[no_mangle]
unsafe extern "C" fn blowfish_mod_init() -> int __init {
    static int __init blowfish_mod_init(void)
    {
    return crypto_register_alg(&alg);
    }
#[no_mangle]
unsafe extern "C" fn blowfish_mod_fini() -> void __exit {
    static void __exit blowfish_mod_fini(void)
    {
    crypto_unregister_alg(&alg);
    }
    module_init(blowfish_mod_init);
    module_exit(blowfish_mod_fini);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Blowfish Cipher Algorithm");
    MODULE_ALIAS_CRYPTO("blowfish");
    MODULE_ALIAS_CRYPTO("blowfish-generic");
