//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/qcom_usb_vbus-regulator.c
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
// Qualcomm PMIC VBUS output regulator driver
//
// Copyright (c) 2020, The Linux Foundation. All rights reserved.

pub const CMD_OTG: c_uint = 0x40;

pub const OTG_CURRENT_LIMIT_CFG: c_uint = 0x52;

pub const OTG_CFG: c_uint = 0x53;

pub const PM4125_VBOOST_EN: c_uint = 0x50;
pub const PM4125_VBOOST_SEL: c_uint = 0x52;

pub const PM4125_VBOOST_CFG: c_uint = 0x56;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_usb_vbus_reg_data {
    pub cmd_otg: u16,
    pub otg_cfg: u16,
    pub otg_en_src_cfg: u8,
    pub csel_reg: u16,
    pub csel_mask: u8,
    pub curr_table: *const c_uint,
    pub n_current_limits: c_uint,
    pub vsel_reg: u16,
    pub vsel_mask: u8,
    pub volt_table: *const c_uint,
    pub n_voltages: c_uint,
    pub ops: *const regulator_ops,
}

    static const unsigned int curr_table[] = {
    500000, 1000000, 1500000, 2000000, 2500000, 3000000,
    };
    static const unsigned int pm4125_vboost_table[] = {
    4250000, 4500000, 4750000, 5000000,
    };
    static const struct regulator_ops qcom_usb_vbus_reg_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .get_current_limit = regulator_get_current_limit_regmap,
    .set_current_limit = regulator_set_current_limit_regmap,
    };
    static const struct qcom_usb_vbus_reg_data pm8150b_data = {
    .cmd_otg = CMD_OTG,
    .otg_cfg = OTG_CFG,
    .otg_en_src_cfg = OTG_EN_SRC_CFG,
    .csel_reg = OTG_CURRENT_LIMIT_CFG,
    .csel_mask = OTG_CURRENT_LIMIT_MASK,
    .curr_table = curr_table,
    .n_current_limits = ARRAY_SIZE(curr_table),
    .ops = &qcom_usb_vbus_reg_ops,
    };
    static const struct regulator_ops qcom_usb_vbus_pm4125_reg_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .list_voltage = regulator_list_voltage_table,
    };
    static const struct qcom_usb_vbus_reg_data pm4125_data = {
    .cmd_otg = PM4125_VBOOST_EN,
    .otg_cfg = PM4125_VBOOST_CFG,
    .otg_en_src_cfg = PM4125_VBOOST_EN_SRC_CFG,
    .vsel_reg = PM4125_VBOOST_SEL,
    .vsel_mask = PM4125_VBOOST_CFG_MASK,
    .volt_table = pm4125_vboost_table,
    .n_voltages = ARRAY_SIZE(pm4125_vboost_table),
    .ops = &qcom_usb_vbus_pm4125_reg_ops,
    };
#[no_mangle]
unsafe extern "C" fn qcom_usb_vbus_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_usb_vbus_regulator_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct qcom_usb_vbus_reg_data *data;
    struct regulator_dev *rdev;
    struct regulator_desc *rdesc;
    struct regmap *regmap;
    let mut config: regulator_config = { };
    struct regulator_init_data *init_data;
    int ret;
    u32 base;
    ret = of_property_read_u32(dev.of_node, "reg", &base);
    if (ret < 0) {
    dev_err(dev, "no base address found\n");
    return ret;
    }
    data = of_device_get_match_data(dev);
    if (!data)
    return -EINVAL;
    regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!regmap) {
    dev_err(dev, "Failed to get regmap\n");
    return -ENOENT;
    }
    rdesc = devm_kzalloc(dev, sizeof(*rdesc), GFP_KERNEL);
    if (!rdesc)
    return -ENOMEM;
    rdesc.name = "usb_vbus";
    rdesc.ops = data.ops;
    rdesc.owner = THIS_MODULE;
    rdesc.type = REGULATOR_VOLTAGE;
    rdesc.enable_reg = base + data.cmd_otg;
    rdesc.enable_mask = OTG_EN;
    if (data.curr_table) {
    rdesc.curr_table = data.curr_table;
    rdesc.n_current_limits = data.n_current_limits;
    rdesc.csel_reg = base + data.csel_reg;
    rdesc.csel_mask = data.csel_mask;
    }
    if (data.volt_table) {
    rdesc.volt_table = data.volt_table;
    rdesc.n_voltages = data.n_voltages;
    rdesc.vsel_reg = base + data.vsel_reg;
    rdesc.vsel_mask = data.vsel_mask;
    }
    init_data = of_get_regulator_init_data(dev, dev.of_node, rdesc);
    if (!init_data)
    return -ENOMEM;
    config.dev = dev;
    config.init_data = init_data;
    config.of_node = dev.of_node;
    config.regmap = regmap;
    rdev = devm_regulator_register(dev, rdesc, &config);
    if (IS_ERR(rdev)) {
    ret = PTR_ERR(rdev);
    dev_err(dev, "not able to register vbus reg %d\n", ret);
    return ret;
    }
// Disable HW logic for VBUS enable
    regmap_update_bits(regmap, base + data.otg_cfg, data.otg_en_src_cfg, 0);
    return 0;
    }
    static const struct of_device_id qcom_usb_vbus_regulator_match[] = {
    { .compatible = "qcom,pm8150b-vbus-reg", .data = &pm8150b_data },
    { .compatible = "qcom,pm4125-vbus-reg",  .data = &pm4125_data },
    { }
    };
    MODULE_DEVICE_TABLE(of, qcom_usb_vbus_regulator_match);
    static struct platform_driver qcom_usb_vbus_regulator_driver = {
    .driver		= {
    .name	= "qcom-usb-vbus-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = qcom_usb_vbus_regulator_match,
    },
    .probe		= qcom_usb_vbus_regulator_probe,
    };
    module_platform_driver(qcom_usb_vbus_regulator_driver);
    MODULE_DESCRIPTION("Qualcomm USB vbus regulator driver");
    MODULE_LICENSE("GPL v2");
