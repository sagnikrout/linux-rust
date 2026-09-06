//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/event_code_tests/invalid_event_code_test.c
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

// The data cache was reloaded from local core's L3 due to a demand load
pub const EventCode_1: c_uint = 0x1340000001c040;
// PM_DATA_RADIX_PROCESS_L2_PTE_FROM_L2
pub const EventCode_2: c_uint = 0x14242;
// Event code with IFM, EBB, BHRB bits set in event code
pub const EventCode_3: c_uint = 0xf00000000000001e;
//
// Some of the bits in the event code is
// reserved for specific platforms.
// Event code bits 52-59 are reserved in power9,
// whereas in ISA v3.1, these are used for programming
// Monitor Mode Control Register 3 (MMCR3).
// Bit 9 in event code is reserved in power9,
// whereas it is used for programming "radix_scope_qual"
// bit 18 in Monitor Mode Control Register 1 (MMCR1).
//
// Testcase to ensure that using reserved bits in
// event code should cause event_open to fail.
//
#[no_mangle]
unsafe extern "C" fn invalid_event_code() -> c_int {
    static int invalid_event_code(void)
    {
    struct event event;
// Check for platform support for the test
    SKIP_IF(platform_check_for_tests());
//
// Events using MMCR3 bits and radix scope qual bits
// should fail in power9 and should succeed in power10 ( ISA v3.1 )
// Init the events and check for pass/fail in event open.
//
    if (have_hwcap2(PPC_FEATURE2_ARCH_3_1)) {
    event_init(&event, EventCode_1);
    FAIL_IF(event_open(&event));
    event_close(&event);
    event_init(&event, EventCode_2);
    FAIL_IF(event_open(&event));
    event_close(&event);
    } else {
    event_init(&event, EventCode_1);
    FAIL_IF(!event_open(&event));
    event_init(&event, EventCode_2);
    FAIL_IF(!event_open(&event));
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(invalid_event_code, "invalid_event_code");
    }
