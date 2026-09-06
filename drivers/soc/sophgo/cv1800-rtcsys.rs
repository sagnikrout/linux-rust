//! Automatically rewritten from C to Rust
//! Source: drivers/soc/sophgo/cv1800-rtcsys.c
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
// Driver for Sophgo CV1800 series SoC RTC subsystem
//
// The RTC module comprises a 32kHz oscillator, Power-on-Reset (PoR) sub-module,
// HW state machine to control chip power-on, power-off and reset. Furthermore,
// the 8051 subsystem is located within RTCSYS including associated SRAM block.
//
// Copyright (C) 2025 Alexander Sverdlin <alexander.sverdlin@gmail.com>
//

    static struct resource cv1800_rtcsys_irq_resources[] = {
    DEFINE_RES_IRQ_NAMED(0, "alarm"),
    };
    static const struct mfd_cell cv1800_rtcsys_subdev[] = {
    {
    .name = "cv1800b-rtc",
    .num_resources = 1,
    .resources = &cv1800_rtcsys_irq_resources[0],
    },
    };
#[no_mangle]
unsafe extern "C" fn cv1800_rtcsys_probe(pdev: *mut platform_device) -> c_int {
    static int cv1800_rtcsys_probe(struct platform_device *pdev)
    {
    int irq;
    irq = platform_get_irq_byname(pdev, "alarm");
    if (irq < 0)
    return irq;
    cv1800_rtcsys_irq_resources[0].start = irq;
    cv1800_rtcsys_irq_resources[0].end = irq;
    return devm_mfd_add_devices(&pdev.dev, PLATFORM_DEVID_AUTO,
    cv1800_rtcsys_subdev,
    ARRAY_SIZE(cv1800_rtcsys_subdev),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    }
    static const struct of_device_id cv1800_rtcsys_of_match[] = {
    { .compatible = "sophgo,cv1800b-rtc" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, cv1800_rtcsys_of_match);
    static struct platform_driver cv1800_rtcsys_mfd = {
    .probe	= cv1800_rtcsys_probe,
    .driver	= {
    .name		= "cv1800_rtcsys",
    .of_match_table	= cv1800_rtcsys_of_match,
    },
    };
    module_platform_driver(cv1800_rtcsys_mfd);
    MODULE_AUTHOR("Alexander Sverdlin <alexander.sverdlin@gmail.com>");
    MODULE_DESCRIPTION("Sophgo CV1800 series SoC RTC subsystem driver");
    MODULE_LICENSE("GPL");
