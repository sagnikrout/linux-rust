//! Automatically rewritten from C to Rust
//! Source: drivers/cpuidle/poll_state.c
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
// poll_state.c - Polling idle state
//

pub const POLL_IDLE_RELAX_COUNT: c_int = 200;
    static int __cpuidle poll_idle(struct cpuidle_device *dev,
    struct cpuidle_driver *drv, int index)
    {
    u64 time_start;
    time_start = local_clock_noinstr();
    dev.poll_time_limit = false;
    raw_local_irq_enable();
    if (!current_set_polling_and_test()) {
    let mut loop_count: c_uint = 0;
    u64 limit;
    limit = cpuidle_poll_time(drv, dev);
    while (!need_resched()) {
    cpu_relax();
    if (loop_count++ < POLL_IDLE_RELAX_COUNT)
    continue;
    loop_count = 0;
    if (local_clock_noinstr() - time_start > limit) {
    dev.poll_time_limit = true;
    break;
    }
    }
    }
    raw_local_irq_disable();
    current_clr_polling();
    return index;
    }
#[no_mangle]
pub unsafe extern "C" fn cpuidle_poll_state_init(drv: *mut cpuidle_driver) {
    void cpuidle_poll_state_init(struct cpuidle_driver *drv)
    {
    struct cpuidle_state *state = &drv.states[0];
    snprintf(state.name, CPUIDLE_NAME_LEN, "POLL");
    snprintf(state.desc, CPUIDLE_DESC_LEN, "CPUIDLE CORE POLL IDLE");
    state.exit_latency = 0;
    state.target_residency = 0;
    state.exit_latency_ns = 0;
    state.target_residency_ns = 0;
    state.power_usage = -1;
    state.enter = poll_idle;
    state.flags = CPUIDLE_FLAG_POLLING;
    }
    EXPORT_SYMBOL_GPL(cpuidle_poll_state_init);
