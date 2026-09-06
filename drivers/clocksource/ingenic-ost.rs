//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/ingenic-ost.c
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
// JZ47xx SoCs TCU Operating System Timer driver
//
// Copyright (C) 2016 Maarten ter Huurne <maarten@treewalker.org>
// Copyright (C) 2020 Paul Cercueil <paul@crapouillou.net>
//

pub const TCU_OST_TCSR_MASK: c_uint = 0xffc0;

pub const TCU_OST_CHANNEL: c_int = 15;
//
// The TCU_REG_OST_CNT{L,R} from <linux/mfd/ingenic-tcu.h> are only for the
// regmap; these are for use with the __iomem pointer.
//
pub const OST_REG_CNTL: c_uint = 0x4;
pub const OST_REG_CNTH: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_ost_soc_info {
    pub is64bit: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_ost {
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub cs: clocksource,
}

    static struct ingenic_ost *ingenic_ost;
#[no_mangle]
unsafe extern "C" fn ingenic_ost_read_cntl() -> u64 notrace {
    static u64 notrace ingenic_ost_read_cntl(void)
    {
// Read using __iomem pointer instead of regmap to avoid locking
    return readl(ingenic_ost.regs + OST_REG_CNTL);
    }
#[no_mangle]
unsafe extern "C" fn ingenic_ost_read_cnth() -> u64 notrace {
    static u64 notrace ingenic_ost_read_cnth(void)
    {
// Read using __iomem pointer instead of regmap to avoid locking
    return readl(ingenic_ost.regs + OST_REG_CNTH);
    }
#[no_mangle]
unsafe extern "C" fn ingenic_ost_clocksource_readl(cs: *mut clocksource) -> u64 notrace {
    static u64 notrace ingenic_ost_clocksource_readl(struct clocksource *cs)
    {
    return ingenic_ost_read_cntl();
    }
#[no_mangle]
unsafe extern "C" fn ingenic_ost_clocksource_readh(cs: *mut clocksource) -> u64 notrace {
    static u64 notrace ingenic_ost_clocksource_readh(struct clocksource *cs)
    {
    return ingenic_ost_read_cnth();
    }
#[no_mangle]
unsafe extern "C" fn ingenic_ost_probe(pdev: *mut platform_device) -> int __init {
    static int __init ingenic_ost_probe(struct platform_device *pdev)
    {
    const struct ingenic_ost_soc_info *soc_info;
    struct device *dev = &pdev.dev;
    struct ingenic_ost *ost;
    struct clocksource *cs;
    struct regmap *map;
    unsigned long rate;
    int err;
    soc_info = device_get_match_data(dev);
    if (!soc_info)
    return -EINVAL;
    ost = devm_kzalloc(dev, sizeof(*ost), GFP_KERNEL);
    if (!ost)
    return -ENOMEM;
    ingenic_ost = ost;
    ost.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ost.regs))
    return PTR_ERR(ost.regs);
    map = device_node_to_regmap(dev.parent.of_node);
    if (IS_ERR(map)) {
    dev_err(dev, "regmap not found");
    return PTR_ERR(map);
    }
    ost.clk = devm_clk_get_enabled(dev, "ost");
    if (IS_ERR(ost.clk))
    return PTR_ERR(ost.clk);
// Clear counter high/low registers
    if (soc_info.is64bit)
    regmap_write(map, TCU_REG_OST_CNTL, 0);
    regmap_write(map, TCU_REG_OST_CNTH, 0);
// Don't reset counter at compare value.
    regmap_update_bits(map, TCU_REG_OST_TCSR,
    TCU_OST_TCSR_MASK, TCU_OST_TCSR_CNT_MD);
    rate = clk_get_rate(ost.clk);
// Enable OST TCU channel
    regmap_write(map, TCU_REG_TESR, BIT(TCU_OST_CHANNEL));
    cs = &ost.cs;
    cs.name	= "ingenic-ost";
    cs.rating	= 320;
    cs.flags	= CLOCK_SOURCE_IS_CONTINUOUS;
    cs.mask	= CLOCKSOURCE_MASK(32);
    if (soc_info.is64bit)
    cs.read = ingenic_ost_clocksource_readl;
    else
    cs.read = ingenic_ost_clocksource_readh;
    err = clocksource_register_hz(cs, rate);
    if (err) {
    dev_err(dev, "clocksource registration failed");
    return err;
    }
    if (soc_info.is64bit)
    sched_clock_register(ingenic_ost_read_cntl, 32, rate);
    else
    sched_clock_register(ingenic_ost_read_cnth, 32, rate);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_ost_suspend(dev: *mut device) -> c_int {
    static int ingenic_ost_suspend(struct device *dev)
    {
    struct ingenic_ost *ost = dev_get_drvdata(dev);
    clk_disable(ost.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_ost_resume(dev: *mut device) -> c_int {
    static int ingenic_ost_resume(struct device *dev)
    {
    struct ingenic_ost *ost = dev_get_drvdata(dev);
    return clk_enable(ost.clk);
    }
    static const struct dev_pm_ops ingenic_ost_pm_ops = {
// _noirq: We want the OST clock to be gated last / ungated first
    .suspend_noirq = ingenic_ost_suspend,
    .resume_noirq  = ingenic_ost_resume,
    };
    static const struct ingenic_ost_soc_info jz4725b_ost_soc_info = {
    .is64bit = false,
    };
    static const struct ingenic_ost_soc_info jz4760b_ost_soc_info = {
    .is64bit = true,
    };
    static const struct of_device_id ingenic_ost_of_match[] = {
    { .compatible = "ingenic,jz4725b-ost", .data = &jz4725b_ost_soc_info, },
    { .compatible = "ingenic,jz4760b-ost", .data = &jz4760b_ost_soc_info, },
    { .compatible = "ingenic,jz4770-ost", .data = &jz4760b_ost_soc_info, },
    { }
    };
    static struct platform_driver ingenic_ost_driver = {
    .driver = {
    .name = "ingenic-ost",
    .pm = pm_sleep_ptr(&ingenic_ost_pm_ops),
    .of_match_table = ingenic_ost_of_match,
    },
    };
    builtin_platform_driver_probe(ingenic_ost_driver, ingenic_ost_probe);
