//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/rtq2134-regulator.c
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

    enum {
    RTQ2134_IDX_BUCK1 = 0,
    RTQ2134_IDX_BUCK2,
    RTQ2134_IDX_BUCK3,
    RTQ2134_IDX_MAX
    };
pub const RTQ2134_AUTO_MODE: c_int = 0;
pub const RTQ2134_FCCM_MODE: c_int = 1;
pub const RTQ2134_BUCK_DVS0_CTRL: c_int = 0;
pub const RTQ2134_BUCK_VSEL_CTRL: c_int = 2;
pub const RTQ2134_REG_IO_CHIPNAME: c_uint = 0x01;
pub const RTQ2134_REG_FLT_RECORDTEMP: c_uint = 0x13;

pub const RTQ2134_REG_BUCK1_CFG0: c_uint = 0x42;
pub const RTQ2134_REG_BUCK1_DVS0CFG1: c_uint = 0x48;
pub const RTQ2134_REG_BUCK1_DVS0CFG0: c_uint = 0x49;
pub const RTQ2134_REG_BUCK1_DVS1CFG1: c_uint = 0x4A;
pub const RTQ2134_REG_BUCK1_DVS1CFG0: c_uint = 0x4B;
pub const RTQ2134_REG_BUCK1_DVSCFG: c_uint = 0x52;
pub const RTQ2134_REG_BUCK1_RSPCFG: c_uint = 0x54;
pub const RTQ2134_REG_BUCK2_CFG0: c_uint = 0x5F;
pub const RTQ2134_REG_BUCK2_DVS0CFG1: c_uint = 0x62;
pub const RTQ2134_REG_BUCK2_DVS0CFG0: c_uint = 0x63;
pub const RTQ2134_REG_BUCK2_DVS1CFG1: c_uint = 0x64;
pub const RTQ2134_REG_BUCK2_DVS1CFG0: c_uint = 0x65;
pub const RTQ2134_REG_BUCK2_DVSCFG: c_uint = 0x6C;
pub const RTQ2134_REG_BUCK2_RSPCFG: c_uint = 0x6E;
pub const RTQ2134_REG_BUCK3_CFG0: c_uint = 0x79;
pub const RTQ2134_REG_BUCK3_DVS0CFG1: c_uint = 0x7C;
pub const RTQ2134_REG_BUCK3_DVS0CFG0: c_uint = 0x7D;
pub const RTQ2134_REG_BUCK3_DVS1CFG1: c_uint = 0x7E;
pub const RTQ2134_REG_BUCK3_DVS1CFG0: c_uint = 0x7F;
pub const RTQ2134_REG_BUCK3_DVSCFG: c_uint = 0x86;
pub const RTQ2134_REG_BUCK3_RSPCFG: c_uint = 0x88;
pub const RTQ2134_REG_BUCK3_SLEWCTRL: c_uint = 0x89;
pub const RTQ2134_VOUT_MAXNUM: c_int = 256;
pub const RTQ2134_VOUT_MASK: c_uint = 0xFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtq2134_regulator_desc {
    pub desc: regulator_desc,
// Extension for proprietary register and mask
    pub mode_reg: c_uint,
    pub mode_mask: c_uint,
    pub suspend_enable_reg: c_uint,
    pub suspend_enable_mask: c_uint,
    pub suspend_vsel_reg: c_uint,
    pub suspend_vsel_mask: c_uint,
    pub suspend_mode_reg: c_uint,
    pub suspend_mode_mask: c_uint,
    pub dvs_ctrl_reg: c_uint,
}

