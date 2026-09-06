//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/da903x-regulator.c
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
// Regulators driver for Dialog Semiconductor DA903x
//
// Copyright (C) 2006-2008 Marvell International Ltd.
// Copyright (C) 2008 Compulab Ltd.

// DA9030 Registers

// DA9034 Registers

// DA9035 Registers. DA9034 Registers are compatible to DA9035.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da903x_regulator_info {
    pub desc: regulator_desc,
    pub max_uV: c_int,
    pub vol_reg: c_int,
    pub vol_shift: c_int,
    pub vol_nbits: c_int,
    pub update_reg: c_int,
    pub update_bit: c_int,
    pub enable_reg: c_int,
    pub enable_bit: c_int,
}

    static inline struct device *to_da903x_dev(struct regulator_dev *rdev)
    {
    return rdev_get_dev(rdev).parent.parent;
    }
    static inline int check_range(struct da903x_regulator_info *info,
    int min_uV, int max_uV)
    {
    if (min_uV < info.desc.min_uV || min_uV > info.max_uV)
    return -EINVAL;
    return 0;
    }
// DA9030/DA9034 common operations
#[no_mangle]
unsafe extern "C" fn da903x_set_voltage_sel(rdev: *mut regulator_dev, selector: unsigned) -> c_int {
    static int da903x_set_voltage_sel(struct regulator_dev *rdev, unsigned selector)
    {
    struct da903x_regulator_info *info = rdev_get_drvdata(rdev);
    struct device *da9034_dev = to_da903x_dev(rdev);
    uint8_t val, mask;
    if (rdev.desc.n_voltages == 1)
    return -EINVAL;
    val = selector << info.vol_shift;
    mask = ((1 << info.vol_nbits) - 1)  << info.vol_shift;
    return da903x_update(da9034_dev, info.vol_reg, val, mask);
    }
#[no_mangle]
unsafe extern "C" fn da903x_get_voltage_sel(rdev: *mut regulator_dev) -> c_int {
    static int da903x_get_voltage_sel(struct regulator_dev *rdev)
    {
    struct da903x_regulator_info *info = rdev_get_drvdata(rdev);
    struct device *da9034_dev = to_da903x_dev(rdev);
    uint8_t val, mask;
    int ret;
    if (rdev.desc.n_voltages == 1)
    return 0;
    ret = da903x_read(da9034_dev, info.vol_reg, &val);
    if (ret)
    return ret;
    mask = ((1 << info.vol_nbits) - 1)  << info.vol_shift;
    val = (val & mask) >> info.vol_shift;
    return val;
    }
#[no_mangle]
unsafe extern "C" fn da903x_enable(rdev: *mut regulator_dev) -> c_int {
    static int da903x_enable(struct regulator_dev *rdev)
    {
    struct da903x_regulator_info *info = rdev_get_drvdata(rdev);
    struct device *da9034_dev = to_da903x_dev(rdev);
    return da903x_set_bits(da9034_dev, info.enable_reg,
    1 << info.enable_bit);
    }
#[no_mangle]
unsafe extern "C" fn da903x_disable(rdev: *mut regulator_dev) -> c_int {
    static int da903x_disable(struct regulator_dev *rdev)
    {
    struct da903x_regulator_info *info = rdev_get_drvdata(rdev);
    struct device *da9034_dev = to_da903x_dev(rdev);
    return da903x_clr_bits(da9034_dev, info.enable_reg,
    1 << info.enable_bit);
    }
#[no_mangle]
unsafe extern "C" fn da903x_is_enabled(rdev: *mut regulator_dev) -> c_int {
    static int da903x_is_enabled(struct regulator_dev *rdev)
    {
    struct da903x_regulator_info *info = rdev_get_drvdata(rdev);
    struct device *da9034_dev = to_da903x_dev(rdev);
    uint8_t reg_val;
    int ret;
    ret = da903x_read(da9034_dev, info.enable_reg, &reg_val);
    if (ret)
    return ret;
    return !!(reg_val & (1 << info.enable_bit));
    }
// DA9030 specific operations
    static int da9030_set_ldo1_15_voltage_sel(struct regulator_dev *rdev,
    unsigned selector)
    {
    struct da903x_regulator_info *info = rdev_get_drvdata(rdev);
    struct device *da903x_dev = to_da903x_dev(rdev);
    uint8_t val, mask;
    int ret;
    val = selector << info.vol_shift;
    mask = ((1 << info.vol_nbits) - 1)  << info.vol_shift;
    val |= DA9030_LDO_UNLOCK; /* have to set UNLOCK bits */
    mask |= DA9030_LDO_UNLOCK_MASK;
// write twice
    ret = da903x_update(da903x_dev, info.vol_reg, val, mask);
    if (ret)
    return ret;
    return da903x_update(da903x_dev, info.vol_reg, val, mask);
    }
    static int da9030_map_ldo14_voltage(struct regulator_dev *rdev,
    int min_uV, int max_uV)
    {
    struct da903x_regulator_info *info = rdev_get_drvdata(rdev);
    int thresh, sel;
    if (check_range(info, min_uV, max_uV)) {
    pr_err("invalid voltage range (%d, %d) uV\n", min_uV, max_uV);
    return -EINVAL;
    }
    thresh = (info.max_uV + info.desc.min_uV) / 2;
    if (min_uV < thresh) {
    sel = DIV_ROUND_UP(thresh - min_uV, info.desc.uV_step);
    sel |= 0x4;
    } else {
    sel = DIV_ROUND_UP(min_uV - thresh, info.desc.uV_step);
    }
    return sel;
    }
    static int da9030_list_ldo14_voltage(struct regulator_dev *rdev,
    unsigned selector)
    {
    struct da903x_regulator_info *info = rdev_get_drvdata(rdev);
    int volt;
    if (selector & 0x4)
    volt = rdev.desc.min_uV +
    rdev.desc.uV_step * (3 - (selector & ~0x4));
    else
    volt = (info.max_uV + rdev.desc.min_uV) / 2 +
    rdev.desc.uV_step * (selector & ~0x4);
    if (volt > info.max_uV)
    return -EINVAL;
    return volt;
    }
// DA9034 specific operations
    static int da9034_set_dvc_voltage_sel(struct regulator_dev *rdev,
    unsigned selector)
    {
    struct da903x_regulator_info *info = rdev_get_drvdata(rdev);
    struct device *da9034_dev = to_da903x_dev(rdev);
    uint8_t val, mask;
    int ret;
    val = selector << info.vol_shift;
    mask = ((1 << info.vol_nbits) - 1)  << info.vol_shift;
    ret = da903x_update(da9034_dev, info.vol_reg, val, mask);
    if (ret)
    return ret;
    ret = da903x_set_bits(da9034_dev, info.update_reg,
    1 << info.update_bit);
    return ret;
    }
    static const struct linear_range da9034_ldo12_ranges[] = {
    REGULATOR_LINEAR_RANGE(1700000, 0, 7, 50000),
    REGULATOR_LINEAR_RANGE(2700000, 8, 15, 50000),
    };
    static const struct regulator_ops da903x_regulator_ldo_ops = {
    .set_voltage_sel = da903x_set_voltage_sel,
    .get_voltage_sel = da903x_get_voltage_sel,
    .list_voltage	= regulator_list_voltage_linear,
    .map_voltage	= regulator_map_voltage_linear,
    .enable		= da903x_enable,
    .disable	= da903x_disable,
    .is_enabled	= da903x_is_enabled,
    };
// NOTE: this is dedicated for the insane DA9030 LDO14
    static const struct regulator_ops da9030_regulator_ldo14_ops = {
    .set_voltage_sel = da903x_set_voltage_sel,
    .get_voltage_sel = da903x_get_voltage_sel,
    .list_voltage	= da9030_list_ldo14_voltage,
    .map_voltage	= da9030_map_ldo14_voltage,
    .enable		= da903x_enable,
    .disable	= da903x_disable,
    .is_enabled	= da903x_is_enabled,
    };
// NOTE: this is dedicated for the DA9030 LDO1 and LDO15 that have locks
    static const struct regulator_ops da9030_regulator_ldo1_15_ops = {
    .set_voltage_sel = da9030_set_ldo1_15_voltage_sel,
    .get_voltage_sel = da903x_get_voltage_sel,
    .list_voltage	= regulator_list_voltage_linear,
    .map_voltage	= regulator_map_voltage_linear,
    .enable		= da903x_enable,
    .disable	= da903x_disable,
    .is_enabled	= da903x_is_enabled,
    };
    static const struct regulator_ops da9034_regulator_dvc_ops = {
    .set_voltage_sel = da9034_set_dvc_voltage_sel,
    .get_voltage_sel = da903x_get_voltage_sel,
    .list_voltage	= regulator_list_voltage_linear,
    .map_voltage	= regulator_map_voltage_linear,
    .enable		= da903x_enable,
    .disable	= da903x_disable,
    .is_enabled	= da903x_is_enabled,
    };
// NOTE: this is dedicated for the insane LDO12
    static const struct regulator_ops da9034_regulator_ldo12_ops = {
    .set_voltage_sel = da903x_set_voltage_sel,
    .get_voltage_sel = da903x_get_voltage_sel,
    .list_voltage	= regulator_list_voltage_linear_range,
    .map_voltage	= regulator_map_voltage_linear_range,
    .enable		= da903x_enable,
    .disable	= da903x_disable,
    .is_enabled	= da903x_is_enabled,
    };

    {									\
    .desc	= {							\
    .name	= "LDO" #_id,					\
    .ops	= &da903x_regulator_ldo_ops,			\
    .type	= REGULATOR_VOLTAGE,				\
    .id	= _pmic##_ID_LDO##_id,				\
    .n_voltages = (step) ? ((max - min) / step + 1) : 1,	\
    .owner	= THIS_MODULE,					\
    .min_uV	 = (min) * 1000,				\
    .uV_step = (step) * 1000,				\
    },								\
    .max_uV		= (max) * 1000,					\
    .vol_reg	= _pmic##_##vreg,				\
    .vol_shift	= (shift),					\
    .vol_nbits	= (nbits),					\
    .enable_reg	= _pmic##_##ereg,				\
    .enable_bit	= (ebit),					\
    }

    {									\
    .desc	= {							\
    .name	= #_id,						\
    .ops	= &da9034_regulator_dvc_ops,			\
    .type	= REGULATOR_VOLTAGE,				\
    .id	= _pmic##_ID_##_id,				\
    .n_voltages = (step) ? ((max - min) / step + 1) : 1,	\
    .owner	= THIS_MODULE,					\
    .min_uV = (min) * 1000,					\
    .uV_step = (step) * 1000,				\
    },								\
    .max_uV		= (max) * 1000,					\
    .vol_reg	= _pmic##_##vreg,				\
    .vol_shift	= (0),						\
    .vol_nbits	= (nbits),					\
    .update_reg	= _pmic##_##ureg,				\
    .update_bit	= (ubit),					\
    .enable_reg	= _pmic##_##ereg,				\
    .enable_bit	= (ebit),					\
    }

    DA903x_LDO(DA9034, _id, min, max, step, vreg, shift, nbits, ereg, ebit)

    DA903x_LDO(DA9030, _id, min, max, step, vreg, shift, nbits, ereg, ebit)

    DA903x_DVC(DA9030, _id, min, max, step, vreg, nbits, ureg, ubit, \
    ereg, ebit)

    DA903x_DVC(DA9034, _id, min, max, step, vreg, nbits, ureg, ubit, \
    ereg, ebit)

    DA903x_DVC(DA9035, _id, min, max, step, vreg, nbits, ureg, ubit, \
    ereg, ebit)
    static struct da903x_regulator_info da903x_regulator_info[] = {
// DA9030
    DA9030_DVC(BUCK2, 850, 1625, 25, BUCK2DVM1, 5, BUCK2DVM1, 7, RCTL11, 0),
    DA9030_LDO( 1, 1200, 3200, 100,    LDO1, 0, 5, RCTL12, 1),
    DA9030_LDO( 2, 1800, 3200, 100,   LDO23, 0, 4, RCTL12, 2),
    DA9030_LDO( 3, 1800, 3200, 100,   LDO23, 4, 4, RCTL12, 3),
    DA9030_LDO( 4, 1800, 3200, 100,   LDO45, 0, 4, RCTL12, 4),
    DA9030_LDO( 5, 1800, 3200, 100,   LDO45, 4, 4, RCTL12, 5),
    DA9030_LDO( 6, 1800, 3200, 100,    LDO6, 0, 4, RCTL12, 6),
    DA9030_LDO( 7, 1800, 3200, 100,   LDO78, 0, 4, RCTL12, 7),
    DA9030_LDO( 8, 1800, 3200, 100,   LDO78, 4, 4, RCTL22, 0),
    DA9030_LDO( 9, 1800, 3200, 100,  LDO912, 0, 4, RCTL22, 1),
    DA9030_LDO(10, 1800, 3200, 100, LDO1011, 0, 4, RCTL22, 2),
    DA9030_LDO(11, 1800, 3200, 100, LDO1011, 4, 4, RCTL22, 3),
    DA9030_LDO(12, 1800, 3200, 100,  LDO912, 4, 4, RCTL22, 4),
    DA9030_LDO(14, 2760, 2940,  30, LDO1416, 0, 3, RCTL11, 4),
    DA9030_LDO(15, 1100, 2650,  50,	  LDO15, 0, 5, RCTL11, 5),
    DA9030_LDO(16, 1100, 2650,  50, LDO1416, 3, 5, RCTL11, 6),
    DA9030_LDO(17, 1800, 3200, 100,   LDO17, 0, 4, RCTL11, 7),
    DA9030_LDO(18, 1800, 3200, 100, LDO1819, 0, 4, RCTL21, 2),
    DA9030_LDO(19, 1800, 3200, 100, LDO1819, 4, 4, RCTL21, 1),
    DA9030_LDO(13, 2100, 2100, 0, INVAL, 0, 0, RCTL11, 3), /* fixed @2.1V */
// DA9034
    DA9034_DVC(BUCK1, 725, 1500, 25, ADTV2, 5, VCC1, 0, OVER1, 0),
    DA9034_DVC(BUCK2, 725, 1500, 25, CDTV2, 5, VCC1, 2, OVER1, 1),
    DA9034_DVC(LDO2,  725, 1500, 25, SDTV2, 5, VCC1, 4, OVER1, 2),
    DA9034_DVC(LDO1, 1700, 2075, 25, MDTV1, 4, VCC1, 6, OVER3, 4),
    DA9034_LDO( 3, 1800, 3300, 100,  LDO643, 0, 4, OVER3, 5),
    DA9034_LDO( 4, 1800, 2900,1100,  LDO643, 4, 1, OVER3, 6),
    DA9034_LDO( 6, 2500, 2850,  50,  LDO643, 5, 3, OVER2, 0),
    DA9034_LDO( 7, 2700, 3050,  50,  LDO987, 0, 3, OVER2, 1),
    DA9034_LDO( 8, 2700, 2850,  50,  LDO987, 3, 2, OVER2, 2),
    DA9034_LDO( 9, 2700, 3050,  50,  LDO987, 5, 3, OVER2, 3),
    DA9034_LDO(10, 2700, 3050,  50, LDO1110, 0, 3, OVER2, 4),
    DA9034_LDO(11, 1800, 3300, 100, LDO1110, 4, 4, OVER2, 5),
    DA9034_LDO(12, 1700, 3050,  50, LDO1312, 0, 4, OVER3, 6),
    DA9034_LDO(13, 1800, 3300, 100, LDO1312, 4, 4, OVER2, 7),
    DA9034_LDO(14, 1800, 3300, 100, LDO1514, 0, 4, OVER3, 0),
    DA9034_LDO(15, 1800, 3300, 100, LDO1514, 4, 4, OVER3, 1),
    DA9034_LDO(5, 3100, 3100, 0, INVAL, 0, 0, OVER3, 7), /* fixed @3.1V */
// DA9035
    DA9035_DVC(BUCK3, 1800, 2200, 100, 3DTV1, 3, VCC2, 0, OVER3, 3),
    };
    static inline struct da903x_regulator_info *find_regulator_info(int id)
    {
    struct da903x_regulator_info *ri;
    int i;
    for (i = 0; i < ARRAY_SIZE(da903x_regulator_info); i++) {
    ri = &da903x_regulator_info[i];
    if (ri.desc.id == id)
    return ri;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn da903x_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int da903x_regulator_probe(struct platform_device *pdev)
    {
    struct da903x_regulator_info *ri = core::ptr::null_mut();
    struct regulator_dev *rdev;
    let mut config: regulator_config = { };
    ri = find_regulator_info(pdev.id);
    if (ri == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "invalid regulator ID specified\n");
    return -EINVAL;
    }
// Workaround for the weird LDO12 voltage setting
    if (ri.desc.id == DA9034_ID_LDO12) {
    ri.desc.ops = &da9034_regulator_ldo12_ops;
    ri.desc.n_voltages = 16;
    ri.desc.linear_ranges = da9034_ldo12_ranges;
    ri.desc.n_linear_ranges = ARRAY_SIZE(da9034_ldo12_ranges);
    }
    if (ri.desc.id == DA9030_ID_LDO14)
    ri.desc.ops = &da9030_regulator_ldo14_ops;
    if (ri.desc.id == DA9030_ID_LDO1 || ri.desc.id == DA9030_ID_LDO15)
    ri.desc.ops = &da9030_regulator_ldo1_15_ops;
    config.dev = &pdev.dev;
    config.init_data = dev_get_platdata(&pdev.dev);
    config.driver_data = ri;
    rdev = devm_regulator_register(&pdev.dev, &ri.desc, &config);
    if (IS_ERR(rdev)) {
    dev_err(&pdev.dev, "failed to register regulator %s\n",
    ri.desc.name);
    return PTR_ERR(rdev);
    }
    platform_set_drvdata(pdev, rdev);
    return 0;
    }
    static struct platform_driver da903x_regulator_driver = {
    .driver	= {
    .name	= "da903x-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe		= da903x_regulator_probe,
    };
#[no_mangle]
unsafe extern "C" fn da903x_regulator_init() -> int __init {
    static int __init da903x_regulator_init(void)
    {
    return platform_driver_register(&da903x_regulator_driver);
    }
    subsys_initcall(da903x_regulator_init);
#[no_mangle]
unsafe extern "C" fn da903x_regulator_exit() -> void __exit {
    static void __exit da903x_regulator_exit(void)
    {
    platform_driver_unregister(&da903x_regulator_driver);
    }
    module_exit(da903x_regulator_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Eric Miao <eric.miao@marvell.com>"
    "Mike Rapoport <mike@compulab.co.il>");
    MODULE_DESCRIPTION("Regulator Driver for Dialog Semiconductor DA903X PMIC");
    MODULE_ALIAS("platform:da903x-regulator");
