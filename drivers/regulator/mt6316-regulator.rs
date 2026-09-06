//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/mt6316-regulator.c
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
// Copyright (c) 2024 MediaTek Inc.
// Copyright (c) 2025 Collabora Ltd
// AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>

pub const MT6316_BUCK_MODE_AUTO: c_int = 0;
pub const MT6316_BUCK_MODE_FORCE_PWM: c_int = 1;
pub const MT6316_BUCK_MODE_LP: c_int = 2;
pub const MT6316_CHIP_ID: c_uint = 0x20b;
pub const MT6316_BUCK_TOP_CON0: c_uint = 0x1440;
pub const EN_SET_OFFSET: c_uint = 0x1;
pub const EN_CLR_OFFSET: c_uint = 0x2;
pub const MT6316_BUCK_TOP_CON1: c_uint = 0x1443;
pub const MT6316_BUCK_TOP_ELR0: c_uint = 0x1448;
pub const MT6316_BUCK_TOP_ELR2: c_uint = 0x144a;
pub const MT6316_BUCK_TOP_ELR4: c_uint = 0x144c;
pub const MT6316_BUCK_TOP_ELR6: c_uint = 0x144e;

pub const MT6316_VBUCK1_DBG: c_uint = 0x14a8;
pub const MT6316_VBUCK2_DBG: c_uint = 0x1528;
pub const MT6316_VBUCK3_DBG: c_uint = 0x15a8;
pub const MT6316_VBUCK4_DBG: c_uint = 0x1628;

pub const MT6316_BUCK_TOP_4PHASE_TOP_ANA_CON0: c_uint = 0x1688;
pub const MT6316_BUCK_TOP_4PHASE_TOP_ELR_0: c_uint = 0x1690;
    enum mt6316_type {
    MT6316_TYPE_2PHASE,
    MT6316_TYPE_3PHASE,
    MT6316_TYPE_4PHASE
    };
//
// struct mt6316_regulator_info - MT6316 regulators information
// @desc: Regulator description structure
// @debug_reg: Debug register for regulator status
// @lp_mode_reg: Low Power mode register (normal/idle)
// @lp_mode_mask: Low Power mode regulator mask
// @modeset_reg: AUTO/PWM mode register
// @modeset_mask: AUTO/PWM regulator mask
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6316_regulator_info {
    pub desc: regulator_desc,
    pub debug_reg: u16,
    pub lp_mode_reg: u16,
    pub lp_mode_mask: u16,
    pub modeset_reg: u16,
    pub modeset_mask: u16,
}

    {									\
    .desc = {							\
    .name = match,						\
    .of_match = of_match_ptr(match),			\
    .ops = &mt6316_vreg_setclr_ops,				\
    .type = REGULATOR_VOLTAGE,				\
    .owner = THIS_MODULE,					\
    .n_voltages = (max - min) / step + 1,			\
    .min_uV = min,						\
    .uV_step = step,					\
    .enable_reg = MT6316_BUCK_TOP_CON0,			\
    .enable_mask = BIT(vreg_id - 1),			\
    .vsel_reg = vs_reg,					\
    .vsel_mask = MT6316_VSEL_MASK,				\
    .of_map_mode = mt6316_map_mode,				\
    },								\
    .lp_mode_reg = MT6316_BUCK_TOP_CON1,				\
    .lp_mode_mask = BIT(vreg_id - 1),				\
    .modeset_reg = MT6316_BUCK_TOP_4PHASE_TOP_ANA_CON0,		\
    .modeset_mask = BIT(vreg_id - 1),				\
    .debug_reg = MT6316_VBUCK##vreg_id##_DBG,			\
    }
