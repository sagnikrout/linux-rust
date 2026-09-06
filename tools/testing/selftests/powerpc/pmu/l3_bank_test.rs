//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/l3_bank_test.c
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
// Copyright 2014, Michael Ellerman, IBM Corp.
//

//
// Tests that the L3 bank handling is correct. We fixed it in commit e9aaac1.
//
#[no_mangle]
unsafe extern "C" fn l3_bank_test() -> c_int {
    static int l3_bank_test(void)
    {
    struct event event;
    char *p;
    int i;
// The L3 bank logic is only used on Power8 or later
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_ARCH_2_07));
    p = malloc(MALLOC_SIZE);
    FAIL_IF(!p);
    event_init(&event, 0x84918F);
    FAIL_IF(event_open(&event));
    for (i = 0; i < MALLOC_SIZE; i += 0x10000)
    p[i] = i;
    event_read(&event);
    event_report(&event);
    FAIL_IF(event.result.running == 0);
    FAIL_IF(event.result.enabled == 0);
    event_close(&event);
    free(p);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(l3_bank_test, "l3_bank_test");
    }
