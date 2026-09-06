//! Automatically rewritten from C to Rust
//! Source: drivers/clk/qcom/tcsrcc-hawi.c
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

    enum {
    DT_BI_TCXO_PAD,
    };
    static struct clk_branch tcsr_pcie_0_clkref_en = {
    .halt_reg = 0x4c,
    .halt_check = BRANCH_HALT_DELAY,
    .clkr = {
    .enable_reg = 0x4c,
    .enable_mask = BIT(0),
    .hw.init = &(const struct clk_init_data) {
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
    .halt_reg = 0x0,
    .halt_check = BRANCH_HALT_DELAY,
    .clkr = {
    .enable_reg = 0x0,
    .enable_mask = BIT(0),
    .hw.init = &(const struct clk_init_data) {
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
    .halt_reg = 0x10,
    .halt_check = BRANCH_HALT_DELAY,
    .clkr = {
    .enable_reg = 0x10,
    .enable_mask = BIT(0),
    .hw.init = &(const struct clk_init_data) {
    .name = "tcsr_ufs_clkref_en",
    .parent_data = &(const struct clk_parent_data){
    .index = DT_BI_TCXO_PAD,
    },
    .num_parents = 1,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch tcsr_usb2_clkref_en = {
    .halt_reg = 0x18,
    .halt_check = BRANCH_HALT_DELAY,
    .clkr = {
    .enable_reg = 0x18,
    .enable_mask = BIT(0),
    .hw.init = &(const struct clk_init_data) {
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
    .halt_reg = 0x8,
    .halt_check = BRANCH_HALT_DELAY,
    .clkr = {
    .enable_reg = 0x8,
    .enable_mask = BIT(0),
    .hw.init = &(const struct clk_init_data) {
    .name = "tcsr_usb3_clkref_en",
    .parent_data = &(const struct clk_parent_data){
    .index = DT_BI_TCXO_PAD,
    },
    .num_parents = 1,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_regmap *tcsr_cc_hawi_clocks[] = {
    [TCSR_PCIE_0_CLKREF_EN] = &tcsr_pcie_0_clkref_en.clkr,
    [TCSR_PCIE_1_CLKREF_EN] = &tcsr_pcie_1_clkref_en.clkr,
    [TCSR_UFS_CLKREF_EN] = &tcsr_ufs_clkref_en.clkr,
    [TCSR_USB2_CLKREF_EN] = &tcsr_usb2_clkref_en.clkr,
    [TCSR_USB3_CLKREF_EN] = &tcsr_usb3_clkref_en.clkr,
    };
    static const struct regmap_config tcsr_cc_hawi_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .max_register = 0x4c,
    .fast_io = true,
    };
    static const struct qcom_cc_desc tcsr_cc_hawi_desc = {
    .config = &tcsr_cc_hawi_regmap_config,
    .clks = tcsr_cc_hawi_clocks,
    .num_clks = ARRAY_SIZE(tcsr_cc_hawi_clocks),
    };
    static const struct of_device_id tcsr_cc_hawi_match_table[] = {
    { .compatible = "qcom,hawi-tcsrcc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, tcsr_cc_hawi_match_table);
#[no_mangle]
unsafe extern "C" fn tcsr_cc_hawi_probe(pdev: *mut platform_device) -> c_int {
    static int tcsr_cc_hawi_probe(struct platform_device *pdev)
    {
    return qcom_cc_probe(pdev, &tcsr_cc_hawi_desc);
    }
    static struct platform_driver tcsr_cc_hawi_driver = {
    .probe = tcsr_cc_hawi_probe,
    .driver = {
    .name = "tcsrcc-hawi",
    .of_match_table = tcsr_cc_hawi_match_table,
    },
    };
    module_platform_driver(tcsr_cc_hawi_driver);
    MODULE_DESCRIPTION("QTI TCSRCC HAWI Driver");
    MODULE_LICENSE("GPL");
