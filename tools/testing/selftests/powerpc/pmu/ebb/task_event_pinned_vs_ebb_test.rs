//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/ebb/task_event_pinned_vs_ebb_test.c
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
// Tests a pinned per-task event vs an EBB - in that order. The pinned per-task
// event should prevent the EBB event from being enabled.
//
#[no_mangle]
unsafe extern "C" fn setup_child_event(event: *mut event, child_pid: pid_t) -> c_int {
    static int setup_child_event(struct event *event, pid_t child_pid)
    {
    event_init_named(event, 0x400FA, "PM_RUN_INST_CMPL");
    event.attr.pinned = 1;
    event.attr.exclude_kernel = 1;
    event.attr.exclude_hv = 1;
    event.attr.exclude_idle = 1;
    FAIL_IF(event_open_with_pid(event, child_pid));
    FAIL_IF(event_enable(event));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn task_event_pinned_vs_ebb() -> c_int {
    int task_event_pinned_vs_ebb(void)
    {
    union pipe read_pipe, write_pipe;
    struct event event;
    pid_t pid;
    int rc;
    SKIP_IF(!ebb_is_supported());
    FAIL_IF(pipe(read_pipe.fds) == -1);
    FAIL_IF(pipe(write_pipe.fds) == -1);
    pid = fork();
    if (pid == 0) {
// NB order of pipes looks reversed
    exit(ebb_child(write_pipe, read_pipe));
    }
// We setup the task event first
    rc = setup_child_event(&event, pid);
    if (rc) {
    kill_child_and_wait(pid);
    return rc;
    }
// Signal the child to install its EBB event and wait
    if (sync_with_child(read_pipe, write_pipe))
// If it fails, wait for it to exit
    goto wait;
// Signal the child to run
    FAIL_IF(sync_with_child(read_pipe, write_pipe));
    wait:
// We expect it to fail to read the event
    FAIL_IF(wait_for_child(pid) != 2);
    FAIL_IF(event_disable(&event));
    FAIL_IF(event_read(&event));
    event_report(&event);
    FAIL_IF(event.result.value == 0);
//
// For reasons I don't understand enabled is usually just slightly
// lower than running. Would be good to confirm why.
//
    FAIL_IF(event.result.enabled == 0);
    FAIL_IF(event.result.running == 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(task_event_pinned_vs_ebb, "task_event_pinned_vs_ebb");
    }
