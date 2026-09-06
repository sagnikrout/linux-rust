//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/sc2731-regulator.c
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
// Copyright (C) 2017 Spreadtrum Communications Inc.
//

//
// SC2731 regulator lock register
//
pub const SC2731_PWR_WR_PROT: c_uint = 0xf0c;
pub const SC2731_WR_UNLOCK_VALUE: c_uint = 0x6e7f;
//
// SC2731 enable register
//
pub const SC2731_POWER_PD_SW: c_uint = 0xc28;
pub const SC2731_LDO_CAMA0_PD: c_uint = 0xcfc;
pub const SC2731_LDO_CAMA1_PD: c_uint = 0xd04;
pub const SC2731_LDO_CAMMOT_PD: c_uint = 0xd0c;
pub const SC2731_LDO_VLDO_PD: c_uint = 0xd6c;
pub const SC2731_LDO_EMMCCORE_PD: c_uint = 0xd2c;
pub const SC2731_LDO_SDCORE_PD: c_uint = 0xd74;
pub const SC2731_LDO_SDIO_PD: c_uint = 0xd70;
pub const SC2731_LDO_WIFIPA_PD: c_uint = 0xd4c;
pub const SC2731_LDO_USB33_PD: c_uint = 0xd5c;
pub const SC2731_LDO_CAMD0_PD: c_uint = 0xd7c;
pub const SC2731_LDO_CAMD1_PD: c_uint = 0xd84;
pub const SC2731_LDO_CON_PD: c_uint = 0xd8c;
pub const SC2731_LDO_CAMIO_PD: c_uint = 0xd94;
pub const SC2731_LDO_SRAM_PD: c_uint = 0xd78;
//
// SC2731 enable mask
//

