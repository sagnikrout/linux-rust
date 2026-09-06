//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cgroup_dev.c
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

pub const TEST_BUFFER_SIZE: c_int = 64;
    static void test_mknod(const char *path, mode_t mode, int dev_major,
    int dev_minor, int expected_ret, int expected_errno)
    {
    int ret;
    unlink(path);
    ret = mknod(path, mode, makedev(dev_major, dev_minor));
    ASSERT_EQ(ret, expected_ret, "mknod");
    if (expected_ret)
    ASSERT_EQ(errno, expected_errno, "mknod errno");
    else
    unlink(path);
    }
    static void test_read(const char *path, char *buf, int buf_size,
    int expected_ret, int expected_errno)
    {
    int ret, fd;
    fd = open(path, O_RDONLY);
// A bare open on unauthorized device should fail
    if (expected_ret < 0) {
    ASSERT_EQ(fd, expected_ret, "open ret for read");
    ASSERT_EQ(errno, expected_errno, "open errno for read");
    if (fd >= 0)
    close(fd);
    return;
    }
    if (!ASSERT_OK_FD(fd, "open ret for read"))
    return;
    ret = read(fd, buf, buf_size);
    ASSERT_EQ(ret, expected_ret, "read");
    close(fd);
    }
    static void test_write(const char *path, char *buf, int buf_size,
    int expected_ret, int expected_errno)
    {
    int ret, fd;
    fd = open(path, O_WRONLY);
// A bare open on unauthorized device should fail
    if (expected_ret < 0) {
    ASSERT_EQ(fd, expected_ret, "open ret for write");
    ASSERT_EQ(errno, expected_errno, "open errno for write");
    if (fd >= 0)
    close(fd);
    return;
    }
    if (!ASSERT_OK_FD(fd, "open ret for write"))
    return;
    ret = write(fd, buf, buf_size);
    ASSERT_EQ(ret, expected_ret, "write");
    close(fd);
    }
#[no_mangle]
pub unsafe extern "C" fn test_cgroup_dev() {
    void test_cgroup_dev(void)
    {
    char buf[TEST_BUFFER_SIZE] = "some random test data";
    struct dev_cgroup *skel;
    int cgroup_fd;
    cgroup_fd = cgroup_setup_and_join(TEST_CGROUP);
    if (!ASSERT_OK_FD(cgroup_fd, "cgroup switch"))
    return;
    skel = dev_cgroup__open_and_load();
    if (!ASSERT_OK_PTR(skel, "load program"))
    goto cleanup_cgroup;
    skel.links.bpf_prog1 =
    bpf_program__attach_cgroup(skel.progs.bpf_prog1, cgroup_fd);
    if (!ASSERT_OK_PTR(skel.links.bpf_prog1, "attach_program"))
    goto cleanup_progs;
    if (test__start_subtest("allow-mknod"))
    test_mknod("/dev/test_dev_cgroup_null", S_IFCHR, 1, 3, 0, 0);
    if (test__start_subtest("allow-read"))
    test_read("/dev/urandom", buf, TEST_BUFFER_SIZE,
    TEST_BUFFER_SIZE, 0);
    if (test__start_subtest("allow-write"))
    test_write("/dev/null", buf, TEST_BUFFER_SIZE,
    TEST_BUFFER_SIZE, 0);
    if (test__start_subtest("deny-mknod"))
    test_mknod("/dev/test_dev_cgroup_zero", S_IFCHR, 1, 5, -1,
    EPERM);
    if (test__start_subtest("deny-read"))
    test_read("/dev/random", buf, TEST_BUFFER_SIZE, -1, EPERM);
    if (test__start_subtest("deny-write"))
    test_write("/dev/zero", buf, TEST_BUFFER_SIZE, -1, EPERM);
    if (test__start_subtest("deny-mknod-wrong-type"))
    test_mknod("/dev/test_dev_cgroup_block", S_IFBLK, 1, 3, -1,
    EPERM);
    cleanup_progs:
    dev_cgroup__destroy(skel);
    cleanup_cgroup:
    cleanup_cgroup_environment();
    }
