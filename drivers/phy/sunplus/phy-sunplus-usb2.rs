//! Automatically rewritten from C to Rust
//! Source: drivers/phy/sunplus/phy-sunplus-usb2.c
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
// Sunplus SP7021 USB 2.0 phy driver
//
// Copyright (C) 2022 Sunplus Technology Inc., All rights reserved.
//
// Note 1 : non-posted write command for the registers accesses of
// Sunplus SP7021.
//

pub const OTP_DISC_LEVEL_DEFAULT: c_uint = 0xd;
// GROUP UPHY
pub const CONFIG1: c_uint = 0x4;

pub const CONFIG3: c_uint = 0xc;

pub const CONFIG7: c_uint = 0x1c;

pub const CONFIG9: c_uint = 0x24;

pub const CONFIG16: c_uint = 0x40;

pub const CONFIG17: c_uint = 0x44;

pub const CONFIG23: c_uint = 0x5c;

// GROUP MOON4
pub const UPHY_CONTROL0: c_uint = 0x0;
pub const UPHY_CONTROL1: c_uint = 0x4;
pub const UPHY_CONTROL2: c_uint = 0x8;

pub const UPHY_CONTROL3: c_uint = 0xc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp_usbphy {
    pub dev: *mut device,
    pub phy_res_mem: *mut resource,
    pub moon4_res_mem: *mut resource,
    pub rstc: *mut reset_control,
    pub phy_clk: *mut clk,
    pub phy_regs: *mut void __iomem,
    pub moon4_regs: *mut void __iomem,
    pub disc_vol_addr_off: u32,
}

