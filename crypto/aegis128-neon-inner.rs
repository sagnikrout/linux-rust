//! Automatically rewritten from C to Rust
//! Source: crypto/aegis128-neon-inner.c
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
// Copyright (C) 2019 Linaro, Ltd. <ard.biesheuvel@linaro.org>
//

pub const AEGIS_BLOCK_SIZE: c_int = 16;

    extern int aegis128_have_aes_insn;
    void *memcpy(void *dest, const void *src, size_t n);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aegis128_state {
    pub v: [uint8x16_t; 5],
}

    extern const uint8_t crypto_aes_sbox[];
#[no_mangle]
unsafe extern "C" fn aegis128_load_state_neon(state: *const c_void) -> aegis128_state {
    static struct aegis128_state aegis128_load_state_neon(const void *state)
    {
    return (struct aegis128_state){ {
    vld1q_u8(state),
    vld1q_u8(state + 16),
    vld1q_u8(state + 32),
    vld1q_u8(state + 48),
    vld1q_u8(state + 64)
    } };
    }
#[no_mangle]
unsafe extern "C" fn aegis128_save_state_neon(st: aegis128_state, state: *mut c_void) {
    static void aegis128_save_state_neon(struct aegis128_state st, void *state)
    {
    vst1q_u8(state, st.v[0]);
    vst1q_u8(state + 16, st.v[1]);
    vst1q_u8(state + 32, st.v[2]);
    vst1q_u8(state + 48, st.v[3]);
    vst1q_u8(state + 64, st.v[4]);
    }
#[no_mangle]
unsafe extern "C" fn __attribute__(_arg: (always_inline)) -> inline {
    static inline __attribute__((always_inline))
#[no_mangle]
pub unsafe extern "C" fn aegis_aes_round(w: uint8x16_t) -> uint8x16_t {
    uint8x16_t aegis_aes_round(uint8x16_t w)
    {
    let mut z: uint8x16_t = {};

    if (!__builtin_expect(aegis128_have_aes_insn, 1)) {
    static const uint8_t shift_rows[] = {
    0x0, 0x5, 0xa, 0xf, 0x4, 0x9, 0xe, 0x3,
    0x8, 0xd, 0x2, 0x7, 0xc, 0x1, 0x6, 0xb,
    };
    static const uint8_t ror32by8[] = {
    0x1, 0x2, 0x3, 0x0, 0x5, 0x6, 0x7, 0x4,
    0x9, 0xa, 0xb, 0x8, 0xd, 0xe, 0xf, 0xc,
    };
    uint8x16_t v;
// shift rows
    w = vqtbl1q_u8(w, vld1q_u8(shift_rows));
// sub bytes

    v = vqtbl4q_u8(vld1q_u8_x4(crypto_aes_sbox), w);
    v = vqtbx4q_u8(v, vld1q_u8_x4(crypto_aes_sbox + 0x40), w - 0x40);
    v = vqtbx4q_u8(v, vld1q_u8_x4(crypto_aes_sbox + 0x80), w - 0x80);
    v = vqtbx4q_u8(v, vld1q_u8_x4(crypto_aes_sbox + 0xc0), w - 0xc0);

    asm("tbl %0.16b, {v16.16b-v19.16b}, %1.16b" : "=w"(v) : "w"(w));
    w -= 0x40;
    asm("tbx %0.16b, {v20.16b-v23.16b}, %1.16b" : "+w"(v) : "w"(w));
    w -= 0x40;
    asm("tbx %0.16b, {v24.16b-v27.16b}, %1.16b" : "+w"(v) : "w"(w));
    w -= 0x40;
    asm("tbx %0.16b, {v28.16b-v31.16b}, %1.16b" : "+w"(v) : "w"(w));

// mix columns
    w = (v << 1) ^ (uint8x16_t)(((int8x16_t)v >> 7) & 0x1b);
    w ^= (uint8x16_t)vrev32q_u16((uint16x8_t)v);
    w ^= vqtbl1q_u8(v ^ w, vld1q_u8(ror32by8));
    return w;
    }

//
// We use inline asm here instead of the vaeseq_u8/vaesmcq_u8 intrinsics
// to force the compiler to issue the aese/aesmc instructions in pairs.
// This is much faster on many cores, where the instruction pair can
// execute in a single cycle.
//
    asm(AES_ROUND : "+w"(w) : "w"(z));
    return w;
    }
#[no_mangle]
unsafe extern "C" fn __attribute__(_arg: (always_inline)) -> inline {
    static inline __attribute__((always_inline))
    struct aegis128_state aegis128_update_neon(struct aegis128_state st,
    uint8x16_t m)
    {
    m       ^= aegis_aes_round(st.v[4]);
    st.v[4] ^= aegis_aes_round(st.v[3]);
    st.v[3] ^= aegis_aes_round(st.v[2]);
    st.v[2] ^= aegis_aes_round(st.v[1]);
    st.v[1] ^= aegis_aes_round(st.v[0]);
    st.v[0] ^= m;
    return st;
    }
#[no_mangle]
unsafe extern "C" fn __attribute__(_arg: (always_inline)) -> inline {
    static inline __attribute__((always_inline))
#[no_mangle]
pub unsafe extern "C" fn preload_sbox() {
    void preload_sbox(void)
    {
    if (!IS_ENABLED(CONFIG_ARM64) ||
    !IS_ENABLED(CONFIG_CC_IS_GCC) ||
    __builtin_expect(aegis128_have_aes_insn, 1))
    return;
    asm("ld1	{v16.16b-v19.16b}, [%0], #64	\n\t"
    "ld1	{v20.16b-v23.16b}, [%0], #64	\n\t"
    "ld1	{v24.16b-v27.16b}, [%0], #64	\n\t"
    "ld1	{v28.16b-v31.16b}, [%0]		\n\t"
    :: "r"(crypto_aes_sbox));
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_aegis128_init_neon(state: *mut c_void, key: *const c_void, iv: *const c_void) {
    void crypto_aegis128_init_neon(void *state, const void *key, const void *iv)
    {
    static const uint8_t const0[] = {
    0x00, 0x01, 0x01, 0x02, 0x03, 0x05, 0x08, 0x0d,
    0x15, 0x22, 0x37, 0x59, 0x90, 0xe9, 0x79, 0x62,
    };
    static const uint8_t const1[] = {
    0xdb, 0x3d, 0x18, 0x55, 0x6d, 0xc2, 0x2f, 0xf1,
    0x20, 0x11, 0x31, 0x42, 0x73, 0xb5, 0x28, 0xdd,
    };
    let mut k: uint8x16_t = vld1q_u8(key);
    let mut kiv: uint8x16_t = k ^ vld1q_u8(iv);
    struct aegis128_state st = {{
    kiv,
    vld1q_u8(const1),
    vld1q_u8(const0),
    k ^ vld1q_u8(const0),
    k ^ vld1q_u8(const1),
    }};
    int i;
    preload_sbox();
    for (i = 0; i < 5; i++) {
    st = aegis128_update_neon(st, k);
    st = aegis128_update_neon(st, kiv);
    }
    aegis128_save_state_neon(st, state);
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_aegis128_update_neon(state: *mut c_void, msg: *const c_void) {
    void crypto_aegis128_update_neon(void *state, const void *msg)
    {
    let mut st: aegis128_state = aegis128_load_state_neon(state);
    preload_sbox();
    st = aegis128_update_neon(st, vld1q_u8(msg));
    aegis128_save_state_neon(st, state);
    }

//
// AArch32 does not provide these intrinsics natively because it does not
// implement the underlying instructions. AArch32 only provides 64-bit
// wide vtbl.8/vtbx.8 instruction, so use those instead.
//
#[no_mangle]
unsafe extern "C" fn vqtbl1q_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    static uint8x16_t vqtbl1q_u8(uint8x16_t a, uint8x16_t b)
    {
    union {
    uint8x16_t	val;
    uint8x8x2_t	pair;
    } __a = { a };
    return vcombine_u8(vtbl2_u8(__a.pair, vget_low_u8(b)),
    vtbl2_u8(__a.pair, vget_high_u8(b)));
    }
#[no_mangle]
unsafe extern "C" fn vqtbx1q_u8(v: uint8x16_t, a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    static uint8x16_t vqtbx1q_u8(uint8x16_t v, uint8x16_t a, uint8x16_t b)
    {
    union {
    uint8x16_t	val;
    uint8x8x2_t	pair;
    } __a = { a };
    return vcombine_u8(vtbx2_u8(vget_low_u8(v), __a.pair, vget_low_u8(b)),
    vtbx2_u8(vget_high_u8(v), __a.pair, vget_high_u8(b)));
    }
#[no_mangle]
unsafe extern "C" fn vminvq_s8(v: int8x16_t) -> i8 {
    static int8_t vminvq_s8(int8x16_t v)
    {
    let mut s: int8x8_t = vpmin_s8(vget_low_s8(v), vget_high_s8(v));
    s = vpmin_s8(s, s);
    s = vpmin_s8(s, s);
    s = vpmin_s8(s, s);
    return vget_lane_s8(s, 0);
    }

    static const uint8_t permute[] __aligned(64) = {
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    };
    void crypto_aegis128_encrypt_chunk_neon(void *state, void *dst, const void *src,
    unsigned int size)
    {
    let mut st: aegis128_state = aegis128_load_state_neon(state);
    let mut short_input: c_int = size < AEGIS_BLOCK_SIZE;
    uint8x16_t msg;
    preload_sbox();
    while (size >= AEGIS_BLOCK_SIZE) {
    let mut s: uint8x16_t = st.v[1] ^ (st.v[2] & st.v[3]) ^ st.v[4];
    msg = vld1q_u8(src);
    st = aegis128_update_neon(st, msg);
    msg ^= s;
    vst1q_u8(dst, msg);
    size -= AEGIS_BLOCK_SIZE;
    src += AEGIS_BLOCK_SIZE;
    dst += AEGIS_BLOCK_SIZE;
    }
    if (size > 0) {
    let mut s: uint8x16_t = st.v[1] ^ (st.v[2] & st.v[3]) ^ st.v[4];
    uint8_t buf[AEGIS_BLOCK_SIZE];
    const void *in = src;
    void *out = dst;
    uint8x16_t m;
    if (__builtin_expect(short_input, 0))
    in = out = memcpy(buf + AEGIS_BLOCK_SIZE - size, src, size);
    m = vqtbl1q_u8(vld1q_u8(in + size - AEGIS_BLOCK_SIZE),
    vld1q_u8(permute + 32 - size));
    st = aegis128_update_neon(st, m);
    vst1q_u8(out + size - AEGIS_BLOCK_SIZE,
    vqtbl1q_u8(m ^ s, vld1q_u8(permute + size)));
    if (__builtin_expect(short_input, 0))
    memcpy(dst, out, size);
    else
    vst1q_u8(out - AEGIS_BLOCK_SIZE, msg);
    }
    aegis128_save_state_neon(st, state);
    }
    void crypto_aegis128_decrypt_chunk_neon(void *state, void *dst, const void *src,
    unsigned int size)
    {
    let mut st: aegis128_state = aegis128_load_state_neon(state);
    let mut short_input: c_int = size < AEGIS_BLOCK_SIZE;
    uint8x16_t msg;
    preload_sbox();
    while (size >= AEGIS_BLOCK_SIZE) {
    msg = vld1q_u8(src) ^ st.v[1] ^ (st.v[2] & st.v[3]) ^ st.v[4];
    st = aegis128_update_neon(st, msg);
    vst1q_u8(dst, msg);
    size -= AEGIS_BLOCK_SIZE;
    src += AEGIS_BLOCK_SIZE;
    dst += AEGIS_BLOCK_SIZE;
    }
    if (size > 0) {
    let mut s: uint8x16_t = st.v[1] ^ (st.v[2] & st.v[3]) ^ st.v[4];
    uint8_t buf[AEGIS_BLOCK_SIZE];
    const void *in = src;
    void *out = dst;
    uint8x16_t m;
    if (__builtin_expect(short_input, 0))
    in = out = memcpy(buf + AEGIS_BLOCK_SIZE - size, src, size);
    m = s ^ vqtbx1q_u8(s, vld1q_u8(in + size - AEGIS_BLOCK_SIZE),
    vld1q_u8(permute + 32 - size));
    st = aegis128_update_neon(st, m);
    vst1q_u8(out + size - AEGIS_BLOCK_SIZE,
    vqtbl1q_u8(m, vld1q_u8(permute + size)));
    if (__builtin_expect(short_input, 0))
    memcpy(dst, out, size);
    else
    vst1q_u8(out - AEGIS_BLOCK_SIZE, msg);
    }
    aegis128_save_state_neon(st, state);
    }
    int crypto_aegis128_final_neon(void *state, void *tag_xor,
    unsigned int assoclen,
    unsigned int cryptlen,
    unsigned int authsize)
    {
    let mut st: aegis128_state = aegis128_load_state_neon(state);
    uint8x16_t v;
    int i;
    preload_sbox();
    v = st.v[3] ^ (uint8x16_t)vcombine_u64(vmov_n_u64(8ULL * assoclen),
    vmov_n_u64(8ULL * cryptlen));
    for (i = 0; i < 7; i++)
    st = aegis128_update_neon(st, v);
    v = st.v[0] ^ st.v[1] ^ st.v[2] ^ st.v[3] ^ st.v[4];
    if (authsize > 0) {
    v = vqtbl1q_u8(~vceqq_u8(v, vld1q_u8(tag_xor)),
    vld1q_u8(permute + authsize));
    return vminvq_s8((int8x16_t)v);
    }
    vst1q_u8(tag_xor, v);
    return 0;
    }
