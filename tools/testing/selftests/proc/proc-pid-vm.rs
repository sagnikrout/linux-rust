//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/proc/proc-pid-vm.c
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
// Copyright (c) 2019 Alexey Dobriyan <adobriyan@gmail.com>
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
// Fork and exec tiny 1 page executable which precisely controls its VM.
// Test /proc/$PID/maps
// Test /proc/$PID/smaps
// Test /proc/$PID/smaps_rollup
// Test /proc/$PID/statm
//
// FIXME require CONFIG_TMPFS which can be disabled
// FIXME test other values from "smaps"
// FIXME support other archs
//

#[no_mangle]
pub unsafe extern "C" fn sys_execveat(dirfd: c_int, pathname: *const c_char, argv: *mut c_char, envp: *mut c_char, flags: c_int) -> c_long {
    static inline long sys_execveat(int dirfd, const char *pathname, char **argv, char **envp, int flags)
    {
    return syscall(SYS_execveat, dirfd, pathname, argv, envp, flags);
    }
#[no_mangle]
unsafe extern "C" fn make_private_tmp() {
    static void make_private_tmp(void)
    {
    if (unshare(CLONE_NEWNS) == -1) {
    if (errno == ENOSYS || errno == EPERM) {
    exit(4);
    }
    exit(1);
    }
    if (mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_PRIVATE|MS_REC, core::ptr::null_mut()) == -1) {
    exit(1);
    }
    if (mount(core::ptr::null_mut(), "/tmp", "tmpfs", 0, core::ptr::null_mut()) == -1) {
    exit(1);
    }
    }
    let mut pid: static pid_t = -1;
#[no_mangle]
unsafe extern "C" fn ate() {
    static void ate(void)
    {
    if (pid > 0) {
    kill(pid, SIGTERM);
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elf64_hdr {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elf64_phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

pub const PAGE_SIZE: c_int = 4096;

pub const MAPS_OFFSET: c_int = 73;
pub const syscall: c_uint = 0x0f, 0x05;

    0x48, 0xbf,	\
    (x)&0xff, ((x)>>8)&0xff, ((x)>>16)&0xff, ((x)>>24)&0xff,	\
    ((x)>>32)&0xff, ((x)>>40)&0xff, ((x)>>48)&0xff, ((x)>>56)&0xff

    0x48, 0xbe,	\
    (x)&0xff, ((x)>>8)&0xff, ((x)>>16)&0xff, ((x)>>24)&0xff,	\
    ((x)>>32)&0xff, ((x)>>40)&0xff, ((x)>>48)&0xff, ((x)>>56)&0xff

    0xb8, (x)&0xff, ((x)>>8)&0xff, ((x)>>16)&0xff, ((x)>>24)&0xff
    static const uint8_t payload[] = {
// Casually unmap stack, vDSO and everything else.
// munmap
    mov_rdi(VADDR + 4096),
    mov_rsi((1ULL << 47) - 4096 - VADDR - 4096),
    mov_eax(11),
    syscall,
// Ping parent.
// write(0, &c, 1);
    0x31, 0xff,					/* xor edi, edi */
    0x48, 0x8d, 0x35, 0x00, 0x00, 0x00, 0x00,	/* lea rsi, [rip] */
    0xba, 0x01, 0x00, 0x00, 0x00,			/* mov edx, 1 */
    mov_eax(1),
    syscall,
// 1: pause();
    mov_eax(34),
    syscall,
    0xeb, 0xf7,	/* jmp 1b */
    };
#[no_mangle]
unsafe extern "C" fn make_exe(payload: *const u8, len: usize) -> c_int {
    static int make_exe(const uint8_t *payload, size_t len)
    {
    struct elf64_hdr h;
    struct elf64_phdr ph;
    struct iovec iov[3] = {
    {&h, sizeof(struct elf64_hdr)},
    {&ph, sizeof(struct elf64_phdr)},
    {(void *)payload, len},
    };
    int fd, fd1;
    char buf[64];
    memset(&h, 0, sizeof(h));
    h.e_ident[0] = 0x7f;
    h.e_ident[1] = 'E';
    h.e_ident[2] = 'L';
    h.e_ident[3] = 'F';
    h.e_ident[4] = 2;
    h.e_ident[5] = 1;
    h.e_ident[6] = 1;
    h.e_ident[7] = 0;
    h.e_type = 2;
    h.e_machine = 0x3e;
    h.e_version = 1;
    h.e_entry = VADDR + sizeof(struct elf64_hdr) + sizeof(struct elf64_phdr);
    h.e_phoff = sizeof(struct elf64_hdr);
    h.e_shoff = 0;
    h.e_flags = 0;
    h.e_ehsize = sizeof(struct elf64_hdr);
    h.e_phentsize = sizeof(struct elf64_phdr);
    h.e_phnum = 1;
    h.e_shentsize = 0;
    h.e_shnum = 0;
    h.e_shstrndx = 0;
    memset(&ph, 0, sizeof(ph));
    ph.p_type = 1;
    ph.p_flags = (1<<2)|1;
    ph.p_offset = 0;
    ph.p_vaddr = VADDR;
    ph.p_paddr = 0;
    ph.p_filesz = sizeof(struct elf64_hdr) + sizeof(struct elf64_phdr) + len;
    ph.p_memsz = sizeof(struct elf64_hdr) + sizeof(struct elf64_phdr) + len;
    ph.p_align = 4096;
    fd = openat(AT_FDCWD, "/tmp", O_WRONLY|O_EXCL|O_TMPFILE, 0700);
    if (fd == -1) {
    exit(1);
    }
    if (writev(fd, iov, 3) != sizeof(struct elf64_hdr) + sizeof(struct elf64_phdr) + len) {
    exit(1);
    }
// Avoid ETXTBSY on exec.
    snprintf(buf, sizeof(buf), "/proc/self/fd/%u", fd);
    fd1 = open(buf, O_RDONLY|O_CLOEXEC);
    close(fd);
    return fd1;
    }

//
// 0: vsyscall VMA doesn't exist	vsyscall=none
// 1: vsyscall VMA is --xp		vsyscall=xonly
// 2: vsyscall VMA is r-xp		vsyscall=emulate
//
    static volatile int g_vsyscall;
    static const char *str_vsyscall __maybe_unused;
    static const char str_vsyscall_0[] __maybe_unused = "";
    static const char str_vsyscall_1[] __maybe_unused =
    "ffffffffff600000-ffffffffff601000 --xp 00000000 00:00 0                  [vsyscall]\n";
    static const char str_vsyscall_2[] __maybe_unused =
    "ffffffffff600000-ffffffffff601000 r-xp 00000000 00:00 0                  [vsyscall]\n";

#[no_mangle]
unsafe extern "C" fn sigaction_SIGSEGV(_: c_int, __: *mut siginfo_t, ___: *mut c_void) {
    static void sigaction_SIGSEGV(int _, siginfo_t *__, void *___)
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
    let mut rlim: rlimit = {0, 0};
    (void)setrlimit(RLIMIT_CORE, &rlim);
// Hide "segfault at ffffffffff600000" messages.
    struct sigaction act;
    memset(&act, 0, sizeof(struct sigaction));
    act.sa_flags = SA_SIGINFO;
    act.sa_sigaction = sigaction_SIGSEGV;
    (void)sigaction(SIGSEGV, &act, core::ptr::null_mut());
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
    fprintf(stderr, "error: wstatus %08x\n", wstatus);
    exit(1);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    int pipefd[2];
    int exec_fd;
    vsyscall();
    switch (g_vsyscall) {
    case 0:
    str_vsyscall = str_vsyscall_0;
    break;
    case 1:
    str_vsyscall = str_vsyscall_1;
    break;
    case 2:
    str_vsyscall = str_vsyscall_2;
    break;
    default:
    abort();
    }
    atexit(ate);
    make_private_tmp();
// Reserve fd 0 for 1-byte pipe ping from child.
    close(0);
    if (open("/", O_RDONLY|O_DIRECTORY|O_PATH) != 0) {
    return 1;
    }
    exec_fd = make_exe(payload, sizeof(payload));
    if (pipe(pipefd) == -1) {
    return 1;
    }
    if (dup2(pipefd[1], 0) != 0) {
    return 1;
    }
    pid = fork();
    if (pid == -1) {
    return 1;
    }
    if (pid == 0) {
    sys_execveat(exec_fd, "", core::ptr::null_mut(), core::ptr::null_mut(), AT_EMPTY_PATH);
    return 1;
    }
    char _;
    if (read(pipefd[0], &_, 1) != 1) {
    return 1;
    }
    struct stat st;
    if (fstat(exec_fd, &st) == -1) {
    return 1;
    }
// Generate "head -n1 /proc/$PID/maps"
    char buf0[256];
    memset(buf0, ' ', sizeof(buf0));
    int len = snprintf(buf0, sizeof(buf0),
    "%08lx-%08lx r-xp 00000000 %02lx:%02lx %llu",
    VADDR, VADDR + PAGE_SIZE,
    MAJOR(st.st_dev), MINOR(st.st_dev),
    (unsigned long long)st.st_ino);
    buf0[len] = ' ';
    snprintf(buf0 + MAPS_OFFSET, sizeof(buf0) - MAPS_OFFSET,
    "/tmp/#%llu (deleted)\n", (unsigned long long)st.st_ino);
// Test /proc/$PID/maps
    {
    let mut len: usize = strlen(buf0) + strlen(str_vsyscall);
    char buf[256];
    ssize_t rv;
    int fd;
    snprintf(buf, sizeof(buf), "/proc/%u/maps", pid);
    fd = open(buf, O_RDONLY);
    if (fd == -1) {
    return 1;
    }
    rv = read(fd, buf, sizeof(buf));
    assert(rv == len);
    assert(memcmp(buf, buf0, strlen(buf0)) == 0);
    if (g_vsyscall > 0) {
    assert(memcmp(buf + strlen(buf0), str_vsyscall, strlen(str_vsyscall)) == 0);
    }
    }
// Test /proc/$PID/smaps
    {
    char buf[4096];
    ssize_t rv;
    int fd;
    snprintf(buf, sizeof(buf), "/proc/%u/smaps", pid);
    fd = open(buf, O_RDONLY);
    if (fd == -1) {
    return 1;
    }
    rv = read(fd, buf, sizeof(buf));
    assert(0 <= rv && rv <= sizeof(buf));
    assert(rv >= strlen(buf0));
    assert(memcmp(buf, buf0, strlen(buf0)) == 0);

    assert(memmem(buf, rv, RSS1, strlen(RSS1)) ||
    memmem(buf, rv, RSS2, strlen(RSS2)));
    assert(memmem(buf, rv, PSS1, strlen(PSS1)) ||
    memmem(buf, rv, PSS2, strlen(PSS2)));
    static const char *S[] = {
    "Size:                  4 kB\n",
    "KernelPageSize:        4 kB\n",
    "MMUPageSize:           4 kB\n",
    "Anonymous:             0 kB\n",
    "AnonHugePages:         0 kB\n",
    "Shared_Hugetlb:        0 kB\n",
    "Private_Hugetlb:       0 kB\n",
    "Locked:                0 kB\n",
    };
    int i;
    for (i = 0; i < ARRAY_SIZE(S); i++) {
    assert(memmem(buf, rv, S[i], strlen(S[i])));
    }
    if (g_vsyscall > 0) {
    assert(memmem(buf, rv, str_vsyscall, strlen(str_vsyscall)));
    }
    }
// Test /proc/$PID/smaps_rollup
    {
    char bufr[256];
    memset(bufr, ' ', sizeof(bufr));
    len = snprintf(bufr, sizeof(bufr),
    "%08lx-%08lx ---p 00000000 00:00 0",
    VADDR, VADDR + PAGE_SIZE);
    bufr[len] = ' ';
    snprintf(bufr + MAPS_OFFSET, sizeof(bufr) - MAPS_OFFSET,
    "[rollup]\n");
    char buf[1024];
    ssize_t rv;
    int fd;
    snprintf(buf, sizeof(buf), "/proc/%u/smaps_rollup", pid);
    fd = open(buf, O_RDONLY);
    if (fd == -1) {
    return 1;
    }
    rv = read(fd, buf, sizeof(buf));
    assert(0 <= rv && rv <= sizeof(buf));
    assert(rv >= strlen(bufr));
    assert(memcmp(buf, bufr, strlen(bufr)) == 0);
    assert(memmem(buf, rv, RSS1, strlen(RSS1)) ||
    memmem(buf, rv, RSS2, strlen(RSS2)));
    assert(memmem(buf, rv, PSS1, strlen(PSS1)) ||
    memmem(buf, rv, PSS2, strlen(PSS2)));
    static const char *S[] = {
    "Anonymous:             0 kB\n",
    "AnonHugePages:         0 kB\n",
    "Shared_Hugetlb:        0 kB\n",
    "Private_Hugetlb:       0 kB\n",
    "Locked:                0 kB\n",
    };
    int i;
    for (i = 0; i < ARRAY_SIZE(S); i++) {
    assert(memmem(buf, rv, S[i], strlen(S[i])));
    }
    }
// Test /proc/$PID/statm
    {
    char buf[64];
    ssize_t rv;
    int fd;
    snprintf(buf, sizeof(buf), "/proc/%u/statm", pid);
    fd = open(buf, O_RDONLY);
    if (fd == -1) {
    return 1;
    }
    rv = read(fd, buf, sizeof(buf));
    assert(rv == 7 * 2);
    assert(buf[0] == '1');	/* .total_vm */
    assert(buf[1] == ' ');
    assert(buf[2] == '0' || buf[2] == '1');	/* rss */
    assert(buf[3] == ' ');
    assert(buf[4] == '0' || buf[2] == '1');	/* file rss */
    assert(buf[5] == ' ');
    assert(buf[6] == '1');	/* ELF executable segments */
    assert(buf[7] == ' ');
    assert(buf[8] == '0');
    assert(buf[9] == ' ');
    assert(buf[10] == '0');	/* .data_vm + .stack_vm */
    assert(buf[11] == ' ');
    assert(buf[12] == '0');
    assert(buf[13] == '\n');
    }
// Test PROCMAP_QUERY ioctl() for /proc/$PID/maps
    {
    char path_buf[256], exp_path_buf[256];
    struct procmap_query q;
    int fd, err;
    snprintf(path_buf, sizeof(path_buf), "/proc/%u/maps", pid);
    fd = open(path_buf, O_RDONLY);
    if (fd == -1)
    return 1;
// CASE 1: exact MATCH at VADDR
    memset(&q, 0, sizeof(q));
    q.size = sizeof(q);
    q.query_addr = VADDR;
    q.query_flags = 0;
    q.vma_name_addr = (__u64)(unsigned long)path_buf;
    q.vma_name_size = sizeof(path_buf);
    err = ioctl(fd, PROCMAP_QUERY, &q);
    assert(err == 0);
    assert(q.query_addr == VADDR);
    assert(q.query_flags == 0);
    assert(q.vma_flags == (PROCMAP_QUERY_VMA_READABLE | PROCMAP_QUERY_VMA_EXECUTABLE));
    assert(q.vma_start == VADDR);
    assert(q.vma_end == VADDR + PAGE_SIZE);
    assert(q.vma_page_size == PAGE_SIZE);
    assert(q.vma_offset == 0);
    assert(q.inode == st.st_ino);
    assert(q.dev_major == MAJOR(st.st_dev));
    assert(q.dev_minor == MINOR(st.st_dev));
    snprintf(exp_path_buf, sizeof(exp_path_buf),
    "/tmp/#%llu (deleted)", (unsigned long long)st.st_ino);
    assert(q.vma_name_size == strlen(exp_path_buf) + 1);
    assert(strcmp(path_buf, exp_path_buf) == 0);
// CASE 2: NO MATCH at VADDR-1
    memset(&q, 0, sizeof(q));
    q.size = sizeof(q);
    q.query_addr = VADDR - 1;
    q.query_flags = 0; /* exact match */
    err = ioctl(fd, PROCMAP_QUERY, &q);
    err = err < 0 ? -errno : 0;
    assert(err == -ENOENT);
// CASE 3: MATCH COVERING_OR_NEXT_VMA at VADDR - 1
    memset(&q, 0, sizeof(q));
    q.size = sizeof(q);
    q.query_addr = VADDR - 1;
    q.query_flags = PROCMAP_QUERY_COVERING_OR_NEXT_VMA;
    err = ioctl(fd, PROCMAP_QUERY, &q);
    assert(err == 0);
    assert(q.query_addr == VADDR - 1);
    assert(q.query_flags == PROCMAP_QUERY_COVERING_OR_NEXT_VMA);
    assert(q.vma_start == VADDR);
    assert(q.vma_end == VADDR + PAGE_SIZE);
// CASE 4: NO MATCH at VADDR + PAGE_SIZE
    memset(&q, 0, sizeof(q));
    q.size = sizeof(q);
    q.query_addr = VADDR + PAGE_SIZE; /* point right after the VMA */
    q.query_flags = PROCMAP_QUERY_COVERING_OR_NEXT_VMA;
    err = ioctl(fd, PROCMAP_QUERY, &q);
    err = err < 0 ? -errno : 0;
    assert(err == -ENOENT);
// CASE 5: NO MATCH WRITABLE at VADDR
    memset(&q, 0, sizeof(q));
    q.size = sizeof(q);
    q.query_addr = VADDR;
    q.query_flags = PROCMAP_QUERY_VMA_WRITABLE;
    err = ioctl(fd, PROCMAP_QUERY, &q);
    err = err < 0 ? -errno : 0;
    assert(err == -ENOENT);
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return 4;
    }
