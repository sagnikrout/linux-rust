//! Automatically rewritten from C to Rust
//! Source: crypto/arc4.c
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
// Cryptographic API
//
// ARC4 Cipher Algorithm
//
// Jon Oberheide <jon@oberheide.org>
//

    static int crypto_arc4_setkey(struct crypto_lskcipher *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct arc4_ctx *ctx = crypto_lskcipher_ctx(tfm);
    return arc4_setkey(ctx, in_key, key_len);
    }
    static int crypto_arc4_crypt(struct crypto_lskcipher *tfm, const u8 *src,
    u8 *dst, unsigned nbytes, u8 *siv, u32 flags)
    {
    struct arc4_ctx *ctx = crypto_lskcipher_ctx(tfm);
    if (!(flags & CRYPTO_LSKCIPHER_FLAG_CONT))
    memcpy(siv, ctx, sizeof(*ctx));
    ctx = (struct arc4_ctx *)siv;
    arc4_crypt(ctx, dst, src, nbytes);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crypto_arc4_init(tfm: *mut crypto_lskcipher) -> c_int {
    static int crypto_arc4_init(struct crypto_lskcipher *tfm)
    {
    pr_warn_ratelimited("\"%s\" (%ld) uses obsolete ecb(arc4) skcipher\n",
    current.comm, (unsigned long)current.pid);
    return 0;
    }
    static struct lskcipher_alg arc4_alg = {
    .co.base.cra_name		=	"arc4",
    .co.base.cra_driver_name	=	"arc4-generic",
    .co.base.cra_priority		=	100,
    .co.base.cra_blocksize		=	ARC4_BLOCK_SIZE,
    .co.base.cra_ctxsize		=	sizeof(struct arc4_ctx),
    .co.base.cra_alignmask		=	ARC4_ALIGN - 1,
    .co.base.cra_module		=	THIS_MODULE,
    .co.min_keysize			=	ARC4_MIN_KEY_SIZE,
    .co.max_keysize			=	ARC4_MAX_KEY_SIZE,
    .co.statesize			=	sizeof(struct arc4_ctx),
    .setkey				=	crypto_arc4_setkey,
    .encrypt			=	crypto_arc4_crypt,
    .decrypt			=	crypto_arc4_crypt,
    .init				=	crypto_arc4_init,
    };
#[no_mangle]
unsafe extern "C" fn arc4_init() -> int __init {
    static int __init arc4_init(void)
    {
    return crypto_register_lskcipher(&arc4_alg);
    }
#[no_mangle]
unsafe extern "C" fn arc4_exit() -> void __exit {
    static void __exit arc4_exit(void)
    {
    crypto_unregister_lskcipher(&arc4_alg);
    }
    module_init(arc4_init);
    module_exit(arc4_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("ARC4 Cipher Algorithm");
    MODULE_AUTHOR("Jon Oberheide <jon@oberheide.org>");
    MODULE_ALIAS_CRYPTO("ecb(arc4)");
