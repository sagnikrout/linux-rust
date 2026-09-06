//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/stm32-booster.c
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
// Copyright (C) STMicroelectronics 2019
// Author(s): Fabrice Gasnier <fabrice.gasnier@st.com>.

// STM32H7 SYSCFG register
pub const STM32H7_SYSCFG_PMCR: c_uint = 0x04;

// STM32MP1 SYSCFG has set and clear registers
pub const STM32MP1_SYSCFG_PMCSETR: c_uint = 0x04;
pub const STM32MP1_SYSCFG_PMCCLRR: c_uint = 0x44;

    static const struct regulator_ops stm32h7_booster_ops = {
    .enable		= regulator_enable_regmap,
    .disable	= regulator_disable_regmap,
    .is_enabled	= regulator_is_enabled_regmap,
    };
    static const struct regulator_desc stm32h7_booster_desc = {
    .name = "booster",
    .supply_name = "vdda",
    .n_voltages = 1,
    .type = REGULATOR_VOLTAGE,
    .fixed_uV = 3300000,
    .ramp_delay = 66000, /* up to 50us to stabilize */
    .ops = &stm32h7_booster_ops,
    .enable_reg = STM32H7_SYSCFG_PMCR,
    .enable_mask = STM32H7_SYSCFG_BOOSTE_MASK,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn stm32mp1_booster_enable(rdev: *mut regulator_dev) -> c_int {
    static int stm32mp1_booster_enable(struct regulator_dev *rdev)
    {
    return regmap_write(rdev.regmap, STM32MP1_SYSCFG_PMCSETR,
    STM32MP1_SYSCFG_EN_BOOSTER_MASK);
    }
#[no_mangle]
unsafe extern "C" fn stm32mp1_booster_disable(rdev: *mut regulator_dev) -> c_int {
    static int stm32mp1_booster_disable(struct regulator_dev *rdev)
    {
    return regmap_write(rdev.regmap, STM32MP1_SYSCFG_PMCCLRR,
    STM32MP1_SYSCFG_EN_BOOSTER_MASK);
    }
    static const struct regulator_ops stm32mp1_booster_ops = {
    .enable		= stm32mp1_booster_enable,
    .disable	= stm32mp1_booster_disable,
    .is_enabled	= regulator_is_enabled_regmap,
    };
    static const struct regulator_desc stm32mp1_booster_desc = {
    .name = "booster",
    .supply_name = "vdda",
    .n_voltages = 1,
    .type = REGULATOR_VOLTAGE,
    .fixed_uV = 3300000,
    .ramp_delay = 66000,
    .ops = &stm32mp1_booster_ops,
    .enable_reg = STM32MP1_SYSCFG_PMCSETR,
    .enable_mask = STM32MP1_SYSCFG_EN_BOOSTER_MASK,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn stm32_booster_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_booster_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = pdev.dev.of_node;
    let mut config: regulator_config = { };
    const struct regulator_desc *desc;
    struct regulator_dev *rdev;
    struct regmap *regmap;
    int ret;
    regmap = syscon_regmap_lookup_by_phandle(np, "st,syscfg");
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    desc = device_get_match_data(dev);
    config.regmap = regmap;
    config.dev = dev;
    config.of_node = np;
    config.init_data = of_get_regulator_init_data(dev, np, desc);
    rdev = devm_regulator_register(dev, desc, &config);
    if (IS_ERR(rdev)) {
    ret = PTR_ERR(rdev);
    dev_err(dev, "register failed with error %d\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct of_device_id __maybe_unused stm32_booster_of_match[] = {
    {
    .compatible = "st,stm32h7-booster",
    .data = (void *)&stm32h7_booster_desc
    }, {
    .compatible = "st,stm32mp1-booster",
    .data = (void *)&stm32mp1_booster_desc
    }, {
    },
    };
    MODULE_DEVICE_TABLE(of, stm32_booster_of_match);
    static struct platform_driver stm32_booster_driver = {
    .probe = stm32_booster_probe,
    .driver = {
    .name  = "stm32-booster",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(stm32_booster_of_match),
    },
    };
    module_platform_driver(stm32_booster_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Fabrice Gasnier <fabrice.gasnier@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics STM32 booster regulator driver");
    MODULE_ALIAS("platform:stm32-booster");
