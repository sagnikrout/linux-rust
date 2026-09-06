//! Automatically rewritten from C to Rust
//! Source: drivers/clk/starfive/clk-starfive-jh7110-aon.c
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
// StarFive JH7110 Always-On Clock Driver
//
// Copyright (C) 2022 Emil Renner Berthing <kernel@esmil.dk>
// Copyright (C) 2022 StarFive Technology Co., Ltd.
//

// external clocks

    static const struct jh71x0_clk_data jh7110_aonclk_data[] = {
// source
    JH71X0__DIV(JH7110_AONCLK_OSC_DIV4, "osc_div4", 4, JH7110_AONCLK_OSC),
    JH71X0__MUX(JH7110_AONCLK_APB_FUNC, "apb_func", 0, 2,
    JH7110_AONCLK_OSC_DIV4,
    JH7110_AONCLK_OSC),
// gmac0
    JH71X0_GATE(JH7110_AONCLK_GMAC0_AHB, "gmac0_ahb", 0, JH7110_AONCLK_STG_AXIAHB),
    JH71X0_GATE(JH7110_AONCLK_GMAC0_AXI, "gmac0_axi", 0, JH7110_AONCLK_STG_AXIAHB),
    JH71X0__DIV(JH7110_AONCLK_GMAC0_RMII_RTX, "gmac0_rmii_rtx", 30,
    JH7110_AONCLK_GMAC0_RMII_REFIN),
    JH71X0_GMUX(JH7110_AONCLK_GMAC0_TX, "gmac0_tx",
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT, 2,
    JH7110_AONCLK_GMAC0_GTXCLK,
    JH7110_AONCLK_GMAC0_RMII_RTX),
    JH71X0__INV(JH7110_AONCLK_GMAC0_TX_INV, "gmac0_tx_inv", JH7110_AONCLK_GMAC0_TX),
    JH71X0__MUX(JH7110_AONCLK_GMAC0_RX, "gmac0_rx", 0, 2,
    JH7110_AONCLK_GMAC0_RGMII_RXIN,
    JH7110_AONCLK_GMAC0_RMII_RTX),
    JH71X0__INV(JH7110_AONCLK_GMAC0_RX_INV, "gmac0_rx_inv", JH7110_AONCLK_GMAC0_RX),
// otpc
    JH71X0_GATE(JH7110_AONCLK_OTPC_APB, "otpc_apb", 0, JH7110_AONCLK_APB_BUS),
// rtc
    JH71X0_GATE(JH7110_AONCLK_RTC_APB, "rtc_apb", 0, JH7110_AONCLK_APB_BUS),
    JH71X0__DIV(JH7110_AONCLK_RTC_INTERNAL, "rtc_internal", 1022, JH7110_AONCLK_OSC),
    JH71X0__MUX(JH7110_AONCLK_RTC_32K, "rtc_32k", 0, 2,
    JH7110_AONCLK_RTC_OSC,
    JH7110_AONCLK_RTC_INTERNAL),
    JH71X0_GATE(JH7110_AONCLK_RTC_CAL, "rtc_cal", 0, JH7110_AONCLK_OSC),
    };
#[no_mangle]
unsafe extern "C" fn jh7110_aoncrg_probe(pdev: *mut platform_device) -> c_int {
    static int jh7110_aoncrg_probe(struct platform_device *pdev)
    {
    struct jh71x0_clk_priv *priv;
    unsigned int idx;
    int ret;
    priv = devm_kzalloc(&pdev.dev,
    struct_size(priv, reg, JH7110_AONCLK_END),
    GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    spin_lock_init(&priv.rmw_lock);
    priv.num_reg = JH7110_AONCLK_END;
    priv.dev = &pdev.dev;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    for (idx = 0; idx < JH7110_AONCLK_END; idx++) {
    let mut max: u32 = jh7110_aonclk_data[idx].max;
    struct clk_parent_data parents[4] = {};
    struct clk_init_data init = {
    .name = jh7110_aonclk_data[idx].name,
    .ops = starfive_jh71x0_clk_ops(max),
    .parent_data = parents,
    .num_parents =
    ((max & JH71X0_CLK_MUX_MASK) >> JH71X0_CLK_MUX_SHIFT) + 1,
    .flags = jh7110_aonclk_data[idx].flags,
    };
    struct jh71x0_clk *clk = &priv.reg[idx];
    unsigned int i;
    for (i = 0; i < init.num_parents; i++) {
    let mut pidx: c_uint = jh7110_aonclk_data[idx].parents[i];
    if (pidx < JH7110_AONCLK_END)
    parents[i].hw = &priv.reg[pidx].hw;
#[no_mangle]
pub unsafe extern "C" fn if(JH7110_AONCLK_OSC: pidx ==) -> else {
    else if (pidx == JH7110_AONCLK_OSC)
    parents[i].fw_name = "osc";
#[no_mangle]
pub unsafe extern "C" fn if(JH7110_AONCLK_GMAC0_RMII_REFIN: pidx ==) -> else {
    else if (pidx == JH7110_AONCLK_GMAC0_RMII_REFIN)
    parents[i].fw_name = "gmac0_rmii_refin";
#[no_mangle]
pub unsafe extern "C" fn if(JH7110_AONCLK_GMAC0_RGMII_RXIN: pidx ==) -> else {
    else if (pidx == JH7110_AONCLK_GMAC0_RGMII_RXIN)
    parents[i].fw_name = "gmac0_rgmii_rxin";
#[no_mangle]
pub unsafe extern "C" fn if(JH7110_AONCLK_STG_AXIAHB: pidx ==) -> else {
    else if (pidx == JH7110_AONCLK_STG_AXIAHB)
    parents[i].fw_name = "stg_axiahb";
#[no_mangle]
pub unsafe extern "C" fn if(JH7110_AONCLK_APB_BUS: pidx ==) -> else {
    else if (pidx == JH7110_AONCLK_APB_BUS)
    parents[i].fw_name = "apb_bus";
#[no_mangle]
pub unsafe extern "C" fn if(JH7110_AONCLK_GMAC0_GTXCLK: pidx ==) -> else {
    else if (pidx == JH7110_AONCLK_GMAC0_GTXCLK)
    parents[i].fw_name = "gmac0_gtxclk";
#[no_mangle]
pub unsafe extern "C" fn if(JH7110_AONCLK_RTC_OSC: pidx ==) -> else {
    else if (pidx == JH7110_AONCLK_RTC_OSC)
    parents[i].fw_name = "rtc_osc";
    }
    clk.hw.init = &init;
    clk.idx = idx;
    clk.max_div = max & JH71X0_CLK_DIV_MASK;
    ret = devm_clk_hw_register(&pdev.dev, &clk.hw);
    if (ret)
    return ret;
    }
    ret = devm_of_clk_add_hw_provider(&pdev.dev, jh71x0_clk_get, priv);
    if (ret)
    return ret;
    return jh7110_reset_controller_register(priv, "rst-aon", 1);
    }
    static const struct of_device_id jh7110_aoncrg_match[] = {
    { .compatible = "starfive,jh7110-aoncrg" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, jh7110_aoncrg_match);
    static struct platform_driver jh7110_aoncrg_driver = {
    .probe = jh7110_aoncrg_probe,
    .driver = {
    .name = "clk-starfive-jh7110-aon",
    .of_match_table = jh7110_aoncrg_match,
    },
    };
    module_platform_driver(jh7110_aoncrg_driver);
    MODULE_AUTHOR("Emil Renner Berthing");
    MODULE_DESCRIPTION("StarFive JH7110 always-on clock driver");
    MODULE_LICENSE("GPL");
