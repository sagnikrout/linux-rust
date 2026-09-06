//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-versatile.c
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
// Copyright (C) 2014 ARM Limited
//

pub const SYS_24MHZ: c_uint = 0x05c;
    static void __iomem *versatile_sys_24mhz;
#[no_mangle]
unsafe extern "C" fn versatile_sys_24mhz_read() -> u64 notrace {
    static u64 notrace versatile_sys_24mhz_read(void)
    {
    return readl(versatile_sys_24mhz);
    }
#[no_mangle]
unsafe extern "C" fn versatile_sched_clock_init(node: *mut device_node) -> int __init {
    static int __init versatile_sched_clock_init(struct device_node *node)
    {
    void __iomem *base = of_iomap(node, 0);
    of_node_clear_flag(node, OF_POPULATED);
    if (!base)
    return -ENXIO;
    versatile_sys_24mhz = base + SYS_24MHZ;
    sched_clock_register(versatile_sys_24mhz_read, 32, 24000000);
    return 0;
    }
    TIMER_OF_DECLARE(vexpress, "arm,vexpress-sysreg",
    versatile_sched_clock_init);
    TIMER_OF_DECLARE(versatile, "arm,versatile-sysreg",
    versatile_sched_clock_init);
