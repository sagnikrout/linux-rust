//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/stringloops/strlen.c
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

pub const SIZE: c_int = 256;
pub const ITERATIONS: c_int = 1000;
pub const ITERATIONS_BENCH: c_int = 100000;
    int test_strlen(const void *s);
// test all offsets and lengths
#[no_mangle]
unsafe extern "C" fn test_one(s: *mut c_char) {
    static void test_one(char *s)
    {
    unsigned long offset;
    for (offset = 0; offset < SIZE; offset++) {
    int x, y;
    unsigned long i;
    y = strlen(s + offset);
    x = test_strlen(s + offset);
    if (x != y) {
    printf("strlen() returned %d, should have returned %d (%p offset %ld)\n", x, y, s, offset);
    for (i = offset; i < SIZE; i++)
    printf("%02x ", s[i]);
    printf("\n");
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn bench_test(s: *mut c_char) {
    static void bench_test(char *s)
    {
    struct timespec ts_start, ts_end;
    int i;
    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    for (i = 0; i < ITERATIONS_BENCH; i++)
    test_strlen(s);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);
    printf("len %3.3d : time = %.6f\n", test_strlen(s), ts_end.tv_sec - ts_start.tv_sec + (ts_end.tv_nsec - ts_start.tv_nsec) / 1e9);
    }
#[no_mangle]
unsafe extern "C" fn testcase() -> c_int {
    static int testcase(void)
    {
    char *s;
    unsigned long i;
    s = memalign(128, SIZE);
    if (!s) {
    perror("memalign");
    exit(1);
    }
    srandom(1);
    memset(s, 0, SIZE);
    for (i = 0; i < SIZE; i++) {
    char c;
    do {
    c = random() & 0x7f;
    } while (!c);
    s[i] = c;
    test_one(s);
    }
    for (i = 0; i < ITERATIONS; i++) {
    unsigned long j;
    for (j = 0; j < SIZE; j++) {
    char c;
    do {
    c = random() & 0x7f;
    } while (!c);
    s[j] = c;
    }
    for (j = 0; j < sizeof(long); j++) {
    s[SIZE - 1 - j] = 0;
    test_one(s);
    }
    }
    for (i = 0; i < SIZE; i++) {
    char c;
    do {
    c = random() & 0x7f;
    } while (!c);
    s[i] = c;
    }
    bench_test(s);
    s[16] = 0;
    bench_test(s);
    s[8] = 0;
    bench_test(s);
    s[4] = 0;
    bench_test(s);
    s[3] = 0;
    bench_test(s);
    s[2] = 0;
    bench_test(s);
    s[1] = 0;
    bench_test(s);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(testcase, "strlen");
    }
