//! Automatically rewritten from C to Rust
//! Source: drivers/clk/starfive/clk-starfive-jh7110-vout.c
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
// StarFive JH7110 Video-Output Clock Driver
//
// Copyright (C) 2022-2023 StarFive Technology Co., Ltd.
//

// external clocks

    static struct clk_bulk_data jh7110_vout_top_clks[] = {
    { .id = "vout_src" },
    { .id = "vout_top_ahb" }
    };
    static const struct jh71x0_clk_data jh7110_voutclk_data[] = {
// divider
    JH71X0__DIV(JH7110_VOUTCLK_APB, "apb", 8, JH7110_VOUTCLK_VOUT_TOP_AHB),
    JH71X0__DIV(JH7110_VOUTCLK_DC8200_PIX, "dc8200_pix", 63, JH7110_VOUTCLK_VOUT_SRC),
    JH71X0__DIV(JH7110_VOUTCLK_DSI_SYS, "dsi_sys", 31, JH7110_VOUTCLK_VOUT_SRC),
    JH71X0__DIV(JH7110_VOUTCLK_TX_ESC, "tx_esc", 31, JH7110_VOUTCLK_VOUT_TOP_AHB),
// dc8200
    JH71X0_GATE(JH7110_VOUTCLK_DC8200_AXI, "dc8200_axi", 0, JH7110_VOUTCLK_VOUT_TOP_AXI),
    JH71X0_GATE(JH7110_VOUTCLK_DC8200_CORE, "dc8200_core", 0, JH7110_VOUTCLK_VOUT_TOP_AXI),
    JH71X0_GATE(JH7110_VOUTCLK_DC8200_AHB, "dc8200_ahb", 0, JH7110_VOUTCLK_VOUT_TOP_AHB),
    JH71X0_GMUX(JH7110_VOUTCLK_DC8200_PIX0, "dc8200_pix0", 0, 2,
    JH7110_VOUTCLK_DC8200_PIX,
    JH7110_VOUTCLK_HDMITX0_PIXELCLK),
    JH71X0_GMUX(JH7110_VOUTCLK_DC8200_PIX1, "dc8200_pix1", 0, 2,
    JH7110_VOUTCLK_DC8200_PIX,
    JH7110_VOUTCLK_HDMITX0_PIXELCLK),
// LCD
    JH71X0_GMUX(JH7110_VOUTCLK_DOM_VOUT_TOP_LCD, "dom_vout_top_lcd", 0, 2,
    JH7110_VOUTCLK_DC8200_PIX0,
    JH7110_VOUTCLK_DC8200_PIX1),
// dsiTx
    JH71X0_GATE(JH7110_VOUTCLK_DSITX_APB, "dsiTx_apb", 0, JH7110_VOUTCLK_DSI_SYS),
    JH71X0_GATE(JH7110_VOUTCLK_DSITX_SYS, "dsiTx_sys", 0, JH7110_VOUTCLK_DSI_SYS),
    JH71X0_GMUX(JH7110_VOUTCLK_DSITX_DPI, "dsiTx_dpi", 0, 2,
    JH7110_VOUTCLK_DC8200_PIX,
    JH7110_VOUTCLK_HDMITX0_PIXELCLK),
    JH71X0_GATE(JH7110_VOUTCLK_DSITX_TXESC, "dsiTx_txesc", 0, JH7110_VOUTCLK_TX_ESC),
// mipitx DPHY
    JH71X0_GATE(JH7110_VOUTCLK_MIPITX_DPHY_TXESC, "mipitx_dphy_txesc", 0,
    JH7110_VOUTCLK_TX_ESC),
// hdmi
    JH71X0_GATE(JH7110_VOUTCLK_HDMI_TX_MCLK, "hdmi_tx_mclk", 0,
    JH7110_VOUTCLK_VOUT_TOP_HDMITX0_MCLK),
    JH71X0_GATE(JH7110_VOUTCLK_HDMI_TX_BCLK, "hdmi_tx_bclk", 0,
    JH7110_VOUTCLK_I2STX0_BCLK),
    JH71X0_GATE(JH7110_VOUTCLK_HDMI_TX_SYS, "hdmi_tx_sys", 0, JH7110_VOUTCLK_APB),
    };
