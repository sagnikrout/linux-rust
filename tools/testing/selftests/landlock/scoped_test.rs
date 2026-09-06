//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/landlock/scoped_test.c
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
// Landlock tests - Common scope restriction
//
// Copyright © 2024 Tahera Fahimi <fahimitahera@gmail.com>
//
// Macro flag: #define _GNU_SOURCE

    TEST(ruleset_with_unknown_scope)
    {
    __u64 scoped_mask;
    for (scoped_mask = 1ULL << 63; scoped_mask != ACCESS_LAST;
    scoped_mask >>= 1) {
    struct landlock_ruleset_attr ruleset_attr = {
    .scoped = scoped_mask,
    };
    ASSERT_EQ(-1, landlock_create_ruleset(&ruleset_attr,
    sizeof(ruleset_attr), 0));
    ASSERT_EQ(EINVAL, errno);
    }
    }
    TEST_HARNESS_MAIN
