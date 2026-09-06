//! Automatically rewritten from C to Rust
//! Source: drivers/clk/hisilicon/clk-hix5hd2.c
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
// Copyright (c) 2014 Linaro Ltd.
// Copyright (c) 2014 Hisilicon Limited.
//

    static struct hisi_fixed_rate_clock hix5hd2_fixed_rate_clks[] __initdata = {
    { HIX5HD2_FIXED_1200M, "1200m", core::ptr::null_mut(), 0, 1200000000, },
    { HIX5HD2_FIXED_400M, "400m", core::ptr::null_mut(), 0, 400000000, },
    { HIX5HD2_FIXED_48M, "48m", core::ptr::null_mut(), 0, 48000000, },
    { HIX5HD2_FIXED_24M, "24m", core::ptr::null_mut(), 0, 24000000, },
    { HIX5HD2_FIXED_600M, "600m", core::ptr::null_mut(), 0, 600000000, },
    { HIX5HD2_FIXED_300M, "300m", core::ptr::null_mut(), 0, 300000000, },
    { HIX5HD2_FIXED_75M, "75m", core::ptr::null_mut(), 0, 75000000, },
    { HIX5HD2_FIXED_200M, "200m", core::ptr::null_mut(), 0, 200000000, },
    { HIX5HD2_FIXED_100M, "100m", core::ptr::null_mut(), 0, 100000000, },
    { HIX5HD2_FIXED_40M, "40m", core::ptr::null_mut(), 0, 40000000, },
    { HIX5HD2_FIXED_150M, "150m", core::ptr::null_mut(), 0, 150000000, },
    { HIX5HD2_FIXED_1728M, "1728m", core::ptr::null_mut(), 0, 1728000000, },
    { HIX5HD2_FIXED_28P8M, "28p8m", core::ptr::null_mut(), 0, 28000000, },
    { HIX5HD2_FIXED_432M, "432m", core::ptr::null_mut(), 0, 432000000, },
    { HIX5HD2_FIXED_345P6M, "345p6m", core::ptr::null_mut(), 0, 345000000, },
    { HIX5HD2_FIXED_288M, "288m", core::ptr::null_mut(), 0, 288000000, },
    { HIX5HD2_FIXED_60M,	"60m", core::ptr::null_mut(), 0, 60000000, },
    { HIX5HD2_FIXED_750M, "750m", core::ptr::null_mut(), 0, 750000000, },
    { HIX5HD2_FIXED_500M, "500m", core::ptr::null_mut(), 0, 500000000, },
    { HIX5HD2_FIXED_54M,	"54m", core::ptr::null_mut(), 0, 54000000, },
    { HIX5HD2_FIXED_27M, "27m", core::ptr::null_mut(), 0, 27000000, },
    { HIX5HD2_FIXED_1500M, "1500m", core::ptr::null_mut(), 0, 1500000000, },
    { HIX5HD2_FIXED_375M, "375m", core::ptr::null_mut(), 0, 375000000, },
    { HIX5HD2_FIXED_187M, "187m", core::ptr::null_mut(), 0, 187000000, },
    { HIX5HD2_FIXED_250M, "250m", core::ptr::null_mut(), 0, 250000000, },
    { HIX5HD2_FIXED_125M, "125m", core::ptr::null_mut(), 0, 125000000, },
    { HIX5HD2_FIXED_2P02M, "2m", core::ptr::null_mut(), 0, 2000000, },
    { HIX5HD2_FIXED_50M, "50m", core::ptr::null_mut(), 0, 50000000, },
    { HIX5HD2_FIXED_25M, "25m", core::ptr::null_mut(), 0, 25000000, },
    { HIX5HD2_FIXED_83M, "83m", core::ptr::null_mut(), 0, 83333333, },
    };
    static const char *const sfc_mux_p[] __initconst = {
    "24m", "150m", "200m", "100m", "75m", };
    static u32 sfc_mux_table[] = {0, 4, 5, 6, 7};
    static const char *const sdio_mux_p[] __initconst = {
    "75m", "100m", "50m", "15m", };
    static u32 sdio_mux_table[] = {0, 1, 2, 3};
    static const char *const fephy_mux_p[] __initconst = { "25m", "125m"};
    static u32 fephy_mux_table[] = {0, 1};
    static struct hisi_mux_clock hix5hd2_mux_clks[] __initdata = {
    { HIX5HD2_SFC_MUX, "sfc_mux", sfc_mux_p, ARRAY_SIZE(sfc_mux_p),
    CLK_SET_RATE_PARENT, 0x5c, 8, 3, 0, sfc_mux_table, },
    { HIX5HD2_MMC_MUX, "mmc_mux", sdio_mux_p, ARRAY_SIZE(sdio_mux_p),
    CLK_SET_RATE_PARENT, 0xa0, 8, 2, 0, sdio_mux_table, },
    { HIX5HD2_SD_MUX, "sd_mux", sdio_mux_p, ARRAY_SIZE(sdio_mux_p),
    CLK_SET_RATE_PARENT, 0x9c, 8, 2, 0, sdio_mux_table, },
    { HIX5HD2_FEPHY_MUX, "fephy_mux",
    fephy_mux_p, ARRAY_SIZE(fephy_mux_p),
    CLK_SET_RATE_PARENT, 0x120, 8, 2, 0, fephy_mux_table, },
    };
    static struct hisi_gate_clock hix5hd2_gate_clks[] __initdata = {
// sfc
    { HIX5HD2_SFC_CLK, "clk_sfc", "sfc_mux",
    CLK_SET_RATE_PARENT, 0x5c, 0, 0, },
    { HIX5HD2_SFC_RST, "rst_sfc", "clk_sfc",
    CLK_SET_RATE_PARENT, 0x5c, 4, CLK_GATE_SET_TO_DISABLE, },
// sdio0
    { HIX5HD2_SD_BIU_CLK, "clk_sd_biu", "200m",
    CLK_SET_RATE_PARENT, 0x9c, 0, 0, },
    { HIX5HD2_SD_CIU_CLK, "clk_sd_ciu", "sd_mux",
    CLK_SET_RATE_PARENT, 0x9c, 1, 0, },
    { HIX5HD2_SD_CIU_RST, "rst_sd_ciu", "clk_sd_ciu",
    CLK_SET_RATE_PARENT, 0x9c, 4, CLK_GATE_SET_TO_DISABLE, },
// sdio1
    { HIX5HD2_MMC_BIU_CLK, "clk_mmc_biu", "200m",
    CLK_SET_RATE_PARENT, 0xa0, 0, 0, },
    { HIX5HD2_MMC_CIU_CLK, "clk_mmc_ciu", "mmc_mux",
    CLK_SET_RATE_PARENT, 0xa0, 1, 0, },
    { HIX5HD2_MMC_CIU_RST, "rst_mmc_ciu", "clk_mmc_ciu",
    CLK_SET_RATE_PARENT, 0xa0, 4, CLK_GATE_SET_TO_DISABLE, },
// gsf
    { HIX5HD2_FWD_BUS_CLK, "clk_fwd_bus", core::ptr::null_mut(), 0, 0xcc, 0, 0, },
    { HIX5HD2_FWD_SYS_CLK, "clk_fwd_sys", "clk_fwd_bus", 0, 0xcc, 5, 0, },
    { HIX5HD2_MAC0_PHY_CLK, "clk_fephy", "clk_fwd_sys",
    CLK_SET_RATE_PARENT, 0x120, 0, 0, },
// wdg0
    { HIX5HD2_WDG0_CLK, "clk_wdg0", "24m",
    CLK_SET_RATE_PARENT, 0x178, 0, 0, },
    { HIX5HD2_WDG0_RST, "rst_wdg0", "clk_wdg0",
    CLK_SET_RATE_PARENT, 0x178, 4, CLK_GATE_SET_TO_DISABLE, },
// I2C
    {HIX5HD2_I2C0_CLK, "clk_i2c0", "100m",
    CLK_SET_RATE_PARENT, 0x06c, 4, 0, },
    {HIX5HD2_I2C0_RST, "rst_i2c0", "clk_i2c0",
    CLK_SET_RATE_PARENT, 0x06c, 5, CLK_GATE_SET_TO_DISABLE, },
    {HIX5HD2_I2C1_CLK, "clk_i2c1", "100m",
    CLK_SET_RATE_PARENT, 0x06c, 8, 0, },
    {HIX5HD2_I2C1_RST, "rst_i2c1", "clk_i2c1",
    CLK_SET_RATE_PARENT, 0x06c, 9, CLK_GATE_SET_TO_DISABLE, },
    {HIX5HD2_I2C2_CLK, "clk_i2c2", "100m",
    CLK_SET_RATE_PARENT, 0x06c, 12, 0, },
    {HIX5HD2_I2C2_RST, "rst_i2c2", "clk_i2c2",
    CLK_SET_RATE_PARENT, 0x06c, 13, CLK_GATE_SET_TO_DISABLE, },
    {HIX5HD2_I2C3_CLK, "clk_i2c3", "100m",
    CLK_SET_RATE_PARENT, 0x06c, 16, 0, },
    {HIX5HD2_I2C3_RST, "rst_i2c3", "clk_i2c3",
    CLK_SET_RATE_PARENT, 0x06c, 17, CLK_GATE_SET_TO_DISABLE, },
    {HIX5HD2_I2C4_CLK, "clk_i2c4", "100m",
    CLK_SET_RATE_PARENT, 0x06c, 20, 0, },
    {HIX5HD2_I2C4_RST, "rst_i2c4", "clk_i2c4",
    CLK_SET_RATE_PARENT, 0x06c, 21, CLK_GATE_SET_TO_DISABLE, },
    {HIX5HD2_I2C5_CLK, "clk_i2c5", "100m",
    CLK_SET_RATE_PARENT, 0x06c, 0, 0, },
    {HIX5HD2_I2C5_RST, "rst_i2c5", "clk_i2c5",
    CLK_SET_RATE_PARENT, 0x06c, 1, CLK_GATE_SET_TO_DISABLE, },
    };
    enum hix5hd2_clk_type {
    TYPE_COMPLEX,
    TYPE_ETHER,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hix5hd2_complex_clock {
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub id: u32,
    pub ctrl_reg: u32,
    pub ctrl_clk_mask: u32,
    pub ctrl_rst_mask: u32,
    pub phy_reg: u32,
    pub phy_clk_mask: u32,
    pub phy_rst_mask: u32,
    pub type: enum hix5hd2_clk_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hix5hd2_clk_complex {
    pub hw: clk_hw,
    pub id: u32,
    pub ctrl_reg: *mut void __iomem,
    pub ctrl_clk_mask: u32,
    pub ctrl_rst_mask: u32,
    pub phy_reg: *mut void __iomem,
    pub phy_clk_mask: u32,
    pub phy_rst_mask: u32,
}

    static struct hix5hd2_complex_clock hix5hd2_complex_clks[] __initdata = {
    {"clk_mac0", "clk_fephy", HIX5HD2_MAC0_CLK,
    0xcc, 0xa, 0x500, 0x120, 0, 0x10, TYPE_ETHER},
    {"clk_mac1", "clk_fwd_sys", HIX5HD2_MAC1_CLK,
    0xcc, 0x14, 0xa00, 0x168, 0x2, 0, TYPE_ETHER},
    {"clk_sata", core::ptr::null_mut(), HIX5HD2_SATA_CLK,
    0xa8, 0x1f, 0x300, 0xac, 0x1, 0x0, TYPE_COMPLEX},
    {"clk_usb", core::ptr::null_mut(), HIX5HD2_USB_CLK,
    0xb8, 0xff, 0x3f000, 0xbc, 0x7, 0x3f00, TYPE_COMPLEX},
    };

#[no_mangle]
unsafe extern "C" fn clk_ether_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_ether_prepare(struct clk_hw *hw)
    {
    struct hix5hd2_clk_complex *clk = to_complex_clk(hw);
    u32 val;
    val = readl_relaxed(clk.ctrl_reg);
    val |= clk.ctrl_clk_mask | clk.ctrl_rst_mask;
    writel_relaxed(val, clk.ctrl_reg);
    val &= ~(clk.ctrl_rst_mask);
    writel_relaxed(val, clk.ctrl_reg);
    val = readl_relaxed(clk.phy_reg);
    val |= clk.phy_clk_mask;
    val &= ~(clk.phy_rst_mask);
    writel_relaxed(val, clk.phy_reg);
    mdelay(10);
    val &= ~(clk.phy_clk_mask);
    val |= clk.phy_rst_mask;
    writel_relaxed(val, clk.phy_reg);
    mdelay(10);
    val |= clk.phy_clk_mask;
    val &= ~(clk.phy_rst_mask);
    writel_relaxed(val, clk.phy_reg);
    mdelay(30);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_ether_unprepare(hw: *mut clk_hw) {
    static void clk_ether_unprepare(struct clk_hw *hw)
    {
    struct hix5hd2_clk_complex *clk = to_complex_clk(hw);
    u32 val;
    val = readl_relaxed(clk.ctrl_reg);
    val &= ~(clk.ctrl_clk_mask);
    writel_relaxed(val, clk.ctrl_reg);
    }
    static const struct clk_ops clk_ether_ops = {
    .prepare = clk_ether_prepare,
    .unprepare = clk_ether_unprepare,
    };
#[no_mangle]
unsafe extern "C" fn clk_complex_enable(hw: *mut clk_hw) -> c_int {
    static int clk_complex_enable(struct clk_hw *hw)
    {
    struct hix5hd2_clk_complex *clk = to_complex_clk(hw);
    u32 val;
    val = readl_relaxed(clk.ctrl_reg);
    val |= clk.ctrl_clk_mask;
    val &= ~(clk.ctrl_rst_mask);
    writel_relaxed(val, clk.ctrl_reg);
    val = readl_relaxed(clk.phy_reg);
    val |= clk.phy_clk_mask;
    val &= ~(clk.phy_rst_mask);
    writel_relaxed(val, clk.phy_reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_complex_disable(hw: *mut clk_hw) {
    static void clk_complex_disable(struct clk_hw *hw)
    {
    struct hix5hd2_clk_complex *clk = to_complex_clk(hw);
    u32 val;
    val = readl_relaxed(clk.ctrl_reg);
    val |= clk.ctrl_rst_mask;
    val &= ~(clk.ctrl_clk_mask);
    writel_relaxed(val, clk.ctrl_reg);
    val = readl_relaxed(clk.phy_reg);
    val |= clk.phy_rst_mask;
    val &= ~(clk.phy_clk_mask);
    writel_relaxed(val, clk.phy_reg);
    }
    static const struct clk_ops clk_complex_ops = {
    .enable = clk_complex_enable,
    .disable = clk_complex_disable,
    };
    static void __init
    hix5hd2_clk_register_complex(struct hix5hd2_complex_clock *clks, int nums,
    struct hisi_clock_data *data)
    {
    void __iomem *base = data.base;
    int i;
    for (i = 0; i < nums; i++) {
    struct hix5hd2_clk_complex *p_clk;
    struct clk *clk;
    struct clk_init_data init;
    p_clk = kzalloc_obj(*p_clk);
    if (!p_clk)
    return;
    init.name = clks[i].name;
    if (clks[i].type == TYPE_ETHER)
    init.ops = &clk_ether_ops;
    else
    init.ops = &clk_complex_ops;
    init.flags = 0;
    init.parent_names =
    (clks[i].parent_name ? &clks[i].parent_name : core::ptr::null_mut());
    init.num_parents = (clks[i].parent_name ? 1 : 0);
    p_clk.ctrl_reg = base + clks[i].ctrl_reg;
    p_clk.ctrl_clk_mask = clks[i].ctrl_clk_mask;
    p_clk.ctrl_rst_mask = clks[i].ctrl_rst_mask;
    p_clk.phy_reg = base + clks[i].phy_reg;
    p_clk.phy_clk_mask = clks[i].phy_clk_mask;
    p_clk.phy_rst_mask = clks[i].phy_rst_mask;
    p_clk.hw.init = &init;
    clk = clk_register(core::ptr::null_mut(), &p_clk.hw);
    if (IS_ERR(clk)) {
    kfree(p_clk);
    pr_err("%s: failed to register clock %s\n",
    __func__, clks[i].name);
    continue;
    }
    data.clk_data.clks[clks[i].id] = clk;
    }
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_clk_init(np: *mut device_node) -> void __init {
    static void __init hix5hd2_clk_init(struct device_node *np)
    {
    struct hisi_clock_data *clk_data;
    clk_data = hisi_clk_init(np, HIX5HD2_NR_CLKS);
    if (!clk_data)
    return;
    hisi_clk_register_fixed_rate(hix5hd2_fixed_rate_clks,
    ARRAY_SIZE(hix5hd2_fixed_rate_clks),
    clk_data);
    hisi_clk_register_mux(hix5hd2_mux_clks, ARRAY_SIZE(hix5hd2_mux_clks),
    clk_data);
    hisi_clk_register_gate(hix5hd2_gate_clks,
    ARRAY_SIZE(hix5hd2_gate_clks), clk_data);
    hix5hd2_clk_register_complex(hix5hd2_complex_clks,
    ARRAY_SIZE(hix5hd2_complex_clks),
    clk_data);
    }
    CLK_OF_DECLARE(hix5hd2_clk, "hisilicon,hix5hd2-clock", hix5hd2_clk_init);