#[no_mangle]
unsafe extern "C" fn jh7110_vout_top_rst_init(priv: *mut jh71x0_clk_priv) -> c_int {
    static int jh7110_vout_top_rst_init(struct jh71x0_clk_priv *priv)
    {
    struct reset_control *top_rst;
// The reset should be shared and other Vout modules will use its.
    top_rst = devm_reset_control_get_shared(priv.dev, core::ptr::null_mut());
    if (IS_ERR(top_rst))
    return dev_err_probe(priv.dev, PTR_ERR(top_rst), "failed to get top reset\n");
    return reset_control_deassert(top_rst);
    }

#[no_mangle]
unsafe extern "C" fn jh7110_voutcrg_suspend(dev: *mut device) -> c_int {
    static int jh7110_voutcrg_suspend(struct device *dev)
    {
    struct jh7110_top_sysclk *top = dev_get_drvdata(dev);
    clk_bulk_disable_unprepare(top.top_clks_num, top.top_clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jh7110_voutcrg_resume(dev: *mut device) -> c_int {
    static int jh7110_voutcrg_resume(struct device *dev)
    {
    struct jh7110_top_sysclk *top = dev_get_drvdata(dev);
    return clk_bulk_prepare_enable(top.top_clks_num, top.top_clks);
    }
    static const struct dev_pm_ops jh7110_voutcrg_pm_ops = {
    RUNTIME_PM_OPS(jh7110_voutcrg_suspend, jh7110_voutcrg_resume, core::ptr::null_mut())
    };

#[no_mangle]
unsafe extern "C" fn jh7110_voutcrg_probe(pdev: *mut platform_device) -> c_int {
    static int jh7110_voutcrg_probe(struct platform_device *pdev)
    {
    struct jh71x0_clk_priv *priv;
    struct jh7110_top_sysclk *top;
    unsigned int idx;
    int ret;
    priv = devm_kzalloc(&pdev.dev,
    struct_size(priv, reg, JH7110_VOUTCLK_END),
    GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    top = devm_kzalloc(&pdev.dev, sizeof(*top), GFP_KERNEL);
    if (!top)
    return -ENOMEM;
    spin_lock_init(&priv.rmw_lock);
    priv.num_reg = JH7110_VOUTCLK_END;
    priv.dev = &pdev.dev;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    top.top_clks = jh7110_vout_top_clks;
    top.top_clks_num = ARRAY_SIZE(jh7110_vout_top_clks);
    ret = devm_clk_bulk_get(priv.dev, top.top_clks_num, top.top_clks);
    if (ret)
    return dev_err_probe(priv.dev, ret, "failed to get top clocks\n");
    dev_set_drvdata(priv.dev, top);
// enable power domain and clocks
    pm_runtime_enable(priv.dev);
    ret = pm_runtime_resume_and_get(priv.dev);
    if (ret < 0)
    return dev_err_probe(priv.dev, ret, "failed to turn on power\n");
    ret = jh7110_vout_top_rst_init(priv);
    if (ret)
    goto err_exit;
    for (idx = 0; idx < JH7110_VOUTCLK_END; idx++) {
    let mut max: u32 = jh7110_voutclk_data[idx].max;
    struct clk_parent_data parents[4] = {};
    struct clk_init_data init = {
    .name = jh7110_voutclk_data[idx].name,
    .ops = starfive_jh71x0_clk_ops(max),
    .parent_data = parents,
    .num_parents =
    ((max & JH71X0_CLK_MUX_MASK) >> JH71X0_CLK_MUX_SHIFT) + 1,
    .flags = jh7110_voutclk_data[idx].flags,
    };
    struct jh71x0_clk *clk = &priv.reg[idx];
    unsigned int i;
    const char *fw_name[JH7110_VOUTCLK_EXT_END - JH7110_VOUTCLK_END] = {
    "vout_src",
    "vout_top_ahb",
    "vout_top_axi",
    "vout_top_hdmitx0_mclk",
    "i2stx0_bclk",
    "hdmitx0_pixelclk"
    };
    for (i = 0; i < init.num_parents; i++) {
    let mut pidx: c_uint = jh7110_voutclk_data[idx].parents[i];
    if (pidx < JH7110_VOUTCLK_END)
    parents[i].hw = &priv.reg[pidx].hw;
#[no_mangle]
pub unsafe extern "C" fn if(JH7110_VOUTCLK_EXT_END: pidx <) -> else {
    else if (pidx < JH7110_VOUTCLK_EXT_END)
    parents[i].fw_name = fw_name[pidx - JH7110_VOUTCLK_END];
    }
    clk.hw.init = &init;
    clk.idx = idx;
    clk.max_div = max & JH71X0_CLK_DIV_MASK;
    ret = devm_clk_hw_register(&pdev.dev, &clk.hw);
    if (ret)
    goto err_exit;
    }
    ret = devm_of_clk_add_hw_provider(&pdev.dev, jh71x0_clk_get, priv);
    if (ret)
    goto err_exit;
    ret = jh7110_reset_controller_register(priv, "rst-vo", 4);
    if (ret)
    goto err_exit;
    return 0;
    err_exit:
    pm_runtime_put_sync(priv.dev);
    pm_runtime_disable(priv.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn jh7110_voutcrg_remove(pdev: *mut platform_device) {
    static void jh7110_voutcrg_remove(struct platform_device *pdev)
    {
    pm_runtime_put_sync(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }
    static const struct of_device_id jh7110_voutcrg_match[] = {
    { .compatible = "starfive,jh7110-voutcrg" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, jh7110_voutcrg_match);
    static struct platform_driver jh7110_voutcrg_driver = {
    .probe = jh7110_voutcrg_probe,
    .remove = jh7110_voutcrg_remove,
    .driver = {
    .name = "clk-starfive-jh7110-vout",
    .of_match_table = jh7110_voutcrg_match,
    .pm = pm_ptr(&jh7110_voutcrg_pm_ops),
    },
    };
    module_platform_driver(jh7110_voutcrg_driver);
    MODULE_AUTHOR("Xingyu Wu <xingyu.wu@starfivetech.com>");
    MODULE_DESCRIPTION("StarFive JH7110 Video-Output clock driver");
    MODULE_LICENSE("GPL");
