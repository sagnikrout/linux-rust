//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/ebb/event_attributes_test.c
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
// Test various attributes of the EBB event are enforced.
//
#[no_mangle]
pub unsafe extern "C" fn event_attributes() -> c_int {
    int event_attributes(void)
    {
    struct event event, leader;
    SKIP_IF(!ebb_is_supported());
    event_init(&event, 0x1001e);
    event_leader_ebb_init(&event);
// Expected to succeed
    FAIL_IF(event_open(&event));
    event_close(&event);
    event_init(&event, 0x001e); /* CYCLES - no PMC specified */
    event_leader_ebb_init(&event);
// Expected to fail, no PMC specified
    FAIL_IF(event_open(&event) == 0);
    event_init(&event, 0x2001e);
    event_leader_ebb_init(&event);
    event.attr.exclusive = 0;
// Expected to fail, not exclusive
    FAIL_IF(event_open(&event) == 0);
    event_init(&event, 0x3001e);
    event_leader_ebb_init(&event);
    event.attr.freq = 1;
// Expected to fail, sets freq
    FAIL_IF(event_open(&event) == 0);
    event_init(&event, 0x4001e);
    event_leader_ebb_init(&event);
    event.attr.sample_period = 1;
// Expected to fail, sets sample_period
    FAIL_IF(event_open(&event) == 0);
    event_init(&event, 0x1001e);
    event_leader_ebb_init(&event);
    event.attr.enable_on_exec = 1;
// Expected to fail, sets enable_on_exec
    FAIL_IF(event_open(&event) == 0);
    event_init(&event, 0x1001e);
    event_leader_ebb_init(&event);
    event.attr.inherit = 1;
// Expected to fail, sets inherit
    FAIL_IF(event_open(&event) == 0);
    event_init(&leader, 0x1001e);
    event_leader_ebb_init(&leader);
    FAIL_IF(event_open(&leader));
    event_init(&event, 0x20002);
    event_ebb_init(&event);
// Expected to succeed
    FAIL_IF(event_open_with_group(&event, leader.fd));
    event_close(&leader);
    event_close(&event);
    event_init(&leader, 0x1001e);
    event_leader_ebb_init(&leader);
    FAIL_IF(event_open(&leader));
    event_init(&event, 0x20002);
// Expected to fail, event doesn't request EBB, leader does
    FAIL_IF(event_open_with_group(&event, leader.fd) == 0);
    event_close(&leader);
    event_init(&leader, 0x1001e);
    event_leader_ebb_init(&leader);
// Clear the EBB flag
    leader.attr.config &= ~(1ull << 63);
    FAIL_IF(event_open(&leader));
    event_init(&event, 0x20002);
    event_ebb_init(&event);
// Expected to fail, leader doesn't request EBB
    FAIL_IF(event_open_with_group(&event, leader.fd) == 0);
    event_close(&leader);
    event_init(&leader, 0x1001e);
    event_leader_ebb_init(&leader);
    leader.attr.exclusive = 0;
// Expected to fail, leader isn't exclusive
    FAIL_IF(event_open(&leader) == 0);
    event_init(&leader, 0x1001e);
    event_leader_ebb_init(&leader);
    leader.attr.pinned = 0;
// Expected to fail, leader isn't pinned
    FAIL_IF(event_open(&leader) == 0);
    event_init(&event, 0x1001e);
    event_leader_ebb_init(&event);
// Expected to fail, not a task event
    SKIP_IF(require_paranoia_below(1));
    FAIL_IF(event_open_with_cpu(&event, 0) == 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(event_attributes, "event_attributes");
    }
