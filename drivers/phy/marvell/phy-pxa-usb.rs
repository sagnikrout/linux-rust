//! Automatically rewritten from C to Rust
//! Source: drivers/phy/marvell/phy-pxa-usb.c
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
// Copyright (C) 2011 Marvell International Ltd. All rights reserved.
// Copyright (C) 2018 Lubomir Rintel <lkundrak@v3.sk>
//

// phy regs
pub const UTMI_REVISION: c_uint = 0x0;
pub const UTMI_CTRL: c_uint = 0x4;
pub const UTMI_PLL: c_uint = 0x8;
pub const UTMI_TX: c_uint = 0xc;
pub const UTMI_RX: c_uint = 0x10;
pub const UTMI_IVREF: c_uint = 0x14;
pub const UTMI_T0: c_uint = 0x18;
pub const UTMI_T1: c_uint = 0x1c;
pub const UTMI_T2: c_uint = 0x20;
pub const UTMI_T3: c_uint = 0x24;
pub const UTMI_T4: c_uint = 0x28;
pub const UTMI_T5: c_uint = 0x2c;
pub const UTMI_RESERVE: c_uint = 0x30;
pub const UTMI_USB_INT: c_uint = 0x34;
pub const UTMI_DBG_CTL: c_uint = 0x38;
pub const UTMI_OTG_ADDON: c_uint = 0x3c;
// For UTMICTRL Register

// pxa168

pub const UTMI_CTRL_INPKT_DELAY_SHIFT: c_int = 30;
pub const UTMI_CTRL_INPKT_DELAY_SOF_SHIFT: c_int = 28;
pub const UTMI_CTRL_PU_REF_SHIFT: c_int = 20;
pub const UTMI_CTRL_ARC_PULLDN_SHIFT: c_int = 12;
pub const UTMI_CTRL_PLL_PWR_UP_SHIFT: c_int = 1;
pub const UTMI_CTRL_PWR_UP_SHIFT: c_int = 0;
// For UTMI_PLL Register
pub const UTMI_PLL_PLLCALI12_SHIFT: c_int = 29;

pub const UTMI_PLL_PLLVDD18_SHIFT: c_int = 27;

pub const UTMI_PLL_PLLVDD12_SHIFT: c_int = 25;

pub const UTMI_PLL_CLK_BLK_EN_SHIFT: c_int = 24;

pub const UTMI_PLL_KVCO_SHIFT: c_int = 15;

pub const UTMI_PLL_ICP_SHIFT: c_int = 12;

pub const UTMI_PLL_FBDIV_SHIFT: c_int = 4;

pub const UTMI_PLL_REFDIV_SHIFT: c_int = 0;

// For UTMI_TX Register
pub const UTMI_TX_REG_EXT_FS_RCAL_SHIFT: c_int = 27;

pub const UTMI_TX_REG_EXT_FS_RCAL_EN_SHIFT: c_int = 26;

pub const UTMI_TX_TXVDD12_SHIFT: c_int = 22;

pub const UTMI_TX_CK60_PHSEL_SHIFT: c_int = 17;

pub const UTMI_TX_IMPCAL_VTH_SHIFT: c_int = 14;

pub const UTMI_TX_LOW_VDD_EN_SHIFT: c_int = 11;
pub const UTMI_TX_AMP_SHIFT: c_int = 0;

// For UTMI_RX Register
pub const UTMI_REG_SQ_LENGTH_SHIFT: c_int = 15;

