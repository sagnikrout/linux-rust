//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/syscon-poweroff.c
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
// Generic Syscon Poweroff Driver
//
// Copyright (c) 2015, National Instruments Corp.
// Author: Moritz Fischer <moritz.fischer@ettus.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscon_poweroff_data {
    pub map: *mut regmap,
    pub offset: u32,
    pub value: u32,
    pub mask: u32,
}

#[no_mangle]
unsafe extern "C" fn syscon_poweroff(off_data: *mut sys_off_data) -> c_int {
    static int syscon_poweroff(struct sys_off_data *off_data)
    {
    struct syscon_poweroff_data *data = off_data.cb_data;
// Issue the poweroff
    regmap_update_bits(data.map, data.offset, data.mask, data.value);
    mdelay(1000);
    pr_emerg("Unable to poweroff system\n");
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn syscon_poweroff_probe(pdev: *mut platform_device) -> c_int {
    static int syscon_poweroff_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct syscon_poweroff_data *data;
    int mask_err, value_err;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.map = syscon_regmap_lookup_by_phandle(dev.of_node, "regmap");
    if (IS_ERR(data.map)) {
    data.map = syscon_node_to_regmap(dev.parent.of_node);
    if (IS_ERR(data.map)) {
    dev_err(dev, "unable to get syscon");
    return PTR_ERR(data.map);
    }
    }
    if (of_property_read_u32(dev.of_node, "offset", &data.offset)) {
    dev_err(dev, "unable to read 'offset'");
    return -EINVAL;
    }
    value_err = of_property_read_u32(dev.of_node, "value", &data.value);
    mask_err = of_property_read_u32(dev.of_node, "mask", &data.mask);
    if (value_err && mask_err) {
    dev_err(dev, "unable to read 'value' and 'mask'");
    return -EINVAL;
    }
    if (value_err) {
// support old binding
    data.value = data.mask;
    data.mask = 0xFFFFFFFF;
    } else if (mask_err) {
// support value without mask
    data.mask = 0xFFFFFFFF;
    }
    return devm_register_sys_off_handler(&pdev.dev,
    SYS_OFF_MODE_POWER_OFF,
    SYS_OFF_PRIO_DEFAULT,
    syscon_poweroff, data);
    }
    static const struct of_device_id syscon_poweroff_of_match[] = {
    { .compatible = "syscon-poweroff" },
    {}
    };
    static struct platform_driver syscon_poweroff_driver = {
    .probe = syscon_poweroff_probe,
    .driver = {
    .name = "syscon-poweroff",
    .of_match_table = syscon_poweroff_of_match,
    },
    };
    builtin_platform_driver(syscon_poweroff_driver);
