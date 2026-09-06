//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/event_code_tests/reserved_bits_mmcra_sample_elig_mode_test.c
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
// Testcase for reserved bits in Monitor Mode Control
// Register A (MMCRA) Random Sampling Mode (SM) value.
// As per Instruction Set Architecture (ISA), the values
// 0x5, 0x9, 0xD, 0x19, 0x1D, 0x1A, 0x1E are reserved
// for sampling mode field. Test that having these reserved
// bit values should cause event_open to fail.
// Input event code uses these sampling bits along with
// 401e0 (PM_MRK_INST_CMPL).
//
#[no_mangle]
unsafe extern "C" fn reserved_bits_mmcra_sample_elig_mode() -> c_int {
    static int reserved_bits_mmcra_sample_elig_mode(void)
    {
    struct event event;
    let mut pvr: c_int = PVR_VER(mfspr(SPRN_PVR));
// Check for platform support for the test
    SKIP_IF(platform_check_for_tests());
// Skip for Generic compat PMU
    SKIP_IF(check_for_generic_compat_pmu());
//
// MMCRA Random Sampling Mode (SM) values: 0x5
// 0x9, 0xD, 0x19, 0x1D, 0x1A, 0x1E is reserved.
// Expected to fail when using these reserved values.
//
    event_init(&event, 0x50401e0);
    FAIL_IF(!event_open(&event));
    event_init(&event, 0x90401e0);
    FAIL_IF(!event_open(&event));
    event_init(&event, 0xD0401e0);
    FAIL_IF(!event_open(&event));
    event_init(&event, 0x190401e0);
    FAIL_IF(!event_open(&event));
    event_init(&event, 0x1D0401e0);
    FAIL_IF(!event_open(&event));
    event_init(&event, 0x1A0401e0);
    FAIL_IF(!event_open(&event));
    event_init(&event, 0x1E0401e0);
    FAIL_IF(!event_open(&event));
//
// MMCRA Random Sampling Mode (SM) value 0x10
// is reserved in power10/power11 and 0xC is reserved in
// power9.
//
    if ((pvr == POWER10) || (pvr == POWER11)) {
    event_init(&event, 0x100401e0);
    FAIL_IF(!event_open(&event));
    } else if (PVR_VER(mfspr(SPRN_PVR)) == POWER9) {
    event_init(&event, 0xC0401e0);
    FAIL_IF(!event_open(&event));
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(reserved_bits_mmcra_sample_elig_mode,
    "reserved_bits_mmcra_sample_elig_mode");
    }
