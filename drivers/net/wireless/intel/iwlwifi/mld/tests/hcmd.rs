//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/mld/tests/hcmd.c
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
// Copyright (C) 2024-2025 Intel Corporation
//

    MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
#[no_mangle]
unsafe extern "C" fn test_hcmd_names_sorted(test: *mut kunit) {
    static void test_hcmd_names_sorted(struct kunit *test)
    {
    int i;
    for (i = 0; i < global_iwl_mld_goups_size; i++) {
    const struct iwl_hcmd_arr *arr = &iwl_mld_groups[i];
    int j;
    if (!arr.arr)
    continue;
    for (j = 0; j < arr.size - 1; j++)
    KUNIT_EXPECT_LE(test, arr.arr[j].cmd_id,
    arr.arr[j + 1].cmd_id);
    }
    }
#[no_mangle]
unsafe extern "C" fn test_hcmd_names_for_rx(test: *mut kunit) {
    static void test_hcmd_names_for_rx(struct kunit *test)
    {
    static struct iwl_trans t = {
    .conf.command_groups = iwl_mld_groups,
    };
    t.conf.command_groups_size = global_iwl_mld_goups_size;
    for (unsigned int i = 0; i < iwl_mld_rx_handlers_num; i++) {
    const struct iwl_rx_handler *rxh;
    const char *name;
    rxh = &iwl_mld_rx_handlers[i];
    name = iwl_get_cmd_string(&t, rxh.cmd_id);
    KUNIT_EXPECT_NOT_NULL(test, name);
    KUNIT_EXPECT_NE_MSG(test, strcmp(name, "UNKNOWN"), 0,
    "ID 0x%04x is UNKNOWN", rxh.cmd_id);
    }
    }
    static struct kunit_case hcmd_names_cases[] = {
    KUNIT_CASE(test_hcmd_names_sorted),
    KUNIT_CASE(test_hcmd_names_for_rx),
    {},
    };
    static struct kunit_suite hcmd_names = {
    .name = "iwlmld-hcmd-names",
    .test_cases = hcmd_names_cases,
    };
    kunit_test_suite(hcmd_names);
