//! Automatically rewritten from C to Rust
//! Source: lib/test_ubsan.c
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

    typedef void(*test_ubsan_fp)(void);

    pr_info("%s " __VA_ARGS__ "%s(%s=%s)\n", __func__,	\
    sizeof(" " __VA_ARGS__) > 2 ? " " : "",		\

    } while (0)
#[no_mangle]
unsafe extern "C" fn test_ubsan_add_overflow() {
    static void test_ubsan_add_overflow(void)
    {
    let mut val: volatile int = INT_MAX;
    UBSAN_TEST(CONFIG_UBSAN_INTEGER_WRAP);
    val += 2;
    }
#[no_mangle]
unsafe extern "C" fn test_ubsan_sub_overflow() {
    static void test_ubsan_sub_overflow(void)
    {
    let mut val: volatile int = INT_MIN;
    let mut val2: volatile int = 2;
    UBSAN_TEST(CONFIG_UBSAN_INTEGER_WRAP);
    val -= val2;
    }
#[no_mangle]
unsafe extern "C" fn test_ubsan_mul_overflow() {
    static void test_ubsan_mul_overflow(void)
    {
    let mut val: volatile int = INT_MAX / 2;
    UBSAN_TEST(CONFIG_UBSAN_INTEGER_WRAP);
    val *= 3;
    }
#[no_mangle]
unsafe extern "C" fn test_ubsan_negate_overflow() {
    static void test_ubsan_negate_overflow(void)
    {
    let mut val: volatile int = INT_MIN;
    UBSAN_TEST(CONFIG_UBSAN_INTEGER_WRAP);
    val = -val;
    }
#[no_mangle]
unsafe extern "C" fn test_ubsan_divrem_overflow() {
    static void test_ubsan_divrem_overflow(void)
    {
    let mut val: volatile int = 16;
    let mut val2: volatile int = 0;
    UBSAN_TEST(CONFIG_UBSAN_DIV_ZERO);
    val /= val2;
    }
#[no_mangle]
unsafe extern "C" fn test_ubsan_truncate_signed() {
    static void test_ubsan_truncate_signed(void)
    {
    let mut val: volatile long = LONG_MAX;
    let mut val2: volatile int = 0;
    UBSAN_TEST(CONFIG_UBSAN_INTEGER_WRAP);
    val2 = val;
    }
#[no_mangle]
unsafe extern "C" fn test_ubsan_shift_out_of_bounds() {
    static void test_ubsan_shift_out_of_bounds(void)
    {
    let mut neg: volatile int = -1, wrap = 4;
    let mut val1: volatile int = 10;
    let mut val2: volatile int = INT_MAX;
    UBSAN_TEST(CONFIG_UBSAN_SHIFT, "negative exponent");
    val1 <<= neg;
    UBSAN_TEST(CONFIG_UBSAN_SHIFT, "left overflow");
    val2 <<= wrap;
    }
#[no_mangle]
unsafe extern "C" fn test_ubsan_out_of_bounds() {
    static void test_ubsan_out_of_bounds(void)
    {
    let mut i: c_int = 4, j = 4, k = -1;
    volatile struct {
    char above[4]; /* Protect surrounding memory. */
    int arr[4];
    char below[4]; /* Protect surrounding memory. */
    } data;
    OPTIMIZER_HIDE_VAR(i);
    OPTIMIZER_HIDE_VAR(j);
    OPTIMIZER_HIDE_VAR(k);
    UBSAN_TEST(CONFIG_UBSAN_BOUNDS, "above");
    data.arr[j] = i;
    UBSAN_TEST(CONFIG_UBSAN_BOUNDS, "below");
    data.arr[k] = i;
    }
    enum ubsan_test_enum {
    UBSAN_TEST_ZERO = 0,
    UBSAN_TEST_ONE,
    UBSAN_TEST_MAX,
    };
#[no_mangle]
unsafe extern "C" fn test_ubsan_load_invalid_value() {
    static void test_ubsan_load_invalid_value(void)
    {
    volatile char *dst, *src;
    bool val, val2, *ptr;
    enum ubsan_test_enum eval, eval2, *eptr;
    let mut c: c_uchar = 0xff;
    UBSAN_TEST(CONFIG_UBSAN_BOOL, "bool");
    dst = (char *)&val;
    src = &c;
// dst = *src;
    ptr = &val2;
    val2 = val;
    UBSAN_TEST(CONFIG_UBSAN_ENUM, "enum");
    dst = (char *)&eval;
    src = &c;
// dst = *src;
    eptr = &eval2;
    eval2 = eval;
    }
#[no_mangle]
unsafe extern "C" fn test_ubsan_misaligned_access() {
    static void test_ubsan_misaligned_access(void)
    {
    volatile char arr[5] __aligned(4) = {1, 2, 3, 4, 5};
    volatile int *ptr, val = 6;
    UBSAN_TEST(CONFIG_UBSAN_ALIGNMENT);
    ptr = (int *)(arr + 1);
// ptr = val;
    }
    static const test_ubsan_fp test_ubsan_array[] = {
    test_ubsan_add_overflow,
    test_ubsan_sub_overflow,
    test_ubsan_mul_overflow,
    test_ubsan_negate_overflow,
    test_ubsan_truncate_signed,
    test_ubsan_shift_out_of_bounds,
    test_ubsan_out_of_bounds,
    test_ubsan_load_invalid_value,
    test_ubsan_misaligned_access,
    };
// Excluded because they Oops the module.
    static __used const test_ubsan_fp skip_ubsan_array[] = {
    test_ubsan_divrem_overflow,
    };
#[no_mangle]
unsafe extern "C" fn test_ubsan_init() -> int __init {
    static int __init test_ubsan_init(void)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(test_ubsan_array); i++)
    test_ubsan_array[i]();
    return 0;
    }
    module_init(test_ubsan_init);
#[no_mangle]
unsafe extern "C" fn test_ubsan_exit() -> void __exit {
    static void __exit test_ubsan_exit(void)
    {
// do nothing
    }
    module_exit(test_ubsan_exit);
    MODULE_AUTHOR("Jinbum Park <jinb.park7@gmail.com>");
    MODULE_DESCRIPTION("UBSAN unit test");
    MODULE_LICENSE("GPL v2");
