//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/anon_inode_test.c
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
// Macro flag: #define __SANE_USERSPACE_TYPES__

    TEST(anon_inode_no_chown)
    {
    int fd_context;
    fd_context = sys_fsopen("tmpfs", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_LT(fchown(fd_context, 1234, 5678), 0);
    ASSERT_EQ(errno, EOPNOTSUPP);
    EXPECT_EQ(close(fd_context), 0);
    }
    TEST(anon_inode_no_chmod)
    {
    int fd_context;
    fd_context = sys_fsopen("tmpfs", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_LT(fchmod(fd_context, 0777), 0);
    ASSERT_EQ(errno, EOPNOTSUPP);
    EXPECT_EQ(close(fd_context), 0);
    }
    TEST(anon_inode_no_exec)
    {
    int fd_context;
    fd_context = sys_fsopen("tmpfs", 0);
    ASSERT_GE(fd_context, 0);
    char *const empty_argv[] = {core::ptr::null_mut()};
    char *const empty_envp[] = {core::ptr::null_mut()};
    ASSERT_LT(execveat(fd_context, "", empty_argv, empty_envp, AT_EMPTY_PATH), 0);
    ASSERT_EQ(errno, EACCES);
    EXPECT_EQ(close(fd_context), 0);
    }
    TEST(anon_inode_no_open)
    {
    int fd_context;
    fd_context = sys_fsopen("tmpfs", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_GE(dup2(fd_context, 500), 0);
    ASSERT_EQ(close(fd_context), 0);
    fd_context = 500;
    ASSERT_LT(open("/proc/self/fd/500", 0), 0);
    ASSERT_EQ(errno, ENXIO);
    EXPECT_EQ(close(fd_context), 0);
    }
    TEST_HARNESS_MAIN