#[no_mangle]
unsafe extern "C" fn update_disc_vol(usbphy: *mut sp_usbphy) -> c_int {
    static int update_disc_vol(struct sp_usbphy *usbphy)
    {
    struct nvmem_cell *cell;
    char *disc_name = "disc_vol";
    let mut otp_l: isize = 0;
    char *otp_v;
    u32 val, set;
    cell = nvmem_cell_get(usbphy.dev, disc_name);
    if (IS_ERR_OR_NULL(cell)) {
    if (PTR_ERR(cell) == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    }
    otp_v = nvmem_cell_read(cell, &otp_l);
    nvmem_cell_put(cell);
    if (!IS_ERR(otp_v)) {
    set = *(otp_v + 1);
    set = (set << (sizeof(char) * 8)) | *otp_v;
    set = (set >> usbphy.disc_vol_addr_off) & J_DISC;
    }
    if (IS_ERR(otp_v) || set == 0)
    set = OTP_DISC_LEVEL_DEFAULT;
    val = readl(usbphy.phy_regs + CONFIG7);
    val = (val & ~J_DISC) | set;
    writel(val, usbphy.phy_regs + CONFIG7);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sp_uphy_init(phy: *mut phy) -> c_int {
    static int sp_uphy_init(struct phy *phy)
    {
    struct sp_usbphy *usbphy = phy_get_drvdata(phy);
    u32 val;
    int ret;
    ret = clk_prepare_enable(usbphy.phy_clk);
    if (ret)
    return ret;
    ret = reset_control_deassert(usbphy.rstc);
    if (ret)
    goto err_clk;
// Default value modification
    writel(HIGH_MASK_BITS | 0x4002, usbphy.moon4_regs + UPHY_CONTROL0);
    writel(HIGH_MASK_BITS | 0x8747, usbphy.moon4_regs + UPHY_CONTROL1);
// disconnect voltage
    ret = update_disc_vol(usbphy);
    if (ret < 0)
    goto err_reset;
// board uphy 0 internal register modification for tid certification
    val = readl(usbphy.phy_regs + CONFIG9);
    val &= ~(J_ECO_PATH);
    writel(val, usbphy.phy_regs + CONFIG9);
    val = readl(usbphy.phy_regs + CONFIG1);
    val &= ~(J_HS_TX_PWRSAV);
    writel(val, usbphy.phy_regs + CONFIG1);
    val = readl(usbphy.phy_regs + CONFIG23);
    val = (val & ~PROB) | PROB;
    writel(val, usbphy.phy_regs + CONFIG23);
// port 0 uphy clk fix
    writel(MASK_MO1_UPHY_RX_CLK_SEL | MO1_UPHY_RX_CLK_SEL,
    usbphy.moon4_regs + UPHY_CONTROL2);
// battery charger
    writel(J_TBCWAIT_1P1_MS | J_TVDM_SRC_DIS_8P2_MS | J_TVDM_SRC_EN_1P6_MS | J_BC_EN,
    usbphy.phy_regs + CONFIG16);
    writel(IBG_TRIM0_SSLVHT | J_VDATREE_TRIM_DEFAULT, usbphy.phy_regs + CONFIG17);
// chirp mode
    writel(J_FORCE_DISC_ON | J_DEBUG_CTRL_ADDR_MACRO, usbphy.phy_regs + CONFIG3);
    return 0;
    err_reset:
    reset_control_assert(usbphy.rstc);
    err_clk:
    clk_disable_unprepare(usbphy.phy_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sp_uphy_power_on(phy: *mut phy) -> c_int {
    static int sp_uphy_power_on(struct phy *phy)
    {
    struct sp_usbphy *usbphy = phy_get_drvdata(phy);
    u32 pll_pwr_on, pll_pwr_off;
// PLL power off/on twice
    pll_pwr_off = (readl(usbphy.moon4_regs + UPHY_CONTROL3) & ~LOW_MASK_BITS)
    | MO1_UPHY_PLL_POWER_OFF_SEL | MO1_UPHY_PLL_POWER_OFF;
    pll_pwr_on = (readl(usbphy.moon4_regs + UPHY_CONTROL3) & ~LOW_MASK_BITS)
    | MO1_UPHY_PLL_POWER_OFF_SEL;
    writel(MASK_MO1_UPHY_PLL_POWER_OFF_SEL | MASK_UPHY_PLL_POWER_OFF | pll_pwr_off,
    usbphy.moon4_regs + UPHY_CONTROL3);
    mdelay(1);
    writel(MASK_MO1_UPHY_PLL_POWER_OFF_SEL | MASK_UPHY_PLL_POWER_OFF | pll_pwr_on,
    usbphy.moon4_regs + UPHY_CONTROL3);
    mdelay(1);
    writel(MASK_MO1_UPHY_PLL_POWER_OFF_SEL | MASK_UPHY_PLL_POWER_OFF | pll_pwr_off,
    usbphy.moon4_regs + UPHY_CONTROL3);
    mdelay(1);
    writel(MASK_MO1_UPHY_PLL_POWER_OFF_SEL | MASK_UPHY_PLL_POWER_OFF | pll_pwr_on,
    usbphy.moon4_regs + UPHY_CONTROL3);
    mdelay(1);
    writel(MASK_MO1_UPHY_PLL_POWER_OFF_SEL | MASK_UPHY_PLL_POWER_OFF | 0x0,
    usbphy.moon4_regs + UPHY_CONTROL3);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sp_uphy_power_off(phy: *mut phy) -> c_int {
    static int sp_uphy_power_off(struct phy *phy)
    {
    struct sp_usbphy *usbphy = phy_get_drvdata(phy);
    u32 pll_pwr_off;
    pll_pwr_off = (readl(usbphy.moon4_regs + UPHY_CONTROL3) & ~LOW_MASK_BITS)
    | MO1_UPHY_PLL_POWER_OFF_SEL | MO1_UPHY_PLL_POWER_OFF;
    writel(MASK_MO1_UPHY_PLL_POWER_OFF_SEL | MASK_UPHY_PLL_POWER_OFF | pll_pwr_off,
    usbphy.moon4_regs + UPHY_CONTROL3);
    mdelay(1);
    writel(MASK_MO1_UPHY_PLL_POWER_OFF_SEL | MASK_UPHY_PLL_POWER_OFF | 0x0,
    usbphy.moon4_regs + UPHY_CONTROL3);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sp_uphy_exit(phy: *mut phy) -> c_int {
    static int sp_uphy_exit(struct phy *phy)
    {
    struct sp_usbphy *usbphy = phy_get_drvdata(phy);
    reset_control_assert(usbphy.rstc);
    clk_disable_unprepare(usbphy.phy_clk);
    return 0;
    }
    static const struct phy_ops sp_uphy_ops = {
    .init		= sp_uphy_init,
    .power_on	= sp_uphy_power_on,
    .power_off	= sp_uphy_power_off,
    .exit		= sp_uphy_exit,
    };
    static const struct of_device_id sp_uphy_dt_ids[] = {
    {.compatible = "sunplus,sp7021-usb2-phy", },
    { }
    };
    MODULE_DEVICE_TABLE(of, sp_uphy_dt_ids);
#[no_mangle]
unsafe extern "C" fn sp_usb_phy_probe(pdev: *mut platform_device) -> c_int {
    static int sp_usb_phy_probe(struct platform_device *pdev)
    {
    struct sp_usbphy *usbphy;
    struct phy_provider *phy_provider;
    struct phy *phy;
    int ret;
    usbphy = devm_kzalloc(&pdev.dev, sizeof(*usbphy), GFP_KERNEL);
    if (!usbphy)
    return -ENOMEM;
    usbphy.dev = &pdev.dev;
    usbphy.phy_res_mem = platform_get_resource_byname(pdev, IORESOURCE_MEM, "phy");
    usbphy.phy_regs = devm_ioremap_resource(&pdev.dev, usbphy.phy_res_mem);
    if (IS_ERR(usbphy.phy_regs))
    return PTR_ERR(usbphy.phy_regs);
    usbphy.moon4_res_mem = platform_get_resource_byname(pdev, IORESOURCE_MEM, "moon4");
    if (!usbphy.moon4_res_mem)
    return -EINVAL;
    usbphy.moon4_regs = devm_ioremap(&pdev.dev, usbphy.moon4_res_mem.start,
    resource_size(usbphy.moon4_res_mem));
    if (!usbphy.moon4_regs)
    return -ENOMEM;
    usbphy.phy_clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(usbphy.phy_clk))
    return PTR_ERR(usbphy.phy_clk);
    usbphy.rstc = devm_reset_control_get_exclusive(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(usbphy.rstc))
    return PTR_ERR(usbphy.rstc);
    of_property_read_u32(pdev.dev.of_node, "sunplus,disc-vol-addr-off",
    &usbphy.disc_vol_addr_off);
    phy = devm_phy_create(&pdev.dev, core::ptr::null_mut(), &sp_uphy_ops);
    if (IS_ERR(phy)) {
    ret = PTR_ERR(phy);
    return ret;
    }
    phy_set_drvdata(phy, usbphy);
    phy_provider = devm_of_phy_provider_register(&pdev.dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static struct platform_driver sunplus_usb_phy_driver = {
    .probe		= sp_usb_phy_probe,
    .driver		= {
    .name	= "sunplus-usb2-phy",
    .of_match_table = sp_uphy_dt_ids,
    },
    };
    module_platform_driver(sunplus_usb_phy_driver);
    MODULE_AUTHOR("Vincent Shih <vincent.shih@sunplus.com>");
    MODULE_DESCRIPTION("Sunplus USB 2.0 phy driver");
    MODULE_LICENSE("GPL");
