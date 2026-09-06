//! Automatically rewritten from C to Rust
//! Source: drivers/cpuidle/cpuidle-clps711x.c
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
// CLPS711X CPU idle driver
//
// Copyright (C) 2014 Alexander Shiyan <shc_work@mail.ru>
//

    static void __iomem *clps711x_halt;
    static int clps711x_cpuidle_halt(struct cpuidle_device *dev,
    struct cpuidle_driver *drv, int index)
    {
    writel(0xaa, clps711x_halt);
    return index;
    }
    static struct cpuidle_driver clps711x_idle_driver = {
    .name		= CLPS711X_CPUIDLE_NAME,
    .owner		= THIS_MODULE,
    .states[0]	= {
    .name		= "HALT",
    .desc		= "CLPS711X HALT",
    .enter		= clps711x_cpuidle_halt,
    .exit_latency	= 1,
    },
    .state_count	= 1,
    };
#[no_mangle]
unsafe extern "C" fn clps711x_cpuidle_probe(pdev: *mut platform_device) -> int __init {
    static int __init clps711x_cpuidle_probe(struct platform_device *pdev)
    {
    clps711x_halt = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(clps711x_halt))
    return PTR_ERR(clps711x_halt);
    return cpuidle_register(&clps711x_idle_driver, core::ptr::null_mut());
    }
    static struct platform_driver clps711x_cpuidle_driver = {
    .driver	= {
    .name	= CLPS711X_CPUIDLE_NAME,
    },
    };
    builtin_platform_driver_probe(clps711x_cpuidle_driver, clps711x_cpuidle_probe);
