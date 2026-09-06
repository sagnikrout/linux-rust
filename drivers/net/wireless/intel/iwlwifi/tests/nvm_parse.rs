//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/tests/nvm_parse.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// KUnit tests for NVM parse
//
// Copyright (C) 2025 Intel Corporation
//

    MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
    static const struct nvm_flag_case {
    const char *desc;
    u16 nvm_flags;
    u32 reg_rule_flags;
    u32 set_reg_rule_flags;
    u32 clear_reg_rule_flags;
    } nvm_flag_cases[] = {
    {
    .desc = "Restricting VLP client and AP access",
    .nvm_flags = 0,
    .set_reg_rule_flags = NL80211_RRF_NO_6GHZ_VLP_CLIENT,
    .clear_reg_rule_flags = NL80211_RRF_ALLOW_6GHZ_VLP_AP,
    },
    {
    .desc = "Allow VLP client and AP access",
    .nvm_flags = NVM_CHANNEL_VLP,
    .set_reg_rule_flags = NL80211_RRF_ALLOW_6GHZ_VLP_AP,
    .clear_reg_rule_flags = NL80211_RRF_NO_6GHZ_VLP_CLIENT,
    },
    {
    .desc = "Allow VLP client access, while restricting AP access",
    .nvm_flags = NVM_CHANNEL_VLP | NVM_CHANNEL_VLP_AP_NOT_ALLOWED,
    .set_reg_rule_flags = 0,
    .clear_reg_rule_flags = NL80211_RRF_ALLOW_6GHZ_VLP_AP |
    NL80211_RRF_NO_6GHZ_VLP_CLIENT,
    },
    };
    KUNIT_ARRAY_PARAM_DESC(nvm_flag, nvm_flag_cases, desc)
#[no_mangle]
unsafe extern "C" fn test_nvm_flags(test: *mut kunit) {
    static void test_nvm_flags(struct kunit *test)
    {
    const struct nvm_flag_case *params = test.param_value;
    let mut reg_capa: iwl_reg_capa = {};
    let mut flags: u32 = 0;
    flags = iwl_nvm_get_regdom_bw_flags(core::ptr::null_mut(), 0, params.nvm_flags,
    reg_capa);
    if ((params.set_reg_rule_flags & flags) != params.set_reg_rule_flags)
    KUNIT_FAIL(test, "Expected set bits:0x%08x flags:0x%08x\n",
    params.set_reg_rule_flags, flags);
    if (params.clear_reg_rule_flags & flags)
    KUNIT_FAIL(test, "Expected clear bits:0x%08x flags:0x%08x\n",
    params.clear_reg_rule_flags, flags);
    }
    static struct kunit_case nvm_flags_test_cases[] = {
    KUNIT_CASE_PARAM(test_nvm_flags,
    nvm_flag_gen_params),
    {},
    };
    static struct kunit_suite nvm_flags_suite = {
    .name = "iwlwifi-nvm_flags",
    .test_cases = nvm_flags_test_cases,
    };
    kunit_test_suite(nvm_flags_suite);
