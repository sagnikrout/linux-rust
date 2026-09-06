//! Automatically rewritten from C to Rust
//! Source: drivers/misc/lis3lv02d/lis3lv02d_i2c.c
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
// drivers/hwmon/lis3lv02d_i2c.c
//
// Implements I2C interface for lis3lv02d (STMicroelectronics) accelerometer.
// Driver is based on corresponding SPI driver written by Daniel Mack
// (lis3lv02d_spi.c (C) 2009 Daniel Mack <daniel@caiaq.de> ).
//
// Copyright (C) 2009 Nokia Corporation and/or its subsidiary(-ies).
//
// Contact: Samu Onkalo <samu.p.onkalo@nokia.com>
//

    static const char reg_vdd[]    = "Vdd";
    static const char reg_vdd_io[] = "Vdd_IO";
#[no_mangle]
unsafe extern "C" fn lis3_reg_ctrl(lis3: *mut lis3lv02d, state: bool) -> c_int {
    static int lis3_reg_ctrl(struct lis3lv02d *lis3, bool state)
    {
    int ret;
    if (state == LIS3_REG_OFF) {
    ret = regulator_bulk_disable(ARRAY_SIZE(lis3.regulators),
    lis3.regulators);
    } else {
    ret = regulator_bulk_enable(ARRAY_SIZE(lis3.regulators),
    lis3.regulators);
// Chip needs time to wakeup. Not mentioned in datasheet
    usleep_range(10000, 20000);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn lis3_i2c_write(lis3: *mut lis3lv02d, reg: c_int, value: u8) -> i32 {
    static inline s32 lis3_i2c_write(struct lis3lv02d *lis3, int reg, u8 value)
    {
    struct i2c_client *c = lis3.bus_priv;
    return i2c_smbus_write_byte_data(c, reg, value);
    }
#[no_mangle]
pub unsafe extern "C" fn lis3_i2c_read(lis3: *mut lis3lv02d, reg: c_int, v: *mut u8) -> i32 {
    static inline s32 lis3_i2c_read(struct lis3lv02d *lis3, int reg, u8 *v)
    {
    struct i2c_client *c = lis3.bus_priv;
// v = i2c_smbus_read_byte_data(c, reg);
    return 0;
    }
    static inline s32 lis3_i2c_blockread(struct lis3lv02d *lis3, int reg, int len,
    u8 *v)
    {
    struct i2c_client *c = lis3.bus_priv;
    reg |= (1 << 7); /* 7th bit enables address auto incrementation */
    return i2c_smbus_read_i2c_block_data(c, reg, len, v);
    }
#[no_mangle]
unsafe extern "C" fn lis3_i2c_init(lis3: *mut lis3lv02d) -> c_int {
    static int lis3_i2c_init(struct lis3lv02d *lis3)
    {
    u8 reg;
    int ret;
    lis3_reg_ctrl(lis3, LIS3_REG_ON);
    lis3.read(lis3, WHO_AM_I, &reg);
    if (reg != lis3.whoami)
    printk(KERN_ERR "lis3: power on failure\n");
// power up the device
    ret = lis3.read(lis3, CTRL_REG1, &reg);
    if (ret < 0)
    return ret;
    if (lis3.whoami == WAI_3DLH)
    reg |= CTRL1_PM0 | CTRL1_Xen | CTRL1_Yen | CTRL1_Zen;
    else
    reg |= CTRL1_PD0 | CTRL1_Xen | CTRL1_Yen | CTRL1_Zen;
    return lis3.write(lis3, CTRL_REG1, reg);
    }
// Default axis mapping but it can be overwritten by platform data
    static union axis_conversion lis3lv02d_axis_map =
    { .as_array = { LIS3_DEV_X, LIS3_DEV_Y, LIS3_DEV_Z } };

    static const struct of_device_id lis3lv02d_i2c_dt_ids[] = {
    { .compatible = "st,lis3lv02d" },
    { }
    };
    MODULE_DEVICE_TABLE(of, lis3lv02d_i2c_dt_ids);

#[no_mangle]
unsafe extern "C" fn lis3lv02d_i2c_probe(client: *mut i2c_client) -> c_int {
    static int lis3lv02d_i2c_probe(struct i2c_client *client)
    {
    let mut ret: c_int = 0;
    struct lis3lv02d_platform_data *pdata = client.dev.platform_data;

    if (of_match_device(lis3lv02d_i2c_dt_ids, &client.dev)) {
    lis3_dev.of_node = client.dev.of_node;
    ret = lis3lv02d_init_dt(&lis3_dev);
    if (ret)
    return ret;
    pdata = lis3_dev.pdata;
    }

    if (pdata) {
    if ((pdata.driver_features & LIS3_USE_BLOCK_READ) &&
    (i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_I2C_BLOCK)))
    lis3_dev.blkread  = lis3_i2c_blockread;
    if (pdata.axis_x)
    lis3lv02d_axis_map.x = pdata.axis_x;
    if (pdata.axis_y)
    lis3lv02d_axis_map.y = pdata.axis_y;
    if (pdata.axis_z)
    lis3lv02d_axis_map.z = pdata.axis_z;
    if (pdata.setup_resources)
    ret = pdata.setup_resources();
    if (ret)
    goto fail;
    }
    lis3_dev.regulators[0].supply = reg_vdd;
    lis3_dev.regulators[1].supply = reg_vdd_io;
    ret = regulator_bulk_get(&client.dev,
    ARRAY_SIZE(lis3_dev.regulators),
    lis3_dev.regulators);
    if (ret < 0)
    goto fail;
    lis3_dev.pdata	  = pdata;
    lis3_dev.bus_priv = client;
    lis3_dev.init	  = lis3_i2c_init;
    lis3_dev.read	  = lis3_i2c_read;
    lis3_dev.write	  = lis3_i2c_write;
    lis3_dev.reg_ctrl = lis3_reg_ctrl;
    lis3_dev.irq	  = client.irq;
    lis3_dev.ac	  = lis3lv02d_axis_map;
    lis3_dev.pm_dev	  = &client.dev;
    i2c_set_clientdata(client, &lis3_dev);
// Provide power over the init call
    lis3_reg_ctrl(&lis3_dev, LIS3_REG_ON);
    ret = lis3lv02d_init_device(&lis3_dev);
    lis3_reg_ctrl(&lis3_dev, LIS3_REG_OFF);
    if (ret)
    goto fail2;
    return 0;
    fail2:
    regulator_bulk_free(ARRAY_SIZE(lis3_dev.regulators),
    lis3_dev.regulators);
    fail:
    if (pdata && pdata.release_resources)
    pdata.release_resources();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lis3lv02d_i2c_remove(client: *mut i2c_client) {
    static void lis3lv02d_i2c_remove(struct i2c_client *client)
    {
    struct lis3lv02d *lis3 = i2c_get_clientdata(client);
    struct lis3lv02d_platform_data *pdata = client.dev.platform_data;
    if (pdata && pdata.release_resources)
    pdata.release_resources();
    lis3lv02d_joystick_disable(lis3);
    lis3lv02d_remove_fs(&lis3_dev);
    regulator_bulk_free(ARRAY_SIZE(lis3.regulators),
    lis3_dev.regulators);
    }

#[no_mangle]
unsafe extern "C" fn lis3lv02d_i2c_suspend(dev: *mut device) -> c_int {
    static int lis3lv02d_i2c_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct lis3lv02d *lis3 = i2c_get_clientdata(client);
// Turn on for wakeup if turned off by runtime suspend
    if (lis3.pdata && lis3.pdata.wakeup_flags) {
    if (pm_runtime_suspended(dev))
    lis3lv02d_poweron(lis3);
// For non wakeup turn off if not already turned off by runtime suspend
    } else if (!pm_runtime_suspended(dev))
    lis3lv02d_poweroff(lis3);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lis3lv02d_i2c_resume(dev: *mut device) -> c_int {
    static int lis3lv02d_i2c_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct lis3lv02d *lis3 = i2c_get_clientdata(client);
// Turn back off if turned on for wakeup and runtime suspended
    if (lis3.pdata && lis3.pdata.wakeup_flags) {
    if (pm_runtime_suspended(dev))
    lis3lv02d_poweroff(lis3);
// For non wakeup turn back on if not runtime suspended
    } else if (!pm_runtime_suspended(dev))
    lis3lv02d_poweron(lis3);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn lis3_i2c_runtime_suspend(dev: *mut device) -> c_int {
    static int lis3_i2c_runtime_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct lis3lv02d *lis3 = i2c_get_clientdata(client);
    lis3lv02d_poweroff(lis3);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lis3_i2c_runtime_resume(dev: *mut device) -> c_int {
    static int lis3_i2c_runtime_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct lis3lv02d *lis3 = i2c_get_clientdata(client);
    lis3lv02d_poweron(lis3);
    return 0;
    }

    static const struct i2c_device_id lis3lv02d_id[] = {
    { .name = "lis3lv02d", .driver_data = LIS3LV02D },
    { .name = "lis331dlh", .driver_data = LIS331DLH },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lis3lv02d_id);
    static const struct dev_pm_ops lis3_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(lis3lv02d_i2c_suspend,
    lis3lv02d_i2c_resume)
    SET_RUNTIME_PM_OPS(lis3_i2c_runtime_suspend,
    lis3_i2c_runtime_resume,
    core::ptr::null_mut())
    };
    static struct i2c_driver lis3lv02d_i2c_driver = {
    .driver	 = {
    .name   = DRV_NAME,
    .pm     = &lis3_pm_ops,
    .of_match_table = of_match_ptr(lis3lv02d_i2c_dt_ids),
    },
    .probe = lis3lv02d_i2c_probe,
    .remove	= lis3lv02d_i2c_remove,
    .id_table = lis3lv02d_id,
    };
    module_i2c_driver(lis3lv02d_i2c_driver);
    MODULE_AUTHOR("Nokia Corporation");
    MODULE_DESCRIPTION("lis3lv02d I2C interface");
    MODULE_LICENSE("GPL");
