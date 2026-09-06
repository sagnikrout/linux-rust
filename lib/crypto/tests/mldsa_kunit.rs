//! Automatically rewritten from C to Rust
//! Source: lib/crypto/tests/mldsa_kunit.c
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
// KUnit tests and benchmark for ML-DSA
//
// Copyright 2025 Google LLC
//

// ML-DSA parameters that the tests use
    static const struct {
    int sig_len;
    int pk_len;
    int k;
    int lambda;
    int gamma1;
    int beta;
    int omega;
    } params[] = {
    [MLDSA44] = {
    .sig_len = MLDSA44_SIGNATURE_SIZE,
    .pk_len = MLDSA44_PUBLIC_KEY_SIZE,
    .k = 4,
    .lambda = 128,
    .gamma1 = 1 << 17,
    .beta = 78,
    .omega = 80,
    },
    [MLDSA65] = {
    .sig_len = MLDSA65_SIGNATURE_SIZE,
    .pk_len = MLDSA65_PUBLIC_KEY_SIZE,
    .k = 6,
    .lambda = 192,
    .gamma1 = 1 << 19,
    .beta = 196,
    .omega = 55,
    },
    [MLDSA87] = {
    .sig_len = MLDSA87_SIGNATURE_SIZE,
    .pk_len = MLDSA87_PUBLIC_KEY_SIZE,
    .k = 8,
    .lambda = 256,
    .gamma1 = 1 << 19,
    .beta = 120,
    .omega = 75,
    },
    };

    static void do_mldsa_and_assert_success(struct kunit *test,
    const struct mldsa_testvector *tv)
    {
    int err = mldsa_verify(tv.alg, tv.sig, tv.sig_len, tv.msg,
    tv.msg_len, tv.pk, tv.pk_len);
    KUNIT_ASSERT_EQ(test, err, 0);
    }
//
// Test that changing coefficients in a valid signature's z vector results in
// the following behavior from mldsa_verify():
//
// * -EBADMSG if a coefficient is changed to have an out-of-range value, i.e.
// absolute value >= gamma1 - beta, corresponding to the verifier detecting
// the out-of-range coefficient and rejecting the signature as malformed
//
// * -EKEYREJECTED if a coefficient is changed to a different in-range value,
// i.e. absolute value < gamma1 - beta, corresponding to the verifier
// continuing to the "real" signature check and that check failing
//
    static void test_mldsa_z_range(struct kunit *test,
    const struct mldsa_testvector *tv)
    {
    u8 *sig = memdup_buf(test, tv.sig, tv.sig_len);
    let mut lambda: c_int = params[tv.alg].lambda;
    let mut gamma1: i32 = params[tv.alg].gamma1;
    let mut beta: c_int = params[tv.alg].beta;
//
// We just modify the first coefficient.  The coefficient is gamma1
// minus either the first 18 or 20 bits of the u32, depending on gamma1.
//
// The layout of ML-DSA signatures is ctilde || z || h.  ctilde is
// lambda / 4 bytes, so z starts at &sig[lambda / 4].
//
    u8 *z_ptr = &sig[lambda / 4];
    let mut z_data: u32 = get_unaligned_le32(z_ptr);
    let mut mask: u32 = (gamma1 << 1) - 1;
// These are the four boundaries of the out-of-range values.
    const s32 out_of_range_coeffs[] = {
    -gamma1 + 1,
    -(gamma1 - beta),
    gamma1,
    gamma1 - beta,
    };
//
// These are the two boundaries of the valid range, along with 0.  We
// assume that none of these matches the original coefficient.
//
    const s32 in_range_coeffs[] = {
    -(gamma1 - beta - 1),
    0,
    gamma1 - beta - 1,
    };
// Initially the signature is valid.
    do_mldsa_and_assert_success(test, tv);
// Test some out-of-range coefficients.
    for (int i = 0; i < ARRAY_SIZE(out_of_range_coeffs); i++) {
    let mut c: i32 = out_of_range_coeffs[i];
    put_unaligned_le32((z_data & ~mask) | (mask & (gamma1 - c)),
    z_ptr);
    KUNIT_ASSERT_EQ(test, -EBADMSG,
    mldsa_verify(tv.alg, sig, tv.sig_len, tv.msg,
    tv.msg_len, tv.pk, tv.pk_len));
    }
// Test some in-range coefficients.
    for (int i = 0; i < ARRAY_SIZE(in_range_coeffs); i++) {
    let mut c: i32 = in_range_coeffs[i];
    put_unaligned_le32((z_data & ~mask) | (mask & (gamma1 - c)),
    z_ptr);
    KUNIT_ASSERT_EQ(test, -EKEYREJECTED,
    mldsa_verify(tv.alg, sig, tv.sig_len, tv.msg,
    tv.msg_len, tv.pk, tv.pk_len));
    }
    }
