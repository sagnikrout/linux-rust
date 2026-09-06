//! Automatically rewritten from C to Rust
//! Source: drivers/clk/uniphier/clk-uniphier-core.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2016 Socionext Inc.
// Author: Masahiro Yamada <yamada.masahiro@socionext.com>
//

    static struct clk_hw *uniphier_clk_register(struct device *dev,
    struct regmap *regmap,
    const struct uniphier_clk_data *data)
    {
    switch (data.type) {
    case UNIPHIER_CLK_TYPE_CPUGEAR:
    return uniphier_clk_register_cpugear(dev, regmap, data.name,
    &data.data.cpugear);
    case UNIPHIER_CLK_TYPE_FIXED_FACTOR:
    return uniphier_clk_register_fixed_factor(dev, data.name,
    &data.data.factor);
    case UNIPHIER_CLK_TYPE_FIXED_RATE:
    return uniphier_clk_register_fixed_rate(dev, data.name,
    &data.data.rate);
    case UNIPHIER_CLK_TYPE_GATE:
    return uniphier_clk_register_gate(dev, regmap, data.name,
    &data.data.gate);
    case UNIPHIER_CLK_TYPE_MUX:
    return uniphier_clk_register_mux(dev, regmap, data.name,
    &data.data.mux);
    default:
    dev_err(dev, "unsupported clock type\n");
    return ERR_PTR(-EINVAL);
    }
    }
#[no_mangle]
unsafe extern "C" fn uniphier_clk_probe(pdev: *mut platform_device) -> c_int {
    static int uniphier_clk_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct clk_hw_onecell_data *hw_data;
    const struct uniphier_clk_data *p, *data;
    struct regmap *regmap;
    struct device_node *parent;
    let mut clk_num: c_int = 0;
    data = of_device_get_match_data(dev);
    if (WARN_ON(!data))
    return -EINVAL;
    parent = of_get_parent(dev.of_node); /* parent should be syscon node */
    regmap = syscon_node_to_regmap(parent);
    of_node_put(parent);
    if (IS_ERR(regmap)) {
    dev_err(dev, "failed to get regmap (error %ld)\n",
    PTR_ERR(regmap));
    return PTR_ERR(regmap);
    }
    for (p = data; p.name; p++)
    clk_num = max(clk_num, p.idx + 1);
    hw_data = devm_kzalloc(dev, struct_size(hw_data, hws, clk_num),
    GFP_KERNEL);
    if (!hw_data)
    return -ENOMEM;
    hw_data.num = clk_num;
// avoid returning NULL for unused idx
    while (--clk_num >= 0)
    hw_data.hws[clk_num] = ERR_PTR(-EINVAL);
    for (p = data; p.name; p++) {
    struct clk_hw *hw;
    dev_dbg(dev, "register %s (index=%d)\n", p.name, p.idx);
    hw = uniphier_clk_register(dev, regmap, p);
    if (WARN(IS_ERR(hw), "failed to register %s", p.name))
    continue;
    if (p.idx >= 0)
    hw_data.hws[p.idx] = hw;
    }
    return devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get,
    hw_data);
    }
    static const struct of_device_id uniphier_clk_match[] = {
// System clock
    {
    .compatible = "socionext,uniphier-ld4-clock",
    .data = uniphier_ld4_sys_clk_data,
    },
    {
    .compatible = "socionext,uniphier-pro4-clock",
    .data = uniphier_pro4_sys_clk_data,
    },
    {
    .compatible = "socionext,uniphier-sld8-clock",
    .data = uniphier_sld8_sys_clk_data,
    },
    {
    .compatible = "socionext,uniphier-pro5-clock",
    .data = uniphier_pro5_sys_clk_data,
    },
    {
    .compatible = "socionext,uniphier-pxs2-clock",
    .data = uniphier_pxs2_sys_clk_data,
    },
    {
    .compatible = "socionext,uniphier-ld11-clock",
    .data = uniphier_ld11_sys_clk_data,
    },
    {
    .compatible = "socionext,uniphier-ld20-clock",
    .data = uniphier_ld20_sys_clk_data,
    },
    {
    .compatible = "socionext,uniphier-pxs3-clock",
    .data = uniphier_pxs3_sys_clk_data,
    },
    {
    .compatible = "socionext,uniphier-nx1-clock",
    .data = uniphier_nx1_sys_clk_data,
    },
// Media I/O clock, SD clock
    {
    .compatible = "socionext,uniphier-ld4-mio-clock",
    .data = uniphier_ld4_mio_clk_data,
    },
    {
    .compatible = "socionext,uniphier-pro4-mio-clock",
    .data = uniphier_ld4_mio_clk_data,
    },
    {
    .compatible = "socionext,uniphier-sld8-mio-clock",
    .data = uniphier_ld4_mio_clk_data,
    },
    {
    .compatible = "socionext,uniphier-pro5-sd-clock",
    .data = uniphier_pro5_sd_clk_data,
    },
    {
    .compatible = "socionext,uniphier-pxs2-sd-clock",
    .data = uniphier_pro5_sd_clk_data,
    },
    {
    .compatible = "socionext,uniphier-ld11-mio-clock",
    .data = uniphier_ld4_mio_clk_data,
    },
    {
    .compatible = "socionext,uniphier-ld20-sd-clock",
    .data = uniphier_pro5_sd_clk_data,
    },
    {
    .compatible = "socionext,uniphier-pxs3-sd-clock",
    .data = uniphier_pro5_sd_clk_data,
    },
    {
    .compatible = "socionext,uniphier-nx1-sd-clock",
    .data = uniphier_pro5_sd_clk_data,
    },
// Peripheral clock
    {
    .compatible = "socionext,uniphier-ld4-peri-clock",
    .data = uniphier_ld4_peri_clk_data,
    },
    {
    .compatible = "socionext,uniphier-pro4-peri-clock",
    .data = uniphier_pro4_peri_clk_data,
    },
    {
    .compatible = "socionext,uniphier-sld8-peri-clock",
    .data = uniphier_ld4_peri_clk_data,
    },
    {
    .compatible = "socionext,uniphier-pro5-peri-clock",
    .data = uniphier_pro4_peri_clk_data,
    },
    {
    .compatible = "socionext,uniphier-pxs2-peri-clock",
    .data = uniphier_pro4_peri_clk_data,
    },
    {
    .compatible = "socionext,uniphier-ld11-peri-clock",
    .data = uniphier_pro4_peri_clk_data,
    },
    {
    .compatible = "socionext,uniphier-ld20-peri-clock",
    .data = uniphier_pro4_peri_clk_data,
    },
    {
    .compatible = "socionext,uniphier-pxs3-peri-clock",
    .data = uniphier_pro4_peri_clk_data,
    },
    {
    .compatible = "socionext,uniphier-nx1-peri-clock",
    .data = uniphier_pro4_peri_clk_data,
    },
// SoC-glue clock
    {
    .compatible = "socionext,uniphier-pro4-sg-clock",
    .data = uniphier_pro4_sg_clk_data,
    },
    { /* sentinel */ }
    };
    static struct platform_driver uniphier_clk_driver = {
    .probe = uniphier_clk_probe,
    .driver = {
    .name = "uniphier-clk",
    .of_match_table = uniphier_clk_match,
    },
    };
    builtin_platform_driver(uniphier_clk_driver);
