//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/libarena/selftests/test_parallel_bitmap.bpf.c
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


// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause

pub const TEST_BITMAP_THREADS: c_int = 2;

    static struct arena_bitmap __arena *bitmap;
    static volatile u64 started;
    static volatile bool test_abort;
//
// The test needs cmpxchg atomics on arena memory.
//

    (defined(__TARGET_ARCH_arm64) || defined(__TARGET_ARCH_x86) || \
    defined(__TARGET_ARCH_s390) || \
    defined(__TARGET_ARCH_powerpc) ||				  \
    (defined(__TARGET_ARCH_riscv) && __riscv_xlen == 64))
#[no_mangle]
unsafe extern "C" fn bitmap_tests_enabled() -> bool {
    static bool bitmap_tests_enabled(void)
    {
    return true;
    }

#[no_mangle]
unsafe extern "C" fn bitmap_tests_enabled() -> bool {
    static bool bitmap_tests_enabled(void)
    {
    return false;
    }

    __weak
#[no_mangle]
pub unsafe extern "C" fn bitmap_wait_for_start() -> c_int {
    int bitmap_wait_for_start(void)
    {
    u64 i;
    __sync_fetch_and_add(&started, 1);
    for (i = zero; i < TEST_BITMAP_SYNC_SPINS && can_loop; i++) {
    if (test_abort)
    return -EINTR;
    if (smp_load_acquire(&started) >= TEST_BITMAP_THREADS)
    return 0;
    }
    test_abort = true;
    return -ETIMEDOUT;
    }
//
// The test makes sure writes don't clobber each other by overwriting
// the same word. One thread always writes on even bits, the other on
// odds. Both should be able to operate on the bitmap oblivious of the
// other's operations.
//
    __weak
#[no_mangle]
pub unsafe extern "C" fn bitmap_test_bit_sequence(bit: u32) -> c_int {
    int bitmap_test_bit_sequence(u32 bit)
    {
    if (bmp_test_and_clear_bit(bit, bitmap))
    return -EINVAL;
    if (bmp_test_and_set_bit(bit, bitmap))
    return -EINVAL;
    if (!bmp_test_bit(bit, bitmap))
    return -EINVAL;
    if (!bmp_test_and_set_bit(bit, bitmap))
    return -EINVAL;
    if (!bmp_test_bit(bit, bitmap))
    return -EINVAL;
    if (!bmp_test_and_clear_bit(bit, bitmap))
    return -EINVAL;
    if (bmp_test_bit(bit, bitmap))
    return -EINVAL;
    if (bmp_test_and_clear_bit(bit, bitmap))
    return -EINVAL;
    bmp_set_bit(bit, bitmap);
    if (!bmp_test_bit(bit, bitmap))
    return -EINVAL;
    bmp_clear_bit(bit, bitmap);
    if (bmp_test_bit(bit, bitmap))
    return -EINVAL;
    bmp_set_bit(bit, bitmap);
    if (!bmp_test_bit(bit, bitmap))
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bitmap_test_reset_single(parity: c_int) {
    static void bitmap_test_reset_single(int parity)
    {
    u32 bit;
    for (bit = parity; bit < TEST_BITMAP_BITS && can_loop; bit += 2)
    bmp_clear_bit(bit, bitmap);
    }
#[no_mangle]
unsafe extern "C" fn bitmap_test_common_single(parity: c_int) -> c_int {
    static int bitmap_test_common_single(int parity)
    {
    u32 bit;
    int ret;
    for (bit = parity; bit < TEST_BITMAP_BITS && can_loop; bit += 2) {
    if (test_abort)
    return -EINTR;
    ret = bitmap_test_bit_sequence(bit);
    if (ret) {
    test_abort = true;
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bitmap_test_common(parity: c_int) -> c_int {
    static int bitmap_test_common(int parity)
    {
    int ret;
    u32 i;
    arena_subprog_init();
    ret = bitmap_wait_for_start();
    if (ret)
    return ret;
    for (i = zero; i < TEST_BITMAP_ITERS && can_loop; i++) {
    ret = bitmap_test_common_single(parity);
    if (ret)
    return ret;
    if (test_abort)
    break;
    bitmap_test_reset_single(parity);
    }
    return 0;
    }
    SEC("syscall") int parallel_test_bitmap__enabled(void)
    {
    return bitmap_tests_enabled() ? 0 : -EOPNOTSUPP;
    }
    SEC("syscall") int parallel_test_bitmap__init(void)
    {
    bitmap = bmp_alloc(TEST_BITMAP_BITS);
    if (!bitmap)
    return -ENOMEM;
    return 0;
    }
    SEC("syscall") int parallel_test_bitmap__fini(void)
    {
    let mut ret: c_int = 0;
    if (!bitmap)
    return -EINVAL;
    bmp_free(bitmap);
    bitmap = core::ptr::null_mut();
    return ret;
    }
    SEC("syscall") int parallel_test_bitmap__0(void)
    {
    return bitmap_test_common(0);
    }
    SEC("syscall") int parallel_test_bitmap__1(void)
    {
    return bitmap_test_common(1);
    }
