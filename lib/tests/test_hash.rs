//! Automatically rewritten from C to Rust
//! Source: lib/tests/test_hash.c
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
//
// Test cases for <linux/hash.h> and <linux/stringhash.h>
// This just verifies that various ways of computing a hash
// produce the same thing and, for cases where a k-bit hash
// value is requested, is of the requested size.
//
// We fill a buffer with a 255-byte null-terminated string,
// and use both full_name_hash() and hashlen_string() to hash the
// substrings from i to j, where 0 <= i < j < 256.
//
// The returned values are used to check that __hash_32() and
// __hash_32_generic() compute the same thing.  Likewise hash_32()
// and hash_64().
//

// 32-bit XORSHIFT generator.  Seed must not be zero.
    static u32 __attribute_const__
    xorshift(u32 seed)
    {
    seed ^= seed << 13;
    seed ^= seed >> 17;
    seed ^= seed << 5;
    return seed;
    }
// Given a non-zero x, returns a non-zero byte.
    static u8 __attribute_const__
    mod255(u32 x)
    {
    x = (x & 0xffff) + (x >> 16);	/* 1 <= x <= 0x1fffe */
    x = (x & 0xff) + (x >> 8);	/* 1 <= x <= 0x2fd */
    x = (x & 0xff) + (x >> 8);	/* 1 <= x <= 0x100 */
    x = (x & 0xff) + (x >> 8);	/* 1 <= x <= 0xff */
    return x;
    }
