//! Automatically rewritten from C to Rust
//! Source: lib/xxhash.c
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
// xxHash - Extremely Fast Hash algorithm
// Copyright (C) 2012-2016, Yann Collet.
//
// BSD 2-Clause License (http://www.opensource.org/licenses/bsd-license.php)
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// This program is free software; you can redistribute it and/or modify it under
// the terms of the GNU General Public License version 2 as published by the
// Free Software Foundation. This program is dual-licensed; you may select
// either version 2 of the GNU General Public License ("GPL") or BSD license
// ("BSD").
//
// You can contact the author at:
// - xxHash homepage: https://cyan4973.github.io/xxHash
// - xxHash source repository: https://github.com/Cyan4973/xxHash
//

// -
// Macros
//

// -
// Constants
//
    let mut PRIME32_1: static uint32_t = 2654435761U;
    let mut PRIME32_2: static uint32_t = 2246822519U;
    let mut PRIME32_3: static uint32_t = 3266489917U;
    let mut PRIME32_4: static uint32_t = 668265263U;
    let mut PRIME32_5: static uint32_t = 374761393U;
    let mut PRIME64_1: static uint64_t = 11400714785074694791ULL;
    let mut PRIME64_2: static uint64_t = 14029467366897019727ULL;
    let mut PRIME64_3: static uint64_t = 1609587929392839161ULL;
    let mut PRIME64_4: static uint64_t = 9650029242287828579ULL;
    let mut PRIME64_5: static uint64_t = 2870177450012600261ULL;
