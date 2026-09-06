//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/namespaces/regression_pidfd_setns_test.c
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

//
// Regression tests for the setns(pidfd) active reference counting bug.
//
// These tests are based on the reproducers that triggered the race condition
// fixed by commit 1c465d0518dc ("ns: handle setns(pidfd, ...) cleanly").
//
// The bug: When using setns() with a pidfd, if the target task exits between
// prepare_nsset() and commit_nsset(), the namespaces would become inactive.
// Then ns_ref_active_get() would increment from 0 without properly resurrecting
// the owner chain, causing active reference count underflows.
//
// Simple pidfd setns test using create_child()+unshare().
//
// Without the fix, this would trigger active refcount warnings when the
// parent exits after doing setns(pidfd) on a child that has already exited.
//
    TEST(simple_pidfd_setns)
    {
    pid_t child_pid;
    let mut pidfd: c_int = -1;
    int ret;
    int sv[2];
    char c;
// Ignore SIGCHLD for autoreap
    ASSERT_NE(signal(SIGCHLD, SIG_IGN), SIG_ERR);
    ASSERT_EQ(socketpair(AF_UNIX, SOCK_STREAM, 0, sv), 0);
// Create a child process without namespaces initially
    child_pid = create_child(&pidfd, 0);
    ASSERT_GE(child_pid, 0);
    if (child_pid == 0) {
    close(sv[0]);
    if (unshare(CLONE_NEWUTS | CLONE_NEWIPC | CLONE_NEWNET | CLONE_NEWUSER) < 0) {
    close(sv[1]);
    _exit(1);
    }
// Signal parent that namespaces are ready
    if (write_nointr(sv[1], "1", 1) < 0) {
    close(sv[1]);
    _exit(1);
    }
    close(sv[1]);
    _exit(0);
    }
    ASSERT_GE(pidfd, 0);
    EXPECT_EQ(close(sv[1]), 0);
    ret = read_nointr(sv[0], &c, 1);
    ASSERT_EQ(ret, 1);
    EXPECT_EQ(close(sv[0]), 0);
// Set to child's namespaces via pidfd
    ret = setns(pidfd, CLONE_NEWUTS | CLONE_NEWIPC);
    TH_LOG("setns() returned %d", ret);
    close(pidfd);
    }
//
// Simple pidfd setns test using create_child().
//
// This variation uses create_child() with namespace flags directly.
// Namespaces are created immediately at clone time.
//
    TEST(simple_pidfd_setns_clone)
    {
    pid_t child_pid;
    let mut pidfd: c_int = -1;
    int ret;
// Ignore SIGCHLD for autoreap
    ASSERT_NE(signal(SIGCHLD, SIG_IGN), SIG_ERR);
// Create a child process with new namespaces using create_child()
    child_pid = create_child(&pidfd, CLONE_NEWUSER | CLONE_NEWUTS | CLONE_NEWIPC | CLONE_NEWNET);
    ASSERT_GE(child_pid, 0);
    if (child_pid == 0) {
// Child: sleep for a while so parent can setns to us
    sleep(2);
    _exit(0);
    }
// Parent: pidfd was already created by create_child()
    ASSERT_GE(pidfd, 0);
// Set to child's namespaces via pidfd
    ret = setns(pidfd, CLONE_NEWUTS | CLONE_NEWIPC);
    close(pidfd);
    TH_LOG("setns() returned %d", ret);
    }
    TEST_HARNESS_MAIN
