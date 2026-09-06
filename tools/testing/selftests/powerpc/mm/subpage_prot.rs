//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/mm/subpage_prot.c
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


//
// Copyright IBM Corp.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of version 2.1 of the GNU Lesser General Public License
// as published by the Free Software Foundation.
//
// This program is distributed in the hope that it would be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
//

    char *file_name;
    int in_test;
    volatile int faulted;
    volatile void *dar;
    int errors;
#[no_mangle]
unsafe extern "C" fn segv(signum: c_int, info: *mut siginfo_t, ctxt_v: *mut c_void) {
    static void segv(int signum, siginfo_t *info, void *ctxt_v)
    {
    ucontext_t *ctxt = (ucontext_t *)ctxt_v;
    struct pt_regs *regs = ctxt.uc_mcontext.regs;
    if (!in_test) {
    fprintf(stderr, "Segfault outside of test !\n");
    exit(1);
    }
    faulted = 1;
    dar = (void *)regs.dar;
    regs.nip += 4;
    }
#[no_mangle]
pub unsafe extern "C" fn do_read(addr: *const volatile void) {
    static inline void do_read(const volatile void *addr)
    {
    int ret;
    asm volatile("lwz %0,0(%1); twi 0,%0,0; isync;\n"
    : "=r" (ret) : "r" (addr) : "memory");
    }
#[no_mangle]
pub unsafe extern "C" fn do_write(addr: *const volatile void) {
    static inline void do_write(const volatile void *addr)
    {
    let mut val: c_int = 0x1234567;
    asm volatile("stw %0,0(%1); sync; \n"
    : : "r" (val), "r" (addr) : "memory");
    }
#[no_mangle]
pub unsafe extern "C" fn check_faulted(addr: *mut c_void, page: c_long, subpage: c_long, write: c_int) {
    static inline void check_faulted(void *addr, long page, long subpage, int write)
    {
    let mut want_fault: c_int = (subpage == ((page + 3) % 16));
    if (write)
    want_fault |= (subpage == ((page + 1) % 16));
    if (faulted != want_fault) {
    printf("Failed at %p (p=%ld,sp=%ld,w=%d), want=%s, got=%s !\n",
    addr, page, subpage, write,
    want_fault ? "fault" : "pass",
    faulted ? "fault" : "pass");
    ++errors;
    }
    if (faulted) {
    if (dar != addr) {
    printf("Fault expected at %p and happened at %p !\n",
    addr, dar);
    }
    faulted = 0;
    asm volatile("sync" : : : "memory");
    }
    }
#[no_mangle]
unsafe extern "C" fn run_test(addr: *mut c_void, size: c_ulong) -> c_int {
    static int run_test(void *addr, unsigned long size)
    {
    unsigned int *map;
    long i, j, pages, err;
    pages = size / 0x10000;
    map = malloc(pages * 4);
    assert(map);
//
// for each page, mark subpage i % 16 read only and subpage
// (i + 3) % 16 inaccessible
//
    for (i = 0; i < pages; i++) {
    map[i] = (0x40000000 >> (((i + 1) * 2) % 32)) |
    (0xc0000000 >> (((i + 3) * 2) % 32));
    }
    err = syscall(__NR_subpage_prot, addr, size, map);
    if (err) {
    perror("subpage_perm");
    return 1;
    }
    free(map);
    in_test = 1;
    errors = 0;
    for (i = 0; i < pages; i++) {
    for (j = 0; j < 16; j++, addr += 0x1000) {
    do_read(addr);
    check_faulted(addr, i, j, 0);
    do_write(addr);
    check_faulted(addr, i, j, 1);
    }
    }
    in_test = 0;
    if (errors) {
    printf("%d errors detected\n", errors);
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn syscall_available() -> c_int {
    static int syscall_available(void)
    {
    int rc;
    errno = 0;
    rc = syscall(__NR_subpage_prot, 0, 0, 0);
    let mut rc: return = = 0 || (errno != ENOENT && errno != ENOSYS);
    }
#[no_mangle]
pub unsafe extern "C" fn test_anon() -> c_int {
    int test_anon(void)
    {
    unsigned long align;
    struct sigaction act = {
    .sa_sigaction = segv,
    .sa_flags = SA_SIGINFO
    };
    void *mallocblock;
    unsigned long mallocsize;
    SKIP_IF(!syscall_available());
    if (getpagesize() != 0x10000) {
    fprintf(stderr, "Kernel page size must be 64K!\n");
    return 1;
    }
    sigaction(SIGSEGV, &act, core::ptr::null_mut());
    mallocsize = 4 * 16 * 1024 * 1024;
    FAIL_IF(posix_memalign(&mallocblock, 64 * 1024, mallocsize));
    align = (unsigned long)mallocblock;
    if (align & 0xffff)
    align = (align | 0xffff) + 1;
    mallocblock = (void *)align;
    printf("allocated malloc block of 0x%lx bytes at %p\n",
    mallocsize, mallocblock);
    printf("testing malloc block...\n");
    return run_test(mallocblock, mallocsize);
    }
#[no_mangle]
pub unsafe extern "C" fn test_file() -> c_int {
    int test_file(void)
    {
    struct sigaction act = {
    .sa_sigaction = segv,
    .sa_flags = SA_SIGINFO
    };
    void *fileblock;
    off_t filesize;
    int fd;
    SKIP_IF(!syscall_available());
    fd = open(file_name, O_RDWR);
    if (fd == -1) {
    perror("failed to open file");
    return 1;
    }
    sigaction(SIGSEGV, &act, core::ptr::null_mut());
    filesize = lseek(fd, 0, SEEK_END);
    if (filesize & 0xffff)
    filesize &= ~0xfffful;
    fileblock = mmap(core::ptr::null_mut(), filesize, PROT_READ | PROT_WRITE,
    MAP_SHARED, fd, 0);
    if (fileblock == MAP_FAILED) {
    perror("failed to map file");
    return 1;
    }
    printf("allocated %s for 0x%llx bytes at %p\n",
    file_name, (long long)filesize, fileblock);
    printf("testing file map...\n");
    return run_test(fileblock, filesize);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int rc;
    rc = test_harness(test_anon, "subpage_prot_anon");
    if (rc)
    return rc;
    if (argc > 1)
    file_name = argv[1];
    else
    file_name = "tempfile";
    return test_harness(test_file, "subpage_prot_file");
    }
