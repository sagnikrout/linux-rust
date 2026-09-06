//! Automatically rewritten from C to Rust
//! Source: drivers/clk/qcom/apss-ipq6018.c
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
//

    enum {
    P_XO,
    P_GPLL0,
    P_APSS_PLL_EARLY,
    };
    static const struct clk_parent_data parents_apcs_alias0_clk_src[] = {
    { .fw_name = "xo" },
    { .fw_name = "gpll0" },
    { .fw_name = "pll" },
    };
    static const struct parent_map parents_apcs_alias0_clk_src_map[] = {
    { P_XO, 0 },
    { P_GPLL0, 4 },
    { P_APSS_PLL_EARLY, 5 },
    };
    static struct clk_rcg2 apcs_alias0_clk_src = {
    .cmd_rcgr = 0x0050,
    .hid_width = 5,
    .parent_map = parents_apcs_alias0_clk_src_map,
    .clkr.hw.init = &(struct clk_init_data){
    .name = "apcs_alias0_clk_src",
    .parent_data = parents_apcs_alias0_clk_src,
    .num_parents = ARRAY_SIZE(parents_apcs_alias0_clk_src),
    .ops = &clk_rcg2_mux_closest_ops,
    .flags = CLK_SET_RATE_PARENT,
    },
    };
    static struct clk_branch apcs_alias0_core_clk = {
    .halt_reg = 0x0058,
    .clkr = {
    .enable_reg = 0x0058,
    .enable_mask = BIT(0),
    .hw.init = &(struct clk_init_data){
    .name = "apcs_alias0_core_clk",
    .parent_hws = (const struct clk_hw *[]){
    &apcs_alias0_clk_src.clkr.hw },
    .num_parents = 1,
    .flags = CLK_SET_RATE_PARENT | CLK_IS_CRITICAL,
    .ops = &clk_branch2_ops,
    },
    },
    };
    static const struct regmap_config apss_ipq6018_regmap_config = {
    .reg_bits       = 32,
    .reg_stride     = 4,
    .val_bits       = 32,
    .max_register   = 0x1000,
    .fast_io        = true,
    };
    static struct clk_regmap *apss_ipq6018_clks[] = {
    [APCS_ALIAS0_CLK_SRC] = &apcs_alias0_clk_src.clkr,
    [APCS_ALIAS0_CORE_CLK] = &apcs_alias0_core_clk.clkr,
    };
    static const struct qcom_cc_desc apss_ipq6018_desc = {
    .config = &apss_ipq6018_regmap_config,
    .clks = apss_ipq6018_clks,
    .num_clks = ARRAY_SIZE(apss_ipq6018_clks),
    };
    static int cpu_clk_notifier_fn(struct notifier_block *nb, unsigned long action,
    void *data)
    {
    struct clk_hw *hw;
    u8 index;
    int err;
    if (action == PRE_RATE_CHANGE)
    index = P_GPLL0;
#[no_mangle]
pub unsafe extern "C" fn if(ABORT_RATE_CHANGE: action == POST_RATE_CHANGE || action ==) -> else {
    else if (action == POST_RATE_CHANGE || action == ABORT_RATE_CHANGE)
    index = P_APSS_PLL_EARLY;
    else
    return NOTIFY_OK;
    hw = &apcs_alias0_clk_src.clkr.hw;
    err = clk_rcg2_mux_closest_ops.set_parent(hw, index);
    return notifier_from_errno(err);
    }
#[no_mangle]
unsafe extern "C" fn apss_ipq6018_probe(pdev: *mut platform_device) -> c_int {
    static int apss_ipq6018_probe(struct platform_device *pdev)
    {
    struct clk_hw *hw = &apcs_alias0_clk_src.clkr.hw;
    struct notifier_block *cpu_clk_notifier;
    struct regmap *regmap;
    u32 soc_id;
    int ret;
    ret = qcom_smem_get_soc_id(&soc_id);
    if (ret)
    return ret;
    regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!regmap)
    return -ENODEV;
    ret = qcom_cc_really_probe(&pdev.dev, &apss_ipq6018_desc, regmap);
    if (ret)
    return ret;
    switch (soc_id) {
// Only below variants of IPQ53xx support scaling
    case QCOM_ID_IPQ5332:
    case QCOM_ID_IPQ5322:
    case QCOM_ID_IPQ5300:
    cpu_clk_notifier = devm_kzalloc(&pdev.dev,
    sizeof(*cpu_clk_notifier),
    GFP_KERNEL);
    if (!cpu_clk_notifier)
    return -ENOMEM;
    cpu_clk_notifier.notifier_call = cpu_clk_notifier_fn;
    ret = devm_clk_notifier_register(&pdev.dev, hw.clk, cpu_clk_notifier);
    if (ret)
    return ret;
    break;
    default:
    break;
    }
    return 0;
    }
    static struct platform_driver apss_ipq6018_driver = {
    .probe = apss_ipq6018_probe,
    .driver = {
    .name   = "qcom,apss-ipq6018-clk",
    },
    };
    module_platform_driver(apss_ipq6018_driver);
    MODULE_DESCRIPTION("QCOM APSS IPQ 6018 CLK Driver");
    MODULE_LICENSE("GPL v2");