// -
// Simple Hash Functions
//
#[no_mangle]
unsafe extern "C" fn xxh32_round(seed: u32, input: u32) -> u32 {
    static uint32_t xxh32_round(uint32_t seed, const uint32_t input)
    {
    seed += input * PRIME32_2;
    seed = xxh_rotl32(seed, 13);
    seed *= PRIME32_1;
    return seed;
    }
#[no_mangle]
pub unsafe extern "C" fn xxh32(input: *const c_void, len: usize, seed: u32) -> u32 {
    uint32_t xxh32(const void *input, const size_t len, const uint32_t seed)
    {
    const uint8_t *p = (const uint8_t *)input;
    const uint8_t *b_end = p + len;
    uint32_t h32;
    if (len >= 16) {
    let mut limit: *const uint8_t const = b_end - 16;
    let mut v1: u32 = seed + PRIME32_1 + PRIME32_2;
    let mut v2: u32 = seed + PRIME32_2;
    let mut v3: u32 = seed + 0;
    let mut v4: u32 = seed - PRIME32_1;
    do {
    v1 = xxh32_round(v1, get_unaligned_le32(p));
    p += 4;
    v2 = xxh32_round(v2, get_unaligned_le32(p));
    p += 4;
    v3 = xxh32_round(v3, get_unaligned_le32(p));
    p += 4;
    v4 = xxh32_round(v4, get_unaligned_le32(p));
    p += 4;
    } while (p <= limit);
    h32 = xxh_rotl32(v1, 1) + xxh_rotl32(v2, 7) +
    xxh_rotl32(v3, 12) + xxh_rotl32(v4, 18);
    } else {
    h32 = seed + PRIME32_5;
    }
    h32 += (uint32_t)len;
    while (p + 4 <= b_end) {
    h32 += get_unaligned_le32(p) * PRIME32_3;
    h32 = xxh_rotl32(h32, 17) * PRIME32_4;
    p += 4;
    }
    while (p < b_end) {
    h32 += (*p) * PRIME32_5;
    h32 = xxh_rotl32(h32, 11) * PRIME32_1;
    p++;
    }
    h32 ^= h32 >> 15;
    h32 *= PRIME32_2;
    h32 ^= h32 >> 13;
    h32 *= PRIME32_3;
    h32 ^= h32 >> 16;
    return h32;
    }
    EXPORT_SYMBOL(xxh32);
#[no_mangle]
unsafe extern "C" fn xxh64_round(acc: u64, input: u64) -> u64 {
    static uint64_t xxh64_round(uint64_t acc, const uint64_t input)
    {
    acc += input * PRIME64_2;
    acc = xxh_rotl64(acc, 31);
    acc *= PRIME64_1;
    return acc;
    }
#[no_mangle]
unsafe extern "C" fn xxh64_merge_round(acc: u64, val: u64) -> u64 {
    static uint64_t xxh64_merge_round(uint64_t acc, uint64_t val)
    {
    val = xxh64_round(0, val);
    acc ^= val;
    acc = acc * PRIME64_1 + PRIME64_4;
    return acc;
    }
#[no_mangle]
pub unsafe extern "C" fn xxh64(input: *const c_void, len: usize, seed: u64) -> u64 {
    uint64_t xxh64(const void *input, const size_t len, const uint64_t seed)
    {
    const uint8_t *p = (const uint8_t *)input;
    let mut b_end: *const uint8_t const = p + len;
    uint64_t h64;
    if (len >= 32) {
    let mut limit: *const uint8_t const = b_end - 32;
    let mut v1: u64 = seed + PRIME64_1 + PRIME64_2;
    let mut v2: u64 = seed + PRIME64_2;
    let mut v3: u64 = seed + 0;
    let mut v4: u64 = seed - PRIME64_1;
    do {
    v1 = xxh64_round(v1, get_unaligned_le64(p));
    p += 8;
    v2 = xxh64_round(v2, get_unaligned_le64(p));
    p += 8;
    v3 = xxh64_round(v3, get_unaligned_le64(p));
    p += 8;
    v4 = xxh64_round(v4, get_unaligned_le64(p));
    p += 8;
    } while (p <= limit);
    h64 = xxh_rotl64(v1, 1) + xxh_rotl64(v2, 7) +
    xxh_rotl64(v3, 12) + xxh_rotl64(v4, 18);
    h64 = xxh64_merge_round(h64, v1);
    h64 = xxh64_merge_round(h64, v2);
    h64 = xxh64_merge_round(h64, v3);
    h64 = xxh64_merge_round(h64, v4);
    } else {
    h64  = seed + PRIME64_5;
    }
    h64 += (uint64_t)len;
    while (p + 8 <= b_end) {
    let mut k1: u64 = xxh64_round(0, get_unaligned_le64(p));
    h64 ^= k1;
    h64 = xxh_rotl64(h64, 27) * PRIME64_1 + PRIME64_4;
    p += 8;
    }
    if (p + 4 <= b_end) {
    h64 ^= (uint64_t)(get_unaligned_le32(p)) * PRIME64_1;
    h64 = xxh_rotl64(h64, 23) * PRIME64_2 + PRIME64_3;
    p += 4;
    }
    while (p < b_end) {
    h64 ^= (*p) * PRIME64_5;
    h64 = xxh_rotl64(h64, 11) * PRIME64_1;
    p++;
    }
    h64 ^= h64 >> 33;
    h64 *= PRIME64_2;
    h64 ^= h64 >> 29;
    h64 *= PRIME64_3;
    h64 ^= h64 >> 32;
    return h64;
    }
    EXPORT_SYMBOL(xxh64);
// -
// Advanced Hash Functions
//
#[no_mangle]
pub unsafe extern "C" fn xxh64_reset(statePtr: *mut xxh64_state, seed: u64) {
    void xxh64_reset(struct xxh64_state *statePtr, const uint64_t seed)
    {
// use a local state for memcpy() to avoid strict-aliasing warnings
    struct xxh64_state state;
    memset(&state, 0, sizeof(state));
    state.v1 = seed + PRIME64_1 + PRIME64_2;
    state.v2 = seed + PRIME64_2;
    state.v3 = seed + 0;
    state.v4 = seed - PRIME64_1;
    memcpy(statePtr, &state, sizeof(state));
    }
    EXPORT_SYMBOL(xxh64_reset);
#[no_mangle]
pub unsafe extern "C" fn xxh64_update(state: *mut xxh64_state, input: *const c_void, len: usize) -> c_int {
    int xxh64_update(struct xxh64_state *state, const void *input, const size_t len)
    {
    const uint8_t *p = (const uint8_t *)input;
    let mut b_end: *const uint8_t const = p + len;
    if (input == core::ptr::null_mut())
    return -EINVAL;
    state.total_len += len;
    if (state.memsize + len < 32) { /* fill in tmp buffer */
    memcpy(((uint8_t *)state.mem64) + state.memsize, input, len);
    state.memsize += (uint32_t)len;
    return 0;
    }
    if (state.memsize) { /* tmp buffer is full */
    uint64_t *p64 = state.mem64;
    memcpy(((uint8_t *)p64) + state.memsize, input,
    32 - state.memsize);
    state.v1 = xxh64_round(state.v1, get_unaligned_le64(p64));
    p64++;
    state.v2 = xxh64_round(state.v2, get_unaligned_le64(p64));
    p64++;
    state.v3 = xxh64_round(state.v3, get_unaligned_le64(p64));
    p64++;
    state.v4 = xxh64_round(state.v4, get_unaligned_le64(p64));
    p += 32 - state.memsize;
    state.memsize = 0;
    }
    if (p + 32 <= b_end) {
    let mut limit: *const uint8_t const = b_end - 32;
    let mut v1: u64 = state.v1;
    let mut v2: u64 = state.v2;
    let mut v3: u64 = state.v3;
    let mut v4: u64 = state.v4;
    do {
    v1 = xxh64_round(v1, get_unaligned_le64(p));
    p += 8;
    v2 = xxh64_round(v2, get_unaligned_le64(p));
    p += 8;
    v3 = xxh64_round(v3, get_unaligned_le64(p));
    p += 8;
    v4 = xxh64_round(v4, get_unaligned_le64(p));
    p += 8;
    } while (p <= limit);
    state.v1 = v1;
    state.v2 = v2;
    state.v3 = v3;
    state.v4 = v4;
    }
    if (p < b_end) {
    memcpy(state.mem64, p, (size_t)(b_end-p));
    state.memsize = (uint32_t)(b_end - p);
    }
    return 0;
    }
    EXPORT_SYMBOL(xxh64_update);
#[no_mangle]
pub unsafe extern "C" fn xxh64_digest(state: *const xxh64_state) -> u64 {
    uint64_t xxh64_digest(const struct xxh64_state *state)
    {
    const uint8_t *p = (const uint8_t *)state.mem64;
    const uint8_t *const b_end = (const uint8_t *)state.mem64 +
    state.memsize;
    uint64_t h64;
    if (state.total_len >= 32) {
    let mut v1: u64 = state.v1;
    let mut v2: u64 = state.v2;
    let mut v3: u64 = state.v3;
    let mut v4: u64 = state.v4;
    h64 = xxh_rotl64(v1, 1) + xxh_rotl64(v2, 7) +
    xxh_rotl64(v3, 12) + xxh_rotl64(v4, 18);
    h64 = xxh64_merge_round(h64, v1);
    h64 = xxh64_merge_round(h64, v2);
    h64 = xxh64_merge_round(h64, v3);
    h64 = xxh64_merge_round(h64, v4);
    } else {
    h64  = state.v3 + PRIME64_5;
    }
    h64 += (uint64_t)state.total_len;
    while (p + 8 <= b_end) {
    let mut k1: u64 = xxh64_round(0, get_unaligned_le64(p));
    h64 ^= k1;
    h64 = xxh_rotl64(h64, 27) * PRIME64_1 + PRIME64_4;
    p += 8;
    }
    if (p + 4 <= b_end) {
    h64 ^= (uint64_t)(get_unaligned_le32(p)) * PRIME64_1;
    h64 = xxh_rotl64(h64, 23) * PRIME64_2 + PRIME64_3;
    p += 4;
    }
    while (p < b_end) {
    h64 ^= (*p) * PRIME64_5;
    h64 = xxh_rotl64(h64, 11) * PRIME64_1;
    p++;
    }
    h64 ^= h64 >> 33;
    h64 *= PRIME64_2;
    h64 ^= h64 >> 29;
    h64 *= PRIME64_3;
    h64 ^= h64 >> 32;
    return h64;
    }
    EXPORT_SYMBOL(xxh64_digest);
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_DESCRIPTION("xxHash");
