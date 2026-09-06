//! Automatically rewritten from C to Rust
//! Source: lib/crypto/chacha.c
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
// The ChaCha stream cipher (RFC7539)
//
// Copyright (C) 2015 Martin Willi
//

    static void __maybe_unused
    chacha_crypt_generic(struct chacha_state *state, u8 *dst, const u8 *src,
    unsigned int bytes, int nrounds)
    {
// aligned to potentially speed up crypto_xor()
    u8 stream[CHACHA_BLOCK_SIZE] __aligned(sizeof(long));
    while (bytes >= CHACHA_BLOCK_SIZE) {
    chacha_block_generic(state, stream, nrounds);
    crypto_xor_cpy(dst, src, stream, CHACHA_BLOCK_SIZE);
    bytes -= CHACHA_BLOCK_SIZE;
    dst += CHACHA_BLOCK_SIZE;
    src += CHACHA_BLOCK_SIZE;
    }
    if (bytes) {
    chacha_block_generic(state, stream, nrounds);
    crypto_xor_cpy(dst, src, stream, bytes);
    }
    }

    void chacha_crypt(struct chacha_state *state, u8 *dst, const u8 *src,
    unsigned int bytes, int nrounds)
    {
    chacha_crypt_arch(state, dst, src, bytes, nrounds);
    }
    EXPORT_SYMBOL_GPL(chacha_crypt);
    void hchacha_block(const struct chacha_state *state,
    u32 out[HCHACHA_OUT_WORDS], int nrounds)
    {
    hchacha_block_arch(state, out, nrounds);
    }
    EXPORT_SYMBOL_GPL(hchacha_block);

#[no_mangle]
unsafe extern "C" fn chacha_mod_init() -> int __init {
    static int __init chacha_mod_init(void)
    {
    chacha_mod_init_arch();
    return 0;
    }
    subsys_initcall(chacha_mod_init);
#[no_mangle]
unsafe extern "C" fn chacha_mod_exit() -> void __exit {
    static void __exit chacha_mod_exit(void)
    {
    }
    module_exit(chacha_mod_exit);

    MODULE_DESCRIPTION("ChaCha stream cipher (RFC7539)");
    MODULE_LICENSE("GPL");
