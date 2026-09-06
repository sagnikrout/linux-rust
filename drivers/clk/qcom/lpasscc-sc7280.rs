//! Automatically rewritten from C to Rust
//! Source: drivers/clk/qcom/lpasscc-sc7280.c
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
// Copyright (c) 2021, The Linux Foundation. All rights reserved.
//

    static struct clk_branch lpass_top_cc_lpi_q6_axim_hs_clk = {
    .halt_reg = 0x0,
    .halt_check = BRANCH_HALT,
    .clkr = {
    .enable_reg = 0x0,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data){
    .name = "lpass_top_cc_lpi_q6_axim_hs_clk",
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch lpass_qdsp6ss_core_clk = {
    .halt_reg = 0x20,
// CLK_OFF would not toggle until LPASS is out of reset
    .halt_check = BRANCH_HALT_SKIP,
    .clkr = {
    .enable_reg = 0x20,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data){
    .name = "lpass_qdsp6ss_core_clk",
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch lpass_qdsp6ss_xo_clk = {
    .halt_reg = 0x38,
// CLK_OFF would not toggle until LPASS is out of reset
    .halt_check = BRANCH_HALT_SKIP,
    .clkr = {
    .enable_reg = 0x38,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data){
    .name = "lpass_qdsp6ss_xo_clk",
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch lpass_qdsp6ss_sleep_clk = {
    .halt_reg = 0x3c,
// CLK_OFF would not toggle until LPASS is out of reset
    .halt_check = BRANCH_HALT_SKIP,
    .clkr = {
    .enable_reg = 0x3c,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data){
    .name = "lpass_qdsp6ss_sleep_clk",
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct regmap_config lpass_regmap_config = {
    .reg_bits	= 32,
    .reg_stride	= 4,
    .val_bits	= 32,
    .fast_io	= true,
    };
    static struct clk_regmap *lpass_cc_top_sc7280_clocks[] = {
    [LPASS_TOP_CC_LPI_Q6_AXIM_HS_CLK] =
    &lpass_top_cc_lpi_q6_axim_hs_clk.clkr,
    };
    static const struct qcom_cc_desc lpass_cc_top_sc7280_desc = {
    .config = &lpass_regmap_config,
    .clks = lpass_cc_top_sc7280_clocks,
    .num_clks = ARRAY_SIZE(lpass_cc_top_sc7280_clocks),
    };
    static struct clk_regmap *lpass_qdsp6ss_sc7280_clocks[] = {
    [LPASS_QDSP6SS_XO_CLK] = &lpass_qdsp6ss_xo_clk.clkr,
    [LPASS_QDSP6SS_SLEEP_CLK] = &lpass_qdsp6ss_sleep_clk.clkr,
    [LPASS_QDSP6SS_CORE_CLK] = &lpass_qdsp6ss_core_clk.clkr,
    };
    static const struct qcom_cc_desc lpass_qdsp6ss_sc7280_desc = {
    .config = &lpass_regmap_config,
    .clks = lpass_qdsp6ss_sc7280_clocks,
    .num_clks = ARRAY_SIZE(lpass_qdsp6ss_sc7280_clocks),
    };
#[no_mangle]
unsafe extern "C" fn lpass_cc_sc7280_probe(pdev: *mut platform_device) -> c_int {
    static int lpass_cc_sc7280_probe(struct platform_device *pdev)
    {
    const struct qcom_cc_desc *desc;
    int ret;
    ret = devm_pm_runtime_enable(&pdev.dev);
    if (ret)
    return ret;
    ret = pm_clk_create(&pdev.dev);
    if (ret)
    return ret;
    ret = pm_clk_add(&pdev.dev, "iface");
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to acquire iface clock\n");
    goto err_destroy_pm_clk;
    }
    ret = pm_runtime_resume_and_get(&pdev.dev);
    if (ret)
    goto err_destroy_pm_clk;
    if (!of_property_read_bool(pdev.dev.of_node, "qcom,adsp-pil-mode")) {
    lpass_regmap_config.name = "qdsp6ss";
    lpass_regmap_config.max_register = 0x3f;
    desc = &lpass_qdsp6ss_sc7280_desc;
    ret = qcom_cc_probe_by_index(pdev, 0, desc);
    if (ret)
    goto err_put_rpm;
    }
    lpass_regmap_config.name = "top_cc";
    lpass_regmap_config.max_register = 0x4;
    desc = &lpass_cc_top_sc7280_desc;
    ret = qcom_cc_probe_by_index(pdev, 1, desc);
    if (ret)
    goto err_put_rpm;
    pm_runtime_put(&pdev.dev);
    return 0;
    err_put_rpm:
    pm_runtime_put_sync(&pdev.dev);
    err_destroy_pm_clk:
    pm_clk_destroy(&pdev.dev);
    return ret;
    }
    static const struct of_device_id lpass_cc_sc7280_match_table[] = {
    { .compatible = "qcom,sc7280-lpasscc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, lpass_cc_sc7280_match_table);
    static struct platform_driver lpass_cc_sc7280_driver = {
    .probe		= lpass_cc_sc7280_probe,
    .driver		= {
    .name	= "sc7280-lpasscc",
    .of_match_table = lpass_cc_sc7280_match_table,
    },
    };
#[no_mangle]
unsafe extern "C" fn lpass_cc_sc7280_init() -> int __init {
    static int __init lpass_cc_sc7280_init(void)
    {
    return platform_driver_register(&lpass_cc_sc7280_driver);
    }
    subsys_initcall(lpass_cc_sc7280_init);
#[no_mangle]
unsafe extern "C" fn lpass_cc_sc7280_exit() -> void __exit {
    static void __exit lpass_cc_sc7280_exit(void)
    {
    platform_driver_unregister(&lpass_cc_sc7280_driver);
    }
    module_exit(lpass_cc_sc7280_exit);
    MODULE_DESCRIPTION("QTI LPASS_CC SC7280 Driver");
    MODULE_LICENSE("GPL v2");
