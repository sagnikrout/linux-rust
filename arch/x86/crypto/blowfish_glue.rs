//! Automatically rewritten from C to Rust
//! Source: arch/x86/crypto/blowfish_glue.c
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
// Glue Code for assembler optimized version of Blowfish
//
// Copyright (c) 2011 Jussi Kivilinna <jussi.kivilinna@mbnet.fi>
//
// CBC & ECB parts based on code (crypto/cbc.c,ecb.c) by:
// Copyright (c) 2006 Herbert Xu <herbert@gondor.apana.org.au>
//

// regular block cipher functions
    asmlinkage void blowfish_enc_blk(struct bf_ctx *ctx, u8 *dst, const u8 *src);
    asmlinkage void blowfish_dec_blk(struct bf_ctx *ctx, u8 *dst, const u8 *src);
// 4-way parallel cipher functions
    asmlinkage void blowfish_enc_blk_4way(struct bf_ctx *ctx, u8 *dst,
    const u8 *src);
    asmlinkage void __blowfish_dec_blk_4way(struct bf_ctx *ctx, u8 *dst,
    const u8 *src, bool cbc);
    static inline void blowfish_dec_ecb_4way(struct bf_ctx *ctx, u8 *dst,
    const u8 *src)
    {
    return __blowfish_dec_blk_4way(ctx, dst, src, false);
    }
    static inline void blowfish_dec_cbc_4way(struct bf_ctx *ctx, u8 *dst,
    const u8 *src)
    {
    return __blowfish_dec_blk_4way(ctx, dst, src, true);
    }
#[no_mangle]
unsafe extern "C" fn blowfish_encrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    static void blowfish_encrypt(struct crypto_tfm *tfm, u8 *dst, const u8 *src)
    {
    blowfish_enc_blk(crypto_tfm_ctx(tfm), dst, src);
    }
#[no_mangle]
unsafe extern "C" fn blowfish_decrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    static void blowfish_decrypt(struct crypto_tfm *tfm, u8 *dst, const u8 *src)
    {
    blowfish_dec_blk(crypto_tfm_ctx(tfm), dst, src);
    }
    static int blowfish_setkey_skcipher(struct crypto_skcipher *tfm,
    const u8 *key, unsigned int keylen)
    {
    return blowfish_setkey(&tfm.base, key, keylen);
    }
