//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-qixis-fpga.c
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
// Layerscape GPIO QIXIS FPGA driver
//
// Copyright 2025 NXP
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qixis_cpld_gpio_config {
    pub output_lines: u64,
}

    static const struct qixis_cpld_gpio_config lx2160ardb_sfp_cfg = {
    .output_lines = BIT(0),
    };
    static const struct qixis_cpld_gpio_config ls1046aqds_stat_pres2_cfg = {
    .output_lines = 0x0,
    };
    static const struct regmap_config regmap_config_8r_8v = {
    .reg_bits = 8,
    .val_bits = 8,
    };
#[no_mangle]
unsafe extern "C" fn qixis_cpld_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int qixis_cpld_gpio_probe(struct platform_device *pdev)
    {
    DECLARE_BITMAP(fixed_direction_output, 8);
    const struct qixis_cpld_gpio_config *cfg;
    let mut config: gpio_regmap_config = {0};
    struct regmap *regmap;
    void __iomem *reg;
    u32 base;
    int ret;
    if (!pdev.dev.parent)
    return -ENODEV;
    cfg = device_get_match_data(&pdev.dev);
    ret = device_property_read_u32(&pdev.dev, "reg", &base);
    if (ret)
    return ret;
    regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!regmap) {
// In case there is no regmap configured by the parent device,
// create our own from the MMIO space.
//
    reg = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reg))
    return PTR_ERR(reg);
    regmap = devm_regmap_init_mmio(&pdev.dev, reg, &regmap_config_8r_8v);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
// In this case, the offset of our register is 0 inside the
// regmap area that we just created.
//
    base = 0;
    }
    config.reg_dat_base = GPIO_REGMAP_ADDR(base);
    config.reg_set_base = GPIO_REGMAP_ADDR(base);
    config.drvdata = (void *)cfg;
    config.regmap = regmap;
    config.parent = &pdev.dev;
    config.ngpio_per_reg = 8;
    config.ngpio = 8;
    bitmap_from_u64(fixed_direction_output, cfg.output_lines);
    config.fixed_direction_output = fixed_direction_output;
    return PTR_ERR_OR_ZERO(devm_gpio_regmap_register(&pdev.dev, &config));
    }
    static const struct of_device_id qixis_cpld_gpio_of_match[] = {
    {
    .compatible = "fsl,lx2160ardb-fpga-gpio-sfp",
    .data = &lx2160ardb_sfp_cfg,
    },
    {
    .compatible = "fsl,ls1046aqds-fpga-gpio-stat-pres2",
    .data = &ls1046aqds_stat_pres2_cfg,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, qixis_cpld_gpio_of_match);
    static struct platform_driver qixis_cpld_gpio_driver = {
    .probe = qixis_cpld_gpio_probe,
    .driver = {
    .name = "gpio-qixis-cpld",
    .of_match_table = qixis_cpld_gpio_of_match,
    },
    };
    module_platform_driver(qixis_cpld_gpio_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Ioana Ciornei <ioana.ciornei@nxp.com>");
    MODULE_DESCRIPTION("Layerscape GPIO QIXIS FPGA driver");
