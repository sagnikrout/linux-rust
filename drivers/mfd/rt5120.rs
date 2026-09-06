//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/rt5120.c
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
// Copyright (C) 2022 Richtek Technology Corp.
// Author: ChiYuan Huang <cy_huang@richtek.com>
//

pub const RT5120_REG_INTENABLE: c_uint = 0x1D;
pub const RT5120_REG_INTSTAT: c_uint = 0x1E;
pub const RT5120_REG_FZCMODE: c_uint = 0x44;
pub const RT5120_INT_HOTDIE: c_int = 0;
pub const RT5120_INT_PWRKEY_REL: c_int = 5;
pub const RT5120_INT_PWRKEY_PRESS: c_int = 6;
    static const struct regmap_range rt5120_rd_yes_ranges[] = {
    regmap_reg_range(0x03, 0x13),
    regmap_reg_range(0x1c, 0x20),
    regmap_reg_range(0x44, 0x44),
    };
    static const struct regmap_range rt5120_wr_yes_ranges[] = {
    regmap_reg_range(0x06, 0x13),
    regmap_reg_range(0x1c, 0x20),
    regmap_reg_range(0x44, 0x44),
    };
    static const struct regmap_access_table rt5120_rd_table = {
    .yes_ranges = rt5120_rd_yes_ranges,
    .n_yes_ranges = ARRAY_SIZE(rt5120_rd_yes_ranges),
    };
    static const struct regmap_access_table rt5120_wr_table = {
    .yes_ranges = rt5120_wr_yes_ranges,
    .n_yes_ranges = ARRAY_SIZE(rt5120_wr_yes_ranges),
    };
    static const struct regmap_config rt5120_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = RT5120_REG_FZCMODE,
    .wr_table = &rt5120_wr_table,
    .rd_table = &rt5120_rd_table,
    };
    static const struct regmap_irq rt5120_irqs[] = {
    REGMAP_IRQ_REG_LINE(RT5120_INT_HOTDIE, 8),
    REGMAP_IRQ_REG_LINE(RT5120_INT_PWRKEY_REL, 8),
    REGMAP_IRQ_REG_LINE(RT5120_INT_PWRKEY_PRESS, 8),
    };
    static const struct regmap_irq_chip rt5120_irq_chip = {
    .name = "rt5120-pmic",
    .status_base = RT5120_REG_INTSTAT,
    .unmask_base = RT5120_REG_INTENABLE,
    .ack_base = RT5120_REG_INTSTAT,
    .use_ack = true,
    .num_regs = 1,
    .irqs = rt5120_irqs,
    .num_irqs = ARRAY_SIZE(rt5120_irqs),
    };
    static const struct resource rt5120_regulator_resources[] = {
    DEFINE_RES_IRQ(RT5120_INT_HOTDIE),
    };
    static const struct resource rt5120_pwrkey_resources[] = {
    DEFINE_RES_IRQ_NAMED(RT5120_INT_PWRKEY_PRESS, "pwrkey-press"),
    DEFINE_RES_IRQ_NAMED(RT5120_INT_PWRKEY_REL, "pwrkey-release"),
    };
    static const struct mfd_cell rt5120_devs[] = {
    MFD_CELL_RES("rt5120-regulator", rt5120_regulator_resources),
    MFD_CELL_OF("rt5120-pwrkey", rt5120_pwrkey_resources, core::ptr::null_mut(), 0, 0, "richtek,rt5120-pwrkey"),
    };
#[no_mangle]
unsafe extern "C" fn rt5120_probe(i2c: *mut i2c_client) -> c_int {
    static int rt5120_probe(struct i2c_client *i2c)
    {
    struct device *dev = &i2c.dev;
    struct regmap *regmap;
    struct regmap_irq_chip_data *irq_data;
    int ret;
    regmap = devm_regmap_init_i2c(i2c, &rt5120_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap),
    "Failed to init regmap\n");
    ret = devm_regmap_add_irq_chip(dev, regmap, i2c.irq, IRQF_ONESHOT, 0,
    &rt5120_irq_chip, &irq_data);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to add IRQ chip\n");
    return devm_mfd_add_devices(dev, PLATFORM_DEVID_AUTO, rt5120_devs,
    ARRAY_SIZE(rt5120_devs), core::ptr::null_mut(), 0,
    regmap_irq_get_domain(irq_data));
    }
    static const struct of_device_id rt5120_device_match_table[] = {
    { .compatible = "richtek,rt5120" },
    {}
    };
    MODULE_DEVICE_TABLE(of, rt5120_device_match_table);
    static struct i2c_driver rt5120_driver = {
    .driver = {
    .name = "rt5120",
    .of_match_table = rt5120_device_match_table,
    },
    .probe = rt5120_probe,
    };
    module_i2c_driver(rt5120_driver);
    MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>");
    MODULE_DESCRIPTION("Richtek RT5120 I2C driver");
    MODULE_LICENSE("GPL v2");
