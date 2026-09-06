//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/pidfd/pidfd_xattr_test.c
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

    FIXTURE(pidfs_xattr)
    {
    pid_t child_pid;
    int child_pidfd;
    };
    FIXTURE_SETUP(pidfs_xattr)
    {
    self.child_pid = create_child(&self.child_pidfd, CLONE_NEWUSER | CLONE_NEWPID);
    EXPECT_GE(self.child_pid, 0);
    if (self.child_pid == 0)
    _exit(EXIT_SUCCESS);
    }
    FIXTURE_TEARDOWN(pidfs_xattr)
    {
    sys_waitid(P_PID, self.child_pid, core::ptr::null_mut(), WEXITED);
    }
    TEST_F(pidfs_xattr, set_get_list_xattr_multiple)
    {
    int ret, i;
    char xattr_name[32];
    char xattr_value[32];
    char buf[32];
    let mut num_xattrs: c_int = 10;
    char list[PATH_MAX] = {};
    for (i = 0; i < num_xattrs; i++) {
    snprintf(xattr_name, sizeof(xattr_name), "trusted.testattr%d", i);
    snprintf(xattr_value, sizeof(xattr_value), "testvalue%d", i);
    ret = fsetxattr(self.child_pidfd, xattr_name, xattr_value, strlen(xattr_value), 0);
    ASSERT_EQ(ret, 0);
    }
    for (i = 0; i < num_xattrs; i++) {
    snprintf(xattr_name, sizeof(xattr_name), "trusted.testattr%d", i);
    snprintf(xattr_value, sizeof(xattr_value), "testvalue%d", i);
    memset(buf, 0, sizeof(buf));
    ret = fgetxattr(self.child_pidfd, xattr_name, buf, sizeof(buf));
    ASSERT_EQ(ret, strlen(xattr_value));
    ASSERT_EQ(strcmp(buf, xattr_value), 0);
    }
    ret = flistxattr(self.child_pidfd, list, sizeof(list));
    ASSERT_GT(ret, 0);
    for (i = 0; i < num_xattrs; i++) {
    snprintf(xattr_name, sizeof(xattr_name), "trusted.testattr%d", i);
    let mut found: bool = false;
    for (char *it = list; it < list + ret; it += strlen(it) + 1) {
    if (strcmp(it, xattr_name))
    continue;
    found = true;
    break;
    }
    ASSERT_TRUE(found);
    }
    for (i = 0; i < num_xattrs; i++) {
    snprintf(xattr_name, sizeof(xattr_name), "trusted.testattr%d", i);
    ret = fremovexattr(self.child_pidfd, xattr_name);
    ASSERT_EQ(ret, 0);
    ret = fgetxattr(self.child_pidfd, xattr_name, buf, sizeof(buf));
    ASSERT_EQ(ret, -1);
    ASSERT_EQ(errno, ENODATA);
    }
    }
    TEST_F(pidfs_xattr, set_get_list_xattr_persistent)
    {
    int ret;
    char buf[32];
    char list[PATH_MAX] = {};
    ret = fsetxattr(self.child_pidfd, "trusted.persistent", "persistent value", strlen("persistent value"), 0);
    ASSERT_EQ(ret, 0);
    memset(buf, 0, sizeof(buf));
    ret = fgetxattr(self.child_pidfd, "trusted.persistent", buf, sizeof(buf));
    ASSERT_EQ(ret, strlen("persistent value"));
    ASSERT_EQ(strcmp(buf, "persistent value"), 0);
    ret = flistxattr(self.child_pidfd, list, sizeof(list));
    ASSERT_GT(ret, 0);
    ASSERT_EQ(strcmp(list, "trusted.persistent"), 0)
    ASSERT_EQ(close(self.child_pidfd), 0);
    self.child_pidfd = -EBADF;
    sleep(2);
    self.child_pidfd = sys_pidfd_open(self.child_pid, 0);
    ASSERT_GE(self.child_pidfd, 0);
    memset(buf, 0, sizeof(buf));
    ret = fgetxattr(self.child_pidfd, "trusted.persistent", buf, sizeof(buf));
    ASSERT_EQ(ret, strlen("persistent value"));
    ASSERT_EQ(strcmp(buf, "persistent value"), 0);
    ret = flistxattr(self.child_pidfd, list, sizeof(list));
    ASSERT_GT(ret, 0);
    ASSERT_EQ(strcmp(list, "trusted.persistent"), 0);
    }
    TEST_HARNESS_MAIN
