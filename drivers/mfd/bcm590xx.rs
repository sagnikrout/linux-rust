//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/bcm590xx.c
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
// Broadcom BCM590xx PMU
//
// Copyright 2014 Linaro Limited
// Author: Matt Porter <mporter@linaro.org>
//

// Under primary I2C address:
pub const BCM590XX_REG_PMUID: c_uint = 0x1e;
pub const BCM590XX_REG_PMUREV: c_uint = 0x1f;
pub const BCM590XX_PMUREV_DIG_MASK: c_uint = 0xF;
pub const BCM590XX_PMUREV_DIG_SHIFT: c_int = 0;
pub const BCM590XX_PMUREV_ANA_MASK: c_uint = 0xF0;
pub const BCM590XX_PMUREV_ANA_SHIFT: c_int = 4;
    static const struct mfd_cell bcm590xx_devs[] = {
    {
    .name = "bcm590xx-vregs",
    },
    };
    static const struct regmap_config bcm590xx_regmap_config_pri = {
    .reg_bits	= 8,
    .val_bits	= 8,
    .max_register	= BCM590XX_MAX_REGISTER_PRI,
    .cache_type	= REGCACHE_MAPLE,
    };
    static const struct regmap_config bcm590xx_regmap_config_sec = {
    .reg_bits	= 8,
    .val_bits	= 8,
    .max_register	= BCM590XX_MAX_REGISTER_SEC,
    .cache_type	= REGCACHE_MAPLE,
    };
// Map PMU ID value to model name string
    static const char * const bcm590xx_names[] = {
    [BCM590XX_PMUID_BCM59054] = "BCM59054",
    [BCM590XX_PMUID_BCM59056] = "BCM59056",
    };
#[no_mangle]
unsafe extern "C" fn bcm590xx_parse_version(bcm590xx: *mut bcm590xx) -> c_int {
    static int bcm590xx_parse_version(struct bcm590xx *bcm590xx)
    {
    unsigned int id, rev;
    int ret;
// Get PMU ID and verify that it matches compatible
    ret = regmap_read(bcm590xx.regmap_pri, BCM590XX_REG_PMUID, &id);
    if (ret) {
    dev_err(bcm590xx.dev, "failed to read PMU ID: %d\n", ret);
    return ret;
    }
    if (id != bcm590xx.pmu_id) {
    dev_err(bcm590xx.dev, "Incorrect ID for %s: expected %x, got %x.\n",
    bcm590xx_names[bcm590xx.pmu_id], bcm590xx.pmu_id, id);
    return -ENODEV;
    }
// Get PMU revision and store it in the info struct
    ret = regmap_read(bcm590xx.regmap_pri, BCM590XX_REG_PMUREV, &rev);
    if (ret) {
    dev_err(bcm590xx.dev, "failed to read PMU revision: %d\n", ret);
    return ret;
    }
    bcm590xx.rev_digital = (rev & BCM590XX_PMUREV_DIG_MASK) >> BCM590XX_PMUREV_DIG_SHIFT;
    bcm590xx.rev_analog = (rev & BCM590XX_PMUREV_ANA_MASK) >> BCM590XX_PMUREV_ANA_SHIFT;
    dev_dbg(bcm590xx.dev, "PMU ID 0x%x (%s), revision: digital %d, analog %d",
    id, bcm590xx_names[id], bcm590xx.rev_digital, bcm590xx.rev_analog);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm590xx_i2c_probe(i2c_pri: *mut i2c_client) -> c_int {
    static int bcm590xx_i2c_probe(struct i2c_client *i2c_pri)
    {
    struct bcm590xx *bcm590xx;
    int ret;
    bcm590xx = devm_kzalloc(&i2c_pri.dev, sizeof(*bcm590xx), GFP_KERNEL);
    if (!bcm590xx)
    return -ENOMEM;
    i2c_set_clientdata(i2c_pri, bcm590xx);
    bcm590xx.dev = &i2c_pri.dev;
    bcm590xx.i2c_pri = i2c_pri;
    bcm590xx.pmu_id = (uintptr_t) of_device_get_match_data(bcm590xx.dev);
    bcm590xx.regmap_pri = devm_regmap_init_i2c(i2c_pri,
    &bcm590xx_regmap_config_pri);
    if (IS_ERR(bcm590xx.regmap_pri)) {
    ret = PTR_ERR(bcm590xx.regmap_pri);
    dev_err(&i2c_pri.dev, "primary regmap init failed: %d\n", ret);
    return ret;
    }
// Secondary I2C slave address is the base address with A(2) asserted
    bcm590xx.i2c_sec = i2c_new_dummy_device(i2c_pri.adapter,
    i2c_pri.addr | BIT(2));
    if (IS_ERR(bcm590xx.i2c_sec)) {
    dev_err(&i2c_pri.dev, "failed to add secondary I2C device\n");
    return PTR_ERR(bcm590xx.i2c_sec);
    }
    i2c_set_clientdata(bcm590xx.i2c_sec, bcm590xx);
    bcm590xx.regmap_sec = devm_regmap_init_i2c(bcm590xx.i2c_sec,
    &bcm590xx_regmap_config_sec);
    if (IS_ERR(bcm590xx.regmap_sec)) {
    ret = PTR_ERR(bcm590xx.regmap_sec);
    dev_err(&bcm590xx.i2c_sec.dev,
    "secondary regmap init failed: %d\n", ret);
    goto err;
    }
    ret = bcm590xx_parse_version(bcm590xx);
    if (ret)
    goto err;
    ret = devm_mfd_add_devices(&i2c_pri.dev, -1, bcm590xx_devs,
    ARRAY_SIZE(bcm590xx_devs), core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret < 0) {
    dev_err(&i2c_pri.dev, "failed to add sub-devices: %d\n", ret);
    goto err;
    }
    return 0;
    err:
    i2c_unregister_device(bcm590xx.i2c_sec);
    return ret;
    }
    static const struct of_device_id bcm590xx_of_match[] = {
    {
    .compatible = "brcm,bcm59054",
    .data = (void *)BCM590XX_PMUID_BCM59054,
    },
    {
    .compatible = "brcm,bcm59056",
    .data = (void *)BCM590XX_PMUID_BCM59056,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, bcm590xx_of_match);
    static const struct i2c_device_id bcm590xx_i2c_id[] = {
    { "bcm59054" },
    { "bcm59056" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, bcm590xx_i2c_id);
    static struct i2c_driver bcm590xx_i2c_driver = {
    .driver = {
    .name = "bcm590xx",
    .of_match_table = bcm590xx_of_match,
    },
    .probe = bcm590xx_i2c_probe,
    .id_table = bcm590xx_i2c_id,
    };
    module_i2c_driver(bcm590xx_i2c_driver);
    MODULE_AUTHOR("Matt Porter <mporter@linaro.org>");
    MODULE_DESCRIPTION("BCM590xx multi-function driver");
    MODULE_LICENSE("GPL v2");