#[no_mangle]
unsafe extern "C" fn ecb_encrypt(req: *mut skcipher_request) -> c_int {
    static int ecb_encrypt(struct skcipher_request *req)
    {
    ECB_WALK_START(req, BF_BLOCK_SIZE, -1);
    ECB_BLOCK(4, blowfish_enc_blk_4way);
    ECB_BLOCK(1, blowfish_enc_blk);
    ECB_WALK_END();
    }
#[no_mangle]
unsafe extern "C" fn ecb_decrypt(req: *mut skcipher_request) -> c_int {
    static int ecb_decrypt(struct skcipher_request *req)
    {
    ECB_WALK_START(req, BF_BLOCK_SIZE, -1);
    ECB_BLOCK(4, blowfish_dec_ecb_4way);
    ECB_BLOCK(1, blowfish_dec_blk);
    ECB_WALK_END();
    }
#[no_mangle]
unsafe extern "C" fn cbc_encrypt(req: *mut skcipher_request) -> c_int {
    static int cbc_encrypt(struct skcipher_request *req)
    {
    CBC_WALK_START(req, BF_BLOCK_SIZE, -1);
    CBC_ENC_BLOCK(blowfish_enc_blk);
    CBC_WALK_END();
    }
#[no_mangle]
unsafe extern "C" fn cbc_decrypt(req: *mut skcipher_request) -> c_int {
    static int cbc_decrypt(struct skcipher_request *req)
    {
    CBC_WALK_START(req, BF_BLOCK_SIZE, -1);
    CBC_DEC_BLOCK(4, blowfish_dec_cbc_4way);
    CBC_DEC_BLOCK(1, blowfish_dec_blk);
    CBC_WALK_END();
    }
    static struct crypto_alg bf_cipher_alg = {
    .cra_name		= "blowfish",
    .cra_driver_name	= "blowfish-asm",
    .cra_priority		= 200,
    .cra_flags		= CRYPTO_ALG_TYPE_CIPHER,
    .cra_blocksize		= BF_BLOCK_SIZE,
    .cra_ctxsize		= sizeof(struct bf_ctx),
    .cra_module		= THIS_MODULE,
    .cra_u = {
    .cipher = {
    .cia_min_keysize	= BF_MIN_KEY_SIZE,
    .cia_max_keysize	= BF_MAX_KEY_SIZE,
    .cia_setkey		= blowfish_setkey,
    .cia_encrypt		= blowfish_encrypt,
    .cia_decrypt		= blowfish_decrypt,
    }
    }
    };
    static struct skcipher_alg bf_skcipher_algs[] = {
    {
    .base.cra_name		= "ecb(blowfish)",
    .base.cra_driver_name	= "ecb-blowfish-asm",
    .base.cra_priority	= 300,
    .base.cra_blocksize	= BF_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct bf_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= BF_MIN_KEY_SIZE,
    .max_keysize		= BF_MAX_KEY_SIZE,
    .setkey			= blowfish_setkey_skcipher,
    .encrypt		= ecb_encrypt,
    .decrypt		= ecb_decrypt,
    }, {
    .base.cra_name		= "cbc(blowfish)",
    .base.cra_driver_name	= "cbc-blowfish-asm",
    .base.cra_priority	= 300,
    .base.cra_blocksize	= BF_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct bf_ctx),
    .base.cra_module	= THIS_MODULE,
    .min_keysize		= BF_MIN_KEY_SIZE,
    .max_keysize		= BF_MAX_KEY_SIZE,
    .ivsize			= BF_BLOCK_SIZE,
    .setkey			= blowfish_setkey_skcipher,
    .encrypt		= cbc_encrypt,
    .decrypt		= cbc_decrypt,
    },
    };
#[no_mangle]
unsafe extern "C" fn is_blacklisted_cpu() -> bool {
    static bool is_blacklisted_cpu(void)
    {
    if (boot_cpu_data.x86_vendor != X86_VENDOR_INTEL)
    return false;
    if (boot_cpu_data.x86 == 0x0f) {
//
// On Pentium 4, blowfish-x86_64 is slower than generic C
// implementation because use of 64bit rotates (which are really
// slow on P4). Therefore blacklist P4s.
//
    return true;
    }
    return false;
    }
    static int force;
    module_param(force, int, 0);
    MODULE_PARM_DESC(force, "Force module load, ignore CPU blacklist");
#[no_mangle]
unsafe extern "C" fn blowfish_init() -> int __init {
    static int __init blowfish_init(void)
    {
    int err;
    if (!force && is_blacklisted_cpu()) {
    printk(KERN_INFO
    "blowfish-x86_64: performance on this CPU "
    "would be suboptimal: disabling "
    "blowfish-x86_64.\n");
    return -ENODEV;
    }
    err = crypto_register_alg(&bf_cipher_alg);
    if (err)
    return err;
    err = crypto_register_skciphers(bf_skcipher_algs,
    ARRAY_SIZE(bf_skcipher_algs));
    if (err)
    crypto_unregister_alg(&bf_cipher_alg);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn blowfish_fini() -> void __exit {
    static void __exit blowfish_fini(void)
    {
    crypto_unregister_alg(&bf_cipher_alg);
    crypto_unregister_skciphers(bf_skcipher_algs,
    ARRAY_SIZE(bf_skcipher_algs));
    }
    module_init(blowfish_init);
    module_exit(blowfish_fini);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Blowfish Cipher Algorithm, asm optimized");
    MODULE_ALIAS_CRYPTO("blowfish");
    MODULE_ALIAS_CRYPTO("blowfish-asm");
