//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-lochnagar.c
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
// Lochnagar clock control
//
// Copyright (c) 2017-2018 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//
// Author: Charles Keepax <ckeepax@opensource.cirrus.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lochnagar_clk {
    pub name: *const *const c_char,
    pub hw: clk_hw,
    pub priv: *mut lochnagar_clk_priv,
    pub cfg_reg: u16,
    pub ena_mask: u16,
    pub src_reg: u16,
    pub src_mask: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lochnagar_clk_priv {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub lclks: [lochnagar_clk; LOCHNAGAR_NUM_CLOCKS],
}

    static const struct clk_parent_data lochnagar1_clk_parents[] = {
    LN_PARENT("ln-none"),
    LN_PARENT("ln-spdif-mclk"),
    LN_PARENT("ln-psia1-mclk"),
    LN_PARENT("ln-psia2-mclk"),
    LN_PARENT("ln-cdc-clkout"),
    LN_PARENT("ln-dsp-clkout"),
    LN_PARENT("ln-pmic-32k"),
    LN_PARENT("ln-gf-mclk1"),
    LN_PARENT("ln-gf-mclk3"),
    LN_PARENT("ln-gf-mclk2"),
    LN_PARENT("ln-gf-mclk4"),
    };
    static const struct clk_parent_data lochnagar2_clk_parents[] = {
    LN_PARENT("ln-none"),
    LN_PARENT("ln-cdc-clkout"),
    LN_PARENT("ln-dsp-clkout"),
    LN_PARENT("ln-pmic-32k"),
    LN_PARENT("ln-spdif-mclk"),
    LN_PARENT("ln-clk-12m"),
    LN_PARENT("ln-clk-11m"),
    LN_PARENT("ln-clk-24m"),
    LN_PARENT("ln-clk-22m"),
    LN_PARENT("ln-clk-8m"),
    LN_PARENT("ln-usb-clk-24m"),
    LN_PARENT("ln-gf-mclk1"),
    LN_PARENT("ln-gf-mclk3"),
    LN_PARENT("ln-gf-mclk2"),
    LN_PARENT("ln-psia1-mclk"),
    LN_PARENT("ln-psia2-mclk"),
    LN_PARENT("ln-spdif-clkout"),
    LN_PARENT("ln-adat-mclk"),
    LN_PARENT("ln-usb-clk-12m"),
    };

    [LOCHNAGAR_##ID] = { \
    .name = NAME, \
    .cfg_reg = LOCHNAGAR1_##REG, \
    .ena_mask = LOCHNAGAR1_##ID##_ENA_MASK, \
    .src_reg = LOCHNAGAR1_##ID##_SEL, \
    .src_mask = LOCHNAGAR1_SRC_MASK, \
    }

    [LOCHNAGAR_##ID] = { \
    .name = NAME, \
    .cfg_reg = LOCHNAGAR2_##ID##_CTRL, \
    .src_reg = LOCHNAGAR2_##ID##_CTRL, \
    .ena_mask = LOCHNAGAR2_CLK_ENA_MASK, \
    .src_mask = LOCHNAGAR2_CLK_SRC_MASK, \
    }
    static const struct lochnagar_clk lochnagar1_clks[LOCHNAGAR_NUM_CLOCKS] = {
    LN1_CLK(CDC_MCLK1,      "ln-cdc-mclk1",  CDC_AIF_CTRL2),
    LN1_CLK(CDC_MCLK2,      "ln-cdc-mclk2",  CDC_AIF_CTRL2),
    LN1_CLK(DSP_CLKIN,      "ln-dsp-clkin",  DSP_AIF),
    LN1_CLK(GF_CLKOUT1,     "ln-gf-clkout1", GF_AIF1),
    };
    static const struct lochnagar_clk lochnagar2_clks[LOCHNAGAR_NUM_CLOCKS] = {
    LN2_CLK(CDC_MCLK1,      "ln-cdc-mclk1"),
    LN2_CLK(CDC_MCLK2,      "ln-cdc-mclk2"),
    LN2_CLK(DSP_CLKIN,      "ln-dsp-clkin"),
    LN2_CLK(GF_CLKOUT1,     "ln-gf-clkout1"),
    LN2_CLK(GF_CLKOUT2,     "ln-gf-clkout2"),
    LN2_CLK(PSIA1_MCLK,     "ln-psia1-mclk"),
    LN2_CLK(PSIA2_MCLK,     "ln-psia2-mclk"),
    LN2_CLK(SPDIF_MCLK,     "ln-spdif-mclk"),
    LN2_CLK(ADAT_MCLK,      "ln-adat-mclk"),
    LN2_CLK(SOUNDCARD_MCLK, "ln-soundcard-mclk"),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lochnagar_config {
    pub parents: *const clk_parent_data,
    pub nparents: c_int,
    pub clks: *const lochnagar_clk,
}

    static const struct lochnagar_config lochnagar1_conf = {
    .parents = lochnagar1_clk_parents,
    .nparents = ARRAY_SIZE(lochnagar1_clk_parents),
    .clks = lochnagar1_clks,
    };
    static const struct lochnagar_config lochnagar2_conf = {
    .parents = lochnagar2_clk_parents,
    .nparents = ARRAY_SIZE(lochnagar2_clk_parents),
    .clks = lochnagar2_clks,
    };
    static inline struct lochnagar_clk *lochnagar_hw_to_lclk(struct clk_hw *hw)
    {
    return container_of(hw, struct lochnagar_clk, hw);
    }
#[no_mangle]
unsafe extern "C" fn lochnagar_clk_prepare(hw: *mut clk_hw) -> c_int {
    static int lochnagar_clk_prepare(struct clk_hw *hw)
    {
    struct lochnagar_clk *lclk = lochnagar_hw_to_lclk(hw);
    struct lochnagar_clk_priv *priv = lclk.priv;
    struct regmap *regmap = priv.regmap;
    int ret;
    ret = regmap_update_bits(regmap, lclk.cfg_reg,
    lclk.ena_mask, lclk.ena_mask);
    if (ret < 0)
    dev_dbg(priv.dev, "Failed to prepare %s: %d\n",
    lclk.name, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lochnagar_clk_unprepare(hw: *mut clk_hw) {
    static void lochnagar_clk_unprepare(struct clk_hw *hw)
    {
    struct lochnagar_clk *lclk = lochnagar_hw_to_lclk(hw);
    struct lochnagar_clk_priv *priv = lclk.priv;
    struct regmap *regmap = priv.regmap;
    int ret;
    ret = regmap_update_bits(regmap, lclk.cfg_reg, lclk.ena_mask, 0);
    if (ret < 0)
    dev_dbg(priv.dev, "Failed to unprepare %s: %d\n",
    lclk.name, ret);
    }
#[no_mangle]
unsafe extern "C" fn lochnagar_clk_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int lochnagar_clk_set_parent(struct clk_hw *hw, u8 index)
    {
    struct lochnagar_clk *lclk = lochnagar_hw_to_lclk(hw);
    struct lochnagar_clk_priv *priv = lclk.priv;
    struct regmap *regmap = priv.regmap;
    int ret;
    ret = regmap_update_bits(regmap, lclk.src_reg, lclk.src_mask, index);
    if (ret < 0)
    dev_dbg(priv.dev, "Failed to reparent %s: %d\n",
    lclk.name, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lochnagar_clk_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 lochnagar_clk_get_parent(struct clk_hw *hw)
    {
    struct lochnagar_clk *lclk = lochnagar_hw_to_lclk(hw);
    struct lochnagar_clk_priv *priv = lclk.priv;
    struct regmap *regmap = priv.regmap;
    unsigned int val;
    int ret;
    ret = regmap_read(regmap, lclk.src_reg, &val);
    if (ret < 0) {
    dev_dbg(priv.dev, "Failed to read parent of %s: %d\n",
    lclk.name, ret);
    return clk_hw_get_num_parents(hw);
    }
    val &= lclk.src_mask;
    return val;
    }
    static const struct clk_ops lochnagar_clk_ops = {
    .prepare = lochnagar_clk_prepare,
    .unprepare = lochnagar_clk_unprepare,
    .determine_rate = clk_hw_determine_rate_no_reparent,
    .set_parent = lochnagar_clk_set_parent,
    .get_parent = lochnagar_clk_get_parent,
    };
    static struct clk_hw *
    lochnagar_of_clk_hw_get(struct of_phandle_args *clkspec, void *data)
    {
    struct lochnagar_clk_priv *priv = data;
    let mut idx: c_uint = clkspec.args[0];
    if (idx >= ARRAY_SIZE(priv.lclks)) {
    dev_err(priv.dev, "Invalid index %u\n", idx);
    return ERR_PTR(-EINVAL);
    }
    return &priv.lclks[idx].hw;
    }
    static const struct of_device_id lochnagar_of_match[] = {
    { .compatible = "cirrus,lochnagar1-clk", .data = &lochnagar1_conf },
    { .compatible = "cirrus,lochnagar2-clk", .data = &lochnagar2_conf },
    {}
    };
    MODULE_DEVICE_TABLE(of, lochnagar_of_match);
#[no_mangle]
unsafe extern "C" fn lochnagar_clk_probe(pdev: *mut platform_device) -> c_int {
    static int lochnagar_clk_probe(struct platform_device *pdev)
    {
    struct clk_init_data clk_init = {
    .ops = &lochnagar_clk_ops,
    };
    struct device *dev = &pdev.dev;
    struct lochnagar_clk_priv *priv;
    struct lochnagar_clk *lclk;
    struct lochnagar_config *conf;
    int ret, i;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    conf = (struct lochnagar_config *)device_get_match_data(dev);
    memcpy(priv.lclks, conf.clks, sizeof(priv.lclks));
    clk_init.parent_data = conf.parents;
    clk_init.num_parents = conf.nparents;
    for (i = 0; i < ARRAY_SIZE(priv.lclks); i++) {
    lclk = &priv.lclks[i];
    if (!lclk.name)
    continue;
    clk_init.name = lclk.name;
    lclk.priv = priv;
    lclk.hw.init = &clk_init;
    ret = devm_clk_hw_register(dev, &lclk.hw);
    if (ret) {
    dev_err(dev, "Failed to register %s: %d\n",
    lclk.name, ret);
    return ret;
    }
    }
    ret = devm_of_clk_add_hw_provider(dev, lochnagar_of_clk_hw_get, priv);
    if (ret < 0)
    dev_err(dev, "Failed to register provider: %d\n", ret);
    return ret;
    }
    static struct platform_driver lochnagar_clk_driver = {
    .driver = {
    .name = "lochnagar-clk",
    .of_match_table = lochnagar_of_match,
    },
    .probe = lochnagar_clk_probe,
    };
    module_platform_driver(lochnagar_clk_driver);
    MODULE_AUTHOR("Charles Keepax <ckeepax@opensource.cirrus.com>");
    MODULE_DESCRIPTION("Clock driver for Cirrus Logic Lochnagar Board");
    MODULE_LICENSE("GPL v2");
