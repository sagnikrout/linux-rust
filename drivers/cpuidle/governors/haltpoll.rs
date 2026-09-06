//! Automatically rewritten from C to Rust
//! Source: drivers/cpuidle/governors/haltpoll.c
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


// SPDX-License-Identifier: GPL-2.0
//
// haltpoll.c - haltpoll idle governor
//
// Copyright 2019 Red Hat, Inc. and/or its affiliates.
//
// This work is licensed under the terms of the GNU GPL, version 2.  See
// the COPYING file in the top-level directory.
//
// Authors: Marcelo Tosatti <mtosatti@redhat.com>
//

    let mut __read_mostly: static unsigned int guest_halt_poll_ns = 200000;
    module_param(guest_halt_poll_ns, uint, 0644);
// division factor to shrink halt_poll_ns
    let mut __read_mostly: static unsigned int guest_halt_poll_shrink = 2;
    module_param(guest_halt_poll_shrink, uint, 0644);
// multiplication factor to grow per-cpu poll_limit_ns
    let mut __read_mostly: static unsigned int guest_halt_poll_grow = 2;
    module_param(guest_halt_poll_grow, uint, 0644);
// value in us to start growing per-cpu halt_poll_ns
    let mut __read_mostly: static unsigned int guest_halt_poll_grow_start = 50000;
    module_param(guest_halt_poll_grow_start, uint, 0644);
// allow shrinking guest halt poll
    let mut __read_mostly: static bool guest_halt_poll_allow_shrink = true;
    module_param(guest_halt_poll_allow_shrink, bool, 0644);
//
// haltpoll_select - selects the next idle state to enter
// @drv: cpuidle driver containing state data
// @dev: the CPU
// @stop_tick: indication on whether or not to stop the tick
//
    static int haltpoll_select(struct cpuidle_driver *drv,
    struct cpuidle_device *dev,
    bool *stop_tick)
    {
    if (cpuidle_governor_latency_req(dev.cpu) == 0) {
// stop_tick = false;
    return 0;
    }
    if (dev.poll_limit_ns == 0)
    return 1;
// Last state was poll?
    if (dev.last_state_idx == 0) {
// Halt if no event occurred on poll window
    if (dev.poll_time_limit == true)
    return 1;
// stop_tick = false;
// Otherwise, poll again
    return 0;
    }
// stop_tick = false;
// Last state was halt: poll
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adjust_poll_limit(dev: *mut cpuidle_device, block_ns: u64) {
    static void adjust_poll_limit(struct cpuidle_device *dev, u64 block_ns)
    {
    unsigned int val;
// Grow cpu_halt_poll_us if
// cpu_halt_poll_us < block_ns < guest_halt_poll_us
//
    if (block_ns > dev.poll_limit_ns && block_ns <= guest_halt_poll_ns) {
    val = dev.poll_limit_ns * guest_halt_poll_grow;
    if (val < guest_halt_poll_grow_start)
    val = guest_halt_poll_grow_start;
    if (val > guest_halt_poll_ns)
    val = guest_halt_poll_ns;
    trace_guest_halt_poll_ns_grow(val, dev.poll_limit_ns);
    dev.poll_limit_ns = val;
    } else if (block_ns > guest_halt_poll_ns &&
    guest_halt_poll_allow_shrink) {
    let mut shrink: c_uint = guest_halt_poll_shrink;
    val = dev.poll_limit_ns;
    if (shrink == 0) {
    val = 0;
    } else {
    val /= shrink;
// Reset value to 0 if shrunk below grow_start
    if (val < guest_halt_poll_grow_start)
    val = 0;
    }
    trace_guest_halt_poll_ns_shrink(val, dev.poll_limit_ns);
    dev.poll_limit_ns = val;
    }
    }
//
// haltpoll_reflect - update variables and update poll time
// @dev: the CPU
// @index: the index of actual entered state
//
#[no_mangle]
unsafe extern "C" fn haltpoll_reflect(dev: *mut cpuidle_device, index: c_int) {
    static void haltpoll_reflect(struct cpuidle_device *dev, int index)
    {
    dev.last_state_idx = index;
    if (index != 0)
    adjust_poll_limit(dev, dev.last_residency_ns);
    }
//
// haltpoll_enable_device - scans a CPU's states and does setup
// @drv: cpuidle driver
// @dev: the CPU
//
    static int haltpoll_enable_device(struct cpuidle_driver *drv,
    struct cpuidle_device *dev)
    {
    dev.poll_limit_ns = 0;
    return 0;
    }
    static struct cpuidle_governor haltpoll_governor = {
    .name =			"haltpoll",
    .rating =		9,
    .enable =		haltpoll_enable_device,
    .select =		haltpoll_select,
    .reflect =		haltpoll_reflect,
    };
#[no_mangle]
unsafe extern "C" fn init_haltpoll() -> int __init {
    static int __init init_haltpoll(void)
    {
    if (kvm_para_available())
    return cpuidle_register_governor(&haltpoll_governor);
    return 0;
    }
    postcore_initcall(init_haltpoll);
