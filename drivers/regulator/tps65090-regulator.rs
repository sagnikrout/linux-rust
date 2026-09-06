//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/tps65090-regulator.c
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
// Regulator driver for tps65090 power management chip.
//
// Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
//

pub const MAX_CTRL_READ_TRIES: c_int = 5;
pub const MAX_FET_ENABLE_TRIES: c_int = 1000;

//
// struct tps65090_regulator - Per-regulator data for a tps65090 regulator
//
// @dev: Pointer to our device.
// @desc: The struct regulator_desc for the regulator.
// @rdev: The struct regulator_dev for the regulator.
// @overcurrent_wait_valid: True if overcurrent_wait is valid.
// @overcurrent_wait: For FETs, the value to put in the WTFET bitfield.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65090_regulator {
    pub dev: *mut device,
    pub desc: *mut regulator_desc,
    pub rdev: *mut regulator_dev,
    pub overcurrent_wait_valid: bool,
    pub overcurrent_wait: c_int,
}

    static const struct regulator_ops tps65090_ext_control_ops = {
    };
//
// tps65090_reg_set_overcurrent_wait - Setup overcurrent wait
//
// This will set the overcurrent wait time based on what's in the regulator
// info.
//
// @ri:		Overall regulator data
// @rdev:	Regulator device
//
// Return: 0 if no error, non-zero if there was an error writing the register.
//
    static int tps65090_reg_set_overcurrent_wait(struct tps65090_regulator *ri,
    struct regulator_dev *rdev)
    {
    int ret;
    ret = regmap_update_bits(rdev.regmap, rdev.desc.enable_reg,
    MAX_OVERCURRENT_WAIT << CTRL_WT_BIT,
    ri.overcurrent_wait << CTRL_WT_BIT);
    if (ret) {
    dev_err(&rdev.dev, "Error updating overcurrent wait %#x\n",
    rdev.desc.enable_reg);
    }
    return ret;
    }
