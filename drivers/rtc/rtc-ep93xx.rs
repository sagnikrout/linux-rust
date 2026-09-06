//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-ep93xx.c
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
// A driver for the RTC embedded in the Cirrus Logic EP93XX processors
// Copyright (c) 2006 Tower Technologies
//
// Author: Alessandro Zummo <a.zummo@towertech.it>
//

pub const EP93XX_RTC_DATA: c_uint = 0x000;
pub const EP93XX_RTC_MATCH: c_uint = 0x004;
pub const EP93XX_RTC_STATUS: c_uint = 0x008;

pub const EP93XX_RTC_LOAD: c_uint = 0x00C;
pub const EP93XX_RTC_CONTROL: c_uint = 0x010;

pub const EP93XX_RTC_SWCOMP: c_uint = 0x108;
pub const EP93XX_RTC_SWCOMP_DEL_MASK: c_uint = 0x001f0000;
pub const EP93XX_RTC_SWCOMP_DEL_SHIFT: c_int = 16;
pub const EP93XX_RTC_SWCOMP_INT_MASK: c_uint = 0x0000ffff;
pub const EP93XX_RTC_SWCOMP_INT_SHIFT: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep93xx_rtc {
    pub mmio_base: *mut void __iomem,
}

    static int ep93xx_rtc_get_swcomp(struct device *dev, unsigned short *preload,
    unsigned short *delete)
    {
    struct ep93xx_rtc *ep93xx_rtc = dev_get_drvdata(dev);
    unsigned long comp;
    comp = readl(ep93xx_rtc.mmio_base + EP93XX_RTC_SWCOMP);
    if (preload)
// preload = (comp & EP93XX_RTC_SWCOMP_INT_MASK)
    >> EP93XX_RTC_SWCOMP_INT_SHIFT;
    if (delete)
// delete = (comp & EP93XX_RTC_SWCOMP_DEL_MASK)
    >> EP93XX_RTC_SWCOMP_DEL_SHIFT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ep93xx_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct ep93xx_rtc *ep93xx_rtc = dev_get_drvdata(dev);
    unsigned long time;
    time = readl(ep93xx_rtc.mmio_base + EP93XX_RTC_DATA);
    rtc_time64_to_tm(time, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ep93xx_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct ep93xx_rtc *ep93xx_rtc = dev_get_drvdata(dev);
    let mut secs: c_ulong = rtc_tm_to_time64(tm);
    writel(secs + 1, ep93xx_rtc.mmio_base + EP93XX_RTC_LOAD);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_rtc_proc(dev: *mut device, seq: *mut seq_file) -> c_int {
    static int ep93xx_rtc_proc(struct device *dev, struct seq_file *seq)
    {
    unsigned short preload, delete;
    ep93xx_rtc_get_swcomp(dev, &preload, &delete);
    seq_printf(seq, "preload\t\t: %d\n", preload);
    seq_printf(seq, "delete\t\t: %d\n", delete);
    return 0;
    }
    static const struct rtc_class_ops ep93xx_rtc_ops = {
    .read_time	= ep93xx_rtc_read_time,
    .set_time	= ep93xx_rtc_set_time,
    .proc		= ep93xx_rtc_proc,
    };
    static ssize_t comp_preload_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    unsigned short preload;
    ep93xx_rtc_get_swcomp(dev.parent, &preload, core::ptr::null_mut());
    return sprintf(buf, "%d\n", preload);
    }
    static DEVICE_ATTR_RO(comp_preload);
    static ssize_t comp_delete_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    unsigned short delete;
    ep93xx_rtc_get_swcomp(dev.parent, core::ptr::null_mut(), &delete);
    return sprintf(buf, "%d\n", delete);
    }
    static DEVICE_ATTR_RO(comp_delete);
    static struct attribute *ep93xx_rtc_attrs[] = {
    &dev_attr_comp_preload.attr,
    &dev_attr_comp_delete.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group ep93xx_rtc_sysfs_files = {
    .attrs	= ep93xx_rtc_attrs,
    };
#[no_mangle]
unsafe extern "C" fn ep93xx_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int ep93xx_rtc_probe(struct platform_device *pdev)
    {
    struct ep93xx_rtc *ep93xx_rtc;
    struct rtc_device *rtc;
    int err;
    ep93xx_rtc = devm_kzalloc(&pdev.dev, sizeof(*ep93xx_rtc), GFP_KERNEL);
    if (!ep93xx_rtc)
    return -ENOMEM;
    ep93xx_rtc.mmio_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ep93xx_rtc.mmio_base))
    return PTR_ERR(ep93xx_rtc.mmio_base);
    platform_set_drvdata(pdev, ep93xx_rtc);
    rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    rtc.ops = &ep93xx_rtc_ops;
    rtc.range_max = U32_MAX;
    err = rtc_add_group(rtc, &ep93xx_rtc_sysfs_files);
    if (err)
    return err;
    return devm_rtc_register_device(rtc);
    }
    static const struct of_device_id ep93xx_rtc_of_ids[] = {
    { .compatible = "cirrus,ep9301-rtc" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ep93xx_rtc_of_ids);
    static struct platform_driver ep93xx_rtc_driver = {
    .driver		= {
    .name	= "ep93xx-rtc",
    .of_match_table = ep93xx_rtc_of_ids,
    },
    .probe		= ep93xx_rtc_probe,
    };
    module_platform_driver(ep93xx_rtc_driver);
    MODULE_AUTHOR("Alessandro Zummo <a.zummo@towertech.it>");
    MODULE_DESCRIPTION("EP93XX RTC driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:ep93xx-rtc");
