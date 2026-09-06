//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/intel_soc_pmic_crc.c
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
// Device access for Crystal Cove PMIC
//
// Copyright (C) 2012-2014, 2022 Intel Corporation. All rights reserved.
//
// Author: Yang, Bin <bin.yang@intel.com>
// Author: Zhu, Lejun <lejun.zhu@linux.intel.com>
//

pub const CRYSTAL_COVE_MAX_REGISTER: c_uint = 0xC6;
pub const CRYSTAL_COVE_REG_IRQLVL1: c_uint = 0x02;
pub const CRYSTAL_COVE_REG_MIRQLVL1: c_uint = 0x0E;
pub const CRYSTAL_COVE_IRQ_PWRSRC: c_int = 0;
pub const CRYSTAL_COVE_IRQ_THRM: c_int = 1;
pub const CRYSTAL_COVE_IRQ_BCU: c_int = 2;
pub const CRYSTAL_COVE_IRQ_ADC: c_int = 3;
pub const CRYSTAL_COVE_IRQ_CHGR: c_int = 4;
pub const CRYSTAL_COVE_IRQ_GPIO: c_int = 5;
pub const CRYSTAL_COVE_IRQ_VHDMIOCP: c_int = 6;
    static const struct resource pwrsrc_resources[] = {
    DEFINE_RES_IRQ_NAMED(CRYSTAL_COVE_IRQ_PWRSRC, "PWRSRC"),
    };
    static const struct resource thermal_resources[] = {
    DEFINE_RES_IRQ_NAMED(CRYSTAL_COVE_IRQ_THRM, "THERMAL"),
    };
    static const struct resource bcu_resources[] = {
    DEFINE_RES_IRQ_NAMED(CRYSTAL_COVE_IRQ_BCU, "BCU"),
    };
    static const struct resource adc_resources[] = {
    DEFINE_RES_IRQ_NAMED(CRYSTAL_COVE_IRQ_ADC, "ADC"),
    };
    static const struct resource charger_resources[] = {
    DEFINE_RES_IRQ_NAMED(CRYSTAL_COVE_IRQ_CHGR, "CHGR"),
    };
    static const struct resource gpio_resources[] = {
    DEFINE_RES_IRQ_NAMED(CRYSTAL_COVE_IRQ_GPIO, "GPIO"),
    };
    static struct mfd_cell crystal_cove_byt_dev[] = {
    {
    .name = "crystal_cove_pwrsrc",
    .num_resources = ARRAY_SIZE(pwrsrc_resources),
    .resources = pwrsrc_resources,
    },
    {
    .name = "crystal_cove_thermal",
    .num_resources = ARRAY_SIZE(thermal_resources),
    .resources = thermal_resources,
    },
    {
    .name = "crystal_cove_bcu",
    .num_resources = ARRAY_SIZE(bcu_resources),
    .resources = bcu_resources,
    },
    {
    .name = "crystal_cove_adc",
    .num_resources = ARRAY_SIZE(adc_resources),
    .resources = adc_resources,
    },
    {
    .name = "crystal_cove_charger",
    .num_resources = ARRAY_SIZE(charger_resources),
    .resources = charger_resources,
    },
    {
    .name = "crystal_cove_gpio",
    .num_resources = ARRAY_SIZE(gpio_resources),
    .resources = gpio_resources,
    },
    {
    .name = "byt_crystal_cove_pmic",
    },
    {
    .name = "crystal_cove_pwm",
    },
    };
    static struct mfd_cell crystal_cove_cht_dev[] = {
    {
    .name = "crystal_cove_gpio",
    .num_resources = ARRAY_SIZE(gpio_resources),
    .resources = gpio_resources,
    },
    {
    .name = "cht_crystal_cove_pmic",
    },
    {
    .name = "crystal_cove_pwm",
    },
    };
    static const struct regmap_config crystal_cove_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = CRYSTAL_COVE_MAX_REGISTER,
    };
    static const struct regmap_irq crystal_cove_irqs[] = {
    REGMAP_IRQ_REG(CRYSTAL_COVE_IRQ_PWRSRC, 0, BIT(CRYSTAL_COVE_IRQ_PWRSRC)),
    REGMAP_IRQ_REG(CRYSTAL_COVE_IRQ_THRM, 0, BIT(CRYSTAL_COVE_IRQ_THRM)),
    REGMAP_IRQ_REG(CRYSTAL_COVE_IRQ_BCU, 0, BIT(CRYSTAL_COVE_IRQ_BCU)),
    REGMAP_IRQ_REG(CRYSTAL_COVE_IRQ_ADC, 0, BIT(CRYSTAL_COVE_IRQ_ADC)),
    REGMAP_IRQ_REG(CRYSTAL_COVE_IRQ_CHGR, 0, BIT(CRYSTAL_COVE_IRQ_CHGR)),
    REGMAP_IRQ_REG(CRYSTAL_COVE_IRQ_GPIO, 0, BIT(CRYSTAL_COVE_IRQ_GPIO)),
    REGMAP_IRQ_REG(CRYSTAL_COVE_IRQ_VHDMIOCP, 0, BIT(CRYSTAL_COVE_IRQ_VHDMIOCP)),
    };
    static const struct regmap_irq_chip crystal_cove_irq_chip = {
    .name = "Crystal Cove",
    .irqs = crystal_cove_irqs,
    .num_irqs = ARRAY_SIZE(crystal_cove_irqs),
    .num_regs = 1,
    .status_base = CRYSTAL_COVE_REG_IRQLVL1,
    .mask_base = CRYSTAL_COVE_REG_MIRQLVL1,
    };
