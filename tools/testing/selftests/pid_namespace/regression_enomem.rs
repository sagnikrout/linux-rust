//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/pid_namespace/regression_enomem.c
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


// Macro flag: #define _GNU_SOURCE

//
// Regression test for:
// 35f71bc0a09a ("fork: report pid reservation failure properly")
// b26ebfe12f34 ("pid: Fix error return value in some cases")
//
    TEST(regression_enomem)
    {
    pid_t pid;
    if (geteuid())
    EXPECT_EQ(0, unshare(CLONE_NEWUSER));
    EXPECT_EQ(0, unshare(CLONE_NEWPID));
    pid = fork();
    ASSERT_GE(pid, 0);
    if (pid == 0)
    exit(EXIT_SUCCESS);
    EXPECT_EQ(0, wait_for_pid(pid));
    pid = fork();
    ASSERT_LT(pid, 0);
    ASSERT_EQ(errno, ENOMEM);
    }
    TEST_HARNESS_MAIN
