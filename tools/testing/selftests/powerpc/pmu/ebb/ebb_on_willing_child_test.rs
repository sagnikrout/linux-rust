//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/ebb/ebb_on_willing_child_test.c
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
// Tests we can setup an EBB on our child. The child expects this and enables
// EBBs, which are then delivered to the child, even though the event is
// created by the parent.
//
#[no_mangle]
unsafe extern "C" fn victim_child(read_pipe: union pipe, write_pipe: union pipe) -> c_int {
    static int victim_child(union pipe read_pipe, union pipe write_pipe)
    {
    FAIL_IF(wait_for_parent(read_pipe));
// Setup our EBB handler, before the EBB event is created
    ebb_enable_pmc_counting(1);
    setup_ebb_handler(standard_ebb_callee);
    ebb_global_enable();
    FAIL_IF(notify_parent(write_pipe));
    while (ebb_state.stats.ebb_count < 20) {
    FAIL_IF(core_busy_loop());
    }
    ebb_global_disable();
    ebb_freeze_pmcs();
    dump_ebb_state();
    FAIL_IF(ebb_state.stats.ebb_count == 0);
    return 0;
    }
// Tests we can setup an EBB on our child - if it's expecting it
#[no_mangle]
pub unsafe extern "C" fn ebb_on_willing_child() -> c_int {
    int ebb_on_willing_child(void)
    {
    union pipe read_pipe, write_pipe;
    struct event event;
    pid_t pid;
    SKIP_IF(!ebb_is_supported());
    FAIL_IF(pipe(read_pipe.fds) == -1);
    FAIL_IF(pipe(write_pipe.fds) == -1);
    pid = fork();
    if (pid == 0) {
// NB order of pipes looks reversed
    exit(victim_child(write_pipe, read_pipe));
    }
// Signal the child to setup its EBB handler
    FAIL_IF(sync_with_child(read_pipe, write_pipe));
// Child is running now
    event_init_named(&event, 0x1001e, "cycles");
    event_leader_ebb_init(&event);
    event.attr.exclude_kernel = 1;
    event.attr.exclude_hv = 1;
    event.attr.exclude_idle = 1;
    FAIL_IF(event_open_with_pid(&event, pid));
    FAIL_IF(ebb_event_enable(&event));
// Child show now take EBBs and then exit
    FAIL_IF(wait_for_child(pid));
    event_close(&event);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(ebb_on_willing_child, "ebb_on_willing_child");
    }
