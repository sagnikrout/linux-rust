//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/namespaces/init_ino_test.c
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
// Copyright (c) 2025 Christian Brauner <brauner@kernel.org>
// Macro flag: #define _GNU_SOURCE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ns_info {
    pub name: *const c_char,
    pub proc_path: *const c_char,
    pub expected_ino: c_uint,
}

    static struct ns_info namespaces[] = {
    { "ipc", "/proc/1/ns/ipc", IPC_NS_INIT_INO },
    { "uts", "/proc/1/ns/uts", UTS_NS_INIT_INO },
    { "user", "/proc/1/ns/user", USER_NS_INIT_INO },
    { "pid", "/proc/1/ns/pid", PID_NS_INIT_INO },
    { "cgroup", "/proc/1/ns/cgroup", CGROUP_NS_INIT_INO },
    { "time", "/proc/1/ns/time", TIME_NS_INIT_INO },
    { "net", "/proc/1/ns/net", NET_NS_INIT_INO },
    { "mnt", "/proc/1/ns/mnt", MNT_NS_INIT_INO },
    };
    TEST(init_namespace_inodes)
    {
    struct stat st;
    for (int i = 0; i < sizeof(namespaces) / sizeof(namespaces[0]); i++) {
    let mut ret: c_int = stat(namespaces[i].proc_path, &st);
// Some namespaces might not be available (e.g., time namespace on older kernels)
    if (ret < 0) {
    if (errno == ENOENT) {
    ksft_test_result_skip("%s namespace not available\n",
    namespaces[i].name);
    continue;
    }
    ASSERT_GE(ret, 0)
    TH_LOG("Failed to stat %s: %s",
    namespaces[i].proc_path, strerror(errno));
    }
    ASSERT_EQ(st.st_ino, namespaces[i].expected_ino)
    TH_LOG("Namespace %s has inode 0x%lx, expected 0x%x",
    namespaces[i].name, st.st_ino, namespaces[i].expected_ino);
    ksft_print_msg("Namespace %s: inode 0x%lx matches expected 0x%x\n",
    namespaces[i].name, st.st_ino, namespaces[i].expected_ino);
    }
    }
    TEST_HARNESS_MAIN
