//! Automatically rewritten from C to Rust
//! Source: drivers/phy/broadcom/phy-brcm-usb-init-synopsys.c
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
// Copyright (c) 2018, Broadcom
//
// This module contains USB PHY initialization for power up and S3 resume
// for newer Synopsys based USB hardware first used on the bcm7216.
//

pub const PHY_LOCK_TIMEOUT_MS: c_int = 200;
// Register definitions for syscon piarbctl registers
pub const PIARBCTL_CAM: c_uint = 0x00;
pub const PIARBCTL_SPLITTER: c_uint = 0x04;
pub const PIARBCTL_MISC: c_uint = 0x08;

    (PIARBCTL_MISC_USB_SELECT_MASK |	\
    PIARBCTL_MISC_USB_4G_SDRAM_MASK |	\
    PIARBCTL_MISC_USB_PRIORITY_MASK |	\
    PIARBCTL_MISC_USB_MEM_PAGE_MASK)
// Register definitions for the USB CTRL block
pub const USB_CTRL_SETUP: c_uint = 0x00;

pub const USB_CTRL_USB_PM: c_uint = 0x04;

pub const USB_CTRL_USB_PM_STATUS: c_uint = 0x08;
pub const USB_CTRL_USB_DEVICE_CTL1: c_uint = 0x10;

pub const USB_CTRL_TEST_PORT_CTL: c_uint = 0x30;

pub const USB_CTRL_TEST_PORT_CTL_TPOUT_SEL_PME_GEN_MASK: c_uint = 0x0000002e;
pub const USB_CTRL_TP_DIAG1: c_uint = 0x34;

pub const USB_CTRL_CTLR_CSHCR: c_uint = 0x50;

pub const USB_CTRL_P0_U2PHY_CFG1: c_uint = 0x68;

pub const USB_CTRL_P0_U2PHY_CFG2: c_uint = 0x6c;

pub const USB_CTRL_P0_U2PHY_CFG2_TXVREFTUNE0_SHIFT: c_int = 17;

pub const USB_CTRL_P0_U2PHY_CFG2_TXRESTUNE0_SHIFT: c_int = 23;

pub const USB_CTRL_P0_U2PHY_CFG2_TXPREEMPAMPTUNE0_SHIFT: c_int = 25;
// Register definitions for the USB_PHY block in 7211b0
pub const USB_PHY_PLL_CTL: c_uint = 0x00;

pub const USB_PHY_PLL_LDO_CTL: c_uint = 0x08;

pub const USB_PHY_UTMI_CTL_1: c_uint = 0x04;

pub const USB_PHY_UTMI_CTL_1_PHY_MODE_SHIFT: c_int = 2;

pub const USB_PHY_IDDQ: c_uint = 0x1c;

pub const USB_PHY_STATUS: c_uint = 0x20;

// Register definitions for the MDIO registers in the DWC2 block of
// the 7211b0.
// NOTE: The PHY's MDIO registers are only accessible through the
// legacy DesignWare USB controller even though it's not being used.
//
pub const USB_GMDIOCSR: c_int = 0;
pub const USB_GMDIOGEN: c_int = 4;
// Register definitions for the BDC EC block in 7211b0
pub const BDC_EC_AXIRDA: c_uint = 0x0c;

