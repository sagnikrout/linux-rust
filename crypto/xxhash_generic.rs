//! Automatically rewritten from C to Rust
//! Source: crypto/xxhash_generic.c
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

pub const XXHASH64_BLOCK_SIZE: c_int = 32;
pub const XXHASH64_DIGEST_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xxhash64_tfm_ctx {
    pub seed: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xxhash64_desc_ctx {
    pub xxhstate: xxh64_state,
}

    static int xxhash64_setkey(struct crypto_shash *tfm, const u8 *key,
    unsigned int keylen)
    {
    struct xxhash64_tfm_ctx *tctx = crypto_shash_ctx(tfm);
    if (keylen != sizeof(tctx.seed))
    return -EINVAL;
    tctx.seed = get_unaligned_le64(key);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xxhash64_init(desc: *mut shash_desc) -> c_int {
    static int xxhash64_init(struct shash_desc *desc)
    {
    struct xxhash64_tfm_ctx *tctx = crypto_shash_ctx(desc.tfm);
    struct xxhash64_desc_ctx *dctx = shash_desc_ctx(desc);
    xxh64_reset(&dctx.xxhstate, tctx.seed);
    return 0;
    }
    static int xxhash64_update(struct shash_desc *desc, const u8 *data,
    unsigned int length)
    {
    struct xxhash64_desc_ctx *dctx = shash_desc_ctx(desc);
    xxh64_update(&dctx.xxhstate, data, length);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xxhash64_final(desc: *mut shash_desc, out: *mut u8) -> c_int {
    static int xxhash64_final(struct shash_desc *desc, u8 *out)
    {
    struct xxhash64_desc_ctx *dctx = shash_desc_ctx(desc);
    put_unaligned_le64(xxh64_digest(&dctx.xxhstate), out);
    return 0;
    }
    static int xxhash64_digest(struct shash_desc *desc, const u8 *data,
    unsigned int length, u8 *out)
    {
    struct xxhash64_tfm_ctx *tctx = crypto_shash_ctx(desc.tfm);
    put_unaligned_le64(xxh64(data, length, tctx.seed), out);
    return 0;
    }
    static struct shash_alg alg = {
    .digestsize	= XXHASH64_DIGEST_SIZE,
    .setkey		= xxhash64_setkey,
    .init		= xxhash64_init,
    .update		= xxhash64_update,
    .final		= xxhash64_final,
    .digest		= xxhash64_digest,
    .descsize	= sizeof(struct xxhash64_desc_ctx),
    .base		= {
    .cra_name	 = "xxhash64",
    .cra_driver_name = "xxhash64-generic",
    .cra_priority	 = 100,
    .cra_flags	 = CRYPTO_ALG_OPTIONAL_KEY,
    .cra_blocksize	 = XXHASH64_BLOCK_SIZE,
    .cra_ctxsize	 = sizeof(struct xxhash64_tfm_ctx),
    .cra_module	 = THIS_MODULE,
    }
    };
#[no_mangle]
unsafe extern "C" fn xxhash_mod_init() -> int __init {
    static int __init xxhash_mod_init(void)
    {
    return crypto_register_shash(&alg);
    }
#[no_mangle]
unsafe extern "C" fn xxhash_mod_fini() -> void __exit {
    static void __exit xxhash_mod_fini(void)
    {
    crypto_unregister_shash(&alg);
    }
    module_init(xxhash_mod_init);
    module_exit(xxhash_mod_fini);
    MODULE_AUTHOR("Nikolay Borisov <nborisov@suse.com>");
    MODULE_DESCRIPTION("xxhash calculations wrapper for lib/xxhash.c");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_CRYPTO("xxhash64");
    MODULE_ALIAS_CRYPTO("xxhash64-generic");
