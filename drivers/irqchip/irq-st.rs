//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-st.c
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
// Copyright (C) 2014 STMicroelectronics – All Rights Reserved
//
// Author: Lee Jones <lee.jones@linaro.org>
//
// This is a re-write of Christophe Kerello's PMU driver.
//

pub const STIH407_SYSCFG_5102: c_uint = 0x198;
pub const ST_A9_IRQ_MASK: c_uint = 0x001FFFFF;
pub const ST_A9_IRQ_MAX_CHANS: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_irq_syscfg {
    pub regmap: *mut regmap,
    pub syscfg: c_uint,
    pub config: c_uint,
    pub ext_inverted: bool,
}

    static const struct of_device_id st_irq_syscfg_match[] = {
    {
    .compatible = "st,stih407-irq-syscfg",
    .data = (void *)STIH407_SYSCFG_5102,
    },
    {}
    };
    static int st_irq_xlate(struct platform_device *pdev,
    int device, int channel, bool irq)
    {
    struct st_irq_syscfg *ddata = dev_get_drvdata(&pdev.dev);
// Set the device enable bit.
    switch (device) {
    case ST_IRQ_SYSCFG_EXT_0:
    ddata.config |= ST_A9_IRQ_EN_EXT_0;
    break;
    case ST_IRQ_SYSCFG_EXT_1:
    ddata.config |= ST_A9_IRQ_EN_EXT_1;
    break;
    case ST_IRQ_SYSCFG_EXT_2:
    ddata.config |= ST_A9_IRQ_EN_EXT_2;
    break;
    case ST_IRQ_SYSCFG_CTI_0:
    ddata.config |= ST_A9_IRQ_EN_CTI_0;
    break;
    case ST_IRQ_SYSCFG_CTI_1:
    ddata.config |= ST_A9_IRQ_EN_CTI_1;
    break;
    case ST_IRQ_SYSCFG_PMU_0:
    ddata.config |= ST_A9_IRQ_EN_PMU_0;
    break;
    case ST_IRQ_SYSCFG_PMU_1:
    ddata.config |= ST_A9_IRQ_EN_PMU_1;
    break;
    case ST_IRQ_SYSCFG_pl310_L2:
    ddata.config |= ST_A9_IRQ_EN_PL310_L2;
    break;
    case ST_IRQ_SYSCFG_DISABLED:
    return 0;
    default:
    dev_err(&pdev.dev, "Unrecognised device %d\n", device);
    return -EINVAL;
    }
// Select IRQ/FIQ channel for device.
    ddata.config |= irq ?
    ST_A9_IRQ_N_SEL(device, channel) :
    ST_A9_FIQ_N_SEL(device, channel);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_irq_syscfg_enable(pdev: *mut platform_device) -> c_int {
    static int st_irq_syscfg_enable(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct st_irq_syscfg *ddata = dev_get_drvdata(&pdev.dev);
    int channels, ret, i;
    u32 device, invert;
    channels = of_property_count_u32_elems(np, "st,irq-device");
    if (channels != ST_A9_IRQ_MAX_CHANS) {
    dev_err(&pdev.dev, "st,enable-irq-device must have 2 elems\n");
    return -EINVAL;
    }
    channels = of_property_count_u32_elems(np, "st,fiq-device");
    if (channels != ST_A9_IRQ_MAX_CHANS) {
    dev_err(&pdev.dev, "st,enable-fiq-device must have 2 elems\n");
    return -EINVAL;
    }
    for (i = 0; i < ST_A9_IRQ_MAX_CHANS; i++) {
    of_property_read_u32_index(np, "st,irq-device", i, &device);
    ret = st_irq_xlate(pdev, device, i, true);
    if (ret)
    return ret;
    of_property_read_u32_index(np, "st,fiq-device", i, &device);
    ret = st_irq_xlate(pdev, device, i, false);
    if (ret)
    return ret;
    }
// External IRQs may be inverted.
    of_property_read_u32(np, "st,invert-ext", &invert);
    ddata.config |= ST_A9_EXTIRQ_INV_SEL(invert);
    return regmap_update_bits(ddata.regmap, ddata.syscfg,
    ST_A9_IRQ_MASK, ddata.config);
    }
#[no_mangle]
unsafe extern "C" fn st_irq_syscfg_probe(pdev: *mut platform_device) -> c_int {
    static int st_irq_syscfg_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct st_irq_syscfg *ddata;
    ddata = devm_kzalloc(&pdev.dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata)
    return -ENOMEM;
    ddata.syscfg = (unsigned int) device_get_match_data(&pdev.dev);
    ddata.regmap = syscon_regmap_lookup_by_phandle(np, "st,syscfg");
    if (IS_ERR(ddata.regmap)) {
    dev_err(&pdev.dev, "syscfg phandle missing\n");
    return PTR_ERR(ddata.regmap);
    }
    dev_set_drvdata(&pdev.dev, ddata);
    return st_irq_syscfg_enable(pdev);
    }
#[no_mangle]
unsafe extern "C" fn st_irq_syscfg_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused st_irq_syscfg_resume(struct device *dev)
    {
    struct st_irq_syscfg *ddata = dev_get_drvdata(dev);
    return regmap_update_bits(ddata.regmap, ddata.syscfg,
    ST_A9_IRQ_MASK, ddata.config);
    }
    static SIMPLE_DEV_PM_OPS(st_irq_syscfg_pm_ops, core::ptr::null_mut(), st_irq_syscfg_resume);
    static struct platform_driver st_irq_syscfg_driver = {
    .driver = {
    .name = "st_irq_syscfg",
    .pm = &st_irq_syscfg_pm_ops,
    .of_match_table = st_irq_syscfg_match,
    },
    .probe = st_irq_syscfg_probe,
    };
#[no_mangle]
unsafe extern "C" fn st_irq_syscfg_init() -> int __init {
    static int __init st_irq_syscfg_init(void)
    {
    return platform_driver_register(&st_irq_syscfg_driver);
    }
    core_initcall(st_irq_syscfg_init);
