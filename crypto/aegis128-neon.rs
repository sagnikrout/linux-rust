//! Automatically rewritten from C to Rust
//! Source: crypto/aegis128-neon.c
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
// Copyright (C) 2019 Linaro Ltd <ard.biesheuvel@linaro.org>
//

    int aegis128_have_aes_insn __ro_after_init;
#[no_mangle]
pub unsafe extern "C" fn crypto_aegis128_have_simd() -> bool {
    bool crypto_aegis128_have_simd(void)
    {
    if (cpu_have_feature(cpu_feature(AES))) {
    aegis128_have_aes_insn = 1;
    return true;
    }
    return IS_ENABLED(CONFIG_ARM64);
    }
    void crypto_aegis128_init_simd(struct aegis_state *state,
    const union aegis_block *key,
    const u8 *iv)
    {
    scoped_ksimd()
    crypto_aegis128_init_neon(state, key, iv);
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_aegis128_update_simd(state: *mut aegis_state, msg: *const c_void) {
    void crypto_aegis128_update_simd(struct aegis_state *state, const void *msg)
    {
    scoped_ksimd()
    crypto_aegis128_update_neon(state, msg);
    }
    void crypto_aegis128_encrypt_chunk_simd(struct aegis_state *state, u8 *dst,
    const u8 *src, unsigned int size)
    {
    scoped_ksimd()
    crypto_aegis128_encrypt_chunk_neon(state, dst, src, size);
    }
    void crypto_aegis128_decrypt_chunk_simd(struct aegis_state *state, u8 *dst,
    const u8 *src, unsigned int size)
    {
    scoped_ksimd()
    crypto_aegis128_decrypt_chunk_neon(state, dst, src, size);
    }
    int crypto_aegis128_final_simd(struct aegis_state *state,
    union aegis_block *tag_xor,
    unsigned int assoclen,
    unsigned int cryptlen,
    unsigned int authsize)
    {
    scoped_ksimd()
    return crypto_aegis128_final_neon(state, tag_xor, assoclen,
    cryptlen, authsize);
    }
