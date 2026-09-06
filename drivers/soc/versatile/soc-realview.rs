//! Automatically rewritten from C to Rust
//! Source: drivers/soc/versatile/soc-realview.c
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
// Copyright (C) 2014 Linaro Ltd.
//
// Author: Linus Walleij <linus.walleij@linaro.org>
//

// System ID in syscon
pub const REALVIEW_SYS_ID_OFFSET: c_uint = 0x00;
    static const struct of_device_id realview_soc_of_match[] = {
    { .compatible = "arm,realview-eb-soc",	},
    { .compatible = "arm,realview-pb1176-soc", },
    { .compatible = "arm,realview-pb11mp-soc", },
    { .compatible = "arm,realview-pba8-soc", },
    { .compatible = "arm,realview-pbx-soc", },
    { }
    };
    static u32 realview_coreid;
    static const char *realview_arch_str(u32 id)
    {
    switch ((id >> 8) & 0xf) {
    case 0x04:
    return "AHB";
    case 0x05:
    return "Multi-layer AXI";
    default:
    return "Unknown";
    }
    }
    static ssize_t
    manufacturer_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    return sprintf(buf, "%02x\n", realview_coreid >> 24);
    }
    static DEVICE_ATTR_RO(manufacturer);
    static ssize_t
    board_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    return sprintf(buf, "HBI-%03x\n", ((realview_coreid >> 16) & 0xfff));
    }
    static DEVICE_ATTR_RO(board);
    static ssize_t
    fpga_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    return sprintf(buf, "%s\n", realview_arch_str(realview_coreid));
    }
    static DEVICE_ATTR_RO(fpga);
    static ssize_t
    build_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    return sprintf(buf, "%02x\n", (realview_coreid & 0xFF));
    }
    static DEVICE_ATTR_RO(build);
    static struct attribute *realview_attrs[] = {
    &dev_attr_manufacturer.attr,
    &dev_attr_board.attr,
    &dev_attr_fpga.attr,
    &dev_attr_build.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(realview);
#[no_mangle]
unsafe extern "C" fn realview_soc_socdev_release(data: *mut c_void) {
    static void realview_soc_socdev_release(void *data)
    {
    struct soc_device *soc_dev = data;
    soc_device_unregister(soc_dev);
    }
#[no_mangle]
unsafe extern "C" fn realview_soc_probe(pdev: *mut platform_device) -> c_int {
    static int realview_soc_probe(struct platform_device *pdev)
    {
    struct regmap *syscon_regmap;
    struct soc_device *soc_dev;
    struct soc_device_attribute *soc_dev_attr;
    struct device_node *np = pdev.dev.of_node;
    int ret;
    syscon_regmap = syscon_regmap_lookup_by_phandle(np, "regmap");
    if (IS_ERR(syscon_regmap))
    return PTR_ERR(syscon_regmap);
    soc_dev_attr = devm_kzalloc(&pdev.dev, sizeof(*soc_dev_attr), GFP_KERNEL);
    if (!soc_dev_attr)
    return -ENOMEM;
    ret = of_property_read_string(np, "compatible",
    &soc_dev_attr.soc_id);
    if (ret)
    return -EINVAL;
    soc_dev_attr.machine = "RealView";
    soc_dev_attr.family = "Versatile";
    soc_dev_attr.custom_attr_group = realview_groups[0];
    soc_dev = soc_device_register(soc_dev_attr);
    if (IS_ERR(soc_dev))
    return -ENODEV;
    ret = devm_add_action_or_reset(&pdev.dev, realview_soc_socdev_release,
    soc_dev);
    if (ret)
    return ret;
    ret = regmap_read(syscon_regmap, REALVIEW_SYS_ID_OFFSET,
    &realview_coreid);
    if (ret)
    return -ENODEV;
    dev_info(&pdev.dev, "RealView Syscon Core ID: 0x%08x, HBI-%03x\n",
    realview_coreid,
    ((realview_coreid >> 16) & 0xfff));
// FIXME: add attributes for SoC to sysfs
    return 0;
    }
    static struct platform_driver realview_soc_driver = {
    .probe = realview_soc_probe,
    .driver = {
    .name = "realview-soc",
    .of_match_table = realview_soc_of_match,
    },
    };
    builtin_platform_driver(realview_soc_driver);
