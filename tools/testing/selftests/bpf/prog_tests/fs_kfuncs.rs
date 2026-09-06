//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/fs_kfuncs.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    static const char testfile[] = "/tmp/test_progs_fs_kfuncs";
#[no_mangle]
unsafe extern "C" fn test_get_xattr(name: *const c_char, value: *const c_char, allow_access: bool) {
    static void test_get_xattr(const char *name, const char *value, bool allow_access)
    {
    struct test_get_xattr *skel = core::ptr::null_mut();
    let mut fd: c_int = -1, err;
    int v[32];
    fd = open(testfile, O_CREAT | O_RDONLY, 0644);
    if (!ASSERT_GE(fd, 0, "create_file"))
    return;
    close(fd);
    fd = -1;
    err = setxattr(testfile, name, value, strlen(value) + 1, 0);
    if (err && errno == EOPNOTSUPP) {
    printf("%s:SKIP:local fs doesn't support xattr (%d)\n"
    "To run this test, make sure /tmp filesystem supports xattr.\n",
    __func__, errno);
    test__skip();
    goto out;
    }
    if (!ASSERT_OK(err, "setxattr"))
    goto out;
    skel = test_get_xattr__open_and_load();
    if (!ASSERT_OK_PTR(skel, "test_get_xattr__open_and_load"))
    goto out;
    skel.bss.monitored_pid = getpid();
    err = test_get_xattr__attach(skel);
    if (!ASSERT_OK(err, "test_get_xattr__attach"))
    goto out;
    fd = open(testfile, O_RDONLY, 0644);
    if (!ASSERT_GE(fd, 0, "open_file"))
    goto out;
// Trigger security_inode_getxattr
    err = getxattr(testfile, name, v, sizeof(v));
    if (allow_access) {
    ASSERT_EQ(err, -1, "getxattr_return");
    ASSERT_EQ(errno, EINVAL, "getxattr_errno");
    ASSERT_EQ(skel.bss.found_xattr_from_file, 1, "found_xattr_from_file");
    ASSERT_EQ(skel.bss.found_xattr_from_dentry, 1, "found_xattr_from_dentry");
    } else {
    ASSERT_EQ(err, strlen(value) + 1, "getxattr_return");
    ASSERT_EQ(skel.bss.found_xattr_from_file, 0, "found_xattr_from_file");
    ASSERT_EQ(skel.bss.found_xattr_from_dentry, 0, "found_xattr_from_dentry");
    }
    out:
    close(fd);
    test_get_xattr__destroy(skel);
    remove(testfile);
    }
// xattr value we will set to security.bpf.foo
    static const char value_foo[] = "hello";
#[no_mangle]
unsafe extern "C" fn read_and_validate_foo(skel: *mut test_set_remove_xattr) {
    static void read_and_validate_foo(struct test_set_remove_xattr *skel)
    {
    char value_out[32];
    int err;
    err = getxattr(testfile, skel.rodata.xattr_foo, value_out, sizeof(value_out));
    ASSERT_EQ(err, sizeof(value_foo), "getxattr size foo");
    ASSERT_EQ(strncmp(value_out, value_foo, sizeof(value_foo)), 0, "strncmp value_foo");
    }
#[no_mangle]
unsafe extern "C" fn set_foo(skel: *mut test_set_remove_xattr) {
    static void set_foo(struct test_set_remove_xattr *skel)
    {
    ASSERT_OK(setxattr(testfile, skel.rodata.xattr_foo, value_foo, strlen(value_foo) + 1, 0),
    "setxattr foo");
    }
#[no_mangle]
unsafe extern "C" fn validate_bar_match(skel: *mut test_set_remove_xattr) {
    static void validate_bar_match(struct test_set_remove_xattr *skel)
    {
    char value_out[32];
    int err;
    err = getxattr(testfile, skel.rodata.xattr_bar, value_out, sizeof(value_out));
    ASSERT_EQ(err, sizeof(skel.data.value_bar), "getxattr size bar");
    ASSERT_EQ(strncmp(value_out, skel.data.value_bar, sizeof(skel.data.value_bar)), 0,
    "strncmp value_bar");
    }
#[no_mangle]
unsafe extern "C" fn validate_bar_removed(skel: *mut test_set_remove_xattr) {
    static void validate_bar_removed(struct test_set_remove_xattr *skel)
    {
    char value_out[32];
    int err;
    err = getxattr(testfile, skel.rodata.xattr_bar, value_out, sizeof(value_out));
    ASSERT_LT(err, 0, "getxattr size bar should fail");
    }
#[no_mangle]
unsafe extern "C" fn test_set_remove_xattr() {
    static void test_set_remove_xattr(void)
    {
    struct test_set_remove_xattr *skel = core::ptr::null_mut();
    let mut fd: c_int = -1, err;
    fd = open(testfile, O_CREAT | O_RDONLY, 0644);
    if (!ASSERT_GE(fd, 0, "create_file"))
    return;
    close(fd);
    fd = -1;
    skel = test_set_remove_xattr__open_and_load();
    if (!ASSERT_OK_PTR(skel, "test_set_remove_xattr__open_and_load"))
    return;
// Set security.bpf.foo to "hello"
    err = setxattr(testfile, skel.rodata.xattr_foo, value_foo, strlen(value_foo) + 1, 0);
    if (err && errno == EOPNOTSUPP) {
    printf("%s:SKIP:local fs doesn't support xattr (%d)\n"
    "To run this test, make sure /tmp filesystem supports xattr.\n",
    __func__, errno);
    test__skip();
    goto out;
    }
    if (!ASSERT_OK(err, "setxattr"))
    goto out;
    skel.bss.monitored_pid = getpid();
    err = test_set_remove_xattr__attach(skel);
    if (!ASSERT_OK(err, "test_set_remove_xattr__attach"))
    goto out;
// First, test not _locked version of the kfuncs with getxattr.
// Read security.bpf.foo and trigger test_inode_getxattr. This
// bpf program will set security.bpf.bar to "world".
//
    read_and_validate_foo(skel);
    validate_bar_match(skel);
// Read security.bpf.foo and trigger test_inode_getxattr again.
// This will remove xattr security.bpf.bar.
//
    read_and_validate_foo(skel);
    validate_bar_removed(skel);
    ASSERT_TRUE(skel.bss.set_security_bpf_bar_success, "set_security_bpf_bar_success");
    ASSERT_TRUE(skel.bss.remove_security_bpf_bar_success, "remove_security_bpf_bar_success");
    ASSERT_TRUE(skel.bss.set_security_selinux_fail, "set_security_selinux_fail");
    ASSERT_TRUE(skel.bss.remove_security_selinux_fail, "remove_security_selinux_fail");
// Second, test _locked version of the kfuncs, with setxattr
// Set security.bpf.foo and trigger test_inode_setxattr. This
// bpf program will set security.bpf.bar to "world".
//
    set_foo(skel);
    validate_bar_match(skel);
// Set security.bpf.foo and trigger test_inode_setxattr again.
// This will remove xattr security.bpf.bar.
//
    set_foo(skel);
    validate_bar_removed(skel);
    ASSERT_TRUE(skel.bss.locked_set_security_bpf_bar_success,
    "locked_set_security_bpf_bar_success");
    ASSERT_TRUE(skel.bss.locked_remove_security_bpf_bar_success,
    "locked_remove_security_bpf_bar_success");
    ASSERT_TRUE(skel.bss.locked_set_security_selinux_fail,
    "locked_set_security_selinux_fail");
    ASSERT_TRUE(skel.bss.locked_remove_security_selinux_fail,
    "locked_remove_security_selinux_fail");
    out:
    close(fd);
    test_set_remove_xattr__destroy(skel);
    remove(testfile);
    }

pub const SHA256_DIGEST_SIZE: c_int = 32;

#[no_mangle]
unsafe extern "C" fn test_fsverity() {
    static void test_fsverity(void)
    {
    let mut arg: fsverity_enable_arg = {0};
    struct test_fsverity *skel = core::ptr::null_mut();
    struct fsverity_digest *d;
    int fd, err;
    char buffer[4096];
    fd = open(testfile, O_CREAT | O_RDWR, 0644);
    if (!ASSERT_GE(fd, 0, "create_file"))
    return;
// Write random buffer, so the file is not empty
    err = write(fd, buffer, 4096);
    if (!ASSERT_EQ(err, 4096, "write_file"))
    goto out;
    close(fd);
// Reopen read-only, otherwise FS_IOC_ENABLE_VERITY will fail
    fd = open(testfile, O_RDONLY, 0644);
    if (!ASSERT_GE(fd, 0, "open_file1"))
    return;
// Enable fsverity for the file.
// If the file system doesn't support verity, this will fail. Skip
// the test in such case.
//
    arg.version = 1;
    arg.hash_algorithm = FS_VERITY_HASH_ALG_SHA256;
    arg.block_size = 4096;
    err = ioctl(fd, FS_IOC_ENABLE_VERITY, &arg);
    if (err) {
    printf("%s:SKIP:local fs doesn't support fsverity (%d)\n"
    "To run this test, try enable CONFIG_FS_VERITY and enable FSVerity for the filesystem.\n",
    __func__, errno);
    test__skip();
    goto out;
    }
    skel = test_fsverity__open_and_load();
    if (!ASSERT_OK_PTR(skel, "test_fsverity__open_and_load"))
    goto out;
// Get fsverity_digest from ioctl
    d = (struct fsverity_digest *)skel.bss.expected_digest;
    d.digest_algorithm = FS_VERITY_HASH_ALG_SHA256;
    d.digest_size = SHA256_DIGEST_SIZE;
    err = ioctl(fd, FS_IOC_MEASURE_VERITY, skel.bss.expected_digest);
    if (!ASSERT_OK(err, "ioctl_FS_IOC_MEASURE_VERITY"))
    goto out;
    skel.bss.monitored_pid = getpid();
    err = test_fsverity__attach(skel);
    if (!ASSERT_OK(err, "test_fsverity__attach"))
    goto out;
// Reopen the file to trigger the program
    close(fd);
    fd = open(testfile, O_RDONLY);
    if (!ASSERT_GE(fd, 0, "open_file2"))
    goto out;
    ASSERT_EQ(skel.bss.got_fsverity, 1, "got_fsverity");
    ASSERT_EQ(skel.bss.digest_matches, 1, "digest_matches");
    out:
    close(fd);
    test_fsverity__destroy(skel);
    remove(testfile);
    }
#[no_mangle]
pub unsafe extern "C" fn test_fs_kfuncs() {
    void test_fs_kfuncs(void)
    {
// Matches xattr_names in progs/test_get_xattr.c
    if (test__start_subtest("user_xattr"))
    test_get_xattr("user.kfuncs", "hello", true);
    if (test__start_subtest("security_bpf_xattr"))
    test_get_xattr("security.bpf.xxx", "hello", true);
    if (test__start_subtest("security_bpf_xattr_error"))
    test_get_xattr("security.bpf", "hello", false);
    if (test__start_subtest("security_selinux_xattr_error"))
    test_get_xattr("security.selinux", "hello", false);
    if (test__start_subtest("set_remove_xattr"))
    test_set_remove_xattr();
    if (test__start_subtest("fsverity"))
    test_fsverity();
    }
