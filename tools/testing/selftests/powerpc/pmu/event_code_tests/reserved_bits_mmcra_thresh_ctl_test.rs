//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/event_code_tests/reserved_bits_mmcra_thresh_ctl_test.c
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
// Copyright 2022, Athira Rajeev, IBM Corp.
//

//
// Testcase for reserved bits in Monitor Mode
// Control Register A (MMCRA) thresh_ctl bits.
// For MMCRA[48:51]/[52:55]) Threshold Start/Stop,
// 0b11110000/0b00001111 is reserved.
//
#[no_mangle]
unsafe extern "C" fn reserved_bits_mmcra_thresh_ctl() -> c_int {
    static int reserved_bits_mmcra_thresh_ctl(void)
    {
    struct event event;
// Check for platform support for the test
    SKIP_IF(platform_check_for_tests());
// Skip for Generic compat PMU
    SKIP_IF(check_for_generic_compat_pmu());
//
// MMCRA[48:51]/[52:55]) Threshold Start/Stop
// events Selection. 0b11110000/0b00001111 is reserved.
// Expected to fail when using these reserved values.
//
    event_init(&event, 0xf0340401e0);
    FAIL_IF(!event_open(&event));
    event_init(&event, 0x0f340401e0);
    FAIL_IF(!event_open(&event));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(reserved_bits_mmcra_thresh_ctl, "reserved_bits_mmcra_thresh_ctl");
    }
