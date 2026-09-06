//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/max77705.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Maxim MAX77705 PMIC core driver
//
// Copyright (C) 2025 Dzmitry Sankouski <dsankouski@gmail.com>
//

    static struct mfd_cell max77705_devs[] = {
    MFD_CELL_OF("max77705-rgb", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "maxim,max77705-rgb"),
    MFD_CELL_OF("max77705-charger", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "maxim,max77705-charger"),
    MFD_CELL_OF("max77705-haptic", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "maxim,max77705-haptic"),
    };
    static const struct regmap_range max77705_readable_ranges[] = {
    regmap_reg_range(MAX77705_PMIC_REG_PMICID1,		MAX77705_PMIC_REG_BSTOUT_MASK),
    regmap_reg_range(MAX77705_PMIC_REG_INTSRC,		MAX77705_PMIC_REG_RESERVED_29),
    regmap_reg_range(MAX77705_PMIC_REG_BOOSTCONTROL1,	MAX77705_PMIC_REG_BOOSTCONTROL1),
    regmap_reg_range(MAX77705_PMIC_REG_MCONFIG,		MAX77705_PMIC_REG_MCONFIG2),
    regmap_reg_range(MAX77705_PMIC_REG_FORCE_EN_MASK,	MAX77705_PMIC_REG_FORCE_EN_MASK),
    regmap_reg_range(MAX77705_PMIC_REG_BOOSTCONTROL1,	MAX77705_PMIC_REG_BOOSTCONTROL1),
    regmap_reg_range(MAX77705_PMIC_REG_BOOSTCONTROL2,	MAX77705_PMIC_REG_BOOSTCONTROL2),
    regmap_reg_range(MAX77705_PMIC_REG_SW_RESET,		MAX77705_PMIC_REG_USBC_RESET),
    };
    static const struct regmap_range max77705_writable_ranges[] = {
    regmap_reg_range(MAX77705_PMIC_REG_MAINCTRL1,		MAX77705_PMIC_REG_BSTOUT_MASK),
    regmap_reg_range(MAX77705_PMIC_REG_INTSRC,		MAX77705_PMIC_REG_RESERVED_29),
    regmap_reg_range(MAX77705_PMIC_REG_BOOSTCONTROL1,	MAX77705_PMIC_REG_BOOSTCONTROL1),
    regmap_reg_range(MAX77705_PMIC_REG_MCONFIG,		MAX77705_PMIC_REG_MCONFIG2),
    regmap_reg_range(MAX77705_PMIC_REG_FORCE_EN_MASK,	MAX77705_PMIC_REG_FORCE_EN_MASK),
    regmap_reg_range(MAX77705_PMIC_REG_BOOSTCONTROL1,	MAX77705_PMIC_REG_BOOSTCONTROL1),
    regmap_reg_range(MAX77705_PMIC_REG_BOOSTCONTROL2,	MAX77705_PMIC_REG_BOOSTCONTROL2),
    regmap_reg_range(MAX77705_PMIC_REG_SW_RESET,		MAX77705_PMIC_REG_USBC_RESET),
    };
    static const struct regmap_access_table max77705_readable_table = {
    .yes_ranges = max77705_readable_ranges,
    .n_yes_ranges = ARRAY_SIZE(max77705_readable_ranges),
    };
    static const struct regmap_access_table max77705_writable_table = {
    .yes_ranges = max77705_writable_ranges,
    .n_yes_ranges = ARRAY_SIZE(max77705_writable_ranges),
    };
    static const struct regmap_config max77705_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .rd_table = &max77705_readable_table,
    .wr_table = &max77705_writable_table,
    .max_register = MAX77705_PMIC_REG_USBC_RESET,
    };
    static const struct regmap_irq max77705_irqs[] = {
    { .mask = MAX77705_SRC_IRQ_CHG, },
    { .mask = MAX77705_SRC_IRQ_TOP, },
    { .mask = MAX77705_SRC_IRQ_FG, },
    { .mask = MAX77705_SRC_IRQ_USBC, },
    };
    static const struct regmap_irq_chip max77705_irq_chip = {
    .name		= "max77705",
    .status_base	= MAX77705_PMIC_REG_INTSRC,
    .ack_base	= MAX77705_PMIC_REG_INTSRC,
    .mask_base	= MAX77705_PMIC_REG_INTSRC_MASK,
    .num_regs	= 1,
    .irqs		= max77705_irqs,
    .num_irqs	= ARRAY_SIZE(max77705_irqs),
    };
