//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/qcom-refgen-regulator.c
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
// Copyright (c) 2017, 2019-2020, The Linux Foundation. All rights reserved.
// Copyright (c) 2023, Linaro Limited

pub const REFGEN_REG_BIAS_EN: c_uint = 0x08;

pub const REFGEN_BIAS_EN_ENABLE: c_uint = 0x7;
pub const REFGEN_BIAS_EN_DISABLE: c_uint = 0x6;
pub const REFGEN_REG_REFGEN_STATUS: c_uint = 0xc;

pub const REFGEN_REG_BG_CTRL: c_uint = 0x14;

pub const REFGEN_BG_CTRL_ENABLE: c_uint = 0x3;
pub const REFGEN_BG_CTRL_DISABLE: c_uint = 0x2;
pub const REFGEN_REG_PWRDWN_CTRL5: c_uint = 0x80;

pub const REFGEN_PWRDWN_CTRL5_ENABLE: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_refgen_regulator_data {
    pub rdesc: *const regulator_desc,
    pub has_clocks: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_refgen_drvdata {
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub is_enabled: bool,
}

#[no_mangle]
unsafe extern "C" fn qcom_sdm845_refgen_enable(rdev: *mut regulator_dev) -> c_int {
    static int qcom_sdm845_refgen_enable(struct regulator_dev *rdev)
    {
    regmap_update_bits(rdev.regmap, REFGEN_REG_BG_CTRL, REFGEN_BG_CTRL_MASK,
    FIELD_PREP(REFGEN_BG_CTRL_MASK, REFGEN_BG_CTRL_ENABLE));
    regmap_write(rdev.regmap, REFGEN_REG_BIAS_EN,
    FIELD_PREP(REFGEN_BIAS_EN_MASK, REFGEN_BIAS_EN_ENABLE));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_sdm845_refgen_disable(rdev: *mut regulator_dev) -> c_int {
    static int qcom_sdm845_refgen_disable(struct regulator_dev *rdev)
    {
    regmap_write(rdev.regmap, REFGEN_REG_BIAS_EN,
    FIELD_PREP(REFGEN_BIAS_EN_MASK, REFGEN_BIAS_EN_DISABLE));
    regmap_update_bits(rdev.regmap, REFGEN_REG_BG_CTRL, REFGEN_BG_CTRL_MASK,
    FIELD_PREP(REFGEN_BG_CTRL_MASK, REFGEN_BG_CTRL_DISABLE));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_sdm845_refgen_is_enabled(rdev: *mut regulator_dev) -> c_int {
    static int qcom_sdm845_refgen_is_enabled(struct regulator_dev *rdev)
    {
    u32 val;
    regmap_read(rdev.regmap, REFGEN_REG_BG_CTRL, &val);
    if (FIELD_GET(REFGEN_BG_CTRL_MASK, val) != REFGEN_BG_CTRL_ENABLE)
    return 0;
    regmap_read(rdev.regmap, REFGEN_REG_BIAS_EN, &val);
    if (FIELD_GET(REFGEN_BIAS_EN_MASK, val) != REFGEN_BIAS_EN_ENABLE)
    return 0;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn qcom_ipq9650_refgen_enable(rdev: *mut regulator_dev) -> c_int {
    static int qcom_ipq9650_refgen_enable(struct regulator_dev *rdev)
    {
    struct qcom_refgen_drvdata *drvdata = rdev_get_drvdata(rdev);
    int ret;
    ret = clk_bulk_prepare_enable(drvdata.num_clks, drvdata.clks);
    if (ret)
    return ret;
    drvdata.is_enabled = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_ipq9650_refgen_disable(rdev: *mut regulator_dev) -> c_int {
    static int qcom_ipq9650_refgen_disable(struct regulator_dev *rdev)
    {
    struct qcom_refgen_drvdata *drvdata = rdev_get_drvdata(rdev);
    clk_bulk_disable_unprepare(drvdata.num_clks, drvdata.clks);
    drvdata.is_enabled = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_ipq9650_refgen_is_enabled(rdev: *mut regulator_dev) -> c_int {
    static int qcom_ipq9650_refgen_is_enabled(struct regulator_dev *rdev)
    {
    struct qcom_refgen_drvdata *drvdata = rdev_get_drvdata(rdev);
    return drvdata.is_enabled;
    }
#[no_mangle]
unsafe extern "C" fn qcom_ipq9650_refgen_get_status(rdev: *mut regulator_dev) -> c_int {
    static int qcom_ipq9650_refgen_get_status(struct regulator_dev *rdev)
    {
    u32 val;
    regmap_read(rdev.regmap, REFGEN_REG_REFGEN_STATUS, &val);
    if (FIELD_GET(REFGEN_STATUS_OUT_MASK, val))
    return REGULATOR_STATUS_ON;
    return REGULATOR_STATUS_OFF;
    }
    static const struct regulator_desc ipq9650_refgen_desc = {
    .enable_time = 5,
    .name = "refgen",
    .owner = THIS_MODULE,
    .type = REGULATOR_CURRENT,
    .ops = &(const struct regulator_ops) {
    .enable		= qcom_ipq9650_refgen_enable,
    .disable	= qcom_ipq9650_refgen_disable,
    .is_enabled	= qcom_ipq9650_refgen_is_enabled,
    .get_status	= qcom_ipq9650_refgen_get_status,
    },
    };
    static const struct regulator_desc sdm845_refgen_desc = {
    .enable_time = 5,
    .name = "refgen",
    .owner = THIS_MODULE,
    .type = REGULATOR_CURRENT,
    .ops = &(const struct regulator_ops) {
    .enable		= qcom_sdm845_refgen_enable,
    .disable	= qcom_sdm845_refgen_disable,
    .is_enabled	= qcom_sdm845_refgen_is_enabled,
    },
    };
    static const struct regulator_desc sm8250_refgen_desc = {
    .enable_reg = REFGEN_REG_PWRDWN_CTRL5,
    .enable_mask = REFGEN_PWRDWN_CTRL5_MASK,
    .enable_val = REFGEN_PWRDWN_CTRL5_ENABLE,
    .disable_val = 0,
    .enable_time = 5,
    .name = "refgen",
    .owner = THIS_MODULE,
    .type = REGULATOR_CURRENT,
    .ops = &(const struct regulator_ops) {
    .enable		= regulator_enable_regmap,
    .disable	= regulator_disable_regmap,
    .is_enabled	= regulator_is_enabled_regmap,
    },
    };
    static const struct qcom_refgen_regulator_data ipq9650_data = {
    .rdesc = &ipq9650_refgen_desc,
    .has_clocks = true,
    };
    static const struct qcom_refgen_regulator_data sdm845_data = {
    .rdesc = &sdm845_refgen_desc,
    };
    static const struct qcom_refgen_regulator_data sm8250_data = {
    .rdesc = &sm8250_refgen_desc,
    };
    static const struct regmap_config qcom_refgen_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    };
#[no_mangle]
unsafe extern "C" fn qcom_refgen_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_refgen_probe(struct platform_device *pdev)
    {
    const struct qcom_refgen_regulator_data *data;
    struct qcom_refgen_drvdata *drvdata = core::ptr::null_mut();
    struct regulator_init_data *init_data;
    let mut config: regulator_config = {};
    const struct regulator_desc *rdesc;
    struct device *dev = &pdev.dev;
    struct regulator_dev *rdev;
    struct regmap *regmap;
    void __iomem *base;
    data = of_device_get_match_data(dev);
    if (!data)
    return -ENODATA;
    if (data.has_clocks) {
    drvdata = devm_kzalloc(dev, sizeof(*drvdata), GFP_KERNEL);
    if (!drvdata)
    return -ENOMEM;
    drvdata.num_clks = devm_clk_bulk_get_all(dev, &drvdata.clks);
    if (drvdata.num_clks < 0)
    return dev_err_probe(dev, drvdata.num_clks,
    "failed to get clocks\n");
    }
    rdesc = data.rdesc;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    regmap = devm_regmap_init_mmio(dev, base, &qcom_refgen_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    init_data = of_get_regulator_init_data(dev, dev.of_node, rdesc);
    if (!init_data)
    return -ENOMEM;
    config.dev = dev;
    config.init_data = init_data;
    config.of_node = dev.of_node;
    config.regmap = regmap;
    config.driver_data = drvdata;
    rdev = devm_regulator_register(dev, rdesc, &config);
    if (IS_ERR(rdev))
    return PTR_ERR(rdev);
    return 0;
    }
    static const struct of_device_id qcom_refgen_match_table[] = {
    { .compatible = "qcom,ipq9650-refgen-regulator", .data = &ipq9650_data },
    { .compatible = "qcom,sdm845-refgen-regulator", .data = &sdm845_data },
    { .compatible = "qcom,sm8250-refgen-regulator", .data = &sm8250_data },
    { }
    };
    MODULE_DEVICE_TABLE(of, qcom_refgen_match_table);
    static struct platform_driver qcom_refgen_driver = {
    .probe = qcom_refgen_probe,
    .driver = {
    .name = "qcom-refgen-regulator",
    .of_match_table = qcom_refgen_match_table,
    },
    };
    module_platform_driver(qcom_refgen_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Qualcomm REFGEN regulator driver");
