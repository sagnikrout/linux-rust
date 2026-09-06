//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/rt4831-regulator.c
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

    enum {
    DSV_OUT_VLCM = 0,
    DSV_OUT_VPOS,
    DSV_OUT_VNEG,
    DSV_OUT_MAX
    };
pub const RT4831_REG_DSVEN: c_uint = 0x09;
pub const RT4831_REG_VLCM: c_uint = 0x0c;
pub const RT4831_REG_VPOS: c_uint = 0x0d;
pub const RT4831_REG_VNEG: c_uint = 0x0e;
pub const RT4831_REG_FLAGS: c_uint = 0x0f;

pub const RT4831_DSVMODE_SHIFT: c_int = 5;

pub const STEP_UV: c_int = 50000;
pub const VLCM_MIN_UV: c_int = 4000000;
pub const VLCM_MAX_UV: c_int = 7150000;

pub const VPN_MIN_UV: c_int = 4000000;
pub const VPN_MAX_UV: c_int = 6500000;

#[no_mangle]
unsafe extern "C" fn rt4831_get_error_flags(rdev: *mut regulator_dev, flags: *mut c_uint) -> c_int {
    static int rt4831_get_error_flags(struct regulator_dev *rdev, unsigned int *flags)
    {
    struct regmap *regmap = rdev_get_regmap(rdev);
    let mut rid: c_int = rdev_get_id(rdev);
    unsigned int val, events = 0;
    int ret;
    ret = regmap_read(regmap, RT4831_REG_FLAGS, &val);
    if (ret)
    return ret;
    if (val & RT4831_OTP_MASK)
    events |= REGULATOR_ERROR_OVER_TEMP;
    if (rid == DSV_OUT_VLCM && (val & RT4831_LCMOVP_MASK))
    events |= REGULATOR_ERROR_OVER_CURRENT;
    if (rid == DSV_OUT_VPOS && (val & RT4831_VPOSSCP_MASK))
    events |= REGULATOR_ERROR_OVER_CURRENT;
    if (rid == DSV_OUT_VNEG && (val & RT4831_VNEGSCP_MASK))
    events |= REGULATOR_ERROR_OVER_CURRENT;
// flags = events;
    return 0;
    }
    static const struct regulator_ops rt4831_dsvlcm_ops = {
    .list_voltage = regulator_list_voltage_linear,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_bypass = regulator_set_bypass_regmap,
    .get_bypass = regulator_get_bypass_regmap,
    .get_error_flags = rt4831_get_error_flags,
    };
    static const struct regulator_ops rt4831_dsvpn_ops = {
    .list_voltage = regulator_list_voltage_linear,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .set_active_discharge = regulator_set_active_discharge_regmap,
    .get_error_flags = rt4831_get_error_flags,
    };
    static const struct regulator_desc rt4831_regulator_descs[] = {
    {
    .name = "DSVLCM",
    .ops = &rt4831_dsvlcm_ops,
    .of_match = of_match_ptr("DSVLCM"),
    .regulators_node = of_match_ptr("regulators"),
    .type = REGULATOR_VOLTAGE,
    .id = DSV_OUT_VLCM,
    .n_voltages = VLCM_N_VOLTAGES,
    .min_uV = VLCM_MIN_UV,
    .uV_step = STEP_UV,
    .vsel_reg = RT4831_REG_VLCM,
    .vsel_mask = RT4831_VOLT_MASK,
    .bypass_reg = RT4831_REG_DSVEN,
    .bypass_mask = RT4831_DSVMODE_MASK,
    .bypass_val_on = DSV_MODE_BYPASS,
    .bypass_val_off = DSV_MODE_NORMAL,
    .owner = THIS_MODULE,
    },
    {
    .name = "DSVP",
    .ops = &rt4831_dsvpn_ops,
    .of_match = of_match_ptr("DSVP"),
    .regulators_node = of_match_ptr("regulators"),
    .type = REGULATOR_VOLTAGE,
    .id = DSV_OUT_VPOS,
    .n_voltages = VPN_N_VOLTAGES,
    .min_uV = VPN_MIN_UV,
    .uV_step = STEP_UV,
    .vsel_reg = RT4831_REG_VPOS,
    .vsel_mask = RT4831_VOLT_MASK,
    .enable_reg = RT4831_REG_DSVEN,
    .enable_mask = RT4831_POSEN_MASK,
    .active_discharge_reg = RT4831_REG_DSVEN,
    .active_discharge_mask = RT4831_POSADEN_MASK,
    .active_discharge_on = RT4831_POSADEN_MASK,
    .owner = THIS_MODULE,
    },
    {
    .name = "DSVN",
    .ops = &rt4831_dsvpn_ops,
    .of_match = of_match_ptr("DSVN"),
    .regulators_node = of_match_ptr("regulators"),
    .type = REGULATOR_VOLTAGE,
    .id = DSV_OUT_VNEG,
    .n_voltages = VPN_N_VOLTAGES,
    .min_uV = VPN_MIN_UV,
    .uV_step = STEP_UV,
    .vsel_reg = RT4831_REG_VNEG,
    .vsel_mask = RT4831_VOLT_MASK,
    .enable_reg = RT4831_REG_DSVEN,
    .enable_mask = RT4831_NEGEN_MASK,
    .active_discharge_reg = RT4831_REG_DSVEN,
    .active_discharge_mask = RT4831_NEGADEN_MASK,
    .active_discharge_on = RT4831_NEGADEN_MASK,
    .owner = THIS_MODULE,
    }
    };
#[no_mangle]
unsafe extern "C" fn rt4831_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int rt4831_regulator_probe(struct platform_device *pdev)
    {
    struct regmap *regmap;
    struct regulator_dev *rdev;
    let mut config: regulator_config = {};
    int i, ret;
    regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!regmap) {
    dev_err(&pdev.dev, "Failed to init regmap\n");
    return -ENODEV;
    }
// Configure DSV mode to normal by default
    ret = regmap_update_bits(regmap, RT4831_REG_DSVEN, RT4831_DSVMODE_MASK, DSV_MODE_NORMAL);
    if (ret) {
    dev_err(&pdev.dev, "Failed to configure dsv mode to normal\n");
    return ret;
    }
    config.dev = pdev.dev.parent;
    config.regmap = regmap;
    for (i = 0; i < DSV_OUT_MAX; i++) {
    rdev = devm_regulator_register(&pdev.dev, rt4831_regulator_descs + i, &config);
    if (IS_ERR(rdev)) {
    dev_err(&pdev.dev, "Failed to register %d regulator\n", i);
    return PTR_ERR(rdev);
    }
    }
    return 0;
    }
    static const struct platform_device_id rt4831_regulator_match[] = {
    { .name = "rt4831-regulator" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, rt4831_regulator_match);
    static struct platform_driver rt4831_regulator_driver = {
    .driver = {
    .name = "rt4831-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .id_table = rt4831_regulator_match,
    .probe = rt4831_regulator_probe,
    };
    module_platform_driver(rt4831_regulator_driver);
    MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>");
    MODULE_DESCRIPTION("Richtek RT4831 DSV Regulators driver");
    MODULE_LICENSE("GPL v2");
