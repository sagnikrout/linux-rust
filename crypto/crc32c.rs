//! Automatically rewritten from C to Rust
//! Source: crypto/crc32c.c
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
// crypto_shash support for CRC-32C
//
// @Article{castagnoli-crc,
// author =       { Guy Castagnoli and Stefan Braeuer and Martin Herrman},
// title =        {{Optimization of Cyclic Redundancy-Check Codes with 24
// and 32 Parity Bits}},
// journal =      IEEE Transactions on Communication,
// year =         {1993},
// volume =       {41},
// number =       {6},
// pages =        {},
// month =        {June},
// }
//
// Copyright (c) 2004 Cisco Systems, Inc.
// Copyright (c) 2008 Herbert Xu <herbert@gondor.apana.org.au>
//

pub const CHKSUM_BLOCK_SIZE: c_int = 1;
pub const CHKSUM_DIGEST_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chksum_ctx {
    pub key: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chksum_desc_ctx {
    pub crc: u32,
}

#[no_mangle]
unsafe extern "C" fn chksum_init(desc: *mut shash_desc) -> c_int {
    static int chksum_init(struct shash_desc *desc)
    {
    struct chksum_ctx *mctx = crypto_shash_ctx(desc.tfm);
    struct chksum_desc_ctx *ctx = shash_desc_ctx(desc);
    ctx.crc = mctx.key;
    return 0;
    }
//
// Setting the seed allows arbitrary accumulators and flexible XOR policy
// If your algorithm starts with ~0, then XOR with ~0 before you set
// the seed.
//
    static int chksum_setkey(struct crypto_shash *tfm, const u8 *key,
    unsigned int keylen)
    {
    struct chksum_ctx *mctx = crypto_shash_ctx(tfm);
    if (keylen != sizeof(mctx.key))
    return -EINVAL;
    mctx.key = get_unaligned_le32(key);
    return 0;
    }
    static int chksum_update(struct shash_desc *desc, const u8 *data,
    unsigned int length)
    {
    struct chksum_desc_ctx *ctx = shash_desc_ctx(desc);
    ctx.crc = crc32c(ctx.crc, data, length);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chksum_final(desc: *mut shash_desc, out: *mut u8) -> c_int {
    static int chksum_final(struct shash_desc *desc, u8 *out)
    {
    struct chksum_desc_ctx *ctx = shash_desc_ctx(desc);
    put_unaligned_le32(~ctx.crc, out);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __chksum_finup(crcp: *mut u32, data: *const u8, len: c_uint, out: *mut u8) -> c_int {
    static int __chksum_finup(u32 *crcp, const u8 *data, unsigned int len, u8 *out)
    {
    put_unaligned_le32(~crc32c(*crcp, data, len), out);
    return 0;
    }
    static int chksum_finup(struct shash_desc *desc, const u8 *data,
    unsigned int len, u8 *out)
    {
    struct chksum_desc_ctx *ctx = shash_desc_ctx(desc);
    return __chksum_finup(&ctx.crc, data, len, out);
    }
    static int chksum_digest(struct shash_desc *desc, const u8 *data,
    unsigned int length, u8 *out)
    {
    struct chksum_ctx *mctx = crypto_shash_ctx(desc.tfm);
    return __chksum_finup(&mctx.key, data, length, out);
    }
#[no_mangle]
unsafe extern "C" fn crc32c_cra_init(tfm: *mut crypto_tfm) -> c_int {
    static int crc32c_cra_init(struct crypto_tfm *tfm)
    {
    struct chksum_ctx *mctx = crypto_tfm_ctx(tfm);
    mctx.key = ~0;
    return 0;
    }
    static struct shash_alg alg = {
    .digestsize		= CHKSUM_DIGEST_SIZE,
    .setkey			= chksum_setkey,
    .init			= chksum_init,
    .update			= chksum_update,
    .final			= chksum_final,
    .finup			= chksum_finup,
    .digest			= chksum_digest,
    .descsize		= sizeof(struct chksum_desc_ctx),
    .base.cra_name		= "crc32c",
    .base.cra_driver_name	= "crc32c-lib",
    .base.cra_priority	= 100,
    .base.cra_flags		= CRYPTO_ALG_OPTIONAL_KEY,
    .base.cra_blocksize	= CHKSUM_BLOCK_SIZE,
    .base.cra_ctxsize	= sizeof(struct chksum_ctx),
    .base.cra_module	= THIS_MODULE,
    .base.cra_init		= crc32c_cra_init,
    };
#[no_mangle]
unsafe extern "C" fn crc32c_mod_init() -> int __init {
    static int __init crc32c_mod_init(void)
    {
    return crypto_register_shash(&alg);
    }
#[no_mangle]
unsafe extern "C" fn crc32c_mod_fini() -> void __exit {
    static void __exit crc32c_mod_fini(void)
    {
    crypto_unregister_shash(&alg);
    }
    module_init(crc32c_mod_init);
    module_exit(crc32c_mod_fini);
    MODULE_AUTHOR("Clay Haapala <chaapala@cisco.com>");
    MODULE_DESCRIPTION("CRC32c (Castagnoli) calculations wrapper for lib/crc32c");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_CRYPTO("crc32c");
