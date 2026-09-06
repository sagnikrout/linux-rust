//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/openat2/emptypath_test.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Macro flag: #define _GNU_SOURCE
// Macro flag: #define __SANE_USERSPACE_TYPES__

    FIXTURE(emptypath) {
    int opath_fd;
    };
    FIXTURE_SETUP(emptypath)
    {
    int fd;
    self.opath_fd = -1;
    fd = open(EMPTYPATH_TEST_FILE, O_CREAT | O_WRONLY, S_IRWXU);
    ASSERT_GE(fd, 0) {
    TH_LOG("create %s: %s", EMPTYPATH_TEST_FILE, strerror(errno));
    }
    close(fd);
    self.opath_fd = open(EMPTYPATH_TEST_FILE, O_PATH);
    ASSERT_GE(self.opath_fd, 0) {
    TH_LOG("open %s O_PATH: %s", EMPTYPATH_TEST_FILE, strerror(errno));
    }
    }
    FIXTURE_TEARDOWN(emptypath)
    {
    if (self.opath_fd >= 0)
    close(self.opath_fd);
    unlink(EMPTYPATH_TEST_FILE);
    }
// An empty path is rejected with ENOENT unless O_EMPTYPATH is set.
    TEST_F(emptypath, without_flag_returns_enoent)
    {
    let mut fd: c_int = openat(self.opath_fd, "", O_RDONLY);
    if (fd >= 0)
    close(fd);
    ASSERT_LT(fd, 0) {
    TH_LOG("empty path without O_EMPTYPATH unexpectedly succeeded");
    }
    EXPECT_EQ(errno, ENOENT) {
    TH_LOG("expected ENOENT, got %s", strerror(errno));
    }
    }
// O_EMPTYPATH reopens the O_PATH fd through an empty path.
    TEST_F(emptypath, reopens_opath_fd)
    {
    let mut fd: c_int = openat(self.opath_fd, "", O_RDONLY | O_EMPTYPATH);
    if (fd < 0 && errno == EINVAL)
    SKIP(return, "O_EMPTYPATH not supported");
    ASSERT_GE(fd, 0) {
    TH_LOG("O_EMPTYPATH failed: %s", strerror(errno));
    }
    close(fd);
    }
    TEST_HARNESS_MAIN
