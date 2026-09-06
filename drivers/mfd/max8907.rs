//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/max8907.c
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
// max8907.c - mfd driver for MAX8907
//
// Copyright (C) 2010 Gyungoh Yoo <jack.yoo@maxim-ic.com>
// Copyright (C) 2010-2012, NVIDIA CORPORATION. All rights reserved.
//

    static const struct mfd_cell max8907_cells[] = {
    { .name = "max8907-regulator", },
    { .name = "max8907-rtc", },
    };
#[no_mangle]
unsafe extern "C" fn max8907_gen_is_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max8907_gen_is_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MAX8907_REG_ON_OFF_IRQ1:
    case MAX8907_REG_ON_OFF_STAT:
    case MAX8907_REG_ON_OFF_IRQ2:
    case MAX8907_REG_CHG_IRQ1:
    case MAX8907_REG_CHG_IRQ2:
    case MAX8907_REG_CHG_STAT:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn max8907_gen_is_precious_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max8907_gen_is_precious_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MAX8907_REG_ON_OFF_IRQ1:
    case MAX8907_REG_ON_OFF_IRQ2:
    case MAX8907_REG_CHG_IRQ1:
    case MAX8907_REG_CHG_IRQ2:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn max8907_gen_is_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max8907_gen_is_writeable_reg(struct device *dev, unsigned int reg)
    {
    return !max8907_gen_is_volatile_reg(dev, reg);
    }
    static const struct regmap_config max8907_regmap_gen_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .volatile_reg = max8907_gen_is_volatile_reg,
    .precious_reg = max8907_gen_is_precious_reg,
    .writeable_reg = max8907_gen_is_writeable_reg,
    .max_register = MAX8907_REG_LDO20VOUT,
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn max8907_rtc_is_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max8907_rtc_is_volatile_reg(struct device *dev, unsigned int reg)
    {
    if (reg <= MAX8907_REG_RTC_YEAR2)
    return true;
    switch (reg) {
    case MAX8907_REG_RTC_STATUS:
    case MAX8907_REG_RTC_IRQ:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn max8907_rtc_is_precious_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max8907_rtc_is_precious_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MAX8907_REG_RTC_IRQ:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn max8907_rtc_is_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool max8907_rtc_is_writeable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MAX8907_REG_RTC_STATUS:
    case MAX8907_REG_RTC_IRQ:
    return false;
    default:
    return true;
    }
    }
    static const struct regmap_config max8907_regmap_rtc_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .volatile_reg = max8907_rtc_is_volatile_reg,
    .precious_reg = max8907_rtc_is_precious_reg,
    .writeable_reg = max8907_rtc_is_writeable_reg,
    .max_register = MAX8907_REG_MPL_CNTL,
    .cache_type = REGCACHE_MAPLE,
    };
    static const struct regmap_irq max8907_chg_irqs[] = {
    { .reg_offset = 0, .mask = 1 << 0, },
    { .reg_offset = 0, .mask = 1 << 1, },
    { .reg_offset = 0, .mask = 1 << 2, },
    { .reg_offset = 1, .mask = 1 << 0, },
    { .reg_offset = 1, .mask = 1 << 1, },
    { .reg_offset = 1, .mask = 1 << 2, },
    { .reg_offset = 1, .mask = 1 << 3, },
    { .reg_offset = 1, .mask = 1 << 4, },
    { .reg_offset = 1, .mask = 1 << 5, },
    { .reg_offset = 1, .mask = 1 << 6, },
    { .reg_offset = 1, .mask = 1 << 7, },
    };
    static const struct regmap_irq_chip max8907_chg_irq_chip = {
    .name = "max8907 chg",
    .status_base = MAX8907_REG_CHG_IRQ1,
    .mask_base = MAX8907_REG_CHG_IRQ1_MASK,
    .wake_base = MAX8907_REG_CHG_IRQ1_MASK,
    .irq_reg_stride = MAX8907_REG_CHG_IRQ2 - MAX8907_REG_CHG_IRQ1,
    .num_regs = 2,
    .irqs = max8907_chg_irqs,
    .num_irqs = ARRAY_SIZE(max8907_chg_irqs),
    };
    static const struct regmap_irq max8907_on_off_irqs[] = {
    { .reg_offset = 0, .mask = 1 << 0, },
    { .reg_offset = 0, .mask = 1 << 1, },
    { .reg_offset = 0, .mask = 1 << 2, },
    { .reg_offset = 0, .mask = 1 << 3, },
    { .reg_offset = 0, .mask = 1 << 4, },
    { .reg_offset = 0, .mask = 1 << 5, },
    { .reg_offset = 0, .mask = 1 << 6, },
    { .reg_offset = 0, .mask = 1 << 7, },
    { .reg_offset = 1, .mask = 1 << 0, },
    { .reg_offset = 1, .mask = 1 << 1, },
    };
    static const struct regmap_irq_chip max8907_on_off_irq_chip = {
    .name = "max8907 on_off",
    .status_base = MAX8907_REG_ON_OFF_IRQ1,
    .mask_base = MAX8907_REG_ON_OFF_IRQ1_MASK,
    .irq_reg_stride = MAX8907_REG_ON_OFF_IRQ2 - MAX8907_REG_ON_OFF_IRQ1,
    .num_regs = 2,
    .irqs = max8907_on_off_irqs,
    .num_irqs = ARRAY_SIZE(max8907_on_off_irqs),
    };
    static const struct regmap_irq max8907_rtc_irqs[] = {
    { .reg_offset = 0, .mask = 1 << 2, },
    { .reg_offset = 0, .mask = 1 << 3, },
    };
    static const struct regmap_irq_chip max8907_rtc_irq_chip = {
    .name = "max8907 rtc",
    .status_base = MAX8907_REG_RTC_IRQ,
    .mask_base = MAX8907_REG_RTC_IRQ_MASK,
    .num_regs = 1,
    .irqs = max8907_rtc_irqs,
    .num_irqs = ARRAY_SIZE(max8907_rtc_irqs),
    };
    static struct max8907 *max8907_pm_off;
