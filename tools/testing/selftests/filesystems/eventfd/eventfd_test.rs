//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/eventfd/eventfd_test.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct error {
    pub code: c_int,
    pub msg: [c_char; 512],
}

#[no_mangle]
unsafe extern "C" fn error_set(err: *mut error, code: c_int, fmt: *const c_char, ...) -> c_int {
    static int error_set(struct error *err, int code, const char *fmt, ...)
    {
    va_list args;
    int r;
    if (code == 0 || !err || err.code != 0)
    return code;
    err.code = code;
    va_start(args, fmt);
    r = vsnprintf(err.msg, sizeof(err.msg), fmt, args);
    assert((size_t)r < sizeof(err.msg));
    va_end(args);
    return code;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_eventfd2(count: c_uint, flags: c_int) -> c_int {
    static inline int sys_eventfd2(unsigned int count, int flags)
    {
    return syscall(__NR_eventfd2, count, flags);
    }
    TEST(eventfd_check_flag_rdwr)
    {
    int fd, flags;
    fd = sys_eventfd2(0, 0);
    ASSERT_GE(fd, 0);
    flags = fcntl(fd, F_GETFL);
// The kernel automatically adds the O_RDWR flag.
    EXPECT_EQ(flags, O_RDWR);
    close(fd);
    }
    TEST(eventfd_check_flag_cloexec)
    {
    int fd, flags;
    fd = sys_eventfd2(0, EFD_CLOEXEC);
    ASSERT_GE(fd, 0);
    flags = fcntl(fd, F_GETFD);
    ASSERT_GT(flags, -1);
    EXPECT_EQ(flags, FD_CLOEXEC);
    close(fd);
    }
    TEST(eventfd_check_flag_nonblock)
    {
    int fd, flags;
    fd = sys_eventfd2(0, EFD_NONBLOCK);
    ASSERT_GE(fd, 0);
    flags = fcntl(fd, F_GETFL);
    ASSERT_GT(flags, -1);
    EXPECT_EQ(flags & EFD_NONBLOCK, EFD_NONBLOCK);
    EXPECT_EQ(flags & O_RDWR, O_RDWR);
    close(fd);
    }
    TEST(eventfd_check_flag_cloexec_and_nonblock)
    {
    int fd, flags;
    fd = sys_eventfd2(0, EFD_CLOEXEC|EFD_NONBLOCK);
    ASSERT_GE(fd, 0);
    flags = fcntl(fd, F_GETFL);
    ASSERT_GT(flags, -1);
    EXPECT_EQ(flags & EFD_NONBLOCK, EFD_NONBLOCK);
    EXPECT_EQ(flags & O_RDWR, O_RDWR);
    flags = fcntl(fd, F_GETFD);
    ASSERT_GT(flags, -1);
    EXPECT_EQ(flags, FD_CLOEXEC);
    close(fd);
    }
#[no_mangle]
pub unsafe extern "C" fn trim_newline(str: *mut c_char) {
    static inline void trim_newline(char *str)
    {
    char *pos = strrchr(str, '\n');
    if (pos)
// pos = '\0';
    }
    static int verify_fdinfo(int fd, struct error *err, const char *prefix,
    size_t prefix_len, const char *expect, ...)
    {
    char buffer[512] = {0, };
    char path[512] = {0, };
    va_list args;
    FILE *f;
    char *line = core::ptr::null_mut();
    let mut n: usize = 0;
    let mut found: c_int = 0;
    int r;
    va_start(args, expect);
    r = vsnprintf(buffer, sizeof(buffer), expect, args);
    assert((size_t)r < sizeof(buffer));
    va_end(args);
    snprintf(path, sizeof(path), "/proc/self/fdinfo/%d", fd);
    f = fopen(path, "re");
    if (!f)
    return error_set(err, -1, "fdinfo open failed for %d", fd);
    while (getline(&line, &n, f) != -1) {
    char *val;
    if (strncmp(line, prefix, prefix_len))
    continue;
    found = 1;
    val = line + prefix_len;
    r = strcmp(val, buffer);
    if (r != 0) {
    trim_newline(line);
    trim_newline(buffer);
    error_set(err, -1, "%s '%s' != '%s'",
    prefix, val, buffer);
    }
    break;
    }
    free(line);
    fclose(f);
    if (found == 0)
    return error_set(err, -1, "%s not found for fd %d",
    prefix, fd);
    return 0;
    }
    TEST(eventfd_check_flag_semaphore)
    {
    let mut err: error = {0};
    int fd, ret;
    fd = sys_eventfd2(0, EFD_SEMAPHORE);
    ASSERT_GE(fd, 0);
    ret = fcntl(fd, F_GETFL);
    ASSERT_GT(ret, -1);
    EXPECT_EQ(ret & O_RDWR, O_RDWR);
// The semaphore could only be obtained from fdinfo.
    ret = verify_fdinfo(fd, &err, "eventfd-semaphore: ", 19, "1\n");
    if (ret != 0)
    ksft_print_msg("eventfd semaphore flag check failed: %s\n", err.msg);
    EXPECT_EQ(ret, 0);
    close(fd);
    }
//
// A write(2) fails with the error EINVAL if the size of the supplied buffer
// is less than 8 bytes, or if an attempt is made to write the value
// 0xffffffffffffffff.
//
    TEST(eventfd_check_write)
    {
    let mut value: u64 = 1;
    ssize_t size;
    int fd;
    fd = sys_eventfd2(0, 0);
    ASSERT_GE(fd, 0);
    size = write(fd, &value, sizeof(int));
    EXPECT_EQ(size, -1);
    EXPECT_EQ(errno, EINVAL);
    size = write(fd, &value, sizeof(value));
    EXPECT_EQ(size, sizeof(value));
    value = (uint64_t)-1;
    size = write(fd, &value, sizeof(value));
    EXPECT_EQ(size, -1);
    EXPECT_EQ(errno, EINVAL);
    close(fd);
    }
//
// A read(2) fails with the error EINVAL if the size of the supplied buffer is
// less than 8 bytes.
//
    TEST(eventfd_check_read)
    {
    uint64_t value;
    ssize_t size;
    int fd;
    fd = sys_eventfd2(1, 0);
    ASSERT_GE(fd, 0);
    size = read(fd, &value, sizeof(int));
    EXPECT_EQ(size, -1);
    EXPECT_EQ(errno, EINVAL);
    size = read(fd, &value, sizeof(value));
    EXPECT_EQ(size, sizeof(value));
    EXPECT_EQ(value, 1);
    close(fd);
    }
//
// If EFD_SEMAPHORE was not specified and the eventfd counter has a nonzero
// value, then a read(2) returns 8 bytes containing that value, and the
// counter's value is reset to zero.
// If the eventfd counter is zero at the time of the call to read(2), then the
// call fails with the error EAGAIN if the file descriptor has been made nonblocking.
//
    TEST(eventfd_check_read_with_nonsemaphore)
    {
    uint64_t value;
    ssize_t size;
    int fd;
    int i;
    fd = sys_eventfd2(0, EFD_NONBLOCK);
    ASSERT_GE(fd, 0);
    value = 1;
    for (i = 0; i < EVENTFD_TEST_ITERATIONS; i++) {
    size = write(fd, &value, sizeof(value));
    EXPECT_EQ(size, sizeof(value));
    }
    size = read(fd, &value, sizeof(value));
    EXPECT_EQ(size, sizeof(uint64_t));
    EXPECT_EQ(value, EVENTFD_TEST_ITERATIONS);
    size = read(fd, &value, sizeof(value));
    EXPECT_EQ(size, -1);
    EXPECT_EQ(errno, EAGAIN);
    close(fd);
    }
//
// If EFD_SEMAPHORE was specified and the eventfd counter has a nonzero value,
// then a read(2) returns 8 bytes containing the value 1, and the counter's
// value is decremented by 1.
// If the eventfd counter is zero at the time of the call to read(2), then the
// call fails with the error EAGAIN if the file descriptor has been made nonblocking.
//
    TEST(eventfd_check_read_with_semaphore)
    {
    uint64_t value;
    ssize_t size;
    int fd;
    int i;
    fd = sys_eventfd2(0, EFD_SEMAPHORE|EFD_NONBLOCK);
    ASSERT_GE(fd, 0);
    value = 1;
    for (i = 0; i < EVENTFD_TEST_ITERATIONS; i++) {
    size = write(fd, &value, sizeof(value));
    EXPECT_EQ(size, sizeof(value));
    }
    for (i = 0; i < EVENTFD_TEST_ITERATIONS; i++) {
    size = read(fd, &value, sizeof(value));
    EXPECT_EQ(size, sizeof(value));
    EXPECT_EQ(value, 1);
    }
    size = read(fd, &value, sizeof(value));
    EXPECT_EQ(size, -1);
    EXPECT_EQ(errno, EAGAIN);
    close(fd);
    }
    TEST_HARNESS_MAIN
