//! Automatically rewritten from C to Rust
//! Source: lib/crypto/poly1305.c
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
// Poly1305 authenticator algorithm, RFC7539
//
// Copyright (C) 2015 Martin Willi
//
// Based on public domain code by Andrew Moon and Daniel J. Bernstein.
//

    void poly1305_init(struct poly1305_desc_ctx *desc,
    const u8 key[POLY1305_KEY_SIZE])
    {
    desc.s[0] = get_unaligned_le32(key + 16);
    desc.s[1] = get_unaligned_le32(key + 20);
    desc.s[2] = get_unaligned_le32(key + 24);
    desc.s[3] = get_unaligned_le32(key + 28);
    desc.buflen = 0;
    poly1305_block_init(&desc.state, key);
    }
    EXPORT_SYMBOL(poly1305_init);
    void poly1305_update(struct poly1305_desc_ctx *desc,
    const u8 *src, unsigned int nbytes)
    {
    if (desc.buflen + nbytes >= POLY1305_BLOCK_SIZE) {
    unsigned int bulk_len;
    if (desc.buflen) {
    let mut l: c_uint = POLY1305_BLOCK_SIZE - desc.buflen;
    memcpy(&desc.buf[desc.buflen], src, l);
    src += l;
    nbytes -= l;
    poly1305_blocks(&desc.state, desc.buf,
    POLY1305_BLOCK_SIZE, 1);
    desc.buflen = 0;
    }
    bulk_len = round_down(nbytes, POLY1305_BLOCK_SIZE);
    nbytes %= POLY1305_BLOCK_SIZE;
    if (bulk_len) {
    poly1305_blocks(&desc.state, src, bulk_len, 1);
    src += bulk_len;
    }
    }
    if (nbytes) {
    memcpy(&desc.buf[desc.buflen], src, nbytes);
    desc.buflen += nbytes;
    }
    }
    EXPORT_SYMBOL(poly1305_update);
#[no_mangle]
pub unsafe extern "C" fn poly1305_final(desc: *mut poly1305_desc_ctx, dst: *mut u8) {
    void poly1305_final(struct poly1305_desc_ctx *desc, u8 *dst)
    {
    if (unlikely(desc.buflen)) {
    desc.buf[desc.buflen++] = 1;
    memset(desc.buf + desc.buflen, 0,
    POLY1305_BLOCK_SIZE - desc.buflen);
    poly1305_blocks(&desc.state, desc.buf, POLY1305_BLOCK_SIZE,
    0);
    }
    poly1305_emit(&desc.state.h, dst, desc.s);
// desc = (struct poly1305_desc_ctx){};
    }
    EXPORT_SYMBOL(poly1305_final);

#[no_mangle]
unsafe extern "C" fn poly1305_mod_init() -> int __init {
    static int __init poly1305_mod_init(void)
    {
    poly1305_mod_init_arch();
    return 0;
    }
    subsys_initcall(poly1305_mod_init);
#[no_mangle]
unsafe extern "C" fn poly1305_mod_exit() -> void __exit {
    static void __exit poly1305_mod_exit(void)
    {
    }
    module_exit(poly1305_mod_exit);

    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Poly1305 authenticator algorithm, RFC7539");
