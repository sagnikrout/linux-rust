//! Automatically rewritten from C to Rust
//! Source: kernel/power/poweroff.c
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
// poweroff.c - sysrq handler to gracefully power down machine.
//

//
// When the user hits Sys-Rq o to power down the machine this is the
// callback we use.
//
#[no_mangle]
unsafe extern "C" fn do_poweroff(dummy: *mut work_struct) {
    static void do_poweroff(struct work_struct *dummy)
    {
    kernel_power_off();
    }
    static DECLARE_WORK(poweroff_work, do_poweroff);
#[no_mangle]
unsafe extern "C" fn handle_poweroff(key: u8) {
    static void handle_poweroff(u8 key)
    {
// run sysrq poweroff on boot cpu
    schedule_work_on(cpumask_first(cpu_online_mask), &poweroff_work);
    }
    static const struct sysrq_key_op	sysrq_poweroff_op = {
    .handler        = handle_poweroff,
    .help_msg       = "poweroff(o)",
    .action_msg     = "Power Off",
    .enable_mask	= SYSRQ_ENABLE_BOOT,
    };
#[no_mangle]
unsafe extern "C" fn pm_sysrq_init() -> int __init {
    static int __init pm_sysrq_init(void)
    {
    register_sysrq_key('o', &sysrq_poweroff_op);
    return 0;
    }
    subsys_initcall(pm_sysrq_init);
