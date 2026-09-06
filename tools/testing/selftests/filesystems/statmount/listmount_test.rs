//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/statmount/listmount_test.c
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
// Copyright (c) 2024 Christian Brauner <brauner@kernel.org>
// Macro flag: #define _GNU_SOURCE

pub const LISTMNT_BUFFER: c_int = 10;
// Check that all mount ids are in increasing order.
    TEST(listmount_forward)
    {
    uint64_t list[LISTMNT_BUFFER], last_mnt_id = 0;
    for (;;) {
    ssize_t nr_mounts;
    nr_mounts = listmount(LSMT_ROOT, 0, last_mnt_id,
    list, LISTMNT_BUFFER, 0);
    ASSERT_GE(nr_mounts, 0);
    if (nr_mounts == 0)
    break;
    for (size_t cur = 0; cur < nr_mounts; cur++) {
    if (cur < nr_mounts - 1)
    ASSERT_LT(list[cur], list[cur + 1]);
    last_mnt_id = list[cur];
    }
    }
    }
// Check that all mount ids are in decreasing order.
    TEST(listmount_backward)
    {
    uint64_t list[LISTMNT_BUFFER], last_mnt_id = 0;
    for (;;) {
    ssize_t nr_mounts;
    nr_mounts = listmount(LSMT_ROOT, 0, last_mnt_id,
    list, LISTMNT_BUFFER, LISTMOUNT_REVERSE);
    ASSERT_GE(nr_mounts, 0);
    if (nr_mounts == 0)
    break;
    for (size_t cur = 0; cur < nr_mounts; cur++) {
    if (cur < nr_mounts - 1)
    ASSERT_GT(list[cur], list[cur + 1]);
    last_mnt_id = list[cur];
    }
    }
    }
    TEST_HARNESS_MAIN
