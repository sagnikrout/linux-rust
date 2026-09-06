//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/mm/bad_accesses.c
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
// Copyright 2019, Michael Ellerman, IBM Corp.
//
// Test that out-of-bounds reads/writes behave as expected.

// Old distros (Ubuntu 16.04 at least) don't define this

pub const SEGV_BNDERR: c_int = 3;

// 64-bit kernel is always here

    static unsigned long kernel_virt_end;
    static volatile int fault_code;
    static volatile unsigned long fault_addr;
    static jmp_buf setjmp_env;
#[no_mangle]
unsafe extern "C" fn segv_handler(n: c_int, info: *mut siginfo_t, ctxt_v: *mut c_void) {
    static void segv_handler(int n, siginfo_t *info, void *ctxt_v)
    {
    fault_code = info.si_code;
    fault_addr = (unsigned long)info.si_addr;
    siglongjmp(setjmp_env, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn bad_access(p: *mut c_char, write: bool) -> c_int {
    int bad_access(char *p, bool write)
    {
    let mut x: c_char = 0;
    fault_code = 0;
    fault_addr = 0;
    if (sigsetjmp(setjmp_env, 1) == 0) {
    if (write)
// p = 1;
    else
    x = *p;
    printf("Bad - no SEGV! (%c)\n", x);
    return 1;
    }
// If we see MAPERR that means we took a page fault rather than an SLB
// miss. We only expect to take page faults for addresses within the
// valid kernel range.
    FAIL_IF(fault_code == SEGV_MAPERR && \
    (fault_addr < PAGE_OFFSET || fault_addr >= kernel_virt_end));
    FAIL_IF(fault_code != SEGV_MAPERR && fault_code != SEGV_BNDERR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test() -> c_int {
    static int test(void)
    {
    unsigned long i, j, addr, region_shift, page_shift, page_size;
    struct sigaction sig;
    bool hash_mmu;
    sig = (struct sigaction) {
    .sa_sigaction = segv_handler,
    .sa_flags = SA_SIGINFO,
    };
    FAIL_IF(sigaction(SIGSEGV, &sig, core::ptr::null_mut()) != 0);
    FAIL_IF(using_hash_mmu(&hash_mmu));
    page_size = sysconf(_SC_PAGESIZE);
    if (page_size == (64 * 1024))
    page_shift = 16;
    else
    page_shift = 12;
    if (page_size == (64 * 1024) || !hash_mmu) {
    region_shift = 52;
// We have 7 512T regions (4 kernel linear, vmalloc, io, vmemmap)
    kernel_virt_end = PAGE_OFFSET + (7 * (512ul << 40));
    } else if (page_size == (4 * 1024) && hash_mmu) {
    region_shift = 46;
// We have 7 64T regions (4 kernel linear, vmalloc, io, vmemmap)
    kernel_virt_end = PAGE_OFFSET + (7 * (64ul << 40));
    } else
    FAIL_IF(true);
    printf("Using %s MMU, PAGE_SIZE = %dKB start address 0x%016lx\n",
    hash_mmu ? "hash" : "radix",
    (1 << page_shift) >> 10,
    1ul << region_shift);
// This generates access patterns like:
// 0x0010000000000000
// 0x0010000000010000
// 0x0010000000020000
// ...
// 0x0014000000000000
// 0x0018000000000000
// 0x0020000000000000
// 0x0020000000010000
// 0x0020000000020000
// ...
// 0xf400000000000000
// 0xf800000000000000
    for (i = 1; i <= ((0xful << 60) >> region_shift); i++) {
    for (j = page_shift - 1; j < 60; j++) {
    unsigned long base, delta;
    base  = i << region_shift;
    delta = 1ul << j;
    if (delta >= base)
    break;
    addr = (base | delta) & ~((1 << page_shift) - 1);
    FAIL_IF(bad_access((char *)addr, false));
    FAIL_IF(bad_access((char *)addr, true));
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    test_harness_set_timeout(300);
    return test_harness(test, "bad_accesses");
    }