#[no_mangle]
unsafe extern "C" fn rtq2134_buck_set_mode(rdev: *mut regulator_dev, mode: c_uint) -> c_int {
    static int rtq2134_buck_set_mode(struct regulator_dev *rdev, unsigned int mode)
    {
    struct rtq2134_regulator_desc *desc =
    (struct rtq2134_regulator_desc *)rdev.desc;
    unsigned int val;
    if (mode == REGULATOR_MODE_NORMAL)
    val = RTQ2134_AUTO_MODE;
#[no_mangle]
pub unsafe extern "C" fn if(REGULATOR_MODE_FAST: mode ==) -> else {
    else if (mode == REGULATOR_MODE_FAST)
    val = RTQ2134_FCCM_MODE;
    else
    return -EINVAL;
    val <<= ffs(desc.mode_mask) - 1;
    return regmap_update_bits(rdev.regmap, desc.mode_reg, desc.mode_mask,
    val);
    }
#[no_mangle]
unsafe extern "C" fn rtq2134_buck_get_mode(rdev: *mut regulator_dev) -> c_uint {
    static unsigned int rtq2134_buck_get_mode(struct regulator_dev *rdev)
    {
    struct rtq2134_regulator_desc *desc =
    (struct rtq2134_regulator_desc *)rdev.desc;
    unsigned int mode;
    int ret;
    ret = regmap_read(rdev.regmap, desc.mode_reg, &mode);
    if (ret)
    return ret;
    if (mode & desc.mode_mask)
    return REGULATOR_MODE_FAST;
    return REGULATOR_MODE_NORMAL;
    }
#[no_mangle]
unsafe extern "C" fn rtq2134_buck_set_suspend_voltage(rdev: *mut regulator_dev, uV: c_int) -> c_int {
    static int rtq2134_buck_set_suspend_voltage(struct regulator_dev *rdev, int uV)
    {
    struct rtq2134_regulator_desc *desc =
    (struct rtq2134_regulator_desc *)rdev.desc;
    int sel;
    sel = regulator_map_voltage_linear_range(rdev, uV, uV);
    if (sel < 0)
    return sel;
    sel <<= ffs(desc.suspend_vsel_mask) - 1;
    return regmap_update_bits(rdev.regmap, desc.suspend_vsel_reg,
    desc.suspend_vsel_mask, sel);
    }
#[no_mangle]
unsafe extern "C" fn rtq2134_buck_set_suspend_enable(rdev: *mut regulator_dev) -> c_int {
    static int rtq2134_buck_set_suspend_enable(struct regulator_dev *rdev)
    {
    struct rtq2134_regulator_desc *desc =
    (struct rtq2134_regulator_desc *)rdev.desc;
    let mut val: c_uint = desc.suspend_enable_mask;
    return regmap_update_bits(rdev.regmap, desc.suspend_enable_reg,
    desc.suspend_enable_mask, val);
    }
#[no_mangle]
unsafe extern "C" fn rtq2134_buck_set_suspend_disable(rdev: *mut regulator_dev) -> c_int {
    static int rtq2134_buck_set_suspend_disable(struct regulator_dev *rdev)
    {
    struct rtq2134_regulator_desc *desc =
    (struct rtq2134_regulator_desc *)rdev.desc;
    return regmap_update_bits(rdev.regmap, desc.suspend_enable_reg,
    desc.suspend_enable_mask, 0);
    }
    static int rtq2134_buck_set_suspend_mode(struct regulator_dev *rdev,
    unsigned int mode)
    {
    struct rtq2134_regulator_desc *desc =
    (struct rtq2134_regulator_desc *)rdev.desc;
    unsigned int val;
    if (mode == REGULATOR_MODE_NORMAL)
    val = RTQ2134_AUTO_MODE;
#[no_mangle]
pub unsafe extern "C" fn if(REGULATOR_MODE_FAST: mode ==) -> else {
    else if (mode == REGULATOR_MODE_FAST)
    val = RTQ2134_FCCM_MODE;
    else
    return -EINVAL;
    val <<= ffs(desc.suspend_mode_mask) - 1;
    return regmap_update_bits(rdev.regmap, desc.suspend_mode_reg,
    desc.suspend_mode_mask, val);
    }
    static int rtq2134_buck_get_error_flags(struct regulator_dev *rdev,
    unsigned int *flags)
    {
    let mut rid: c_int = rdev_get_id(rdev);
    unsigned int chip_error, buck_error, events = 0;
    int ret;
    ret = regmap_read(rdev.regmap, RTQ2134_REG_FLT_RECORDTEMP,
    &chip_error);
    if (ret) {
    dev_err(&rdev.dev, "Failed to get chip error flag\n");
    return ret;
    }
    ret = regmap_read(rdev.regmap, RTQ2134_REG_FLT_RECORDBUCK(rid),
    &buck_error);
    if (ret) {
    dev_err(&rdev.dev, "Failed to get buck error flag\n");
    return ret;
    }
    if (chip_error & RTQ2134_CHIPOT_MASK)
    events |= REGULATOR_ERROR_OVER_TEMP;
    if (buck_error & RTQ2134_BUCKUV_MASK)
    events |= REGULATOR_ERROR_UNDER_VOLTAGE;
    if (buck_error & RTQ2134_BUCKOV_MASK)
    events |= REGULATOR_ERROR_REGULATION_OUT;
// flags = events;
    return 0;
    }
    static const struct regulator_ops rtq2134_buck_ops = {
    .list_voltage = regulator_list_voltage_linear_range,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .set_active_discharge = regulator_set_active_discharge_regmap,
    .set_ramp_delay = regulator_set_ramp_delay_regmap,
    .set_mode = rtq2134_buck_set_mode,
    .get_mode = rtq2134_buck_get_mode,
    .set_suspend_voltage = rtq2134_buck_set_suspend_voltage,
    .set_suspend_enable = rtq2134_buck_set_suspend_enable,
    .set_suspend_disable = rtq2134_buck_set_suspend_disable,
    .set_suspend_mode = rtq2134_buck_set_suspend_mode,
    .get_error_flags = rtq2134_buck_get_error_flags,
    };
    static const struct linear_range rtq2134_buck_vout_ranges[] = {
    REGULATOR_LINEAR_RANGE(300000, 0, 200, 5000),
    REGULATOR_LINEAR_RANGE(1310000, 201, 255, 10000)
    };
#[no_mangle]
unsafe extern "C" fn rtq2134_buck_of_map_mode(mode: c_uint) -> c_uint {
    static unsigned int rtq2134_buck_of_map_mode(unsigned int mode)
    {
    switch (mode) {
    case RTQ2134_AUTO_MODE:
    return REGULATOR_MODE_NORMAL;
    case RTQ2134_FCCM_MODE:
    return REGULATOR_MODE_FAST;
    }
    return REGULATOR_MODE_INVALID;
    }
    static int rtq2134_buck_of_parse_cb(struct device_node *np,
    const struct regulator_desc *desc,
    struct regulator_config *cfg)
    {
    struct rtq2134_regulator_desc *rdesc =
    (struct rtq2134_regulator_desc *)desc;
    let mut rid: c_int = desc.id;
    bool uv_shutdown, vsel_dvs;
    unsigned int val;
    int ret;
    vsel_dvs = of_property_read_bool(np, "richtek,use-vsel-dvs");
    if (vsel_dvs)
    val = RTQ2134_BUCK_VSEL_CTRL;
    else
    val = RTQ2134_BUCK_DVS0_CTRL;
    ret = regmap_update_bits(cfg.regmap, rdesc.dvs_ctrl_reg,
    RTQ2134_BUCKDVS_CTRL_MASK, val);
    if (ret)
    return ret;
    uv_shutdown = of_property_read_bool(np, "richtek,uv-shutdown");
    if (uv_shutdown)
    val = 0;
    else
    val = RTQ2134_UVHICCUP_MASK;
    return regmap_update_bits(cfg.regmap, RTQ2134_REG_FLT_BUCKCTRL(rid),
    RTQ2134_UVHICCUP_MASK, val);
    }
    static const unsigned int rtq2134_buck_ramp_delay_table[] = {
    0, 16000, 0, 8000, 4000, 2000, 1000, 500
    };

    .desc = { \
    .name = "rtq2134_buck" #_id, \
    .of_match = of_match_ptr("buck" #_id), \
    .regulators_node = of_match_ptr("regulators"), \
    .id = RTQ2134_IDX_BUCK##_id, \
    .type = REGULATOR_VOLTAGE, \
    .owner = THIS_MODULE, \
    .ops = &rtq2134_buck_ops, \
    .n_voltages = RTQ2134_VOUT_MAXNUM, \
    .linear_ranges = rtq2134_buck_vout_ranges, \
    .n_linear_ranges = ARRAY_SIZE(rtq2134_buck_vout_ranges), \
    .vsel_reg = RTQ2134_REG_BUCK##_id##_DVS0CFG1, \
    .vsel_mask = RTQ2134_VOUT_MASK, \
    .enable_reg = RTQ2134_REG_BUCK##_id##_DVS0CFG0, \
    .enable_mask = RTQ2134_VOUTEN_MASK, \
    .active_discharge_reg = RTQ2134_REG_BUCK##_id##_CFG0, \
    .active_discharge_mask = RTQ2134_ACTDISCHG_MASK, \
    .active_discharge_on = RTQ2134_ACTDISCHG_MASK, \
    .ramp_reg = RTQ2134_REG_BUCK##_id##_RSPCFG, \
    .ramp_mask = RTQ2134_RSPUP_MASK, \
    .ramp_delay_table = rtq2134_buck_ramp_delay_table, \
    .n_ramp_values = ARRAY_SIZE(rtq2134_buck_ramp_delay_table), \
    .of_map_mode = rtq2134_buck_of_map_mode, \
    .of_parse_cb = rtq2134_buck_of_parse_cb, \
    }, \
    .mode_reg = RTQ2134_REG_BUCK##_id##_DVS0CFG0, \
    .mode_mask = RTQ2134_FCCM_MASK, \
    .suspend_mode_reg = RTQ2134_REG_BUCK##_id##_DVS1CFG0, \
    .suspend_mode_mask = RTQ2134_FCCM_MASK, \
    .suspend_enable_reg = RTQ2134_REG_BUCK##_id##_DVS1CFG0, \
    .suspend_enable_mask = RTQ2134_VOUTEN_MASK, \
    .suspend_vsel_reg = RTQ2134_REG_BUCK##_id##_DVS1CFG1, \
    .suspend_vsel_mask = RTQ2134_VOUT_MASK, \
    .dvs_ctrl_reg = RTQ2134_REG_BUCK##_id##_DVSCFG, \
    }
    static const struct rtq2134_regulator_desc rtq2134_regulator_descs[] = {
    RTQ2134_BUCK_DESC(1),
    RTQ2134_BUCK_DESC(2),
    RTQ2134_BUCK_DESC(3)
    };
#[no_mangle]
unsafe extern "C" fn rtq2134_is_accissible_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool rtq2134_is_accissible_reg(struct device *dev, unsigned int reg)
    {
    if (reg >= RTQ2134_REG_IO_CHIPNAME && reg <= RTQ2134_REG_BUCK3_SLEWCTRL)
    return true;
    return false;
    }
    static const struct regmap_config rtq2134_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = RTQ2134_REG_BUCK3_SLEWCTRL,
    .readable_reg = rtq2134_is_accissible_reg,
    .writeable_reg = rtq2134_is_accissible_reg,
    };
#[no_mangle]
unsafe extern "C" fn rtq2134_probe(i2c: *mut i2c_client) -> c_int {
    static int rtq2134_probe(struct i2c_client *i2c)
    {
    struct regmap *regmap;
    struct regulator_dev *rdev;
    let mut regulator_cfg: regulator_config = {};
    int i;
    regmap = devm_regmap_init_i2c(i2c, &rtq2134_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&i2c.dev, "Failed to allocate regmap\n");
    return PTR_ERR(regmap);
    }
    regulator_cfg.dev = &i2c.dev;
    regulator_cfg.regmap = regmap;
    for (i = 0; i < ARRAY_SIZE(rtq2134_regulator_descs); i++) {
    rdev = devm_regulator_register(&i2c.dev,
    &rtq2134_regulator_descs[i].desc,
    &regulator_cfg);
    if (IS_ERR(rdev)) {
    dev_err(&i2c.dev, "Failed to init %d regulator\n", i);
    return PTR_ERR(rdev);
    }
    }
    return 0;
    }
    static const struct of_device_id __maybe_unused rtq2134_device_tables[] = {
    { .compatible = "richtek,rtq2134", },
    {}
    };
    MODULE_DEVICE_TABLE(of, rtq2134_device_tables);
    static struct i2c_driver rtq2134_driver = {
    .driver = {
    .name = "rtq2134",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = rtq2134_device_tables,
    },
    .probe = rtq2134_probe,
    };
    module_i2c_driver(rtq2134_driver);
    MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>");
    MODULE_DESCRIPTION("Richtek RTQ2134 Regulator Driver");
    MODULE_LICENSE("GPL v2");
