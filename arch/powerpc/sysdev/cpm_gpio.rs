//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/cpm_gpio.c
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
// Common CPM GPIO wrapper for the CPM GPIO ports
//
// Author: Christophe Leroy <christophe.leroy@c-s.fr>
//
// Copyright 2017 CS Systemes d'Information.
//

#[no_mangle]
unsafe extern "C" fn cpm_gpio_probe(ofdev: *mut platform_device) -> c_int {
    static int cpm_gpio_probe(struct platform_device *ofdev)
    {
    struct device *dev = &ofdev.dev;
    int (*gp_add)(struct device *dev) = of_device_get_match_data(dev);
    if (!gp_add)
    return -ENODEV;
    return gp_add(dev);
    }
    static const struct of_device_id cpm_gpio_match[] = {

    {
    .compatible = "fsl,cpm1-pario-bank-a",
    .data = cpm1_gpiochip_add16,
    },
    {
    .compatible = "fsl,cpm1-pario-bank-b",
    .data = cpm1_gpiochip_add32,
    },
    {
    .compatible = "fsl,cpm1-pario-bank-c",
    .data = cpm1_gpiochip_add16,
    },
    {
    .compatible = "fsl,cpm1-pario-bank-d",
    .data = cpm1_gpiochip_add16,
    },
// Port E uses CPM2 layout
    {
    .compatible = "fsl,cpm1-pario-bank-e",
    .data = cpm2_gpiochip_add32,
    },

    {
    .compatible = "fsl,cpm2-pario-bank",
    .data = cpm2_gpiochip_add32,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, cpm_gpio_match);
    static struct platform_driver cpm_gpio_driver = {
    .probe		= cpm_gpio_probe,
    .driver		= {
    .name	= "cpm-gpio",
    .of_match_table	= cpm_gpio_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn cpm_gpio_init() -> int __init {
    static int __init cpm_gpio_init(void)
    {
    return platform_driver_register(&cpm_gpio_driver);
    }
    arch_initcall(cpm_gpio_init);
    MODULE_AUTHOR("Christophe Leroy <christophe.leroy@c-s.fr>");
    MODULE_DESCRIPTION("Driver for CPM GPIO");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:cpm-gpio");
