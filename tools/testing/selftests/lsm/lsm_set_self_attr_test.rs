//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/lsm/lsm_set_self_attr_test.c
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
// Linux Security Module infrastructure tests
// Tests for the lsm_set_self_attr system call
//
// Copyright © 2022 Casey Schaufler <casey@schaufler-ca.com>
//
// Macro flag: #define _GNU_SOURCE

    TEST(ctx_null_lsm_set_self_attr)
    {
    ASSERT_EQ(-1, lsm_set_self_attr(LSM_ATTR_CURRENT, core::ptr::null_mut(),
    sizeof(struct lsm_ctx), 0));
    }
    TEST(size_too_small_lsm_set_self_attr)
    {
    let mut page_size: c_long = sysconf(_SC_PAGESIZE);
    struct lsm_ctx *ctx = calloc(page_size, 1);
    let mut size: __u32 = page_size;
    ASSERT_NE(core::ptr::null_mut(), ctx);
    if (attr_lsm_count()) {
    ASSERT_LE(1, lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, &size,
    0));
    }
    ASSERT_EQ(-1, lsm_set_self_attr(LSM_ATTR_CURRENT, ctx, 1, 0));
    free(ctx);
    }
    TEST(flags_zero_lsm_set_self_attr)
    {
    let mut page_size: c_long = sysconf(_SC_PAGESIZE);
    struct lsm_ctx *ctx = calloc(page_size, 1);
    let mut size: __u32 = page_size;
    ASSERT_NE(core::ptr::null_mut(), ctx);
    if (attr_lsm_count()) {
    ASSERT_LE(1, lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, &size,
    0));
    }
    ASSERT_EQ(-1, lsm_set_self_attr(LSM_ATTR_CURRENT, ctx, size, 1));
    free(ctx);
    }
    TEST(flags_overset_lsm_set_self_attr)
    {
    let mut page_size: c_long = sysconf(_SC_PAGESIZE);
    struct lsm_ctx *ctx = calloc(page_size, 1);
    let mut size: __u32 = page_size;
    ASSERT_NE(core::ptr::null_mut(), ctx);
    if (attr_lsm_count()) {
    ASSERT_LE(1, lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, &size,
    0));
    }
    ASSERT_EQ(-1, lsm_set_self_attr(LSM_ATTR_CURRENT | LSM_ATTR_PREV, ctx,
    size, 0));
    free(ctx);
    }
    TEST_HARNESS_MAIN
