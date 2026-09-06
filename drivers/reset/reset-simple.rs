//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-simple.c
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
// Simple Reset Controller Driver
//
// Copyright (C) 2017 Pengutronix, Philipp Zabel <kernel@pengutronix.de>
//
// Based on Allwinner SoCs Reset Controller driver
//
// Copyright 2013 Maxime Ripard
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//

    static inline struct reset_simple_data *
    to_reset_simple_data(struct reset_controller_dev *rcdev)
    {
    return container_of(rcdev, struct reset_simple_data, rcdev);
    }
    static int reset_simple_update(struct reset_controller_dev *rcdev,
    unsigned long id, bool assert)
    {
    struct reset_simple_data *data = to_reset_simple_data(rcdev);
    let mut reg_width: c_int = sizeof(u32);
    let mut bank: c_int = id / (reg_width * BITS_PER_BYTE);
    let mut offset: c_int = id % (reg_width * BITS_PER_BYTE);
    unsigned long flags;
    u32 reg;
    spin_lock_irqsave(&data.lock, flags);
    reg = readl(data.membase + (bank * reg_width));
    if (assert ^ data.active_low)
    reg |= BIT(offset);
    else
    reg &= ~BIT(offset);
    writel(reg, data.membase + (bank * reg_width));
    spin_unlock_irqrestore(&data.lock, flags);
    return 0;
    }
    static int reset_simple_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return reset_simple_update(rcdev, id, true);
    }
    static int reset_simple_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return reset_simple_update(rcdev, id, false);
    }
    static int reset_simple_reset(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct reset_simple_data *data = to_reset_simple_data(rcdev);
    int ret;
    if (!data.reset_us)
    return -ENOTSUPP;
    ret = reset_simple_assert(rcdev, id);
    if (ret)
    return ret;
    usleep_range(data.reset_us, data.reset_us * 2);
    return reset_simple_deassert(rcdev, id);
    }
    static int reset_simple_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct reset_simple_data *data = to_reset_simple_data(rcdev);
    let mut reg_width: c_int = sizeof(u32);
    let mut bank: c_int = id / (reg_width * BITS_PER_BYTE);
    let mut offset: c_int = id % (reg_width * BITS_PER_BYTE);
    u32 reg;
    reg = readl(data.membase + (bank * reg_width));
    return !(reg & BIT(offset)) ^ !data.status_active_low;
    }
    const struct reset_control_ops reset_simple_ops = {
    .assert		= reset_simple_assert,
    .deassert	= reset_simple_deassert,
    .reset		= reset_simple_reset,
    .status		= reset_simple_status,
    };
    EXPORT_SYMBOL_GPL(reset_simple_ops);
//
// struct reset_simple_devdata - simple reset controller properties
// @reg_offset: offset between base address and first reset register.
// @nr_resets: number of resets. If not set, default to resource size in bits.
// @active_low: if true, bits are cleared to assert the reset. Otherwise, bits
// are set to assert the reset.
// @status_active_low: if true, bits read back as cleared while the reset is
// asserted. Otherwise, bits read back as set while the
// reset is asserted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reset_simple_devdata {
    pub reg_offset: u32,
    pub nr_resets: u32,
    pub active_low: bool,
    pub status_active_low: bool,
}

pub const SOCFPGA_NR_BANKS: c_int = 8;
    static const struct reset_simple_devdata reset_simple_socfpga = {
    .reg_offset = 0x20,
    .nr_resets = SOCFPGA_NR_BANKS * 32,
    .status_active_low = true,
    };
    static const struct reset_simple_devdata reset_simple_active_low = {
    .active_low = true,
    .status_active_low = true,
    };
    static const struct of_device_id reset_simple_dt_ids[] = {
    { .compatible = "altr,stratix10-rst-mgr",
    .data = &reset_simple_socfpga },
    { .compatible = "st,stm32-rcc", },
    { .compatible = "allwinner,sun6i-a31-clock-reset",
    .data = &reset_simple_active_low },
    { .compatible = "zte,zx296718-reset",
    .data = &reset_simple_active_low },
    { .compatible = "aspeed,ast2400-lpc-reset" },
    { .compatible = "aspeed,ast2500-lpc-reset" },
    { .compatible = "aspeed,ast2600-lpc-reset" },
    { .compatible = "bitmain,bm1880-reset",
    .data = &reset_simple_active_low },
    { .compatible = "brcm,bcm4908-misc-pcie-reset",
    .data = &reset_simple_active_low },
    { .compatible = "snps,dw-high-reset" },
    { .compatible = "snps,dw-low-reset",
    .data = &reset_simple_active_low },
    { .compatible = "sophgo,cv1800b-reset",
    .data = &reset_simple_active_low },
    { .compatible = "sophgo,sg2042-reset",
    .data = &reset_simple_active_low },
    { /* sentinel */ },
    };
#[no_mangle]
unsafe extern "C" fn reset_simple_probe(pdev: *mut platform_device) -> c_int {
    static int reset_simple_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct reset_simple_devdata *devdata;
    struct reset_simple_data *data;
    void __iomem *membase;
    struct resource *res;
    let mut reg_offset: u32 = 0;
    devdata = of_device_get_match_data(dev);
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    membase = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(membase))
    return PTR_ERR(membase);
    spin_lock_init(&data.lock);
    data.membase = membase;
    data.rcdev.owner = THIS_MODULE;
    data.rcdev.nr_resets = resource_size(res) * BITS_PER_BYTE;
    data.rcdev.ops = &reset_simple_ops;
    data.rcdev.of_node = dev.of_node;
    if (devdata) {
    reg_offset = devdata.reg_offset;
    if (devdata.nr_resets)
    data.rcdev.nr_resets = devdata.nr_resets;
    data.active_low = devdata.active_low;
    data.status_active_low = devdata.status_active_low;
    }
    data.membase += reg_offset;
    return devm_reset_controller_register(dev, &data.rcdev);
    }
    static struct platform_driver reset_simple_driver = {
    .probe	= reset_simple_probe,
    .driver = {
    .name		= "simple-reset",
    .of_match_table	= reset_simple_dt_ids,
    },
    };
    builtin_platform_driver(reset_simple_driver);
