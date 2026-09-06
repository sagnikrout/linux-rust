//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/proc/proc-empty-vm.c
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
// Copyright (c) 2022 Alexey Dobriyan <adobriyan@gmail.com>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// Create a process without mappings by unmapping everything at once and
// holding it with ptrace(2). See what happens to
//
// /proc/${pid}/maps
// /proc/${pid}/numa_maps
// /proc/${pid}/smaps
// /proc/${pid}/smaps_rollup
//

// Macro flag: #define _GNU_SOURCE

// Macro flag: #define TEST_VSYSCALL

pub const SYS_pkey_alloc: c_int = 330;

pub const SYS_pkey_free: c_int = 331;

pub const SYS_pkey_alloc: c_int = 381;

pub const SYS_pkey_free: c_int = 382;

    static int g_protection_key_support;
#[no_mangle]
unsafe extern "C" fn protection_key_support() -> c_int {
    static int protection_key_support(void)
    {
    let mut rv: c_long = syscall(SYS_pkey_alloc, 0, 0);
    if (rv > 0) {
    syscall(SYS_pkey_free, (int)rv);
    return 1;
    } else if (rv == -1 && errno == ENOSYS) {
    return 0;
    } else if (rv == -1 && errno == EINVAL) {
// ospke=n
    return 0;
    } else {
    fprintf(stderr, "%s: error: rv %ld, errno %d\n", __func__, rv, errno);
    exit(EXIT_FAILURE);
    }
    }
//
// 0: vsyscall VMA doesn't exist	vsyscall=none
// 1: vsyscall VMA is --xp		vsyscall=xonly
// 2: vsyscall VMA is r-xp		vsyscall=emulate
//
    static volatile int g_vsyscall;
    static const char *g_proc_pid_maps_vsyscall;
    static const char *g_proc_pid_smaps_vsyscall;
    static const char proc_pid_maps_vsyscall_0[] = "";
    static const char proc_pid_maps_vsyscall_1[] =
    "ffffffffff600000-ffffffffff601000 --xp 00000000 00:00 0                  [vsyscall]\n";
    static const char proc_pid_maps_vsyscall_2[] =
    "ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]\n";
    static const char proc_pid_smaps_vsyscall_0[] = "";
    static const char proc_pid_smaps_vsyscall_1[] =
    "ffffffffff600000-ffffffffff601000 --xp 00000000 00:00 0                  [vsyscall]\n"
    "Size:                  4 kB\n"
    "KernelPageSize:        4 kB\n"
    "MMUPageSize:           4 kB\n"
    "Rss:                   0 kB\n"
    "Pss:                   0 kB\n"
    "Pss_Dirty:             0 kB\n"
    "Shared_Clean:          0 kB\n"
    "Shared_Dirty:          0 kB\n"
    "Private_Clean:         0 kB\n"
    "Private_Dirty:         0 kB\n"
    "Referenced:            0 kB\n"
    "Anonymous:             0 kB\n"
    "KSM:                   0 kB\n"
    "LazyFree:              0 kB\n"
    "AnonHugePages:         0 kB\n"
    "ShmemPmdMapped:        0 kB\n"
    "FilePmdMapped:         0 kB\n"
    "Shared_Hugetlb:        0 kB\n"
    "Private_Hugetlb:       0 kB\n"
    "Swap:                  0 kB\n"
    "SwapPss:               0 kB\n"
    "Locked:                0 kB\n"
    "THPeligible:           0\n"
    ;
    static const char proc_pid_smaps_vsyscall_2[] =
    "ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]\n"
    "Size:                  4 kB\n"
    "KernelPageSize:        4 kB\n"
    "MMUPageSize:           4 kB\n"
    "Rss:                   0 kB\n"
    "Pss:                   0 kB\n"
    "Pss_Dirty:             0 kB\n"
    "Shared_Clean:          0 kB\n"
    "Shared_Dirty:          0 kB\n"
    "Private_Clean:         0 kB\n"
    "Private_Dirty:         0 kB\n"
    "Referenced:            0 kB\n"
    "Anonymous:             0 kB\n"
    "KSM:                   0 kB\n"
    "LazyFree:              0 kB\n"
    "AnonHugePages:         0 kB\n"
    "ShmemPmdMapped:        0 kB\n"
    "FilePmdMapped:         0 kB\n"
    "Shared_Hugetlb:        0 kB\n"
    "Private_Hugetlb:       0 kB\n"
    "Swap:                  0 kB\n"
    "SwapPss:               0 kB\n"
    "Locked:                0 kB\n"
    "THPeligible:           0\n"
    ;
#[no_mangle]
unsafe extern "C" fn sigaction_SIGSEGV(_: c_int, __: *mut siginfo_t, ___: *mut c_void) {
    static void sigaction_SIGSEGV(int _, siginfo_t *__, void *___)
    {
    _exit(EXIT_FAILURE);
    }

#[no_mangle]
unsafe extern "C" fn sigaction_SIGSEGV_vsyscall(_: c_int, __: *mut siginfo_t, ___: *mut c_void) {
    static void sigaction_SIGSEGV_vsyscall(int _, siginfo_t *__, void *___)
    {
    _exit(g_vsyscall);
    }
//
// vsyscall page can't be unmapped, probe it directly.
//
#[no_mangle]
unsafe extern "C" fn vsyscall() {
    static void vsyscall(void)
    {
    pid_t pid;
    int wstatus;
    pid = fork();
    if (pid < 0) {
    fprintf(stderr, "fork, errno %d\n", errno);
    exit(1);
    }
    if (pid == 0) {
    setrlimit(RLIMIT_CORE, &(struct rlimit){});
// Hide "segfault at ffffffffff600000" messages.
    let mut act: sigaction = {};
    act.sa_flags = SA_SIGINFO;
    act.sa_sigaction = sigaction_SIGSEGV_vsyscall;
    sigaction(SIGSEGV, &act, core::ptr::null_mut());
    g_vsyscall = 0;
// gettimeofday(NULL, NULL);
    let mut rax: u64 = 0xffffffffff600000;
    asm volatile (
    "call *%[rax]"
    : [rax] "+a" (rax)
    : "D" (core::ptr::null_mut()), "S" (core::ptr::null_mut())
    : "rcx", "r11"
    );
    g_vsyscall = 1;
// (volatile int *)0xffffffffff600000UL;
    g_vsyscall = 2;
    exit(g_vsyscall);
    }
    waitpid(pid, &wstatus, 0);
    if (WIFEXITED(wstatus)) {
    g_vsyscall = WEXITSTATUS(wstatus);
    } else {
    fprintf(stderr, "error: vsyscall wstatus %08x\n", wstatus);
    exit(1);
    }
    }

#[no_mangle]
unsafe extern "C" fn test_proc_pid_maps(pid: pid_t) -> c_int {
    static int test_proc_pid_maps(pid_t pid)
    {
    char buf[4096];
    snprintf(buf, sizeof(buf), "/proc/%u/maps", pid);
    let mut fd: c_int = open(buf, O_RDONLY);
    if (fd == -1) {
    perror("open /proc/${pid}/maps");
    return EXIT_FAILURE;
    } else {
    let mut rv: isize = read(fd, buf, sizeof(buf));
    close(fd);
    if (g_vsyscall == 0) {
    assert(rv == 0);
    } else {
    let mut len: usize = strlen(g_proc_pid_maps_vsyscall);
    assert(rv == len);
    assert(memcmp(buf, g_proc_pid_maps_vsyscall, len) == 0);
    }
    return EXIT_SUCCESS;
    }
    }
#[no_mangle]
unsafe extern "C" fn test_proc_pid_numa_maps(pid: pid_t) -> c_int {
    static int test_proc_pid_numa_maps(pid_t pid)
    {
    char buf[4096];
    snprintf(buf, sizeof(buf), "/proc/%u/numa_maps", pid);
    let mut fd: c_int = open(buf, O_RDONLY);
    if (fd == -1) {
    if (errno == ENOENT) {
//
// /proc/${pid}/numa_maps is under CONFIG_NUMA,
// it doesn't necessarily exist.
//
    return EXIT_SUCCESS;
    }
    perror("open /proc/${pid}/numa_maps");
    return EXIT_FAILURE;
    } else {
    let mut rv: isize = read(fd, buf, sizeof(buf));
    close(fd);
    assert(rv == 0);
    return EXIT_SUCCESS;
    }
    }
#[no_mangle]
unsafe extern "C" fn test_proc_pid_smaps(pid: pid_t) -> c_int {
    static int test_proc_pid_smaps(pid_t pid)
    {
    char buf[4096];
    snprintf(buf, sizeof(buf), "/proc/%u/smaps", pid);
    let mut fd: c_int = open(buf, O_RDONLY);
    if (fd == -1) {
    if (errno == ENOENT) {
//
// /proc/${pid}/smaps is under CONFIG_PROC_PAGE_MONITOR,
// it doesn't necessarily exist.
//
    return EXIT_SUCCESS;
    }
    perror("open /proc/${pid}/smaps");
    return EXIT_FAILURE;
    }
    let mut rv: isize = read(fd, buf, sizeof(buf));
    close(fd);
    assert(0 <= rv);
    assert(rv <= sizeof(buf));
    if (g_vsyscall == 0) {
    assert(rv == 0);
    } else {
    let mut len: usize = strlen(g_proc_pid_smaps_vsyscall);
    assert(rv > len);
    assert(memcmp(buf, g_proc_pid_smaps_vsyscall, len) == 0);
    if (g_protection_key_support) {

    assert(memmem(buf, rv, PROTECTION_KEY, strlen(PROTECTION_KEY)));
    }
    }
    return EXIT_SUCCESS;
    }
    static const char g_smaps_rollup[] =
    "00000000-00000000 ---p 00000000 00:00 0                                  [rollup]\n"
    "Rss:                   0 kB\n"
    "Pss:                   0 kB\n"
    "Pss_Dirty:             0 kB\n"
    "Pss_Anon:              0 kB\n"
    "Pss_File:              0 kB\n"
    "Pss_Shmem:             0 kB\n"
    "Shared_Clean:          0 kB\n"
    "Shared_Dirty:          0 kB\n"
    "Private_Clean:         0 kB\n"
    "Private_Dirty:         0 kB\n"
    "Referenced:            0 kB\n"
    "Anonymous:             0 kB\n"
    "KSM:                   0 kB\n"
    "LazyFree:              0 kB\n"
    "AnonHugePages:         0 kB\n"
    "ShmemPmdMapped:        0 kB\n"
    "FilePmdMapped:         0 kB\n"
    "Shared_Hugetlb:        0 kB\n"
    "Private_Hugetlb:       0 kB\n"
    "Swap:                  0 kB\n"
    "SwapPss:               0 kB\n"
    "Locked:                0 kB\n"
    ;
#[no_mangle]
unsafe extern "C" fn test_proc_pid_smaps_rollup(pid: pid_t) -> c_int {
    static int test_proc_pid_smaps_rollup(pid_t pid)
    {
    char buf[4096];
    snprintf(buf, sizeof(buf), "/proc/%u/smaps_rollup", pid);
    let mut fd: c_int = open(buf, O_RDONLY);
    if (fd == -1) {
    if (errno == ENOENT) {
//
// /proc/${pid}/smaps_rollup is under CONFIG_PROC_PAGE_MONITOR,
// it doesn't necessarily exist.
//
    return EXIT_SUCCESS;
    }
    perror("open /proc/${pid}/smaps_rollup");
    return EXIT_FAILURE;
    } else {
    let mut rv: isize = read(fd, buf, sizeof(buf));
    close(fd);
    assert(rv == sizeof(g_smaps_rollup) - 1);
    assert(memcmp(buf, g_smaps_rollup, sizeof(g_smaps_rollup) - 1) == 0);
    return EXIT_SUCCESS;
    }
    }
    static const char *parse_u64(const char *p, const char *const end, uint64_t *rv)
    {
// rv = 0;
    for (; p != end; p += 1) {
    if ('0' <= *p && *p <= '9') {
    assert(!__builtin_mul_overflow(*rv, 10, rv));
    assert(!__builtin_add_overflow(*rv, *p - '0', rv));
    } else {
    break;
    }
    }
    assert(p != end);
    return p;
    }
//
// There seems to be 2 types of valid output:
// "0 A A B 0 0 0\n" for dynamic exeuctables,
// "0 0 0 B 0 0 0\n" for static executables.
//
#[no_mangle]
unsafe extern "C" fn test_proc_pid_statm(pid: pid_t) -> c_int {
    static int test_proc_pid_statm(pid_t pid)
    {
    char buf[4096];
    snprintf(buf, sizeof(buf), "/proc/%u/statm", pid);
    let mut fd: c_int = open(buf, O_RDONLY);
    if (fd == -1) {
    perror("open /proc/${pid}/statm");
    return EXIT_FAILURE;
    }
    let mut rv: isize = read(fd, buf, sizeof(buf));
    close(fd);
    assert(rv >= 0);
    assert(rv <= sizeof(buf));
    const char *p = buf;
    let mut end: *const char const = p + rv;
// size
    assert(p != end && *p++ == '0');
    assert(p != end && *p++ == ' ');
    uint64_t resident;
    p = parse_u64(p, end, &resident);
    assert(p != end && *p++ == ' ');
    uint64_t shared;
    p = parse_u64(p, end, &shared);
    assert(p != end && *p++ == ' ');
    uint64_t text;
    p = parse_u64(p, end, &text);
    assert(p != end && *p++ == ' ');
    assert(p != end && *p++ == '0');
    assert(p != end && *p++ == ' ');
// data
    assert(p != end && *p++ == '0');
    assert(p != end && *p++ == ' ');
    assert(p != end && *p++ == '0');
    assert(p != end && *p++ == '\n');
    assert(p == end);
//
// "text" is "mm->end_code - mm->start_code" at execve(2) time.
// munmap() doesn't change it. It can be anything (just link
// statically). It can't be 0 because executing to this point
// implies at least 1 page of code.
//
    assert(text > 0);
//
// These two are always equal. Always 0 for statically linked
// executables and sometimes 0 for dynamically linked executables.
// There is no way to tell one from another without parsing ELF
// which is too much for this test.
//
    assert(resident == shared);
    return EXIT_SUCCESS;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    let mut rv: c_int = EXIT_SUCCESS;

    vsyscall();

    switch (g_vsyscall) {
    case 0:
    g_proc_pid_maps_vsyscall  = proc_pid_maps_vsyscall_0;
    g_proc_pid_smaps_vsyscall = proc_pid_smaps_vsyscall_0;
    break;
    case 1:
    g_proc_pid_maps_vsyscall  = proc_pid_maps_vsyscall_1;
    g_proc_pid_smaps_vsyscall = proc_pid_smaps_vsyscall_1;
    break;
    case 2:
    g_proc_pid_maps_vsyscall  = proc_pid_maps_vsyscall_2;
    g_proc_pid_smaps_vsyscall = proc_pid_smaps_vsyscall_2;
    break;
    default:
    abort();
    }
    g_protection_key_support = protection_key_support();
    let mut pid: pid_t = fork();
    if (pid == -1) {
    perror("fork");
    return EXIT_FAILURE;
    } else if (pid == 0) {
    rv = ptrace(PTRACE_TRACEME, 0, core::ptr::null_mut(), core::ptr::null_mut());
    if (rv != 0) {
    if (errno == EPERM) {
    fprintf(stderr,
    "Did you know? ptrace(PTRACE_TRACEME) doesn't work under strace.\n"
    );
    kill(getppid(), SIGTERM);
    return EXIT_FAILURE;
    }
    perror("ptrace PTRACE_TRACEME");
    return EXIT_FAILURE;
    }
//
// Hide "segfault at ..." messages. Signal handler won't run.
//
    let mut act: sigaction = {};
    act.sa_flags = SA_SIGINFO;
    act.sa_sigaction = sigaction_SIGSEGV;
    sigaction(SIGSEGV, &act, core::ptr::null_mut());

    munmap(core::ptr::null_mut(), ((size_t)1 << 47) - 4096);

    {
    size_t len;
    for (len = -4096;; len -= 4096) {
    munmap(core::ptr::null_mut(), len);
    }
    }

    return EXIT_FAILURE;
    } else {
//
// TODO find reliable way to signal parent that munmap(2) completed.
// Child can't do it directly because it effectively doesn't exist
// anymore. Looking at child's VM files isn't 100% reliable either:
// due to a bug they may not become empty or empty-like.
//
    sleep(1);
    if (rv == EXIT_SUCCESS) {
    rv = test_proc_pid_maps(pid);
    }
    if (rv == EXIT_SUCCESS) {
    rv = test_proc_pid_numa_maps(pid);
    }
    if (rv == EXIT_SUCCESS) {
    rv = test_proc_pid_smaps(pid);
    }
    if (rv == EXIT_SUCCESS) {
    rv = test_proc_pid_smaps_rollup(pid);
    }
    if (rv == EXIT_SUCCESS) {
    rv = test_proc_pid_statm(pid);
    }
// Cut the rope.
    int wstatus;
    waitpid(pid, &wstatus, 0);
    assert(WIFSTOPPED(wstatus));
    assert(WSTOPSIG(wstatus) == SIGSEGV);
    }
    return rv;
    }

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return 4;
    }