// Test that mldsa_verify() rejects malformed hint vectors with -EBADMSG.
    static void test_mldsa_bad_hints(struct kunit *test,
    const struct mldsa_testvector *tv)
    {
    let mut omega: c_int = params[tv.alg].omega;
    let mut k: c_int = params[tv.alg].k;
    u8 *sig = memdup_buf(test, tv.sig, tv.sig_len);
// Pointer to the encoded hint vector in the signature
    u8 *hintvec = &sig[tv.sig_len - omega - k];
    u8 h;
// Initially the signature is valid.
    do_mldsa_and_assert_success(test, tv);
// Cumulative hint count exceeds omega
    memcpy(sig, tv.sig, tv.sig_len);
    hintvec[omega + k - 1] = omega + 1;
    KUNIT_ASSERT_EQ(test, -EBADMSG,
    mldsa_verify(tv.alg, sig, tv.sig_len, tv.msg,
    tv.msg_len, tv.pk, tv.pk_len));
// Cumulative hint count decreases
    memcpy(sig, tv.sig, tv.sig_len);
    KUNIT_ASSERT_GE(test, hintvec[omega + k - 2], 1);
    hintvec[omega + k - 1] = hintvec[omega + k - 2] - 1;
    KUNIT_ASSERT_EQ(test, -EBADMSG,
    mldsa_verify(tv.alg, sig, tv.sig_len, tv.msg,
    tv.msg_len, tv.pk, tv.pk_len));
//
// Hint indices out of order.  To test this, swap hintvec[0] and
// hintvec[1].  This assumes that the original valid signature had at
// least two nonzero hints in the first element (asserted below).
//
    memcpy(sig, tv.sig, tv.sig_len);
    KUNIT_ASSERT_GE(test, hintvec[omega], 2);
    h = hintvec[0];
    hintvec[0] = hintvec[1];
    hintvec[1] = h;
    KUNIT_ASSERT_EQ(test, -EBADMSG,
    mldsa_verify(tv.alg, sig, tv.sig_len, tv.msg,
    tv.msg_len, tv.pk, tv.pk_len));
//
// Extra hint indices given.  For this test to work, the original valid
// signature must have fewer than omega nonzero hints (asserted below).
//
    memcpy(sig, tv.sig, tv.sig_len);
    KUNIT_ASSERT_LT(test, hintvec[omega + k - 1], omega);
    hintvec[omega - 1] = 0xff;
    KUNIT_ASSERT_EQ(test, -EBADMSG,
    mldsa_verify(tv.alg, sig, tv.sig_len, tv.msg,
    tv.msg_len, tv.pk, tv.pk_len));
    }
    static void test_mldsa_mutation(struct kunit *test,
    const struct mldsa_testvector *tv)
    {
    let mut sig_len: c_int = tv.sig_len;
    let mut msg_len: c_int = tv.msg_len;
    let mut pk_len: c_int = tv.pk_len;
    let mut num_iter: c_int = 200;
    u8 *sig = memdup_buf(test, tv.sig, sig_len);
    u8 *msg = memdup_buf(test, tv.msg, msg_len);
    u8 *pk = memdup_buf(test, tv.pk, pk_len);
// Initially the signature is valid.
    do_mldsa_and_assert_success(test, tv);
// Changing any bit in the signature should invalidate the signature
    for (int i = 0; i < num_iter; i++) {
    let mut pos: usize = get_random_u32_below(sig_len);
    let mut b: u8 = 1 << get_random_u32_below(8);
    sig[pos] ^= b;
    KUNIT_ASSERT_NE(test, 0,
    mldsa_verify(tv.alg, sig, sig_len, msg,
    msg_len, pk, pk_len));
    sig[pos] ^= b;
    }
// Changing any bit in the message should invalidate the signature
    for (int i = 0; i < num_iter; i++) {
    let mut pos: usize = get_random_u32_below(msg_len);
    let mut b: u8 = 1 << get_random_u32_below(8);
    msg[pos] ^= b;
    KUNIT_ASSERT_NE(test, 0,
    mldsa_verify(tv.alg, sig, sig_len, msg,
    msg_len, pk, pk_len));
    msg[pos] ^= b;
    }
// Changing any bit in the public key should invalidate the signature
    for (int i = 0; i < num_iter; i++) {
    let mut pos: usize = get_random_u32_below(pk_len);
    let mut b: u8 = 1 << get_random_u32_below(8);
    pk[pos] ^= b;
    KUNIT_ASSERT_NE(test, 0,
    mldsa_verify(tv.alg, sig, sig_len, msg,
    msg_len, pk, pk_len));
    pk[pos] ^= b;
    }
// All changes should have been undone.
    KUNIT_ASSERT_EQ(test, 0,
    mldsa_verify(tv.alg, sig, sig_len, msg, msg_len, pk,
    pk_len));
    }