#[no_mangle]
unsafe extern "C" fn max77705_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int max77705_i2c_probe(struct i2c_client *i2c)
    {
    struct device *dev = &i2c.dev;
    struct max77693_dev *max77705;
    struct regmap_irq_chip_data *irq_data;
    struct irq_domain *domain;
    enum max77705_hw_rev pmic_rev;
    unsigned int pmic_rev_value;
    int ret;
    max77705 = devm_kzalloc(dev, sizeof(*max77705), GFP_KERNEL);
    if (!max77705)
    return -ENOMEM;
    max77705.i2c = i2c;
    max77705.type = TYPE_MAX77705;
    i2c_set_clientdata(i2c, max77705);
    max77705.regmap = devm_regmap_init_i2c(i2c, &max77705_regmap_config);
    if (IS_ERR(max77705.regmap))
    return PTR_ERR(max77705.regmap);
    ret = regmap_read(max77705.regmap, MAX77705_PMIC_REG_PMICREV, &pmic_rev_value);
    if (ret < 0)
    return -ENODEV;
    pmic_rev = pmic_rev_value & MAX77705_REVISION_MASK;
    if (pmic_rev != MAX77705_PASS3)
    return dev_err_probe(dev, -ENODEV, "Rev.0x%x is not tested\n", pmic_rev);
// Active Discharge Enable
    regmap_update_bits(max77705.regmap, MAX77705_PMIC_REG_MAINCTRL1, 1, 1);
    ret = devm_regmap_add_irq_chip(dev, max77705.regmap,
    i2c.irq,
    IRQF_ONESHOT, 0,
    &max77705_irq_chip,
    &irq_data);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to add IRQ chip\n");
    domain = regmap_irq_get_domain(irq_data);
    ret = devm_mfd_add_devices(dev, PLATFORM_DEVID_NONE,
    max77705_devs, ARRAY_SIZE(max77705_devs),
    core::ptr::null_mut(), 0, domain);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register child devices\n");
    ret = devm_device_init_wakeup(dev);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to init wakeup\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max77705_suspend(dev: *mut device) -> c_int {
    static int max77705_suspend(struct device *dev)
    {
    struct i2c_client *i2c = to_i2c_client(dev);
    disable_irq(i2c.irq);
    if (device_may_wakeup(dev))
    enable_irq_wake(i2c.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max77705_resume(dev: *mut device) -> c_int {
    static int max77705_resume(struct device *dev)
    {
    struct i2c_client *i2c = to_i2c_client(dev);
    if (device_may_wakeup(dev))
    disable_irq_wake(i2c.irq);
    enable_irq(i2c.irq);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(max77705_pm_ops, max77705_suspend, max77705_resume);
    static const struct of_device_id max77705_i2c_of_match[] = {
    { .compatible = "maxim,max77705" },
    { }
    };
    MODULE_DEVICE_TABLE(of, max77705_i2c_of_match);
    static struct i2c_driver max77705_i2c_driver = {
    .driver = {
    .name			= "max77705",
    .of_match_table		= max77705_i2c_of_match,
    .pm			= pm_sleep_ptr(&max77705_pm_ops),
    },
    .probe = max77705_i2c_probe
    };
    module_i2c_driver(max77705_i2c_driver);
    MODULE_DESCRIPTION("Maxim MAX77705 PMIC core driver");
    MODULE_AUTHOR("Dzmitry Sankouski <dsankouski@gmail.com>");
    MODULE_LICENSE("GPL");
