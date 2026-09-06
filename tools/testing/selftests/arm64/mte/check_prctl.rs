//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/arm64/mte/check_prctl.c
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
// Copyright (C) 2022 ARM Limited

pub const AT_HWCAP3: c_int = 29;

#[no_mangle]
unsafe extern "C" fn set_tagged_addr_ctrl(val: c_int) -> c_int {
    static int set_tagged_addr_ctrl(int val)
    {
    int ret;
    ret = prctl(PR_SET_TAGGED_ADDR_CTRL, val, 0, 0, 0);
    if (ret < 0)
    ksft_print_msg("PR_SET_TAGGED_ADDR_CTRL: failed %d %d (%s)\n",
    ret, errno, strerror(errno));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn get_tagged_addr_ctrl() -> c_int {
    static int get_tagged_addr_ctrl(void)
    {
    int ret;
    ret = prctl(PR_GET_TAGGED_ADDR_CTRL, 0, 0, 0, 0);
    if (ret < 0)
    ksft_print_msg("PR_GET_TAGGED_ADDR_CTRL failed: %d %d (%s)\n",
    ret, errno, strerror(errno));
    return ret;
    }
//
// Read the current mode without having done any configuration, should
// run first.
//
#[no_mangle]
pub unsafe extern "C" fn check_basic_read() {
    void check_basic_read(void)
    {
    int ret;
    ret = get_tagged_addr_ctrl();
    if (ret < 0) {
    ksft_test_result_fail("check_basic_read\n");
    return;
    }
    if (ret & PR_MTE_TCF_SYNC)
    ksft_print_msg("SYNC enabled\n");
    if (ret & PR_MTE_TCF_ASYNC)
    ksft_print_msg("ASYNC enabled\n");
// Any configuration is valid
    ksft_test_result_pass("check_basic_read\n");
    }
//
// Attempt to set a specified combination of modes.
//
#[no_mangle]
pub unsafe extern "C" fn set_mode_test(name: *const c_char, hwcap2: c_int, hwcap3: c_int, mask: c_int) {
    void set_mode_test(const char *name, int hwcap2, int hwcap3, int mask)
    {
    int ret;
    if ((getauxval(AT_HWCAP2) & hwcap2) != hwcap2) {
    ksft_test_result_skip("%s\n", name);
    return;
    }
    if ((getauxval(AT_HWCAP3) & hwcap3) != hwcap3) {
    ksft_test_result_skip("%s\n", name);
    return;
    }
    ret = set_tagged_addr_ctrl(mask);
    if (ret < 0) {
    ksft_test_result_fail("%s\n", name);
    return;
    }
    ret = get_tagged_addr_ctrl();
    if (ret < 0) {
    ksft_test_result_fail("%s\n", name);
    return;
    }
    if ((ret & (PR_MTE_TCF_MASK | PR_MTE_STORE_ONLY)) == mask) {
    ksft_test_result_pass("%s\n", name);
    } else {
    ksft_print_msg("Got %x, expected %x\n",
    (ret & (int)PR_MTE_TCF_MASK), mask);
    ksft_test_result_fail("%s\n", name);
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mte_mode {
    pub mask: c_int,
    pub hwcap2: c_int,
    pub hwcap3: c_int,
    pub name: *const c_char,
    } mte_modes[] = {
    { PR_MTE_TCF_NONE,                                        0,          0,                     "NONE"  },
    { PR_MTE_TCF_SYNC,                                        HWCAP2_MTE, 0,                     "SYNC"  },
    { PR_MTE_TCF_ASYNC,                                       HWCAP2_MTE, 0,                     "ASYNC" },
    { PR_MTE_TCF_SYNC | PR_MTE_TCF_ASYNC,                     HWCAP2_MTE, 0,                     "SYNC+ASYNC"  },
    { PR_MTE_TCF_SYNC | PR_MTE_STORE_ONLY,                    HWCAP2_MTE, HWCAP3_MTE_STORE_ONLY, "SYNC+STONLY" },
    { PR_MTE_TCF_ASYNC | PR_MTE_STORE_ONLY,                   HWCAP2_MTE, HWCAP3_MTE_STORE_ONLY, "ASYNC+STONLY" },
    { PR_MTE_TCF_SYNC | PR_MTE_TCF_ASYNC | PR_MTE_STORE_ONLY, HWCAP2_MTE, HWCAP3_MTE_STORE_ONLY, "SYNC+ASYNC+STONLY" },
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    int i;
    ksft_print_header();
    ksft_set_plan(ARRAY_SIZE(mte_modes) + 1);
    check_basic_read();
    for (i = 0; i < ARRAY_SIZE(mte_modes); i++)
    set_mode_test(mte_modes[i].name, mte_modes[i].hwcap2, mte_modes[i].hwcap3,
    mte_modes[i].mask);
    ksft_print_cnts();
    return 0;
    }
