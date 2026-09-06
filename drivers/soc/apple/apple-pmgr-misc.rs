//! Automatically rewritten from C to Rust
//! Source: drivers/soc/apple/apple-pmgr-misc.c
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Apple SoC PMGR device power state driver
//
// Copyright The Asahi Linux Contributors
//

pub const APPLE_CLKGEN_PSTATE: c_int = 0;

pub const DCS_DEV_PSTATE_MIN_T600X: c_int = 7;
pub const SYS_DEV_PSTATE_SUSPEND: c_int = 1;
    enum sys_device {
    DEV_FABRIC,
    DEV_DCS,
    DEV_MAX,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_pmgr_sys_device {
    pub base: *mut void __iomem,
    pub active_state: u32,
    pub suspend_state: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_pmgr_misc_hw {
    pub dev_min_ps: [u32; DEV_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_pmgr_misc {
    pub dev: *mut device,
    pub devices: [apple_pmgr_sys_device; DEV_MAX],
}

    static void apple_pmgr_sys_dev_set_pstate(struct apple_pmgr_misc *misc,
    enum sys_device dev, bool active)
    {
    u32 pstate;
    u32 val;
    if (!misc.devices[dev].base)
    return;
    if (active)
    pstate = misc.devices[dev].active_state;
    else
    pstate = misc.devices[dev].suspend_state;
    dev_dbg(misc.dev, "set %d ps to pstate %d\n", dev, pstate);
    val = readl_relaxed(misc.devices[dev].base + APPLE_CLKGEN_PSTATE);
    FIELD_MODIFY(APPLE_CLKGEN_PSTATE_DESIRED, &val, pstate);
    writel_relaxed(val, misc.devices[dev].base + APPLE_CLKGEN_PSTATE);
    }
#[no_mangle]
unsafe extern "C" fn apple_pmgr_misc_suspend_noirq(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused apple_pmgr_misc_suspend_noirq(struct device *dev)
    {
    struct apple_pmgr_misc *misc = dev_get_drvdata(dev);
    int i;
    for (i = 0; i < DEV_MAX; i++)
    apple_pmgr_sys_dev_set_pstate(misc, i, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_pmgr_misc_resume_noirq(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused apple_pmgr_misc_resume_noirq(struct device *dev)
    {
    struct apple_pmgr_misc *misc = dev_get_drvdata(dev);
    int i;
    for (i = 0; i < DEV_MAX; i++)
    apple_pmgr_sys_dev_set_pstate(misc, i, true);
    return 0;
    }
    static int apple_pmgr_init_device(struct apple_pmgr_misc *misc,
    const struct apple_pmgr_misc_hw *hw,
    enum sys_device dev,
    const char *device_name)
    {
    void __iomem *base;
    char name[32];
    u32 val;
    snprintf(name, sizeof(name), "%s-ps", device_name);
    base = devm_platform_ioremap_resource_byname(
    to_platform_device(misc.dev), name);
    if (IS_ERR(base))
    return PTR_ERR(base);
    val = readl_relaxed(base + APPLE_CLKGEN_PSTATE);
    misc.devices[dev].base = base;
    misc.devices[dev].active_state =
    FIELD_GET(APPLE_CLKGEN_PSTATE_DESIRED, val);
    misc.devices[dev].suspend_state = hw.dev_min_ps[dev];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_pmgr_misc_probe(pdev: *mut platform_device) -> c_int {
    static int apple_pmgr_misc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct apple_pmgr_misc_hw *hw;
    struct apple_pmgr_misc *misc;
    int ret;
    misc = devm_kzalloc(dev, sizeof(*misc), GFP_KERNEL);
    if (!misc)
    return -ENOMEM;
    misc.dev = dev;
    hw = of_device_get_match_data(dev);
    if (!hw)
    return -EINVAL;
    ret = apple_pmgr_init_device(misc, hw, DEV_FABRIC, "fabric");
    if (ret)
    return ret;
    ret = apple_pmgr_init_device(misc, hw, DEV_DCS, "dcs");
    if (ret)
    return ret;
    platform_set_drvdata(pdev, misc);
    return 0;
    }
    static const struct apple_pmgr_misc_hw apple_pmgr_misc_hw_t600x = {
    .dev_min_ps = {
    [DEV_FABRIC] = SYS_DEV_PSTATE_SUSPEND,
    [DEV_DCS] = DCS_DEV_PSTATE_MIN_T600X,
    },
    };
    static const struct apple_pmgr_misc_hw apple_pmgr_misc_hw_t602x = {
    .dev_min_ps = {
    [DEV_FABRIC] = SYS_DEV_PSTATE_SUSPEND,
    [DEV_DCS] = SYS_DEV_PSTATE_SUSPEND,
    },
    };
    static const struct of_device_id apple_pmgr_misc_of_match[] = {
    { .compatible = "apple,t6000-pmgr-misc", .data = &apple_pmgr_misc_hw_t600x },
    { .compatible = "apple,t6020-pmgr-misc", .data = &apple_pmgr_misc_hw_t602x },
    {}
    };
    MODULE_DEVICE_TABLE(of, apple_pmgr_misc_of_match);
    static const struct dev_pm_ops apple_pmgr_misc_pm_ops = {
    SET_NOIRQ_SYSTEM_SLEEP_PM_OPS(apple_pmgr_misc_suspend_noirq,
    apple_pmgr_misc_resume_noirq)
    };
    static struct platform_driver apple_pmgr_misc_driver = {
    .probe = apple_pmgr_misc_probe,
    .driver = {
    .name = "apple-pmgr-misc",
    .of_match_table = apple_pmgr_misc_of_match,
    .pm = pm_ptr(&apple_pmgr_misc_pm_ops),
    },
    };
    MODULE_AUTHOR("Hector Martin <marcan@marcan.st>");
    MODULE_DESCRIPTION("PMGR misc driver for Apple SoCs");
    MODULE_LICENSE("GPL");
    module_platform_driver(apple_pmgr_misc_driver);
