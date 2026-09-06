//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/sdhci-of-aspeed-test.c
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
// Copyright (C) 2020 IBM Corp.

#[no_mangle]
unsafe extern "C" fn aspeed_sdhci_phase_ddr52(test: *mut kunit) {
    static void aspeed_sdhci_phase_ddr52(struct kunit *test)
    {
    let mut rate: c_int = 52000000;
    KUNIT_EXPECT_EQ(test, 0,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 0));
    KUNIT_EXPECT_EQ(test, 0,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 1));
    KUNIT_EXPECT_EQ(test, 1,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 2));
    KUNIT_EXPECT_EQ(test, 1,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 3));
    KUNIT_EXPECT_EQ(test, 2,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 4));
    KUNIT_EXPECT_EQ(test, 3,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 5));
    KUNIT_EXPECT_EQ(test, 14,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 23));
    KUNIT_EXPECT_EQ(test, 15,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 24));
    KUNIT_EXPECT_EQ(test, 15,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 25));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK | 0,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 180));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK | 0,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 181));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK | 1,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 182));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK | 1,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 183));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK | 2,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 184));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK | 3,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 185));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK | 14,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 203));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK | 15,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 204));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK | 15,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 205));
    }
#[no_mangle]
unsafe extern "C" fn aspeed_sdhci_phase_hs200(test: *mut kunit) {
    static void aspeed_sdhci_phase_hs200(struct kunit *test)
    {
    let mut rate: c_int = 200000000;
    KUNIT_EXPECT_EQ(test, 0,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 0));
    KUNIT_EXPECT_EQ(test, 0,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 5));
    KUNIT_EXPECT_EQ(test, 1,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 6));
    KUNIT_EXPECT_EQ(test, 1,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 7));
    KUNIT_EXPECT_EQ(test, 14,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 89));
    KUNIT_EXPECT_EQ(test, 15,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 90));
    KUNIT_EXPECT_EQ(test, 15,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 91));
    KUNIT_EXPECT_EQ(test, 15,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 96));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 180));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 185));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK | 1,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 186));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK | 1,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 187));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK | 14,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 269));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK | 15,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 270));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK | 15,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 271));
    KUNIT_EXPECT_EQ(test, ASPEED_SDHCI_TAP_PARAM_INVERT_CLK | 15,
    aspeed_sdhci_phase_to_tap(core::ptr::null_mut(), rate, 276));
    }
    static struct kunit_case aspeed_sdhci_test_cases[] = {
    KUNIT_CASE(aspeed_sdhci_phase_ddr52),
    KUNIT_CASE(aspeed_sdhci_phase_hs200),
    {}
    };
    static struct kunit_suite aspeed_sdhci_test_suite = {
    .name = "sdhci-of-aspeed",
    .test_cases = aspeed_sdhci_test_cases,
    };
    kunit_test_suite(aspeed_sdhci_test_suite);
