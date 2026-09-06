//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/pmu/ebb/ebb_vs_cpu_event_test.c
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
// Tests an EBB vs a cpu event - in that order. The EBB should force the cpu
// event off the PMU.
//
#[no_mangle]
unsafe extern "C" fn setup_cpu_event(event: *mut event, cpu: c_int) -> c_int {
    static int setup_cpu_event(struct event *event, int cpu)
    {
    event_init_named(event, 0x400FA, "PM_RUN_INST_CMPL");
    event.attr.exclude_kernel = 1;
    event.attr.exclude_hv = 1;
    event.attr.exclude_idle = 1;
    SKIP_IF(require_paranoia_below(1));
    FAIL_IF(event_open_with_cpu(event, cpu));
    FAIL_IF(event_enable(event));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ebb_vs_cpu_event() -> c_int {
    int ebb_vs_cpu_event(void)
    {
    union pipe read_pipe, write_pipe;
    struct event event;
    int cpu, rc;
    pid_t pid;
    SKIP_IF(!ebb_is_supported());
    cpu = bind_to_cpu(BIND_CPU_ANY);
    FAIL_IF(cpu < 0);
    FAIL_IF(pipe(read_pipe.fds) == -1);
    FAIL_IF(pipe(write_pipe.fds) == -1);
    pid = fork();
    if (pid == 0) {
// NB order of pipes looks reversed
    exit(ebb_child(write_pipe, read_pipe));
    }
// Signal the child to install its EBB event and wait
    FAIL_IF(sync_with_child(read_pipe, write_pipe));
// Now try to install our CPU event
    rc = setup_cpu_event(&event, cpu);
    if (rc) {
    kill_child_and_wait(pid);
    return rc;
    }
// Signal the child to run
    FAIL_IF(sync_with_child(read_pipe, write_pipe));
// .. and wait for it to complete
    FAIL_IF(wait_for_child(pid));
    FAIL_IF(event_disable(&event));
    FAIL_IF(event_read(&event));
    event_report(&event);
// The cpu event may have run, but we don't expect 100%
    FAIL_IF(event.result.enabled >= event.result.running);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(ebb_vs_cpu_event, "ebb_vs_cpu_event");
    }
