//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/rt4831.c
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
// Copyright (c) 2021 Richtek Technology Corp.
//
// Author: ChiYuan Huang <cy_huang@richtek.com>
//

pub const RT4831_REG_REVISION: c_uint = 0x01;
pub const RT4831_REG_ENABLE: c_uint = 0x08;
pub const RT4831_REG_I2CPROT: c_uint = 0x15;
pub const RICHTEK_VENDOR_ID: c_uint = 0x03;

    static const struct mfd_cell rt4831_subdevs[] = {
    MFD_CELL_OF("rt4831-backlight", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "richtek,rt4831-backlight"),
    MFD_CELL_NAME("rt4831-regulator")
    };
#[no_mangle]
unsafe extern "C" fn rt4831_is_accessible_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool rt4831_is_accessible_reg(struct device *dev, unsigned int reg)
    {
    if (reg >= RT4831_REG_REVISION && reg <= RT4831_REG_I2CPROT)
    return true;
    return false;
    }
    static const struct regmap_config rt4831_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = RT4831_REG_I2CPROT,
    .readable_reg = rt4831_is_accessible_reg,
    .writeable_reg = rt4831_is_accessible_reg,
    };
#[no_mangle]
unsafe extern "C" fn rt4831_probe(client: *mut i2c_client) -> c_int {
    static int rt4831_probe(struct i2c_client *client)
    {
    struct gpio_desc *enable_gpio;
    struct regmap *regmap;
    unsigned int chip_id;
    int ret;
    enable_gpio = devm_gpiod_get_optional(&client.dev, "enable", GPIOD_OUT_HIGH);
    if (IS_ERR(enable_gpio)) {
    dev_err(&client.dev, "Failed to get 'enable' GPIO\n");
    return PTR_ERR(enable_gpio);
    }
    regmap = devm_regmap_init_i2c(client, &rt4831_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&client.dev, "Failed to initialize regmap\n");
    return PTR_ERR(regmap);
    }
    ret = regmap_read(regmap, RT4831_REG_REVISION, &chip_id);
    if (ret) {
    dev_err(&client.dev, "Failed to get H/W revision\n");
    return ret;
    }
    if ((chip_id & RT4831_VID_MASK) != RICHTEK_VENDOR_ID) {
    dev_err(&client.dev, "Chip vendor ID 0x%02x not matched\n", chip_id);
    return -ENODEV;
    }
//
// Used to prevent the abnormal shutdown.
// If SCL/SDA both keep low for one second to reset HW.
//
    ret = regmap_update_bits(regmap, RT4831_REG_I2CPROT, RT4831_I2CSAFETMR_MASK,
    RT4831_I2CSAFETMR_MASK);
    if (ret) {
    dev_err(&client.dev, "Failed to enable I2C safety timer\n");
    return ret;
    }
    return devm_mfd_add_devices(&client.dev, PLATFORM_DEVID_AUTO, rt4831_subdevs,
    ARRAY_SIZE(rt4831_subdevs), core::ptr::null_mut(), 0, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn rt4831_remove(client: *mut i2c_client) {
    static void rt4831_remove(struct i2c_client *client)
    {
    struct regmap *regmap = dev_get_regmap(&client.dev, core::ptr::null_mut());
    int ret;
// Disable WLED and DSV outputs
    ret = regmap_update_bits(regmap, RT4831_REG_ENABLE, RT4831_RESET_MASK, RT4831_RESET_MASK);
    if (ret)
    dev_warn(&client.dev, "Failed to disable outputs (%pe)\n", ERR_PTR(ret));
    }
    static const struct of_device_id __maybe_unused rt4831_of_match[] = {
    { .compatible = "richtek,rt4831", },
    {}
    };
    MODULE_DEVICE_TABLE(of, rt4831_of_match);
    static struct i2c_driver rt4831_driver = {
    .driver = {
    .name = "rt4831",
    .of_match_table = rt4831_of_match,
    },
    .probe = rt4831_probe,
    .remove = rt4831_remove,
    };
    module_i2c_driver(rt4831_driver);
    MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>");
    MODULE_DESCRIPTION("Richtek RT4831 core driver");
    MODULE_LICENSE("GPL v2");