#[no_mangle]
unsafe extern "C" fn test_mldsa(test: *mut kunit, tv: *const mldsa_testvector) {
    static void test_mldsa(struct kunit *test, const struct mldsa_testvector *tv)
    {
// Valid signature
    KUNIT_ASSERT_EQ(test, tv.sig_len, params[tv.alg].sig_len);
    KUNIT_ASSERT_EQ(test, tv.pk_len, params[tv.alg].pk_len);
    do_mldsa_and_assert_success(test, tv);
// Signature too short
    KUNIT_ASSERT_EQ(test, -EBADMSG,
    mldsa_verify(tv.alg, tv.sig, tv.sig_len - 1, tv.msg,
    tv.msg_len, tv.pk, tv.pk_len));
// Signature too long
    KUNIT_ASSERT_EQ(test, -EBADMSG,
    mldsa_verify(tv.alg, tv.sig, tv.sig_len + 1, tv.msg,
    tv.msg_len, tv.pk, tv.pk_len));
// Public key too short
    KUNIT_ASSERT_EQ(test, -EBADMSG,
    mldsa_verify(tv.alg, tv.sig, tv.sig_len, tv.msg,
    tv.msg_len, tv.pk, tv.pk_len - 1));
// Public key too long
    KUNIT_ASSERT_EQ(test, -EBADMSG,
    mldsa_verify(tv.alg, tv.sig, tv.sig_len, tv.msg,
    tv.msg_len, tv.pk, tv.pk_len + 1));
//
// Message too short.  Error is EKEYREJECTED because it gets rejected by
// the "real" signature check rather than the well-formedness checks.
//
    KUNIT_ASSERT_EQ(test, -EKEYREJECTED,
    mldsa_verify(tv.alg, tv.sig, tv.sig_len, tv.msg,
    tv.msg_len - 1, tv.pk, tv.pk_len));
//
// Can't simply try (tv->msg, tv->msg_len + 1) too, as tv->msg would be
// accessed out of bounds.  However, ML-DSA just hashes the message and
// doesn't handle different message lengths differently anyway.
//
// Test the validity checks on the z vector.
    test_mldsa_z_range(test, tv);
// Test the validity checks on the hint vector.
    test_mldsa_bad_hints(test, tv);
// Test randomly mutating the inputs.
    test_mldsa_mutation(test, tv);
    }
#[no_mangle]
unsafe extern "C" fn test_mldsa44(test: *mut kunit) {
    static void test_mldsa44(struct kunit *test)
    {
    test_mldsa(test, &mldsa44_testvector);
    }
#[no_mangle]
unsafe extern "C" fn test_mldsa65(test: *mut kunit) {
    static void test_mldsa65(struct kunit *test)
    {
    test_mldsa(test, &mldsa65_testvector);
    }
#[no_mangle]
unsafe extern "C" fn test_mldsa87(test: *mut kunit) {
    static void test_mldsa87(struct kunit *test)
    {
    test_mldsa(test, &mldsa87_testvector);
    }
#[no_mangle]
unsafe extern "C" fn mod(a: i32, m: i32) -> i32 {
    static s32 mod(s32 a, s32 m)
    {
    a %= m;
    if (a < 0)
    a += m;
    return a;
    }
#[no_mangle]
unsafe extern "C" fn symmetric_mod(a: i32, m: i32) -> i32 {
    static s32 symmetric_mod(s32 a, s32 m)
    {
    a = mod(a, m);
    if (a > m / 2)
    a -= m;
    return a;
    }
// Mechanical, inefficient translation of FIPS 204 Algorithm 36, Decompose
#[no_mangle]
unsafe extern "C" fn decompose_ref(r: i32, gamma2: i32, r0: *mut i32, r1: *mut i32) {
    static void decompose_ref(s32 r, s32 gamma2, s32 *r0, s32 *r1)
    {
    let mut rplus: i32 = mod(r, Q);
// r0 = symmetric_mod(rplus, 2 * gamma2);
    if (rplus - *r0 == Q - 1) {
// r1 = 0;
// r0 = *r0 - 1;
    } else {
// r1 = (rplus - *r0) / (2 * gamma2);
    }
    }
// Mechanical, inefficient translation of FIPS 204 Algorithm 40, UseHint
#[no_mangle]
unsafe extern "C" fn use_hint_ref(h: u8, r: i32, gamma2: i32) -> i32 {
    static s32 use_hint_ref(u8 h, s32 r, s32 gamma2)
    {
    let mut m: i32 = (Q - 1) / (2 * gamma2);
    s32 r0, r1;
    decompose_ref(r, gamma2, &r0, &r1);
    if (h == 1 && r0 > 0)
    return mod(r1 + 1, m);
    if (h == 1 && r0 <= 0)
    return mod(r1 - 1, m);
    return r1;
    }
//
// Test that for all possible inputs, mldsa_use_hint() gives the same output as
// a mechanical translation of the pseudocode from FIPS 204.
//
#[no_mangle]
unsafe extern "C" fn test_mldsa_use_hint(test: *mut kunit) {
    static void test_mldsa_use_hint(struct kunit *test)
    {
    for (int i = 0; i < 2; i++) {
    let mut gamma2: i32 = (Q - 1) / (i == 0 ? 88 : 32);
    for (u8 h = 0; h < 2; h++) {
    for (s32 r = 0; r < Q; r++) {
    KUNIT_ASSERT_EQ(test,
    mldsa_use_hint(h, r, gamma2),
    use_hint_ref(h, r, gamma2));
    }
    }
    }
    }
    static void benchmark_mldsa(struct kunit *test,
    const struct mldsa_testvector *tv)
    {
    let mut warmup_niter: c_int = 200;
    let mut benchmark_niter: c_int = 200;
    u64 t0, t1;
    if (!IS_ENABLED(CONFIG_CRYPTO_LIB_BENCHMARK))
    kunit_skip(test, "not enabled");
    for (int i = 0; i < warmup_niter; i++)
    do_mldsa_and_assert_success(test, tv);
    t0 = ktime_get_ns();
    for (int i = 0; i < benchmark_niter; i++)
    do_mldsa_and_assert_success(test, tv);
    t1 = ktime_get_ns();
    kunit_info(test, "%llu ops/s",
    div64_u64((u64)benchmark_niter * NSEC_PER_SEC,
    t1 - t0 ?: 1));
    }
#[no_mangle]
unsafe extern "C" fn benchmark_mldsa44(test: *mut kunit) {
    static void benchmark_mldsa44(struct kunit *test)
    {
    benchmark_mldsa(test, &mldsa44_testvector);
    }
#[no_mangle]
unsafe extern "C" fn benchmark_mldsa65(test: *mut kunit) {
    static void benchmark_mldsa65(struct kunit *test)
    {
    benchmark_mldsa(test, &mldsa65_testvector);
    }
#[no_mangle]
unsafe extern "C" fn benchmark_mldsa87(test: *mut kunit) {
    static void benchmark_mldsa87(struct kunit *test)
    {
    benchmark_mldsa(test, &mldsa87_testvector);
    }
    static struct kunit_case mldsa_kunit_cases[] = {
    KUNIT_CASE(test_mldsa44),
    KUNIT_CASE(test_mldsa65),
    KUNIT_CASE(test_mldsa87),
    KUNIT_CASE(test_mldsa_use_hint),
    KUNIT_CASE(benchmark_mldsa44),
    KUNIT_CASE(benchmark_mldsa65),
    KUNIT_CASE(benchmark_mldsa87),
    {},
    };
    static struct kunit_suite mldsa_kunit_suite = {
    .name = "mldsa",
    .test_cases = mldsa_kunit_cases,
    };
    kunit_test_suite(mldsa_kunit_suite);
    MODULE_DESCRIPTION("KUnit tests and benchmark for ML-DSA");
    MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
    MODULE_LICENSE("GPL");
