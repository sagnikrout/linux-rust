//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/88pm80x.c
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
// I2C driver for Marvell 88PM80x
//
// Copyright (C) 2012 Marvell International Ltd.
// Haojian Zhuang <haojian.zhuang@marvell.com>
// Joseph(Yossi) Hanin <yhanin@marvell.com>
// Qiao Zhou <zhouqiao@marvell.com>
//

// 88pm80x chips have same definition for chip id register.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm80x_chip_mapping {
    pub id: c_uint,
    pub type: c_int,
}

    static struct pm80x_chip_mapping chip_mapping[] = {
// 88PM800 chip id number
    {0x3,	CHIP_PM800},
// 88PM805 chip id number
    {0x0,	CHIP_PM805},
// 88PM860 chip id number
    {0x4,	CHIP_PM860},
    };
//
// workaround: some registers needed by pm805 are defined in pm800, so
// need to use this global variable to maintain the relation between
// pm800 and pm805. would remove it after HW chip fixes the issue.
//
    static struct pm80x_chip *g_pm80x_chip;
    const struct regmap_config pm80x_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
    EXPORT_SYMBOL_GPL(pm80x_regmap_config);
#[no_mangle]
pub unsafe extern "C" fn pm80x_init(client: *mut i2c_client) -> c_int {
    int pm80x_init(struct i2c_client *client)
    {
    struct pm80x_chip *chip;
    struct regmap *map;
    unsigned int val;
    int i, ret = 0;
    chip =
    devm_kzalloc(&client.dev, sizeof(struct pm80x_chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    map = devm_regmap_init_i2c(client, &pm80x_regmap_config);
    if (IS_ERR(map)) {
    ret = PTR_ERR(map);
    dev_err(&client.dev, "Failed to allocate register map: %d\n",
    ret);
    return ret;
    }
    chip.client = client;
    chip.regmap = map;
    chip.irq = client.irq;
    chip.dev = &client.dev;
    i2c_set_clientdata(chip.client, chip);
    ret = regmap_read(chip.regmap, PM80X_CHIP_ID, &val);
    if (ret < 0) {
    dev_err(chip.dev, "Failed to read CHIP ID: %d\n", ret);
    return ret;
    }
    for (i = 0; i < ARRAY_SIZE(chip_mapping); i++) {
    if (chip_mapping[i].id == PM80X_CHIP_ID_NUM(val)) {
    chip.type = chip_mapping[i].type;
    break;
    }
    }
    if (i == ARRAY_SIZE(chip_mapping)) {
    dev_err(chip.dev,
    "Failed to detect Marvell 88PM800:ChipID[0x%x]\n", val);
    return -EINVAL;
    }
    device_init_wakeup(&client.dev, 1);
//
// workaround: set g_pm80x_chip to the first probed chip. if the
// second chip is probed, just point to the companion to each
// other so that pm805 can access those specific register. would
// remove it after HW chip fixes the issue.
//
    if (!g_pm80x_chip)
    g_pm80x_chip = chip;
    else {
    chip.companion = g_pm80x_chip.client;
    g_pm80x_chip.companion = chip.client;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(pm80x_init);
#[no_mangle]
pub unsafe extern "C" fn pm80x_deinit() -> c_int {
    int pm80x_deinit(void)
    {
//
// workaround: clear the dependency between pm800 and pm805.
// would remove it after HW chip fixes the issue.
//
    if (g_pm80x_chip.companion)
    g_pm80x_chip.companion = core::ptr::null_mut();
    else
    g_pm80x_chip = core::ptr::null_mut();
    return 0;
    }
    EXPORT_SYMBOL_GPL(pm80x_deinit);
#[no_mangle]
unsafe extern "C" fn pm80x_suspend(dev: *mut device) -> c_int {
    static int pm80x_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct pm80x_chip *chip = i2c_get_clientdata(client);
    if (chip && chip.wu_flag)
    if (device_may_wakeup(chip.dev))
    enable_irq_wake(chip.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm80x_resume(dev: *mut device) -> c_int {
    static int pm80x_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct pm80x_chip *chip = i2c_get_clientdata(client);
    if (chip && chip.wu_flag)
    if (device_may_wakeup(chip.dev))
    disable_irq_wake(chip.irq);
    return 0;
    }
    EXPORT_GPL_SIMPLE_DEV_PM_OPS(pm80x_pm_ops, pm80x_suspend, pm80x_resume);
    MODULE_DESCRIPTION("I2C Driver for Marvell 88PM80x");
    MODULE_AUTHOR("Qiao Zhou <zhouqiao@marvell.com>");
    MODULE_LICENSE("GPL");
