//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/broadcom/sr-thermal.c
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
// Copyright (C) 2018 Broadcom
//

//
// In stingray thermal IO memory,
// Total Number of available TMONs MASK is at offset 0
// temperature registers BASE is at 4 byte offset.
// Each TMON temperature register size is 4.
//

pub const SR_TMON_MAX_LIST: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sr_tmon {
    pub crit_temp: c_uint,
    pub tmon_id: c_uint,
    pub priv: *mut sr_thermal,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sr_thermal {
    pub regs: *mut void __iomem,
    pub max_crit_temp: c_uint,
    pub tmon: [sr_tmon; SR_TMON_MAX_LIST],
}

#[no_mangle]
unsafe extern "C" fn sr_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int sr_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct sr_tmon *tmon = thermal_zone_device_priv(tz);
    struct sr_thermal *sr_thermal = tmon.priv;
// temp = readl(sr_thermal->regs + SR_TMON_TEMP_BASE(tmon->tmon_id));
    return 0;
    }
    static const struct thermal_zone_device_ops sr_tz_ops = {
    .get_temp = sr_get_temp,
    };
#[no_mangle]
unsafe extern "C" fn sr_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int sr_thermal_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct thermal_zone_device *tz;
    struct sr_thermal *sr_thermal;
    struct sr_tmon *tmon;
    struct resource *res;
    let mut sr_tmon_list: u32 = 0;
    unsigned int i;
    int ret;
    sr_thermal = devm_kzalloc(dev, sizeof(*sr_thermal), GFP_KERNEL);
    if (!sr_thermal)
    return -ENOMEM;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENOENT;
    sr_thermal.regs = (void __iomem *)devm_memremap(&pdev.dev, res.start,
    resource_size(res),
    MEMREMAP_WB);
    if (IS_ERR(sr_thermal.regs)) {
    dev_err(dev, "failed to get io address\n");
    return PTR_ERR(sr_thermal.regs);
    }
    ret = device_property_read_u32(dev, "brcm,tmon-mask", &sr_tmon_list);
    if (ret)
    return ret;
    tmon = sr_thermal.tmon;
    for (i = 0; i < SR_TMON_MAX_LIST; i++, tmon++) {
    if (!(sr_tmon_list & BIT(i)))
    continue;
// Flush temperature registers
    writel(0, sr_thermal.regs + SR_TMON_TEMP_BASE(i));
    tmon.tmon_id = i;
    tmon.priv = sr_thermal;
    tz = devm_thermal_of_zone_register(dev, i, tmon,
    &sr_tz_ops);
    if (IS_ERR(tz))
    return PTR_ERR(tz);
    dev_dbg(dev, "thermal sensor %d registered\n", i);
    }
    return 0;
    }
    static const struct of_device_id sr_thermal_of_match[] = {
    { .compatible = "brcm,sr-thermal", },
    {},
    };
    MODULE_DEVICE_TABLE(of, sr_thermal_of_match);
    static struct platform_driver sr_thermal_driver = {
    .probe		= sr_thermal_probe,
    .driver = {
    .name = "sr-thermal",
    .of_match_table = sr_thermal_of_match,
    },
    };
    module_platform_driver(sr_thermal_driver);
    MODULE_AUTHOR("Pramod Kumar <pramod.kumar@broadcom.com>");
    MODULE_DESCRIPTION("Stingray thermal driver");
    MODULE_LICENSE("GPL v2");
