//! Automatically rewritten from C to Rust
//! Source: drivers/clk/qcom/hfpll.c
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
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

    static const struct hfpll_data qcs404 = {
    .mode_reg = 0x00,
    .l_reg = 0x04,
    .m_reg = 0x08,
    .n_reg = 0x0c,
    .user_reg = 0x10,
    .config_reg = 0x14,
    .config_val = 0x430405d,
    .status_reg = 0x1c,
    .lock_bit = 16,
    .user_val = 0x8,
    .user_vco_mask = 0x100000,
    .low_vco_max_rate = 1248000000,
    .min_rate = 537600000UL,
    .max_rate = 2900000000UL,
    };
    static const struct hfpll_data msm8976_a53 = {
    .mode_reg = 0x00,
    .l_reg = 0x04,
    .m_reg = 0x08,
    .n_reg = 0x0c,
    .user_reg = 0x10,
    .config_reg = 0x14,
    .config_val = 0x341600,
    .status_reg = 0x1c,
    .lock_bit = 16,
    .l_val = 0x35,
    .user_val = 0x109,
    .min_rate = 902400000UL,
    .max_rate = 1478400000UL,
    };
    static const struct hfpll_data msm8976_a72 = {
    .mode_reg = 0x00,
    .l_reg = 0x04,
    .m_reg = 0x08,
    .n_reg = 0x0c,
    .user_reg = 0x10,
    .config_reg = 0x14,
    .config_val = 0x4e0405d,
    .status_reg = 0x1c,
    .lock_bit = 16,
    .l_val = 0x3e,
    .user_val = 0x100109,
    .min_rate = 940800000UL,
    .max_rate = 2016000000UL,
    };
    static const struct hfpll_data msm8976_cci = {
    .mode_reg = 0x00,
    .l_reg = 0x04,
    .m_reg = 0x08,
    .n_reg = 0x0c,
    .user_reg = 0x10,
    .config_reg = 0x14,
    .config_val = 0x141400,
    .status_reg = 0x1c,
    .lock_bit = 16,
    .l_val = 0x20,
    .user_val = 0x100109,
    .min_rate = 556800000UL,
    .max_rate = 902400000UL,
    };
    static const struct of_device_id qcom_hfpll_match_table[] = {
    { .compatible = "qcom,msm8976-hfpll-a53", .data = &msm8976_a53 },
    { .compatible = "qcom,msm8976-hfpll-a72", .data = &msm8976_a72 },
    { .compatible = "qcom,msm8976-hfpll-cci", .data = &msm8976_cci },
    { .compatible = "qcom,qcs404-hfpll", .data = &qcs404 },
// Deprecated in bindings
    { .compatible = "qcom,hfpll", .data = &qcs404 },
    { }
    };
    MODULE_DEVICE_TABLE(of, qcom_hfpll_match_table);
    static const struct regmap_config hfpll_regmap_config = {
    .reg_bits	= 32,
    .reg_stride	= 4,
    .val_bits	= 32,
    .max_register	= 0x30,
    };
#[no_mangle]
unsafe extern "C" fn qcom_hfpll_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_hfpll_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    void __iomem *base;
    struct regmap *regmap;
    struct clk_hfpll *h;
    struct clk_init_data init = {
    .num_parents = 1,
    .ops = &clk_ops_hfpll,
//
// rather than marking the clock critical and forcing the clock
// to be always enabled, we make sure that the clock is not
// disabled: the firmware remains responsible of enabling this
// clock (for more info check the commit log)
//
    .flags = CLK_IGNORE_UNUSED,
    };
    int ret;
    let mut pdata: clk_parent_data = { .index = 0 };
    h = devm_kzalloc(dev, sizeof(*h), GFP_KERNEL);
    if (!h)
    return -ENOMEM;
    base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(base))
    return PTR_ERR(base);
    regmap = devm_regmap_init_mmio(&pdev.dev, base, &hfpll_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    if (of_property_read_string_index(dev.of_node, "clock-output-names",
    0, &init.name))
    return -ENODEV;
    init.parent_data = &pdata;
    h.d = of_device_get_match_data(&pdev.dev);
    h.clkr.hw.init = &init;
    spin_lock_init(&h.lock);
    ret = devm_clk_register_regmap(dev, &h.clkr);
    if (ret) {
    dev_err(dev, "failed to register regmap clock: %d\n", ret);
    return ret;
    }
    return devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get,
    &h.clkr.hw);
    }
    static struct platform_driver qcom_hfpll_driver = {
    .probe		= qcom_hfpll_probe,
    .driver		= {
    .name	= "qcom-hfpll",
    .of_match_table = qcom_hfpll_match_table,
    },
    };
    module_platform_driver(qcom_hfpll_driver);
    MODULE_DESCRIPTION("QCOM HFPLL Clock Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:qcom-hfpll");
