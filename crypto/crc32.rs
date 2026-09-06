//! Automatically rewritten from C to Rust
//! Source: crypto/crc32.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2012 Xyratex Technology Limited
//
// This is crypto api shash wrappers to crc32_le.
//

pub const CHKSUM_BLOCK_SIZE: c_int = 1;
pub const CHKSUM_DIGEST_SIZE: c_int = 4;
// No default init with ~0
#[no_mangle]
unsafe extern "C" fn crc32_cra_init(tfm: *mut crypto_tfm) -> c_int {
    static int crc32_cra_init(struct crypto_tfm *tfm)
    {
    u32 *key = crypto_tfm_ctx(tfm);
// key = 0;
    return 0;
    }
//
// Setting the seed allows arbitrary accumulators and flexible XOR policy
// If your algorithm starts with ~0, then XOR with ~0 before you set
// the seed.
//
    static int crc32_setkey(struct crypto_shash *hash, const u8 *key,
    unsigned int keylen)
    {
    u32 *mctx = crypto_shash_ctx(hash);
    if (keylen != sizeof(u32))
    return -EINVAL;
// mctx = get_unaligned_le32(key);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crc32_init(desc: *mut shash_desc) -> c_int {
    static int crc32_init(struct shash_desc *desc)
    {
    u32 *mctx = crypto_shash_ctx(desc.tfm);
    u32 *crcp = shash_desc_ctx(desc);
// crcp = *mctx;
    return 0;
    }
    static int crc32_update(struct shash_desc *desc, const u8 *data,
    unsigned int len)
    {
    u32 *crcp = shash_desc_ctx(desc);
// crcp = crc32_le(*crcp, data, len);
    return 0;
    }
// No final XOR 0xFFFFFFFF, like crc32_le
#[no_mangle]
unsafe extern "C" fn __crc32_finup(crcp: *mut u32, data: *const u8, len: c_uint, out: *mut u8) -> c_int {
    static int __crc32_finup(u32 *crcp, const u8 *data, unsigned int len, u8 *out)
    {
    put_unaligned_le32(crc32_le(*crcp, data, len), out);
    return 0;
    }
    static int crc32_finup(struct shash_desc *desc, const u8 *data,
    unsigned int len, u8 *out)
    {
    return __crc32_finup(shash_desc_ctx(desc), data, len, out);
    }
#[no_mangle]
unsafe extern "C" fn crc32_final(desc: *mut shash_desc, out: *mut u8) -> c_int {
    static int crc32_final(struct shash_desc *desc, u8 *out)
    {
    u32 *crcp = shash_desc_ctx(desc);
    put_unaligned_le32(*crcp, out);
    return 0;
    }
    static int crc32_digest(struct shash_desc *desc, const u8 *data,
    unsigned int len, u8 *out)
    {
    return __crc32_finup(crypto_shash_ctx(desc.tfm), data, len, out);
    }
    static struct shash_alg alg = {
    .setkey			= crc32_setkey,
    .init			= crc32_init,
    .update			= crc32_update,
    .final			= crc32_final,
    .finup			= crc32_finup,
    .digest			= crc32_digest,
    .descsize		= sizeof(u32),
    .digestsize		= CHKSUM_DIGEST_SIZE,
    .base.cra_name		= "crc32",
    .base.cra_driver_name	= "crc32-lib",
    .base.cra_priority	= 100,
    .base.cra_flags		= CRYPTO_ALG_OPTIONAL_KEY,
    .base.cra_blocksize	= CHKSUM_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(u32),
    .base.cra_module	= THIS_MODULE,
    .base.cra_init		= crc32_cra_init,
    };
#[no_mangle]
unsafe extern "C" fn crc32_mod_init() -> int __init {
    static int __init crc32_mod_init(void)
    {
    return crypto_register_shash(&alg);
    }
#[no_mangle]
unsafe extern "C" fn crc32_mod_fini() -> void __exit {
    static void __exit crc32_mod_fini(void)
    {
    crypto_unregister_shash(&alg);
    }
    module_init(crc32_mod_init);
    module_exit(crc32_mod_fini);
    MODULE_AUTHOR("Alexander Boyko <alexander_boyko@xyratex.com>");
    MODULE_DESCRIPTION("CRC32 calculations wrapper for lib/crc32");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_CRYPTO("crc32");
