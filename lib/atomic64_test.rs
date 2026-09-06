//! Automatically rewritten from C to Rust
//! Source: lib/atomic64_test.c
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
// Testsuite for atomic64_t functions
//
// Copyright © 2010  Luca Barbieri
//

    do {								\
    atomic##bit##_set(&v, v0);				\
    r = v0;							\
    atomic##bit##_##op(val, &v);				\
    r c_op val;						\
    WARN(atomic##bit##_read(&v) != r, "%Lx != %Lx\n",	\
    (unsigned long long)atomic##bit##_read(&v),	\
    (unsigned long long)r);				\
    } while (0)
//
// Test for a atomic operation family,
// @test should be a macro accepting parameters (bit, op, ...)
//

    do {						\
    test(bit, op, ##args);		\
    test(bit, op##_acquire, ##args);	\
    test(bit, op##_release, ##args);	\
    test(bit, op##_relaxed, ##args);	\
    } while (0)

    do {								\
    atomic##bit##_set(&v, v0);				\
    r = v0;							\
    r c_op val;						\
    BUG_ON(atomic##bit##_##op(val, &v) != r);		\
    BUG_ON(atomic##bit##_read(&v) != r);			\
    } while (0)

    do {								\
    atomic##bit##_set(&v, v0);				\
    r = v0;							\
    r c_op val;						\
    BUG_ON(atomic##bit##_##op(val, &v) != v0);		\
    BUG_ON(atomic##bit##_read(&v) != r);			\
    } while (0)

    do {								\
    FAMILY_TEST(TEST_RETURN, bit, op, c_op, val);		\
    } while (0)

    do {								\
    FAMILY_TEST(TEST_FETCH, bit, op, c_op, val);		\
    } while (0)

    do {								\
    atomic##bit##_set(&v, init);				\
    BUG_ON(atomic##bit##_##op(&v, ##args) != ret);		\
    BUG_ON(atomic##bit##_read(&v) != expect);		\
    } while (0)

    do {									\
    FAMILY_TEST(TEST_ARGS, bit, xchg, init, init, new, new);	\
    } while (0)

    do {									\
    FAMILY_TEST(TEST_ARGS, bit, cmpxchg, 				\
    init, init, new, init, new);			\
    FAMILY_TEST(TEST_ARGS, bit, cmpxchg,				\
    init, init, init, wrong, new);			\
    } while (0)

    do {							\
    FAMILY_TEST(TEST_ARGS, bit, inc_return,		\
    i, (i) + one, (i) + one);	\
    } while (0)

    do {							\
    FAMILY_TEST(TEST_ARGS, bit, dec_return,		\
    i, (i) - one, (i) - one);	\
    } while (0)
#[no_mangle]
unsafe extern "C" fn test_atomic() -> __init void {
    static __init void test_atomic(void)
    {
    let mut v0: c_int = 0xaaa31337;
    let mut v1: c_int = 0xdeadbeef;
    let mut onestwos: c_int = 0x11112222;
    let mut one: c_int = 1;
    atomic_t v;
    int r;
    TEST(, add, +=, onestwos);
    TEST(, add, +=, -one);
    TEST(, sub, -=, onestwos);
    TEST(, sub, -=, -one);
    TEST(, or, |=, v1);
    TEST(, and, &=, v1);
    TEST(, xor, ^=, v1);
    TEST(, andnot, &= ~, v1);
    RETURN_FAMILY_TEST(, add_return, +=, onestwos);
    RETURN_FAMILY_TEST(, add_return, +=, -one);
    RETURN_FAMILY_TEST(, sub_return, -=, onestwos);
    RETURN_FAMILY_TEST(, sub_return, -=, -one);
    FETCH_FAMILY_TEST(, fetch_add, +=, onestwos);
    FETCH_FAMILY_TEST(, fetch_add, +=, -one);
    FETCH_FAMILY_TEST(, fetch_sub, -=, onestwos);
    FETCH_FAMILY_TEST(, fetch_sub, -=, -one);
    FETCH_FAMILY_TEST(, fetch_or,  |=, v1);
    FETCH_FAMILY_TEST(, fetch_and, &=, v1);
    FETCH_FAMILY_TEST(, fetch_andnot, &= ~, v1);
    FETCH_FAMILY_TEST(, fetch_xor, ^=, v1);
    INC_RETURN_FAMILY_TEST(, v0);
    DEC_RETURN_FAMILY_TEST(, v0);
    XCHG_FAMILY_TEST(, v0, v1);
    CMPXCHG_FAMILY_TEST(, v0, v1, onestwos);
    }

#[no_mangle]
unsafe extern "C" fn test_atomic64() -> __init void {
    static __init void test_atomic64(void)
    {
    let mut v0: c_longlong = 0xaaa31337c001d00dLL;
    let mut v1: c_longlong = 0xdeadbeefdeafcafeLL;
    let mut v2: c_longlong = 0xfaceabadf00df001LL;
    let mut v3: c_longlong = 0x8000000000000000LL;
    let mut onestwos: c_longlong = 0x1111111122222222LL;
    let mut one: c_longlong = 1LL;
    int r_int;
    let mut v: core::sync::atomic::AtomicI64 = ATOMIC64_INIT(v0);
    let mut r: c_longlong = v0;
    BUG_ON(v.counter != r);
    atomic64_set(&v, v1);
    r = v1;
    BUG_ON(v.counter != r);
    BUG_ON(atomic64_read(&v) != r);
    TEST(64, add, +=, onestwos);
    TEST(64, add, +=, -one);
    TEST(64, sub, -=, onestwos);
    TEST(64, sub, -=, -one);
    TEST(64, or, |=, v1);
    TEST(64, and, &=, v1);
    TEST(64, xor, ^=, v1);
    TEST(64, andnot, &= ~, v1);
    RETURN_FAMILY_TEST(64, add_return, +=, onestwos);
    RETURN_FAMILY_TEST(64, add_return, +=, -one);
    RETURN_FAMILY_TEST(64, sub_return, -=, onestwos);
    RETURN_FAMILY_TEST(64, sub_return, -=, -one);
    FETCH_FAMILY_TEST(64, fetch_add, +=, onestwos);
    FETCH_FAMILY_TEST(64, fetch_add, +=, -one);
    FETCH_FAMILY_TEST(64, fetch_sub, -=, onestwos);
    FETCH_FAMILY_TEST(64, fetch_sub, -=, -one);
    FETCH_FAMILY_TEST(64, fetch_or,  |=, v1);
    FETCH_FAMILY_TEST(64, fetch_and, &=, v1);
    FETCH_FAMILY_TEST(64, fetch_andnot, &= ~, v1);
    FETCH_FAMILY_TEST(64, fetch_xor, ^=, v1);
    INIT(v0);
    atomic64_inc(&v);
    r += one;
    BUG_ON(v.counter != r);
    INIT(v0);
    atomic64_dec(&v);
    r -= one;
    BUG_ON(v.counter != r);
    INC_RETURN_FAMILY_TEST(64, v0);
    DEC_RETURN_FAMILY_TEST(64, v0);
    XCHG_FAMILY_TEST(64, v0, v1);
    CMPXCHG_FAMILY_TEST(64, v0, v1, v2);
    INIT(v0);
    BUG_ON(atomic64_add_unless(&v, one, v0));
    BUG_ON(v.counter != r);
    INIT(v0);
    BUG_ON(!atomic64_add_unless(&v, one, v1));
    r += one;
    BUG_ON(v.counter != r);
    INIT(onestwos);
    BUG_ON(atomic64_dec_if_positive(&v) != (onestwos - 1));
    r -= one;
    BUG_ON(v.counter != r);
    INIT(0);
    BUG_ON(atomic64_dec_if_positive(&v) != -one);
    BUG_ON(v.counter != r);
    INIT(-one);
    BUG_ON(atomic64_dec_if_positive(&v) != (-one - one));
    BUG_ON(v.counter != r);
    INIT(onestwos);
    BUG_ON(!atomic64_inc_not_zero(&v));
    r += one;
    BUG_ON(v.counter != r);
    INIT(0);
    BUG_ON(atomic64_inc_not_zero(&v));
    BUG_ON(v.counter != r);
    INIT(-one);
    BUG_ON(!atomic64_inc_not_zero(&v));
    r += one;
    BUG_ON(v.counter != r);
// Confirm the return value fits in an int, even if the value doesn't
    INIT(v3);
    r_int = atomic64_inc_not_zero(&v);
    BUG_ON(!r_int);
    }
#[no_mangle]
unsafe extern "C" fn test_atomics_init() -> __init int {
    static __init int test_atomics_init(void)
    {
    test_atomic();
    test_atomic64();

    pr_info("passed for %s platform %s CX8 and %s SSE\n",

    "x86-64",

    "i586+",

    "i386+",

    boot_cpu_has(X86_FEATURE_CX8) ? "with" : "without",
    boot_cpu_has(X86_FEATURE_XMM) ? "with" : "without");

    pr_info("passed\n");

    return 0;
    }
    static __exit void test_atomics_exit(void) {}
    module_init(test_atomics_init);
    module_exit(test_atomics_exit);
    MODULE_DESCRIPTION("Testsuite for atomic64_t functions");
    MODULE_LICENSE("GPL");
