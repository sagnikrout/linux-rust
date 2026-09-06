//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/bq257xx-regulator.c
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
// BQ257XX Battery Charger Driver
// Copyright (C) 2025 Chris Morgan <macromorgan@hotmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bq257xx_reg_data {
    pub bq257xx_reg: *mut regulator_dev,
    pub otg_en_gpio: *mut gpio_desc,
    pub desc: regulator_desc,
}

#[no_mangle]
unsafe extern "C" fn bq25703_vbus_get_cur_limit(rdev: *mut regulator_dev) -> c_int {
    static int bq25703_vbus_get_cur_limit(struct regulator_dev *rdev)
    {
    struct regmap *regmap = rdev_get_regmap(rdev);
    int ret;
    unsigned int reg;
    ret = regmap_read(regmap, BQ25703_OTG_CURRENT, &reg);
    if (ret)
    return ret;
    return FIELD_GET(BQ25703_OTG_CUR_MASK, reg) * BQ25703_OTG_CUR_STEP_UA;
    }
//
// Check if the minimum current and maximum current requested are
// sane values, then set the register accordingly.
//
    static int bq25703_vbus_set_cur_limit(struct regulator_dev *rdev,
    int min_uA, int max_uA)
    {
    struct regmap *regmap = rdev_get_regmap(rdev);
    unsigned int reg;
    if ((min_uA > BQ25703_OTG_CUR_MAX_UA) || (max_uA < 0))
    return -EINVAL;
    reg = (max_uA / BQ25703_OTG_CUR_STEP_UA);
// Catch rounding errors since our step is 50000uA.
    if ((reg * BQ25703_OTG_CUR_STEP_UA) < min_uA)
    return -EINVAL;
    return regmap_write(regmap, BQ25703_OTG_CURRENT,
    FIELD_PREP(BQ25703_OTG_CUR_MASK, reg));
    }
#[no_mangle]
unsafe extern "C" fn bq25703_vbus_enable(rdev: *mut regulator_dev) -> c_int {
    static int bq25703_vbus_enable(struct regulator_dev *rdev)
    {
    struct bq257xx_reg_data *pdata = rdev_get_drvdata(rdev);
    if (pdata.otg_en_gpio)
    gpiod_set_value_cansleep(pdata.otg_en_gpio, 1);
    return regulator_enable_regmap(rdev);
    }
#[no_mangle]
unsafe extern "C" fn bq25703_vbus_disable(rdev: *mut regulator_dev) -> c_int {
    static int bq25703_vbus_disable(struct regulator_dev *rdev)
    {
    struct bq257xx_reg_data *pdata = rdev_get_drvdata(rdev);
    if (pdata.otg_en_gpio)
    gpiod_set_value_cansleep(pdata.otg_en_gpio, 0);
    return regulator_disable_regmap(rdev);
    }
    static const struct regulator_ops bq25703_vbus_ops = {
    .enable = bq25703_vbus_enable,
    .disable = bq25703_vbus_disable,
    .is_enabled = regulator_is_enabled_regmap,
    .list_voltage = regulator_list_voltage_linear,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_current_limit = bq25703_vbus_get_cur_limit,
    .set_current_limit = bq25703_vbus_set_cur_limit,
    };
    static const struct regulator_desc bq25703_vbus_desc = {
    .name = "vbus",
    .of_match = of_match_ptr("vbus"),
    .regulators_node = of_match_ptr("regulators"),
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .ops = &bq25703_vbus_ops,
    .min_uV = BQ25703_OTG_VOLT_MIN_UV,
    .uV_step = BQ25703_OTG_VOLT_STEP_UV,
    .n_voltages = BQ25703_OTG_VOLT_NUM_VOLT,
    .enable_mask = BQ25703_EN_OTG_MASK,
    .enable_reg = BQ25703_CHARGE_OPTION_3,
    .enable_val = BQ25703_EN_OTG_MASK,
    .disable_val = 0,
    .vsel_reg = BQ25703_OTG_VOLT,
    .vsel_mask = BQ25703_OTG_VOLT_MASK,
    };
// Get optional GPIO for OTG regulator enable.
#[no_mangle]
unsafe extern "C" fn bq257xx_reg_dt_parse_gpio(pdev: *mut platform_device) {
    static void bq257xx_reg_dt_parse_gpio(struct platform_device *pdev)
    {
    struct device_node *child, *subchild;
    struct bq257xx_reg_data *pdata = platform_get_drvdata(pdev);
    child = of_get_child_by_name(pdev.dev.of_node,
    pdata.desc.regulators_node);
    if (!child)
    return;
    subchild = of_get_child_by_name(child, pdata.desc.of_match);
    of_node_put(child);
    if (!subchild)
    return;
    pdata.otg_en_gpio = devm_fwnode_gpiod_get_index(&pdev.dev,
    of_fwnode_handle(subchild),
    "enable", 0,
    GPIOD_OUT_LOW,
    pdata.desc.of_match);
    of_node_put(subchild);
    if (IS_ERR(pdata.otg_en_gpio)) {
    if (PTR_ERR(pdata.otg_en_gpio) == -ENOENT) {
// No GPIO, will only use register writes for OTG
    pdata.otg_en_gpio = core::ptr::null_mut();
    return;
    }
    dev_err(&pdev.dev, "Error getting enable gpio: %ld\n",
    PTR_ERR(pdata.otg_en_gpio));
    return;
    }
    }
#[no_mangle]
unsafe extern "C" fn bq257xx_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int bq257xx_regulator_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct bq257xx_reg_data *pdata;
    let mut cfg: regulator_config = {};
    device_set_of_node_from_dev(&pdev.dev, pdev.dev.parent);
    pdata = devm_kzalloc(&pdev.dev, sizeof(struct bq257xx_reg_data), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    pdata.desc = bq25703_vbus_desc;
    platform_set_drvdata(pdev, pdata);
    bq257xx_reg_dt_parse_gpio(pdev);
    cfg.dev = &pdev.dev;
    cfg.driver_data = pdata;
    cfg.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!cfg.regmap)
    return -ENODEV;
    pdata.bq257xx_reg = devm_regulator_register(dev, &pdata.desc, &cfg);
    if (IS_ERR(pdata.bq257xx_reg)) {
    return dev_err_probe(&pdev.dev, PTR_ERR(pdata.bq257xx_reg),
    "error registering bq257xx regulator");
    }
    return 0;
    }
    static struct platform_driver bq257xx_reg_driver = {
    .driver = {
    .name = "bq257xx-regulator",
    },
    .probe = bq257xx_regulator_probe,
    };
    module_platform_driver(bq257xx_reg_driver);
    MODULE_DESCRIPTION("bq257xx regulator driver");
    MODULE_AUTHOR("Chris Morgan <macromorgan@hotmail.com>");
    MODULE_LICENSE("GPL");
