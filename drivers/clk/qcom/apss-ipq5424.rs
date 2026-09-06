//! Automatically rewritten from C to Rust
//! Source: drivers/clk/qcom/apss-ipq5424.c
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
// Copyright (c) 2018, The Linux Foundation. All rights reserved.
// Copyright (c) 2025, Qualcomm Innovation Center, Inc. All rights reserved.
//

    enum {
    DT_XO,
    DT_CLK_REF,
    };
    enum {
    P_XO,
    P_GPLL0,
    P_APSS_PLL_EARLY,
    P_L3_PLL,
    };
    static const struct alpha_pll_config apss_pll_config = {
    .l = 0x3b,
    .config_ctl_val = 0x08200920,
    .config_ctl_hi_val = 0x05008001,
    .config_ctl_hi1_val = 0x04000000,
    .user_ctl_val = 0xf,
    };
    static struct clk_alpha_pll ipq5424_apss_pll = {
    .offset = 0x0,
    .config = &apss_pll_config,
    .regs = clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_HUAYRA_2290],
    .flags = SUPPORTS_DYNAMIC_UPDATE,
    .clkr = {
    .enable_reg = 0x0,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data){
    .name = "apss_pll",
    .parent_data = &(const struct clk_parent_data) {
    .index = DT_XO,
    },
    .num_parents = 1,
    .ops = &clk_alpha_pll_huayra_ops,
    },
    },
    };
    static const struct clk_parent_data parents_apss_silver_clk_src[] = {
    { .index = DT_XO },
    { .index = DT_CLK_REF },
    { .hw = &ipq5424_apss_pll.clkr.hw },
    };
    static const struct parent_map parents_apss_silver_clk_src_map[] = {
    { P_XO, 0 },
    { P_GPLL0, 4 },
    { P_APSS_PLL_EARLY, 5 },
    };
    static const struct freq_tbl ftbl_apss_clk_src[] = {
    F(816000000, P_APSS_PLL_EARLY, 1, 0, 0),
    F(1416000000, P_APSS_PLL_EARLY, 1, 0, 0),
    F(1800000000, P_APSS_PLL_EARLY, 1, 0, 0),
    { }
    };
    static struct clk_rcg2 apss_silver_clk_src = {
    .cmd_rcgr = 0x0080,
    .freq_tbl = ftbl_apss_clk_src,
    .hid_width = 5,
    .parent_map = parents_apss_silver_clk_src_map,
    .clkr.hw.init = &(struct clk_init_data) {
    .name = "apss_silver_clk_src",
    .parent_data = parents_apss_silver_clk_src,
    .num_parents = ARRAY_SIZE(parents_apss_silver_clk_src),
    .ops = &clk_rcg2_ops,
    .flags = CLK_SET_RATE_PARENT,
    },
    };
    static struct clk_branch apss_silver_core_clk = {
    .halt_reg = 0x008c,
    .clkr = {
    .enable_reg = 0x008c,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data) {
    .name = "apss_silver_core_clk",
    .parent_hws = (const struct clk_hw *[]) {
    &apss_silver_clk_src.clkr.hw
    },
    .num_parents = 1,
    .flags = CLK_SET_RATE_PARENT | CLK_IS_CRITICAL,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static const struct alpha_pll_config l3_pll_config = {
    .l = 0x29,
    .config_ctl_val = 0x08200920,
    .config_ctl_hi_val = 0x05008001,
    .config_ctl_hi1_val = 0x04000000,
    .user_ctl_val = 0xf,
    };
    static struct clk_alpha_pll ipq5424_l3_pll = {
    .offset = 0x10000,
    .config = &l3_pll_config,
    .regs = clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_HUAYRA_2290],
    .flags = SUPPORTS_DYNAMIC_UPDATE,
    .clkr = {
    .enable_reg = 0x0,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data) {
    .name = "l3_pll",
    .parent_data = &(const struct clk_parent_data) {
    .index = DT_XO,
    },
    .num_parents = 1,
    .ops = &clk_alpha_pll_huayra_ops,
    },
    },
    };
    static const struct clk_parent_data parents_l3_clk_src[] = {
    { .index = DT_XO },
    { .index = DT_CLK_REF },
    { .hw = &ipq5424_l3_pll.clkr.hw },
    };
    static const struct parent_map parents_l3_clk_src_map[] = {
    { P_XO, 0 },
    { P_GPLL0, 4 },
    { P_L3_PLL, 5 },
    };
    static const struct freq_tbl ftbl_l3_clk_src[] = {
    F(816000000, P_L3_PLL, 1, 0, 0),
    F(984000000, P_L3_PLL, 1, 0, 0),
    F(1272000000, P_L3_PLL, 1, 0, 0),
    { }
    };
    static struct clk_rcg2 l3_clk_src = {
    .cmd_rcgr = 0x10080,
    .freq_tbl = ftbl_l3_clk_src,
    .hid_width = 5,
    .parent_map = parents_l3_clk_src_map,
    .clkr.hw.init = &(struct clk_init_data) {
    .name = "l3_clk_src",
    .parent_data = parents_l3_clk_src,
    .num_parents = ARRAY_SIZE(parents_l3_clk_src),
    .ops = &clk_rcg2_ops,
    .flags = CLK_SET_RATE_PARENT,
    },
    };
    static struct clk_branch l3_core_clk = {
    .halt_reg = 0x1008c,
    .clkr = {
    .enable_reg = 0x1008c,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data) {
    .name = "l3_clk",
    .parent_hws = (const struct clk_hw *[]) {
    &l3_clk_src.clkr.hw
    },
    .num_parents = 1,
    .flags = CLK_SET_RATE_PARENT | CLK_IS_CRITICAL,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static const struct regmap_config apss_ipq5424_regmap_config = {
    .reg_bits       = 32,
    .reg_stride     = 4,
    .val_bits       = 32,
    .max_register   = 0x20000,
    .fast_io        = true,
    };
    static struct clk_regmap *apss_ipq5424_clks[] = {
    [APSS_PLL_EARLY] = &ipq5424_apss_pll.clkr,
    [APSS_SILVER_CLK_SRC] = &apss_silver_clk_src.clkr,
    [APSS_SILVER_CORE_CLK] = &apss_silver_core_clk.clkr,
    [L3_PLL] = &ipq5424_l3_pll.clkr,
    [L3_CLK_SRC] = &l3_clk_src.clkr,
    [L3_CORE_CLK] = &l3_core_clk.clkr,
    };
    static struct clk_alpha_pll *ipa5424_apss_plls[] = {
    &ipq5424_l3_pll,
    &ipq5424_apss_pll,
    };
    static const struct qcom_cc_driver_data ipa5424_apss_driver_data = {
    .alpha_plls = ipa5424_apss_plls,
    .num_alpha_plls = ARRAY_SIZE(ipa5424_apss_plls),
    };

    static const struct qcom_icc_hws_data icc_ipq5424_cpu_l3[] = {
    { MASTER_CPU, SLAVE_L3, L3_CORE_CLK },
    };
    static const struct qcom_cc_desc apss_ipq5424_desc = {
    .config = &apss_ipq5424_regmap_config,
    .clks = apss_ipq5424_clks,
    .num_clks = ARRAY_SIZE(apss_ipq5424_clks),
    .icc_hws = icc_ipq5424_cpu_l3,
    .num_icc_hws = ARRAY_SIZE(icc_ipq5424_cpu_l3),
    .icc_first_node_id = IPQ_APPS_PLL_ID,
    .driver_data = &ipa5424_apss_driver_data,
    };
#[no_mangle]
unsafe extern "C" fn apss_ipq5424_probe(pdev: *mut platform_device) -> c_int {
    static int apss_ipq5424_probe(struct platform_device *pdev)
    {
    return qcom_cc_probe(pdev, &apss_ipq5424_desc);
    }
    static const struct of_device_id apss_ipq5424_match_table[] = {
    { .compatible = "qcom,ipq5424-apss-clk" },
    { }
    };
    MODULE_DEVICE_TABLE(of, apss_ipq5424_match_table);
    static struct platform_driver apss_ipq5424_driver = {
    .probe = apss_ipq5424_probe,
    .driver = {
    .name   = "apss-ipq5424-clk",
    .of_match_table = apss_ipq5424_match_table,
    .sync_state = icc_sync_state,
    },
    };
    module_platform_driver(apss_ipq5424_driver);
    MODULE_DESCRIPTION("QCOM APSS IPQ5424 CLK Driver");
    MODULE_LICENSE("GPL");
