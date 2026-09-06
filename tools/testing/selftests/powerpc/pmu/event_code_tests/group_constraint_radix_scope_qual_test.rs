//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/event_code_tests/group_constraint_radix_scope_qual_test.c
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

// PM_DATA_RADIX_PROCESS_L2_PTE_FROM_L2
pub const EventCode_1: c_uint = 0x14242;
// PM_DATA_RADIX_PROCESS_L2_PTE_FROM_L3
pub const EventCode_2: c_uint = 0x24242;
//
// Testcase for group constraint check for radix_scope_qual
// field which is used to program Monitor Mode Control
// egister (MMCR1)  bit 18.
// All events in the group should match radix_scope_qual,
// bits otherwise event_open for the group should fail.
//
#[no_mangle]
unsafe extern "C" fn group_constraint_radix_scope_qual() -> c_int {
    static int group_constraint_radix_scope_qual(void)
    {
    struct event event, leader;
//
// Check for platform support for the test.
// This test is aplicable on ISA v3.1 only.
//
    SKIP_IF(platform_check_for_tests());
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_ARCH_3_1));
// Init the events for the group contraint check for radix_scope_qual bits
    event_init(&leader, EventCode_1);
    FAIL_IF(event_open(&leader));
    event_init(&event, 0x200fc);
// Expected to fail as sibling event doesn't request same radix_scope_qual bits as leader
    FAIL_IF(!event_open_with_group(&event, leader.fd));
    event_init(&event, EventCode_2);
// Expected to pass as sibling event request same radix_scope_qual bits as leader
    FAIL_IF(event_open_with_group(&event, leader.fd));
    event_close(&leader);
    event_close(&event);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(group_constraint_radix_scope_qual,
    "group_constraint_radix_scope_qual");
    }
