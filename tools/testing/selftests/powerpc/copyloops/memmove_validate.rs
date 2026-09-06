//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/copyloops/memmove_validate.c
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

    void *TEST_MEMMOVE(const void *s1, const void *s2, size_t n);
pub const BUF_LEN: c_int = 65536;
pub const MAX_OFFSET: c_int = 512;
#[no_mangle]
pub unsafe extern "C" fn max(a: usize, b: usize) -> usize {
    size_t max(size_t a, size_t b)
    {
    if (a >= b)
    return a;
    return b;
    }
#[no_mangle]
unsafe extern "C" fn testcase_run() -> c_int {
    static int testcase_run(void)
    {
    size_t i, src_off, dst_off, len;
    char *usermap = memalign(BUF_LEN, BUF_LEN);
    char *kernelmap = memalign(BUF_LEN, BUF_LEN);
    assert(usermap != core::ptr::null_mut());
    assert(kernelmap != core::ptr::null_mut());
    memset(usermap, 0, BUF_LEN);
    memset(kernelmap, 0, BUF_LEN);
    for (i = 0; i < BUF_LEN; i++) {
    usermap[i] = i & 0xff;
    kernelmap[i] = i & 0xff;
    }
    for (src_off = 0; src_off < MAX_OFFSET; src_off++) {
    for (dst_off = 0; dst_off < MAX_OFFSET; dst_off++) {
    for (len = 1; len < MAX_OFFSET - max(src_off, dst_off); len++) {
    memmove(usermap + dst_off, usermap + src_off, len);
    TEST_MEMMOVE(kernelmap + dst_off, kernelmap + src_off, len);
    if (memcmp(usermap, kernelmap, MAX_OFFSET) != 0) {
    printf("memmove failed at %ld %ld %ld\n",
    src_off, dst_off, len);
    abort();
    }
    }
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(testcase_run, "memmove");
    }
