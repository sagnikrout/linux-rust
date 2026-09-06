//! Automatically rewritten from C to Rust
//! Source: drivers/platform/mips/ls2k-reset.c
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
// Copyright (C) 2021, Qing Zhang <zhangqing@loongson.cn>
// Loongson-2K1000 reset support
//

pub const PM1_STS: c_uint = 0x0c /* Power Management 1 Status Register */;
pub const PM1_CNT: c_uint = 0x14 /* Power Management 1 Control Register */;
pub const RST_CNT: c_uint = 0x30 /* Reset Control Register */;
    static void __iomem *base;
#[no_mangle]
unsafe extern "C" fn ls2k_restart(command: *mut c_char) {
    static void ls2k_restart(char *command)
    {
    writel(0x1, base + RST_CNT);
    }
#[no_mangle]
unsafe extern "C" fn ls2k_poweroff() {
    static void ls2k_poweroff(void)
    {
// Clear
    writel((readl(base + PM1_STS) & 0xffffffff), base + PM1_STS);
// Sleep Enable | Soft Off
    writel(GENMASK(12, 10) | BIT(13), base + PM1_CNT);
    }
#[no_mangle]
unsafe extern "C" fn ls2k_reset_init() -> c_int {
    static int ls2k_reset_init(void)
    {
    struct device_node *np;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "loongson,ls2k-pm");
    if (!np) {
    pr_info("Failed to get PM node\n");
    return -ENODEV;
    }
    base = of_iomap(np, 0);
    of_node_put(np);
    if (!base) {
    pr_info("Failed to map PM register base address\n");
    return -ENOMEM;
    }
    _machine_restart = ls2k_restart;
    pm_power_off = ls2k_poweroff;
    return 0;
    }
    arch_initcall(ls2k_reset_init);
