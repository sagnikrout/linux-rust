//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/event_code_tests/blacklisted_events_test.c
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

pub const PM_DTLB_MISS_16G: c_uint = 0x1c058;
pub const PM_DERAT_MISS_2M: c_uint = 0x1c05a;
pub const PM_DTLB_MISS_2M: c_uint = 0x1c05c;
pub const PM_MRK_DTLB_MISS_1G: c_uint = 0x1d15c;
pub const PM_DTLB_MISS_4K: c_uint = 0x2c056;
pub const PM_DERAT_MISS_1G: c_uint = 0x2c05a;
pub const PM_MRK_DERAT_MISS_2M: c_uint = 0x2d152;
pub const PM_MRK_DTLB_MISS_4K: c_uint = 0x2d156;
pub const PM_MRK_DTLB_MISS_16G: c_uint = 0x2d15e;
pub const PM_DTLB_MISS_64K: c_uint = 0x3c056;
pub const PM_MRK_DERAT_MISS_1G: c_uint = 0x3d152;
pub const PM_MRK_DTLB_MISS_64K: c_uint = 0x3d156;
pub const PM_DISP_HELD_SYNC_HOLD: c_uint = 0x4003c;
pub const PM_DTLB_MISS_16M: c_uint = 0x4c056;
pub const PM_DTLB_MISS_1G: c_uint = 0x4c05a;
pub const PM_MRK_DTLB_MISS_16M: c_uint = 0x4c15e;
pub const PM_MRK_ST_DONE_L2: c_uint = 0x10134;
pub const PM_RADIX_PWC_L1_HIT: c_uint = 0x1f056;
pub const PM_FLOP_CMPL: c_uint = 0x100f4;
pub const PM_MRK_NTF_FIN: c_uint = 0x20112;
pub const PM_RADIX_PWC_L2_HIT: c_uint = 0x2d024;
pub const PM_IFETCH_THROTTLE: c_uint = 0x3405e;
pub const PM_MRK_L2_TM_ST_ABORT_SISTER: c_uint = 0x3e15c;
pub const PM_RADIX_PWC_L3_HIT: c_uint = 0x3f056;
pub const PM_RUN_CYC_SMT2_MODE: c_uint = 0x3006c;
pub const PM_TM_TX_PASS_RUN_INST: c_uint = 0x4e014;
pub const PVR_POWER9_CUMULUS: c_uint = 0x00002000;
    int blacklist_events_dd21[] = {
    PM_MRK_ST_DONE_L2,
    PM_RADIX_PWC_L1_HIT,
    PM_FLOP_CMPL,
    PM_MRK_NTF_FIN,
    PM_RADIX_PWC_L2_HIT,
    PM_IFETCH_THROTTLE,
    PM_MRK_L2_TM_ST_ABORT_SISTER,
    PM_RADIX_PWC_L3_HIT,
    PM_RUN_CYC_SMT2_MODE,
    PM_TM_TX_PASS_RUN_INST,
    PM_DISP_HELD_SYNC_HOLD,
    };
    int blacklist_events_dd22[] = {
    PM_DTLB_MISS_16G,
    PM_DERAT_MISS_2M,
    PM_DTLB_MISS_2M,
    PM_MRK_DTLB_MISS_1G,
    PM_DTLB_MISS_4K,
    PM_DERAT_MISS_1G,
    PM_MRK_DERAT_MISS_2M,
    PM_MRK_DTLB_MISS_4K,
    PM_MRK_DTLB_MISS_16G,
    PM_DTLB_MISS_64K,
    PM_MRK_DERAT_MISS_1G,
    PM_MRK_DTLB_MISS_64K,
    PM_DISP_HELD_SYNC_HOLD,
    PM_DTLB_MISS_16M,
    PM_DTLB_MISS_1G,
    PM_MRK_DTLB_MISS_16M,
    };
    int pvr_min;
//
// check for power9 support for 2.1 and
// 2.2 model where blacklist is applicable.
//
#[no_mangle]
pub unsafe extern "C" fn check_for_power9_version() -> c_int {
    int check_for_power9_version(void)
    {
    pvr_min = PVR_MIN(mfspr(SPRN_PVR));
    SKIP_IF(PVR_VER(pvr) != POWER9);
    SKIP_IF(!(pvr & PVR_POWER9_CUMULUS));
    SKIP_IF(!(3 - pvr_min));
    return 0;
    }
//
// Testcase to ensure that using blacklisted bits in
// event code should cause event_open to fail in power9
//
#[no_mangle]
unsafe extern "C" fn blacklisted_events() -> c_int {
    static int blacklisted_events(void)
    {
    struct event event;
    let mut i: c_int = 0;
// Check for platform support for the test
    SKIP_IF(platform_check_for_tests());
//
// check for power9 support for 2.1 and
// 2.2 model where blacklist is applicable.
//
    SKIP_IF(check_for_power9_version());
// Skip for Generic compat mode
    SKIP_IF(check_for_generic_compat_pmu());
    if (pvr_min == 1) {
    for (i = 0; i < ARRAY_SIZE(blacklist_events_dd21); i++) {
    event_init(&event, blacklist_events_dd21[i]);
    FAIL_IF(!event_open(&event));
    }
    } else if (pvr_min == 2) {
    for (i = 0; i < ARRAY_SIZE(blacklist_events_dd22); i++) {
    event_init(&event, blacklist_events_dd22[i]);
    FAIL_IF(!event_open(&event));
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(blacklisted_events, "blacklisted_events");
    }
