//! Automatically rewritten from C to Rust
//! Source: drivers/clk/qcom/turingcc-qcs404.c
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
// Copyright (c) 2019, Linaro Ltd.
//

    static struct clk_branch turing_wrapper_aon_cbcr = {
    .halt_reg = 0x5098,
    .halt_check = BRANCH_HALT,
    .clkr = {
    .enable_reg = 0x5098,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data) {
    .name = "turing_wrapper_aon_clk",
    .ops = &clk_branch2_aon_ops,
    },
    },
    };
    static struct clk_branch turing_q6ss_ahbm_aon_cbcr = {
    .halt_reg = 0x9000,
    .halt_check = BRANCH_HALT,
    .clkr = {
    .enable_reg = 0x9000,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data) {
    .name = "turing_q6ss_ahbm_aon_cbcr",
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch turing_q6ss_q6_axim_clk = {
    .halt_reg = 0xb000,
    .halt_check = BRANCH_HALT,
    .clkr = {
    .enable_reg = 0xb000,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data) {
    .name = "turing_q6ss_q6_axim_clk",
    .ops = &clk_branch2_aon_ops,
    },
    },
    };
    static struct clk_branch turing_q6ss_ahbs_aon_cbcr = {
    .halt_reg = 0x10000,
    .halt_check = BRANCH_HALT,
    .clkr = {
    .enable_reg = 0x10000,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data) {
    .name = "turing_q6ss_ahbs_aon_clk",
    .ops = &clk_branch2_aon_ops,
    },
    },
    };
    static struct clk_branch turing_wrapper_qos_ahbs_aon_cbcr = {
    .halt_reg = 0x11014,
    .halt_check = BRANCH_HALT,
    .clkr = {
    .enable_reg = 0x11014,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data) {
    .name = "turing_wrapper_qos_ahbs_aon_clk",
    .ops = &clk_branch2_aon_ops,
    },
    },
    };
    static struct clk_regmap *turingcc_clocks[] = {
    [TURING_WRAPPER_AON_CLK] = &turing_wrapper_aon_cbcr.clkr,
    [TURING_Q6SS_AHBM_AON_CLK] = &turing_q6ss_ahbm_aon_cbcr.clkr,
    [TURING_Q6SS_Q6_AXIM_CLK] = &turing_q6ss_q6_axim_clk.clkr,
    [TURING_Q6SS_AHBS_AON_CLK] = &turing_q6ss_ahbs_aon_cbcr.clkr,
    [TURING_WRAPPER_QOS_AHBS_AON_CLK] = &turing_wrapper_qos_ahbs_aon_cbcr.clkr,
    };
    static const struct regmap_config turingcc_regmap_config = {
    .reg_bits	= 32,
    .reg_stride	= 4,
    .val_bits	= 32,
    .max_register	= 0x23004,
    .fast_io	= true,
    };
    static const struct qcom_cc_desc turingcc_desc = {
    .config = &turingcc_regmap_config,
    .clks = turingcc_clocks,
    .num_clks = ARRAY_SIZE(turingcc_clocks),
    };
#[no_mangle]
unsafe extern "C" fn turingcc_probe(pdev: *mut platform_device) -> c_int {
    static int turingcc_probe(struct platform_device *pdev)
    {
    int ret;
    ret = devm_pm_runtime_enable(&pdev.dev);
    if (ret)
    return ret;
    ret = devm_pm_clk_create(&pdev.dev);
    if (ret)
    return ret;
    ret = pm_clk_add(&pdev.dev, core::ptr::null_mut());
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to acquire iface clock\n");
    return ret;
    }
    ret = pm_runtime_resume_and_get(&pdev.dev);
    if (ret)
    return ret;
    ret = qcom_cc_probe(pdev, &turingcc_desc);
    if (ret < 0)
    goto err_put_rpm;
    pm_runtime_put(&pdev.dev);
    return 0;
    err_put_rpm:
    pm_runtime_put_sync(&pdev.dev);
    return ret;
    }
    static const struct dev_pm_ops turingcc_pm_ops = {
    SET_RUNTIME_PM_OPS(pm_clk_suspend, pm_clk_resume, core::ptr::null_mut())
    };
    static const struct of_device_id turingcc_match_table[] = {
    { .compatible = "qcom,qcs404-turingcc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, turingcc_match_table);
    static struct platform_driver turingcc_driver = {
    .probe		= turingcc_probe,
    .driver		= {
    .name	= "qcs404-turingcc",
    .of_match_table = turingcc_match_table,
    .pm = &turingcc_pm_ops,
    },
    };
    module_platform_driver(turingcc_driver);
    MODULE_DESCRIPTION("Qualcomm QCS404 Turing Clock Controller");
    MODULE_LICENSE("GPL v2");
