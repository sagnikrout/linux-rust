//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/riscv/abi/pointer_masking.c
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


// SPDX-License-Identifier: GPL-2.0-only

pub const PR_PMLEN_SHIFT: c_int = 24;

    static int dev_zero;
    static int pipefd[2];
    static sigjmp_buf jmpbuf;
#[no_mangle]
unsafe extern "C" fn sigsegv_handler(sig: c_int) {
    static void sigsegv_handler(int sig)
    {
    siglongjmp(jmpbuf, 1);
    }
    static int min_pmlen;
    static int max_pmlen;
#[no_mangle]
pub unsafe extern "C" fn valid_pmlen(pmlen: c_int) -> bool {
    static inline bool valid_pmlen(int pmlen)
    {
    let mut pmlen: return = = 0 || pmlen == 7 || pmlen == 16;
    }
#[no_mangle]
unsafe extern "C" fn test_pmlen() {
    static void test_pmlen(void)
    {
    ksft_print_msg("Testing available PMLEN values\n");
    for (int request = 0; request <= 16; request++) {
    int pmlen, ret;
    ret = prctl(PR_SET_TAGGED_ADDR_CTRL, request << PR_PMLEN_SHIFT, 0, 0, 0);
    if (ret)
    goto pr_set_error;
    ret = prctl(PR_GET_TAGGED_ADDR_CTRL, 0, 0, 0, 0);
    ksft_test_result(ret >= 0, "PMLEN=%d PR_GET_TAGGED_ADDR_CTRL\n", request);
    if (ret < 0)
    goto pr_get_error;
    pmlen = (ret & PR_PMLEN_MASK) >> PR_PMLEN_SHIFT;
    ksft_test_result(pmlen >= request, "PMLEN=%d constraint\n", request);
    ksft_test_result(valid_pmlen(pmlen), "PMLEN=%d validity\n", request);
    if (min_pmlen == 0)
    min_pmlen = pmlen;
    if (max_pmlen < pmlen)
    max_pmlen = pmlen;
    continue;
    pr_set_error:
    ksft_test_result_skip("PMLEN=%d PR_GET_TAGGED_ADDR_CTRL\n", request);
    pr_get_error:
    ksft_test_result_skip("PMLEN=%d constraint\n", request);
    ksft_test_result_skip("PMLEN=%d validity\n", request);
    }
    if (max_pmlen == 0)
    ksft_exit_fail_msg("Failed to enable pointer masking\n");
    }
#[no_mangle]
unsafe extern "C" fn set_tagged_addr_ctrl(pmlen: c_int, tagged_addr_abi: bool) -> c_int {
    static int set_tagged_addr_ctrl(int pmlen, bool tagged_addr_abi)
    {
    int arg, ret;
    arg = pmlen << PR_PMLEN_SHIFT | tagged_addr_abi;
    ret = prctl(PR_SET_TAGGED_ADDR_CTRL, arg, 0, 0, 0);
    if (!ret) {
    ret = prctl(PR_GET_TAGGED_ADDR_CTRL, 0, 0, 0, 0);
    if (ret == arg)
    return 0;
    }
    return ret < 0 ? -errno : -ENODATA;
    }
#[no_mangle]
unsafe extern "C" fn test_dereference_pmlen(pmlen: c_int) {
    static void test_dereference_pmlen(int pmlen)
    {
    static volatile int i;
    volatile int *p;
    int ret;
    ret = set_tagged_addr_ctrl(pmlen, false);
    if (ret)
    return ksft_test_result_error("PMLEN=%d setup (%d)\n", pmlen, ret);
    i = pmlen;
    if (pmlen) {
    p = (volatile int *)((uintptr_t)&i | 1UL << (__riscv_xlen - pmlen));
// These dereferences should succeed.
    if (sigsetjmp(jmpbuf, 1))
    return ksft_test_result_fail("PMLEN=%d valid tag\n", pmlen);
    if (*p != pmlen)
    return ksft_test_result_fail("PMLEN=%d bad value\n", pmlen);
    ++*p;
    }
    p = (volatile int *)((uintptr_t)&i | 1UL << (__riscv_xlen - pmlen - 1));
// These dereferences should raise SIGSEGV.
    if (sigsetjmp(jmpbuf, 1))
    return ksft_test_result_pass("PMLEN=%d dereference\n", pmlen);
    ++*p;
    ksft_test_result_fail("PMLEN=%d invalid tag\n", pmlen);
    }
#[no_mangle]
unsafe extern "C" fn test_dereference() {
    static void test_dereference(void)
    {
    ksft_print_msg("Testing userspace pointer dereference\n");
    signal(SIGSEGV, sigsegv_handler);
    test_dereference_pmlen(0);
    test_dereference_pmlen(min_pmlen);
    test_dereference_pmlen(max_pmlen);
    signal(SIGSEGV, SIG_DFL);
    }
#[no_mangle]
unsafe extern "C" fn execve_child_sigsegv_handler(sig: c_int) {
    static void execve_child_sigsegv_handler(int sig)
    {
    exit(42);
    }
#[no_mangle]
unsafe extern "C" fn execve_child() -> c_int {
    static int execve_child(void)
    {
    static volatile int i;
    volatile int *p = (volatile int *)((uintptr_t)&i | 1UL << (__riscv_xlen - 7));
    signal(SIGSEGV, execve_child_sigsegv_handler);
// This dereference should raise SIGSEGV.
    return *p;
    }
#[no_mangle]
unsafe extern "C" fn test_fork_exec() {
    static void test_fork_exec(void)
    {
    int ret, status;
    ksft_print_msg("Testing fork/exec behavior\n");
    ret = set_tagged_addr_ctrl(min_pmlen, false);
    if (ret)
    return ksft_test_result_error("setup (%d)\n", ret);
    if (fork()) {
    wait(&status);
    ksft_test_result(WIFEXITED(status) && WEXITSTATUS(status) == 42,
    "dereference after fork\n");
    } else {
    let mut i: static volatile int = 42;
    volatile int *p;
    p = (volatile int *)((uintptr_t)&i | 1UL << (__riscv_xlen - min_pmlen));
// This dereference should succeed.
    exit(*p);
    }
    if (fork()) {
    wait(&status);
    ksft_test_result(WIFEXITED(status) && WEXITSTATUS(status) == 42,
    "dereference after fork+exec\n");
    } else {
// Will call execve_child().
    execve("/proc/self/exe", (char *const []) { "", core::ptr::null_mut() }, core::ptr::null_mut());
    }
    }
#[no_mangle]
unsafe extern "C" fn pwrite_wrapper(fd: c_int, buf: *mut c_void, count: usize, msg: *const c_char) -> bool {
    static bool pwrite_wrapper(int fd, void *buf, size_t count, const char *msg)
    {
    let mut ret: c_int = pwrite(fd, buf, count, 0);
    if (ret != count) {
    ksft_perror(msg);
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn test_tagged_addr_abi_sysctl() {
    static void test_tagged_addr_abi_sysctl(void)
    {
    char *err_pwrite_msg = "failed to write to /proc/sys/abi/tagged_addr_disabled\n";
    char value;
    int fd;
    ksft_print_msg("Testing tagged address ABI sysctl\n");
    fd = open("/proc/sys/abi/tagged_addr_disabled", O_WRONLY);
    if (fd < 0) {
    ksft_test_result_skip("failed to open sysctl file\n");
    ksft_test_result_skip("failed to open sysctl file\n");
    return;
    }
    value = '1';
    if (!pwrite_wrapper(fd, &value, 1, "write '1'"))
    ksft_test_result_fail(err_pwrite_msg);
    else
    ksft_test_result(set_tagged_addr_ctrl(min_pmlen, true) == -EINVAL,
    "sysctl disabled\n");
    value = '0';
    if (!pwrite_wrapper(fd, &value, 1, "write '0'"))
    ksft_test_result_fail(err_pwrite_msg);
    else
    ksft_test_result(set_tagged_addr_ctrl(min_pmlen, true) == 0,
    "sysctl enabled\n");
    set_tagged_addr_ctrl(0, false);
    close(fd);
    }
#[no_mangle]
unsafe extern "C" fn test_tagged_addr_abi_pmlen(pmlen: c_int) {
    static void test_tagged_addr_abi_pmlen(int pmlen)
    {
    int i, *p, ret;
    i = ~pmlen;
    if (pmlen) {
    p = (int *)((uintptr_t)&i | 1UL << (__riscv_xlen - pmlen));
    ret = set_tagged_addr_ctrl(pmlen, false);
    if (ret)
    return ksft_test_result_error("PMLEN=%d ABI disabled setup (%d)\n",
    pmlen, ret);
    ret = write(pipefd[1], p, sizeof(*p));
    if (ret >= 0 || errno != EFAULT)
    return ksft_test_result_fail("PMLEN=%d ABI disabled write\n", pmlen);
    ret = read(dev_zero, p, sizeof(*p));
    if (ret >= 0 || errno != EFAULT)
    return ksft_test_result_fail("PMLEN=%d ABI disabled read\n", pmlen);
    if (i != ~pmlen)
    return ksft_test_result_fail("PMLEN=%d ABI disabled value\n", pmlen);
    ret = set_tagged_addr_ctrl(pmlen, true);
    if (ret)
    return ksft_test_result_error("PMLEN=%d ABI enabled setup (%d)\n",
    pmlen, ret);
    ret = write(pipefd[1], p, sizeof(*p));
    if (ret != sizeof(*p))
    return ksft_test_result_fail("PMLEN=%d ABI enabled write\n", pmlen);
    ret = read(dev_zero, p, sizeof(*p));
    if (ret != sizeof(*p))
    return ksft_test_result_fail("PMLEN=%d ABI enabled read\n", pmlen);
    if (i)
    return ksft_test_result_fail("PMLEN=%d ABI enabled value\n", pmlen);
    i = ~pmlen;
    } else {
// The tagged address ABI cannot be enabled when PMLEN == 0.
    ret = set_tagged_addr_ctrl(pmlen, true);
    if (ret != -EINVAL)
    return ksft_test_result_error("PMLEN=%d ABI setup (%d)\n",
    pmlen, ret);
    }
    p = (int *)((uintptr_t)&i | 1UL << (__riscv_xlen - pmlen - 1));
    ret = write(pipefd[1], p, sizeof(*p));
    if (ret >= 0 || errno != EFAULT)
    return ksft_test_result_fail("PMLEN=%d invalid tag write (%d)\n", pmlen, errno);
    ret = read(dev_zero, p, sizeof(*p));
    if (ret >= 0 || errno != EFAULT)
    return ksft_test_result_fail("PMLEN=%d invalid tag read\n", pmlen);
    if (i != ~pmlen)
    return ksft_test_result_fail("PMLEN=%d invalid tag value\n", pmlen);
    ksft_test_result_pass("PMLEN=%d tagged address ABI\n", pmlen);
    }
#[no_mangle]
unsafe extern "C" fn test_tagged_addr_abi() {
    static void test_tagged_addr_abi(void)
    {
    ksft_print_msg("Testing tagged address ABI\n");
    test_tagged_addr_abi_pmlen(0);
    test_tagged_addr_abi_pmlen(min_pmlen);
    test_tagged_addr_abi_pmlen(max_pmlen);
    }
    static struct test_info {
    unsigned int nr_tests;
    void (*test_fn)(void);
    } tests[] = {
    { .nr_tests = 17 * 3, test_pmlen },
    { .nr_tests = 3, test_dereference },
    { .nr_tests = 2, test_fork_exec },
    { .nr_tests = 2, test_tagged_addr_abi_sysctl },
    { .nr_tests = 3, test_tagged_addr_abi },
    };
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut plan: c_uint = 0;
    int ret;
// Check if this is the child process after execve().
    if (!argv[0][0])
    return execve_child();
    dev_zero = open("/dev/zero", O_RDWR);
    if (dev_zero < 0)
    return 1;
// Write to a pipe so the kernel must dereference the buffer pointer.
    ret = pipe(pipefd);
    if (ret)
    return 1;
    ksft_print_header();
    for (int i = 0; i < ARRAY_SIZE(tests); i++)
    plan += tests[i].nr_tests;
    ksft_set_plan(plan);
    for (int i = 0; i < ARRAY_SIZE(tests); i++)
    tests[i].test_fn();
    ksft_finished();
    }
