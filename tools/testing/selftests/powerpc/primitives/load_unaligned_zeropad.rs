//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/primitives/load_unaligned_zeropad.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Userspace test harness for load_unaligned_zeropad. Creates two
// pages and uses mprotect to prevent access to the second page and
// a SEGV handler that walks the exception tables and runs the fixup
// routine.
//
// The results are compared against a normal load that is that is
// performed while access to the second page is enabled via mprotect.
//
// Copyright (C) 2014 Anton Blanchard <anton@au.ibm.com>, IBM
//

    static inline unsigned long __fls(unsigned long x);

#[no_mangle]
pub unsafe extern "C" fn __fls(x: c_ulong) -> c_ulong {
    static inline unsigned long __fls(unsigned long x)
    {
    int lz;
    asm (PPC_CNTLZL "%0,%1" : "=r" (lz) : "r" (x));
    return sizeof(unsigned long) - 1 - lz;
    }
    static int page_size;
    static char *mem_region;
#[no_mangle]
unsafe extern "C" fn protect_region() -> c_int {
    static int protect_region(void)
    {
    if (mprotect(mem_region + page_size, page_size, PROT_NONE)) {
    perror("mprotect");
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn unprotect_region() -> c_int {
    static int unprotect_region(void)
    {
    if (mprotect(mem_region + page_size, page_size, PROT_READ|PROT_WRITE)) {
    perror("mprotect");
    return 1;
    }
    return 0;
    }
    extern char __start___ex_table[];
    extern char __stop___ex_table[];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct extbl_entry {
    pub insn: c_int,
    pub fixup: c_int,
}

#[no_mangle]
unsafe extern "C" fn segv_handler(signr: c_int, info: *mut siginfo_t, ptr: *mut c_void) {
    static void segv_handler(int signr, siginfo_t *info, void *ptr)
    {
    ucontext_t *uc = (ucontext_t *)ptr;
    let mut addr: c_ulong = (unsigned long)info.si_addr;
    unsigned long *ip = &UCONTEXT_NIA(uc);
    struct extbl_entry *entry = (struct extbl_entry *)__start___ex_table;
    while (entry < (struct extbl_entry *)__stop___ex_table) {
    unsigned long insn, fixup;
    insn  = (unsigned long)&entry.insn + entry.insn;
    fixup = (unsigned long)&entry.fixup + entry.fixup;
    if (insn == *ip) {
// ip = fixup;
    return;
    }
    }
    printf("No exception table match for NIA %lx ADDR %lx\n", *ip, addr);
    abort();
    }
#[no_mangle]
unsafe extern "C" fn setup_segv_handler() {
    static void setup_segv_handler(void)
    {
    struct sigaction action;
    memset(&action, 0, sizeof(action));
    action.sa_sigaction = segv_handler;
    action.sa_flags = SA_SIGINFO;
    sigaction(SIGSEGV, &action, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn do_one_test(p: *mut c_char, page_offset: c_int) -> c_int {
    static int do_one_test(char *p, int page_offset)
    {
    unsigned long should;
    unsigned long got;
    FAIL_IF(unprotect_region());
    should = *(unsigned long *)p;
    FAIL_IF(protect_region());
    got = load_unaligned_zeropad(p);
    if (should != got) {
    printf("offset %u load_unaligned_zeropad returned 0x%lx, should be 0x%lx\n", page_offset, got, should);
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_body() -> c_int {
    static int test_body(void)
    {
    unsigned long i;
    page_size = getpagesize();
    mem_region = mmap(core::ptr::null_mut(), page_size * 2, PROT_READ|PROT_WRITE,
    MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
    FAIL_IF(mem_region == MAP_FAILED);
    for (i = 0; i < page_size; i++)
    mem_region[i] = i;
    memset(mem_region+page_size, 0, page_size);
    setup_segv_handler();
    for (i = 0; i < page_size; i++)
    FAIL_IF(do_one_test(mem_region+i, i));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test_body, "load_unaligned_zeropad");
    }
