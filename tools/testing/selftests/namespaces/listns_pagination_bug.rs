//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/namespaces/listns_pagination_bug.c
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
// Minimal test case to reproduce KASAN out-of-bounds in listns pagination.
//
// The bug occurs when:
// 1. Filtering by a specific namespace type (e.g., CLONE_NEWUSER)
// 2. Using pagination (req.ns_id != 0)
// 3. The lookup_ns_id_at() call in do_listns() passes ns_type=0 instead of
// the filtered type, causing it to search the unified tree and potentially
// return a namespace of the wrong type.
//
    TEST(pagination_with_type_filter)
    {
    struct ns_id_req req = {
    .size = sizeof(req),
    .spare = 0,
    .ns_id = 0,
    .ns_type = CLONE_NEWUSER,  /* Filter by user namespace */
    .spare2 = 0,
    .user_ns_id = 0,
    };
    pid_t pids[10];
    let mut num_children: c_int = 10;
    int i;
    int sv[2];
    __u64 first_batch[3];
    ssize_t ret;
    ASSERT_EQ(socketpair(AF_UNIX, SOCK_STREAM, 0, sv), 0);
// Create children with user namespaces
    for (i = 0; i < num_children; i++) {
    pids[i] = fork();
    ASSERT_GE(pids[i], 0);
    if (pids[i] == 0) {
    char c;
    close(sv[0]);
    if (setup_userns() < 0) {
    close(sv[1]);
    exit(1);
    }
// Signal parent we're ready
    if (write(sv[1], &c, 1) != 1) {
    close(sv[1]);
    exit(1);
    }
// Wait for parent signal to exit
    if (read(sv[1], &c, 1) != 1) {
    close(sv[1]);
    exit(1);
    }
    close(sv[1]);
    exit(0);
    }
    }
    close(sv[1]);
// Wait for all children to signal ready
    for (i = 0; i < num_children; i++) {
    char c;
    if (read(sv[0], &c, 1) != 1) {
    close(sv[0]);
    for (int j = 0; j < num_children; j++)
    kill(pids[j], SIGKILL);
    for (int j = 0; j < num_children; j++)
    waitpid(pids[j], core::ptr::null_mut(), 0);
    ASSERT_TRUE(false);
    }
    }
// First batch - this should work
    ret = sys_listns(&req, first_batch, 3, 0);
    if (ret < 0) {
    if (errno == ENOSYS) {
    close(sv[0]);
    for (i = 0; i < num_children; i++)
    kill(pids[i], SIGKILL);
    for (i = 0; i < num_children; i++)
    waitpid(pids[i], core::ptr::null_mut(), 0);
    SKIP(return, "listns() not supported");
    }
    ASSERT_GE(ret, 0);
    }
    TH_LOG("First batch returned %zd entries", ret);
    if (ret == 3) {
    __u64 second_batch[3];
// Second batch - pagination triggers the bug
    req.ns_id = first_batch[2];  /* Continue from last ID */
    ret = sys_listns(&req, second_batch, 3, 0);
    TH_LOG("Second batch returned %zd entries", ret);
    ASSERT_GE(ret, 0);
    }
// Signal all children to exit
    for (i = 0; i < num_children; i++) {
    let mut c: c_char = 'X';
    if (write(sv[0], &c, 1) != 1) {
    close(sv[0]);
    for (int j = i; j < num_children; j++)
    kill(pids[j], SIGKILL);
    for (int j = 0; j < num_children; j++)
    waitpid(pids[j], core::ptr::null_mut(), 0);
    ASSERT_TRUE(false);
    }
    }
    close(sv[0]);
// Cleanup
    for (i = 0; i < num_children; i++) {
    int status;
    waitpid(pids[i], &status, 0);
    }
    }
    TEST_HARNESS_MAIN
