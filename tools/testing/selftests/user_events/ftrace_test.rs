//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/user_events/ftrace_test.c
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
// User Events FTrace Test Program
//
// Copyright (c) 2021 Beau Belgrave <beaub@linux.microsoft.com>
//

    const char *data_file = "/sys/kernel/tracing/user_events_data";
    const char *status_file = "/sys/kernel/tracing/user_events_status";
    const char *enable_file = "/sys/kernel/tracing/events/user_events/__test_event/enable";
    const char *trace_file = "/sys/kernel/tracing/trace";
    const char *fmt_file = "/sys/kernel/tracing/events/user_events/__test_event/format";
#[no_mangle]
unsafe extern "C" fn trace_bytes() -> c_int {
    static int trace_bytes(void)
    {
    let mut fd: c_int = open(trace_file, O_RDONLY);
    char buf[256];
    let mut bytes: c_int = 0, got;
    if (fd == -1)
    return -1;
    while (true) {
    got = read(fd, buf, sizeof(buf));
    if (got == -1)
    return -1;
    if (got == 0)
    break;
    bytes += got;
    }
    close(fd);
    return bytes;
    }
#[no_mangle]
unsafe extern "C" fn skip_until_empty_line(fp: *mut FILE) -> c_int {
    static int skip_until_empty_line(FILE *fp)
    {
    int c, last = 0;
    while (true) {
    c = getc(fp);
    if (c == EOF)
    break;
    if (last == '\n' && c == '\n')
    return 0;
    last = c;
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn get_print_fmt(buffer: *mut c_char, len: c_int) -> c_int {
    static int get_print_fmt(char *buffer, int len)
    {
    FILE *fp = fopen(fmt_file, "r");
    char *newline;
    if (!fp)
    return -1;
// Read until empty line (Skip Common)
    if (skip_until_empty_line(fp) < 0)
    goto err;
// Read until empty line (Skip Properties)
    if (skip_until_empty_line(fp) < 0)
    goto err;
// Read in print_fmt:
    if (fgets(buffer, len, fp) == core::ptr::null_mut())
    goto err;
    newline = strchr(buffer, '\n');
    if (newline)
// newline = '\0';
    fclose(fp);
    return 0;
    err:
    fclose(fp);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn wait_for_delete() -> bool {
    static bool wait_for_delete(void)
    {
    int i;
    for (i = 0; i < 1000; ++i) {
    let mut fd: c_int = open(enable_file, O_RDONLY);
    if (fd == -1)
    return true;
    close(fd);
    usleep(1000);
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn clear(check: *mut c_int) -> c_int {
    static int clear(int *check)
    {
    let mut unreg: user_unreg = {0};
    int fd;
    unreg.size = sizeof(unreg);
    unreg.disable_bit = 31;
    unreg.disable_addr = (__u64)check;
    fd = open(data_file, O_RDWR);
    if (fd == -1)
    return -1;
    if (ioctl(fd, DIAG_IOCSUNREG, &unreg) == -1)
    if (errno != ENOENT)
    goto fail;
    if (ioctl(fd, DIAG_IOCSDEL, "__test_event") == -1) {
    if (errno == EBUSY) {
    if (!wait_for_delete())
    goto fail;
    } else if (errno != ENOENT)
    goto fail;
    }
    close(fd);
    return 0;
    fail:
    close(fd);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn check_print_fmt(event: *const c_char, expected: *const c_char, check: *mut c_int) -> c_int {
    static int check_print_fmt(const char *event, const char *expected, int *check)
    {
    let mut reg: user_reg = {0};
    char print_fmt[256];
    int ret;
    int fd;
// Ensure cleared
    ret = clear(check);
    if (ret != 0)
    return ret;
    fd = open(data_file, O_RDWR);
    if (fd == -1)
    return fd;
    reg.size = sizeof(reg);
    reg.name_args = (__u64)event;
    reg.enable_bit = 31;
    reg.enable_addr = (__u64)check;
    reg.enable_size = sizeof(*check);
// Register should work
    ret = ioctl(fd, DIAG_IOCSREG, &reg);
    if (ret != 0) {
    close(fd);
    printf("Reg failed in fmt\n");
    return ret;
    }
// Ensure correct print_fmt
    ret = get_print_fmt(print_fmt, sizeof(print_fmt));
    close(fd);
    if (ret != 0)
    return ret;
    return strcmp(print_fmt, expected);
    }
    FIXTURE(user) {
    int status_fd;
    int data_fd;
    int enable_fd;
    int check;
    bool umount;
    };
    FIXTURE_SETUP(user) {
    USER_EVENT_FIXTURE_SETUP(return, self.umount);
    self.status_fd = open(status_file, O_RDONLY);
    ASSERT_NE(-1, self.status_fd);
    self.data_fd = open(data_file, O_RDWR);
    ASSERT_NE(-1, self.data_fd);
    self.enable_fd = -1;
    }
    FIXTURE_TEARDOWN(user) {
    USER_EVENT_FIXTURE_TEARDOWN(self.umount);
    close(self.status_fd);
    close(self.data_fd);
    if (self.enable_fd != -1) {
    write(self.enable_fd, "0", sizeof("0"));
    close(self.enable_fd);
    }
    if (clear(&self.check) != 0)
    printf("WARNING: Clear didn't work!\n");
    }
    TEST_F(user, register_events) {
    let mut reg: user_reg = {0};
    let mut unreg: user_unreg = {0};
    reg.size = sizeof(reg);
    reg.name_args = (__u64)"__test_event u32 field1; u32 field2";
    reg.enable_bit = 31;
    reg.enable_addr = (__u64)&self.check;
    reg.enable_size = sizeof(self.check);
    unreg.size = sizeof(unreg);
    unreg.disable_bit = 31;
    unreg.disable_addr = (__u64)&self.check;
// Register should work
    ASSERT_EQ(0, ioctl(self.data_fd, DIAG_IOCSREG, &reg));
    ASSERT_EQ(0, reg.write_index);
// Multiple registers to the same addr + bit should fail
    ASSERT_EQ(-1, ioctl(self.data_fd, DIAG_IOCSREG, &reg));
    ASSERT_EQ(EADDRINUSE, errno);
// Multiple registers to same name should result in same index
    reg.enable_bit = 30;
    ASSERT_EQ(0, ioctl(self.data_fd, DIAG_IOCSREG, &reg));
    ASSERT_EQ(0, reg.write_index);
// Register without separator spacing should still match
    reg.enable_bit = 29;
    reg.name_args = (__u64)"__test_event u32 field1;u32 field2";
    ASSERT_EQ(0, ioctl(self.data_fd, DIAG_IOCSREG, &reg));
    ASSERT_EQ(0, reg.write_index);
// Multiple registers to same name but different args should fail
    reg.enable_bit = 29;
    reg.name_args = (__u64)"__test_event u32 field1;";
    ASSERT_EQ(-1, ioctl(self.data_fd, DIAG_IOCSREG, &reg));
    ASSERT_EQ(EADDRINUSE, errno);
// Ensure disabled
    self.enable_fd = open(enable_file, O_RDWR);
    ASSERT_NE(-1, self.enable_fd);
    ASSERT_NE(-1, write(self.enable_fd, "0", sizeof("0")))
// Enable event and ensure bits updated in status
    ASSERT_NE(-1, write(self.enable_fd, "1", sizeof("1")))
    ASSERT_EQ(1 << reg.enable_bit, self.check);
// Disable event and ensure bits updated in status
    ASSERT_NE(-1, write(self.enable_fd, "0", sizeof("0")))
    ASSERT_EQ(0, self.check);
// File still open should return -EBUSY for delete
    ASSERT_EQ(-1, ioctl(self.data_fd, DIAG_IOCSDEL, "__test_event"));
    ASSERT_EQ(EBUSY, errno);
// Unregister
    ASSERT_EQ(0, ioctl(self.data_fd, DIAG_IOCSUNREG, &unreg));
    unreg.disable_bit = 30;
    ASSERT_EQ(0, ioctl(self.data_fd, DIAG_IOCSUNREG, &unreg));
    unreg.disable_bit = 29;
    ASSERT_EQ(0, ioctl(self.data_fd, DIAG_IOCSUNREG, &unreg));
// Delete should have been auto-done after close and unregister
    close(self.data_fd);
    ASSERT_EQ(true, wait_for_delete());
    }
    TEST_F(user, write_events) {
    let mut reg: user_reg = {0};
    struct iovec io[3];
    __u32 field1, field2;
    let mut before: c_int = 0, after = 0;
    reg.size = sizeof(reg);
    reg.name_args = (__u64)"__test_event u32 field1; u32 field2";
    reg.enable_bit = 31;
    reg.enable_addr = (__u64)&self.check;
    reg.enable_size = sizeof(self.check);
    field1 = 1;
    field2 = 2;
    io[0].iov_base = &reg.write_index;
    io[0].iov_len = sizeof(reg.write_index);
    io[1].iov_base = &field1;
    io[1].iov_len = sizeof(field1);
    io[2].iov_base = &field2;
    io[2].iov_len = sizeof(field2);
// Register should work
    ASSERT_EQ(0, ioctl(self.data_fd, DIAG_IOCSREG, &reg));
    ASSERT_EQ(0, reg.write_index);
    ASSERT_EQ(0, self.check);
// Write should fail on invalid slot with ENOENT
    io[0].iov_base = &field2;
    io[0].iov_len = sizeof(field2);
    ASSERT_EQ(-1, writev(self.data_fd, (const struct iovec *)io, 3));
    ASSERT_EQ(ENOENT, errno);
    io[0].iov_base = &reg.write_index;
    io[0].iov_len = sizeof(reg.write_index);
// Write should return -EBADF when event is not enabled
    ASSERT_EQ(-1, writev(self.data_fd, (const struct iovec *)io, 3));
    ASSERT_EQ(EBADF, errno);
// Enable event
    self.enable_fd = open(enable_file, O_RDWR);
    ASSERT_NE(-1, write(self.enable_fd, "1", sizeof("1")))
// Event should now be enabled
    ASSERT_NE(1 << reg.enable_bit, self.check);
// Write should make it out to ftrace buffers
    before = trace_bytes();
    ASSERT_NE(-1, writev(self.data_fd, (const struct iovec *)io, 3));
    after = trace_bytes();
    ASSERT_GT(after, before);
// Negative index should fail with EINVAL
    reg.write_index = -1;
    ASSERT_EQ(-1, writev(self.data_fd, (const struct iovec *)io, 3));
    ASSERT_EQ(EINVAL, errno);
    }
    TEST_F(user, write_empty_events) {
    let mut reg: user_reg = {0};
    struct iovec io[1];
    let mut before: c_int = 0, after = 0;
    reg.size = sizeof(reg);
    reg.name_args = (__u64)"__test_event";
    reg.enable_bit = 31;
    reg.enable_addr = (__u64)&self.check;
    reg.enable_size = sizeof(self.check);
    io[0].iov_base = &reg.write_index;
    io[0].iov_len = sizeof(reg.write_index);
// Register should work
    ASSERT_EQ(0, ioctl(self.data_fd, DIAG_IOCSREG, &reg));
    ASSERT_EQ(0, reg.write_index);
    ASSERT_EQ(0, self.check);
// Enable event
    self.enable_fd = open(enable_file, O_RDWR);
    ASSERT_NE(-1, write(self.enable_fd, "1", sizeof("1")))
// Event should now be enabled
    ASSERT_EQ(1 << reg.enable_bit, self.check);
// Write should make it out to ftrace buffers
    before = trace_bytes();
    ASSERT_NE(-1, writev(self.data_fd, (const struct iovec *)io, 1));
    after = trace_bytes();
    ASSERT_GT(after, before);
    }
    TEST_F(user, write_fault) {
    let mut reg: user_reg = {0};
    struct iovec io[2];
    let mut l: c_int = sizeof(__u64);
    void *anon;
    reg.size = sizeof(reg);
    reg.name_args = (__u64)"__test_event u64 anon";
    reg.enable_bit = 31;
    reg.enable_addr = (__u64)&self.check;
    reg.enable_size = sizeof(self.check);
    anon = mmap(core::ptr::null_mut(), l, PROT_READ, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    ASSERT_NE(MAP_FAILED, anon);
    io[0].iov_base = &reg.write_index;
    io[0].iov_len = sizeof(reg.write_index);
    io[1].iov_base = anon;
    io[1].iov_len = l;
// Register should work
    ASSERT_EQ(0, ioctl(self.data_fd, DIAG_IOCSREG, &reg));
    ASSERT_EQ(0, reg.write_index);
// Enable event
    self.enable_fd = open(enable_file, O_RDWR);
    ASSERT_NE(-1, write(self.enable_fd, "1", sizeof("1")))
// Write should work normally
    ASSERT_NE(-1, writev(self.data_fd, (const struct iovec *)io, 2));
// Faulted data should zero fill and work
    ASSERT_EQ(0, madvise(anon, l, MADV_DONTNEED));
    ASSERT_NE(-1, writev(self.data_fd, (const struct iovec *)io, 2));
    ASSERT_EQ(0, munmap(anon, l));
    }
    TEST_F(user, write_validator) {
    let mut reg: user_reg = {0};
    struct iovec io[3];
    int loc, bytes;
    char data[8];
    let mut before: c_int = 0, after = 0;
    reg.size = sizeof(reg);
    reg.name_args = (__u64)"__test_event __rel_loc char[] data";
    reg.enable_bit = 31;
    reg.enable_addr = (__u64)&self.check;
    reg.enable_size = sizeof(self.check);
// Register should work
    ASSERT_EQ(0, ioctl(self.data_fd, DIAG_IOCSREG, &reg));
    ASSERT_EQ(0, reg.write_index);
    ASSERT_EQ(0, self.check);
    io[0].iov_base = &reg.write_index;
    io[0].iov_len = sizeof(reg.write_index);
    io[1].iov_base = &loc;
    io[1].iov_len = sizeof(loc);
    io[2].iov_base = data;
    bytes = snprintf(data, sizeof(data), "Test") + 1;
    io[2].iov_len = bytes;
// Undersized write should fail
    ASSERT_EQ(-1, writev(self.data_fd, (const struct iovec *)io, 1));
    ASSERT_EQ(EINVAL, errno);
// Enable event
    self.enable_fd = open(enable_file, O_RDWR);
    ASSERT_NE(-1, write(self.enable_fd, "1", sizeof("1")))
// Event should now be enabled
    ASSERT_EQ(1 << reg.enable_bit, self.check);
// Full in-bounds write should work
    before = trace_bytes();
    loc = DYN_LOC(0, bytes);
    ASSERT_NE(-1, writev(self.data_fd, (const struct iovec *)io, 3));
    after = trace_bytes();
    ASSERT_GT(after, before);
// Out of bounds write should fault (offset way out)
    loc = DYN_LOC(1024, bytes);
    ASSERT_EQ(-1, writev(self.data_fd, (const struct iovec *)io, 3));
    ASSERT_EQ(EFAULT, errno);
// Out of bounds write should fault (offset 1 byte out)
    loc = DYN_LOC(1, bytes);
    ASSERT_EQ(-1, writev(self.data_fd, (const struct iovec *)io, 3));
    ASSERT_EQ(EFAULT, errno);
// Out of bounds write should fault (size way out)
    loc = DYN_LOC(0, bytes + 1024);
    ASSERT_EQ(-1, writev(self.data_fd, (const struct iovec *)io, 3));
    ASSERT_EQ(EFAULT, errno);
// Out of bounds write should fault (size 1 byte out)
    loc = DYN_LOC(0, bytes + 1);
    ASSERT_EQ(-1, writev(self.data_fd, (const struct iovec *)io, 3));
    ASSERT_EQ(EFAULT, errno);
// Non-Null should fault
    memset(data, 'A', sizeof(data));
    loc = DYN_LOC(0, bytes);
    ASSERT_EQ(-1, writev(self.data_fd, (const struct iovec *)io, 3));
    ASSERT_EQ(EFAULT, errno);
    }
    TEST_F(user, print_fmt) {
    int ret;
    ret = check_print_fmt("__test_event __rel_loc char[] data",
    "print fmt: \"data=%s\", __get_rel_str(data)",
    &self.check);
    ASSERT_EQ(0, ret);
    ret = check_print_fmt("__test_event __data_loc char[] data",
    "print fmt: \"data=%s\", __get_str(data)",
    &self.check);
    ASSERT_EQ(0, ret);
    ret = check_print_fmt("__test_event s64 data",
    "print fmt: \"data=%lld\", REC.data",
    &self.check);
    ASSERT_EQ(0, ret);
    ret = check_print_fmt("__test_event u64 data",
    "print fmt: \"data=%llu\", REC.data",
    &self.check);
    ASSERT_EQ(0, ret);
    ret = check_print_fmt("__test_event s32 data",
    "print fmt: \"data=%d\", REC.data",
    &self.check);
    ASSERT_EQ(0, ret);
    ret = check_print_fmt("__test_event u32 data",
    "print fmt: \"data=%u\", REC.data",
    &self.check);
    ASSERT_EQ(0, ret);
    ret = check_print_fmt("__test_event int data",
    "print fmt: \"data=%d\", REC.data",
    &self.check);
    ASSERT_EQ(0, ret);
    ret = check_print_fmt("__test_event unsigned int data",
    "print fmt: \"data=%u\", REC.data",
    &self.check);
    ASSERT_EQ(0, ret);
    ret = check_print_fmt("__test_event s16 data",
    "print fmt: \"data=%d\", REC.data",
    &self.check);
    ASSERT_EQ(0, ret);
    ret = check_print_fmt("__test_event u16 data",
    "print fmt: \"data=%u\", REC.data",
    &self.check);
    ASSERT_EQ(0, ret);
    ret = check_print_fmt("__test_event short data",
    "print fmt: \"data=%d\", REC.data",
    &self.check);
    ASSERT_EQ(0, ret);
    ret = check_print_fmt("__test_event unsigned short data",
    "print fmt: \"data=%u\", REC.data",
    &self.check);
    ASSERT_EQ(0, ret);
    ret = check_print_fmt("__test_event s8 data",
    "print fmt: \"data=%d\", REC.data",
    &self.check);
    ASSERT_EQ(0, ret);
    ret = check_print_fmt("__test_event u8 data",
    "print fmt: \"data=%u\", REC.data",
    &self.check);
    ASSERT_EQ(0, ret);
    ret = check_print_fmt("__test_event char data",
    "print fmt: \"data=%d\", REC.data",
    &self.check);
    ASSERT_EQ(0, ret);
    ret = check_print_fmt("__test_event unsigned char data",
    "print fmt: \"data=%u\", REC.data",
    &self.check);
    ASSERT_EQ(0, ret);
    ret = check_print_fmt("__test_event char[4] data",
    "print fmt: \"data=%s\", REC.data",
    &self.check);
    ASSERT_EQ(0, ret);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    return test_harness_run(argc, argv);
    }
