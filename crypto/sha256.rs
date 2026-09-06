//! Automatically rewritten from C to Rust
//! Source: crypto/sha256.c
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
// Crypto API support for SHA-224, SHA-256, HMAC-SHA224, and HMAC-SHA256
//
// Copyright (c) Jean-Luc Cooke <jlcooke@certainkey.com>
// Copyright (c) Andrew McDonald <andrew@mcdonald.org.uk>
// Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
// SHA224 Support Copyright 2007 Intel Corporation <jonathan.lynch@intel.com>
// Copyright 2025 Google LLC
//

//
// Export and import functions.  crypto_shash wants a particular format that
// matches that used by some legacy drivers.  It currently is the same as the
// library SHA context, except the value in bytecount must be block-aligned and
// the remainder must be stored in an extra u8 appended to the struct.
//
pub const SHA256_SHASH_STATE_SIZE: c_int = 105;
    static_assert(offsetof(struct __sha256_ctx, state) == 0);
    static_assert(offsetof(struct __sha256_ctx, bytecount) == 32);
    static_assert(offsetof(struct __sha256_ctx, buf) == 40);
    static_assert(sizeof(struct __sha256_ctx) + 1 == SHA256_SHASH_STATE_SIZE);
#[no_mangle]
unsafe extern "C" fn __crypto_sha256_export(ctx0: *const __sha256_ctx, out: *mut c_void) -> c_int {
    static int __crypto_sha256_export(const struct __sha256_ctx *ctx0, void *out)
    {
    let mut ctx: __sha256_ctx = *ctx0;
    unsigned int partial;
    u8 *p = out;
    partial = ctx.bytecount % SHA256_BLOCK_SIZE;
    ctx.bytecount -= partial;
    memcpy(p, &ctx, sizeof(ctx));
    p += sizeof(ctx);
// p = partial;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __crypto_sha256_import(ctx: *mut __sha256_ctx, in: *const c_void) -> c_int {
    static int __crypto_sha256_import(struct __sha256_ctx *ctx, const void *in)
    {
    const u8 *p = in;
    memcpy(ctx, p, sizeof(*ctx));
    p += sizeof(*ctx);
    ctx.bytecount += *p;
    return 0;
    }
    static int __crypto_sha256_export_core(const struct __sha256_ctx *ctx,
    void *out)
    {
    memcpy(out, ctx, offsetof(struct __sha256_ctx, buf));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __crypto_sha256_import_core(ctx: *mut __sha256_ctx, in: *const c_void) -> c_int {
    static int __crypto_sha256_import_core(struct __sha256_ctx *ctx, const void *in)
    {
    memcpy(ctx, in, offsetof(struct __sha256_ctx, buf));
    return 0;
    }
// SHA-224
    const u8 sha224_zero_message_hash[SHA224_DIGEST_SIZE] = {
    0xd1, 0x4a, 0x02, 0x8c, 0x2a, 0x3a, 0x2b, 0xc9, 0x47,
    0x61, 0x02, 0xbb, 0x28, 0x82, 0x34, 0xc4, 0x15, 0xa2,
    0xb0, 0x1f, 0x82, 0x8e, 0xa6, 0x2a, 0xc5, 0xb3, 0xe4,
    0x2f
    };
    EXPORT_SYMBOL_GPL(sha224_zero_message_hash);

#[no_mangle]
unsafe extern "C" fn crypto_sha224_init(desc: *mut shash_desc) -> c_int {
    static int crypto_sha224_init(struct shash_desc *desc)
    {
    sha224_init(SHA224_CTX(desc));
    return 0;
    }
    static int crypto_sha224_update(struct shash_desc *desc,
    const u8 *data, unsigned int len)
    {
    sha224_update(SHA224_CTX(desc), data, len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_sha224_final(desc: *mut shash_desc, out: *mut u8) -> c_int {
    static int crypto_sha224_final(struct shash_desc *desc, u8 *out)
    {
    sha224_final(SHA224_CTX(desc), out);
    return 0;
    }
    static int crypto_sha224_digest(struct shash_desc *desc,
    const u8 *data, unsigned int len, u8 *out)
    {
    sha224(data, len, out);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_sha224_export(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int crypto_sha224_export(struct shash_desc *desc, void *out)
    {
    return __crypto_sha256_export(&SHA224_CTX(desc).ctx, out);
    }
#[no_mangle]
unsafe extern "C" fn crypto_sha224_import(desc: *mut shash_desc, in: *const c_void) -> c_int {
    static int crypto_sha224_import(struct shash_desc *desc, const void *in)
    {
    return __crypto_sha256_import(&SHA224_CTX(desc).ctx, in);
    }
#[no_mangle]
unsafe extern "C" fn crypto_sha224_export_core(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int crypto_sha224_export_core(struct shash_desc *desc, void *out)
    {
    return __crypto_sha256_export_core(&SHA224_CTX(desc).ctx, out);
    }
#[no_mangle]
unsafe extern "C" fn crypto_sha224_import_core(desc: *mut shash_desc, in: *const c_void) -> c_int {
    static int crypto_sha224_import_core(struct shash_desc *desc, const void *in)
    {
    return __crypto_sha256_import_core(&SHA224_CTX(desc).ctx, in);
    }
// SHA-256
    const u8 sha256_zero_message_hash[SHA256_DIGEST_SIZE] = {
    0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14,
    0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9, 0x24,
    0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c,
    0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55
    };
    EXPORT_SYMBOL_GPL(sha256_zero_message_hash);

#[no_mangle]
unsafe extern "C" fn crypto_sha256_init(desc: *mut shash_desc) -> c_int {
    static int crypto_sha256_init(struct shash_desc *desc)
    {
    sha256_init(SHA256_CTX(desc));
    return 0;
    }
    static int crypto_sha256_update(struct shash_desc *desc,
    const u8 *data, unsigned int len)
    {
    sha256_update(SHA256_CTX(desc), data, len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_sha256_final(desc: *mut shash_desc, out: *mut u8) -> c_int {
    static int crypto_sha256_final(struct shash_desc *desc, u8 *out)
    {
    sha256_final(SHA256_CTX(desc), out);
    return 0;
    }
    static int crypto_sha256_digest(struct shash_desc *desc,
    const u8 *data, unsigned int len, u8 *out)
    {
    sha256(data, len, out);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_sha256_export(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int crypto_sha256_export(struct shash_desc *desc, void *out)
    {
    return __crypto_sha256_export(&SHA256_CTX(desc).ctx, out);
    }
#[no_mangle]
unsafe extern "C" fn crypto_sha256_import(desc: *mut shash_desc, in: *const c_void) -> c_int {
    static int crypto_sha256_import(struct shash_desc *desc, const void *in)
    {
    return __crypto_sha256_import(&SHA256_CTX(desc).ctx, in);
    }
#[no_mangle]
unsafe extern "C" fn crypto_sha256_export_core(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int crypto_sha256_export_core(struct shash_desc *desc, void *out)
    {
    return __crypto_sha256_export_core(&SHA256_CTX(desc).ctx, out);
    }
#[no_mangle]
unsafe extern "C" fn crypto_sha256_import_core(desc: *mut shash_desc, in: *const c_void) -> c_int {
    static int crypto_sha256_import_core(struct shash_desc *desc, const void *in)
    {
    return __crypto_sha256_import_core(&SHA256_CTX(desc).ctx, in);
    }
// HMAC-SHA224

    static int crypto_hmac_sha224_setkey(struct crypto_shash *tfm,
    const u8 *raw_key, unsigned int keylen)
    {
    hmac_sha224_preparekey(HMAC_SHA224_KEY(tfm), raw_key, keylen);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_hmac_sha224_init(desc: *mut shash_desc) -> c_int {
    static int crypto_hmac_sha224_init(struct shash_desc *desc)
    {
    hmac_sha224_init(HMAC_SHA224_CTX(desc), HMAC_SHA224_KEY(desc.tfm));
    return 0;
    }
    static int crypto_hmac_sha224_update(struct shash_desc *desc,
    const u8 *data, unsigned int len)
    {
    hmac_sha224_update(HMAC_SHA224_CTX(desc), data, len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_hmac_sha224_final(desc: *mut shash_desc, out: *mut u8) -> c_int {
    static int crypto_hmac_sha224_final(struct shash_desc *desc, u8 *out)
    {
    hmac_sha224_final(HMAC_SHA224_CTX(desc), out);
    return 0;
    }
    static int crypto_hmac_sha224_digest(struct shash_desc *desc,
    const u8 *data, unsigned int len,
    u8 *out)
    {
    hmac_sha224(HMAC_SHA224_KEY(desc.tfm), data, len, out);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_hmac_sha224_export(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int crypto_hmac_sha224_export(struct shash_desc *desc, void *out)
    {
    return __crypto_sha256_export(&HMAC_SHA224_CTX(desc).ctx.sha_ctx, out);
    }
#[no_mangle]
unsafe extern "C" fn crypto_hmac_sha224_import(desc: *mut shash_desc, in: *const c_void) -> c_int {
    static int crypto_hmac_sha224_import(struct shash_desc *desc, const void *in)
    {
    struct hmac_sha224_ctx *ctx = HMAC_SHA224_CTX(desc);
    ctx.ctx.ostate = HMAC_SHA224_KEY(desc.tfm).key.ostate;
    return __crypto_sha256_import(&ctx.ctx.sha_ctx, in);
    }
#[no_mangle]
unsafe extern "C" fn crypto_hmac_sha224_export_core(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int crypto_hmac_sha224_export_core(struct shash_desc *desc, void *out)
    {
    return __crypto_sha256_export_core(&HMAC_SHA224_CTX(desc).ctx.sha_ctx,
    out);
    }
    static int crypto_hmac_sha224_import_core(struct shash_desc *desc,
    const void *in)
    {
    struct hmac_sha224_ctx *ctx = HMAC_SHA224_CTX(desc);
    ctx.ctx.ostate = HMAC_SHA224_KEY(desc.tfm).key.ostate;
    return __crypto_sha256_import_core(&ctx.ctx.sha_ctx, in);
    }
// HMAC-SHA256

    static int crypto_hmac_sha256_setkey(struct crypto_shash *tfm,
    const u8 *raw_key, unsigned int keylen)
    {
    hmac_sha256_preparekey(HMAC_SHA256_KEY(tfm), raw_key, keylen);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_hmac_sha256_init(desc: *mut shash_desc) -> c_int {
    static int crypto_hmac_sha256_init(struct shash_desc *desc)
    {
    hmac_sha256_init(HMAC_SHA256_CTX(desc), HMAC_SHA256_KEY(desc.tfm));
    return 0;
    }
    static int crypto_hmac_sha256_update(struct shash_desc *desc,
    const u8 *data, unsigned int len)
    {
    hmac_sha256_update(HMAC_SHA256_CTX(desc), data, len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_hmac_sha256_final(desc: *mut shash_desc, out: *mut u8) -> c_int {
    static int crypto_hmac_sha256_final(struct shash_desc *desc, u8 *out)
    {
    hmac_sha256_final(HMAC_SHA256_CTX(desc), out);
    return 0;
    }
    static int crypto_hmac_sha256_digest(struct shash_desc *desc,
    const u8 *data, unsigned int len,
    u8 *out)
    {
    hmac_sha256(HMAC_SHA256_KEY(desc.tfm), data, len, out);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_hmac_sha256_export(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int crypto_hmac_sha256_export(struct shash_desc *desc, void *out)
    {
    return __crypto_sha256_export(&HMAC_SHA256_CTX(desc).ctx.sha_ctx, out);
    }
#[no_mangle]
unsafe extern "C" fn crypto_hmac_sha256_import(desc: *mut shash_desc, in: *const c_void) -> c_int {
    static int crypto_hmac_sha256_import(struct shash_desc *desc, const void *in)
    {
    struct hmac_sha256_ctx *ctx = HMAC_SHA256_CTX(desc);
    ctx.ctx.ostate = HMAC_SHA256_KEY(desc.tfm).key.ostate;
    return __crypto_sha256_import(&ctx.ctx.sha_ctx, in);
    }
#[no_mangle]
unsafe extern "C" fn crypto_hmac_sha256_export_core(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int crypto_hmac_sha256_export_core(struct shash_desc *desc, void *out)
    {
    return __crypto_sha256_export_core(&HMAC_SHA256_CTX(desc).ctx.sha_ctx,
    out);
    }
    static int crypto_hmac_sha256_import_core(struct shash_desc *desc,
    const void *in)
    {
    struct hmac_sha256_ctx *ctx = HMAC_SHA256_CTX(desc);
    ctx.ctx.ostate = HMAC_SHA256_KEY(desc.tfm).key.ostate;
    return __crypto_sha256_import_core(&ctx.ctx.sha_ctx, in);
    }
// Algorithm definitions
    static struct shash_alg algs[] = {
    {
    .base.cra_name		= "sha224",
    .base.cra_driver_name	= "sha224-lib",
    .base.cra_priority	= 300,
    .base.cra_blocksize	= SHA224_BLOCK_SIZE,
    .base.cra_module	= THIS_MODULE,
    .digestsize		= SHA224_DIGEST_SIZE,
    .init			= crypto_sha224_init,
    .update			= crypto_sha224_update,
    .final			= crypto_sha224_final,
    .digest			= crypto_sha224_digest,
    .export			= crypto_sha224_export,
    .import			= crypto_sha224_import,
    .export_core		= crypto_sha224_export_core,
    .import_core		= crypto_sha224_import_core,
    .descsize		= sizeof(struct sha224_ctx),
    .statesize		= SHA256_SHASH_STATE_SIZE,
    },
    {
    .base.cra_name		= "sha256",
    .base.cra_driver_name	= "sha256-lib",
    .base.cra_priority	= 300,
    .base.cra_blocksize	= SHA256_BLOCK_SIZE,
    .base.cra_module	= THIS_MODULE,
    .digestsize		= SHA256_DIGEST_SIZE,
    .init			= crypto_sha256_init,
    .update			= crypto_sha256_update,
    .final			= crypto_sha256_final,
    .digest			= crypto_sha256_digest,
    .export			= crypto_sha256_export,
    .import			= crypto_sha256_import,
    .export_core		= crypto_sha256_export_core,
    .import_core		= crypto_sha256_import_core,
    .descsize		= sizeof(struct sha256_ctx),
    .statesize		= SHA256_SHASH_STATE_SIZE,
    },
    {
    .base.cra_name		= "hmac(sha224)",
    .base.cra_driver_name	= "hmac-sha224-lib",
    .base.cra_priority	= 300,
    .base.cra_blocksize	= SHA224_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct hmac_sha224_key),
    .base.cra_module	= THIS_MODULE,
    .digestsize		= SHA224_DIGEST_SIZE,
    .setkey			= crypto_hmac_sha224_setkey,
    .init			= crypto_hmac_sha224_init,
    .update			= crypto_hmac_sha224_update,
    .final			= crypto_hmac_sha224_final,
    .digest			= crypto_hmac_sha224_digest,
    .export			= crypto_hmac_sha224_export,
    .import			= crypto_hmac_sha224_import,
    .export_core		= crypto_hmac_sha224_export_core,
    .import_core		= crypto_hmac_sha224_import_core,
    .descsize		= sizeof(struct hmac_sha224_ctx),
    .statesize		= SHA256_SHASH_STATE_SIZE,
    },
    {
    .base.cra_name		= "hmac(sha256)",
    .base.cra_driver_name	= "hmac-sha256-lib",
    .base.cra_priority	= 300,
    .base.cra_blocksize	= SHA256_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct hmac_sha256_key),
    .base.cra_module	= THIS_MODULE,
    .digestsize		= SHA256_DIGEST_SIZE,
    .setkey			= crypto_hmac_sha256_setkey,
    .init			= crypto_hmac_sha256_init,
    .update			= crypto_hmac_sha256_update,
    .final			= crypto_hmac_sha256_final,
    .digest			= crypto_hmac_sha256_digest,
    .export			= crypto_hmac_sha256_export,
    .import			= crypto_hmac_sha256_import,
    .export_core		= crypto_hmac_sha256_export_core,
    .import_core		= crypto_hmac_sha256_import_core,
    .descsize		= sizeof(struct hmac_sha256_ctx),
    .statesize		= SHA256_SHASH_STATE_SIZE,
    },
    };
#[no_mangle]
unsafe extern "C" fn crypto_sha256_mod_init() -> int __init {
    static int __init crypto_sha256_mod_init(void)
    {
    return crypto_register_shashes(algs, ARRAY_SIZE(algs));
    }
    module_init(crypto_sha256_mod_init);
#[no_mangle]
unsafe extern "C" fn crypto_sha256_mod_exit() -> void __exit {
    static void __exit crypto_sha256_mod_exit(void)
    {
    crypto_unregister_shashes(algs, ARRAY_SIZE(algs));
    }
    module_exit(crypto_sha256_mod_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Crypto API support for SHA-224, SHA-256, HMAC-SHA224, and HMAC-SHA256");
    MODULE_ALIAS_CRYPTO("sha224");
    MODULE_ALIAS_CRYPTO("sha224-lib");
    MODULE_ALIAS_CRYPTO("sha256");
    MODULE_ALIAS_CRYPTO("sha256-lib");
    MODULE_ALIAS_CRYPTO("hmac(sha224)");
    MODULE_ALIAS_CRYPTO("hmac-sha224-lib");
    MODULE_ALIAS_CRYPTO("hmac(sha256)");
    MODULE_ALIAS_CRYPTO("hmac-sha256-lib");
