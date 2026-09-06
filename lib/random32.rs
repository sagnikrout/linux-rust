//! Automatically rewritten from C to Rust
//! Source: lib/random32.c
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
// This is a maximally equidistributed combined Tausworthe generator
// based on code from GNU Scientific Library 1.5 (30 Jun 2004)
//
// lfsr113 version:
//
// x_n = (s1_n ^ s2_n ^ s3_n ^ s4_n)
//
// s1_{n+1} = (((s1_n & 4294967294) << 18) ^ (((s1_n <<  6) ^ s1_n) >> 13))
// s2_{n+1} = (((s2_n & 4294967288) <<  2) ^ (((s2_n <<  2) ^ s2_n) >> 27))
// s3_{n+1} = (((s3_n & 4294967280) <<  7) ^ (((s3_n << 13) ^ s3_n) >> 21))
// s4_{n+1} = (((s4_n & 4294967168) << 13) ^ (((s4_n <<  3) ^ s4_n) >> 12))
//
// The period of this generator is about 2^113 (see erratum paper).
//
// From: P. L'Ecuyer, "Maximally Equidistributed Combined Tausworthe
// Generators", Mathematics of Computation, 65, 213 (1996), 203--213:
// http://www.iro.umontreal.ca/~lecuyer/myftp/papers/tausme.ps
// ftp://ftp.iro.umontreal.ca/pub/simulation/lecuyer/papers/tausme.ps
//
// There is an erratum in the paper "Tables of Maximally Equidistributed
// Combined LFSR Generators", Mathematics of Computation, 68, 225 (1999),
// 261--269: http://www.iro.umontreal.ca/~lecuyer/myftp/papers/tausme2.ps
//
// ... the k_j most significant bits of z_j must be non-zero,
// for each j. (Note: this restriction also applies to the
// computer code given in [4], but was mistakenly not mentioned
// in that paper.)
//
// This affects the seeding procedure by imposing the requirement
// s1 > 1, s2 > 7, s3 > 15, s4 > 127.
//

//
// prandom_u32_state - seeded pseudo-random number generator.
// @state: pointer to state structure holding seeded state.
//
// This is used for pseudo-randomness with no outside seeding.
// For more random results, use get_random_u32().
//
#[no_mangle]
pub unsafe extern "C" fn prandom_u32_state(state: *mut rnd_state) -> u32 {
    u32 prandom_u32_state(struct rnd_state *state)
    {

    state.s1 = TAUSWORTHE(state.s1,  6U, 13U, 4294967294U, 18U);
    state.s2 = TAUSWORTHE(state.s2,  2U, 27U, 4294967288U,  2U);
    state.s3 = TAUSWORTHE(state.s3, 13U, 21U, 4294967280U,  7U);
    state.s4 = TAUSWORTHE(state.s4,  3U, 12U, 4294967168U, 13U);
    return (state.s1 ^ state.s2 ^ state.s3 ^ state.s4);
    }
    EXPORT_SYMBOL(prandom_u32_state);
//
// prandom_bytes_state - get the requested number of pseudo-random bytes
//
// @state: pointer to state structure holding seeded state.
// @buf: where to copy the pseudo-random bytes to
// @bytes: the requested number of bytes
//
// This is used for pseudo-randomness with no outside seeding.
// For more random results, use get_random_bytes().
//
#[no_mangle]
pub unsafe extern "C" fn prandom_bytes_state(state: *mut rnd_state, buf: *mut c_void, bytes: usize) {
    void prandom_bytes_state(struct rnd_state *state, void *buf, size_t bytes)
    {
    u8 *ptr = buf;
    while (bytes >= sizeof(u32)) {
    put_unaligned(prandom_u32_state(state), (u32 *) ptr);
    ptr += sizeof(u32);
    bytes -= sizeof(u32);
    }
    if (bytes > 0) {
    let mut rem: u32 = prandom_u32_state(state);
    do {
// ptr++ = (u8) rem;
    bytes--;
    rem >>= BITS_PER_BYTE;
    } while (bytes > 0);
    }
    }
    EXPORT_SYMBOL(prandom_bytes_state);
//
// Only declared here so that it has a prototype when made
// non-static for KUnit testing (avoids -Wmissing-prototypes).
//

    void prandom_warmup(struct rnd_state *state);

#[no_mangle]
pub unsafe extern "C" fn prandom_warmup(state: *mut rnd_state) -> VISIBLE_IF_KUNIT void {
    VISIBLE_IF_KUNIT void prandom_warmup(struct rnd_state *state)
    {
// Calling RNG ten times to satisfy recurrence condition
    prandom_u32_state(state);
    prandom_u32_state(state);
    prandom_u32_state(state);
    prandom_u32_state(state);
    prandom_u32_state(state);
    prandom_u32_state(state);
    prandom_u32_state(state);
    prandom_u32_state(state);
    prandom_u32_state(state);
    prandom_u32_state(state);
    }
    EXPORT_SYMBOL_IF_KUNIT(prandom_warmup);
#[no_mangle]
pub unsafe extern "C" fn prandom_seed_full_state(pcpu_state: *mut rnd_state __percpu) {
    void prandom_seed_full_state(struct rnd_state __percpu *pcpu_state)
    {
    int i;
    for_each_possible_cpu(i) {
    struct rnd_state *state = per_cpu_ptr(pcpu_state, i);
    u32 seeds[4];
    get_random_bytes(&seeds, sizeof(seeds));
    state.s1 = __seed(seeds[0],   2U);
    state.s2 = __seed(seeds[1],   8U);
    state.s3 = __seed(seeds[2],  16U);
    state.s4 = __seed(seeds[3], 128U);
    prandom_warmup(state);
    }
    }
    EXPORT_SYMBOL(prandom_seed_full_state);
