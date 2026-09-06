//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/xe/tests/xe_lmtt_test.c
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


// SPDX-License-Identifier: GPL-2.0 AND MIT
//
// Copyright © 2023 Intel Corporation
//

    static const struct lmtt_ops_param {
    const char *desc;
    const struct xe_lmtt_ops *ops;
    } lmtt_ops_params[] = {
    { "2-level", &lmtt_2l_ops, },
    { "multi-level", &lmtt_ml_ops, },
    };
#[no_mangle]
unsafe extern "C" fn lmtt_ops_param_get_desc(p: *const lmtt_ops_param, desc: *mut c_char) {
    static void lmtt_ops_param_get_desc(const struct lmtt_ops_param *p, char *desc)
    {
    snprintf(desc, KUNIT_PARAM_DESC_SIZE, "%s", p.desc);
    }
    KUNIT_ARRAY_PARAM(lmtt_ops, lmtt_ops_params, lmtt_ops_param_get_desc);
#[no_mangle]
unsafe extern "C" fn test_ops(test: *mut kunit) {
    static void test_ops(struct kunit *test)
    {
    const struct lmtt_ops_param *p = test.param_value;
    const struct xe_lmtt_ops *ops = p.ops;
    unsigned int n;
    KUNIT_ASSERT_NOT_NULL(test, ops.lmtt_root_pd_level);
    KUNIT_ASSERT_NOT_NULL(test, ops.lmtt_pte_num);
    KUNIT_ASSERT_NOT_NULL(test, ops.lmtt_pte_size);
    KUNIT_ASSERT_NOT_NULL(test, ops.lmtt_pte_shift);
    KUNIT_ASSERT_NOT_NULL(test, ops.lmtt_pte_index);
    KUNIT_ASSERT_NOT_NULL(test, ops.lmtt_pte_encode);
    KUNIT_EXPECT_NE(test, ops.lmtt_root_pd_level(), 0);
    for (n = 0; n <= ops.lmtt_root_pd_level(); n++) {
    KUNIT_EXPECT_NE_MSG(test, ops.lmtt_pte_num(n), 0,
    "level=%u", n);
    KUNIT_EXPECT_NE_MSG(test, ops.lmtt_pte_size(n), 0,
    "level=%u", n);
    KUNIT_EXPECT_NE_MSG(test, ops.lmtt_pte_encode(0, n), LMTT_PTE_INVALID,
    "level=%u", n);
    }
    for (n = 0; n < ops.lmtt_root_pd_level(); n++) {
    let mut addr: u64 = BIT_ULL(ops.lmtt_pte_shift(n));
    KUNIT_EXPECT_NE_MSG(test, ops.lmtt_pte_shift(n), 0,
    "level=%u", n);
    KUNIT_EXPECT_EQ_MSG(test, ops.lmtt_pte_index(addr - 1, n), 0,
    "addr=%#llx level=%u", addr, n);
    KUNIT_EXPECT_EQ_MSG(test, ops.lmtt_pte_index(addr + 1, n), 1,
    "addr=%#llx level=%u", addr, n);
    KUNIT_EXPECT_EQ_MSG(test, ops.lmtt_pte_index(addr * 2 - 1, n), 1,
    "addr=%#llx level=%u", addr, n);
    KUNIT_EXPECT_EQ_MSG(test, ops.lmtt_pte_index(addr * 2, n), 2,
    "addr=%#llx level=%u", addr, n);
    }
    }
    static struct kunit_case lmtt_test_cases[] = {
    KUNIT_CASE_PARAM(test_ops, lmtt_ops_gen_params),
    {}
    };
    static struct kunit_suite lmtt_suite = {
    .name = "lmtt",
    .test_cases = lmtt_test_cases,
    };
    kunit_test_suites(&lmtt_suite);
