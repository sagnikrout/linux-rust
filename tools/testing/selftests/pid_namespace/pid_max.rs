//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/pid_namespace/pid_max.c
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
// The kernel computes the minimum allowed pid_max as:
// max(RESERVED_PIDS + 1, PIDS_PER_CPU_MIN * num_possible_cpus())
// Mirror that here so the test values are always valid.
//
// Note: glibc's get_nprocs_conf() returns the number of *configured
// (present) CPUs, not *possible* CPUs.  The kernel uses
// num_possible_cpus() which corresponds to /sys/devices/system/cpu/possible.
// These can differ significantly (e.g. 16 configured vs 128 possible).
//
pub const RESERVED_PIDS: c_int = 300;
pub const PIDS_PER_CPU_MIN: c_int = 8;
// Count CPUs from a range list like "0-31" or "0-15,32-47".
#[no_mangle]
unsafe extern "C" fn num_possible_cpus() -> c_int {
    static int num_possible_cpus(void)
    {
    FILE *f;
    let mut count: c_int = 0;
    int lo, hi;
    f = fopen("/sys/devices/system/cpu/possible", "r");
    if (!f)
    return 0;
    while (fscanf(f, "%d", &lo) == 1) {
    if (fscanf(f, "-%d", &hi) == 1)
    count += hi - lo + 1;
    else
    count++;
// skip comma separator
    fscanf(f, ",");
    }
    fclose(f);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn pid_min() -> c_int {
    static int pid_min(void)
    {
    let mut cpu_min: c_int = PIDS_PER_CPU_MIN * num_possible_cpus();
    return cpu_min > (RESERVED_PIDS + 1) ? cpu_min : (RESERVED_PIDS + 1);
    }
//
// Outer and inner pid_max limits used by the tests.  The outer limit is
// the more restrictive ancestor; the inner limit is set higher in a
// nested namespace but must still be capped by the outer limit.
// Both are derived from the kernel's minimum so they are always writable.
//
// Global so that clone callbacks can access them without parameter plumbing.
//
    static int outer_limit;
    static int inner_limit;
#[no_mangle]
unsafe extern "C" fn write_int_to_fd(fd: c_int, val: c_int) -> c_int {
    static int write_int_to_fd(int fd, int val)
    {
    char buf[12];
    let mut len: c_int = snprintf(buf, sizeof(buf), "%d", val);
    return write(fd, buf, len);
    }

#[no_mangle]
unsafe extern "C" fn do_clone(): *mut *mut int (fn)(void, arg: *mut c_void, flags: c_int) -> pid_t {
    static pid_t do_clone(int (*fn)(void *), void *arg, int flags)
    {
    char *stack;
    pid_t ret;
    stack = malloc(__STACK_SIZE);
    if (!stack)
    return -ENOMEM;

    ret = __clone2(fn, stack, __STACK_SIZE, flags | SIGCHLD, arg);

    ret = clone(fn, stack + __STACK_SIZE, flags | SIGCHLD, arg);

    free(stack);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pid_max_cb(data: *mut c_void) -> c_int {
    static int pid_max_cb(void *data)
    {
    int fd, ret;
    pid_t pid;
    ret = mount("", "/", core::ptr::null_mut(), MS_PRIVATE | MS_REC, 0);
    if (ret) {
    fprintf(stderr, "%m - Failed to make rootfs private mount\n");
    return -1;
    }
    umount2("/proc", MNT_DETACH);
    ret = mount("proc", "/proc", "proc", 0, core::ptr::null_mut());
    if (ret) {
    fprintf(stderr, "%m - Failed to mount proc\n");
    return -1;
    }
    fd = open("/proc/sys/kernel/pid_max", O_RDWR | O_CLOEXEC | O_NOCTTY);
    if (fd < 0) {
    fprintf(stderr, "%m - Failed to open pid_max\n");
    return -1;
    }
    ret = write_int_to_fd(fd, inner_limit);
    if (ret < 0) {
    fprintf(stderr, "%m - Failed to write pid_max\n");
    return -1;
    }
    for (int i = 0; i < inner_limit + 1; i++) {
    pid = fork();
    if (pid == 0)
    exit(EXIT_SUCCESS);
    wait_for_pid(pid);
    if (pid > inner_limit) {
    fprintf(stderr, "Managed to create pid number beyond limit\n");
    return -1;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pid_max_nested_inner(data: *mut c_void) -> c_int {
    static int pid_max_nested_inner(void *data)
    {
    let mut fret: c_int = -1;
    pid_t pids[2];
    int fd, i, ret;
    ret = mount("", "/", core::ptr::null_mut(), MS_PRIVATE | MS_REC, 0);
    if (ret) {
    fprintf(stderr, "%m - Failed to make rootfs private mount\n");
    return fret;
    }
    umount2("/proc", MNT_DETACH);
    ret = mount("proc", "/proc", "proc", 0, core::ptr::null_mut());
    if (ret) {
    fprintf(stderr, "%m - Failed to mount proc\n");
    return fret;
    }
    fd = open("/proc/sys/kernel/pid_max", O_RDWR | O_CLOEXEC | O_NOCTTY);
    if (fd < 0) {
    fprintf(stderr, "%m - Failed to open pid_max\n");
    return fret;
    }
    ret = write_int_to_fd(fd, inner_limit);
    close(fd);
    if (ret < 0) {
    fprintf(stderr, "%m - Failed to write pid_max\n");
    return fret;
    }
    pids[0] = fork();
    if (pids[0] < 0) {
    fprintf(stderr, "Failed to create first new process\n");
    return fret;
    }
    if (pids[0] == 0)
    exit(EXIT_SUCCESS);
    pids[1] = fork();
    wait_for_pid(pids[0]);
    if (pids[1] >= 0) {
    if (pids[1] == 0)
    exit(EXIT_SUCCESS);
    wait_for_pid(pids[1]);
    fprintf(stderr, "Managed to create process even though ancestor pid namespace had a limit\n");
    return fret;
    }
// Now make sure that we wrap pids at outer_limit.
    for (i = 0; i < inner_limit + 10; i++) {
    pid_t pid;
    pid = fork();
    if (pid < 0)
    return fret;
    if (pid == 0)
    exit(EXIT_SUCCESS);
    wait_for_pid(pid);
    if (pid >= inner_limit) {
    fprintf(stderr, "Managed to create process with pid %d beyond configured limit\n", pid);
    return fret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pid_max_nested_outer(data: *mut c_void) -> c_int {
    static int pid_max_nested_outer(void *data)
    {
    let mut fret: c_int = -1, nr_procs = 0;
    pid_t *pids;
    int fd, ret;
    pid_t pid;
    pids = malloc(outer_limit * sizeof(pid_t));
    if (!pids)
    return -1;
    ret = mount("", "/", core::ptr::null_mut(), MS_PRIVATE | MS_REC, 0);
    if (ret) {
    fprintf(stderr, "%m - Failed to make rootfs private mount\n");
    goto out;
    }
    umount2("/proc", MNT_DETACH);
    ret = mount("proc", "/proc", "proc", 0, core::ptr::null_mut());
    if (ret) {
    fprintf(stderr, "%m - Failed to mount proc\n");
    goto out;
    }
    fd = open("/proc/sys/kernel/pid_max", O_RDWR | O_CLOEXEC | O_NOCTTY);
    if (fd < 0) {
    fprintf(stderr, "%m - Failed to open pid_max\n");
    goto out;
    }
    ret = write_int_to_fd(fd, outer_limit);
    close(fd);
    if (ret < 0) {
    fprintf(stderr, "%m - Failed to write pid_max\n");
    goto out;
    }
//
// Create (outer_limit - 4) processes. This leaves room for
// do_clone() and one more. So creating another process needs
// to fail.
//
    for (nr_procs = 0; nr_procs < outer_limit - 4; nr_procs++) {
    pid = fork();
    if (pid < 0)
    goto reap;
    if (pid == 0)
    exit(EXIT_SUCCESS);
    pids[nr_procs] = pid;
    }
    pid = do_clone(pid_max_nested_inner, core::ptr::null_mut(), CLONE_NEWPID | CLONE_NEWNS);
    if (pid < 0) {
    fprintf(stderr, "%m - Failed to clone nested pidns\n");
    goto reap;
    }
    if (wait_for_pid(pid)) {
    fprintf(stderr, "%m - Nested pid_max failed\n");
    goto reap;
    }
    fret = 0;
    reap:
    for (int i = 0; i < nr_procs; i++)
    wait_for_pid(pids[i]);
    out:
    free(pids);
    return fret;
    }
#[no_mangle]
unsafe extern "C" fn pid_max_nested_limit_inner(data: *mut c_void) -> c_int {
    static int pid_max_nested_limit_inner(void *data)
    {
    let mut fret: c_int = -1, nr_procs = 0;
    int fd, ret;
    pid_t pid;
    pid_t *pids;
    pids = malloc(inner_limit * sizeof(pid_t));
    if (!pids)
    return -1;
    ret = mount("", "/", core::ptr::null_mut(), MS_PRIVATE | MS_REC, 0);
    if (ret) {
    fprintf(stderr, "%m - Failed to make rootfs private mount\n");
    goto out;
    }
    umount2("/proc", MNT_DETACH);
    ret = mount("proc", "/proc", "proc", 0, core::ptr::null_mut());
    if (ret) {
    fprintf(stderr, "%m - Failed to mount proc\n");
    goto out;
    }
    fd = open("/proc/sys/kernel/pid_max", O_RDWR | O_CLOEXEC | O_NOCTTY);
    if (fd < 0) {
    fprintf(stderr, "%m - Failed to open pid_max\n");
    goto out;
    }
    ret = write_int_to_fd(fd, inner_limit);
    close(fd);
    if (ret < 0) {
    fprintf(stderr, "%m - Failed to write pid_max\n");
    goto out;
    }
    for (nr_procs = 0; nr_procs < inner_limit; nr_procs++) {
    pid = fork();
    if (pid < 0)
    break;
    if (pid == 0)
    exit(EXIT_SUCCESS);
    pids[nr_procs] = pid;
    }
    if (nr_procs >= outer_limit) {
    fprintf(stderr, "Managed to create processes beyond the configured outer limit\n");
    goto reap;
    }
    fret = 0;
    reap:
    for (int i = 0; i < nr_procs; i++)
    wait_for_pid(pids[i]);
    out:
    free(pids);
    return fret;
    }
#[no_mangle]
unsafe extern "C" fn pid_max_nested_limit_outer(data: *mut c_void) -> c_int {
    static int pid_max_nested_limit_outer(void *data)
    {
    int fd, ret;
    pid_t pid;
    ret = mount("", "/", core::ptr::null_mut(), MS_PRIVATE | MS_REC, 0);
    if (ret) {
    fprintf(stderr, "%m - Failed to make rootfs private mount\n");
    return -1;
    }
    umount2("/proc", MNT_DETACH);
    ret = mount("proc", "/proc", "proc", 0, core::ptr::null_mut());
    if (ret) {
    fprintf(stderr, "%m - Failed to mount proc\n");
    return -1;
    }
    fd = open("/proc/sys/kernel/pid_max", O_RDWR | O_CLOEXEC | O_NOCTTY);
    if (fd < 0) {
    fprintf(stderr, "%m - Failed to open pid_max\n");
    return -1;
    }
    ret = write_int_to_fd(fd, outer_limit);
    close(fd);
    if (ret < 0) {
    fprintf(stderr, "%m - Failed to write pid_max\n");
    return -1;
    }
    pid = do_clone(pid_max_nested_limit_inner, core::ptr::null_mut(), CLONE_NEWPID | CLONE_NEWNS);
    if (pid < 0) {
    fprintf(stderr, "%m - Failed to clone nested pidns\n");
    return -1;
    }
    if (wait_for_pid(pid)) {
    fprintf(stderr, "%m - Nested pid_max failed\n");
    return -1;
    }
    return 0;
    }
    FIXTURE(pid_max) {
    int dummy;
    };
    FIXTURE_SETUP(pid_max)
    {
    let mut min: c_int = pid_min();
    outer_limit = min + 100;
    inner_limit = min + 200;
    }
    FIXTURE_TEARDOWN(pid_max)
    {
    }
    TEST_F(pid_max, simple)
    {
    pid_t pid;
    pid = do_clone(pid_max_cb, core::ptr::null_mut(), CLONE_NEWPID | CLONE_NEWNS);
    ASSERT_GT(pid, 0);
    ASSERT_EQ(0, wait_for_pid(pid));
    }
    TEST_F(pid_max, nested_limit)
    {
    pid_t pid;
    pid = do_clone(pid_max_nested_limit_outer, core::ptr::null_mut(), CLONE_NEWPID | CLONE_NEWNS);
    ASSERT_GT(pid, 0);
    ASSERT_EQ(0, wait_for_pid(pid));
    }
    TEST_F(pid_max, nested)
    {
    pid_t pid;
    pid = do_clone(pid_max_nested_outer, core::ptr::null_mut(), CLONE_NEWPID | CLONE_NEWNS);
    ASSERT_GT(pid, 0);
    ASSERT_EQ(0, wait_for_pid(pid));
    }
    TEST_HARNESS_MAIN
