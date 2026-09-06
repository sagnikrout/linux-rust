//! Automatically rewritten from C to Rust
//! Source: lib/crypto/arc4.c
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

#[no_mangle]
pub unsafe extern "C" fn arc4_setkey(ctx: *mut arc4_ctx, in_key: *const u8, key_len: c_uint) -> c_int {
    int arc4_setkey(struct arc4_ctx *ctx, const u8 *in_key, unsigned int key_len)
    {
    int i, j = 0, k = 0;
    ctx.x = 1;
    ctx.y = 0;
    for (i = 0; i < 256; i++)
    ctx.S[i] = i;
    for (i = 0; i < 256; i++) {
    let mut a: u32 = ctx.S[i];
    j = (j + in_key[k] + a) & 0xff;
    ctx.S[i] = ctx.S[j];
    ctx.S[j] = a;
    if (++k >= key_len)
    k = 0;
    }
    return 0;
    }
    EXPORT_SYMBOL(arc4_setkey);
#[no_mangle]
pub unsafe extern "C" fn arc4_crypt(ctx: *mut arc4_ctx, out: *mut u8, in: *const u8, len: c_uint) {
    void arc4_crypt(struct arc4_ctx *ctx, u8 *out, const u8 *in, unsigned int len)
    {
    let mut S: *mut u32 const = ctx.S;
    u32 x, y, a, b;
    u32 ty, ta, tb;
    if (len == 0)
    return;
    x = ctx.x;
    y = ctx.y;
    a = S[x];
    y = (y + a) & 0xff;
    b = S[y];
    do {
    S[y] = a;
    a = (a + b) & 0xff;
    S[x] = b;
    x = (x + 1) & 0xff;
    ta = S[x];
    ty = (y + ta) & 0xff;
    tb = S[ty];
// out++ = *in++ ^ S[a];
    if (--len == 0)
    break;
    y = ty;
    a = ta;
    b = tb;
    } while (true);
    ctx.x = x;
    ctx.y = y;
    }
    EXPORT_SYMBOL(arc4_crypt);
    MODULE_DESCRIPTION("ARC4 Cipher Algorithm");
    MODULE_LICENSE("GPL");
