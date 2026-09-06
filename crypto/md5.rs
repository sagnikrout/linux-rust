//! Automatically rewritten from C to Rust
//! Source: crypto/md5.c
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
// Crypto API support for MD5 and HMAC-MD5
//
// Copyright 2025 Google LLC
//

//
// Export and import functions.  crypto_shash wants a particular format that
// matches that used by some legacy drivers.  It currently is the same as the
// library MD5 context, except the value in bytecount must be block-aligned and
// the remainder must be stored in an extra u8 appended to the struct.
//

    static_assert(sizeof(struct md5_ctx) == sizeof(struct md5_state));
    static_assert(offsetof(struct md5_ctx, state) == offsetof(struct md5_state, hash));
    static_assert(offsetof(struct md5_ctx, bytecount) == offsetof(struct md5_state, byte_count));
    static_assert(offsetof(struct md5_ctx, buf) == offsetof(struct md5_state, block));
#[no_mangle]
unsafe extern "C" fn __crypto_md5_export(ctx0: *const md5_ctx, out: *mut c_void) -> c_int {
    static int __crypto_md5_export(const struct md5_ctx *ctx0, void *out)
    {
    let mut ctx: md5_ctx = *ctx0;
    unsigned int partial;
    u8 *p = out;
    partial = ctx.bytecount % MD5_BLOCK_SIZE;
    ctx.bytecount -= partial;
    memcpy(p, &ctx, sizeof(ctx));
    p += sizeof(ctx);
// p = partial;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __crypto_md5_import(ctx: *mut md5_ctx, in: *const c_void) -> c_int {
    static int __crypto_md5_import(struct md5_ctx *ctx, const void *in)
    {
    const u8 *p = in;
    memcpy(ctx, p, sizeof(*ctx));
    p += sizeof(*ctx);
    ctx.bytecount += *p;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __crypto_md5_export_core(ctx: *const md5_ctx, out: *mut c_void) -> c_int {
    static int __crypto_md5_export_core(const struct md5_ctx *ctx, void *out)
    {
    memcpy(out, ctx, offsetof(struct md5_ctx, buf));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __crypto_md5_import_core(ctx: *mut md5_ctx, in: *const c_void) -> c_int {
    static int __crypto_md5_import_core(struct md5_ctx *ctx, const void *in)
    {
    memcpy(ctx, in, offsetof(struct md5_ctx, buf));
    return 0;
    }
    const u8 md5_zero_message_hash[MD5_DIGEST_SIZE] = {
    0xd4, 0x1d, 0x8c, 0xd9, 0x8f, 0x00, 0xb2, 0x04,
    0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8, 0x42, 0x7e,
    };
    EXPORT_SYMBOL_GPL(md5_zero_message_hash);

#[no_mangle]
unsafe extern "C" fn crypto_md5_init(desc: *mut shash_desc) -> c_int {
    static int crypto_md5_init(struct shash_desc *desc)
    {
    md5_init(MD5_CTX(desc));
    return 0;
    }
    static int crypto_md5_update(struct shash_desc *desc,
    const u8 *data, unsigned int len)
    {
    md5_update(MD5_CTX(desc), data, len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_md5_final(desc: *mut shash_desc, out: *mut u8) -> c_int {
    static int crypto_md5_final(struct shash_desc *desc, u8 *out)
    {
    md5_final(MD5_CTX(desc), out);
    return 0;
    }
    static int crypto_md5_digest(struct shash_desc *desc,
    const u8 *data, unsigned int len, u8 *out)
    {
    md5(data, len, out);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_md5_export(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int crypto_md5_export(struct shash_desc *desc, void *out)
    {
    return __crypto_md5_export(MD5_CTX(desc), out);
    }
#[no_mangle]
unsafe extern "C" fn crypto_md5_import(desc: *mut shash_desc, in: *const c_void) -> c_int {
    static int crypto_md5_import(struct shash_desc *desc, const void *in)
    {
    return __crypto_md5_import(MD5_CTX(desc), in);
    }
#[no_mangle]
unsafe extern "C" fn crypto_md5_export_core(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int crypto_md5_export_core(struct shash_desc *desc, void *out)
    {
    return __crypto_md5_export_core(MD5_CTX(desc), out);
    }
#[no_mangle]
unsafe extern "C" fn crypto_md5_import_core(desc: *mut shash_desc, in: *const c_void) -> c_int {
    static int crypto_md5_import_core(struct shash_desc *desc, const void *in)
    {
    return __crypto_md5_import_core(MD5_CTX(desc), in);
    }

    static int crypto_hmac_md5_setkey(struct crypto_shash *tfm,
    const u8 *raw_key, unsigned int keylen)
    {
    hmac_md5_preparekey(HMAC_MD5_KEY(tfm), raw_key, keylen);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_hmac_md5_init(desc: *mut shash_desc) -> c_int {
    static int crypto_hmac_md5_init(struct shash_desc *desc)
    {
    hmac_md5_init(HMAC_MD5_CTX(desc), HMAC_MD5_KEY(desc.tfm));
    return 0;
    }
    static int crypto_hmac_md5_update(struct shash_desc *desc,
    const u8 *data, unsigned int len)
    {
    hmac_md5_update(HMAC_MD5_CTX(desc), data, len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_hmac_md5_final(desc: *mut shash_desc, out: *mut u8) -> c_int {
    static int crypto_hmac_md5_final(struct shash_desc *desc, u8 *out)
    {
    hmac_md5_final(HMAC_MD5_CTX(desc), out);
    return 0;
    }
    static int crypto_hmac_md5_digest(struct shash_desc *desc,
    const u8 *data, unsigned int len, u8 *out)
    {
    hmac_md5(HMAC_MD5_KEY(desc.tfm), data, len, out);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_hmac_md5_export(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int crypto_hmac_md5_export(struct shash_desc *desc, void *out)
    {
    return __crypto_md5_export(&HMAC_MD5_CTX(desc).hash_ctx, out);
    }
#[no_mangle]
unsafe extern "C" fn crypto_hmac_md5_import(desc: *mut shash_desc, in: *const c_void) -> c_int {
    static int crypto_hmac_md5_import(struct shash_desc *desc, const void *in)
    {
    struct hmac_md5_ctx *ctx = HMAC_MD5_CTX(desc);
    ctx.ostate = HMAC_MD5_KEY(desc.tfm).ostate;
    return __crypto_md5_import(&ctx.hash_ctx, in);
    }
#[no_mangle]
unsafe extern "C" fn crypto_hmac_md5_export_core(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int crypto_hmac_md5_export_core(struct shash_desc *desc, void *out)
    {
    return __crypto_md5_export_core(&HMAC_MD5_CTX(desc).hash_ctx, out);
    }
#[no_mangle]
unsafe extern "C" fn crypto_hmac_md5_import_core(desc: *mut shash_desc, in: *const c_void) -> c_int {
    static int crypto_hmac_md5_import_core(struct shash_desc *desc, const void *in)
    {
    struct hmac_md5_ctx *ctx = HMAC_MD5_CTX(desc);
    ctx.ostate = HMAC_MD5_KEY(desc.tfm).ostate;
    return __crypto_md5_import_core(&ctx.hash_ctx, in);
    }
    static struct shash_alg algs[] = {
    {
    .base.cra_name		= "md5",
    .base.cra_driver_name	= "md5-lib",
    .base.cra_priority	= 300,
    .base.cra_blocksize	= MD5_BLOCK_SIZE,
    .base.cra_module	= THIS_MODULE,
    .digestsize		= MD5_DIGEST_SIZE,
    .init			= crypto_md5_init,
    .update			= crypto_md5_update,
    .final			= crypto_md5_final,
    .digest			= crypto_md5_digest,
    .export			= crypto_md5_export,
    .import			= crypto_md5_import,
    .export_core		= crypto_md5_export_core,
    .import_core		= crypto_md5_import_core,
    .descsize		= sizeof(struct md5_ctx),
    .statesize		= MD5_SHASH_STATE_SIZE,
    },
    {
    .base.cra_name		= "hmac(md5)",
    .base.cra_driver_name	= "hmac-md5-lib",
    .base.cra_priority	= 300,
    .base.cra_blocksize	= MD5_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct hmac_md5_key),
    .base.cra_module	= THIS_MODULE,
    .digestsize		= MD5_DIGEST_SIZE,
    .setkey			= crypto_hmac_md5_setkey,
    .init			= crypto_hmac_md5_init,
    .update			= crypto_hmac_md5_update,
    .final			= crypto_hmac_md5_final,
    .digest			= crypto_hmac_md5_digest,
    .export			= crypto_hmac_md5_export,
    .import			= crypto_hmac_md5_import,
    .export_core		= crypto_hmac_md5_export_core,
    .import_core		= crypto_hmac_md5_import_core,
    .descsize		= sizeof(struct hmac_md5_ctx),
    .statesize		= MD5_SHASH_STATE_SIZE,
    },
    };
#[no_mangle]
unsafe extern "C" fn crypto_md5_mod_init() -> int __init {
    static int __init crypto_md5_mod_init(void)
    {
    return crypto_register_shashes(algs, ARRAY_SIZE(algs));
    }
    module_init(crypto_md5_mod_init);
#[no_mangle]
unsafe extern "C" fn crypto_md5_mod_exit() -> void __exit {
    static void __exit crypto_md5_mod_exit(void)
    {
    crypto_unregister_shashes(algs, ARRAY_SIZE(algs));
    }
    module_exit(crypto_md5_mod_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Crypto API support for MD5 and HMAC-MD5");
    MODULE_ALIAS_CRYPTO("md5");
    MODULE_ALIAS_CRYPTO("md5-lib");
    MODULE_ALIAS_CRYPTO("hmac(md5)");
    MODULE_ALIAS_CRYPTO("hmac-md5-lib");
