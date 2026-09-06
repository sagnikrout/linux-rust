//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/clone3/clone3_cap_checkpoint_restore.c
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
// Based on Christian Brauner's clone3() example.
// These tests are assuming to be running in the host's
// PID namespace.
//
// capabilities related code based on selftests/bpf/test_verifier.c
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
unsafe extern "C" fn child_exit(ret: c_int) {
    static void child_exit(int ret)
    {
    fflush(stdout);
    fflush(stderr);
    _exit(ret);
    }
    static int call_clone3_set_tid(struct __test_metadata *_metadata,
    pid_t *set_tid, size_t set_tid_size)
    {
    int status;
    let mut pid: pid_t = -1;
    struct __clone_args args = {
    .exit_signal = SIGCHLD,
    .set_tid = ptr_to_u64(set_tid),
    .set_tid_size = set_tid_size,
    };
    pid = sys_clone3(&args, sizeof(args));
    if (pid < 0) {
    TH_LOG("%s - Failed to create new process", strerror(errno));
    return -errno;
    }
    if (pid == 0) {
    TH_LOG("I am the child, my PID is %d (expected %d)", getpid(), set_tid[0]);
    if (set_tid[0] != getpid())
    child_exit(EXIT_FAILURE);
    child_exit(EXIT_SUCCESS);
    }
    TH_LOG("I am the parent (%d). My child's pid is %d", getpid(), pid);
    if (waitpid(pid, &status, 0) < 0) {
    TH_LOG("Child returned %s", strerror(errno));
    return -errno;
    }
    if (!WIFEXITED(status))
    return -1;
    return WEXITSTATUS(status);
    }
    static int test_clone3_set_tid(struct __test_metadata *_metadata,
    pid_t *set_tid, size_t set_tid_size)
    {
    int ret;
    TH_LOG("[%d] Trying clone3() with CLONE_SET_TID to %d", getpid(), set_tid[0]);
    ret = call_clone3_set_tid(_metadata, set_tid, set_tid_size);
    TH_LOG("[%d] clone3() with CLONE_SET_TID %d says:%d", getpid(), set_tid[0], ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn set_capability() -> c_int {
    static int set_capability(void)
    {
    cap_value_t cap_values[] = {
    CAP_SETUID, CAP_SETGID, CAP_CHECKPOINT_RESTORE
    };
    let mut ret: c_int = -1;
    cap_t caps;
    caps = cap_get_proc();
    if (!caps) {
    perror("cap_get_proc");
    return -1;
    }
// Drop all capabilities
    if (cap_clear(caps)) {
    perror("cap_clear");
    goto out;
    }
    cap_set_flag(caps, CAP_EFFECTIVE, 3, cap_values, CAP_SET);
    cap_set_flag(caps, CAP_PERMITTED, 3, cap_values, CAP_SET);
    if (cap_set_proc(caps)) {
    perror("cap_set_proc");
    goto out;
    }
    ret = 0;
    out:
    if (cap_free(caps))
    perror("cap_free");
    return ret;
    }
    TEST(clone3_cap_checkpoint_restore)
    {
    pid_t pid;
    int status;
    pid_t set_tid[1];
    test_clone3_supported();
    EXPECT_EQ(getuid(), 0)
    SKIP(return, "Skipping all tests as non-root");
    memset(&set_tid, 0, sizeof(set_tid));
// Find the current active PID
    pid = fork();
    if (pid == 0) {
    TH_LOG("Child has PID %d", getpid());
    child_exit(EXIT_SUCCESS);
    }
    ASSERT_GT(waitpid(pid, &status, 0), 0)
    TH_LOG("Waiting for child %d failed", pid);
// After the child has finished, its PID should be free.
    set_tid[0] = pid;
    ASSERT_EQ(set_capability(), 0)
    TH_LOG("Could not set CAP_CHECKPOINT_RESTORE");
    ASSERT_EQ(prctl(PR_SET_KEEPCAPS, 1, 0, 0, 0), 0);
    EXPECT_EQ(setgid(65534), 0)
    TH_LOG("Failed to setgid(65534)");
    ASSERT_EQ(setuid(65534), 0);
    set_tid[0] = pid;
// This would fail without CAP_CHECKPOINT_RESTORE
    ASSERT_EQ(test_clone3_set_tid(_metadata, set_tid, 1), -EPERM);
    ASSERT_EQ(set_capability(), 0)
    TH_LOG("Could not set CAP_CHECKPOINT_RESTORE");
// This should work as we have CAP_CHECKPOINT_RESTORE as non-root
    ASSERT_EQ(test_clone3_set_tid(_metadata, set_tid, 1), 0);
    }
    TEST_HARNESS_MAIN
