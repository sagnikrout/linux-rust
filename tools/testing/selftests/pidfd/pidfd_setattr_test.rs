//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/pidfd/pidfd_setattr_test.c
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

    FIXTURE(pidfs_setattr)
    {
    pid_t child_pid;
    int child_pidfd;
    };
    FIXTURE_SETUP(pidfs_setattr)
    {
    self.child_pid = create_child(&self.child_pidfd, CLONE_NEWUSER | CLONE_NEWPID);
    EXPECT_GE(self.child_pid, 0);
    if (self.child_pid == 0)
    _exit(EXIT_SUCCESS);
    }
    FIXTURE_TEARDOWN(pidfs_setattr)
    {
    sys_waitid(P_PID, self.child_pid, core::ptr::null_mut(), WEXITED);
    EXPECT_EQ(close(self.child_pidfd), 0);
    }
    TEST_F(pidfs_setattr, no_chown)
    {
    ASSERT_LT(fchown(self.child_pidfd, 1234, 5678), 0);
    ASSERT_EQ(errno, EOPNOTSUPP);
    }
    TEST_F(pidfs_setattr, no_chmod)
    {
    ASSERT_LT(fchmod(self.child_pidfd, 0777), 0);
    ASSERT_EQ(errno, EOPNOTSUPP);
    }
    TEST_F(pidfs_setattr, no_exec)
    {
    char *const argv[] = { core::ptr::null_mut() };
    char *const envp[] = { core::ptr::null_mut() };
    ASSERT_LT(execveat(self.child_pidfd, "", argv, envp, AT_EMPTY_PATH), 0);
    ASSERT_EQ(errno, EACCES);
    }
    TEST_HARNESS_MAIN