pub const BDC_EC_AXIRDA_RTS_SHIFT: c_int = 28;
pub const USB_XHCI_GBL_GUSB2PHYCFG: c_uint = 0x100;

    static void usb_mdio_write_7211b0(struct brcm_usb_init_params *params,
    uint8_t addr, uint16_t data)
    {
    void __iomem *usb_mdio = params.regs[BRCM_REGS_USB_MDIO];
    addr &= 0x1f; /* 5-bit address */
    brcm_usb_writel(0xffffffff, usb_mdio + USB_GMDIOGEN);
    while (brcm_usb_readl(usb_mdio + USB_GMDIOCSR) & (1<<31))
    ;
    brcm_usb_writel(0x59020000 | (addr << 18) | data,
    usb_mdio + USB_GMDIOGEN);
    while (brcm_usb_readl(usb_mdio + USB_GMDIOCSR) & (1<<31))
    ;
    brcm_usb_writel(0x00000000, usb_mdio + USB_GMDIOGEN);
    while (brcm_usb_readl(usb_mdio + USB_GMDIOCSR) & (1<<31))
    ;
    }
    static uint16_t __maybe_unused usb_mdio_read_7211b0(
    struct brcm_usb_init_params *params, uint8_t addr)
    {
    void __iomem *usb_mdio = params.regs[BRCM_REGS_USB_MDIO];
    addr &= 0x1f; /* 5-bit address */
    brcm_usb_writel(0xffffffff, usb_mdio + USB_GMDIOGEN);
    while (brcm_usb_readl(usb_mdio + USB_GMDIOCSR) & (1<<31))
    ;
    brcm_usb_writel(0x69020000 | (addr << 18), usb_mdio + USB_GMDIOGEN);
    while (brcm_usb_readl(usb_mdio + USB_GMDIOCSR) & (1<<31))
    ;
    brcm_usb_writel(0x00000000, usb_mdio + USB_GMDIOGEN);
    while (brcm_usb_readl(usb_mdio + USB_GMDIOCSR) & (1<<31))
    ;
    return brcm_usb_readl(usb_mdio + USB_GMDIOCSR) & 0xffff;
    }
