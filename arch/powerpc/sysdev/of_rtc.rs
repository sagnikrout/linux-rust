//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/of_rtc.c
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
// Instantiate mmio-mapped RTC chips based on device tree information
//
// Copyright 2007 David Gibson <dwg@au1.ibm.com>, IBM Corporation.
//

    static __initdata struct {
    const char *compatible;
    char *plat_name;
    } of_rtc_table[] = {
    { "ds1743-nvram", "rtc-ds1742" },
    };
#[no_mangle]
pub unsafe extern "C" fn of_instantiate_rtc() -> void __init {
    void __init of_instantiate_rtc(void)
    {
    struct device_node *node;
    int err;
    int i;
    for (i = 0; i < ARRAY_SIZE(of_rtc_table); i++) {
    char *plat_name = of_rtc_table[i].plat_name;
    for_each_compatible_node(node, core::ptr::null_mut(),
    of_rtc_table[i].compatible) {
    struct resource *res;
    res = kmalloc_obj(*res);
    if (!res) {
    printk(KERN_ERR "OF RTC: Out of memory "
    "allocating resource structure for %pOF\n",
    node);
    continue;
    }
    err = of_address_to_resource(node, 0, res);
    if (err) {
    printk(KERN_ERR "OF RTC: Error "
    "translating resources for %pOF\n",
    node);
    continue;
    }
    printk(KERN_INFO "OF_RTC: %pOF is a %s @ 0x%llx-0x%llx\n",
    node, plat_name,
    (unsigned long long)res.start,
    (unsigned long long)res.end);
    platform_device_register_simple(plat_name, -1, res, 1);
    }
    }
    }
