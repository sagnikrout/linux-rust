//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/geode/geos.c
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
// System Specific setup for Traverse Technologies GEOS.
// At the moment this means setup of GPIO control of LEDs.
//
// Copyright (C) 2008 Constantin Baranov <const@mimas.ru>
// Copyright (C) 2011 Ed Wildgoose <kernel@wildgooses.com>
// and Philip Prindeville <philipp@redfish-solutions.com>
//

    static const struct geode_led geos_leds[] __initconst = {
    { 6, true },
    { 25, false },
    { 27, false },
    };
#[no_mangle]
unsafe extern "C" fn register_geos() -> void __init {
    static void __init register_geos(void)
    {
    geode_create_restart_key(3);
    geode_create_leds("geos", geos_leds, ARRAY_SIZE(geos_leds));
    }
#[no_mangle]
unsafe extern "C" fn geos_init() -> int __init {
    static int __init geos_init(void)
    {
    const char *vendor, *product;
    if (!is_geode())
    return 0;
    vendor = dmi_get_system_info(DMI_SYS_VENDOR);
    if (!vendor || strcmp(vendor, "Traverse Technologies"))
    return 0;
    product = dmi_get_system_info(DMI_PRODUCT_NAME);
    if (!product || strcmp(product, "Geos"))
    return 0;
    printk(KERN_INFO "%s: system is recognized as \"%s %s\"\n",
    KBUILD_MODNAME, vendor, product);
    register_geos();
    return 0;
    }
    device_initcall(geos_init);
