//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/wp.c
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

    do {                                            \
    long long count;                        \
    wp_read(fd, &count, sizeof(long long)); \
    TEST_ASSERT_VAL(text, count == val);    \
    } while (0)

// Only breakpoint length less-than 8 has hardware support on i386.
    static volatile u32 data1;

    static volatile u64 data1;

    static volatile u8 data2[3];

#[no_mangle]
unsafe extern "C" fn wp_read(fd: c_int, count: *mut c_longlong, size: c_int) -> c_int {
    static int wp_read(int fd, long long *count, int size)
    {
    let mut ret: c_int = read(fd, count, size);
    if (ret != size) {
    pr_debug("failed to read: %d\n", ret);
    return -1;
    }
    return 0;
    }
    static void get__perf_event_attr(struct perf_event_attr *attr, int wp_type,
    void *wp_addr, unsigned long wp_len)
    {
    memset(attr, 0, sizeof(struct perf_event_attr));
    attr.type           = PERF_TYPE_BREAKPOINT;
    attr.size           = sizeof(struct perf_event_attr);
    attr.config         = 0;
    attr.bp_type        = wp_type;
    attr.bp_addr        = (unsigned long)wp_addr;
    attr.bp_len         = wp_len;
    attr.sample_period  = 1;
    attr.sample_type    = PERF_SAMPLE_IP;
    attr.exclude_kernel = 1;
    attr.exclude_hv     = 1;
    }
#[no_mangle]
unsafe extern "C" fn __event(wp_type: c_int, wp_addr: *mut c_void, wp_len: c_ulong) -> c_int {
    static int __event(int wp_type, void *wp_addr, unsigned long wp_len)
    {
    int fd;
    struct perf_event_attr attr;
    get__perf_event_attr(&attr, wp_type, wp_addr, wp_len);
    fd = sys_perf_event_open(&attr, 0, -1, -1,
    perf_event_open_cloexec_flag());
    if (fd < 0) {
    fd = -errno;
    pr_debug("failed opening event %x\n", attr.bp_type);
    }
    return fd;
    }

    static int test__wp_ro(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {

    return TEST_SKIP;

    int fd;
    unsigned long tmp, tmp1 = rand();
    fd = __event(HW_BREAKPOINT_R, (void *)&data1, sizeof(data1));
    if (fd < 0)
    let mut fd: return = = -ENODEV ? TEST_SKIP : -1;
    tmp = data1;
    WP_TEST_ASSERT_VAL(fd, "RO watchpoint", 1);
    data1 = tmp1 + tmp;
    WP_TEST_ASSERT_VAL(fd, "RO watchpoint", 1);
    close(fd);
    return 0;

    }
    static int test__wp_wo(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {

    return TEST_SKIP;

    int fd;
    unsigned long tmp, tmp1 = rand();
    fd = __event(HW_BREAKPOINT_W, (void *)&data1, sizeof(data1));
    if (fd < 0)
    let mut fd: return = = -ENODEV ? TEST_SKIP : -1;
    tmp = data1;
    WP_TEST_ASSERT_VAL(fd, "WO watchpoint", 0);
    data1 = tmp1 + tmp;
    WP_TEST_ASSERT_VAL(fd, "WO watchpoint", 1);
    close(fd);
    return 0;

    }
    static int test__wp_rw(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {

    return TEST_SKIP;

    int fd;
    unsigned long tmp, tmp1 = rand();
    fd = __event(HW_BREAKPOINT_R | HW_BREAKPOINT_W, (void *)&data1,
    sizeof(data1));
    if (fd < 0)
    let mut fd: return = = -ENODEV ? TEST_SKIP : -1;
    tmp = data1;
    WP_TEST_ASSERT_VAL(fd, "RW watchpoint", 1);
    data1 = tmp1 + tmp;
    WP_TEST_ASSERT_VAL(fd, "RW watchpoint", 2);
    close(fd);
    return 0;

    }
#[no_mangle]
unsafe extern "C" fn test__wp_modify(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__wp_modify(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {

    return TEST_SKIP;

    int fd, ret;
    let mut tmp: c_ulong = rand();
    struct perf_event_attr new_attr;
    fd = __event(HW_BREAKPOINT_W, (void *)&data1, sizeof(data1));
    if (fd < 0)
    let mut fd: return = = -ENODEV ? TEST_SKIP : -1;
    data1 = tmp;
    WP_TEST_ASSERT_VAL(fd, "Modify watchpoint", 1);
// Modify watchpoint with disabled = 1
    get__perf_event_attr(&new_attr, HW_BREAKPOINT_W, (void *)&data2[0],
    sizeof(u8) * 2);
    new_attr.disabled = 1;
    ret = ioctl(fd, PERF_EVENT_IOC_MODIFY_ATTRIBUTES, &new_attr);
    if (ret < 0) {
    if (errno == ENOTTY) {
    test.test_cases[subtest].skip_reason = "missing kernel support";
    ret = TEST_SKIP;
    }
    pr_debug("ioctl(PERF_EVENT_IOC_MODIFY_ATTRIBUTES) failed\n");
    close(fd);
    return ret;
    }
    data2[1] = tmp; /* Not Counted */
    WP_TEST_ASSERT_VAL(fd, "Modify watchpoint", 1);
// Enable the event
    ioctl(fd, PERF_EVENT_IOC_ENABLE, 0);
    if (ret < 0) {
    pr_debug("Failed to enable event\n");
    close(fd);
    return ret;
    }
    data2[1] = tmp; /* Counted */
    WP_TEST_ASSERT_VAL(fd, "Modify watchpoint", 2);
    data2[2] = tmp; /* Not Counted */
    WP_TEST_ASSERT_VAL(fd, "Modify watchpoint", 2);
    close(fd);
    return 0;

    }
    static struct test_case wp_tests[] = {
    TEST_CASE_REASON("Read Only Watchpoint", wp_ro, "missing hardware support"),
    TEST_CASE_REASON("Write Only Watchpoint", wp_wo, "missing hardware support"),
    TEST_CASE_REASON("Read / Write Watchpoint", wp_rw, "missing hardware support"),
    TEST_CASE_REASON("Modify Watchpoint", wp_modify, "missing hardware support"),
    { .name = core::ptr::null_mut(), }
    };
    struct test_suite suite__wp = {
    .desc = "Watchpoint",
    .test_cases = wp_tests,
    };
