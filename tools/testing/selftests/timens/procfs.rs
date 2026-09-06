//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timens/procfs.c
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
// Macro flag: #define _GNU_SOURCE

//
// Test shouldn't be run for a day, so add 10 days to child
// time and check parent's time to be in the same day.
//

    static int child_ns, parent_ns;
#[no_mangle]
unsafe extern "C" fn switch_ns(fd: c_int) -> c_int {
    static int switch_ns(int fd)
    {
    if (setns(fd, CLONE_NEWTIME))
    return pr_perror("setns()");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_namespaces() -> c_int {
    static int init_namespaces(void)
    {
    char path[] = "/proc/self/ns/time_for_children";
    struct stat st1, st2;
    parent_ns = open(path, O_RDONLY);
    if (parent_ns <= 0)
    return pr_perror("Unable to open %s", path);
    if (fstat(parent_ns, &st1))
    return pr_perror("Unable to stat the parent timens");
    if (unshare_timens())
    return -1;
    child_ns = open(path, O_RDONLY);
    if (child_ns <= 0)
    return pr_perror("Unable to open %s", path);
    if (fstat(child_ns, &st2))
    return pr_perror("Unable to stat the timens");
    if (st1.st_ino == st2.st_ino)
    return pr_err("The same child_ns after CLONE_NEWTIME");
    if (_settime(CLOCK_BOOTTIME, TEN_DAYS_IN_SEC))
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn read_proc_uptime(uptime: *mut timespec) -> c_int {
    static int read_proc_uptime(struct timespec *uptime)
    {
    unsigned long up_sec, up_nsec;
    FILE *proc;
    proc = fopen("/proc/uptime", "r");
    if (proc == core::ptr::null_mut()) {
    pr_perror("Unable to open /proc/uptime");
    return -1;
    }
    if (fscanf(proc, "%lu.%02lu", &up_sec, &up_nsec) != 2) {
    if (errno) {
    pr_perror("fscanf");
    return -errno;
    }
    pr_err("failed to parse /proc/uptime");
    return -1;
    }
    fclose(proc);
    uptime.tv_sec = up_sec;
    uptime.tv_nsec = up_nsec;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn read_proc_stat_btime(boottime_sec: *mut c_ulonglong) -> c_int {
    static int read_proc_stat_btime(unsigned long long *boottime_sec)
    {
    FILE *proc;
    char line_buf[2048];
    proc = fopen("/proc/stat", "r");
    if (proc == core::ptr::null_mut()) {
    pr_perror("Unable to open /proc/stat");
    return -1;
    }
    while (fgets(line_buf, 2048, proc)) {
    if (sscanf(line_buf, "btime %llu", boottime_sec) != 1)
    continue;
    fclose(proc);
    return 0;
    }
    if (errno) {
    pr_perror("fscanf");
    fclose(proc);
    return -errno;
    }
    pr_err("failed to parse /proc/stat");
    fclose(proc);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn check_uptime() -> c_int {
    static int check_uptime(void)
    {
    struct timespec uptime_new, uptime_old;
    time_t uptime_expected;
    let mut prec: double = MAX_TEST_TIME_SEC;
    if (switch_ns(parent_ns))
    return pr_err("switch_ns(%d)", parent_ns);
    if (read_proc_uptime(&uptime_old))
    return 1;
    if (switch_ns(child_ns))
    return pr_err("switch_ns(%d)", child_ns);
    if (read_proc_uptime(&uptime_new))
    return 1;
    uptime_expected = uptime_old.tv_sec + TEN_DAYS_IN_SEC;
    if (fabs(difftime(uptime_new.tv_sec, uptime_expected)) > prec) {
    pr_fail("uptime in /proc/uptime: old %ld, new %ld [%ld]",
    uptime_old.tv_sec, uptime_new.tv_sec,
    uptime_old.tv_sec + TEN_DAYS_IN_SEC);
    return 1;
    }
    ksft_test_result_pass("Passed for /proc/uptime\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_stat_btime() -> c_int {
    static int check_stat_btime(void)
    {
    unsigned long long btime_new, btime_old;
    unsigned long long btime_expected;
    if (switch_ns(parent_ns))
    return pr_err("switch_ns(%d)", parent_ns);
    if (read_proc_stat_btime(&btime_old))
    return 1;
    if (switch_ns(child_ns))
    return pr_err("switch_ns(%d)", child_ns);
    if (read_proc_stat_btime(&btime_new))
    return 1;
    btime_expected = btime_old - TEN_DAYS_IN_SEC;
    if (btime_new != btime_expected) {
    pr_fail("btime in /proc/stat: old %llu, new %llu [%llu]",
    btime_old, btime_new, btime_expected);
    return 1;
    }
    ksft_test_result_pass("Passed for /proc/stat btime\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut ret: c_int = 0;
    ksft_print_header();
    nscheck();
    ksft_set_plan(2);
    if (init_namespaces())
    return 1;
    ret |= check_uptime();
    ret |= check_stat_btime();
    if (ret)
    ksft_exit_fail();
    ksft_exit_pass();
    return ret;
    }