// PWM consumed by the Intel GFX
    static struct pwm_lookup crc_pwm_lookup[] = {
    PWM_LOOKUP_WITH_MODULE("crystal_cove_pwm", 0, "0000:00:02.0",
    "pwm_pmic_backlight", 0, PWM_POLARITY_NORMAL,
    "pwm-crc"),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crystal_cove_config {
    pub irq_flags: c_ulong,
    pub cell_dev: *mut mfd_cell,
    pub n_cell_devs: c_int,
    pub regmap_config: *const regmap_config,
    pub irq_chip: *const regmap_irq_chip,
}

    static const struct crystal_cove_config crystal_cove_config_byt_crc = {
    .irq_flags = IRQF_TRIGGER_RISING,
    .cell_dev = crystal_cove_byt_dev,
    .n_cell_devs = ARRAY_SIZE(crystal_cove_byt_dev),
    .regmap_config = &crystal_cove_regmap_config,
    .irq_chip = &crystal_cove_irq_chip,
    };
    static const struct crystal_cove_config crystal_cove_config_cht_crc = {
    .irq_flags = IRQF_TRIGGER_RISING,
    .cell_dev = crystal_cove_cht_dev,
    .n_cell_devs = ARRAY_SIZE(crystal_cove_cht_dev),
    .regmap_config = &crystal_cove_regmap_config,
    .irq_chip = &crystal_cove_irq_chip,
    };
#[no_mangle]
unsafe extern "C" fn crystal_cove_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int crystal_cove_i2c_probe(struct i2c_client *i2c)
    {
    const struct crystal_cove_config *config;
    struct device *dev = &i2c.dev;
    struct intel_soc_pmic *pmic;
    int ret;
    if (soc_intel_is_byt())
    config = &crystal_cove_config_byt_crc;
    else
    config = &crystal_cove_config_cht_crc;
    pmic = devm_kzalloc(dev, sizeof(*pmic), GFP_KERNEL);
    if (!pmic)
    return -ENOMEM;
    i2c_set_clientdata(i2c, pmic);
    pmic.regmap = devm_regmap_init_i2c(i2c, config.regmap_config);
    if (IS_ERR(pmic.regmap))
    return PTR_ERR(pmic.regmap);
    pmic.irq = i2c.irq;
    ret = devm_regmap_add_irq_chip(dev, pmic.regmap, pmic.irq,
    config.irq_flags | IRQF_ONESHOT,
    0, config.irq_chip, &pmic.irq_chip_data);
    if (ret)
    return ret;
    ret = enable_irq_wake(pmic.irq);
    if (ret)
    dev_warn(dev, "Can't enable IRQ as wake source: %d\n", ret);
// Add lookup table for crc-pwm
    pwm_add_table(crc_pwm_lookup, ARRAY_SIZE(crc_pwm_lookup));
// To distuingish this domain from the GPIO/charger's irqchip domains
    irq_domain_update_bus_token(regmap_irq_get_domain(pmic.irq_chip_data),
    DOMAIN_BUS_NEXUS);
    ret = mfd_add_devices(dev, PLATFORM_DEVID_NONE, config.cell_dev,
    config.n_cell_devs, core::ptr::null_mut(), 0,
    regmap_irq_get_domain(pmic.irq_chip_data));
    if (ret)
    pwm_remove_table(crc_pwm_lookup, ARRAY_SIZE(crc_pwm_lookup));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn crystal_cove_i2c_remove(i2c: *mut i2c_client) {
    static void crystal_cove_i2c_remove(struct i2c_client *i2c)
    {
// remove crc-pwm lookup table
    pwm_remove_table(crc_pwm_lookup, ARRAY_SIZE(crc_pwm_lookup));
    mfd_remove_devices(&i2c.dev);
    }
#[no_mangle]
unsafe extern "C" fn crystal_cove_shutdown(i2c: *mut i2c_client) {
    static void crystal_cove_shutdown(struct i2c_client *i2c)
    {
    struct intel_soc_pmic *pmic = i2c_get_clientdata(i2c);
    disable_irq(pmic.irq);
    return;
    }
#[no_mangle]
unsafe extern "C" fn crystal_cove_suspend(dev: *mut device) -> c_int {
    static int crystal_cove_suspend(struct device *dev)
    {
    struct intel_soc_pmic *pmic = dev_get_drvdata(dev);
    disable_irq(pmic.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn crystal_cove_resume(dev: *mut device) -> c_int {
    static int crystal_cove_resume(struct device *dev)
    {
    struct intel_soc_pmic *pmic = dev_get_drvdata(dev);
    enable_irq(pmic.irq);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(crystal_cove_pm_ops, crystal_cove_suspend, crystal_cove_resume);
    static const struct acpi_device_id crystal_cove_acpi_match[] = {
    { "INT33FD" },
    { },
    };
    MODULE_DEVICE_TABLE(acpi, crystal_cove_acpi_match);
    static const struct i2c_device_id crystal_cove_i2c_match[] = {
    { "intel_soc_pmic_crc" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, crystal_cove_i2c_match);
    static struct i2c_driver crystal_cove_i2c_driver = {
    .driver = {
    .name = "intel_soc_pmic_crc",
    .pm = pm_sleep_ptr(&crystal_cove_pm_ops),
    .acpi_match_table = crystal_cove_acpi_match,
    },
    .id_table = crystal_cove_i2c_match,
    .probe = crystal_cove_i2c_probe,
    .remove = crystal_cove_i2c_remove,
    .shutdown = crystal_cove_shutdown,
    };
    module_i2c_driver(crystal_cove_i2c_driver);
    MODULE_DESCRIPTION("I2C driver for Intel SoC PMIC");
    MODULE_AUTHOR("Yang, Bin <bin.yang@intel.com>");
    MODULE_AUTHOR("Zhu, Lejun <lejun.zhu@linux.intel.com>");
