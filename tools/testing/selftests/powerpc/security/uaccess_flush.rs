//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/security/uaccess_flush.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2018 IBM Corporation.
// Copyright 2020 Canonical Ltd.
//
// Macro flag: #define __SANE_USERSPACE_TYPES__

#[no_mangle]
pub unsafe extern "C" fn uaccess_flush_test() -> c_int {
    int uaccess_flush_test(void)
    {
    char *p;
    let mut repetitions: c_int = 10;
    int fd, passes = 0, iter, rc = 0;
    struct perf_event_read v;
    let mut l1d_misses_total: __u64 = 0;
    let mut iterations: c_ulong = 100000, zero_size = 24 * 1024;
    unsigned long l1d_misses_expected;
    int rfi_flush_orig;
    int entry_flush_orig;
    int uaccess_flush, uaccess_flush_orig;
    SKIP_IF(geteuid() != 0);
// The PMU event we use only works on Power7 or later
    SKIP_IF(!have_hwcap(PPC_FEATURE_ARCH_2_06));
    if (read_debugfs_int("powerpc/rfi_flush", &rfi_flush_orig) < 0) {
    perror("Unable to read powerpc/rfi_flush debugfs file");
    SKIP_IF(1);
    }
    if (read_debugfs_int("powerpc/entry_flush", &entry_flush_orig) < 0) {
    perror("Unable to read powerpc/entry_flush debugfs file");
    SKIP_IF(1);
    }
    if (read_debugfs_int("powerpc/uaccess_flush", &uaccess_flush_orig) < 0) {
    perror("Unable to read powerpc/entry_flush debugfs file");
    SKIP_IF(1);
    }
    if (rfi_flush_orig != 0) {
    if (write_debugfs_int("powerpc/rfi_flush", 0) < 0) {
    perror("error writing to powerpc/rfi_flush debugfs file");
    FAIL_IF(1);
    }
    }
    if (entry_flush_orig != 0) {
    if (write_debugfs_int("powerpc/entry_flush", 0) < 0) {
    perror("error writing to powerpc/entry_flush debugfs file");
    FAIL_IF(1);
    }
    }
    uaccess_flush = uaccess_flush_orig;
    fd = perf_event_open_counter(PERF_TYPE_HW_CACHE, PERF_L1D_READ_MISS_CONFIG, -1);
    FAIL_IF(fd < 0);
    p = (char *)memalign(zero_size, CACHELINE_SIZE);
    FAIL_IF(perf_event_enable(fd));
// disable L1 prefetching
    set_dscr(1);
    iter = repetitions;
//
// We expect to see l1d miss for each cacheline access when entry_flush
// is set. Allow a small variation on this.
//
    l1d_misses_expected = iterations * (zero_size / CACHELINE_SIZE - 2);
    again:
    FAIL_IF(perf_event_reset(fd));
    syscall_loop_uaccess(p, iterations, zero_size);
    FAIL_IF(read(fd, &v, sizeof(v)) != sizeof(v));
    if (uaccess_flush && v.l1d_misses >= l1d_misses_expected)
    passes++;
#[no_mangle]
pub unsafe extern "C" fn if(2): !uaccess_flush && v.l1d_misses < (l1d_misses_expected /) -> else {
    else if (!uaccess_flush && v.l1d_misses < (l1d_misses_expected / 2))
    passes++;
    l1d_misses_total += v.l1d_misses;
    while (--iter)
    goto again;
    if (passes < repetitions) {
    printf("FAIL (L1D misses with uaccess_flush=%d: %llu %c %lu) [%d/%d failures]\n",
    uaccess_flush, l1d_misses_total, uaccess_flush ? '<' : '>',
    uaccess_flush ? repetitions * l1d_misses_expected :
    repetitions * l1d_misses_expected / 2,
    repetitions - passes, repetitions);
    rc = 1;
    } else {
    printf("PASS (L1D misses with uaccess_flush=%d: %llu %c %lu) [%d/%d pass]\n",
    uaccess_flush, l1d_misses_total, uaccess_flush ? '>' : '<',
    uaccess_flush ? repetitions * l1d_misses_expected :
    repetitions * l1d_misses_expected / 2,
    passes, repetitions);
    }
    if (uaccess_flush == uaccess_flush_orig) {
    uaccess_flush = !uaccess_flush_orig;
    if (write_debugfs_int("powerpc/uaccess_flush", uaccess_flush) < 0) {
    perror("error writing to powerpc/uaccess_flush debugfs file");
    return 1;
    }
    iter = repetitions;
    l1d_misses_total = 0;
    passes = 0;
    goto again;
    }
    perf_event_disable(fd);
    close(fd);
    set_dscr(0);
    if (write_debugfs_int("powerpc/rfi_flush", rfi_flush_orig) < 0) {
    perror("unable to restore original value of powerpc/rfi_flush debugfs file");
    return 1;
    }
    if (write_debugfs_int("powerpc/entry_flush", entry_flush_orig) < 0) {
    perror("unable to restore original value of powerpc/entry_flush debugfs file");
    return 1;
    }
    if (write_debugfs_int("powerpc/uaccess_flush", uaccess_flush_orig) < 0) {
    perror("unable to restore original value of powerpc/uaccess_flush debugfs file");
    return 1;
    }
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    return test_harness(uaccess_flush_test, "uaccess_flush_test");
    }
