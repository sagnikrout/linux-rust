//! Automatically rewritten from C to Rust
//! Source: lib/crypto/sha1.c
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
//
// SHA-1 and HMAC-SHA1 library functions
//

    static const struct sha1_block_state sha1_iv = {
    .h = { SHA1_H0, SHA1_H1, SHA1_H2, SHA1_H3, SHA1_H4 },
    };
//
// If you have 32 registers or more, the compiler can (and should)
// try to change the array[] accesses into registers. However, on
// machines with less than ~25 registers, that won't really work,
// and at least gcc will make an unholy mess of it.
//
// So to avoid that mess which just slows things down, we force
// the stores to memory to actually happen (we might be better off
// with a 'W(t)=(val);asm("":"+m" (W(t))' there instead, as
// suggested by Artur Skawina - that will also make gcc unable to
// try to do the silly "optimize away loads" part because it won't
// see what the value will be).
//
// Ben Herrenschmidt reports that on PPC, the C version comes close
// to the optimized asm with this (ie on PPC you don't want that
// 'volatile', since there are lots of registers).
//
// On ARM we get the best code generation by forcing a full memory barrier
// between each SHA_ROUND, otherwise gcc happily get wild with spilling and
// the stack frame size simply explode and performance goes down the drain.
//

// This "rolls" over the 512-bit array

//
// Where do we get the source from? The first 16 iterations get it from
// the input data, the next mix it from the 512-bit array.
//

    __u32 TEMP = input(t); setW(t, TEMP); \
    E += TEMP + rol32(A,5) + (fn) + (constant); \
    B = ror32(B, 2); \
    TEMP = E; E = D; D = C; C = B; B = A; A = TEMP; } while (0)

pub const SHA1_WORKSPACE_WORDS: c_int = 16;
    static void sha1_block_generic(struct sha1_block_state *state,
    const u8 data[SHA1_BLOCK_SIZE],
    u32 workspace[SHA1_WORKSPACE_WORDS])
    {
    __u32 A, B, C, D, E;
    let mut i: c_uint = 0;
    A = state.h[0];
    B = state.h[1];
    C = state.h[2];
    D = state.h[3];
    E = state.h[4];
// Round 1 - iterations 0-16 take their input from 'data'
    for (; i < 16; ++i)
    T_0_15(i, A, B, C, D, E);
// Round 1 - tail. Input from 512-bit mixing array
    for (; i < 20; ++i)
    T_16_19(i, A, B, C, D, E);
// Round 2
    for (; i < 40; ++i)
    T_20_39(i, A, B, C, D, E);
// Round 3
    for (; i < 60; ++i)
    T_40_59(i, A, B, C, D, E);
// Round 4
    for (; i < 80; ++i)
    T_60_79(i, A, B, C, D, E);
    state.h[0] += A;
    state.h[1] += B;
    state.h[2] += C;
    state.h[3] += D;
    state.h[4] += E;
    }
    static void __maybe_unused sha1_blocks_generic(struct sha1_block_state *state,
    const u8 *data, size_t nblocks)
    {
    u32 workspace[SHA1_WORKSPACE_WORDS];
    do {
    sha1_block_generic(state, data, workspace);
    data += SHA1_BLOCK_SIZE;
    } while (--nblocks);
    memzero_explicit(workspace, sizeof(workspace));
    }

