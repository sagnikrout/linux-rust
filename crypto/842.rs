//! Automatically rewritten from C to Rust
//! Source: crypto/842.c
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
// Cryptographic API for the 842 software compression algorithm.
//
// Copyright (C) IBM Corporation, 2011-2015
//
// Original Authors: Robert Jennings <rcj@linux.vnet.ibm.com>
// Seth Jennings <sjenning@linux.vnet.ibm.com>
//
// Rewrite: Dan Streetman <ddstreet@ieee.org>
//
// This is the software implementation of compression and decompression using
// the 842 format.  This uses the software 842 library at lib/842/ which is
// only a reference implementation, and is very, very slow as compared to other
// software compressors.  You probably do not want to use this software
// compression.  If you have access to the PowerPC 842 compression hardware, you
// want to use the 842 hardware compression interface, which is at:
// drivers/crypto/nx/nx-842-crypto.c
//

    static void *crypto842_alloc_ctx(void)
    {
    void *ctx;
    ctx = kmalloc(SW842_MEM_COMPRESS, GFP_KERNEL);
    if (!ctx)
    return ERR_PTR(-ENOMEM);
    return ctx;
    }
#[no_mangle]
unsafe extern "C" fn crypto842_free_ctx(ctx: *mut c_void) {
    static void crypto842_free_ctx(void *ctx)
    {
    kfree(ctx);
    }
    static int crypto842_scompress(struct crypto_scomp *tfm,
    const u8 *src, unsigned int slen,
    u8 *dst, unsigned int *dlen, void *ctx)
    {
    return sw842_compress(src, slen, dst, dlen, ctx);
    }
    static int crypto842_sdecompress(struct crypto_scomp *tfm,
    const u8 *src, unsigned int slen,
    u8 *dst, unsigned int *dlen, void *ctx)
    {
    return sw842_decompress(src, slen, dst, dlen);
    }
    static struct scomp_alg scomp = {
    .streams		= {
    .alloc_ctx	= crypto842_alloc_ctx,
    .free_ctx	= crypto842_free_ctx,
    },
    .compress		= crypto842_scompress,
    .decompress		= crypto842_sdecompress,
    .base			= {
    .cra_name	= "842",
    .cra_driver_name = "842-scomp",
    .cra_priority	 = 100,
    .cra_module	 = THIS_MODULE,
    }
    };
#[no_mangle]
unsafe extern "C" fn crypto842_mod_init() -> int __init {
    static int __init crypto842_mod_init(void)
    {
    return crypto_register_scomp(&scomp);
    }
    module_init(crypto842_mod_init);
#[no_mangle]
unsafe extern "C" fn crypto842_mod_exit() -> void __exit {
    static void __exit crypto842_mod_exit(void)
    {
    crypto_unregister_scomp(&scomp);
    }
    module_exit(crypto842_mod_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("842 Software Compression Algorithm");
    MODULE_ALIAS_CRYPTO("842");
    MODULE_ALIAS_CRYPTO("842-generic");
    MODULE_AUTHOR("Dan Streetman <ddstreet@ieee.org>");