//
// tps65090_try_enable_fet - Try to enable a FET
//
// @rdev:	Regulator device
//
// Return: 0 if ok, -ENOTRECOVERABLE if the FET power good bit did not get
// set, or some other -ve value if another error occurred (e.g. i2c error)
//
#[no_mangle]
unsafe extern "C" fn tps65090_try_enable_fet(rdev: *mut regulator_dev) -> c_int {
    static int tps65090_try_enable_fet(struct regulator_dev *rdev)
    {
    unsigned int control;
    int ret, i;
    ret = regmap_update_bits(rdev.regmap, rdev.desc.enable_reg,
    rdev.desc.enable_mask,
    rdev.desc.enable_mask);
    if (ret < 0) {
    dev_err(&rdev.dev, "Error in updating reg %#x\n",
    rdev.desc.enable_reg);
    return ret;
    }
    for (i = 0; i < MAX_CTRL_READ_TRIES; i++) {
    ret = regmap_read(rdev.regmap, rdev.desc.enable_reg,
    &control);
    if (ret < 0)
    return ret;
    if (!(control & BIT(CTRL_TO_BIT)))
    break;
    usleep_range(1000, 1500);
    }
    if (!(control & BIT(CTRL_PG_BIT)))
    return -ENOTRECOVERABLE;
    return 0;
    }
//
// tps65090_fet_enable - Enable a FET, trying a few times if it fails
//
// Some versions of the tps65090 have issues when turning on the FETs.
// This function goes through several steps to ensure the best chance of the
// FET going on.  Specifically:
// - We'll make sure that we bump the "overcurrent wait" to the maximum, which
// increases the chances that we'll turn on properly.
// - We'll retry turning the FET on multiple times (turning off in between).
//
// @rdev:	Regulator device
//
// Return: 0 if ok, non-zero if it fails.
//
#[no_mangle]
unsafe extern "C" fn tps65090_fet_enable(rdev: *mut regulator_dev) -> c_int {
    static int tps65090_fet_enable(struct regulator_dev *rdev)
    {
    int ret, tries;
//
// Try enabling multiple times until we succeed since sometimes the
// first try times out.
//
    tries = 0;
    while (true) {
    ret = tps65090_try_enable_fet(rdev);
    if (!ret)
    break;
    if (ret != -ENOTRECOVERABLE || tries == MAX_FET_ENABLE_TRIES)
    goto err;
// Try turning the FET off (and then on again)
    ret = regmap_update_bits(rdev.regmap, rdev.desc.enable_reg,
    rdev.desc.enable_mask, 0);
    if (ret)
    goto err;
    tries++;
    }
    if (tries)
    dev_warn(&rdev.dev, "reg %#x enable ok after %d tries\n",
    rdev.desc.enable_reg, tries);
    return 0;
    err:
    dev_warn(&rdev.dev, "reg %#x enable failed\n", rdev.desc.enable_reg);
    WARN_ON(1);
    return ret;
    }
    static const struct regulator_ops tps65090_reg_control_ops = {
    .enable		= regulator_enable_regmap,
    .disable	= regulator_disable_regmap,
    .is_enabled	= regulator_is_enabled_regmap,
    };
    static const struct regulator_ops tps65090_fet_control_ops = {
    .enable		= tps65090_fet_enable,
    .disable	= regulator_disable_regmap,
    .is_enabled	= regulator_is_enabled_regmap,
    };
    static const struct regulator_ops tps65090_ldo_ops = {
    };

    {							\
    .name = "TPS65090_RAILS"#_id,			\
    .supply_name = _sname,				\
    .id = TPS65090_REGULATOR_##_id,			\
    .n_voltages = _nvolt,				\
    .ops = &_ops,					\
    .fixed_uV = _volt,				\
    .enable_reg = _en_reg,				\
    .enable_val = _en_bits,				\
    .enable_mask = _en_bits,			\
    .type = REGULATOR_VOLTAGE,			\
    .owner = THIS_MODULE,				\
    }

    tps65090_REG_DESC(_id, _sname, en_reg, _en_bits, 1, _volt, _ops)

    tps65090_REG_DESC(_id, _sname, en_reg, _en_bits, 0, 0, _ops)
    static struct regulator_desc tps65090_regulator_desc[] = {
    tps65090_REG_FIXEDV(DCDC1, "vsys1",   0x0C, BIT(CTRL_EN_BIT), 5000000,
    tps65090_reg_control_ops),
    tps65090_REG_FIXEDV(DCDC2, "vsys2",   0x0D, BIT(CTRL_EN_BIT), 3300000,
    tps65090_reg_control_ops),
    tps65090_REG_SWITCH(DCDC3, "vsys3",   0x0E, BIT(CTRL_EN_BIT),
    tps65090_reg_control_ops),
    tps65090_REG_SWITCH(FET1,  "infet1",  0x0F,
    BIT(CTRL_EN_BIT) | BIT(CTRL_PG_BIT),
    tps65090_fet_control_ops),
    tps65090_REG_SWITCH(FET2,  "infet2",  0x10,
    BIT(CTRL_EN_BIT) | BIT(CTRL_PG_BIT),
    tps65090_fet_control_ops),
    tps65090_REG_SWITCH(FET3,  "infet3",  0x11,
    BIT(CTRL_EN_BIT) | BIT(CTRL_PG_BIT),
    tps65090_fet_control_ops),
    tps65090_REG_SWITCH(FET4,  "infet4",  0x12,
    BIT(CTRL_EN_BIT) | BIT(CTRL_PG_BIT),
    tps65090_fet_control_ops),
    tps65090_REG_SWITCH(FET5,  "infet5",  0x13,
    BIT(CTRL_EN_BIT) | BIT(CTRL_PG_BIT),
    tps65090_fet_control_ops),
    tps65090_REG_SWITCH(FET6,  "infet6",  0x14,
    BIT(CTRL_EN_BIT) | BIT(CTRL_PG_BIT),
    tps65090_fet_control_ops),
    tps65090_REG_SWITCH(FET7,  "infet7",  0x15,
    BIT(CTRL_EN_BIT) | BIT(CTRL_PG_BIT),
    tps65090_fet_control_ops),
    tps65090_REG_FIXEDV(LDO1,  "vsys-l1", 0, 0, 5000000,
    tps65090_ldo_ops),
    tps65090_REG_FIXEDV(LDO2,  "vsys-l2", 0, 0, 3300000,
    tps65090_ldo_ops),
    };
#[no_mangle]
pub unsafe extern "C" fn is_dcdc(id: c_int) -> bool {
    static inline bool is_dcdc(int id)
    {
    switch (id) {
    case TPS65090_REGULATOR_DCDC1:
    case TPS65090_REGULATOR_DCDC2:
    case TPS65090_REGULATOR_DCDC3:
    return true;
    default:
    return false;
    }
    }
    static int tps65090_config_ext_control(
    struct tps65090_regulator *ri, bool enable)
    {
    int ret;
    struct device *parent = ri.dev.parent;
    let mut reg_en_reg: c_uint = ri.desc.enable_reg;
    if (enable)
    ret = tps65090_set_bits(parent, reg_en_reg, 1);
    else
    ret =  tps65090_clr_bits(parent, reg_en_reg, 1);
    if (ret < 0)
    dev_err(ri.dev, "Error in updating reg 0x%x\n", reg_en_reg);
    return ret;
    }
    static int tps65090_regulator_disable_ext_control(
    struct tps65090_regulator *ri,
    struct tps65090_regulator_plat_data *tps_pdata)
    {
    let mut ret: c_int = 0;
    struct device *parent = ri.dev.parent;
    let mut reg_en_reg: c_uint = ri.desc.enable_reg;
//
// First enable output for internal control if require.
// And then disable external control.
//
    if (tps_pdata.reg_init_data.constraints.always_on ||
    tps_pdata.reg_init_data.constraints.boot_on) {
    ret =  tps65090_set_bits(parent, reg_en_reg, 0);
    if (ret < 0) {
    dev_err(ri.dev, "Error in set reg 0x%x\n", reg_en_reg);
    return ret;
    }
    }
    return tps65090_config_ext_control(ri, false);
    }

    static struct of_regulator_match tps65090_matches[] = {
    { .name = "dcdc1", },
    { .name = "dcdc2", },
    { .name = "dcdc3", },
    { .name = "fet1",  },
    { .name = "fet2",  },
    { .name = "fet3",  },
    { .name = "fet4",  },
    { .name = "fet5",  },
    { .name = "fet6",  },
    { .name = "fet7",  },
    { .name = "ldo1",  },
    { .name = "ldo2",  },
    };
    static struct tps65090_platform_data *tps65090_parse_dt_reg_data(
    struct platform_device *pdev,
    struct of_regulator_match **tps65090_reg_matches)
    {
    struct tps65090_platform_data *tps65090_pdata;
    struct device_node *np = pdev.dev.parent.of_node;
    struct device_node *regulators;
    let mut idx: c_int = 0, ret;
    struct tps65090_regulator_plat_data *reg_pdata;
    tps65090_pdata = devm_kzalloc(&pdev.dev, sizeof(*tps65090_pdata),
    GFP_KERNEL);
    if (!tps65090_pdata)
    return ERR_PTR(-ENOMEM);
    reg_pdata = devm_kcalloc(&pdev.dev,
    TPS65090_REGULATOR_MAX, sizeof(*reg_pdata),
    GFP_KERNEL);
    if (!reg_pdata)
    return ERR_PTR(-ENOMEM);
    regulators = of_get_child_by_name(np, "regulators");
    if (!regulators) {
    dev_err(&pdev.dev, "regulator node not found\n");
    return ERR_PTR(-ENODEV);
    }
    ret = of_regulator_match(&pdev.dev, regulators, tps65090_matches,
    ARRAY_SIZE(tps65090_matches));
    of_node_put(regulators);
    if (ret < 0) {
    dev_err(&pdev.dev,
    "Error parsing regulator init data: %d\n", ret);
    return ERR_PTR(ret);
    }
// tps65090_reg_matches = tps65090_matches;
    for (idx = 0; idx < ARRAY_SIZE(tps65090_matches); idx++) {
    struct regulator_init_data *ri_data;
    struct tps65090_regulator_plat_data *rpdata;
    struct device_node *np;
    rpdata = &reg_pdata[idx];
    ri_data = tps65090_matches[idx].init_data;
    if (!ri_data)
    continue;
    np = tps65090_matches[idx].of_node;
    if (!np)
    continue;
    rpdata.reg_init_data = ri_data;
    rpdata.enable_ext_control = of_property_read_bool(np,
    "ti,enable-ext-control");
    if (rpdata.enable_ext_control) {
    enum gpiod_flags gflags;
    if (ri_data.constraints.always_on ||
    ri_data.constraints.boot_on)
    gflags = GPIOD_OUT_HIGH;
    else
    gflags = GPIOD_OUT_LOW;
    gflags |= GPIOD_FLAGS_BIT_NONEXCLUSIVE;
    rpdata.gpiod = devm_fwnode_gpiod_get(
    &pdev.dev,
    of_fwnode_handle(np),
    "dcdc-ext-control",
    gflags,
    "tps65090");
    if (PTR_ERR(rpdata.gpiod) == -ENOENT) {
    dev_err(&pdev.dev,
    "could not find DCDC external control GPIO\n");
    rpdata.gpiod = core::ptr::null_mut();
    } else if (IS_ERR(rpdata.gpiod))
    return ERR_CAST(rpdata.gpiod);
    }
    if (of_property_read_u32(np, "ti,overcurrent-wait",
    &rpdata.overcurrent_wait) == 0)
    rpdata.overcurrent_wait_valid = true;
    tps65090_pdata.reg_pdata[idx] = rpdata;
    }
    return tps65090_pdata;
    }

    static inline struct tps65090_platform_data *tps65090_parse_dt_reg_data(
    struct platform_device *pdev,
    struct of_regulator_match **tps65090_reg_matches)
    {
// tps65090_reg_matches = NULL;
    return core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn tps65090_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int tps65090_regulator_probe(struct platform_device *pdev)
    {
    struct tps65090 *tps65090_mfd = dev_get_drvdata(pdev.dev.parent);
    struct tps65090_regulator *ri = core::ptr::null_mut();
    let mut config: regulator_config = { };
    struct regulator_dev *rdev;
    struct tps65090_regulator_plat_data *tps_pdata;
    struct tps65090_regulator *pmic;
    struct tps65090_platform_data *tps65090_pdata;
    struct of_regulator_match *tps65090_reg_matches = core::ptr::null_mut();
    int num;
    int ret;
    dev_dbg(&pdev.dev, "Probing regulator\n");
    tps65090_pdata = dev_get_platdata(pdev.dev.parent);
    if (!tps65090_pdata && tps65090_mfd.dev.of_node)
    tps65090_pdata = tps65090_parse_dt_reg_data(pdev,
    &tps65090_reg_matches);
    if (IS_ERR_OR_NULL(tps65090_pdata)) {
    dev_err(&pdev.dev, "Platform data missing\n");
    return tps65090_pdata ? PTR_ERR(tps65090_pdata) : -EINVAL;
    }
    pmic = devm_kcalloc(&pdev.dev,
    TPS65090_REGULATOR_MAX, sizeof(*pmic),
    GFP_KERNEL);
    if (!pmic)
    return -ENOMEM;
    for (num = 0; num < TPS65090_REGULATOR_MAX; num++) {
    tps_pdata = tps65090_pdata.reg_pdata[num];
    ri = &pmic[num];
    ri.dev = &pdev.dev;
    ri.desc = &tps65090_regulator_desc[num];
    if (tps_pdata) {
    ri.overcurrent_wait_valid =
    tps_pdata.overcurrent_wait_valid;
    ri.overcurrent_wait = tps_pdata.overcurrent_wait;
    }
//
// TPS5090 DCDC support the control from external digital input.
// Configure it as per platform data.
//
    if (tps_pdata && is_dcdc(num) && tps_pdata.reg_init_data) {
    if (tps_pdata.enable_ext_control) {
    config.ena_gpiod = tps_pdata.gpiod;
    ri.desc.ops = &tps65090_ext_control_ops;
    } else {
    ret = tps65090_regulator_disable_ext_control(
    ri, tps_pdata);
    if (ret < 0) {
    dev_err(&pdev.dev,
    "failed disable ext control\n");
    return ret;
    }
    }
    }
    config.dev = pdev.dev.parent;
    config.driver_data = ri;
    config.regmap = tps65090_mfd.rmap;
    if (tps_pdata)
    config.init_data = tps_pdata.reg_init_data;
    else
    config.init_data = core::ptr::null_mut();
    if (tps65090_reg_matches)
    config.of_node = tps65090_reg_matches[num].of_node;
    else
    config.of_node = core::ptr::null_mut();
//
// Hand the GPIO descriptor management over to the regulator
// core, remove it from devres management.
//
    if (config.ena_gpiod)
    devm_gpiod_unhinge(&pdev.dev, config.ena_gpiod);
    rdev = devm_regulator_register(&pdev.dev, ri.desc, &config);
    if (IS_ERR(rdev)) {
    dev_err(&pdev.dev, "failed to register regulator %s\n",
    ri.desc.name);
    return PTR_ERR(rdev);
    }
    ri.rdev = rdev;
    if (ri.overcurrent_wait_valid) {
    ret = tps65090_reg_set_overcurrent_wait(ri, rdev);
    if (ret < 0)
    return ret;
    }
// Enable external control if it is require
    if (tps_pdata && is_dcdc(num) && tps_pdata.reg_init_data &&
    tps_pdata.enable_ext_control) {
    ret = tps65090_config_ext_control(ri, true);
    if (ret < 0)
    return ret;
    }
    }
    platform_set_drvdata(pdev, pmic);
    return 0;
    }
    static struct platform_driver tps65090_regulator_driver = {
    .driver	= {
    .name	= "tps65090-pmic",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe		= tps65090_regulator_probe,
    };
#[no_mangle]
unsafe extern "C" fn tps65090_regulator_init() -> int __init {
    static int __init tps65090_regulator_init(void)
    {
    return platform_driver_register(&tps65090_regulator_driver);
    }
    subsys_initcall(tps65090_regulator_init);
#[no_mangle]
unsafe extern "C" fn tps65090_regulator_exit() -> void __exit {
    static void __exit tps65090_regulator_exit(void)
    {
    platform_driver_unregister(&tps65090_regulator_driver);
    }
    module_exit(tps65090_regulator_exit);
    MODULE_DESCRIPTION("tps65090 regulator driver");
    MODULE_AUTHOR("Venu Byravarasu <vbyravarasu@nvidia.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:tps65090-pmic");