// Fill the buffer with non-zero bytes.
#[no_mangle]
unsafe extern "C" fn fill_buf(buf: *mut c_char, len: usize, seed: u32) {
    static void fill_buf(char *buf, size_t len, u32 seed)
    {
    size_t i;
    for (i = 0; i < len; i++) {
    seed = xorshift(seed);
    buf[i] = mod255(seed);
    }
    }
// Holds most testing variables for the int test.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_hash_params {
// Pointer to integer to be hashed.
    pub h64: *mut c_ulonglong,
// Low 32-bits of integer to be hashed.
    pub h0: u32,
// Arch-specific hash result.
    pub h1: u32,
// Generic hash result.
    pub h2: u32,
// ORed hashes of given size (in bits).
    pub (*hash_or)[33]: *mut u32,
}

    static void
    test_int__hash_32(struct kunit *test, struct test_hash_params *params)
    {
    params.hash_or[1][0] |= params.h2 = __hash_32_generic(params.h0);

    KUNIT_EXPECT_EQ_MSG(test, params.h1, params.h2,
    "__hash_32(%#x) = %#x != __hash_32_generic() = %#x",
    params.h0, params.h1, params.h2);

    }

    static void
    test_int_hash_64(struct kunit *test, struct test_hash_params *params, u32 const *m, int *k)
    {
    params.h2 = hash_64_generic(*params.h64, *k);

    KUNIT_EXPECT_EQ_MSG(test, params.h1, params.h2,
    "hash_64(%#llx, %d) = %#x != hash_64_generic() = %#x",
// params->h64, *k, params->h1, params->h2);

    KUNIT_EXPECT_LE_MSG(test, params.h1, params.h2,
    "hash_64_generic(%#llx, %d) = %#x > %#x",
// params->h64, *k, params->h1, *m);

    }

//
// Test the various integer hash functions.  h64 (or its low-order bits)
// is the integer to hash.  hash_or accumulates the OR of the hash values,
// which are later checked to see that they cover all the requested bits.
//
// Because these functions (as opposed to the string hashes) are all
// inline, the code being tested is actually in the module, and you can
// recompile and re-test the module without rebooting.
//
    static void
    test_int_hash(struct kunit *test, unsigned long long h64, u32 hash_or[2][33])
    {
    int k;
    let mut params: test_hash_params = { &h64, (u32)h64, 0, 0, hash_or };
// Test __hash32
    hash_or[0][0] |= params.h1 = __hash_32(params.h0);

    test_int__hash_32(test, &params);

// Test k = 1..32 bits
    for (k = 1; k <= 32; k++) {
    u32 const m = ((u32)2 << (k-1)) - 1;	/* Low k bits set */
// Test hash_32
    hash_or[0][k] |= params.h1 = hash_32(params.h0, k);
    KUNIT_EXPECT_LE_MSG(test, params.h1, m,
    "hash_32(%#x, %d) = %#x > %#x",
    params.h0, k, params.h1, m);
// Test hash_64
    hash_or[1][k] |= params.h1 = hash_64(h64, k);
    KUNIT_EXPECT_LE_MSG(test, params.h1, m,
    "hash_64(%#llx, %d) = %#x > %#x",
    h64, k, params.h1, m);

    test_int_hash_64(test, &params, &m, &k);

    }
    }

#[no_mangle]
unsafe extern "C" fn test_string_or(test: *mut kunit) {
    static void test_string_or(struct kunit *test)
    {
    char buf[SIZE+1];
    let mut string_or: u32 = 0;
    int i, j;
    fill_buf(buf, SIZE, 1);
// Test every possible non-empty substring in the buffer.
    for (j = SIZE; j > 0; --j) {
    buf[j] = '\0';
    for (i = 0; i <= j; i++) {
    let mut h0: u32 = full_name_hash(buf+i, buf+i, j-i);
    string_or |= h0;
    } /* i */
    } /* j */
// The OR of all the hash values should cover all the bits
    KUNIT_EXPECT_EQ_MSG(test, string_or, -1u,
    "OR of all string hash results = %#x != %#x",
    string_or, -1u);
    }
#[no_mangle]
unsafe extern "C" fn test_hash_or(test: *mut kunit) {
    static void test_hash_or(struct kunit *test)
    {
    char buf[SIZE+1];
    u32 hash_or[2][33] = { { 0, } };
    let mut h64: c_ulonglong = 0;
    int i, j;
    fill_buf(buf, SIZE, 1);
// Test every possible non-empty substring in the buffer.
    for (j = SIZE; j > 0; --j) {
    buf[j] = '\0';
    for (i = 0; i <= j; i++) {
    let mut hashlen: u64 = hashlen_string(buf+i, buf+i);
    let mut h0: u32 = full_name_hash(buf+i, buf+i, j-i);
// Check that hashlen_string gets the length right
    KUNIT_EXPECT_EQ_MSG(test, hashlen_len(hashlen), j-i,
    "hashlen_string(%d..%d) returned length %u, expected %d",
    i, j, hashlen_len(hashlen), j-i);
// Check that the hashes match
    KUNIT_EXPECT_EQ_MSG(test, hashlen_hash(hashlen), h0,
    "hashlen_string(%d..%d) = %08x != full_name_hash() = %08x",
    i, j, hashlen_hash(hashlen), h0);
    h64 = h64 << 32 | h0;	/* For use with hash_64 */
    test_int_hash(test, h64, hash_or);
    } /* i */
    } /* j */
    KUNIT_EXPECT_EQ_MSG(test, hash_or[0][0], -1u,
    "OR of all __hash_32 results = %#x != %#x",
    hash_or[0][0], -1u);

    KUNIT_EXPECT_EQ_MSG(test, hash_or[1][0], -1u,
    "OR of all __hash_32_generic results = %#x != %#x",
    hash_or[1][0], -1u);

// Likewise for all the i-bit hash values
    for (i = 1; i <= 32; i++) {
    u32 const m = ((u32)2 << (i-1)) - 1;	/* Low i bits set */
    KUNIT_EXPECT_EQ_MSG(test, hash_or[0][i], m,
    "OR of all hash_32(%d) results = %#x (%#x expected)",
    i, hash_or[0][i], m);
    KUNIT_EXPECT_EQ_MSG(test, hash_or[1][i], m,
    "OR of all hash_64(%d) results = %#x (%#x expected)",
    i, hash_or[1][i], m);
    }
    }
    static struct kunit_case hash_test_cases[] __refdata = {
    KUNIT_CASE(test_string_or),
    KUNIT_CASE(test_hash_or),
    {}
    };
    static struct kunit_suite hash_test_suite = {
    .name = "hash",
    .test_cases = hash_test_cases,
    };
    kunit_test_suite(hash_test_suite);
    MODULE_DESCRIPTION("Test cases for <linux/hash.h> and <linux/stringhash.h>");
    MODULE_LICENSE("GPL");
