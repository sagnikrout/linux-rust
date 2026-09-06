//! Automatically rewritten from C to Rust
//! Source: mm/rodata_test.c
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
// rodata_test.c: functional test for mark_rodata_ro function
//
// (C) Copyright 2008 Intel Corporation
// Author: Arjan van de Ven <arjan@linux.intel.com>
//

pub const TEST_VALUE: c_uint = 0xC3;
    let mut rodata_test_data: static int = TEST_VALUE;
#[no_mangle]
pub unsafe extern "C" fn rodata_test() {
    void rodata_test(void)
    {
    let mut zero: c_int = 0;
// test 1: read the value
// If this test fails, some previous testrun has clobbered the state
    if (unlikely(READ_ONCE(rodata_test_data) != TEST_VALUE)) {
    pr_err("test 1 fails (start data)\n");
    return;
    }
// test 2: write to the variable; this should fault
    if (!copy_to_kernel_nofault((void *)&rodata_test_data,
    (void *)&zero, sizeof(zero))) {
    pr_err("test data was not read only\n");
    return;
    }
// test 3: check the value hasn't changed
    if (unlikely(READ_ONCE(rodata_test_data) != TEST_VALUE)) {
    pr_err("test data was changed\n");
    return;
    }
// test 4: check if the rodata section is PAGE_SIZE aligned
    if (!PAGE_ALIGNED(__start_rodata)) {
    pr_err("start of .rodata is not page size aligned\n");
    return;
    }
    if (!PAGE_ALIGNED(__end_rodata)) {
    pr_err("end of .rodata is not page size aligned\n");
    return;
    }
    pr_info("all tests were successful\n");
    }
