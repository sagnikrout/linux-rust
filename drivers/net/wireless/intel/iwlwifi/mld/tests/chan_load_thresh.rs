//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/mld/tests/chan_load_thresh.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// KUnit tests for channel helper functions
//
// Copyright (C) 2026 Intel Corporation
//

    MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_chan_load_case {
    pub desc: *const c_char,
    pub load: u32,
    pub old_lvl: enum iwl_mld_link_chan_load_level,
    pub expected_lvl: enum iwl_mld_link_chan_load_level,
    pub expected_scan_trig: bool,
}

    static const struct test_chan_load_case test_chan_load_thresh_cases[] = {
// Level-up transitions
    {
    .desc = "Transition NONE.NONE",
    .load = 20,
    .old_lvl = LINK_CHAN_LOAD_LVL_NONE,
    .expected_lvl = LINK_CHAN_LOAD_LVL_NONE,
    .expected_scan_trig = false,
    },
    {
    .desc = "Transition NONE.LVL1",
    .load = 50,
    .old_lvl = LINK_CHAN_LOAD_LVL_NONE,
    .expected_lvl = LINK_CHAN_LOAD_LVL1,
    .expected_scan_trig = true,
    },
    {
    .desc = "Transition LVL1.LVL2",
    .load = 75,
    .old_lvl = LINK_CHAN_LOAD_LVL1,
    .expected_lvl = LINK_CHAN_LOAD_LVL2,
    .expected_scan_trig = true,
    },
    {
    .desc = "Transition LVL2.LVL3",
    .load = 90,
    .old_lvl = LINK_CHAN_LOAD_LVL2,
    .expected_lvl = LINK_CHAN_LOAD_LVL3,
    .expected_scan_trig = true,
    },
// Level-down transitions
    {
    .desc = "Transition LVL1.NONE",
    .load = 30,
    .old_lvl = LINK_CHAN_LOAD_LVL1,
    .expected_lvl = LINK_CHAN_LOAD_LVL_NONE,
    .expected_scan_trig = false,
    },
    {
    .desc = "Transition LVL2.LVL1",
    .load = 60,
    .old_lvl = LINK_CHAN_LOAD_LVL2,
    .expected_lvl = LINK_CHAN_LOAD_LVL1,
    .expected_scan_trig = false,
    },
    {
    .desc = "Transition LVL3.LVL2",
    .load = 70,
    .old_lvl = LINK_CHAN_LOAD_LVL3,
    .expected_lvl = LINK_CHAN_LOAD_LVL2,
    .expected_scan_trig = false,
    },
// No change
    {
    .desc = "Transition LVL2.LVL2",
    .load = 72,
    .old_lvl = LINK_CHAN_LOAD_LVL2,
    .expected_lvl = LINK_CHAN_LOAD_LVL2,
    .expected_scan_trig = false,
    },
    };
    KUNIT_ARRAY_PARAM_DESC(test_chan_load_thresh_cases,
    test_chan_load_thresh_cases, desc);
#[no_mangle]
unsafe extern "C" fn test_chan_load_thresholds(test: *mut kunit) {
    static void test_chan_load_thresholds(struct kunit *test)
    {
    const struct test_chan_load_case *tc = test.param_value;
    struct iwl_mld *mld = test.priv;
    struct ieee80211_vif *vif;
    struct iwl_mld_vif *mld_vif;
    struct ieee80211_bss_conf *link_conf;
    struct iwl_mld_link *mld_link;
    struct iwl_mld_kunit_link assoc_link = {
    .id = 0,
    .chandef = &chandef_6ghz_160mhz,
    };
    bool scan_trig;
    u32 chan_load;
// Setup associated non-MLO station
    vif = iwlmld_kunit_setup_non_mlo_assoc(&assoc_link);
    mld_vif = iwl_mld_vif_from_mac80211(vif);
    link_conf = &vif.bss_conf;
    mld_link = &mld_vif.deflink;
    chan_load = tc.load;
    mld_link.chan_load_lvl = tc.old_lvl;
// Execute function under test
    wiphy_lock(mld.wiphy);
    scan_trig = iwl_mld_chan_load_requires_scan(mld, link_conf, chan_load);
    wiphy_unlock(mld.wiphy);
// Check return value
    KUNIT_EXPECT_EQ(test, tc.expected_scan_trig, scan_trig);
// Check updated channel-load level
    KUNIT_EXPECT_EQ(test, tc.expected_lvl, mld_link.chan_load_lvl);
    }
    static struct kunit_case chan_load_thresh_test_cases[] = {
    KUNIT_CASE_PARAM(test_chan_load_thresholds,
    test_chan_load_thresh_cases_gen_params),
    {}
    };
    static struct kunit_suite chan_load_thresh_test_suite = {
    .name = "iwl_mld_chan_load_threshold_tests",
    .init = iwlmld_kunit_test_init,
    .test_cases = chan_load_thresh_test_cases,
    };
    kunit_test_suite(chan_load_thresh_test_suite);
