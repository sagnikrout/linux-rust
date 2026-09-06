//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/atmel-hlcdc.c
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
// Copyright (C) 2014 Free Electrons
// Copyright (C) 2014 Atmel
//
// Author: Boris BREZILLON <boris.brezillon@free-electrons.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_hlcdc_regmap {
    pub regs: *mut void __iomem,
    pub dev: *mut device,
}

    static const struct mfd_cell atmel_hlcdc_cells[] = {
    {
    .name = "atmel-hlcdc-pwm",
    .of_compatible = "atmel,hlcdc-pwm",
    },
    {
    .name = "atmel-hlcdc-dc",
    .of_compatible = "atmel,hlcdc-display-controller",
    },
    };
    static int regmap_atmel_hlcdc_reg_write(void *context, unsigned int reg,
    unsigned int val)
    {
    struct atmel_hlcdc_regmap *hregmap = context;
    if (reg <= ATMEL_HLCDC_DIS) {
    u32 status;
    int ret;
    ret = readl_poll_timeout_atomic(hregmap.regs + ATMEL_HLCDC_SR,
    status,
    !(status & ATMEL_HLCDC_SIP),
    1, 100);
    if (ret) {
    dev_err(hregmap.dev,
    "Timeout! Clock domain synchronization is in progress!\n");
    return ret;
    }
    }
    writel(val, hregmap.regs + reg);
    return 0;
    }
    static int regmap_atmel_hlcdc_reg_read(void *context, unsigned int reg,
    unsigned int *val)
    {
    struct atmel_hlcdc_regmap *hregmap = context;
// val = readl(hregmap->regs + reg);
    return 0;
    }
    static const struct regmap_config atmel_hlcdc_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = ATMEL_HLCDC_REG_MAX,
    .reg_write = regmap_atmel_hlcdc_reg_write,
    .reg_read = regmap_atmel_hlcdc_reg_read,
    .fast_io = true,
    };
#[no_mangle]
unsafe extern "C" fn atmel_hlcdc_probe(pdev: *mut platform_device) -> c_int {
    static int atmel_hlcdc_probe(struct platform_device *pdev)
    {
    struct atmel_hlcdc_regmap *hregmap;
    struct device *dev = &pdev.dev;
    struct atmel_hlcdc *hlcdc;
    hregmap = devm_kzalloc(dev, sizeof(*hregmap), GFP_KERNEL);
    if (!hregmap)
    return -ENOMEM;
    hlcdc = devm_kzalloc(dev, sizeof(*hlcdc), GFP_KERNEL);
    if (!hlcdc)
    return -ENOMEM;
    hregmap.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(hregmap.regs))
    return PTR_ERR(hregmap.regs);
    hregmap.dev = &pdev.dev;
    hlcdc.irq = platform_get_irq(pdev, 0);
    if (hlcdc.irq < 0)
    return hlcdc.irq;
    hlcdc.periph_clk = devm_clk_get(dev, "periph_clk");
    if (IS_ERR(hlcdc.periph_clk)) {
    dev_err(dev, "failed to get peripheral clock\n");
    return PTR_ERR(hlcdc.periph_clk);
    }
//
// Retrieve one of the primary clocks required for LCD operation:
// prefer sys_clk (for RGB/MIPI), and fall back to lvds_pll_clk
// (for LVDS) if needed.
//
    hlcdc.sys_clk = devm_clk_get(dev, "sys_clk");
    if (IS_ERR(hlcdc.sys_clk)) {
    hlcdc.sys_clk = core::ptr::null_mut();
    hlcdc.lvds_pll_clk = devm_clk_get(dev, "lvds_pll_clk");
    if (IS_ERR(hlcdc.lvds_pll_clk)) {
    dev_err(dev, "Failed to obtain both the LCDC (generic) and LVDS PLL clocks\n");
    return PTR_ERR(hlcdc.lvds_pll_clk);
    }
    }
    hlcdc.slow_clk = devm_clk_get(dev, "slow_clk");
    if (IS_ERR(hlcdc.slow_clk)) {
    dev_err(dev, "failed to get slow clock\n");
    return PTR_ERR(hlcdc.slow_clk);
    }
    hlcdc.regmap = devm_regmap_init(dev, core::ptr::null_mut(), hregmap,
    &atmel_hlcdc_regmap_config);
    if (IS_ERR(hlcdc.regmap))
    return PTR_ERR(hlcdc.regmap);
    dev_set_drvdata(dev, hlcdc);
    return devm_mfd_add_devices(dev, -1, atmel_hlcdc_cells,
    ARRAY_SIZE(atmel_hlcdc_cells),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    }
    static const struct of_device_id atmel_hlcdc_match[] = {
    { .compatible = "atmel,at91sam9n12-hlcdc" },
    { .compatible = "atmel,at91sam9x5-hlcdc" },
    { .compatible = "atmel,sama5d2-hlcdc" },
    { .compatible = "atmel,sama5d3-hlcdc" },
    { .compatible = "atmel,sama5d4-hlcdc" },
    { .compatible = "microchip,sam9x60-hlcdc" },
    { .compatible = "microchip,sam9x75-xlcdc" },
    { .compatible = "microchip,sama7d65-xlcdc" },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, atmel_hlcdc_match);
    static struct platform_driver atmel_hlcdc_driver = {
    .probe = atmel_hlcdc_probe,
    .driver = {
    .name = "atmel-hlcdc",
    .of_match_table = atmel_hlcdc_match,
    },
    };
    module_platform_driver(atmel_hlcdc_driver);
    MODULE_ALIAS("platform:atmel-hlcdc");
    MODULE_AUTHOR("Boris Brezillon <boris.brezillon@free-electrons.com>");
    MODULE_DESCRIPTION("Atmel HLCDC driver");
    MODULE_LICENSE("GPL v2");
