//! Automatically rewritten from C to Rust
//! Source: drivers/clk/qcom/cambistmclkcc-kaanapali.c
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
    DT_AHB_CLK,
    DT_BI_TCXO,
    DT_BI_TCXO_AO,
    DT_SLEEP_CLK,
    };
    enum {
    P_BI_TCXO,
    P_CAM_BIST_MCLK_CC_PLL0_OUT_EVEN,
    P_CAM_BIST_MCLK_CC_PLL0_OUT_MAIN,
    };
    static const struct pll_vco rivian_eko_t_vco[] = {
    { 883200000, 1171200000, 0 },
    };
// 960.0 MHz Configuration
    static const struct alpha_pll_config cam_bist_mclk_cc_pll0_config = {
    .l = 0x32,
    .cal_l = 0x32,
    .alpha = 0x0,
    .config_ctl_val = 0x12000000,
    .config_ctl_hi_val = 0x00890263,
    .config_ctl_hi1_val = 0x1af04237,
    .config_ctl_hi2_val = 0x00000000,
    };
    static struct clk_alpha_pll cam_bist_mclk_cc_pll0 = {
    .offset = 0x0,
    .config = &cam_bist_mclk_cc_pll0_config,
    .vco_table = rivian_eko_t_vco,
    .num_vco = ARRAY_SIZE(rivian_eko_t_vco),
    .regs = clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_RIVIAN_EKO_T],
    .clkr = {
    .hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_pll0",
    .parent_data = &(const struct clk_parent_data) {
    .index = DT_BI_TCXO,
    },
    .num_parents = 1,
    .ops = &clk_alpha_pll_rivian_eko_t_ops,
    },
    },
    };
    static const struct parent_map cam_bist_mclk_cc_parent_map_0[] = {
    { P_BI_TCXO, 0 },
    { P_CAM_BIST_MCLK_CC_PLL0_OUT_EVEN, 3 },
    { P_CAM_BIST_MCLK_CC_PLL0_OUT_MAIN, 5 },
    };
    static const struct clk_parent_data cam_bist_mclk_cc_parent_data_0[] = {
    { .index = DT_BI_TCXO },
    { .hw = &cam_bist_mclk_cc_pll0.clkr.hw },
    { .hw = &cam_bist_mclk_cc_pll0.clkr.hw },
    };
    static const struct freq_tbl ftbl_cam_bist_mclk_cc_mclk0_clk_src[] = {
    F(19200000, P_BI_TCXO, 1, 0, 0),
    F(24000000, P_CAM_BIST_MCLK_CC_PLL0_OUT_EVEN, 10, 1, 4),
    F(68571429, P_CAM_BIST_MCLK_CC_PLL0_OUT_MAIN, 14, 0, 0),
    { }
    };
    static struct clk_rcg2 cam_bist_mclk_cc_mclk0_clk_src = {
    .cmd_rcgr = 0x4000,
    .mnd_width = 8,
    .hid_width = 5,
    .parent_map = cam_bist_mclk_cc_parent_map_0,
    .hw_clk_ctrl = true,
    .freq_tbl = ftbl_cam_bist_mclk_cc_mclk0_clk_src,
    .clkr.hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_mclk0_clk_src",
    .parent_data = cam_bist_mclk_cc_parent_data_0,
    .num_parents = ARRAY_SIZE(cam_bist_mclk_cc_parent_data_0),
    .flags = CLK_SET_RATE_PARENT,
    .ops = &clk_rcg2_shared_ops,
    },
    };
    static struct clk_rcg2 cam_bist_mclk_cc_mclk1_clk_src = {
    .cmd_rcgr = 0x401c,
    .mnd_width = 8,
    .hid_width = 5,
    .parent_map = cam_bist_mclk_cc_parent_map_0,
    .hw_clk_ctrl = true,
    .freq_tbl = ftbl_cam_bist_mclk_cc_mclk0_clk_src,
    .clkr.hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_mclk1_clk_src",
    .parent_data = cam_bist_mclk_cc_parent_data_0,
    .num_parents = ARRAY_SIZE(cam_bist_mclk_cc_parent_data_0),
    .flags = CLK_SET_RATE_PARENT,
    .ops = &clk_rcg2_shared_ops,
    },
    };
    static struct clk_rcg2 cam_bist_mclk_cc_mclk2_clk_src = {
    .cmd_rcgr = 0x4038,
    .mnd_width = 8,
    .hid_width = 5,
    .parent_map = cam_bist_mclk_cc_parent_map_0,
    .hw_clk_ctrl = true,
    .freq_tbl = ftbl_cam_bist_mclk_cc_mclk0_clk_src,
    .clkr.hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_mclk2_clk_src",
    .parent_data = cam_bist_mclk_cc_parent_data_0,
    .num_parents = ARRAY_SIZE(cam_bist_mclk_cc_parent_data_0),
    .flags = CLK_SET_RATE_PARENT,
    .ops = &clk_rcg2_shared_ops,
    },
    };
    static struct clk_rcg2 cam_bist_mclk_cc_mclk3_clk_src = {
    .cmd_rcgr = 0x4054,
    .mnd_width = 8,
    .hid_width = 5,
    .parent_map = cam_bist_mclk_cc_parent_map_0,
    .hw_clk_ctrl = true,
    .freq_tbl = ftbl_cam_bist_mclk_cc_mclk0_clk_src,
    .clkr.hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_mclk3_clk_src",
    .parent_data = cam_bist_mclk_cc_parent_data_0,
    .num_parents = ARRAY_SIZE(cam_bist_mclk_cc_parent_data_0),
    .flags = CLK_SET_RATE_PARENT,
    .ops = &clk_rcg2_shared_ops,
    },
    };
    static struct clk_rcg2 cam_bist_mclk_cc_mclk4_clk_src = {
    .cmd_rcgr = 0x4070,
    .mnd_width = 8,
    .hid_width = 5,
    .parent_map = cam_bist_mclk_cc_parent_map_0,
    .hw_clk_ctrl = true,
    .freq_tbl = ftbl_cam_bist_mclk_cc_mclk0_clk_src,
    .clkr.hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_mclk4_clk_src",
    .parent_data = cam_bist_mclk_cc_parent_data_0,
    .num_parents = ARRAY_SIZE(cam_bist_mclk_cc_parent_data_0),
    .flags = CLK_SET_RATE_PARENT,
    .ops = &clk_rcg2_shared_ops,
    },
    };
    static struct clk_rcg2 cam_bist_mclk_cc_mclk5_clk_src = {
    .cmd_rcgr = 0x408c,
    .mnd_width = 8,
    .hid_width = 5,
    .parent_map = cam_bist_mclk_cc_parent_map_0,
    .hw_clk_ctrl = true,
    .freq_tbl = ftbl_cam_bist_mclk_cc_mclk0_clk_src,
    .clkr.hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_mclk5_clk_src",
    .parent_data = cam_bist_mclk_cc_parent_data_0,
    .num_parents = ARRAY_SIZE(cam_bist_mclk_cc_parent_data_0),
    .flags = CLK_SET_RATE_PARENT,
    .ops = &clk_rcg2_shared_ops,
    },
    };
    static struct clk_rcg2 cam_bist_mclk_cc_mclk6_clk_src = {
    .cmd_rcgr = 0x40a8,
    .mnd_width = 8,
    .hid_width = 5,
    .parent_map = cam_bist_mclk_cc_parent_map_0,
    .hw_clk_ctrl = true,
    .freq_tbl = ftbl_cam_bist_mclk_cc_mclk0_clk_src,
    .clkr.hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_mclk6_clk_src",
    .parent_data = cam_bist_mclk_cc_parent_data_0,
    .num_parents = ARRAY_SIZE(cam_bist_mclk_cc_parent_data_0),
    .flags = CLK_SET_RATE_PARENT,
    .ops = &clk_rcg2_shared_ops,
    },
    };
    static struct clk_rcg2 cam_bist_mclk_cc_mclk7_clk_src = {
    .cmd_rcgr = 0x40c4,
    .mnd_width = 8,
    .hid_width = 5,
    .parent_map = cam_bist_mclk_cc_parent_map_0,
    .hw_clk_ctrl = true,
    .freq_tbl = ftbl_cam_bist_mclk_cc_mclk0_clk_src,
    .clkr.hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_mclk7_clk_src",
    .parent_data = cam_bist_mclk_cc_parent_data_0,
    .num_parents = ARRAY_SIZE(cam_bist_mclk_cc_parent_data_0),
    .flags = CLK_SET_RATE_PARENT,
    .ops = &clk_rcg2_shared_ops,
    },
    };
    static struct clk_branch cam_bist_mclk_cc_mclk0_clk = {
    .halt_reg = 0x4018,
    .halt_check = BRANCH_HALT,
    .clkr = {
    .enable_reg = 0x4018,
    .enable_mask = BIT(0),
    .hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_mclk0_clk",
    .parent_hws = (const struct clk_hw*[]) {
    &cam_bist_mclk_cc_mclk0_clk_src.clkr.hw,
    },
    .num_parents = 1,
    .flags = CLK_SET_RATE_PARENT,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch cam_bist_mclk_cc_mclk1_clk = {
    .halt_reg = 0x4034,
    .halt_check = BRANCH_HALT,
    .clkr = {
    .enable_reg = 0x4034,
    .enable_mask = BIT(0),
    .hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_mclk1_clk",
    .parent_hws = (const struct clk_hw*[]) {
    &cam_bist_mclk_cc_mclk1_clk_src.clkr.hw,
    },
    .num_parents = 1,
    .flags = CLK_SET_RATE_PARENT,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch cam_bist_mclk_cc_mclk2_clk = {
    .halt_reg = 0x4050,
    .halt_check = BRANCH_HALT,
    .clkr = {
    .enable_reg = 0x4050,
    .enable_mask = BIT(0),
    .hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_mclk2_clk",
    .parent_hws = (const struct clk_hw*[]) {
    &cam_bist_mclk_cc_mclk2_clk_src.clkr.hw,
    },
    .num_parents = 1,
    .flags = CLK_SET_RATE_PARENT,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch cam_bist_mclk_cc_mclk3_clk = {
    .halt_reg = 0x406c,
    .halt_check = BRANCH_HALT,
    .clkr = {
    .enable_reg = 0x406c,
    .enable_mask = BIT(0),
    .hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_mclk3_clk",
    .parent_hws = (const struct clk_hw*[]) {
    &cam_bist_mclk_cc_mclk3_clk_src.clkr.hw,
    },
    .num_parents = 1,
    .flags = CLK_SET_RATE_PARENT,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch cam_bist_mclk_cc_mclk4_clk = {
    .halt_reg = 0x4088,
    .halt_check = BRANCH_HALT,
    .clkr = {
    .enable_reg = 0x4088,
    .enable_mask = BIT(0),
    .hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_mclk4_clk",
    .parent_hws = (const struct clk_hw*[]) {
    &cam_bist_mclk_cc_mclk4_clk_src.clkr.hw,
    },
    .num_parents = 1,
    .flags = CLK_SET_RATE_PARENT,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch cam_bist_mclk_cc_mclk5_clk = {
    .halt_reg = 0x40a4,
    .halt_check = BRANCH_HALT,
    .clkr = {
    .enable_reg = 0x40a4,
    .enable_mask = BIT(0),
    .hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_mclk5_clk",
    .parent_hws = (const struct clk_hw*[]) {
    &cam_bist_mclk_cc_mclk5_clk_src.clkr.hw,
    },
    .num_parents = 1,
    .flags = CLK_SET_RATE_PARENT,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch cam_bist_mclk_cc_mclk6_clk = {
    .halt_reg = 0x40c0,
    .halt_check = BRANCH_HALT,
    .clkr = {
    .enable_reg = 0x40c0,
    .enable_mask = BIT(0),
    .hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_mclk6_clk",
    .parent_hws = (const struct clk_hw*[]) {
    &cam_bist_mclk_cc_mclk6_clk_src.clkr.hw,
    },
    .num_parents = 1,
    .flags = CLK_SET_RATE_PARENT,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_branch cam_bist_mclk_cc_mclk7_clk = {
    .halt_reg = 0x40dc,
    .halt_check = BRANCH_HALT,
    .clkr = {
    .enable_reg = 0x40dc,
    .enable_mask = BIT(0),
    .hw.init = &(const struct clk_init_data) {
    .name = "cam_bist_mclk_cc_mclk7_clk",
    .parent_hws = (const struct clk_hw*[]) {
    &cam_bist_mclk_cc_mclk7_clk_src.clkr.hw,
    },
    .num_parents = 1,
    .flags = CLK_SET_RATE_PARENT,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static struct clk_regmap *cam_bist_mclk_cc_kaanapali_clocks[] = {
    [CAM_BIST_MCLK_CC_MCLK0_CLK] = &cam_bist_mclk_cc_mclk0_clk.clkr,
    [CAM_BIST_MCLK_CC_MCLK0_CLK_SRC] = &cam_bist_mclk_cc_mclk0_clk_src.clkr,
    [CAM_BIST_MCLK_CC_MCLK1_CLK] = &cam_bist_mclk_cc_mclk1_clk.clkr,
    [CAM_BIST_MCLK_CC_MCLK1_CLK_SRC] = &cam_bist_mclk_cc_mclk1_clk_src.clkr,
    [CAM_BIST_MCLK_CC_MCLK2_CLK] = &cam_bist_mclk_cc_mclk2_clk.clkr,
    [CAM_BIST_MCLK_CC_MCLK2_CLK_SRC] = &cam_bist_mclk_cc_mclk2_clk_src.clkr,
    [CAM_BIST_MCLK_CC_MCLK3_CLK] = &cam_bist_mclk_cc_mclk3_clk.clkr,
    [CAM_BIST_MCLK_CC_MCLK3_CLK_SRC] = &cam_bist_mclk_cc_mclk3_clk_src.clkr,
    [CAM_BIST_MCLK_CC_MCLK4_CLK] = &cam_bist_mclk_cc_mclk4_clk.clkr,
    [CAM_BIST_MCLK_CC_MCLK4_CLK_SRC] = &cam_bist_mclk_cc_mclk4_clk_src.clkr,
    [CAM_BIST_MCLK_CC_MCLK5_CLK] = &cam_bist_mclk_cc_mclk5_clk.clkr,
    [CAM_BIST_MCLK_CC_MCLK5_CLK_SRC] = &cam_bist_mclk_cc_mclk5_clk_src.clkr,
    [CAM_BIST_MCLK_CC_MCLK6_CLK] = &cam_bist_mclk_cc_mclk6_clk.clkr,
    [CAM_BIST_MCLK_CC_MCLK6_CLK_SRC] = &cam_bist_mclk_cc_mclk6_clk_src.clkr,
    [CAM_BIST_MCLK_CC_MCLK7_CLK] = &cam_bist_mclk_cc_mclk7_clk.clkr,
    [CAM_BIST_MCLK_CC_MCLK7_CLK_SRC] = &cam_bist_mclk_cc_mclk7_clk_src.clkr,
    [CAM_BIST_MCLK_CC_PLL0] = &cam_bist_mclk_cc_pll0.clkr,
    };
    static struct clk_alpha_pll *cam_bist_mclk_cc_kaanapali_plls[] = {
    &cam_bist_mclk_cc_pll0,
    };
    static const u32 cam_bist_mclk_cc_kaanapali_critical_cbcrs[] = {
    0x40e0, /* CAM_BIST_MCLK_CC_SLEEP_CLK */
    };
    static const struct regmap_config cam_bist_mclk_cc_kaanapali_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .max_register = 0x5010,
    .fast_io = true,
    };
    static const struct qcom_cc_driver_data cam_bist_mclk_cc_kaanapali_driver_data = {
    .alpha_plls = cam_bist_mclk_cc_kaanapali_plls,
    .num_alpha_plls = ARRAY_SIZE(cam_bist_mclk_cc_kaanapali_plls),
    .clk_cbcrs = cam_bist_mclk_cc_kaanapali_critical_cbcrs,
    .num_clk_cbcrs = ARRAY_SIZE(cam_bist_mclk_cc_kaanapali_critical_cbcrs),
    };
    static const struct qcom_cc_desc cam_bist_mclk_cc_kaanapali_desc = {
    .config = &cam_bist_mclk_cc_kaanapali_regmap_config,
    .clks = cam_bist_mclk_cc_kaanapali_clocks,
    .num_clks = ARRAY_SIZE(cam_bist_mclk_cc_kaanapali_clocks),
    .use_rpm = true,
    .driver_data = &cam_bist_mclk_cc_kaanapali_driver_data,
    };
    static const struct of_device_id cam_bist_mclk_cc_kaanapali_match_table[] = {
    { .compatible = "qcom,kaanapali-cambistmclkcc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, cam_bist_mclk_cc_kaanapali_match_table);
#[no_mangle]
unsafe extern "C" fn cam_bist_mclk_cc_kaanapali_probe(pdev: *mut platform_device) -> c_int {
    static int cam_bist_mclk_cc_kaanapali_probe(struct platform_device *pdev)
    {
    return qcom_cc_probe(pdev, &cam_bist_mclk_cc_kaanapali_desc);
    }
    static struct platform_driver cam_bist_mclk_cc_kaanapali_driver = {
    .probe = cam_bist_mclk_cc_kaanapali_probe,
    .driver = {
    .name = "cambistmclkcc-kaanapali",
    .of_match_table = cam_bist_mclk_cc_kaanapali_match_table,
    },
    };
    module_platform_driver(cam_bist_mclk_cc_kaanapali_driver);
    MODULE_DESCRIPTION("QTI CAMBISTMCLKCC Kaanapali Driver");
    MODULE_LICENSE("GPL");
