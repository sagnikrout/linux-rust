//! Automatically rewritten from C to Rust
//! Source: crypto/md4.c
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


//
// Cryptographic API.
//
// MD4 Message Digest Algorithm (RFC1320).
//
// Implementation derived from Andrew Tridgell and Steve French's
// CIFS MD4 implementation, and the cryptoapi implementation
// originally based on the public domain implementation written
// by Colin Plumb in 1993.
//
// Copyright (c) Andrew Tridgell 1997-1998.
// Modified by Steve French (sfrench@us.ibm.com) 2002
// Copyright (c) Cryptoapi developers.
// Copyright (c) 2002 David S. Miller (davem@redhat.com)
// Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//

pub const MD4_DIGEST_SIZE: c_int = 16;
pub const MD4_HMAC_BLOCK_SIZE: c_int = 64;
pub const MD4_BLOCK_WORDS: c_int = 16;
pub const MD4_HASH_WORDS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md4_ctx {
    pub hash: [u32; MD4_HASH_WORDS],
    pub block: [u32; MD4_BLOCK_WORDS],
    pub byte_count: u64,
}

#[no_mangle]
pub unsafe extern "C" fn lshift(x: u32, s: c_uint) -> u32 {
    static inline u32 lshift(u32 x, unsigned int s)
    {
    x &= 0xFFFFFFFF;
    return ((x << s) & 0xFFFFFFFF) | (x >> (32 - s));
    }
#[no_mangle]
pub unsafe extern "C" fn F(x: u32, y: u32, z: u32) -> u32 {
    static inline u32 F(u32 x, u32 y, u32 z)
    {
    return (x & y) | ((~x) & z);
    }
#[no_mangle]
pub unsafe extern "C" fn G(x: u32, y: u32, z: u32) -> u32 {
    static inline u32 G(u32 x, u32 y, u32 z)
    {
    return (x & y) | (x & z) | (y & z);
    }
#[no_mangle]
pub unsafe extern "C" fn H(x: u32, y: u32, z: u32) -> u32 {
    static inline u32 H(u32 x, u32 y, u32 z)
    {
    return x ^ y ^ z;
    }

#[no_mangle]
unsafe extern "C" fn md4_transform(hash: *mut u32, in: *const u32) {
    static void md4_transform(u32 *hash, u32 const *in)
    {
    u32 a, b, c, d;
    a = hash[0];
    b = hash[1];
    c = hash[2];
    d = hash[3];
    ROUND1(a, b, c, d, in[0], 3);
    ROUND1(d, a, b, c, in[1], 7);
    ROUND1(c, d, a, b, in[2], 11);
    ROUND1(b, c, d, a, in[3], 19);
    ROUND1(a, b, c, d, in[4], 3);
    ROUND1(d, a, b, c, in[5], 7);
    ROUND1(c, d, a, b, in[6], 11);
    ROUND1(b, c, d, a, in[7], 19);
    ROUND1(a, b, c, d, in[8], 3);
    ROUND1(d, a, b, c, in[9], 7);
    ROUND1(c, d, a, b, in[10], 11);
    ROUND1(b, c, d, a, in[11], 19);
    ROUND1(a, b, c, d, in[12], 3);
    ROUND1(d, a, b, c, in[13], 7);
    ROUND1(c, d, a, b, in[14], 11);
    ROUND1(b, c, d, a, in[15], 19);
    ROUND2(a, b, c, d,in[ 0], 3);
    ROUND2(d, a, b, c, in[4], 5);
    ROUND2(c, d, a, b, in[8], 9);
    ROUND2(b, c, d, a, in[12], 13);
    ROUND2(a, b, c, d, in[1], 3);
    ROUND2(d, a, b, c, in[5], 5);
    ROUND2(c, d, a, b, in[9], 9);
    ROUND2(b, c, d, a, in[13], 13);
    ROUND2(a, b, c, d, in[2], 3);
    ROUND2(d, a, b, c, in[6], 5);
    ROUND2(c, d, a, b, in[10], 9);
    ROUND2(b, c, d, a, in[14], 13);
    ROUND2(a, b, c, d, in[3], 3);
    ROUND2(d, a, b, c, in[7], 5);
    ROUND2(c, d, a, b, in[11], 9);
    ROUND2(b, c, d, a, in[15], 13);
    ROUND3(a, b, c, d,in[ 0], 3);
    ROUND3(d, a, b, c, in[8], 9);
    ROUND3(c, d, a, b, in[4], 11);
    ROUND3(b, c, d, a, in[12], 15);
    ROUND3(a, b, c, d, in[2], 3);
    ROUND3(d, a, b, c, in[10], 9);
    ROUND3(c, d, a, b, in[6], 11);
    ROUND3(b, c, d, a, in[14], 15);
    ROUND3(a, b, c, d, in[1], 3);
    ROUND3(d, a, b, c, in[9], 9);
    ROUND3(c, d, a, b, in[5], 11);
    ROUND3(b, c, d, a, in[13], 15);
    ROUND3(a, b, c, d, in[3], 3);
    ROUND3(d, a, b, c, in[11], 9);
    ROUND3(c, d, a, b, in[7], 11);
    ROUND3(b, c, d, a, in[15], 15);
    hash[0] += a;
    hash[1] += b;
    hash[2] += c;
    hash[3] += d;
    }
#[no_mangle]
pub unsafe extern "C" fn md4_transform_helper(ctx: *mut md4_ctx) {
    static inline void md4_transform_helper(struct md4_ctx *ctx)
    {
    le32_to_cpu_array(ctx.block, ARRAY_SIZE(ctx.block));
    md4_transform(ctx.hash, ctx.block);
    }
#[no_mangle]
unsafe extern "C" fn md4_init(desc: *mut shash_desc) -> c_int {
    static int md4_init(struct shash_desc *desc)
    {
    struct md4_ctx *mctx = shash_desc_ctx(desc);
    mctx.hash[0] = 0x67452301;
    mctx.hash[1] = 0xefcdab89;
    mctx.hash[2] = 0x98badcfe;
    mctx.hash[3] = 0x10325476;
    mctx.byte_count = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn md4_update(desc: *mut shash_desc, data: *const u8, len: c_uint) -> c_int {
    static int md4_update(struct shash_desc *desc, const u8 *data, unsigned int len)
    {
    struct md4_ctx *mctx = shash_desc_ctx(desc);
    let mut avail: u32 = sizeof(mctx.block) - (mctx.byte_count & 0x3f);
    mctx.byte_count += len;
    if (avail > len) {
    memcpy((char *)mctx.block + (sizeof(mctx.block) - avail),
    data, len);
    return 0;
    }
    memcpy((char *)mctx.block + (sizeof(mctx.block) - avail),
    data, avail);
    md4_transform_helper(mctx);
    data += avail;
    len -= avail;
    while (len >= sizeof(mctx.block)) {
    memcpy(mctx.block, data, sizeof(mctx.block));
    md4_transform_helper(mctx);
    data += sizeof(mctx.block);
    len -= sizeof(mctx.block);
    }
    memcpy(mctx.block, data, len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn md4_final(desc: *mut shash_desc, out: *mut u8) -> c_int {
    static int md4_final(struct shash_desc *desc, u8 *out)
    {
    struct md4_ctx *mctx = shash_desc_ctx(desc);
    let mut offset: c_uint = mctx.byte_count & 0x3f;
    char *p = (char *)mctx.block + offset;
    let mut padding: c_int = 56 - (offset + 1);
// p++ = 0x80;
    if (padding < 0) {
    memset(p, 0x00, padding + sizeof (u64));
    md4_transform_helper(mctx);
    p = (char *)mctx.block;
    padding = 56;
    }
    memset(p, 0, padding);
    mctx.block[14] = mctx.byte_count << 3;
    mctx.block[15] = mctx.byte_count >> 29;
    le32_to_cpu_array(mctx.block, (sizeof(mctx.block) -
    sizeof(u64)) / sizeof(u32));
    md4_transform(mctx.hash, mctx.block);
    cpu_to_le32_array(mctx.hash, ARRAY_SIZE(mctx.hash));
    memcpy(out, mctx.hash, sizeof(mctx.hash));
    memset(mctx, 0, sizeof(*mctx));
    return 0;
    }
    static struct shash_alg alg = {
    .digestsize	=	MD4_DIGEST_SIZE,
    .init		=	md4_init,
    .update		=	md4_update,
    .final		=	md4_final,
    .descsize	=	sizeof(struct md4_ctx),
    .base		=	{
    .cra_name	 =	"md4",
    .cra_driver_name =	"md4-generic",
    .cra_blocksize	 =	MD4_HMAC_BLOCK_SIZE,
    .cra_module	 =	THIS_MODULE,
    }
    };
#[no_mangle]
unsafe extern "C" fn md4_mod_init() -> int __init {
    static int __init md4_mod_init(void)
    {
    return crypto_register_shash(&alg);
    }
#[no_mangle]
unsafe extern "C" fn md4_mod_fini() -> void __exit {
    static void __exit md4_mod_fini(void)
    {
    crypto_unregister_shash(&alg);
    }
    module_init(md4_mod_init);
    module_exit(md4_mod_fini);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("MD4 Message Digest Algorithm");
    MODULE_ALIAS_CRYPTO("md4");
