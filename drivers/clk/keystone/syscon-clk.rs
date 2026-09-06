//! Automatically rewritten from C to Rust
//! Source: drivers/clk/keystone/syscon-clk.c
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
// Copyright (C) 2020 Texas Instruments Incorporated - https://www.ti.com
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_syscon_gate_clk_priv {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub reg: u32,
    pub idx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_syscon_gate_clk_data {
    pub name: *mut c_char,
    pub offset: u32,
    pub bit_idx: u32,
}

    static struct
    ti_syscon_gate_clk_priv *to_ti_syscon_gate_clk_priv(struct clk_hw *hw)
    {
    return container_of(hw, struct ti_syscon_gate_clk_priv, hw);
    }
#[no_mangle]
unsafe extern "C" fn ti_syscon_gate_clk_enable(hw: *mut clk_hw) -> c_int {
    static int ti_syscon_gate_clk_enable(struct clk_hw *hw)
    {
    struct ti_syscon_gate_clk_priv *priv = to_ti_syscon_gate_clk_priv(hw);
    return regmap_write_bits(priv.regmap, priv.reg, priv.idx,
    priv.idx);
    }
#[no_mangle]
unsafe extern "C" fn ti_syscon_gate_clk_disable(hw: *mut clk_hw) {
    static void ti_syscon_gate_clk_disable(struct clk_hw *hw)
    {
    struct ti_syscon_gate_clk_priv *priv = to_ti_syscon_gate_clk_priv(hw);
    regmap_write_bits(priv.regmap, priv.reg, priv.idx, 0);
    }
#[no_mangle]
unsafe extern "C" fn ti_syscon_gate_clk_is_enabled(hw: *mut clk_hw) -> c_int {
    static int ti_syscon_gate_clk_is_enabled(struct clk_hw *hw)
    {
    unsigned int val;
    struct ti_syscon_gate_clk_priv *priv = to_ti_syscon_gate_clk_priv(hw);
    regmap_read(priv.regmap, priv.reg, &val);
    return !!(val & priv.idx);
    }
    static const struct clk_ops ti_syscon_gate_clk_ops = {
    .enable		= ti_syscon_gate_clk_enable,
    .disable	= ti_syscon_gate_clk_disable,
    .is_enabled	= ti_syscon_gate_clk_is_enabled,
    };
    static struct clk_hw
// ti_syscon_gate_clk_register(struct device *dev, struct regmap *regmap,
    const char *parent_name,
    const struct ti_syscon_gate_clk_data *data)
    {
    struct ti_syscon_gate_clk_priv *priv;
    struct clk_init_data init;
    char *name = core::ptr::null_mut();
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return ERR_PTR(-ENOMEM);
    init.ops = &ti_syscon_gate_clk_ops;
    if (parent_name) {
    name = kasprintf(GFP_KERNEL, "%s:%s", data.name, parent_name);
    init.name = name;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    init.flags = CLK_SET_RATE_PARENT;
    } else {
    init.name = data.name;
    init.parent_names = core::ptr::null_mut();
    init.num_parents = 0;
    init.flags = 0;
    }
    priv.regmap = regmap;
    priv.reg = data.offset;
    priv.idx = BIT(data.bit_idx);
    priv.hw.init = &init;
    ret = devm_clk_hw_register(dev, &priv.hw);
    if (name)
    kfree(init.name);
    if (ret)
    return ERR_PTR(ret);
    return &priv.hw;
    }
    static const struct regmap_config ti_syscon_regmap_cfg = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    };
#[no_mangle]
unsafe extern "C" fn ti_syscon_gate_clk_probe(pdev: *mut platform_device) -> c_int {
    static int ti_syscon_gate_clk_probe(struct platform_device *pdev)
    {
    const struct ti_syscon_gate_clk_data *data, *p;
    struct clk_hw_onecell_data *hw_data;
    struct device *dev = &pdev.dev;
    int num_clks, num_parents, i;
    const char *parent_name;
    struct regmap *regmap;
    void __iomem *base;
    data = device_get_match_data(dev);
    if (!data)
    return -EINVAL;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    regmap = devm_regmap_init_mmio(dev, base, &ti_syscon_regmap_cfg);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap),
    "failed to get regmap\n");
    num_clks = 0;
    for (p = data; p.name; p++)
    num_clks++;
    num_parents = of_clk_get_parent_count(dev.of_node);
    if (of_device_is_compatible(dev.of_node, "ti,am62-audio-refclk") &&
    num_parents == 0) {
    return dev_err_probe(dev, -EINVAL,
    "must specify a parent clock\n");
    }
    hw_data = devm_kzalloc(dev, struct_size(hw_data, hws, num_clks),
    GFP_KERNEL);
    if (!hw_data)
    return -ENOMEM;
    hw_data.num = num_clks;
    parent_name = of_clk_get_parent_name(dev.of_node, 0);
    for (i = 0; i < num_clks; i++) {
    hw_data.hws[i] = ti_syscon_gate_clk_register(dev, regmap,
    parent_name,
    &data[i]);
    if (IS_ERR(hw_data.hws[i]))
    dev_warn(dev, "failed to register %s\n",
    data[i].name);
    }
    if (num_clks == 1)
    return devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get,
    hw_data.hws[0]);
    return devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get, hw_data);
    }

    {						\
    .name = _name,				\
    .offset = (_offset),			\
    .bit_idx = (_bit_idx),			\
    }
    static const struct ti_syscon_gate_clk_data am654_clk_data[] = {
    TI_SYSCON_CLK_GATE("ehrpwm_tbclk0", 0x0, 0),
    TI_SYSCON_CLK_GATE("ehrpwm_tbclk1", 0x4, 0),
    TI_SYSCON_CLK_GATE("ehrpwm_tbclk2", 0x8, 0),
    TI_SYSCON_CLK_GATE("ehrpwm_tbclk3", 0xc, 0),
    TI_SYSCON_CLK_GATE("ehrpwm_tbclk4", 0x10, 0),
    TI_SYSCON_CLK_GATE("ehrpwm_tbclk5", 0x14, 0),
    { /* Sentinel */ },
    };
    static const struct ti_syscon_gate_clk_data am64_clk_data[] = {
    TI_SYSCON_CLK_GATE("epwm_tbclk0", 0x0, 0),
    TI_SYSCON_CLK_GATE("epwm_tbclk1", 0x0, 1),
    TI_SYSCON_CLK_GATE("epwm_tbclk2", 0x0, 2),
    TI_SYSCON_CLK_GATE("epwm_tbclk3", 0x0, 3),
    TI_SYSCON_CLK_GATE("epwm_tbclk4", 0x0, 4),
    TI_SYSCON_CLK_GATE("epwm_tbclk5", 0x0, 5),
    TI_SYSCON_CLK_GATE("epwm_tbclk6", 0x0, 6),
    TI_SYSCON_CLK_GATE("epwm_tbclk7", 0x0, 7),
    TI_SYSCON_CLK_GATE("epwm_tbclk8", 0x0, 8),
    { /* Sentinel */ },
    };
    static const struct ti_syscon_gate_clk_data am62_clk_data[] = {
    TI_SYSCON_CLK_GATE("epwm_tbclk0", 0x0, 0),
    TI_SYSCON_CLK_GATE("epwm_tbclk1", 0x0, 1),
    TI_SYSCON_CLK_GATE("epwm_tbclk2", 0x0, 2),
    { /* Sentinel */ },
    };
    static const struct ti_syscon_gate_clk_data am62_audio_clk_data[] = {
    TI_SYSCON_CLK_GATE("audio_refclk", 0x0, 15),
    { /* Sentinel */ },
    };
    static const struct of_device_id ti_syscon_gate_clk_ids[] = {
    {
    .compatible = "ti,am654-ehrpwm-tbclk",
    .data = &am654_clk_data,
    },
    {
    .compatible = "ti,am64-epwm-tbclk",
    .data = &am64_clk_data,
    },
    {
    .compatible = "ti,am62-epwm-tbclk",
    .data = &am62_clk_data,
    },
    {
    .compatible = "ti,am62-audio-refclk",
    .data = &am62_audio_clk_data,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, ti_syscon_gate_clk_ids);
    static struct platform_driver ti_syscon_gate_clk_driver = {
    .probe = ti_syscon_gate_clk_probe,
    .driver = {
    .name = "ti-syscon-gate-clk",
    .of_match_table = ti_syscon_gate_clk_ids,
    },
    };
    module_platform_driver(ti_syscon_gate_clk_driver);
    MODULE_AUTHOR("Vignesh Raghavendra <vigneshr@ti.com>");
    MODULE_DESCRIPTION("Syscon backed gate-clock driver");
    MODULE_LICENSE("GPL");
