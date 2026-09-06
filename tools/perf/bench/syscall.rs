//! Automatically rewritten from C to Rust
//! Source: tools/perf/bench/syscall.c
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
// syscall.c
//
// syscall: Benchmark for system call performance
//

    static	int loops;
    static const struct option options[] = {
    OPT_INTEGER('l', "loop",	&loops,		"Specify number of loops"),
    OPT_END()
    };
    static const char * const bench_syscall_usage[] = {
    "perf bench syscall <options>",
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn test_fork() {
    static void test_fork(void)
    {
    let mut pid: pid_t = fork();
    if (pid < 0) {
    fprintf(stderr, "fork failed\n");
    exit(1);
    } else if (pid == 0) {
    exit(0);
    } else {
    if (waitpid(pid, core::ptr::null_mut(), 0) < 0) {
    fprintf(stderr, "waitpid failed\n");
    exit(1);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn test_execve() {
    static void test_execve(void)
    {
    const char *pathname = "/bin/true";
    char *const argv[] = { (char *)pathname, core::ptr::null_mut() };
    let mut pid: pid_t = fork();
    if (pid < 0) {
    fprintf(stderr, "fork failed\n");
    exit(1);
    } else if (pid == 0) {
    execve(pathname, argv, core::ptr::null_mut());
    fprintf(stderr, "execve /bin/true failed\n");
    exit(1);
    } else {
    if (waitpid(pid, core::ptr::null_mut(), 0) < 0) {
    fprintf(stderr, "waitpid failed\n");
    exit(1);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn bench_syscall_common(argc: c_int, argv: *const c_char, syscall: c_int) -> c_int {
    static int bench_syscall_common(int argc, const char **argv, int syscall)
    {
    struct timeval start, stop, diff;
    let mut result_usec: c_ulonglong = 0;
    const char *name = core::ptr::null_mut();
    int i;
    switch (syscall) {
    case __NR_fork:
    case __NR_execve:
// Limit default loop to 10000 times to save time
    loops = 10000;
    break;
    default:
    loops = 10000000;
    break;
    }
// Options -l and --loops override default above
    argc = parse_options(argc, argv, options, bench_syscall_usage, 0);
    gettimeofday(&start, core::ptr::null_mut());
    for (i = 0; i < loops; i++) {
    switch (syscall) {
    case __NR_getppid:
    getppid();
    break;
    case __NR_getpgid:
    getpgid(0);
    break;
    case __NR_fork:
    test_fork();
    break;
    case __NR_execve:
    test_execve();
    default:
    break;
    }
    }
    gettimeofday(&stop, core::ptr::null_mut());
    timersub(&stop, &start, &diff);
    switch (syscall) {
    case __NR_getppid:
    name = "getppid()";
    break;
    case __NR_getpgid:
    name = "getpgid()";
    break;
    case __NR_fork:
    name = "fork()";
    break;
    case __NR_execve:
    name = "execve()";
    break;
    default:
    break;
    }
    switch (bench_format) {
    case BENCH_FORMAT_DEFAULT:
    printf("# Executed %'d %s calls\n", loops, name);
    result_usec = diff.tv_sec * 1000000;
    result_usec += diff.tv_usec;
    printf(" %14s: %lu.%03lu [sec]\n\n", "Total time",
    (unsigned long) diff.tv_sec,
    (unsigned long) (diff.tv_usec/1000));
    printf(" %14lf usecs/op\n",
    (double)result_usec / (double)loops);
    printf(" %'14d ops/sec\n",
    (int)((double)loops /
    ((double)result_usec / (double)1000000)));
    break;
    case BENCH_FORMAT_SIMPLE:
    printf("%lu.%03lu\n",
    (unsigned long) diff.tv_sec,
    (unsigned long) (diff.tv_usec / 1000));
    break;
    default:
// reaching here is something disaster
    fprintf(stderr, "Unknown format:%d\n", bench_format);
    exit(1);
    break;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bench_syscall_basic(argc: c_int, argv: *const c_char) -> c_int {
    int bench_syscall_basic(int argc, const char **argv)
    {
    return bench_syscall_common(argc, argv, __NR_getppid);
    }
#[no_mangle]
pub unsafe extern "C" fn bench_syscall_getpgid(argc: c_int, argv: *const c_char) -> c_int {
    int bench_syscall_getpgid(int argc, const char **argv)
    {
    return bench_syscall_common(argc, argv, __NR_getpgid);
    }
#[no_mangle]
pub unsafe extern "C" fn bench_syscall_fork(argc: c_int, argv: *const c_char) -> c_int {
    int bench_syscall_fork(int argc, const char **argv)
    {
    return bench_syscall_common(argc, argv, __NR_fork);
    }
#[no_mangle]
pub unsafe extern "C" fn bench_syscall_execve(argc: c_int, argv: *const c_char) -> c_int {
    int bench_syscall_execve(int argc, const char **argv)
    {
    return bench_syscall_common(argc, argv, __NR_execve);
    }