// Values in some MT6316 registers are big endian, 9 bits long...
#[no_mangle]
pub unsafe extern "C" fn mt6316_be9_to_cpu(val: u16) -> u16 {
    static inline u16 mt6316_be9_to_cpu(u16 val)
    {
    return ((val >> 8) & BIT(0)) | ((val & GENMASK(7, 0)) << 1);
    }
#[no_mangle]
pub unsafe extern "C" fn mt6316_cpu_to_be9(val: u16) -> u16 {
    static inline u16 mt6316_cpu_to_be9(u16 val)
    {
    return ((val & BIT(0)) << 8) | (val >> 1);
    }
#[no_mangle]
unsafe extern "C" fn mt6316_map_mode(mode: u32) -> c_uint {
    static unsigned int mt6316_map_mode(u32 mode)
    {
    switch (mode) {
    case MT6316_BUCK_MODE_AUTO:
    return REGULATOR_MODE_NORMAL;
    case MT6316_BUCK_MODE_FORCE_PWM:
    return REGULATOR_MODE_FAST;
    case MT6316_BUCK_MODE_LP:
    return REGULATOR_MODE_IDLE;
    default:
    return REGULATOR_MODE_INVALID;
    }
    }
#[no_mangle]
unsafe extern "C" fn mt6316_vreg_enable_setclr(rdev: *mut regulator_dev) -> c_int {
    static int mt6316_vreg_enable_setclr(struct regulator_dev *rdev)
    {
    return regmap_write(rdev.regmap, rdev.desc.enable_reg + EN_SET_OFFSET,
    rdev.desc.enable_mask);
    }
#[no_mangle]
unsafe extern "C" fn mt6316_vreg_disable_setclr(rdev: *mut regulator_dev) -> c_int {
    static int mt6316_vreg_disable_setclr(struct regulator_dev *rdev)
    {
    return regmap_write(rdev.regmap, rdev.desc.enable_reg + EN_CLR_OFFSET,
    rdev.desc.enable_mask);
    }
#[no_mangle]
unsafe extern "C" fn mt6316_regulator_set_voltage_sel(rdev: *mut regulator_dev, selector: c_uint) -> c_int {
    static int mt6316_regulator_set_voltage_sel(struct regulator_dev *rdev, unsigned int selector)
    {
    let mut val: u16 = mt6316_cpu_to_be9(selector);
    return regmap_bulk_write(rdev.regmap, rdev.desc.vsel_reg, &val, sizeof(val));
    }
#[no_mangle]
unsafe extern "C" fn mt6316_regulator_get_voltage_sel(rdev: *mut regulator_dev) -> c_int {
    static int mt6316_regulator_get_voltage_sel(struct regulator_dev *rdev)
    {
    u16 val;
    int ret;
    ret = regmap_bulk_read(rdev.regmap, rdev.desc.vsel_reg, &val, sizeof(val));
    if (ret)
    return ret;
    return mt6316_be9_to_cpu(val & rdev.desc.vsel_mask);
    }
#[no_mangle]
unsafe extern "C" fn mt6316_regulator_get_status(rdev: *mut regulator_dev) -> c_int {
    static int mt6316_regulator_get_status(struct regulator_dev *rdev)
    {
    struct mt6316_regulator_info *info = rdev_get_drvdata(rdev);
    u32 val;
    int ret;
    ret = regmap_read(rdev.regmap, info.debug_reg, &val);
    if (ret)
    return ret;
    return val & MT6316_BUCK_QI ? REGULATOR_STATUS_ON : REGULATOR_STATUS_OFF;
    }
#[no_mangle]
unsafe extern "C" fn mt6316_regulator_get_mode(rdev: *mut regulator_dev) -> c_uint {
    static unsigned int mt6316_regulator_get_mode(struct regulator_dev *rdev)
    {
    struct mt6316_regulator_info *info = rdev_get_drvdata(rdev);
    unsigned int val;
    int ret;
    ret = regmap_read(rdev.regmap, info.modeset_reg, &val);
    if (ret) {
    dev_err(&rdev.dev, "Failed to get mode: %d\n", ret);
    return ret;
    }
    if ((val & info.modeset_mask) == info.modeset_mask)
    return REGULATOR_MODE_FAST;
    ret = regmap_read(rdev.regmap, info.lp_mode_reg, &val);
    val &= info.lp_mode_mask;
    if (ret) {
    dev_err(&rdev.dev, "Failed to get lp mode: %d\n", ret);
    return ret;
    }
    return val ? REGULATOR_MODE_IDLE : REGULATOR_MODE_NORMAL;
    }
    static int mt6316_regulator_set_mode(struct regulator_dev *rdev,
    unsigned int mode)
    {
    struct mt6316_regulator_info *info = rdev_get_drvdata(rdev);
    struct regmap *regmap = rdev.regmap;
    int cur_mode, ret;
    switch (mode) {
    case REGULATOR_MODE_FAST:
    ret = regmap_set_bits(regmap, info.modeset_reg, info.modeset_mask);
    break;
    case REGULATOR_MODE_NORMAL:
    cur_mode = mt6316_regulator_get_mode(rdev);
    if (cur_mode < 0) {
    ret = cur_mode;
    break;
    }
    if (cur_mode == REGULATOR_MODE_FAST) {
    ret = regmap_clear_bits(regmap, info.modeset_reg, info.modeset_mask);
    break;
    } else if (cur_mode == REGULATOR_MODE_IDLE) {
    ret = regmap_clear_bits(regmap, info.lp_mode_reg, info.lp_mode_mask);
    if (ret == 0)
    usleep_range(100, 200);
    } else {
    ret = 0;
    }
    break;
    case REGULATOR_MODE_IDLE:
    ret = regmap_set_bits(regmap, info.lp_mode_reg, info.lp_mode_mask);
    break;
    default:
    ret = -EINVAL;
    }
    if (ret) {
    dev_err(&rdev.dev, "Failed to set mode %u: %d\n", mode, ret);
    return ret;
    }
    return 0;
    }
    static const struct regulator_ops mt6316_vreg_setclr_ops = {
    .list_voltage = regulator_list_voltage_linear,
    .map_voltage = regulator_map_voltage_linear,
    .set_voltage_sel = mt6316_regulator_set_voltage_sel,
    .get_voltage_sel = mt6316_regulator_get_voltage_sel,
    .set_voltage_time_sel = regulator_set_voltage_time_sel,
    .enable = mt6316_vreg_enable_setclr,
    .disable = mt6316_vreg_disable_setclr,
    .is_enabled = regulator_is_enabled_regmap,
    .get_status = mt6316_regulator_get_status,
    .set_mode = mt6316_regulator_set_mode,
    .get_mode = mt6316_regulator_get_mode,
    };
// MT6316BP/VP - 2+2 phase buck
    static struct mt6316_regulator_info mt6316bv_regulators[] = {
    MT6316_BUCK("vbuck12", 1, 0, 1277500, 2500, MT6316_BUCK_TOP_ELR0),
    MT6316_BUCK("vbuck34", 3, 0, 1277500, 2500, MT6316_BUCK_TOP_ELR4),
    };
// MT6316CP/HP/KP - 3+1 phase buck
    static struct mt6316_regulator_info mt6316chk_regulators[] = {
    MT6316_BUCK("vbuck124", 1, 0, 1277500, 2500, MT6316_BUCK_TOP_ELR0),
    MT6316_BUCK("vbuck3", 3, 0, 1277500, 2500, MT6316_BUCK_TOP_ELR4),
    };
// MT6316DP/TP - 4 phase buck
    static struct mt6316_regulator_info mt6316dt_regulators[] = {
    MT6316_BUCK("vbuck1234", 1, 0, 1277500, 2500, MT6316_BUCK_TOP_ELR0),
    };
    static const struct regmap_config mt6316_spmi_regmap_config = {
    .reg_bits	= 16,
    .val_bits	= 8,
    .max_register	= 0x1700,
    .fast_io	= true,
    };
#[no_mangle]
unsafe extern "C" fn mt6316_regulator_probe(sdev: *mut spmi_device) -> c_int {
    static int mt6316_regulator_probe(struct spmi_device *sdev)
    {
    let mut config: regulator_config = {};
    struct mt6316_regulator_info *info;
    struct regulator_dev *rdev;
    enum mt6316_type type;
    int num_vregs, ret;
    unsigned int i;
    u32 chip_id;
    config.regmap = devm_regmap_init_spmi_ext(sdev, &mt6316_spmi_regmap_config);
    if (IS_ERR(config.regmap))
    return PTR_ERR(config.regmap);
//
// The first read is expected to fail: this PMIC needs to be woken up
// and that can be done with any activity over the SPMI bus.
//
    regmap_read(config.regmap, MT6316_CHIP_ID, &chip_id);
// The second read, instead, shall not fail!
    ret = regmap_read(config.regmap, MT6316_CHIP_ID, &chip_id);
    if (ret) {
    dev_err(&sdev.dev, "Cannot read Chip ID!\n");
    return ret;
    }
    dev_dbg(&sdev.dev, "Chip ID: 0x%x\n", chip_id);
    config.dev = &sdev.dev;
    type = (uintptr_t)device_get_match_data(&sdev.dev);
    switch (type) {
    case MT6316_TYPE_2PHASE:
    info = mt6316bv_regulators;
    num_vregs = ARRAY_SIZE(mt6316bv_regulators);
    break;
    case MT6316_TYPE_3PHASE:
    info = mt6316chk_regulators;
    num_vregs = ARRAY_SIZE(mt6316chk_regulators);
    break;
    case MT6316_TYPE_4PHASE:
    info = mt6316dt_regulators;
    num_vregs = ARRAY_SIZE(mt6316dt_regulators);
    break;
    default:
    return -EINVAL;
    }
    for (i = 0; i < num_vregs; i++) {
    config.driver_data = &info[i];
    rdev = devm_regulator_register(&sdev.dev, &info[i].desc, &config);
    if (IS_ERR(rdev))
    return dev_err_probe(&sdev.dev, PTR_ERR(rdev),
    "failed to register %s\n", info[i].desc.name);
    }
    return 0;
    }
    static const struct of_device_id mt6316_regulator_match[] = {
    { .compatible = "mediatek,mt6316b-regulator", .data = (void *)MT6316_TYPE_2PHASE },
    { .compatible = "mediatek,mt6316c-regulator", .data = (void *)MT6316_TYPE_3PHASE },
    { .compatible = "mediatek,mt6316d-regulator", .data = (void *)MT6316_TYPE_4PHASE },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, mt6316_regulator_match);
    static struct spmi_driver mt6316_regulator_driver = {
    .driver = {
    .name = "mt6316-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = mt6316_regulator_match,
    },
    .probe = mt6316_regulator_probe,
    };
    module_spmi_driver(mt6316_regulator_driver);
    MODULE_AUTHOR("AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>");
    MODULE_DESCRIPTION("Regulator Driver for MediaTek MT6316 PMIC");
    MODULE_LICENSE("GPL");
