//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/test_shadow_stack.c
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
//
// This program test's basic kernel shadow stack support. It enables shadow
// stack manual via the arch_prctl(), instead of relying on glibc. It's
// Makefile doesn't compile with shadow stack support, so it doesn't rely on
// any particular glibc. As a result it can't do any operations that require
// special glibc shadow stack support (longjmp(), swapcontext(), etc). Just
// stick to the basics and hope the compiler doesn't do anything strange.
//
// Macro flag: #define _GNU_SOURCE

//
// Define the ABI defines if needed, so people can run the tests
// without building the headers.
//

pub const __NR_map_shadow_stack: c_int = 453;

pub const ARCH_SHSTK_ENABLE: c_uint = 0x5001;
pub const ARCH_SHSTK_DISABLE: c_uint = 0x5002;
pub const ARCH_SHSTK_LOCK: c_uint = 0x5003;
pub const ARCH_SHSTK_UNLOCK: c_uint = 0x5004;
pub const ARCH_SHSTK_STATUS: c_uint = 0x5005;

pub const NT_X86_SHSTK: c_uint = 0x204;

pub const SS_SIZE: c_uint = 0x200000;
pub const PAGE_SIZE: c_uint = 0x1000;

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    printf("[SKIP]\tCompiler does not support CET.\n");
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn write_shstk(addr: *mut c_ulong, val: c_ulong) {
    void write_shstk(unsigned long *addr, unsigned long val)
    {
    asm volatile("wrssq %[val], (%[addr])\n"
    : "=m" (addr)
    : [addr] "r" (addr), [val] "r" (val));
    }
#[no_mangle]
pub unsafe extern "C" fn __attribute__(get_ssp(void: (always_inline))) -> c_ulong {
    static inline unsigned long __attribute__((always_inline)) get_ssp(void)
    {
    let mut ret: c_ulong = 0;
    asm volatile("xor %0, %0; rdsspq %0" : "=r" (ret));
    return ret;
    }
//
// For use in inline enablement of shadow stack.
//
// The program can't return from the point where shadow stack gets enabled
// because there will be no address on the shadow stack. So it can't use
// syscall() for enablement, since it is a function.
//
// Based on code from nolibc.h. Keep a copy here because this can't pull in all
// of nolibc.h.
//

    ({								\
    long _ret;						\
    register long _num  asm("eax") = __NR_arch_prctl;	\
    register long _arg1 asm("rdi") = (long)(arg1);		\
    register long _arg2 asm("rsi") = (long)(arg2);		\
    \
    asm volatile (						\
    "syscall\n"					\
    : "=a"(_ret)					\
    : "r"(_arg1), "r"(_arg2),			\
    "0"(_num)					\
    : "rcx", "r11", "memory", "cc"			\
    );							\
    _ret;							\
    })
    void *create_shstk(void *addr)
    {
    return (void *)syscall(__NR_map_shadow_stack, addr, SS_SIZE, SHADOW_STACK_SET_TOKEN);
    }
    void *create_normal_mem(void *addr)
    {
    return mmap(addr, SS_SIZE, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS, 0, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn free_shstk(shstk: *mut c_void) {
    void free_shstk(void *shstk)
    {
    munmap(shstk, SS_SIZE);
    }
#[no_mangle]
pub unsafe extern "C" fn reset_shstk(shstk: *mut c_void) -> c_int {
    int reset_shstk(void *shstk)
    {
    return madvise(shstk, SS_SIZE, MADV_DONTNEED);
    }
#[no_mangle]
pub unsafe extern "C" fn try_shstk(new_ssp: c_ulong) {
    void try_shstk(unsigned long new_ssp)
    {
    unsigned long ssp;
    printf("[INFO]\tnew_ssp = %lx, *new_ssp = %lx\n",
    new_ssp, *((unsigned long *)new_ssp));
    ssp = get_ssp();
    printf("[INFO]\tchanging ssp from %lx to %lx\n", ssp, new_ssp);
    asm volatile("rstorssp (%0)\n":: "r" (new_ssp));
    asm volatile("saveprevssp");
    printf("[INFO]\tssp is now %lx\n", get_ssp());
// Switch back to original shadow stack
    ssp -= 8;
    asm volatile("rstorssp (%0)\n":: "r" (ssp));
    asm volatile("saveprevssp");
    }
#[no_mangle]
pub unsafe extern "C" fn test_shstk_pivot() -> c_int {
    int test_shstk_pivot(void)
    {
    void *shstk = create_shstk(0);
    if (shstk == MAP_FAILED) {
    printf("[FAIL]\tError creating shadow stack: %d\n", errno);
    return 1;
    }
    try_shstk((unsigned long)shstk + SS_SIZE - 8);
    free_shstk(shstk);
    printf("[OK]\tShadow stack pivot\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_shstk_faults() -> c_int {
    int test_shstk_faults(void)
    {
    unsigned long *shstk = create_shstk(0);
// Read shadow stack, test if it's zero to not get read optimized out
    if (*shstk != 0)
    goto err;
// Wrss memory that was already read.
    write_shstk(shstk, 1);
    if (*shstk != 1)
    goto err;
// Page out memory, so we can wrss it again.
    if (reset_shstk((void *)shstk))
    goto err;
    write_shstk(shstk, 1);
    if (*shstk != 1)
    goto err;
    printf("[OK]\tShadow stack faults\n");
    return 0;
    err:
    return 1;
    }
    unsigned long saved_ssp;
    unsigned long saved_ssp_val;
    volatile bool segv_triggered;
#[no_mangle]
pub unsafe extern "C" fn __attribute__(violate_ss(void: (noinline))) {
    void __attribute__((noinline)) violate_ss(void)
    {
    saved_ssp = get_ssp();
    saved_ssp_val = *(unsigned long *)saved_ssp;
// Corrupt shadow stack
    printf("[INFO]\tCorrupting shadow stack\n");
    write_shstk((void *)saved_ssp, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn segv_handler(signum: c_int, si: *mut siginfo_t, uc: *mut c_void) {
    void segv_handler(int signum, siginfo_t *si, void *uc)
    {
    printf("[INFO]\tGenerated shadow stack violation successfully\n");
    segv_triggered = true;
// Fix shadow stack
    write_shstk((void *)saved_ssp, saved_ssp_val);
    }
#[no_mangle]
pub unsafe extern "C" fn test_shstk_violation() -> c_int {
    int test_shstk_violation(void)
    {
    let mut sa: sigaction = {};
    sa.sa_sigaction = segv_handler;
    sa.sa_flags = SA_SIGINFO;
    if (sigaction(SIGSEGV, &sa, core::ptr::null_mut()))
    return 1;
    segv_triggered = false;
// Make sure segv_triggered is set before violate_ss()
    asm volatile("" : : : "memory");
    violate_ss();
    signal(SIGSEGV, SIG_DFL);
    printf("[OK]\tShadow stack violation test\n");
    return !segv_triggered;
    }
// Gup test state
pub const MAGIC_VAL: c_uint = 0x12345678;
    bool is_shstk_access;
    void *shstk_ptr;
    int fd;
#[no_mangle]
pub unsafe extern "C" fn reset_test_shstk(addr: *mut c_void) {
    void reset_test_shstk(void *addr)
    {
    if (shstk_ptr)
    free_shstk(shstk_ptr);
    shstk_ptr = create_shstk(addr);
    }
#[no_mangle]
pub unsafe extern "C" fn test_access_fix_handler(signum: c_int, si: *mut siginfo_t, uc: *mut c_void) {
    void test_access_fix_handler(int signum, siginfo_t *si, void *uc)
    {
    printf("[INFO]\tViolation from %s\n", is_shstk_access ? "shstk access" : "normal write");
    segv_triggered = true;
// Fix shadow stack
    if (is_shstk_access) {
    reset_test_shstk(shstk_ptr);
    return;
    }
    free_shstk(shstk_ptr);
    create_normal_mem(shstk_ptr);
    }
#[no_mangle]
pub unsafe extern "C" fn test_shstk_access(ptr: *mut c_void) -> bool {
    bool test_shstk_access(void *ptr)
    {
    is_shstk_access = true;
    segv_triggered = false;
    write_shstk(ptr, MAGIC_VAL);
    asm volatile("" : : : "memory");
    return segv_triggered;
    }
#[no_mangle]
pub unsafe extern "C" fn test_write_access(ptr: *mut c_void) -> bool {
    bool test_write_access(void *ptr)
    {
    is_shstk_access = false;
    segv_triggered = false;
// (unsigned long *)ptr = MAGIC_VAL;
    asm volatile("" : : : "memory");
    return segv_triggered;
    }
#[no_mangle]
pub unsafe extern "C" fn gup_write(ptr: *mut c_void) -> bool {
    bool gup_write(void *ptr)
    {
    unsigned long val;
    lseek(fd, (unsigned long)ptr, SEEK_SET);
    if (write(fd, &val, sizeof(val)) < 0)
    return 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn gup_read(ptr: *mut c_void) -> bool {
    bool gup_read(void *ptr)
    {
    unsigned long val;
    lseek(fd, (unsigned long)ptr, SEEK_SET);
    if (read(fd, &val, sizeof(val)) < 0)
    return 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_gup() -> c_int {
    int test_gup(void)
    {
    let mut sa: sigaction = {};
    int status;
    pid_t pid;
    sa.sa_sigaction = test_access_fix_handler;
    sa.sa_flags = SA_SIGINFO;
    if (sigaction(SIGSEGV, &sa, core::ptr::null_mut()))
    return 1;
    segv_triggered = false;
    fd = open("/proc/self/mem", O_RDWR);
    if (fd == -1)
    return 1;
    reset_test_shstk(0);
    if (gup_read(shstk_ptr))
    return 1;
    if (test_shstk_access(shstk_ptr))
    return 1;
    printf("[INFO]\tGup read . shstk access success\n");
    reset_test_shstk(0);
    if (gup_write(shstk_ptr))
    return 1;
    if (test_shstk_access(shstk_ptr))
    return 1;
    printf("[INFO]\tGup write . shstk access success\n");
    reset_test_shstk(0);
    if (gup_read(shstk_ptr))
    return 1;
    if (!test_write_access(shstk_ptr))
    return 1;
    printf("[INFO]\tGup read . write access success\n");
    reset_test_shstk(0);
    if (gup_write(shstk_ptr))
    return 1;
    if (!test_write_access(shstk_ptr))
    return 1;
    printf("[INFO]\tGup write . write access success\n");
    close(fd);
// COW/gup test
    reset_test_shstk(0);
    pid = fork();
    if (!pid) {
    fd = open("/proc/self/mem", O_RDWR);
    if (fd == -1)
    exit(1);
    if (gup_write(shstk_ptr)) {
    close(fd);
    exit(1);
    }
    close(fd);
    exit(0);
    }
    waitpid(pid, &status, 0);
    if (WEXITSTATUS(status)) {
    printf("[FAIL]\tWrite in child failed\n");
    return 1;
    }
    if (*(unsigned long *)shstk_ptr == MAGIC_VAL) {
    printf("[FAIL]\tWrite in child wrote through to shared memory\n");
    return 1;
    }
    printf("[INFO]\tCow gup write . write access success\n");
    free_shstk(shstk_ptr);
    signal(SIGSEGV, SIG_DFL);
    printf("[OK]\tShadow gup test\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_mprotect() -> c_int {
    int test_mprotect(void)
    {
    let mut sa: sigaction = {};
    sa.sa_sigaction = test_access_fix_handler;
    sa.sa_flags = SA_SIGINFO;
    if (sigaction(SIGSEGV, &sa, core::ptr::null_mut()))
    return 1;
    segv_triggered = false;
// mprotect a shadow stack as read only
    reset_test_shstk(0);
    if (mprotect(shstk_ptr, SS_SIZE, PROT_READ) < 0) {
    printf("[FAIL]\tmprotect(PROT_READ) failed\n");
    return 1;
    }
// try to wrss it and fail
    if (!test_shstk_access(shstk_ptr)) {
    printf("[FAIL]\tShadow stack access to read-only memory succeeded\n");
    return 1;
    }
//
// The shadow stack was reset above to resolve the fault, make the new one
// read-only.
//
    if (mprotect(shstk_ptr, SS_SIZE, PROT_READ) < 0) {
    printf("[FAIL]\tmprotect(PROT_READ) failed\n");
    return 1;
    }
// then back to writable
    if (mprotect(shstk_ptr, SS_SIZE, PROT_WRITE | PROT_READ) < 0) {
    printf("[FAIL]\tmprotect(PROT_WRITE) failed\n");
    return 1;
    }
// then wrss to it and succeed
    if (test_shstk_access(shstk_ptr)) {
    printf("[FAIL]\tShadow stack access to mprotect() writable memory failed\n");
    return 1;
    }
    free_shstk(shstk_ptr);
    signal(SIGSEGV, SIG_DFL);
    printf("[OK]\tmprotect() test\n");
    return 0;
    }
    char zero[4096];
    static void *uffd_thread(void *arg)
    {
    struct uffdio_copy req;
    let mut uffd: c_int = *(int *)arg;
    struct uffd_msg msg;
    int ret;
    while (1) {
    ret = read(uffd, &msg, sizeof(msg));
    if (ret > 0)
    break;
#[no_mangle]
pub unsafe extern "C" fn if(EAGAIN: errno ==) -> else {
    else if (errno == EAGAIN)
    continue;
    return (void *)1;
    }
    req.dst = msg.arg.pagefault.address;
    req.src = (__u64)zero;
    req.len = 4096;
    req.mode = 0;
    if (ioctl(uffd, UFFDIO_COPY, &req))
    return (void *)1;
    return (void *)0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_userfaultfd() -> c_int {
    int test_userfaultfd(void)
    {
    struct uffdio_register uffdio_register;
    struct uffdio_api uffdio_api;
    let mut sa: sigaction = {};
    pthread_t thread;
    void *res;
    int uffd;
    sa.sa_sigaction = test_access_fix_handler;
    sa.sa_flags = SA_SIGINFO;
    if (sigaction(SIGSEGV, &sa, core::ptr::null_mut()))
    return 1;
    uffd = syscall(__NR_userfaultfd, O_CLOEXEC | O_NONBLOCK);
    if (uffd < 0) {
    printf("[SKIP]\tUserfaultfd unavailable.\n");
    return 0;
    }
    reset_test_shstk(0);
    uffdio_api.api = UFFD_API;
    uffdio_api.features = 0;
    if (ioctl(uffd, UFFDIO_API, &uffdio_api))
    goto err;
    uffdio_register.range.start = (__u64)shstk_ptr;
    uffdio_register.range.len = 4096;
    uffdio_register.mode = UFFDIO_REGISTER_MODE_MISSING;
    if (ioctl(uffd, UFFDIO_REGISTER, &uffdio_register))
    goto err;
    if (pthread_create(&thread, core::ptr::null_mut(), &uffd_thread, &uffd))
    goto err;
    reset_shstk(shstk_ptr);
    test_shstk_access(shstk_ptr);
    if (pthread_join(thread, &res))
    goto err;
    if (test_shstk_access(shstk_ptr))
    goto err;
    free_shstk(shstk_ptr);
    signal(SIGSEGV, SIG_DFL);
    if (!res)
    printf("[OK]\tUserfaultfd test\n");
    return !!res;
    err:
    free_shstk(shstk_ptr);
    close(uffd);
    signal(SIGSEGV, SIG_DFL);
    return 1;
    }
// Simple linked list for keeping track of mappings in test_guard_gap()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct node {
    pub next: *mut node,
    pub mapping: *mut c_void,
}

//
// This tests whether mmap will place other mappings in a shadow stack's guard
// gap. The steps are:
// 1. Finds an empty place by mapping and unmapping something.
// 2. Map a shadow stack in the middle of the known empty area.
// 3. Map a bunch of PAGE_SIZE mappings. These will use the search down
// direction, filling any gaps until it encounters the shadow stack's
// guard gap.
// 4. When a mapping lands below the shadow stack from step 2, then all
// of the above gaps are filled. The search down algorithm will have
// looked at the shadow stack gaps.
// 5. See if it landed in the gap.
//
#[no_mangle]
pub unsafe extern "C" fn test_guard_gap_other_gaps() -> c_int {
    int test_guard_gap_other_gaps(void)
    {
    void *free_area, *shstk, *test_map = (void *)0xFFFFFFFFFFFFFFFF;
    struct node *head = core::ptr::null_mut(), *cur;
    free_area = mmap(0, SS_SIZE * 3, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    munmap(free_area, SS_SIZE * 3);
    shstk = create_shstk(free_area + SS_SIZE);
    if (shstk == MAP_FAILED)
    return 1;
    while (test_map > shstk) {
    test_map = mmap(0, PAGE_SIZE, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (test_map == MAP_FAILED)
    return 1;
    cur = malloc(sizeof(*cur));
    cur.mapping = test_map;
    cur.next = head;
    head = cur;
    }
    while (head) {
    cur = head;
    head = cur.next;
    munmap(cur.mapping, PAGE_SIZE);
    free(cur);
    }
    free_shstk(shstk);
    if (shstk - test_map - PAGE_SIZE != PAGE_SIZE)
    return 1;
    printf("[OK]\tGuard gap test, other mapping's gaps\n");
    return 0;
    }
// Tests respecting the guard gap of the mapping getting placed
#[no_mangle]
pub unsafe extern "C" fn test_guard_gap_new_mappings_gaps() -> c_int {
    int test_guard_gap_new_mappings_gaps(void)
    {
    void *free_area, *shstk_start, *test_map = (void *)0xFFFFFFFFFFFFFFFF;
    struct node *head = core::ptr::null_mut(), *cur;
    let mut ret: c_int = 0;
    free_area = mmap(0, PAGE_SIZE * 4, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    munmap(free_area, PAGE_SIZE * 4);
// Test letting map_shadow_stack find a free space
    shstk_start = mmap(free_area, PAGE_SIZE, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (shstk_start == MAP_FAILED || shstk_start != free_area)
    return 1;
    while (test_map > shstk_start) {
    test_map = (void *)syscall(__NR_map_shadow_stack, 0, PAGE_SIZE, 0);
    if (test_map == MAP_FAILED) {
    printf("[INFO]\tmap_shadow_stack MAP_FAILED\n");
    ret = 1;
    break;
    }
    cur = malloc(sizeof(*cur));
    cur.mapping = test_map;
    cur.next = head;
    head = cur;
    if (test_map == free_area + PAGE_SIZE) {
    printf("[INFO]\tNew mapping has other mapping in guard gap!\n");
    ret = 1;
    break;
    }
    }
    while (head) {
    cur = head;
    head = cur.next;
    munmap(cur.mapping, PAGE_SIZE);
    free(cur);
    }
    munmap(shstk_start, PAGE_SIZE);
    if (!ret)
    printf("[OK]\tGuard gap test, placement mapping's gaps\n");
    return ret;
    }
//
// Too complicated to pull it out of the 32 bit header, but also get the
// 64 bit one needed above. Just define a copy here.
//
pub const __NR_compat_sigaction: c_int = 67;
//
// Call 32 bit signal handler to get 32 bit signals ABI. Make sure
// to push the registers that will get clobbered.
//
    int sigaction32(int signum, const struct sigaction *restrict act,
    struct sigaction *restrict oldact)
    {
    register long syscall_reg asm("eax") = __NR_compat_sigaction;
    register long signum_reg asm("ebx") = signum;
    register long act_reg asm("ecx") = (long)act;
    register long oldact_reg asm("edx") = (long)oldact;
    let mut ret: c_int = 0;
    asm volatile ("int $0x80;"
    : "=a"(ret), "=m"(oldact)
    : "r"(syscall_reg), "r"(signum_reg), "r"(act_reg),
    "r"(oldact_reg)
    : "r8", "r9", "r10", "r11"
    );
    return ret;
    }
    sigjmp_buf jmp_buffer;
#[no_mangle]
pub unsafe extern "C" fn segv_gp_handler(signum: c_int, si: *mut siginfo_t, uc: *mut c_void) {
    void segv_gp_handler(int signum, siginfo_t *si, void *uc)
    {
    segv_triggered = true;
//
// To work with old glibc, this can't rely on siglongjmp working with
// shadow stack enabled, so disable shadow stack before siglongjmp().
//
    ARCH_PRCTL(ARCH_SHSTK_DISABLE, ARCH_SHSTK_SHSTK);
    siglongjmp(jmp_buffer, -1);
    }
//
// Transition to 32 bit mode and check that a #GP triggers a segfault.
//
#[no_mangle]
pub unsafe extern "C" fn test_32bit() -> c_int {
    int test_32bit(void)
    {
    let mut sa: sigaction = {};
    struct sigaction *sa32;
// Create sigaction in 32 bit address range
    sa32 = mmap(0, 4096, PROT_READ | PROT_WRITE,
    MAP_32BIT | MAP_PRIVATE | MAP_ANONYMOUS, 0, 0);
    sa32.sa_flags = SA_SIGINFO;
    sa.sa_sigaction = segv_gp_handler;
    sa.sa_flags = SA_SIGINFO;
    if (sigaction(SIGSEGV, &sa, core::ptr::null_mut()))
    return 1;
    segv_triggered = false;
// Make sure segv_triggered is set before triggering the #GP
    asm volatile("" : : : "memory");
//
// Set handler to somewhere in 32 bit address space
//
    sa32.sa_handler = (void *)sa32;
    if (sigaction32(SIGUSR1, sa32, core::ptr::null_mut()))
    return 1;
    if (!sigsetjmp(jmp_buffer, 1))
    raise(SIGUSR1);
    if (segv_triggered)
    printf("[OK]\t32 bit test\n");
    return !segv_triggered;
    }
#[no_mangle]
unsafe extern "C" fn parse_uint_from_file(file: *const c_char, fmt: *const c_char) -> c_int {
    static int parse_uint_from_file(const char *file, const char *fmt)
    {
    int err, ret;
    FILE *f;
    f = fopen(file, "re");
    if (!f) {
    err = -errno;
    printf("failed to open '%s': %d\n", file, err);
    return err;
    }
    err = fscanf(f, fmt, &ret);
    if (err != 1) {
    err = err == EOF ? -EIO : -errno;
    printf("failed to parse '%s': %d\n", file, err);
    fclose(f);
    return err;
    }
    fclose(f);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn determine_uprobe_perf_type() -> c_int {
    static int determine_uprobe_perf_type(void)
    {
    const char *file = "/sys/bus/event_source/devices/uprobe/type";
    return parse_uint_from_file(file, "%d\n");
    }
#[no_mangle]
unsafe extern "C" fn determine_uprobe_retprobe_bit() -> c_int {
    static int determine_uprobe_retprobe_bit(void)
    {
    const char *file = "/sys/bus/event_source/devices/uprobe/format/retprobe";
    return parse_uint_from_file(file, "config:%d\n");
    }
#[no_mangle]
unsafe extern "C" fn get_uprobe_offset(addr: *const c_void) -> isize {
    static ssize_t get_uprobe_offset(const void *addr)
    {
    size_t start, end, base;
    char buf[256];
    let mut found: bool = false;
    FILE *f;
    f = fopen("/proc/self/maps", "r");
    if (!f)
    return -errno;
    while (fscanf(f, "%zx-%zx %s %zx %*[^\n]\n", &start, &end, buf, &base) == 4) {
    if (buf[2] == 'x' && (uintptr_t)addr >= start && (uintptr_t)addr < end) {
    found = true;
    break;
    }
    }
    fclose(f);
    if (!found)
    return -ESRCH;
    return (uintptr_t)addr - start + base;
    }
#[no_mangle]
pub unsafe extern "C" fn __attribute__(uretprobe_trigger(void: (noinline)) void) -> static {
    static __attribute__((noinline)) void uretprobe_trigger(void)
    {
    asm volatile ("");
    }
//
// This test setups return uprobe, which is sensitive to shadow stack
// (crashes without extra fix). After executing the uretprobe we fail
// the test if we receive SIGSEGV, no crash means we're good.
//
// Helper functions above borrowed from bpf selftests.
//
#[no_mangle]
unsafe extern "C" fn test_uretprobe() -> c_int {
    static int test_uretprobe(void)
    {
    let mut attr_sz: usize = sizeof(struct perf_event_attr);
    const char *file = "/proc/self/exe";
    int bit, fd = 0, type, err = 1;
    struct perf_event_attr attr;
    let mut sa: sigaction = {};
    ssize_t offset;
    type = determine_uprobe_perf_type();
    if (type < 0) {
    if (type == -ENOENT)
    printf("[SKIP]\tUretprobe test, uprobes are not available\n");
    return 0;
    }
    offset = get_uprobe_offset(uretprobe_trigger);
    if (offset < 0)
    return 1;
    bit = determine_uprobe_retprobe_bit();
    if (bit < 0)
    return 1;
    sa.sa_sigaction = segv_gp_handler;
    sa.sa_flags = SA_SIGINFO;
    if (sigaction(SIGSEGV, &sa, core::ptr::null_mut()))
    return 1;
// Setup return uprobe through perf event interface.
    memset(&attr, 0, attr_sz);
    attr.size = attr_sz;
    attr.type = type;
    attr.config = 1 << bit;
    attr.config1 = (__u64) (unsigned long) file;
    attr.config2 = offset;
    fd = syscall(__NR_perf_event_open, &attr, 0 /* pid */, -1 /* cpu */,
    -1 /* group_fd */, PERF_FLAG_FD_CLOEXEC);
    if (fd < 0)
    goto out;
    if (sigsetjmp(jmp_buffer, 1))
    goto out;
    ARCH_PRCTL(ARCH_SHSTK_ENABLE, ARCH_SHSTK_SHSTK);
//
// This either segfaults and goes through sigsetjmp above
// or succeeds and we're good.
//
    uretprobe_trigger();
    printf("[OK]\tUretprobe test\n");
    err = 0;
    out:
    ARCH_PRCTL(ARCH_SHSTK_DISABLE, ARCH_SHSTK_SHSTK);
    signal(SIGSEGV, SIG_DFL);
    if (fd)
    close(fd);
    return err;
    }
// Keep the CALL first so the function address is exactly the probed CALL.
    extern void uprobe_call_trigger(void);
    asm (".pushsection .text\n"
    ".global uprobe_call_target\n"
    ".type uprobe_call_target, @function\n"
    "uprobe_call_target:\n"
    "	ret\n"
    ".size uprobe_call_target, .-uprobe_call_target\n"
    ".global uprobe_call_trigger\n"
    ".type uprobe_call_trigger, @function\n"
    "uprobe_call_trigger:\n"
    "	call uprobe_call_target\n"
    "	ret\n"
    ".size uprobe_call_trigger, .-uprobe_call_trigger\n"
    ".popsection\n"
    );
// If CALL emulation misses the shadow stack update, this exits via SIGSEGV.
#[no_mangle]
unsafe extern "C" fn test_uprobe_call() -> c_int {
    static int test_uprobe_call(void)
    {
    let mut attr_sz: usize = sizeof(struct perf_event_attr);
    const char *file = "/proc/self/exe";
    let mut fd: c_int = -1, type, err = 1;
    struct perf_event_attr attr;
    let mut sa: sigaction = {};
    ssize_t offset;
    type = determine_uprobe_perf_type();
    if (type < 0) {
    if (type == -ENOENT)
    printf("[SKIP]\tUprobe on CALL test, uprobes are not available\n");
    return 0;
    }
    offset = get_uprobe_offset(uprobe_call_trigger);
    if (offset < 0)
    return 1;
    sa.sa_sigaction = segv_gp_handler;
    sa.sa_flags = SA_SIGINFO;
    if (sigaction(SIGSEGV, &sa, core::ptr::null_mut()))
    return 1;
// Setup entry uprobe through perf event interface.
    memset(&attr, 0, attr_sz);
    attr.size = attr_sz;
    attr.type = type;
    attr.config = 0;
    attr.config1 = (__u64)(unsigned long)file;
    attr.config2 = offset;
    fd = syscall(__NR_perf_event_open, &attr, 0 /* pid */, -1 /* cpu */,
    -1 /* group_fd */, PERF_FLAG_FD_CLOEXEC);
    if (fd < 0)
    goto out;
    if (sigsetjmp(jmp_buffer, 1))
    goto out;
    if (ARCH_PRCTL(ARCH_SHSTK_ENABLE, ARCH_SHSTK_SHSTK))
    goto out;
//
// This either segfaults and goes through sigsetjmp above
// or succeeds and we're good.
//
    uprobe_call_trigger();
    printf("[OK]\tUprobe on CALL test\n");
    err = 0;
    out:
    ARCH_PRCTL(ARCH_SHSTK_DISABLE, ARCH_SHSTK_SHSTK);
    signal(SIGSEGV, SIG_DFL);
    if (fd >= 0)
    close(fd);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn segv_handler_ptrace(signum: c_int, si: *mut siginfo_t, uc: *mut c_void) {
    void segv_handler_ptrace(int signum, siginfo_t *si, void *uc)
    {
// The SSP adjustment caused a segfault.
    exit(0);
    }
#[no_mangle]
pub unsafe extern "C" fn test_ptrace() -> c_int {
    int test_ptrace(void)
    {
    unsigned long saved_ssp, ssp = 0;
    let mut sa: sigaction = {};
    struct iovec iov;
    int status;
    int pid;
    iov.iov_base = &ssp;
    iov.iov_len = sizeof(ssp);
    pid = fork();
    if (!pid) {
    ssp = get_ssp();
    sa.sa_sigaction = segv_handler_ptrace;
    sa.sa_flags = SA_SIGINFO;
    if (sigaction(SIGSEGV, &sa, core::ptr::null_mut()))
    return 1;
    ptrace(PTRACE_TRACEME, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
//
// The parent will tweak the SSP and return from this function
// will #CP.
//
    raise(SIGTRAP);
    exit(1);
    }
    while (waitpid(pid, &status, 0) != -1 && WSTOPSIG(status) != SIGTRAP);
    if (ptrace(PTRACE_GETREGSET, pid, NT_X86_SHSTK, &iov)) {
    printf("[INFO]\tFailed to PTRACE_GETREGS\n");
    goto out_kill;
    }
    if (!ssp) {
    printf("[INFO]\tPtrace child SSP was 0\n");
    goto out_kill;
    }
    saved_ssp = ssp;
    iov.iov_len = 0;
    if (!ptrace(PTRACE_SETREGSET, pid, NT_X86_SHSTK, &iov)) {
    printf("[INFO]\tToo small size accepted via PTRACE_SETREGS\n");
    goto out_kill;
    }
    iov.iov_len = sizeof(ssp) + 1;
    if (!ptrace(PTRACE_SETREGSET, pid, NT_X86_SHSTK, &iov)) {
    printf("[INFO]\tToo large size accepted via PTRACE_SETREGS\n");
    goto out_kill;
    }
    ssp += 1;
    if (!ptrace(PTRACE_SETREGSET, pid, NT_X86_SHSTK, &iov)) {
    printf("[INFO]\tUnaligned SSP written via PTRACE_SETREGS\n");
    goto out_kill;
    }
    ssp = 0xFFFFFFFFFFFF0000;
    if (!ptrace(PTRACE_SETREGSET, pid, NT_X86_SHSTK, &iov)) {
    printf("[INFO]\tKernel range SSP written via PTRACE_SETREGS\n");
    goto out_kill;
    }
//
// Tweak the SSP so the child with #CP when it resumes and returns
// from raise()
//
    ssp = saved_ssp + 8;
    iov.iov_len = sizeof(ssp);
    if (ptrace(PTRACE_SETREGSET, pid, NT_X86_SHSTK, &iov)) {
    printf("[INFO]\tFailed to PTRACE_SETREGS\n");
    goto out_kill;
    }
    if (ptrace(PTRACE_DETACH, pid, core::ptr::null_mut(), core::ptr::null_mut())) {
    printf("[INFO]\tFailed to PTRACE_DETACH\n");
    goto out_kill;
    }
    waitpid(pid, &status, 0);
    if (WEXITSTATUS(status))
    return 1;
    printf("[OK]\tPtrace test\n");
    return 0;
    out_kill:
    kill(pid, SIGKILL);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut ret: c_int = 0;
    if (ARCH_PRCTL(ARCH_SHSTK_ENABLE, ARCH_SHSTK_SHSTK)) {
    printf("[SKIP]\tCould not enable Shadow stack\n");
    return 1;
    }
    if (ARCH_PRCTL(ARCH_SHSTK_DISABLE, ARCH_SHSTK_SHSTK)) {
    ret = 1;
    printf("[FAIL]\tDisabling shadow stack failed\n");
    }
    if (ARCH_PRCTL(ARCH_SHSTK_ENABLE, ARCH_SHSTK_SHSTK)) {
    printf("[SKIP]\tCould not re-enable Shadow stack\n");
    return 1;
    }
    if (ARCH_PRCTL(ARCH_SHSTK_ENABLE, ARCH_SHSTK_WRSS)) {
    printf("[SKIP]\tCould not enable WRSS\n");
    ret = 1;
    goto out;
    }
// Should have succeeded if here, but this is a test, so double check.
    if (!get_ssp()) {
    printf("[FAIL]\tShadow stack disabled\n");
    return 1;
    }
    if (test_shstk_pivot()) {
    ret = 1;
    printf("[FAIL]\tShadow stack pivot\n");
    goto out;
    }
    if (test_shstk_faults()) {
    ret = 1;
    printf("[FAIL]\tShadow stack fault test\n");
    goto out;
    }
    if (test_shstk_violation()) {
    ret = 1;
    printf("[FAIL]\tShadow stack violation test\n");
    goto out;
    }
    if (test_gup()) {
    ret = 1;
    printf("[FAIL]\tShadow shadow stack gup\n");
    goto out;
    }
    if (test_mprotect()) {
    ret = 1;
    printf("[FAIL]\tShadow shadow mprotect test\n");
    goto out;
    }
    if (test_userfaultfd()) {
    ret = 1;
    printf("[FAIL]\tUserfaultfd test\n");
    goto out;
    }
    if (test_guard_gap_other_gaps()) {
    ret = 1;
    printf("[FAIL]\tGuard gap test, other mappings' gaps\n");
    goto out;
    }
    if (test_guard_gap_new_mappings_gaps()) {
    ret = 1;
    printf("[FAIL]\tGuard gap test, placement mapping's gaps\n");
    goto out;
    }
    if (test_ptrace()) {
    ret = 1;
    printf("[FAIL]\tptrace test\n");
    }
    if (test_32bit()) {
    ret = 1;
    printf("[FAIL]\t32 bit test\n");
    goto out;
    }
    if (test_uretprobe()) {
    ret = 1;
    printf("[FAIL]\turetprobe test\n");
    goto out;
    }
    if (test_uprobe_call()) {
    ret = 1;
    printf("[FAIL]\tuprobe on CALL test\n");
    goto out;
    }
    return ret;
    out:
//
// Disable shadow stack before the function returns, or there will be a
// shadow stack violation.
//
    if (ARCH_PRCTL(ARCH_SHSTK_DISABLE, ARCH_SHSTK_SHSTK)) {
    ret = 1;
    printf("[FAIL]\tDisabling shadow stack failed\n");
    }
    return ret;
    }