#[no_mangle]
unsafe extern "C" fn max8907_power_off() {
    static void max8907_power_off(void)
    {
    regmap_update_bits(max8907_pm_off.regmap_gen, MAX8907_REG_RESET_CNFG,
    MAX8907_MASK_POWER_OFF, MAX8907_MASK_POWER_OFF);
    }
#[no_mangle]
unsafe extern "C" fn max8907_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int max8907_i2c_probe(struct i2c_client *i2c)
    {
    struct max8907 *max8907;
    int ret;
    struct max8907_platform_data *pdata = dev_get_platdata(&i2c.dev);
    let mut pm_off: bool = false;
    if (pdata)
    pm_off = pdata.pm_off;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: i2c->dev.of_node) -> else {
    else if (i2c.dev.of_node)
    pm_off = of_property_read_bool(i2c.dev.of_node,
    "maxim,system-power-controller");
    max8907 = devm_kzalloc(&i2c.dev, sizeof(struct max8907), GFP_KERNEL);
    if (!max8907) {
    ret = -ENOMEM;
    goto err_alloc_drvdata;
    }
    max8907.dev = &i2c.dev;
    max8907.i2c_gen = i2c;
    i2c_set_clientdata(i2c, max8907);
    max8907.regmap_gen = devm_regmap_init_i2c(i2c,
    &max8907_regmap_gen_config);
    if (IS_ERR(max8907.regmap_gen)) {
    ret = PTR_ERR(max8907.regmap_gen);
    dev_err(&i2c.dev, "gen regmap init failed: %d\n", ret);
    goto err_regmap_gen;
    }
    max8907.i2c_rtc = i2c_new_dummy_device(i2c.adapter, MAX8907_RTC_I2C_ADDR);
    if (IS_ERR(max8907.i2c_rtc)) {
    ret = PTR_ERR(max8907.i2c_rtc);
    goto err_dummy_rtc;
    }
    i2c_set_clientdata(max8907.i2c_rtc, max8907);
    max8907.regmap_rtc = devm_regmap_init_i2c(max8907.i2c_rtc,
    &max8907_regmap_rtc_config);
    if (IS_ERR(max8907.regmap_rtc)) {
    ret = PTR_ERR(max8907.regmap_rtc);
    dev_err(&i2c.dev, "rtc regmap init failed: %d\n", ret);
    goto err_regmap_rtc;
    }
    ret = regmap_add_irq_chip(max8907.regmap_gen, max8907.i2c_gen.irq,
    IRQF_ONESHOT | IRQF_SHARED,
    -1, &max8907_chg_irq_chip,
    &max8907.irqc_chg);
    if (ret != 0) {
    dev_err(&i2c.dev, "failed to add chg irq chip: %d\n", ret);
    goto err_irqc_chg;
    }
    ret = regmap_add_irq_chip(max8907.regmap_gen, max8907.i2c_gen.irq,
    IRQF_ONESHOT | IRQF_SHARED, -1,
    &max8907_on_off_irq_chip,
    &max8907.irqc_on_off);
    if (ret != 0) {
    dev_err(&i2c.dev, "failed to add on off irq chip: %d\n", ret);
    goto err_irqc_on_off;
    }
    ret = regmap_add_irq_chip(max8907.regmap_rtc, max8907.i2c_gen.irq,
    IRQF_ONESHOT | IRQF_SHARED, -1,
    &max8907_rtc_irq_chip,
    &max8907.irqc_rtc);
    if (ret != 0) {
    dev_err(&i2c.dev, "failed to add rtc irq chip: %d\n", ret);
    goto err_irqc_rtc;
    }
    ret = mfd_add_devices(max8907.dev, -1, max8907_cells,
    ARRAY_SIZE(max8907_cells), core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret != 0) {
    dev_err(&i2c.dev, "failed to add MFD devices %d\n", ret);
    goto err_add_devices;
    }
    if (pm_off && !pm_power_off) {
    max8907_pm_off = max8907;
    pm_power_off = max8907_power_off;
    }
    return 0;
    err_add_devices:
    regmap_del_irq_chip(max8907.i2c_gen.irq, max8907.irqc_rtc);
    err_irqc_rtc:
    regmap_del_irq_chip(max8907.i2c_gen.irq, max8907.irqc_on_off);
    err_irqc_on_off:
    regmap_del_irq_chip(max8907.i2c_gen.irq, max8907.irqc_chg);
    err_irqc_chg:
    err_regmap_rtc:
    i2c_unregister_device(max8907.i2c_rtc);
    err_dummy_rtc:
    err_regmap_gen:
    err_alloc_drvdata:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn max8907_i2c_remove(i2c: *mut i2c_client) {
    static void max8907_i2c_remove(struct i2c_client *i2c)
    {
    struct max8907 *max8907 = i2c_get_clientdata(i2c);
    mfd_remove_devices(max8907.dev);
    regmap_del_irq_chip(max8907.i2c_gen.irq, max8907.irqc_rtc);
    regmap_del_irq_chip(max8907.i2c_gen.irq, max8907.irqc_on_off);
    regmap_del_irq_chip(max8907.i2c_gen.irq, max8907.irqc_chg);
    i2c_unregister_device(max8907.i2c_rtc);
    }

    static const struct of_device_id max8907_of_match[] = {
    { .compatible = "maxim,max8907" },
    { },
    };
    MODULE_DEVICE_TABLE(of, max8907_of_match);

    static const struct i2c_device_id max8907_i2c_id[] = {
    { "max8907" },
    {}
    };
    MODULE_DEVICE_TABLE(i2c, max8907_i2c_id);
    static struct i2c_driver max8907_i2c_driver = {
    .driver = {
    .name = "max8907",
    .of_match_table = of_match_ptr(max8907_of_match),
    },
    .probe = max8907_i2c_probe,
    .remove = max8907_i2c_remove,
    .id_table = max8907_i2c_id,
    };
#[no_mangle]
unsafe extern "C" fn max8907_i2c_init() -> int __init {
    static int __init max8907_i2c_init(void)
    {
    let mut ret: c_int = -ENODEV;
    ret = i2c_add_driver(&max8907_i2c_driver);
    if (ret != 0)
    pr_err("Failed to register I2C driver: %d\n", ret);
    return ret;
    }
    subsys_initcall(max8907_i2c_init);
#[no_mangle]
unsafe extern "C" fn max8907_i2c_exit() -> void __exit {
    static void __exit max8907_i2c_exit(void)
    {
    i2c_del_driver(&max8907_i2c_driver);
    }
    module_exit(max8907_i2c_exit);
    MODULE_DESCRIPTION("MAX8907 multi-function core driver");
    MODULE_AUTHOR("Gyungoh Yoo <jack.yoo@maxim-ic.com>");
    MODULE_LICENSE("GPL v2");
