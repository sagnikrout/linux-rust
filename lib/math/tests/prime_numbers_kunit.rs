//! Automatically rewritten from C to Rust
//! Source: lib/math/tests/prime_numbers_kunit.c
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


// SPDX-License-Identifier: GPL-2.0-only

#[no_mangle]
unsafe extern "C" fn dump_primes(ctx: *mut c_void, p: *const primes) {
    static void dump_primes(void *ctx, const struct primes *p)
    {
    struct kunit_suite *suite = ctx;
    kunit_info(suite, "primes.{last=%lu, .sz=%lu, .primes[]=...x%lx} = %*pbl",
    p.last, p.sz, p.primes[BITS_TO_LONGS(p.sz) - 1], (int)p.sz, p.primes);
    }
#[no_mangle]
unsafe extern "C" fn prime_numbers_test(test: *mut kunit) {
    static void prime_numbers_test(struct kunit *test)
    {
    let mut max: c_ulong = 65536;
    unsigned long x, last, next;
    for (last = 0, x = 2; x < max; x++) {
    let mut slow: bool = slow_is_prime_number(x);
    let mut fast: bool = is_prime_number(x);
    KUNIT_ASSERT_EQ_MSG(test, slow, fast, "is-prime(%lu)", x);
    if (!slow)
    continue;
    next = next_prime_number(last);
    KUNIT_ASSERT_EQ_MSG(test, next, x, "next-prime(%lu)", last);
    last = next;
    }
    }
#[no_mangle]
unsafe extern "C" fn kunit_suite_exit(suite: *mut kunit_suite) {
    static void kunit_suite_exit(struct kunit_suite *suite)
    {
    with_primes(suite, dump_primes);
    }
    static struct kunit_case prime_numbers_cases[] = {
    KUNIT_CASE(prime_numbers_test),
    {},
    };
    static struct kunit_suite prime_numbers_suite = {
    .name = "math-prime_numbers",
    .suite_exit = kunit_suite_exit,
    .test_cases = prime_numbers_cases,
    };
    kunit_test_suite(prime_numbers_suite);
    MODULE_AUTHOR("Intel Corporation");
    MODULE_DESCRIPTION("Prime number library");
    MODULE_LICENSE("GPL");
