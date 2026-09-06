//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/bp_account.c
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
// Powerpc needs __SANE_USERSPACE_TYPES__ before <linux/types.h> to select
// 'int-ll64.h' and avoid compile warnings when printing __u64 with %llu.
//
// Macro flag: #define __SANE_USERSPACE_TYPES__

//
// PowerPC and S390 do not support creation of instruction breakpoints using the
// perf_event interface.
//
// Just disable the test for these architectures until these issues are
// resolved.
//

pub const BP_ACCOUNT_IS_SUPPORTED: c_int = 0;

pub const BP_ACCOUNT_IS_SUPPORTED: c_int = 1;

    static volatile long the_var;
#[no_mangle]
unsafe extern "C" fn test_function() -> noinline int {
    static noinline int test_function(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __event(is_x: bool, addr: *mut c_void, attr: *mut perf_event_attr) -> c_int {
    static int __event(bool is_x, void *addr, struct perf_event_attr *attr)
    {
    int fd;
    memset(attr, 0, sizeof(struct perf_event_attr));
    attr.type = PERF_TYPE_BREAKPOINT;
    attr.size = sizeof(struct perf_event_attr);
    attr.config = 0;
    attr.bp_type = is_x ? HW_BREAKPOINT_X : HW_BREAKPOINT_W;
    attr.bp_addr = (unsigned long) addr;
    attr.bp_len = is_x ? default_breakpoint_len() : sizeof(long);
    attr.sample_period = 1;
    attr.sample_type = PERF_SAMPLE_IP;
    attr.exclude_kernel = 1;
    attr.exclude_hv = 1;
    fd = sys_perf_event_open(attr, -1, 0, -1,
    perf_event_open_cloexec_flag());
    if (fd < 0) {
    pr_debug("failed opening event %llx\n", attr.config);
    return TEST_FAIL;
    }
    return fd;
    }
#[no_mangle]
unsafe extern "C" fn wp_event(addr: *mut c_void, attr: *mut perf_event_attr) -> c_int {
    static int wp_event(void *addr, struct perf_event_attr *attr)
    {
    return __event(false, addr, attr);
    }
#[no_mangle]
unsafe extern "C" fn bp_event(addr: *mut c_void, attr: *mut perf_event_attr) -> c_int {
    static int bp_event(void *addr, struct perf_event_attr *attr)
    {
    return __event(true, addr, attr);
    }
#[no_mangle]
unsafe extern "C" fn bp_accounting(wp_cnt: c_int, share: c_int) -> c_int {
    static int bp_accounting(int wp_cnt, int share)
    {
    struct perf_event_attr attr, attr_mod, attr_new;
    int i, fd[wp_cnt], fd_wp, ret;
    for (i = 0; i < wp_cnt; i++) {
    fd[i] = wp_event((void *)&the_var, &attr);
    TEST_ASSERT_VAL("failed to create wp\n", fd[i] != -1);
    pr_debug("wp %d created\n", i);
    }
    attr_mod = attr;
    attr_mod.bp_type = HW_BREAKPOINT_X;
    attr_mod.bp_addr = (unsigned long) test_function;
    attr_mod.bp_len = default_breakpoint_len();
    ret = ioctl(fd[0], PERF_EVENT_IOC_MODIFY_ATTRIBUTES, &attr_mod);
    TEST_ASSERT_VAL("failed to modify wp\n", ret == 0);
    pr_debug("wp 0 modified to bp\n");
    if (!share) {
    fd_wp = wp_event((void *)&the_var, &attr_new);
    TEST_ASSERT_VAL("failed to create max wp\n", fd_wp != -1);
    pr_debug("wp max created\n");
    close(fd_wp);
    }
    for (i = 0; i < wp_cnt; i++)
    close(fd[i]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn detect_cnt(is_x: bool) -> c_int {
    static int detect_cnt(bool is_x)
    {
    struct perf_event_attr attr;
    void *addr = is_x ? (void *)test_function : (void *)&the_var;
    int fd[100], cnt = 0, i;
    while (1) {
    if (cnt == 100) {
    pr_debug("way too many debug registers, fix the test\n");
    return 0;
    }
    fd[cnt] = __event(is_x, addr, &attr);
    if (fd[cnt] < 0)
    break;
    cnt++;
    }
    for (i = 0; i < cnt; i++)
    close(fd[i]);
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn detect_ioctl() -> c_int {
    static int detect_ioctl(void)
    {
    struct perf_event_attr attr;
    int fd, ret = 1;
    fd = wp_event((void *) &the_var, &attr);
    if (fd > 0) {
    ret = ioctl(fd, PERF_EVENT_IOC_MODIFY_ATTRIBUTES, &attr);
    close(fd);
    }
    return ret ? 0 : 1;
    }
#[no_mangle]
unsafe extern "C" fn detect_share(wp_cnt: c_int, bp_cnt: c_int) -> c_int {
    static int detect_share(int wp_cnt, int bp_cnt)
    {
    struct perf_event_attr attr;
    int i, *fd = core::ptr::null_mut(), ret = -1;
    if (wp_cnt + bp_cnt == 0)
    return 0;
    fd = malloc(sizeof(int) * (wp_cnt + bp_cnt));
    if (!fd)
    return -1;
    for (i = 0; i < wp_cnt; i++) {
    fd[i] = wp_event((void *)&the_var, &attr);
    if (fd[i] == -1) {
    pr_err("failed to create wp\n");
    goto out;
    }
    }
    for (; i < (bp_cnt + wp_cnt); i++) {
    fd[i] = bp_event((void *)test_function, &attr);
    if (fd[i] == -1)
    break;
    }
    ret = i != (bp_cnt + wp_cnt);
    out:
    while (i--)
    close(fd[i]);
    free(fd);
    return ret;
    }
//
// This test does following:
// - detects the number of watch/break-points,
// skip test if any is missing
// - detects PERF_EVENT_IOC_MODIFY_ATTRIBUTES ioctl,
// skip test if it's missing
// - detects if watchpoints and breakpoints share
// same slots
// - create all possible watchpoints on cpu 0
// - change one of it to breakpoint
// - in case wp and bp do not share slots,
// we create another watchpoint to ensure
// the slot accounting is correct
//
#[no_mangle]
unsafe extern "C" fn test__bp_accounting(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__bp_accounting(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    let mut has_ioctl: c_int = detect_ioctl();
    let mut wp_cnt: c_int = detect_cnt(false);
    let mut bp_cnt: c_int = detect_cnt(true);
    let mut share: c_int = detect_share(wp_cnt, bp_cnt);
    if (!BP_ACCOUNT_IS_SUPPORTED) {
    pr_debug("Test not supported on this architecture");
    return TEST_SKIP;
    }
    pr_debug("watchpoints count %d, breakpoints count %d, has_ioctl %d, share %d\n",
    wp_cnt, bp_cnt, has_ioctl, share);
    if (!wp_cnt || !bp_cnt || !has_ioctl)
    return TEST_SKIP;
    return bp_accounting(wp_cnt, share);
    }
    DEFINE_SUITE("Breakpoint accounting", bp_accounting);