pub const UTMI_RX_SQ_THRESH_SHIFT: c_int = 4;

    enum pxa_usb_phy_version {
    PXA_USB_PHY_MMP2,
    PXA_USB_PHY_PXA910,
    PXA_USB_PHY_PXA168,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa_usb_phy {
    pub phy: *mut phy,
    pub base: *mut void __iomem,
    pub version: enum pxa_usb_phy_version,
}

//
// The registers read/write routines
//
#[no_mangle]
unsafe extern "C" fn u2o_get(base: *mut void __iomem, offset: c_uint) -> c_uint {
    static unsigned int u2o_get(void __iomem *base, unsigned int offset)
    {
    return readl_relaxed(base + offset);
    }
    static void u2o_set(void __iomem *base, unsigned int offset,
    unsigned int value)
    {
    u32 reg;
    reg = readl_relaxed(base + offset);
    reg |= value;
    writel_relaxed(reg, base + offset);
    readl_relaxed(base + offset);
    }
    static void u2o_clear(void __iomem *base, unsigned int offset,
    unsigned int value)
    {
    u32 reg;
    reg = readl_relaxed(base + offset);
    reg &= ~value;
    writel_relaxed(reg, base + offset);
    readl_relaxed(base + offset);
    }
    static void u2o_write(void __iomem *base, unsigned int offset,
    unsigned int value)
    {
    writel_relaxed(value, base + offset);
    readl_relaxed(base + offset);
    }
#[no_mangle]
unsafe extern "C" fn pxa_usb_phy_init(phy: *mut phy) -> c_int {
    static int pxa_usb_phy_init(struct phy *phy)
    {
    struct pxa_usb_phy *pxa_usb_phy = phy_get_drvdata(phy);
    void __iomem *base = pxa_usb_phy.base;
    int loops;
    dev_info(&phy.dev, "initializing Marvell PXA USB PHY");
// Initialize the USB PHY power
    if (pxa_usb_phy.version == PXA_USB_PHY_PXA910) {
    u2o_set(base, UTMI_CTRL, (1<<UTMI_CTRL_INPKT_DELAY_SOF_SHIFT)
    | (1<<UTMI_CTRL_PU_REF_SHIFT));
    }
    u2o_set(base, UTMI_CTRL, 1<<UTMI_CTRL_PLL_PWR_UP_SHIFT);
    u2o_set(base, UTMI_CTRL, 1<<UTMI_CTRL_PWR_UP_SHIFT);
// UTMI_PLL settings
    u2o_clear(base, UTMI_PLL, UTMI_PLL_PLLVDD18_MASK
    | UTMI_PLL_PLLVDD12_MASK | UTMI_PLL_PLLCALI12_MASK
    | UTMI_PLL_FBDIV_MASK | UTMI_PLL_REFDIV_MASK
    | UTMI_PLL_ICP_MASK | UTMI_PLL_KVCO_MASK);
    u2o_set(base, UTMI_PLL, 0xee<<UTMI_PLL_FBDIV_SHIFT
    | 0xb<<UTMI_PLL_REFDIV_SHIFT | 3<<UTMI_PLL_PLLVDD18_SHIFT
    | 3<<UTMI_PLL_PLLVDD12_SHIFT | 3<<UTMI_PLL_PLLCALI12_SHIFT
    | 1<<UTMI_PLL_ICP_SHIFT | 3<<UTMI_PLL_KVCO_SHIFT);
// UTMI_TX
    u2o_clear(base, UTMI_TX, UTMI_TX_REG_EXT_FS_RCAL_EN_MASK
    | UTMI_TX_TXVDD12_MASK | UTMI_TX_CK60_PHSEL_MASK
    | UTMI_TX_IMPCAL_VTH_MASK | UTMI_TX_REG_EXT_FS_RCAL_MASK
    | UTMI_TX_AMP_MASK);
    u2o_set(base, UTMI_TX, 3<<UTMI_TX_TXVDD12_SHIFT
    | 4<<UTMI_TX_CK60_PHSEL_SHIFT | 4<<UTMI_TX_IMPCAL_VTH_SHIFT
    | 8<<UTMI_TX_REG_EXT_FS_RCAL_SHIFT | 3<<UTMI_TX_AMP_SHIFT);
// UTMI_RX
    u2o_clear(base, UTMI_RX, UTMI_RX_SQ_THRESH_MASK
    | UTMI_REG_SQ_LENGTH_MASK);
    u2o_set(base, UTMI_RX, 7<<UTMI_RX_SQ_THRESH_SHIFT
    | 2<<UTMI_REG_SQ_LENGTH_SHIFT);
// UTMI_IVREF
    if (pxa_usb_phy.version == PXA_USB_PHY_PXA168) {
//
// fixing Microsoft Altair board interface with NEC hub issue -
// Set UTMI_IVREF from 0x4a3 to 0x4bf
//
    u2o_write(base, UTMI_IVREF, 0x4bf);
    }
// toggle VCOCAL_START bit of UTMI_PLL
    udelay(200);
    u2o_set(base, UTMI_PLL, VCOCAL_START);
    udelay(40);
    u2o_clear(base, UTMI_PLL, VCOCAL_START);
// toggle REG_RCAL_START bit of UTMI_TX
    udelay(400);
    u2o_set(base, UTMI_TX, REG_RCAL_START);
    udelay(40);
    u2o_clear(base, UTMI_TX, REG_RCAL_START);
    udelay(400);
// Make sure PHY PLL is ready
    loops = 0;
    while ((u2o_get(base, UTMI_PLL) & PLL_READY) == 0) {
    mdelay(1);
    loops++;
    if (loops > 100) {
    dev_warn(&phy.dev, "calibrate timeout, UTMI_PLL %x\n",
    u2o_get(base, UTMI_PLL));
    break;
    }
    }
    if (pxa_usb_phy.version == PXA_USB_PHY_PXA168) {
    u2o_set(base, UTMI_RESERVE, 1 << 5);
// Turn on UTMI PHY OTG extension
    u2o_write(base, UTMI_OTG_ADDON, 1);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa_usb_phy_exit(phy: *mut phy) -> c_int {
    static int pxa_usb_phy_exit(struct phy *phy)
    {
    struct pxa_usb_phy *pxa_usb_phy = phy_get_drvdata(phy);
    void __iomem *base = pxa_usb_phy.base;
    dev_info(&phy.dev, "deinitializing Marvell PXA USB PHY");
    if (pxa_usb_phy.version == PXA_USB_PHY_PXA168)
    u2o_clear(base, UTMI_OTG_ADDON, UTMI_OTG_ADDON_OTG_ON);
    u2o_clear(base, UTMI_CTRL, UTMI_CTRL_RXBUF_PDWN);
    u2o_clear(base, UTMI_CTRL, UTMI_CTRL_TXBUF_PDWN);
    u2o_clear(base, UTMI_CTRL, UTMI_CTRL_USB_CLK_EN);
    u2o_clear(base, UTMI_CTRL, 1<<UTMI_CTRL_PWR_UP_SHIFT);
    u2o_clear(base, UTMI_CTRL, 1<<UTMI_CTRL_PLL_PWR_UP_SHIFT);
    return 0;
    }
    static const struct phy_ops pxa_usb_phy_ops = {
    .init	= pxa_usb_phy_init,
    .exit	= pxa_usb_phy_exit,
    .owner	= THIS_MODULE,
    };
    static const struct of_device_id pxa_usb_phy_of_match[] = {
    {
    .compatible = "marvell,mmp2-usb-phy",
    .data = (void *)PXA_USB_PHY_MMP2,
    }, {
    .compatible = "marvell,pxa910-usb-phy",
    .data = (void *)PXA_USB_PHY_PXA910,
    }, {
    .compatible = "marvell,pxa168-usb-phy",
    .data = (void *)PXA_USB_PHY_PXA168,
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, pxa_usb_phy_of_match);
#[no_mangle]
unsafe extern "C" fn pxa_usb_phy_probe(pdev: *mut platform_device) -> c_int {
    static int pxa_usb_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct pxa_usb_phy *pxa_usb_phy;
    struct phy_provider *provider;
    const struct of_device_id *of_id;
    pxa_usb_phy = devm_kzalloc(dev, sizeof(struct pxa_usb_phy), GFP_KERNEL);
    if (!pxa_usb_phy)
    return -ENOMEM;
    of_id = of_match_node(pxa_usb_phy_of_match, dev.of_node);
    if (of_id)
    pxa_usb_phy.version = (uintptr_t)of_id.data;
    else
    pxa_usb_phy.version = PXA_USB_PHY_MMP2;
    pxa_usb_phy.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pxa_usb_phy.base)) {
    dev_err(dev, "failed to remap PHY regs\n");
    return PTR_ERR(pxa_usb_phy.base);
    }
    pxa_usb_phy.phy = devm_phy_create(dev, core::ptr::null_mut(), &pxa_usb_phy_ops);
    if (IS_ERR(pxa_usb_phy.phy)) {
    dev_err(dev, "failed to create PHY\n");
    return PTR_ERR(pxa_usb_phy.phy);
    }
    phy_set_drvdata(pxa_usb_phy.phy, pxa_usb_phy);
    provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(provider)) {
    dev_err(dev, "failed to register PHY provider\n");
    return PTR_ERR(provider);
    }
    if (!dev.of_node) {
    phy_create_lookup(pxa_usb_phy.phy, "usb", "mv-udc");
    phy_create_lookup(pxa_usb_phy.phy, "usb", "pxa-u2oehci");
    phy_create_lookup(pxa_usb_phy.phy, "usb", "mv-otg");
    }
    return 0;
    }
    static struct platform_driver pxa_usb_phy_driver = {
    .probe		= pxa_usb_phy_probe,
    .driver		= {
    .name	= "pxa-usb-phy",
    .of_match_table = pxa_usb_phy_of_match,
    },
    };
    module_platform_driver(pxa_usb_phy_driver);
    MODULE_AUTHOR("Lubomir Rintel <lkundrak@v3.sk>");
    MODULE_DESCRIPTION("Marvell PXA USB PHY Driver");
    MODULE_LICENSE("GPL v2");
