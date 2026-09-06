//! Automatically rewritten from C to Rust
//! Source: arch/arm64/crypto/sm4-ce-cipher-glue.c
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


// SPDX-License-Identifier: GPL-2.0

    MODULE_ALIAS_CRYPTO("sm4");
    MODULE_ALIAS_CRYPTO("sm4-ce");
    MODULE_DESCRIPTION("SM4 symmetric cipher using ARMv8 Crypto Extensions");
    MODULE_AUTHOR("Ard Biesheuvel <ard.biesheuvel@linaro.org>");
    MODULE_LICENSE("GPL v2");
    asmlinkage void sm4_ce_do_crypt(const u32 *rk, void *out, const void *in);
    static int sm4_ce_setkey(struct crypto_tfm *tfm, const u8 *key,
    unsigned int key_len)
    {
    struct sm4_ctx *ctx = crypto_tfm_ctx(tfm);
    return sm4_expandkey(ctx, key, key_len);
    }
#[no_mangle]
unsafe extern "C" fn sm4_ce_encrypt(tfm: *mut crypto_tfm, out: *mut u8, in: *const u8) {
    static void sm4_ce_encrypt(struct crypto_tfm *tfm, u8 *out, const u8 *in)
    {
    const struct sm4_ctx *ctx = crypto_tfm_ctx(tfm);
    if (!crypto_simd_usable()) {
    sm4_crypt_block(ctx.rkey_enc, out, in);
    } else {
    scoped_ksimd()
    sm4_ce_do_crypt(ctx.rkey_enc, out, in);
    }
    }
#[no_mangle]
unsafe extern "C" fn sm4_ce_decrypt(tfm: *mut crypto_tfm, out: *mut u8, in: *const u8) {
    static void sm4_ce_decrypt(struct crypto_tfm *tfm, u8 *out, const u8 *in)
    {
    const struct sm4_ctx *ctx = crypto_tfm_ctx(tfm);
    if (!crypto_simd_usable()) {
    sm4_crypt_block(ctx.rkey_dec, out, in);
    } else {
    scoped_ksimd()
    sm4_ce_do_crypt(ctx.rkey_dec, out, in);
    }
    }
    static struct crypto_alg sm4_ce_alg = {
    .cra_name			= "sm4",
    .cra_driver_name		= "sm4-ce",
    .cra_priority			= 300,
    .cra_flags			= CRYPTO_ALG_TYPE_CIPHER,
    .cra_blocksize			= SM4_BLOCK_SIZE,
    .cra_ctxsize			= sizeof(struct sm4_ctx),
    .cra_module			= THIS_MODULE,
    .cra_u.cipher = {
    .cia_min_keysize	= SM4_KEY_SIZE,
    .cia_max_keysize	= SM4_KEY_SIZE,
    .cia_setkey		= sm4_ce_setkey,
    .cia_encrypt		= sm4_ce_encrypt,
    .cia_decrypt		= sm4_ce_decrypt
    }
    };
#[no_mangle]
unsafe extern "C" fn sm4_ce_mod_init() -> int __init {
    static int __init sm4_ce_mod_init(void)
    {
    return crypto_register_alg(&sm4_ce_alg);
    }
#[no_mangle]
unsafe extern "C" fn sm4_ce_mod_fini() -> void __exit {
    static void __exit sm4_ce_mod_fini(void)
    {
    crypto_unregister_alg(&sm4_ce_alg);
    }
    module_cpu_feature_match(SM4, sm4_ce_mod_init);
    module_exit(sm4_ce_mod_fini);
