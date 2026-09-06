//! Automatically rewritten from C to Rust
//! Source: drivers/pps/generators/pps_gen-dummy.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// PPS dummy generator
//
// Copyright (C) 2024 Rodolfo Giometti <giometti@enneenne.com>
//

    static struct pps_gen_device *pps_gen;
    static struct timer_list ktimer;
#[no_mangle]
unsafe extern "C" fn get_random_delay() -> c_uint {
    static unsigned int get_random_delay(void)
    {
    let mut delay: c_uint = get_random_u8() & 0x0f;
    return (delay + 1) * HZ;
    }
//
// The kernel timer
//
#[no_mangle]
unsafe extern "C" fn pps_gen_ktimer_event(unused: *mut timer_list) {
    static void pps_gen_ktimer_event(struct timer_list *unused)
    {
    pps_gen_event(pps_gen, PPS_GEN_EVENT_MISSEDPULSE, core::ptr::null_mut());
    }
//
// PPS Generator methods
//
    static int pps_gen_dummy_get_time(struct pps_gen_device *pps_gen,
    struct timespec64 *time)
    {
    ktime_get_real_ts64(time);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pps_gen_dummy_enable(pps_gen: *mut pps_gen_device, enable: bool) -> c_int {
    static int pps_gen_dummy_enable(struct pps_gen_device *pps_gen, bool enable)
    {
    if (enable)
    mod_timer(&ktimer, jiffies + get_random_delay());
    else
    timer_delete_sync(&ktimer);
    return 0;
    }
//
// The PPS info struct
//
    static const struct pps_gen_source_info pps_gen_dummy_info = {
    .use_system_clock	= true,
    .get_time		= pps_gen_dummy_get_time,
    .enable			= pps_gen_dummy_enable,
    };
//
// Module staff
//
#[no_mangle]
unsafe extern "C" fn pps_gen_dummy_exit() -> void __exit {
    static void __exit pps_gen_dummy_exit(void)
    {
    timer_delete_sync(&ktimer);
    pps_gen_unregister_source(pps_gen);
    }
#[no_mangle]
unsafe extern "C" fn pps_gen_dummy_init() -> int __init {
    static int __init pps_gen_dummy_init(void)
    {
    pps_gen = pps_gen_register_source(&pps_gen_dummy_info);
    if (IS_ERR(pps_gen))
    return PTR_ERR(pps_gen);
    timer_setup(&ktimer, pps_gen_ktimer_event, 0);
    return 0;
    }
    module_init(pps_gen_dummy_init);
    module_exit(pps_gen_dummy_exit);
    MODULE_AUTHOR("Rodolfo Giometti <giometti@enneenne.com>");
    MODULE_DESCRIPTION("LinuxPPS dummy generator");
    MODULE_LICENSE("GPL");
