//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/event_code_tests/event_alternatives_tests_p9.c
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

pub const PM_RUN_CYC_ALT: c_uint = 0x200f4;
pub const PM_INST_DISP: c_uint = 0x200f2;
pub const PM_BR_2PATH: c_uint = 0x20036;
pub const PM_LD_MISS_L1: c_uint = 0x3e054;
pub const PM_RUN_INST_CMPL_ALT: c_uint = 0x400fa;
pub const EventCode_1: c_uint = 0x200fa;
pub const EventCode_2: c_uint = 0x200fc;
pub const EventCode_3: c_uint = 0x300fc;
pub const EventCode_4: c_uint = 0x400fc;
//
// Check for event alternatives.
//
#[no_mangle]
unsafe extern "C" fn event_alternatives_tests_p9() -> c_int {
    static int event_alternatives_tests_p9(void)
    {
    struct event event, leader;
// Check for platform support for the test
    SKIP_IF(platform_check_for_tests());
//
// PVR check is used here since PMU specific data like
// alternative events is handled by respective PMU driver
// code and using PVR will work correctly for all cases
// including generic compat mode.
//
    SKIP_IF(PVR_VER(mfspr(SPRN_PVR)) != POWER9);
// Skip for generic compat PMU
    SKIP_IF(check_for_generic_compat_pmu());
// Init the event for PM_RUN_CYC_ALT
    event_init(&leader, PM_RUN_CYC_ALT);
    FAIL_IF(event_open(&leader));
    event_init(&event, EventCode_1);
//
// Expected to pass since PM_RUN_CYC_ALT in PMC2 has alternative event
// 0x600f4. So it can go in with EventCode_1 which is using PMC2
//
    FAIL_IF(event_open_with_group(&event, leader.fd));
    event_close(&leader);
    event_close(&event);
    event_init(&leader, PM_INST_DISP);
    FAIL_IF(event_open(&leader));
    event_init(&event, EventCode_2);
//
// Expected to pass since PM_INST_DISP in PMC2 has alternative event
// 0x300f2 in PMC3. So it can go in with EventCode_2 which is using PMC2
//
    FAIL_IF(event_open_with_group(&event, leader.fd));
    event_close(&leader);
    event_close(&event);
    event_init(&leader, PM_BR_2PATH);
    FAIL_IF(event_open(&leader));
    event_init(&event, EventCode_2);
//
// Expected to pass since PM_BR_2PATH in PMC2 has alternative event
// 0x40036 in PMC4. So it can go in with EventCode_2 which is using PMC2
//
    FAIL_IF(event_open_with_group(&event, leader.fd));
    event_close(&leader);
    event_close(&event);
    event_init(&leader, PM_LD_MISS_L1);
    FAIL_IF(event_open(&leader));
    event_init(&event, EventCode_3);
//
// Expected to pass since PM_LD_MISS_L1 in PMC3 has alternative event
// 0x400f0 in PMC4. So it can go in with EventCode_3 which is using PMC3
//
    FAIL_IF(event_open_with_group(&event, leader.fd));
    event_close(&leader);
    event_close(&event);
    event_init(&leader, PM_RUN_INST_CMPL_ALT);
    FAIL_IF(event_open(&leader));
    event_init(&event, EventCode_4);
//
// Expected to pass since PM_RUN_INST_CMPL_ALT in PMC4 has alternative event
// 0x500fa in PMC5. So it can go in with EventCode_4 which is using PMC4
//
    FAIL_IF(event_open_with_group(&event, leader.fd));
    event_close(&leader);
    event_close(&event);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(event_alternatives_tests_p9, "event_alternatives_tests_p9");
    }
