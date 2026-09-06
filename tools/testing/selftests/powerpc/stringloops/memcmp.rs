//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/stringloops/memcmp.c
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
pub const ITERATIONS: c_int = 10000;

pub const LARGE_ITERATIONS: c_int = 1000;
pub const LARGE_MAX_OFFSET: c_int = 32;
pub const LARGE_SIZE_START: c_int = 4096;
// This is big enough to fit LARGE_SIZE and works on 4K & 64K kernels

pub const MAX_OFFSET_DIFF_S1_S2: c_int = 48;
    int vmx_count;
#[no_mangle]
pub unsafe extern "C" fn enter_vmx_ops() -> c_int {
    int enter_vmx_ops(void)
    {
    vmx_count++;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn exit_vmx_ops() {
    void exit_vmx_ops(void)
    {
    vmx_count--;
    }
    int test_memcmp(const void *s1, const void *s2, size_t n);
// test all offsets and lengths
    static void test_one(char *s1, char *s2, unsigned long max_offset,
    unsigned long size_start, unsigned long max_size)
    {
    unsigned long offset, size;
    for (offset = 0; offset < max_offset; offset++) {
    for (size = size_start; size < (max_size - offset); size++) {
    int x, y;
    unsigned long i;
    y = memcmp(s1+offset, s2+offset, size);
    x = test_memcmp(s1+offset, s2+offset, size);
    if (((x ^ y) < 0) &&	/* Trick to compare sign */
    ((x | y) != 0)) { /* check for zero */
    printf("memcmp returned %d, should have returned %d (offset %ld size %ld)\n", x, y, offset, size);
    for (i = offset; i < offset+size; i++)
    printf("%02x ", s1[i]);
    printf("\n");
    for (i = offset; i < offset+size; i++)
    printf("%02x ", s2[i]);
    printf("\n");
    abort();
    }
    if (vmx_count != 0) {
    printf("vmx enter/exit not paired.(offset:%ld size:%ld s1:%p s2:%p vc:%d\n",
    offset, size, s1, s2, vmx_count);
    printf("\n");
    abort();
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn testcase(islarge: bool) -> c_int {
    static int testcase(bool islarge)
    {
    unsigned long i, comp_size, alloc_size;
    char *p, *s1, *s2;
    int iterations;
    comp_size = (islarge ? LARGE_SIZE : SIZE);
    alloc_size = comp_size + MAX_OFFSET_DIFF_S1_S2;
    iterations = islarge ? LARGE_ITERATIONS : ITERATIONS;
    p = mmap(core::ptr::null_mut(), 4 * MAP_SIZE, PROT_READ | PROT_WRITE,
    MAP_ANONYMOUS | MAP_PRIVATE, -1, 0);
    FAIL_IF(p == MAP_FAILED);
// Put s1/s2 at the end of a page
    s1 = p + MAP_SIZE - alloc_size;
    s2 = p + 3 * MAP_SIZE - alloc_size;
// And unmap the subsequent page to force a fault if we overread
    munmap(p + MAP_SIZE, MAP_SIZE);
    munmap(p + 3 * MAP_SIZE, MAP_SIZE);
    srandom(time(0));
    for (i = 0; i < iterations; i++) {
    unsigned long j;
    unsigned long change;
    char *rand_s1 = s1;
    char *rand_s2 = s2;
    for (j = 0; j < alloc_size; j++)
    s1[j] = random();
    rand_s1 += random() % MAX_OFFSET_DIFF_S1_S2;
    rand_s2 += random() % MAX_OFFSET_DIFF_S1_S2;
    memcpy(rand_s2, rand_s1, comp_size);
// change one byte
    change = random() % comp_size;
    rand_s2[change] = random() & 0xff;
    if (islarge)
    test_one(rand_s1, rand_s2, LARGE_MAX_OFFSET,
    LARGE_SIZE_START, comp_size);
    else
    test_one(rand_s1, rand_s2, SIZE, 0, comp_size);
    }
    srandom(time(0));
    for (i = 0; i < iterations; i++) {
    unsigned long j;
    unsigned long change;
    char *rand_s1 = s1;
    char *rand_s2 = s2;
    for (j = 0; j < alloc_size; j++)
    s1[j] = random();
    rand_s1 += random() % MAX_OFFSET_DIFF_S1_S2;
    rand_s2 += random() % MAX_OFFSET_DIFF_S1_S2;
    memcpy(rand_s2, rand_s1, comp_size);
// change multiple bytes, 1/8 of total
    for (j = 0; j < comp_size / 8; j++) {
    change = random() % comp_size;
    s2[change] = random() & 0xff;
    }
    if (islarge)
    test_one(rand_s1, rand_s2, LARGE_MAX_OFFSET,
    LARGE_SIZE_START, comp_size);
    else
    test_one(rand_s1, rand_s2, SIZE, 0, comp_size);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn testcases() -> c_int {
    static int testcases(void)
    {

// vcmpequd used in memcmp_64.S is v2.07
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_ARCH_2_07));

    testcase(0);
    testcase(1);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    test_harness_set_timeout(300);
    return test_harness(testcases, "memcmp");
    }
