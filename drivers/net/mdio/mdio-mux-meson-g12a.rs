//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-mux-meson-g12a.c
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
// Copyright (c) 2019 Baylibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>
//

pub const ETH_PLL_STS: c_uint = 0x40;
pub const ETH_PLL_CTL0: c_uint = 0x44;

pub const PLL_LOCK_TIMEOUT: c_int = 1000000;
pub const PLL_MUX_NUM_PARENT: c_int = 2;
pub const ETH_PLL_CTL1: c_uint = 0x48;
pub const ETH_PLL_CTL2: c_uint = 0x4c;
pub const ETH_PLL_CTL3: c_uint = 0x50;
pub const ETH_PLL_CTL4: c_uint = 0x54;
pub const ETH_PLL_CTL5: c_uint = 0x58;
pub const ETH_PLL_CTL6: c_uint = 0x5c;
pub const ETH_PLL_CTL7: c_uint = 0x60;
pub const ETH_PHY_CNTL0: c_uint = 0x80;
pub const EPHY_G12A_ID: c_uint = 0x33010180;
pub const ETH_PHY_CNTL1: c_uint = 0x84;

pub const EPHY_DFLT_ADD: c_int = 8;

pub const EPHY_MODE_RMII: c_uint = 0x1;

pub const ETH_PHY_CNTL2: c_uint = 0x88;

pub const MESON_G12A_MDIO_EXTERNAL_ID: c_int = 0;
pub const MESON_G12A_MDIO_INTERNAL_ID: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct g12a_mdio_mux {
    pub regs: *mut void __iomem,
    pub mux_handle: *mut c_void,
    pub pll: *mut clk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct g12a_ephy_pll {
    pub base: *mut void __iomem,
    pub hw: clk_hw,
}

    container_of(_hw, struct g12a_ephy_pll, hw)
    static unsigned long g12a_ephy_pll_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct g12a_ephy_pll *pll = g12a_ephy_pll_to_dev(hw);
    u32 val, m, n;
    val = readl(pll.base + ETH_PLL_CTL0);
    m = FIELD_GET(PLL_CTL0_M, val);
    n = FIELD_GET(PLL_CTL0_N, val);
    return parent_rate * m / n;
    }
