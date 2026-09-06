//! Automatically rewritten from C to Rust
//! Source: drivers/clk/qcom/tcsrcc-sm8550.c
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
// Copyright (c) 2021, The Linux Foundation. All rights reserved.
// Copyright (c) 2022, Qualcomm Innovation Center, Inc. All rights reserved.
// Copyright (c) 2022, Linaro Limited
//

    enum {
    DT_BI_TCXO_PAD,
    };
    static struct clk_branch tcsr_pcie_0_clkref_en = {
    .halt_reg = 0x15100,
    .halt_check = BRANCH_HALT_SKIP,
    .clkr = {
    .enable_reg = 0x15100,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data){
    .name = "tcsr_pcie_0_clkref_en",
    .parent_data = &(const struct clk_parent_data){
    .index = DT_BI_TCXO_PAD,
    },
    .num_parents = 1,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch tcsr_pcie_1_clkref_en = {
    .halt_reg = 0x15114,
    .halt_check = BRANCH_HALT_SKIP,
    .clkr = {
    .enable_reg = 0x15114,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data){
    .name = "tcsr_pcie_1_clkref_en",
    .parent_data = &(const struct clk_parent_data){
    .index = DT_BI_TCXO_PAD,
    },
    .num_parents = 1,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch tcsr_ufs_clkref_en = {
    .halt_reg = 0x15110,
    .halt_check = BRANCH_HALT_SKIP,
    .clkr = {
    .enable_reg = 0x15110,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data){
    .name = "tcsr_ufs_clkref_en",
    .parent_data = &(const struct clk_parent_data){
    .index = DT_BI_TCXO_PAD,
    },
    .num_parents = 1,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch tcsr_ufs_pad_clkref_en = {
    .halt_reg = 0x15104,
    .halt_check = BRANCH_HALT_SKIP,
    .clkr = {
    .enable_reg = 0x15104,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data){
    .name = "tcsr_ufs_pad_clkref_en",
    .parent_data = &(const struct clk_parent_data){
    .index = DT_BI_TCXO_PAD,
    },
    .num_parents = 1,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch tcsr_usb2_clkref_en = {
    .halt_reg = 0x15118,
    .halt_check = BRANCH_HALT_SKIP,
    .clkr = {
    .enable_reg = 0x15118,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data){
    .name = "tcsr_usb2_clkref_en",
    .parent_data = &(const struct clk_parent_data){
    .index = DT_BI_TCXO_PAD,
    },
    .num_parents = 1,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch tcsr_usb3_clkref_en = {
    .halt_reg = 0x15108,
    .halt_check = BRANCH_HALT_SKIP,
    .clkr = {
    .enable_reg = 0x15108,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data){
    .name = "tcsr_usb3_clkref_en",
    .parent_data = &(const struct clk_parent_data){
    .index = DT_BI_TCXO_PAD,
    },
    .num_parents = 1,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_regmap *tcsr_cc_sar2130p_clocks[] = {
    [TCSR_PCIE_0_CLKREF_EN] = &tcsr_pcie_0_clkref_en.clkr,
    [TCSR_PCIE_1_CLKREF_EN] = &tcsr_pcie_1_clkref_en.clkr,
    [TCSR_USB2_CLKREF_EN] = &tcsr_usb2_clkref_en.clkr,
    [TCSR_USB3_CLKREF_EN] = &tcsr_usb3_clkref_en.clkr,
    };
    static struct clk_regmap *tcsr_cc_sm8550_clocks[] = {
    [TCSR_PCIE_0_CLKREF_EN] = &tcsr_pcie_0_clkref_en.clkr,
    [TCSR_PCIE_1_CLKREF_EN] = &tcsr_pcie_1_clkref_en.clkr,
    [TCSR_UFS_CLKREF_EN] = &tcsr_ufs_clkref_en.clkr,
    [TCSR_UFS_PAD_CLKREF_EN] = &tcsr_ufs_pad_clkref_en.clkr,
    [TCSR_USB2_CLKREF_EN] = &tcsr_usb2_clkref_en.clkr,
    [TCSR_USB3_CLKREF_EN] = &tcsr_usb3_clkref_en.clkr,
    };
    static const struct regmap_config tcsr_cc_sm8550_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .max_register = 0x2f000,
    .fast_io = true,
    };
    static const struct qcom_cc_desc tcsr_cc_sar2130p_desc = {
    .config = &tcsr_cc_sm8550_regmap_config,
    .clks = tcsr_cc_sar2130p_clocks,
    .num_clks = ARRAY_SIZE(tcsr_cc_sar2130p_clocks),
    };
    static const struct qcom_cc_desc tcsr_cc_sm8550_desc = {
    .config = &tcsr_cc_sm8550_regmap_config,
    .clks = tcsr_cc_sm8550_clocks,
    .num_clks = ARRAY_SIZE(tcsr_cc_sm8550_clocks),
    };
    static const struct of_device_id tcsr_cc_sm8550_match_table[] = {
    { .compatible = "qcom,sar2130p-tcsr", .data = &tcsr_cc_sar2130p_desc },
    { .compatible = "qcom,sm8550-tcsr", .data = &tcsr_cc_sm8550_desc },
    { }
    };
    MODULE_DEVICE_TABLE(of, tcsr_cc_sm8550_match_table);
#[no_mangle]
unsafe extern "C" fn tcsr_cc_sm8550_probe(pdev: *mut platform_device) -> c_int {
    static int tcsr_cc_sm8550_probe(struct platform_device *pdev)
    {
    struct regmap *regmap;
    regmap = qcom_cc_map(pdev, of_device_get_match_data(&pdev.dev));
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return qcom_cc_really_probe(&pdev.dev, &tcsr_cc_sm8550_desc, regmap);
    }
    static struct platform_driver tcsr_cc_sm8550_driver = {
    .probe = tcsr_cc_sm8550_probe,
    .driver = {
    .name = "tcsr_cc-sm8550",
    .of_match_table = tcsr_cc_sm8550_match_table,
    },
    };
#[no_mangle]
unsafe extern "C" fn tcsr_cc_sm8550_init() -> int __init {
    static int __init tcsr_cc_sm8550_init(void)
    {
    return platform_driver_register(&tcsr_cc_sm8550_driver);
    }
    subsys_initcall(tcsr_cc_sm8550_init);
#[no_mangle]
unsafe extern "C" fn tcsr_cc_sm8550_exit() -> void __exit {
    static void __exit tcsr_cc_sm8550_exit(void)
    {
    platform_driver_unregister(&tcsr_cc_sm8550_driver);
    }
    module_exit(tcsr_cc_sm8550_exit);
    MODULE_DESCRIPTION("QTI TCSRCC SM8550 Driver");
    MODULE_LICENSE("GPL");
