//! Automatically rewritten from C to Rust
//! Source: crypto/aria_generic.c
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
// ARIA Cipher Algorithm.
//
// Documentation of ARIA can be found in RFC 5794.
// Copyright (c) 2022 Taehee Yoo <ap420073@gmail.com>
//
// Information for ARIA
// http://210.104.33.10/ARIA/index-e.html (English)
// http://seed.kisa.or.kr/ (Korean)
//
// Public domain version is distributed above.
//

    static const u32 key_rc[20] = {
    0x517cc1b7, 0x27220a94, 0xfe13abe8, 0xfa9a6ee0,
    0x6db14acc, 0x9e21c820, 0xff28b1d5, 0xef5de2b0,
    0xdb92371d, 0x2126e970, 0x03249775, 0x04e8c90e,
    0x517cc1b7, 0x27220a94, 0xfe13abe8, 0xfa9a6ee0,
    0x6db14acc, 0x9e21c820, 0xff28b1d5, 0xef5de2b0
    };
    static void aria_set_encrypt_key(struct aria_ctx *ctx, const u8 *in_key,
    unsigned int key_len)
    {
    u32 w0[4], w1[4], w2[4], w3[4];
    u32 reg0, reg1, reg2, reg3;
    const u32 *ck;
    let mut rkidx: c_int = 0;
    ck = &key_rc[(key_len - 16) / 2];
    w0[0] = get_unaligned_be32(&in_key[0]);
    w0[1] = get_unaligned_be32(&in_key[4]);
    w0[2] = get_unaligned_be32(&in_key[8]);
    w0[3] = get_unaligned_be32(&in_key[12]);
    reg0 = w0[0] ^ ck[0];
    reg1 = w0[1] ^ ck[1];
    reg2 = w0[2] ^ ck[2];
    reg3 = w0[3] ^ ck[3];
    aria_subst_diff_odd(&reg0, &reg1, &reg2, &reg3);
    if (key_len > 16) {
    w1[0] = get_unaligned_be32(&in_key[16]);
    w1[1] = get_unaligned_be32(&in_key[20]);
    if (key_len > 24) {
    w1[2] = get_unaligned_be32(&in_key[24]);
    w1[3] = get_unaligned_be32(&in_key[28]);
    } else {
    w1[2] = 0;
    w1[3] = 0;
    }
    } else {
    w1[0] = 0;
    w1[1] = 0;
    w1[2] = 0;
    w1[3] = 0;
    }
    w1[0] ^= reg0;
    w1[1] ^= reg1;
    w1[2] ^= reg2;
    w1[3] ^= reg3;
    reg0 = w1[0];
    reg1 = w1[1];
    reg2 = w1[2];
    reg3 = w1[3];
    reg0 ^= ck[4];
    reg1 ^= ck[5];
    reg2 ^= ck[6];
    reg3 ^= ck[7];
    aria_subst_diff_even(&reg0, &reg1, &reg2, &reg3);
    reg0 ^= w0[0];
    reg1 ^= w0[1];
    reg2 ^= w0[2];
    reg3 ^= w0[3];
    w2[0] = reg0;
    w2[1] = reg1;
    w2[2] = reg2;
    w2[3] = reg3;
    reg0 ^= ck[8];
    reg1 ^= ck[9];
    reg2 ^= ck[10];
    reg3 ^= ck[11];
    aria_subst_diff_odd(&reg0, &reg1, &reg2, &reg3);
    w3[0] = reg0 ^ w1[0];
    w3[1] = reg1 ^ w1[1];
    w3[2] = reg2 ^ w1[2];
    w3[3] = reg3 ^ w1[3];
    aria_gsrk(ctx.enc_key[rkidx], w0, w1, 19);
    rkidx++;
    aria_gsrk(ctx.enc_key[rkidx], w1, w2, 19);
    rkidx++;
    aria_gsrk(ctx.enc_key[rkidx], w2, w3, 19);
    rkidx++;
    aria_gsrk(ctx.enc_key[rkidx], w3, w0, 19);
    rkidx++;
    aria_gsrk(ctx.enc_key[rkidx], w0, w1, 31);
    rkidx++;
    aria_gsrk(ctx.enc_key[rkidx], w1, w2, 31);
    rkidx++;
    aria_gsrk(ctx.enc_key[rkidx], w2, w3, 31);
    rkidx++;
    aria_gsrk(ctx.enc_key[rkidx], w3, w0, 31);
    rkidx++;
    aria_gsrk(ctx.enc_key[rkidx], w0, w1, 67);
    rkidx++;
    aria_gsrk(ctx.enc_key[rkidx], w1, w2, 67);
    rkidx++;
    aria_gsrk(ctx.enc_key[rkidx], w2, w3, 67);
    rkidx++;
    aria_gsrk(ctx.enc_key[rkidx], w3, w0, 67);
    rkidx++;
    aria_gsrk(ctx.enc_key[rkidx], w0, w1, 97);
    if (key_len > 16) {
    rkidx++;
    aria_gsrk(ctx.enc_key[rkidx], w1, w2, 97);
    rkidx++;
    aria_gsrk(ctx.enc_key[rkidx], w2, w3, 97);
    if (key_len > 24) {
    rkidx++;
    aria_gsrk(ctx.enc_key[rkidx], w3, w0, 97);
    rkidx++;
    aria_gsrk(ctx.enc_key[rkidx], w0, w1, 109);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn aria_set_decrypt_key(ctx: *mut aria_ctx) {
    static void aria_set_decrypt_key(struct aria_ctx *ctx)
    {
    int i;
    for (i = 0; i < 4; i++) {
    ctx.dec_key[0][i] = ctx.enc_key[ctx.rounds][i];
    ctx.dec_key[ctx.rounds][i] = ctx.enc_key[0][i];
    }
    for (i = 1; i < ctx.rounds; i++) {
    ctx.dec_key[i][0] = aria_m(ctx.enc_key[ctx.rounds - i][0]);
    ctx.dec_key[i][1] = aria_m(ctx.enc_key[ctx.rounds - i][1]);
    ctx.dec_key[i][2] = aria_m(ctx.enc_key[ctx.rounds - i][2]);
    ctx.dec_key[i][3] = aria_m(ctx.enc_key[ctx.rounds - i][3]);
    aria_diff_word(&ctx.dec_key[i][0], &ctx.dec_key[i][1],
    &ctx.dec_key[i][2], &ctx.dec_key[i][3]);
    aria_diff_byte(&ctx.dec_key[i][1],
    &ctx.dec_key[i][2], &ctx.dec_key[i][3]);
    aria_diff_word(&ctx.dec_key[i][0], &ctx.dec_key[i][1],
    &ctx.dec_key[i][2], &ctx.dec_key[i][3]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn aria_set_key(tfm: *mut crypto_tfm, in_key: *const u8, key_len: c_uint) -> c_int {
    int aria_set_key(struct crypto_tfm *tfm, const u8 *in_key, unsigned int key_len)
    {
    struct aria_ctx *ctx = crypto_tfm_ctx(tfm);
    if (key_len != 16 && key_len != 24 && key_len != 32)
    return -EINVAL;
    BUILD_BUG_ON(sizeof(ctx.enc_key) != 272);
    BUILD_BUG_ON(sizeof(ctx.dec_key) != 272);
    BUILD_BUG_ON(sizeof(int) != sizeof(ctx.rounds));
    ctx.key_length = key_len;
    ctx.rounds = (key_len + 32) / 4;
    aria_set_encrypt_key(ctx, in_key, key_len);
    aria_set_decrypt_key(ctx);
    return 0;
    }
    EXPORT_SYMBOL_GPL(aria_set_key);
    static void __aria_crypt(struct aria_ctx *ctx, u8 *out, const u8 *in,
    u32 key[][ARIA_RD_KEY_WORDS])
    {
    u32 reg0, reg1, reg2, reg3;
    int rounds, rkidx = 0;
    rounds = ctx.rounds;
    reg0 = get_unaligned_be32(&in[0]);
    reg1 = get_unaligned_be32(&in[4]);
    reg2 = get_unaligned_be32(&in[8]);
    reg3 = get_unaligned_be32(&in[12]);
    aria_add_round_key(key[rkidx], &reg0, &reg1, &reg2, &reg3);
    rkidx++;
    aria_subst_diff_odd(&reg0, &reg1, &reg2, &reg3);
    aria_add_round_key(key[rkidx], &reg0, &reg1, &reg2, &reg3);
    rkidx++;
    while ((rounds -= 2) > 0) {
    aria_subst_diff_even(&reg0, &reg1, &reg2, &reg3);
    aria_add_round_key(key[rkidx], &reg0, &reg1, &reg2, &reg3);
    rkidx++;
    aria_subst_diff_odd(&reg0, &reg1, &reg2, &reg3);
    aria_add_round_key(key[rkidx], &reg0, &reg1, &reg2, &reg3);
    rkidx++;
    }
    reg0 = key[rkidx][0] ^ make_u32((u8)(x1[get_u8(reg0, 0)]),
    (u8)(x2[get_u8(reg0, 1)] >> 8),
    (u8)(s1[get_u8(reg0, 2)]),
    (u8)(s2[get_u8(reg0, 3)]));
    reg1 = key[rkidx][1] ^ make_u32((u8)(x1[get_u8(reg1, 0)]),
    (u8)(x2[get_u8(reg1, 1)] >> 8),
    (u8)(s1[get_u8(reg1, 2)]),
    (u8)(s2[get_u8(reg1, 3)]));
    reg2 = key[rkidx][2] ^ make_u32((u8)(x1[get_u8(reg2, 0)]),
    (u8)(x2[get_u8(reg2, 1)] >> 8),
    (u8)(s1[get_u8(reg2, 2)]),
    (u8)(s2[get_u8(reg2, 3)]));
    reg3 = key[rkidx][3] ^ make_u32((u8)(x1[get_u8(reg3, 0)]),
    (u8)(x2[get_u8(reg3, 1)] >> 8),
    (u8)(s1[get_u8(reg3, 2)]),
    (u8)(s2[get_u8(reg3, 3)]));
    put_unaligned_be32(reg0, &out[0]);
    put_unaligned_be32(reg1, &out[4]);
    put_unaligned_be32(reg2, &out[8]);
    put_unaligned_be32(reg3, &out[12]);
    }
#[no_mangle]
pub unsafe extern "C" fn aria_encrypt(_ctx: *mut c_void, out: *mut u8, in: *const u8) {
    void aria_encrypt(void *_ctx, u8 *out, const u8 *in)
    {
    struct aria_ctx *ctx = (struct aria_ctx *)_ctx;
    __aria_crypt(ctx, out, in, ctx.enc_key);
    }
    EXPORT_SYMBOL_GPL(aria_encrypt);
#[no_mangle]
pub unsafe extern "C" fn aria_decrypt(_ctx: *mut c_void, out: *mut u8, in: *const u8) {
    void aria_decrypt(void *_ctx, u8 *out, const u8 *in)
    {
    struct aria_ctx *ctx = (struct aria_ctx *)_ctx;
    __aria_crypt(ctx, out, in, ctx.dec_key);
    }
    EXPORT_SYMBOL_GPL(aria_decrypt);
#[no_mangle]
unsafe extern "C" fn __aria_encrypt(tfm: *mut crypto_tfm, out: *mut u8, in: *const u8) {
    static void __aria_encrypt(struct crypto_tfm *tfm, u8 *out, const u8 *in)
    {
    struct aria_ctx *ctx = crypto_tfm_ctx(tfm);
    __aria_crypt(ctx, out, in, ctx.enc_key);
    }
#[no_mangle]
unsafe extern "C" fn __aria_decrypt(tfm: *mut crypto_tfm, out: *mut u8, in: *const u8) {
    static void __aria_decrypt(struct crypto_tfm *tfm, u8 *out, const u8 *in)
    {
    struct aria_ctx *ctx = crypto_tfm_ctx(tfm);
    __aria_crypt(ctx, out, in, ctx.dec_key);
    }
    static struct crypto_alg aria_alg = {
    .cra_name		=	"aria",
    .cra_driver_name	=	"aria-generic",
    .cra_priority		=	100,
    .cra_flags		=	CRYPTO_ALG_TYPE_CIPHER,
    .cra_blocksize		=	ARIA_BLOCK_SIZE,
    .cra_ctxsize		=	sizeof(struct aria_ctx),
    .cra_module		=	THIS_MODULE,
    .cra_u			=	{
    .cipher = {
    .cia_min_keysize	=	ARIA_MIN_KEY_SIZE,
    .cia_max_keysize	=	ARIA_MAX_KEY_SIZE,
    .cia_setkey		=	aria_set_key,
    .cia_encrypt		=	__aria_encrypt,
    .cia_decrypt		=	__aria_decrypt
    }
    }
    };
#[no_mangle]
unsafe extern "C" fn aria_init() -> int __init {
    static int __init aria_init(void)
    {
    return crypto_register_alg(&aria_alg);
    }
#[no_mangle]
unsafe extern "C" fn aria_fini() -> void __exit {
    static void __exit aria_fini(void)
    {
    crypto_unregister_alg(&aria_alg);
    }
    module_init(aria_init);
    module_exit(aria_fini);
    MODULE_DESCRIPTION("ARIA Cipher Algorithm");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Taehee Yoo <ap420073@gmail.com>");
    MODULE_ALIAS_CRYPTO("aria");
    MODULE_ALIAS_CRYPTO("aria-generic");
