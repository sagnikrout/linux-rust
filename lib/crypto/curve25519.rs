//! Automatically rewritten from C to Rust
//! Source: lib/crypto/curve25519.c
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//
// This is an implementation of the Curve25519 ECDH algorithm, using either an
// architecture-optimized implementation or a generic implementation. The
// generic implementation is either 32-bit, or 64-bit with 128-bit integers,
// depending on what is supported by the target compiler.
//
// Information: https://cr.yp.to/ecdh.html
//

    static const u8 curve25519_null_point[CURVE25519_KEY_SIZE] __aligned(32) = { 0 };
    static const u8 curve25519_base_point[CURVE25519_KEY_SIZE] __aligned(32) = { 9 };

    static void curve25519_arch(u8 mypublic[CURVE25519_KEY_SIZE],
    const u8 secret[CURVE25519_KEY_SIZE],
    const u8 basepoint[CURVE25519_KEY_SIZE])
    {
    curve25519_generic(mypublic, secret, basepoint);
    }
    static void curve25519_base_arch(u8 pub[CURVE25519_KEY_SIZE],
    const u8 secret[CURVE25519_KEY_SIZE])
    {
    curve25519_generic(pub, secret, curve25519_base_point);
    }

    bool __must_check
    curve25519(u8 mypublic[CURVE25519_KEY_SIZE],
    const u8 secret[CURVE25519_KEY_SIZE],
    const u8 basepoint[CURVE25519_KEY_SIZE])
    {
    curve25519_arch(mypublic, secret, basepoint);
    return crypto_memneq(mypublic, curve25519_null_point,
    CURVE25519_KEY_SIZE);
    }
    EXPORT_SYMBOL(curve25519);
    bool __must_check
    curve25519_generate_public(u8 pub[CURVE25519_KEY_SIZE],
    const u8 secret[CURVE25519_KEY_SIZE])
    {
    if (unlikely(!crypto_memneq(secret, curve25519_null_point,
    CURVE25519_KEY_SIZE)))
    return false;
    curve25519_base_arch(pub, secret);
    return crypto_memneq(pub, curve25519_null_point, CURVE25519_KEY_SIZE);
    }
    EXPORT_SYMBOL(curve25519_generate_public);

#[no_mangle]
unsafe extern "C" fn curve25519_mod_init() -> int __init {
    static int __init curve25519_mod_init(void)
    {
    curve25519_mod_init_arch();
    return 0;
    }
    subsys_initcall(curve25519_mod_init);
#[no_mangle]
unsafe extern "C" fn curve25519_mod_exit() -> void __exit {
    static void __exit curve25519_mod_exit(void)
    {
    }
    module_exit(curve25519_mod_exit);

    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Curve25519 algorithm");
    MODULE_AUTHOR("Jason A. Donenfeld <Jason@zx2c4.com>");