//
// SC2731 vsel register
//
pub const SC2731_DCDC_CPU0_VOL: c_uint = 0xc54;
pub const SC2731_DCDC_CPU1_VOL: c_uint = 0xc64;
pub const SC2731_DCDC_RF_VOL: c_uint = 0xcb8;
pub const SC2731_LDO_CAMA0_VOL: c_uint = 0xd00;
pub const SC2731_LDO_CAMA1_VOL: c_uint = 0xd08;
pub const SC2731_LDO_CAMMOT_VOL: c_uint = 0xd10;
pub const SC2731_LDO_VLDO_VOL: c_uint = 0xd28;
pub const SC2731_LDO_EMMCCORE_VOL: c_uint = 0xd30;
pub const SC2731_LDO_SDCORE_VOL: c_uint = 0xd38;
pub const SC2731_LDO_SDIO_VOL: c_uint = 0xd40;
pub const SC2731_LDO_WIFIPA_VOL: c_uint = 0xd50;
pub const SC2731_LDO_USB33_VOL: c_uint = 0xd60;
pub const SC2731_LDO_CAMD0_VOL: c_uint = 0xd80;
pub const SC2731_LDO_CAMD1_VOL: c_uint = 0xd88;
pub const SC2731_LDO_CON_VOL: c_uint = 0xd90;
pub const SC2731_LDO_CAMIO_VOL: c_uint = 0xd98;
pub const SC2731_LDO_SRAM_VOL: c_uint = 0xdB0;
//
// SC2731 vsel register mask
//

    enum sc2731_regulator_id {
    SC2731_BUCK_CPU0,
    SC2731_BUCK_CPU1,
    SC2731_BUCK_RF,
    SC2731_LDO_CAMA0,
    SC2731_LDO_CAMA1,
    SC2731_LDO_CAMMOT,
    SC2731_LDO_VLDO,
    SC2731_LDO_EMMCCORE,
    SC2731_LDO_SDCORE,
    SC2731_LDO_SDIO,
    SC2731_LDO_WIFIPA,
    SC2731_LDO_USB33,
    SC2731_LDO_CAMD0,
    SC2731_LDO_CAMD1,
    SC2731_LDO_CON,
    SC2731_LDO_CAMIO,
    SC2731_LDO_SRAM,
    };
    static const struct regulator_ops sc2731_regu_linear_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .list_voltage = regulator_list_voltage_linear,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    };

    vstep, vmin, vmax) {			\
    .name			= #_id,				\
    .of_match		= of_match_ptr(#_id),		\
    .ops			= &sc2731_regu_linear_ops,	\
    .type			= REGULATOR_VOLTAGE,		\
    .id			= SC2731_##_id,			\
    .owner			= THIS_MODULE,			\
    .min_uV			= vmin,				\
    .n_voltages		= ((vmax) - (vmin)) / (vstep) + 1,	\
    .uV_step		= vstep,			\
    .enable_is_inverted	= true,				\
    .enable_val		= 0,				\
    .enable_reg		= en_reg,			\
    .enable_mask		= en_mask,			\
    .vsel_reg		= vreg,				\
    .vsel_mask		= vmask,			\
    }
    static const struct regulator_desc regulators[] = {
    SC2731_REGU_LINEAR(BUCK_CPU0, SC2731_POWER_PD_SW,
    SC2731_DCDC_CPU0_PD_MASK, SC2731_DCDC_CPU0_VOL,
    SC2731_DCDC_CPU0_VOL_MASK, 3125, 400000, 1996875),
    SC2731_REGU_LINEAR(BUCK_CPU1, SC2731_POWER_PD_SW,
    SC2731_DCDC_CPU1_PD_MASK, SC2731_DCDC_CPU1_VOL,
    SC2731_DCDC_CPU1_VOL_MASK, 3125, 400000, 1996875),
    SC2731_REGU_LINEAR(BUCK_RF, SC2731_POWER_PD_SW, SC2731_DCDC_RF_PD_MASK,
    SC2731_DCDC_RF_VOL, SC2731_DCDC_RF_VOL_MASK,
    3125, 600000, 2196875),
    SC2731_REGU_LINEAR(LDO_CAMA0, SC2731_LDO_CAMA0_PD,
    SC2731_LDO_CAMA0_PD_MASK, SC2731_LDO_CAMA0_VOL,
    SC2731_LDO_CAMA0_VOL_MASK, 10000, 1200000, 3750000),
    SC2731_REGU_LINEAR(LDO_CAMA1, SC2731_LDO_CAMA1_PD,
    SC2731_LDO_CAMA1_PD_MASK, SC2731_LDO_CAMA1_VOL,
    SC2731_LDO_CAMA1_VOL_MASK, 10000, 1200000, 3750000),
    SC2731_REGU_LINEAR(LDO_CAMMOT, SC2731_LDO_CAMMOT_PD,
    SC2731_LDO_CAMMOT_PD_MASK, SC2731_LDO_CAMMOT_VOL,
    SC2731_LDO_CAMMOT_VOL_MASK, 10000, 1200000, 3750000),
    SC2731_REGU_LINEAR(LDO_VLDO, SC2731_LDO_VLDO_PD,
    SC2731_LDO_VLDO_PD_MASK, SC2731_LDO_VLDO_VOL,
    SC2731_LDO_VLDO_VOL_MASK, 10000, 1200000, 3750000),
    SC2731_REGU_LINEAR(LDO_EMMCCORE, SC2731_LDO_EMMCCORE_PD,
    SC2731_LDO_EMMCCORE_PD_MASK, SC2731_LDO_EMMCCORE_VOL,
    SC2731_LDO_EMMCCORE_VOL_MASK, 10000, 1200000,
    3750000),
    SC2731_REGU_LINEAR(LDO_SDCORE, SC2731_LDO_SDCORE_PD,
    SC2731_LDO_SDCORE_PD_MASK, SC2731_LDO_SDCORE_VOL,
    SC2731_LDO_SDCORE_VOL_MASK, 10000, 1200000, 3750000),
    SC2731_REGU_LINEAR(LDO_SDIO, SC2731_LDO_SDIO_PD,
    SC2731_LDO_SDIO_PD_MASK, SC2731_LDO_SDIO_VOL,
    SC2731_LDO_SDIO_VOL_MASK, 10000, 1200000, 3750000),
    SC2731_REGU_LINEAR(LDO_WIFIPA, SC2731_LDO_WIFIPA_PD,
    SC2731_LDO_WIFIPA_PD_MASK, SC2731_LDO_WIFIPA_VOL,
    SC2731_LDO_WIFIPA_VOL_MASK, 10000, 1200000, 3750000),
    SC2731_REGU_LINEAR(LDO_USB33, SC2731_LDO_USB33_PD,
    SC2731_LDO_USB33_PD_MASK, SC2731_LDO_USB33_VOL,
    SC2731_LDO_USB33_VOL_MASK, 10000, 1200000, 3750000),
    SC2731_REGU_LINEAR(LDO_CAMD0, SC2731_LDO_CAMD0_PD,
    SC2731_LDO_CAMD0_PD_MASK, SC2731_LDO_CAMD0_VOL,
    SC2731_LDO_CAMD0_VOL_MASK, 6250, 1000000, 1793750),
    SC2731_REGU_LINEAR(LDO_CAMD1, SC2731_LDO_CAMD1_PD,
    SC2731_LDO_CAMD1_PD_MASK, SC2731_LDO_CAMD1_VOL,
    SC2731_LDO_CAMD1_VOL_MASK, 6250, 1000000, 1793750),
    SC2731_REGU_LINEAR(LDO_CON, SC2731_LDO_CON_PD,
    SC2731_LDO_CON_PD_MASK, SC2731_LDO_CON_VOL,
    SC2731_LDO_CON_VOL_MASK, 6250, 1000000, 1793750),
    SC2731_REGU_LINEAR(LDO_CAMIO, SC2731_LDO_CAMIO_PD,
    SC2731_LDO_CAMIO_PD_MASK, SC2731_LDO_CAMIO_VOL,
    SC2731_LDO_CAMIO_VOL_MASK, 6250, 1000000, 1793750),
    SC2731_REGU_LINEAR(LDO_SRAM, SC2731_LDO_SRAM_PD,
    SC2731_LDO_SRAM_PD_MASK, SC2731_LDO_SRAM_VOL,
    SC2731_LDO_SRAM_VOL_MASK, 6250, 1000000, 1793750),
    };
#[no_mangle]
unsafe extern "C" fn sc2731_regulator_unlock(regmap: *mut regmap) -> c_int {
    static int sc2731_regulator_unlock(struct regmap *regmap)
    {
    return regmap_write(regmap, SC2731_PWR_WR_PROT,
    SC2731_WR_UNLOCK_VALUE);
    }
#[no_mangle]
unsafe extern "C" fn sc2731_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int sc2731_regulator_probe(struct platform_device *pdev)
    {
    int i, ret;
    struct regmap *regmap;
    let mut config: regulator_config = { };
    struct regulator_dev *rdev;
    regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!regmap) {
    dev_err(&pdev.dev, "failed to get regmap.\n");
    return -ENODEV;
    }
    ret = sc2731_regulator_unlock(regmap);
    if (ret) {
    dev_err(&pdev.dev, "failed to release regulator lock\n");
    return ret;
    }
    config.dev = &pdev.dev;
    config.regmap = regmap;
    for (i = 0; i < ARRAY_SIZE(regulators); i++) {
    rdev = devm_regulator_register(&pdev.dev, &regulators[i],
    &config);
    if (IS_ERR(rdev)) {
    dev_err(&pdev.dev, "failed to register regulator %s\n",
    regulators[i].name);
    return PTR_ERR(rdev);
    }
    }
    return 0;
    }
    static struct platform_driver sc2731_regulator_driver = {
    .driver = {
    .name = "sc27xx-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe = sc2731_regulator_probe,
    };
    module_platform_driver(sc2731_regulator_driver);
    MODULE_AUTHOR("Chen Junhui <erick.chen@spreadtrum.com>");
    MODULE_DESCRIPTION("Spreadtrum SC2731 regulator driver");
    MODULE_LICENSE("GPL v2");
