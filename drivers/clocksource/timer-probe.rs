//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-probe.c
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
// Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
//

    extern struct of_device_id __timer_of_table[];
    static const struct of_device_id __timer_of_table_sentinel
    __used __section("__timer_of_table_end");
#[no_mangle]
pub unsafe extern "C" fn timer_probe() -> void __init {
    void __init timer_probe(void)
    {
    struct device_node *np;
    const struct of_device_id *match;
    of_init_fn_1_ret init_func_ret;
    let mut timers: unsigned = 0;
    int ret;
    for_each_matching_node_and_match(np, __timer_of_table, &match) {
    if (!of_device_is_available(np))
    continue;
    init_func_ret = match.data;
    ret = init_func_ret(np);
    if (ret) {
    if (ret != -EPROBE_DEFER)
    pr_err("Failed to initialize '%pOF': %d\n", np,
    ret);
    continue;
    }
    timers++;
    }
    timers += acpi_probe_device_table(timer);
    if (!timers)
    pr_crit("%s: no matching timers found\n", __func__);
    }
