//! Automatically rewritten from C to Rust
//! Source: crypto/sha3.c
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
// Crypto API support for SHA-3
// (https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf)
//

#[no_mangle]
unsafe extern "C" fn crypto_sha3_224_init(desc: *mut shash_desc) -> c_int {
    static int crypto_sha3_224_init(struct shash_desc *desc)
    {
    sha3_224_init(SHA3_CTX(desc));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_sha3_256_init(desc: *mut shash_desc) -> c_int {
    static int crypto_sha3_256_init(struct shash_desc *desc)
    {
    sha3_256_init(SHA3_CTX(desc));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_sha3_384_init(desc: *mut shash_desc) -> c_int {
    static int crypto_sha3_384_init(struct shash_desc *desc)
    {
    sha3_384_init(SHA3_CTX(desc));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_sha3_512_init(desc: *mut shash_desc) -> c_int {
    static int crypto_sha3_512_init(struct shash_desc *desc)
    {
    sha3_512_init(SHA3_CTX(desc));
    return 0;
    }
    static int crypto_sha3_update(struct shash_desc *desc, const u8 *data,
    unsigned int len)
    {
    sha3_update(SHA3_CTX(desc), data, len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_sha3_final(desc: *mut shash_desc, out: *mut u8) -> c_int {
    static int crypto_sha3_final(struct shash_desc *desc, u8 *out)
    {
    sha3_final(SHA3_CTX(desc), out);
    return 0;
    }
    static int crypto_sha3_224_digest(struct shash_desc *desc,
    const u8 *data, unsigned int len, u8 *out)
    {
    sha3_224(data, len, out);
    return 0;
    }
    static int crypto_sha3_256_digest(struct shash_desc *desc,
    const u8 *data, unsigned int len, u8 *out)
    {
    sha3_256(data, len, out);
    return 0;
    }
    static int crypto_sha3_384_digest(struct shash_desc *desc,
    const u8 *data, unsigned int len, u8 *out)
    {
    sha3_384(data, len, out);
    return 0;
    }
    static int crypto_sha3_512_digest(struct shash_desc *desc,
    const u8 *data, unsigned int len, u8 *out)
    {
    sha3_512(data, len, out);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_sha3_export_core(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int crypto_sha3_export_core(struct shash_desc *desc, void *out)
    {
    memcpy(out, SHA3_CTX(desc), sizeof(struct sha3_ctx));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_sha3_import_core(desc: *mut shash_desc, in: *const c_void) -> c_int {
    static int crypto_sha3_import_core(struct shash_desc *desc, const void *in)
    {
    memcpy(SHA3_CTX(desc), in, sizeof(struct sha3_ctx));
    return 0;
    }
    static struct shash_alg algs[] = { {
    .digestsize		= SHA3_224_DIGEST_SIZE,
    .init			= crypto_sha3_224_init,
    .update			= crypto_sha3_update,
    .final			= crypto_sha3_final,
    .digest			= crypto_sha3_224_digest,
    .export_core		= crypto_sha3_export_core,
    .import_core		= crypto_sha3_import_core,
    .descsize		= sizeof(struct sha3_ctx),
    .base.cra_name		= "sha3-224",
    .base.cra_driver_name	= "sha3-224-lib",
    .base.cra_blocksize	= SHA3_224_BLOCK_SIZE,
    .base.cra_module	= THIS_MODULE,
    }, {
    .digestsize		= SHA3_256_DIGEST_SIZE,
    .init			= crypto_sha3_256_init,
    .update			= crypto_sha3_update,
    .final			= crypto_sha3_final,
    .digest			= crypto_sha3_256_digest,
    .export_core		= crypto_sha3_export_core,
    .import_core		= crypto_sha3_import_core,
    .descsize		= sizeof(struct sha3_ctx),
    .base.cra_name		= "sha3-256",
    .base.cra_driver_name	= "sha3-256-lib",
    .base.cra_blocksize	= SHA3_256_BLOCK_SIZE,
    .base.cra_module	= THIS_MODULE,
    }, {
    .digestsize		= SHA3_384_DIGEST_SIZE,
    .init			= crypto_sha3_384_init,
    .update			= crypto_sha3_update,
    .final			= crypto_sha3_final,
    .digest			= crypto_sha3_384_digest,
    .export_core		= crypto_sha3_export_core,
    .import_core		= crypto_sha3_import_core,
    .descsize		= sizeof(struct sha3_ctx),
    .base.cra_name		= "sha3-384",
    .base.cra_driver_name	= "sha3-384-lib",
    .base.cra_blocksize	= SHA3_384_BLOCK_SIZE,
    .base.cra_module	= THIS_MODULE,
    }, {
    .digestsize		= SHA3_512_DIGEST_SIZE,
    .init			= crypto_sha3_512_init,
    .update			= crypto_sha3_update,
    .final			= crypto_sha3_final,
    .digest			= crypto_sha3_512_digest,
    .export_core		= crypto_sha3_export_core,
    .import_core		= crypto_sha3_import_core,
    .descsize		= sizeof(struct sha3_ctx),
    .base.cra_name		= "sha3-512",
    .base.cra_driver_name	= "sha3-512-lib",
    .base.cra_blocksize	= SHA3_512_BLOCK_SIZE,
    .base.cra_module	= THIS_MODULE,
    } };
#[no_mangle]
unsafe extern "C" fn crypto_sha3_mod_init() -> int __init {
    static int __init crypto_sha3_mod_init(void)
    {
    return crypto_register_shashes(algs, ARRAY_SIZE(algs));
    }
    module_init(crypto_sha3_mod_init);
#[no_mangle]
unsafe extern "C" fn crypto_sha3_mod_exit() -> void __exit {
    static void __exit crypto_sha3_mod_exit(void)
    {
    crypto_unregister_shashes(algs, ARRAY_SIZE(algs));
    }
    module_exit(crypto_sha3_mod_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Crypto API support for SHA-3");
    MODULE_ALIAS_CRYPTO("sha3-224");
    MODULE_ALIAS_CRYPTO("sha3-224-lib");
    MODULE_ALIAS_CRYPTO("sha3-256");
    MODULE_ALIAS_CRYPTO("sha3-256-lib");
    MODULE_ALIAS_CRYPTO("sha3-384");
    MODULE_ALIAS_CRYPTO("sha3-384-lib");
    MODULE_ALIAS_CRYPTO("sha3-512");
    MODULE_ALIAS_CRYPTO("sha3-512-lib");