#[no_mangle]
unsafe extern "C" fn g12a_ephy_pll_enable(hw: *mut clk_hw) -> c_int {
    static int g12a_ephy_pll_enable(struct clk_hw *hw)
    {
    struct g12a_ephy_pll *pll = g12a_ephy_pll_to_dev(hw);
    let mut val: u32 = readl(pll.base + ETH_PLL_CTL0);
// Apply both enable an reset
    val |= PLL_CTL0_RST | PLL_CTL0_EN;
    writel(val, pll.base + ETH_PLL_CTL0);
// Clear the reset to let PLL lock
    val &= ~PLL_CTL0_RST;
    writel(val, pll.base + ETH_PLL_CTL0);
// Poll on the digital lock instead of the usual analog lock
// This is done because bit 31 is unreliable on some SoC. Bit
// 31 may indicate that the PLL is not lock even though the clock
// is actually running
//
    return readl_poll_timeout(pll.base + ETH_PLL_CTL0, val,
    val & PLL_CTL0_LOCK_DIG, 0, PLL_LOCK_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn g12a_ephy_pll_disable(hw: *mut clk_hw) {
    static void g12a_ephy_pll_disable(struct clk_hw *hw)
    {
    struct g12a_ephy_pll *pll = g12a_ephy_pll_to_dev(hw);
    u32 val;
    val = readl(pll.base + ETH_PLL_CTL0);
    val &= ~PLL_CTL0_EN;
    val |= PLL_CTL0_RST;
    writel(val, pll.base + ETH_PLL_CTL0);
    }
#[no_mangle]
unsafe extern "C" fn g12a_ephy_pll_is_enabled(hw: *mut clk_hw) -> c_int {
    static int g12a_ephy_pll_is_enabled(struct clk_hw *hw)
    {
    struct g12a_ephy_pll *pll = g12a_ephy_pll_to_dev(hw);
    unsigned int val;
    val = readl(pll.base + ETH_PLL_CTL0);
    return (val & PLL_CTL0_LOCK_DIG) ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn g12a_ephy_pll_init(hw: *mut clk_hw) -> c_int {
    static int g12a_ephy_pll_init(struct clk_hw *hw)
    {
    struct g12a_ephy_pll *pll = g12a_ephy_pll_to_dev(hw);
// Apply PLL HW settings
    writel(0x29c0040a, pll.base + ETH_PLL_CTL0);
    writel(0x927e0000, pll.base + ETH_PLL_CTL1);
    writel(0xac5f49e5, pll.base + ETH_PLL_CTL2);
    writel(0x00000000, pll.base + ETH_PLL_CTL3);
    writel(0x00000000, pll.base + ETH_PLL_CTL4);
    writel(0x20200000, pll.base + ETH_PLL_CTL5);
    writel(0x0000c002, pll.base + ETH_PLL_CTL6);
    writel(0x00000023, pll.base + ETH_PLL_CTL7);
    return 0;
    }
    static const struct clk_ops g12a_ephy_pll_ops = {
    .recalc_rate	= g12a_ephy_pll_recalc_rate,
    .is_enabled	= g12a_ephy_pll_is_enabled,
    .enable		= g12a_ephy_pll_enable,
    .disable	= g12a_ephy_pll_disable,
    .init		= g12a_ephy_pll_init,
    };
#[no_mangle]
unsafe extern "C" fn g12a_enable_internal_mdio(priv: *mut g12a_mdio_mux) -> c_int {
    static int g12a_enable_internal_mdio(struct g12a_mdio_mux *priv)
    {
    u32 value;
    int ret;
// Enable the phy clock
    if (!__clk_is_enabled(priv.pll)) {
    ret = clk_prepare_enable(priv.pll);
    if (ret)
    return ret;
    }
// Initialize ephy control
    writel(EPHY_G12A_ID, priv.regs + ETH_PHY_CNTL0);
// Make sure we get a 0 -> 1 transition on the enable bit
    value = FIELD_PREP(PHY_CNTL1_ST_MODE, 3) |
    FIELD_PREP(PHY_CNTL1_ST_PHYADD, EPHY_DFLT_ADD) |
    FIELD_PREP(PHY_CNTL1_MII_MODE, EPHY_MODE_RMII) |
    PHY_CNTL1_CLK_EN |
    PHY_CNTL1_CLKFREQ;
    writel(value, priv.regs + ETH_PHY_CNTL1);
    writel(PHY_CNTL2_USE_INTERNAL |
    PHY_CNTL2_SMI_SRC_MAC |
    PHY_CNTL2_RX_CLK_EPHY,
    priv.regs + ETH_PHY_CNTL2);
    value |= PHY_CNTL1_PHY_ENB;
    writel(value, priv.regs + ETH_PHY_CNTL1);
// The phy needs a bit of time to power up
    mdelay(10);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn g12a_enable_external_mdio(priv: *mut g12a_mdio_mux) -> c_int {
    static int g12a_enable_external_mdio(struct g12a_mdio_mux *priv)
    {
// Reset the mdio bus mux
    writel_relaxed(0x0, priv.regs + ETH_PHY_CNTL2);
// Disable the phy clock if enabled
    if (__clk_is_enabled(priv.pll))
    clk_disable_unprepare(priv.pll);
    return 0;
    }
    static int g12a_mdio_switch_fn(int current_child, int desired_child,
    void *data)
    {
    struct g12a_mdio_mux *priv = dev_get_drvdata(data);
    if (current_child == desired_child)
    return 0;
    switch (desired_child) {
    case MESON_G12A_MDIO_EXTERNAL_ID:
    return g12a_enable_external_mdio(priv);
    case MESON_G12A_MDIO_INTERNAL_ID:
    return g12a_enable_internal_mdio(priv);
    default:
    return -EINVAL;
    }
    }
    static const struct of_device_id g12a_mdio_mux_match[] = {
    { .compatible = "amlogic,g12a-mdio-mux", },
    {},
    };
    MODULE_DEVICE_TABLE(of, g12a_mdio_mux_match);
#[no_mangle]
unsafe extern "C" fn g12a_ephy_glue_clk_register(dev: *mut device) -> c_int {
    static int g12a_ephy_glue_clk_register(struct device *dev)
    {
    struct g12a_mdio_mux *priv = dev_get_drvdata(dev);
    const char *parent_names[PLL_MUX_NUM_PARENT];
    struct clk_init_data init;
    struct g12a_ephy_pll *pll;
    struct clk_mux *mux;
    struct clk *clk;
    char *name;
    int i;
// get the mux parents
    for (i = 0; i < PLL_MUX_NUM_PARENT; i++) {
    char in_name[8];
    snprintf(in_name, sizeof(in_name), "clkin%d", i);
    clk = devm_clk_get(dev, in_name);
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk),
    "Missing clock %s\n", in_name);
    parent_names[i] = __clk_get_name(clk);
    }
// create the input mux
    mux = devm_kzalloc(dev, sizeof(*mux), GFP_KERNEL);
    if (!mux)
    return -ENOMEM;
    name = kasprintf(GFP_KERNEL, "%s#mux", dev_name(dev));
    if (!name)
    return -ENOMEM;
    init.name = name;
    init.ops = &clk_mux_ro_ops;
    init.flags = 0;
    init.parent_names = parent_names;
    init.num_parents = PLL_MUX_NUM_PARENT;
    mux.reg = priv.regs + ETH_PLL_CTL0;
    mux.shift = __ffs(PLL_CTL0_SEL);
    mux.mask = PLL_CTL0_SEL >> mux.shift;
    mux.hw.init = &init;
    clk = devm_clk_register(dev, &mux.hw);
    kfree(name);
    if (IS_ERR(clk)) {
    dev_err(dev, "failed to register input mux\n");
    return PTR_ERR(clk);
    }
// create the pll
    pll = devm_kzalloc(dev, sizeof(*pll), GFP_KERNEL);
    if (!pll)
    return -ENOMEM;
    name = kasprintf(GFP_KERNEL, "%s#pll", dev_name(dev));
    if (!name)
    return -ENOMEM;
    init.name = name;
    init.ops = &g12a_ephy_pll_ops;
    init.flags = 0;
    parent_names[0] = __clk_get_name(clk);
    init.parent_names = parent_names;
    init.num_parents = 1;
    pll.base = priv.regs;
    pll.hw.init = &init;
    clk = devm_clk_register(dev, &pll.hw);
    kfree(name);
    if (IS_ERR(clk)) {
    dev_err(dev, "failed to register input mux\n");
    return PTR_ERR(clk);
    }
    priv.pll = clk;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn g12a_mdio_mux_probe(pdev: *mut platform_device) -> c_int {
    static int g12a_mdio_mux_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct g12a_mdio_mux *priv;
    struct clk *pclk;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
    priv.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.regs))
    return PTR_ERR(priv.regs);
    pclk = devm_clk_get_enabled(dev, "pclk");
    if (IS_ERR(pclk))
    return dev_err_probe(dev, PTR_ERR(pclk),
    "failed to get peripheral clock\n");
// Register PLL in CCF
    ret = g12a_ephy_glue_clk_register(dev);
    if (ret)
    return ret;
    ret = mdio_mux_init(dev, dev.of_node, g12a_mdio_switch_fn,
    &priv.mux_handle, dev, core::ptr::null_mut());
    if (ret)
    dev_err_probe(dev, ret, "mdio multiplexer init failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn g12a_mdio_mux_remove(pdev: *mut platform_device) {
    static void g12a_mdio_mux_remove(struct platform_device *pdev)
    {
    struct g12a_mdio_mux *priv = platform_get_drvdata(pdev);
    mdio_mux_uninit(priv.mux_handle);
    if (__clk_is_enabled(priv.pll))
    clk_disable_unprepare(priv.pll);
    }
    static struct platform_driver g12a_mdio_mux_driver = {
    .probe		= g12a_mdio_mux_probe,
    .remove		= g12a_mdio_mux_remove,
    .driver		= {
    .name	= "g12a-mdio_mux",
    .of_match_table = g12a_mdio_mux_match,
    },
    };
    module_platform_driver(g12a_mdio_mux_driver);
    MODULE_DESCRIPTION("Amlogic G12a MDIO multiplexer driver");
    MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
    MODULE_LICENSE("GPL v2");
