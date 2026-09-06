//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/mm/hugetlb_vs_thp_test.c
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

// This must match the huge page & THP size

#[no_mangle]
unsafe extern "C" fn test_body() -> c_int {
    static int test_body(void)
    {
    void *addr;
    char *p;
    addr = (void *)0xa0000000;
    p = mmap(addr, SIZE, PROT_READ | PROT_WRITE,
    MAP_HUGETLB | MAP_ANONYMOUS | MAP_PRIVATE, -1, 0);
    if (p != MAP_FAILED) {
//
// Typically the mmap will fail because no huge pages are
// allocated on the system. But if there are huge pages
// allocated the mmap will succeed. That's fine too, we just
// munmap here before continuing.  munmap() length of
// MAP_HUGETLB memory must be hugepage aligned.
//
    if (munmap(addr, SIZE)) {
    perror("munmap");
    return 1;
    }
    }
    p = mmap(addr, SIZE, PROT_READ | PROT_WRITE,
    MAP_ANONYMOUS | MAP_PRIVATE, -1, 0);
    if (p == MAP_FAILED) {
    printf("Mapping failed @ %p\n", addr);
    perror("mmap");
    return 1;
    }
//
// Either a user or kernel access is sufficient to trigger the bug.
// A kernel access is easier to spot & debug, as it will trigger the
// softlockup or RCU stall detectors, and when the system is kicked
// into xmon we get a backtrace in the kernel.
//
// A good option is:
// getcwd(p, SIZE);
//
// For the purposes of this testcase it's preferable to spin in
// userspace, so the harness can kill us if we get stuck. That way we
// see a test failure rather than a dead system.
//
// p = 0xf;
    munmap(addr, SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_main() -> c_int {
    static int test_main(void)
    {
    int i;
// 10,000 because it's a "bunch", and completes reasonably quickly
    for (i = 0; i < 10000; i++)
    if (test_body())
    return 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test_main, "hugetlb_vs_thp");
    }