#[no_mangle]
pub unsafe extern "C" fn sha1_init(ctx: *mut sha1_ctx) {
    void sha1_init(struct sha1_ctx *ctx)
    {
    ctx.state = sha1_iv;
    ctx.bytecount = 0;
    }
    EXPORT_SYMBOL_GPL(sha1_init);
#[no_mangle]
pub unsafe extern "C" fn sha1_update(ctx: *mut sha1_ctx, data: *const u8, len: usize) {
    void sha1_update(struct sha1_ctx *ctx, const u8 *data, size_t len)
    {
    let mut partial: usize = ctx.bytecount % SHA1_BLOCK_SIZE;
    ctx.bytecount += len;
    if (partial + len >= SHA1_BLOCK_SIZE) {
    size_t nblocks;
    if (partial) {
    let mut l: usize = SHA1_BLOCK_SIZE - partial;
    memcpy(&ctx.buf[partial], data, l);
    data += l;
    len -= l;
    sha1_blocks(&ctx.state, ctx.buf, 1);
    }
    nblocks = len / SHA1_BLOCK_SIZE;
    len %= SHA1_BLOCK_SIZE;
    if (nblocks) {
    sha1_blocks(&ctx.state, data, nblocks);
    data += nblocks * SHA1_BLOCK_SIZE;
    }
    partial = 0;
    }
    if (len)
    memcpy(&ctx.buf[partial], data, len);
    }
    EXPORT_SYMBOL_GPL(sha1_update);
#[no_mangle]
unsafe extern "C" fn __sha1_final(ctx: *mut sha1_ctx, out[SHA1_DIGEST_SIZE]: u8) {
    static void __sha1_final(struct sha1_ctx *ctx, u8 out[SHA1_DIGEST_SIZE])
    {
    let mut bitcount: u64 = ctx.bytecount << 3;
    let mut partial: usize = ctx.bytecount % SHA1_BLOCK_SIZE;
    ctx.buf[partial++] = 0x80;
    if (partial > SHA1_BLOCK_SIZE - 8) {
    memset(&ctx.buf[partial], 0, SHA1_BLOCK_SIZE - partial);
    sha1_blocks(&ctx.state, ctx.buf, 1);
    partial = 0;
    }
    memset(&ctx.buf[partial], 0, SHA1_BLOCK_SIZE - 8 - partial);
// (__be64 *)&ctx->buf[SHA1_BLOCK_SIZE - 8] = cpu_to_be64(bitcount);
    sha1_blocks(&ctx.state, ctx.buf, 1);
    for (size_t i = 0; i < SHA1_DIGEST_SIZE; i += 4)
    put_unaligned_be32(ctx.state.h[i / 4], out + i);
    }
#[no_mangle]
pub unsafe extern "C" fn sha1_final(ctx: *mut sha1_ctx, out[SHA1_DIGEST_SIZE]: u8) {
    void sha1_final(struct sha1_ctx *ctx, u8 out[SHA1_DIGEST_SIZE])
    {
    __sha1_final(ctx, out);
    memzero_explicit(ctx, sizeof(*ctx));
    }
    EXPORT_SYMBOL_GPL(sha1_final);
#[no_mangle]
pub unsafe extern "C" fn sha1(data: *const u8, len: usize, out[SHA1_DIGEST_SIZE]: u8) {
    void sha1(const u8 *data, size_t len, u8 out[SHA1_DIGEST_SIZE])
    {
    struct sha1_ctx ctx;
    sha1_init(&ctx);
    sha1_update(&ctx, data, len);
    sha1_final(&ctx, out);
    }
    EXPORT_SYMBOL_GPL(sha1);
    static void __hmac_sha1_preparekey(struct sha1_block_state *istate,
    struct sha1_block_state *ostate,
    const u8 *raw_key, size_t raw_key_len)
    {
    union {
    u8 b[SHA1_BLOCK_SIZE];
    unsigned long w[SHA1_BLOCK_SIZE / sizeof(unsigned long)];
    } derived_key = { 0 };
    if (unlikely(raw_key_len > SHA1_BLOCK_SIZE))
    sha1(raw_key, raw_key_len, derived_key.b);
    else
    memcpy(derived_key.b, raw_key, raw_key_len);
    for (size_t i = 0; i < ARRAY_SIZE(derived_key.w); i++)
    derived_key.w[i] ^= REPEAT_BYTE(HMAC_IPAD_VALUE);
// istate = sha1_iv;
    sha1_blocks(istate, derived_key.b, 1);
    for (size_t i = 0; i < ARRAY_SIZE(derived_key.w); i++)
    derived_key.w[i] ^= REPEAT_BYTE(HMAC_OPAD_VALUE ^
    HMAC_IPAD_VALUE);
// ostate = sha1_iv;
    sha1_blocks(ostate, derived_key.b, 1);
    memzero_explicit(&derived_key, sizeof(derived_key));
    }
    void hmac_sha1_preparekey(struct hmac_sha1_key *key,
    const u8 *raw_key, size_t raw_key_len)
    {
    __hmac_sha1_preparekey(&key.istate, &key.ostate,
    raw_key, raw_key_len);
    }
    EXPORT_SYMBOL_GPL(hmac_sha1_preparekey);
#[no_mangle]
pub unsafe extern "C" fn hmac_sha1_init(ctx: *mut hmac_sha1_ctx, key: *const hmac_sha1_key) {
    void hmac_sha1_init(struct hmac_sha1_ctx *ctx, const struct hmac_sha1_key *key)
    {
    ctx.sha_ctx.state = key.istate;
    ctx.sha_ctx.bytecount = SHA1_BLOCK_SIZE;
    ctx.ostate = key.ostate;
    }
    EXPORT_SYMBOL_GPL(hmac_sha1_init);
    void hmac_sha1_init_usingrawkey(struct hmac_sha1_ctx *ctx,
    const u8 *raw_key, size_t raw_key_len)
    {
    __hmac_sha1_preparekey(&ctx.sha_ctx.state, &ctx.ostate,
    raw_key, raw_key_len);
    ctx.sha_ctx.bytecount = SHA1_BLOCK_SIZE;
    }
    EXPORT_SYMBOL_GPL(hmac_sha1_init_usingrawkey);
#[no_mangle]
pub unsafe extern "C" fn hmac_sha1_final(ctx: *mut hmac_sha1_ctx, out[SHA1_DIGEST_SIZE]: u8) {
    void hmac_sha1_final(struct hmac_sha1_ctx *ctx, u8 out[SHA1_DIGEST_SIZE])
    {
// Generate the padded input for the outer hash in ctx->sha_ctx.buf.
    __sha1_final(&ctx.sha_ctx, ctx.sha_ctx.buf);
    memset(&ctx.sha_ctx.buf[SHA1_DIGEST_SIZE], 0,
    SHA1_BLOCK_SIZE - SHA1_DIGEST_SIZE);
    ctx.sha_ctx.buf[SHA1_DIGEST_SIZE] = 0x80;
// (__be32 *)&ctx->sha_ctx.buf[SHA1_BLOCK_SIZE - 4] =
    cpu_to_be32(8 * (SHA1_BLOCK_SIZE + SHA1_DIGEST_SIZE));
// Compute the outer hash, which gives the HMAC value.
    sha1_blocks(&ctx.ostate, ctx.sha_ctx.buf, 1);
    for (size_t i = 0; i < SHA1_DIGEST_SIZE; i += 4)
    put_unaligned_be32(ctx.ostate.h[i / 4], out + i);
    memzero_explicit(ctx, sizeof(*ctx));
    }
    EXPORT_SYMBOL_GPL(hmac_sha1_final);
    void hmac_sha1(const struct hmac_sha1_key *key,
    const u8 *data, size_t data_len, u8 out[SHA1_DIGEST_SIZE])
    {
    struct hmac_sha1_ctx ctx;
    hmac_sha1_init(&ctx, key);
    hmac_sha1_update(&ctx, data, data_len);
    hmac_sha1_final(&ctx, out);
    }
    EXPORT_SYMBOL_GPL(hmac_sha1);
    void hmac_sha1_usingrawkey(const u8 *raw_key, size_t raw_key_len,
    const u8 *data, size_t data_len,
    u8 out[SHA1_DIGEST_SIZE])
    {
    struct hmac_sha1_ctx ctx;
    hmac_sha1_init_usingrawkey(&ctx, raw_key, raw_key_len);
    hmac_sha1_update(&ctx, data, data_len);
    hmac_sha1_final(&ctx, out);
    }
    EXPORT_SYMBOL_GPL(hmac_sha1_usingrawkey);

#[no_mangle]
unsafe extern "C" fn sha1_mod_init() -> int __init {
    static int __init sha1_mod_init(void)
    {

    sha1_mod_init_arch();

    if (fips_enabled) {
//
// FIPS cryptographic algorithm self-test.  As per the FIPS
// Implementation Guidance, testing HMAC-SHA1 satisfies the test
// requirement for SHA-1 too.
//
    u8 mac[SHA1_DIGEST_SIZE];
    hmac_sha1_usingrawkey(fips_test_key, sizeof(fips_test_key),
    fips_test_data, sizeof(fips_test_data),
    mac);
    if (memcmp(fips_test_hmac_sha1_value, mac, sizeof(mac)) != 0)
    panic("sha1: FIPS self-test failed\n");
    }
    return 0;
    }
    subsys_initcall(sha1_mod_init);
#[no_mangle]
unsafe extern "C" fn sha1_mod_exit() -> void __exit {
    static void __exit sha1_mod_exit(void)
    {
    }
    module_exit(sha1_mod_exit);

    MODULE_DESCRIPTION("SHA-1 and HMAC-SHA1 library functions");
    MODULE_LICENSE("GPL");
