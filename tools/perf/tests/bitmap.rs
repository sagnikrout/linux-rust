//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/bitmap.c
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

pub const NBITS: c_int = 100;
    static unsigned long *get_bitmap(const char *str, int nbits)
    {
    struct perf_cpu_map *map = perf_cpu_map__new(str);
    unsigned long *bm;
    bm = bitmap_zalloc(nbits);
    if (map && bm) {
    unsigned int i;
    struct perf_cpu cpu;
    perf_cpu_map__for_each_cpu(cpu, i, map)
    __set_bit(cpu.cpu, bm);
    }
    perf_cpu_map__put(map);
    return bm;
    }
#[no_mangle]
unsafe extern "C" fn test_bitmap(str: *const c_char) -> c_int {
    static int test_bitmap(const char *str)
    {
    unsigned long *bm = get_bitmap(str, NBITS);
    char buf[100];
    int ret;
    bitmap_scnprintf(bm, NBITS, buf, sizeof(buf));
    pr_debug("bitmap: %s\n", buf);
    ret = !strcmp(buf, str);
    free(bm);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test__bitmap_print(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__bitmap_print(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    TEST_ASSERT_VAL("failed to convert map", test_bitmap("1"));
    TEST_ASSERT_VAL("failed to convert map", test_bitmap("1,5"));
    TEST_ASSERT_VAL("failed to convert map", test_bitmap("1,3,5,7,9,11,13,15,17,19,21-40"));
    TEST_ASSERT_VAL("failed to convert map", test_bitmap("2-5"));
    TEST_ASSERT_VAL("failed to convert map", test_bitmap("1,3-6,8-10,24,35-37"));
    TEST_ASSERT_VAL("failed to convert map", test_bitmap("1,3-6,8-10,24,35-37"));
    TEST_ASSERT_VAL("failed to convert map", test_bitmap("1-10,12-20,22-30,32-40"));
    return 0;
    }
    DEFINE_SUITE("Print bitmap", bitmap_print);
