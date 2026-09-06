//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/stm/heartbeat.c
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
// Simple heartbeat STM source driver
// Copyright (c) 2016, Intel Corporation.
//
// Heartbeat STM source will send repetitive messages over STM devices to a
// trace host.
//

pub const STM_HEARTBEAT_MAX: c_int = 32;
    let mut nr_devs: static int = 4;
    let mut interval_ms: static int = 10;
    module_param(nr_devs, int, 0400);
    module_param(interval_ms, int, 0600);
    static struct stm_heartbeat {
    struct stm_source_data	data;
    struct hrtimer		hrtimer;
    unsigned int		active;
    } stm_heartbeat[STM_HEARTBEAT_MAX];
    static const char str[] = "heartbeat stm source driver is here to serve you";
#[no_mangle]
unsafe extern "C" fn stm_heartbeat_hrtimer_handler(hr: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart stm_heartbeat_hrtimer_handler(struct hrtimer *hr)
    {
    struct stm_heartbeat *heartbeat = container_of(hr, struct stm_heartbeat,
    hrtimer);
    stm_source_write(&heartbeat.data, 0, str, sizeof str);
    if (heartbeat.active)
    hrtimer_forward_now(hr, ms_to_ktime(interval_ms));
    return heartbeat.active ? HRTIMER_RESTART : HRTIMER_NORESTART;
    }
#[no_mangle]
unsafe extern "C" fn stm_heartbeat_link(data: *mut stm_source_data) -> c_int {
    static int stm_heartbeat_link(struct stm_source_data *data)
    {
    struct stm_heartbeat *heartbeat =
    container_of(data, struct stm_heartbeat, data);
    heartbeat.active = 1;
    hrtimer_start(&heartbeat.hrtimer, ms_to_ktime(interval_ms),
    HRTIMER_MODE_ABS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm_heartbeat_unlink(data: *mut stm_source_data) {
    static void stm_heartbeat_unlink(struct stm_source_data *data)
    {
    struct stm_heartbeat *heartbeat =
    container_of(data, struct stm_heartbeat, data);
    heartbeat.active = 0;
    hrtimer_cancel(&heartbeat.hrtimer);
    }
#[no_mangle]
unsafe extern "C" fn stm_heartbeat_init() -> c_int {
    static int stm_heartbeat_init(void)
    {
    int i, ret;
    if (nr_devs < 0 || nr_devs > STM_HEARTBEAT_MAX)
    return -EINVAL;
    for (i = 0; i < nr_devs; i++) {
    stm_heartbeat[i].data.name =
    kasprintf(GFP_KERNEL, "heartbeat.%d", i);
    if (!stm_heartbeat[i].data.name) {
    ret = -ENOMEM;
    goto fail_unregister;
    }
    stm_heartbeat[i].data.nr_chans	= 1;
    stm_heartbeat[i].data.type	= STM_USER;
    stm_heartbeat[i].data.link	= stm_heartbeat_link;
    stm_heartbeat[i].data.unlink	= stm_heartbeat_unlink;
    hrtimer_setup(&stm_heartbeat[i].hrtimer, stm_heartbeat_hrtimer_handler,
    CLOCK_MONOTONIC, HRTIMER_MODE_ABS);
    ret = stm_source_register_device(core::ptr::null_mut(), &stm_heartbeat[i].data);
    if (ret)
    goto fail_free;
    }
    return 0;
    fail_unregister:
    for (i--; i >= 0; i--) {
    stm_source_unregister_device(&stm_heartbeat[i].data);
    fail_free:
    kfree(stm_heartbeat[i].data.name);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm_heartbeat_exit() {
    static void stm_heartbeat_exit(void)
    {
    int i;
    for (i = 0; i < nr_devs; i++) {
    stm_source_unregister_device(&stm_heartbeat[i].data);
    kfree(stm_heartbeat[i].data.name);
    }
    }
    module_init(stm_heartbeat_init);
    module_exit(stm_heartbeat_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("stm_heartbeat driver");
    MODULE_AUTHOR("Alexander Shishkin <alexander.shishkin@linux.intel.com>");