#[no_mangle]
unsafe extern "C" fn usb2_eye_fix_7211b0(params: *mut brcm_usb_init_params) {
    static void usb2_eye_fix_7211b0(struct brcm_usb_init_params *params)
    {
// select bank
    usb_mdio_write_7211b0(params, 0x1f, 0x80a0);
// Set the eye
    usb_mdio_write_7211b0(params, 0x0a, 0xc6a0);
    }
    static void xhci_soft_reset(struct brcm_usb_init_params *params,
    int on_off)
    {
    void __iomem *ctrl = params.regs[BRCM_REGS_CTRL];
    void __iomem *xhci_gbl = params.regs[BRCM_REGS_XHCI_GBL];
// Assert reset
    if (on_off) {
    USB_CTRL_UNSET(ctrl, USB_PM, XHC_SOFT_RESETB);
// De-assert reset
    } else {
    USB_CTRL_SET(ctrl, USB_PM, XHC_SOFT_RESETB);
// Required for COMMONONN to be set
    if (params.supported_port_modes != USB_CTLR_MODE_DRD)
    USB_XHCI_GBL_UNSET(xhci_gbl, GUSB2PHYCFG,
    U2_FREECLK_EXISTS);
    }
    }
#[no_mangle]
unsafe extern "C" fn usb_init_ipp(params: *mut brcm_usb_init_params) {
    static void usb_init_ipp(struct brcm_usb_init_params *params)
    {
    void __iomem *ctrl = params.regs[BRCM_REGS_CTRL];
    u32 reg;
    u32 orig_reg;
    pr_debug("%s\n", __func__);
    orig_reg = reg = brcm_usb_readl(USB_CTRL_REG(ctrl, SETUP));
    if (params.ipp != 2)
// override ipp strap pin (if it exits)
    reg &= ~(USB_CTRL_MASK(SETUP, STRAP_IPP_SEL));
// Override the default OC and PP polarity
    reg &= ~(USB_CTRL_MASK(SETUP, IPP) | USB_CTRL_MASK(SETUP, IOC));
    if (params.ioc)
    reg |= USB_CTRL_MASK(SETUP, IOC);
    if (params.ipp == 1)
    reg |= USB_CTRL_MASK(SETUP, IPP);
    brcm_usb_writel(reg, USB_CTRL_REG(ctrl, SETUP));
//
// If we're changing IPP, make sure power is off long enough
// to turn off any connected devices.
//
    if ((reg ^ orig_reg) & USB_CTRL_MASK(SETUP, IPP))
    msleep(50);
    }
#[no_mangle]
unsafe extern "C" fn syscon_piarbctl_init(rmap: *mut regmap) {
    static void syscon_piarbctl_init(struct regmap *rmap)
    {
// Switch from legacy USB OTG controller to new STB USB controller
    regmap_update_bits(rmap, PIARBCTL_MISC, PIARBCTL_MISC_USB_ONLY_MASK,
    PIARBCTL_MISC_USB_SELECT_MASK |
    PIARBCTL_MISC_USB_4G_SDRAM_MASK);
    }
#[no_mangle]
unsafe extern "C" fn usb_init_common(params: *mut brcm_usb_init_params) {
    static void usb_init_common(struct brcm_usb_init_params *params)
    {
    u32 reg;
    void __iomem *ctrl = params.regs[BRCM_REGS_CTRL];
    pr_debug("%s\n", __func__);
    if (USB_CTRL_MASK(USB_DEVICE_CTL1, PORT_MODE)) {
    reg = brcm_usb_readl(USB_CTRL_REG(ctrl, USB_DEVICE_CTL1));
    reg &= ~USB_CTRL_MASK(USB_DEVICE_CTL1, PORT_MODE);
    reg |= params.port_mode;
    brcm_usb_writel(reg, USB_CTRL_REG(ctrl, USB_DEVICE_CTL1));
    }
    switch (params.supported_port_modes) {
    case USB_CTLR_MODE_HOST:
    USB_CTRL_UNSET(ctrl, USB_PM, BDC_SOFT_RESETB);
    break;
    default:
    USB_CTRL_UNSET(ctrl, USB_PM, BDC_SOFT_RESETB);
    USB_CTRL_SET(ctrl, USB_PM, BDC_SOFT_RESETB);
    break;
    }
    }
    static void usb_wake_enable_7211b0(struct brcm_usb_init_params *params,
    bool enable)
    {
    void __iomem *ctrl = params.regs[BRCM_REGS_CTRL];
    if (enable)
    USB_CTRL_SET(ctrl, CTLR_CSHCR, ctl_pme_en);
    else
    USB_CTRL_UNSET(ctrl, CTLR_CSHCR, ctl_pme_en);
    }
    static void usb_wake_enable_7216(struct brcm_usb_init_params *params,
    bool enable)
    {
    void __iomem *ctrl = params.regs[BRCM_REGS_CTRL];
    if (enable)
    USB_CTRL_SET(ctrl, USB_PM, XHC_PME_EN);
    else
    USB_CTRL_UNSET(ctrl, USB_PM, XHC_PME_EN);
    }
#[no_mangle]
unsafe extern "C" fn usb_init_common_7211b0(params: *mut brcm_usb_init_params) {
    static void usb_init_common_7211b0(struct brcm_usb_init_params *params)
    {
    void __iomem *ctrl = params.regs[BRCM_REGS_CTRL];
    void __iomem *usb_phy = params.regs[BRCM_REGS_USB_PHY];
    void __iomem *bdc_ec = params.regs[BRCM_REGS_BDC_EC];
    let mut timeout_ms: c_int = PHY_LOCK_TIMEOUT_MS;
    u32 reg;
    if (params.syscon_piarbctl)
    syscon_piarbctl_init(params.syscon_piarbctl);
    USB_CTRL_UNSET(ctrl, USB_PM, USB_PWRDN);
    usb_wake_enable_7211b0(params, false);
    if (!params.wake_enabled) {
// undo possible suspend settings
    brcm_usb_writel(0, usb_phy + USB_PHY_IDDQ);
    reg = brcm_usb_readl(usb_phy + USB_PHY_PLL_CTL);
    reg |= USB_PHY_PLL_CTL_PLL_RESETB_MASK;
    brcm_usb_writel(reg, usb_phy + USB_PHY_PLL_CTL);
// temporarily enable FSM so PHY comes up properly
    reg = brcm_usb_readl(usb_phy + USB_PHY_UTMI_CTL_1);
    reg |= USB_PHY_UTMI_CTL_1_POWER_UP_FSM_EN_MASK;
    brcm_usb_writel(reg, usb_phy + USB_PHY_UTMI_CTL_1);
    }
// Disable PLL auto suspend
    reg = brcm_usb_readl(usb_phy + USB_PHY_PLL_CTL);
    reg |= USB_PHY_PLL_CTL_PLL_SUSPEND_MASK;
    brcm_usb_writel(reg, usb_phy + USB_PHY_PLL_CTL);
// Init the PHY
    reg = USB_PHY_PLL_LDO_CTL_AFE_CORERDY_MASK |
    USB_PHY_PLL_LDO_CTL_AFE_LDO_PWRDWNB_MASK |
    USB_PHY_PLL_LDO_CTL_AFE_BG_PWRDWNB_MASK;
    brcm_usb_writel(reg, usb_phy + USB_PHY_PLL_LDO_CTL);
// wait for lock
    while (timeout_ms-- > 0) {
    reg = brcm_usb_readl(usb_phy + USB_PHY_STATUS);
    if (reg & USB_PHY_STATUS_pll_lock_MASK)
    break;
    usleep_range(1000, 2000);
    }
// Set the PHY_MODE
    reg = brcm_usb_readl(usb_phy + USB_PHY_UTMI_CTL_1);
    reg &= ~USB_PHY_UTMI_CTL_1_PHY_MODE_MASK;
    reg |= params.supported_port_modes << USB_PHY_UTMI_CTL_1_PHY_MODE_SHIFT;
    brcm_usb_writel(reg, usb_phy + USB_PHY_UTMI_CTL_1);
    usb_init_common(params);
//
// The BDC controller will get occasional failures with
// the default "Read Transaction Size" of 6 (1024 bytes).
// Set it to 4 (256 bytes).
//
    if ((params.supported_port_modes != USB_CTLR_MODE_HOST) && bdc_ec) {
    reg = brcm_usb_readl(bdc_ec + BDC_EC_AXIRDA);
    reg &= ~BDC_EC_AXIRDA_RTS_MASK;
    reg |= (0x4 << BDC_EC_AXIRDA_RTS_SHIFT);
    brcm_usb_writel(reg, bdc_ec + BDC_EC_AXIRDA);
    }
//
// Disable FSM, otherwise the PHY will auto suspend when no
// device is connected and will be reset on resume.
//
    reg = brcm_usb_readl(usb_phy + USB_PHY_UTMI_CTL_1);
    reg &= ~USB_PHY_UTMI_CTL_1_POWER_UP_FSM_EN_MASK;
    brcm_usb_writel(reg, usb_phy + USB_PHY_UTMI_CTL_1);
    usb2_eye_fix_7211b0(params);
    }
#[no_mangle]
unsafe extern "C" fn usb_init_common_7216(params: *mut brcm_usb_init_params) {
    static void usb_init_common_7216(struct brcm_usb_init_params *params)
    {
    void __iomem *ctrl = params.regs[BRCM_REGS_CTRL];
    USB_CTRL_UNSET(ctrl, USB_PM, XHC_S2_CLK_SWITCH_EN);
//
// The PHY might be in a bad state if it is already powered
// up. Toggle the power just in case.
//
    USB_CTRL_SET(ctrl, USB_PM, USB_PWRDN);
    USB_CTRL_UNSET(ctrl, USB_PM, USB_PWRDN);
// 1 millisecond - for USB clocks to settle down
    usleep_range(1000, 2000);
//
// Disable PHY when port is suspended
// Does not work in DRD mode
//
    if (params.supported_port_modes != USB_CTLR_MODE_DRD)
    USB_CTRL_SET(ctrl, P0_U2PHY_CFG1, COMMONONN);
    usb_wake_enable_7216(params, false);
    usb_init_common(params);
    }
#[no_mangle]
unsafe extern "C" fn usb_init_xhci(params: *mut brcm_usb_init_params) {
    static void usb_init_xhci(struct brcm_usb_init_params *params)
    {
    pr_debug("%s\n", __func__);
    xhci_soft_reset(params, 0);
    }
#[no_mangle]
unsafe extern "C" fn usb_uninit_common_7216(params: *mut brcm_usb_init_params) {
    static void usb_uninit_common_7216(struct brcm_usb_init_params *params)
    {
    void __iomem *ctrl = params.regs[BRCM_REGS_CTRL];
    pr_debug("%s\n", __func__);
    if (params.wake_enabled) {
// Switch to using slower clock during suspend to save power
    USB_CTRL_SET(ctrl, USB_PM, XHC_S2_CLK_SWITCH_EN);
    usb_wake_enable_7216(params, true);
    } else {
    USB_CTRL_SET(ctrl, USB_PM, USB_PWRDN);
    }
    }
#[no_mangle]
unsafe extern "C" fn usb_init_common_74110(params: *mut brcm_usb_init_params) {
    static void usb_init_common_74110(struct brcm_usb_init_params *params)
    {
    void __iomem *ctrl = params.regs[BRCM_REGS_CTRL];
    u32 reg;
    reg = brcm_usb_readl(USB_CTRL_REG(ctrl, USB_PM));
    reg &= ~(USB_CTRL_MASK(USB_PM, REF_S2_CLK_SWITCH_EN) |
    USB_CTRL_MASK(USB_PM, UTMI_S2_CLK_SWITCH_EN));
    brcm_usb_writel(reg, USB_CTRL_REG(ctrl, USB_PM));
    usb_init_common_7216(params);
    reg = brcm_usb_readl(USB_CTRL_REG(ctrl, P0_U2PHY_CFG2));
    reg &= ~(USB_CTRL_P0_U2PHY_CFG2_TXVREFTUNE0_MASK |
    USB_CTRL_P0_U2PHY_CFG2_TXRESTUNE0_MASK |
    USB_CTRL_P0_U2PHY_CFG2_TXPREEMPAMPTUNE0_MASK);
    reg |= (0x6 << USB_CTRL_P0_U2PHY_CFG2_TXVREFTUNE0_SHIFT) |
    (0x3 << USB_CTRL_P0_U2PHY_CFG2_TXRESTUNE0_SHIFT) |
    (0x2 << USB_CTRL_P0_U2PHY_CFG2_TXPREEMPAMPTUNE0_SHIFT);
    brcm_usb_writel(reg, USB_CTRL_REG(ctrl, P0_U2PHY_CFG2));
    }
#[no_mangle]
unsafe extern "C" fn usb_uninit_common_74110(params: *mut brcm_usb_init_params) {
    static void usb_uninit_common_74110(struct brcm_usb_init_params *params)
    {
    void __iomem *ctrl = params.regs[BRCM_REGS_CTRL];
    u32 reg;
    if (params.wake_enabled) {
    reg = brcm_usb_readl(USB_CTRL_REG(ctrl, USB_PM));
    reg |= (USB_CTRL_MASK(USB_PM, REF_S2_CLK_SWITCH_EN) |
    USB_CTRL_MASK(USB_PM, UTMI_S2_CLK_SWITCH_EN));
    brcm_usb_writel(reg, USB_CTRL_REG(ctrl, USB_PM));
    }
    usb_uninit_common_7216(params);
    }
#[no_mangle]
unsafe extern "C" fn usb_uninit_common_7211b0(params: *mut brcm_usb_init_params) {
    static void usb_uninit_common_7211b0(struct brcm_usb_init_params *params)
    {
    void __iomem *ctrl = params.regs[BRCM_REGS_CTRL];
    void __iomem *usb_phy = params.regs[BRCM_REGS_USB_PHY];
    u32 reg;
    pr_debug("%s\n", __func__);
    if (params.wake_enabled) {
    USB_CTRL_SET(ctrl, TEST_PORT_CTL, TPOUT_SEL_PME_GEN);
    usb_wake_enable_7211b0(params, true);
    } else {
    USB_CTRL_SET(ctrl, USB_PM, USB_PWRDN);
    brcm_usb_writel(0, usb_phy + USB_PHY_PLL_LDO_CTL);
    reg = brcm_usb_readl(usb_phy + USB_PHY_PLL_CTL);
    reg &= ~USB_PHY_PLL_CTL_PLL_RESETB_MASK;
    brcm_usb_writel(reg, usb_phy + USB_PHY_PLL_CTL);
    brcm_usb_writel(USB_PHY_IDDQ_phy_iddq_MASK,
    usb_phy + USB_PHY_IDDQ);
    }
    }
#[no_mangle]
unsafe extern "C" fn usb_uninit_xhci(params: *mut brcm_usb_init_params) {
    static void usb_uninit_xhci(struct brcm_usb_init_params *params)
    {
    pr_debug("%s\n", __func__);
    if (!params.wake_enabled)
    xhci_soft_reset(params, 1);
    }
#[no_mangle]
unsafe extern "C" fn usb_get_dual_select(params: *mut brcm_usb_init_params) -> c_int {
    static int usb_get_dual_select(struct brcm_usb_init_params *params)
    {
    void __iomem *ctrl = params.regs[BRCM_REGS_CTRL];
    let mut reg: u32 = 0;
    pr_debug("%s\n", __func__);
    reg = brcm_usb_readl(USB_CTRL_REG(ctrl, USB_DEVICE_CTL1));
    reg &= USB_CTRL_MASK(USB_DEVICE_CTL1, PORT_MODE);
    return reg;
    }
#[no_mangle]
unsafe extern "C" fn usb_set_dual_select(params: *mut brcm_usb_init_params) {
    static void usb_set_dual_select(struct brcm_usb_init_params *params)
    {
    void __iomem *ctrl = params.regs[BRCM_REGS_CTRL];
    u32 reg;
    pr_debug("%s\n", __func__);
    reg = brcm_usb_readl(USB_CTRL_REG(ctrl, USB_DEVICE_CTL1));
    reg &= ~USB_CTRL_MASK(USB_DEVICE_CTL1, PORT_MODE);
    reg |= params.port_mode;
    brcm_usb_writel(reg, USB_CTRL_REG(ctrl, USB_DEVICE_CTL1));
    }
    static const struct brcm_usb_init_ops bcm74110_ops = {
    .init_ipp = usb_init_ipp,
    .init_common = usb_init_common_74110,
    .init_xhci = usb_init_xhci,
    .uninit_common = usb_uninit_common_74110,
    .uninit_xhci = usb_uninit_xhci,
    .get_dual_select = usb_get_dual_select,
    .set_dual_select = usb_set_dual_select,
    };
    static const struct brcm_usb_init_ops bcm7216_ops = {
    .init_ipp = usb_init_ipp,
    .init_common = usb_init_common_7216,
    .init_xhci = usb_init_xhci,
    .uninit_common = usb_uninit_common_7216,
    .uninit_xhci = usb_uninit_xhci,
    .get_dual_select = usb_get_dual_select,
    .set_dual_select = usb_set_dual_select,
    };
    static const struct brcm_usb_init_ops bcm7211b0_ops = {
    .init_ipp = usb_init_ipp,
    .init_common = usb_init_common_7211b0,
    .init_xhci = usb_init_xhci,
    .uninit_common = usb_uninit_common_7211b0,
    .uninit_xhci = usb_uninit_xhci,
    .get_dual_select = usb_get_dual_select,
    .set_dual_select = usb_set_dual_select,
    };
#[no_mangle]
pub unsafe extern "C" fn brcm_usb_dvr_init_74110(params: *mut brcm_usb_init_params) {
    void brcm_usb_dvr_init_74110(struct brcm_usb_init_params *params)
    {
    params.family_name = "74110";
    params.ops = &bcm74110_ops;
    }
#[no_mangle]
pub unsafe extern "C" fn brcm_usb_dvr_init_7216(params: *mut brcm_usb_init_params) {
    void brcm_usb_dvr_init_7216(struct brcm_usb_init_params *params)
    {
    pr_debug("%s\n", __func__);
    params.family_name = "7216";
    params.ops = &bcm7216_ops;
    }
#[no_mangle]
pub unsafe extern "C" fn brcm_usb_dvr_init_7211b0(params: *mut brcm_usb_init_params) {
    void brcm_usb_dvr_init_7211b0(struct brcm_usb_init_params *params)
    {
    pr_debug("%s\n", __func__);
    params.family_name = "7211";
    params.ops = &bcm7211b0_ops;
    }
