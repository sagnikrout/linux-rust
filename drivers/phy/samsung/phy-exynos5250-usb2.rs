//! Automatically rewritten from C to Rust
//! Source: drivers/phy/samsung/phy-exynos5250-usb2.c
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
// Samsung SoC USB 1.1/2.0 PHY driver - Exynos 5250 support
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// Author: Kamil Debski <k.debski@samsung.com>
//

// Exynos USB PHY registers
pub const EXYNOS_5250_REFCLKSEL_CRYSTAL: c_uint = 0x0;
pub const EXYNOS_5250_REFCLKSEL_XO: c_uint = 0x1;
pub const EXYNOS_5250_REFCLKSEL_CLKCORE: c_uint = 0x2;
pub const EXYNOS_5250_FSEL_9MHZ6: c_uint = 0x0;
pub const EXYNOS_5250_FSEL_10MHZ: c_uint = 0x1;
pub const EXYNOS_5250_FSEL_12MHZ: c_uint = 0x2;
pub const EXYNOS_5250_FSEL_19MHZ2: c_uint = 0x3;
pub const EXYNOS_5250_FSEL_20MHZ: c_uint = 0x4;
pub const EXYNOS_5250_FSEL_24MHZ: c_uint = 0x5;
pub const EXYNOS_5250_FSEL_50MHZ: c_uint = 0x7;
// Normal host
pub const EXYNOS_5250_HOSTPHYCTRL0: c_uint = 0x0;

pub const EXYNOS_5250_HOSTPHYCTRL0_REFCLKSEL_SHIFT: c_int = 19;

    (0x3 << EXYNOS_5250_HOSTPHYCTRL0_REFCLKSEL_SHIFT)
pub const EXYNOS_5250_HOSTPHYCTRL0_FSEL_SHIFT: c_int = 16;

    (0x7 << EXYNOS_5250_HOSTPHYCTRL0_FSEL_SHIFT)

// HSIC0 & HSIC1
pub const EXYNOS_5250_HSICPHYCTRL1: c_uint = 0x10;
pub const EXYNOS_5250_HSICPHYCTRL2: c_uint = 0x20;

// EHCI control
pub const EXYNOS_5250_HOSTEHCICTRL: c_uint = 0x30;

pub const EXYNOS_5250_HOSTEHCICTRL_FLADJVAL0_SHIFT: c_int = 19;

    (0x3f << EXYNOS_5250_HOSTEHCICTRL_FLADJVAL0_SHIFT)
pub const EXYNOS_5250_HOSTEHCICTRL_FLADJVAL1_SHIFT: c_int = 13;

    (0x3f << EXYNOS_5250_HOSTEHCICTRL_FLADJVAL1_SHIFT)
pub const EXYNOS_5250_HOSTEHCICTRL_FLADJVAL2_SHIFT: c_int = 7;

    (0x3f << EXYNOS_5250_HOSTEHCICTRL_FLADJVAL0_SHIFT)
pub const EXYNOS_5250_HOSTEHCICTRL_FLADJVALHOST_SHIFT: c_int = 1;

    (0x1 << EXYNOS_5250_HOSTEHCICTRL_FLADJVALHOST_SHIFT)

// OHCI control
pub const EXYNOS_5250_HOSTOHCICTRL: c_uint = 0x34;
pub const EXYNOS_5250_HOSTOHCICTRL_FRAMELENVAL_SHIFT: c_int = 1;

    (0x3ff << EXYNOS_5250_HOSTOHCICTRL_FRAMELENVAL_SHIFT)

// USBOTG
pub const EXYNOS_5250_USBOTGSYS: c_uint = 0x38;

pub const EXYNOS_5250_USBOTGSYS_REFCLKSEL_SHIFT: c_int = 9;

    (0x3 << EXYNOS_5250_USBOTGSYS_REFCLKSEL_SHIFT)

pub const EXYNOS_5250_USBOTGSYS_FSEL_SHIFT: c_int = 4;

    (0x3 << EXYNOS_5250_USBOTGSYS_FSEL_SHIFT)

// Isolation, configured in the power management unit
pub const EXYNOS_5250_USB_ISOL_OTG_OFFSET: c_uint = 0x704;
pub const EXYNOS_5250_USB_ISOL_HOST_OFFSET: c_uint = 0x708;
pub const EXYNOS_5420_USB_ISOL_HOST_OFFSET: c_uint = 0x70C;

// Mode switch register
pub const EXYNOS_5250_MODE_SWITCH_OFFSET: c_uint = 0x230;
pub const EXYNOS_5250_MODE_SWITCH_MASK: c_int = 1;
pub const EXYNOS_5250_MODE_SWITCH_DEVICE: c_int = 0;
pub const EXYNOS_5250_MODE_SWITCH_HOST: c_int = 1;
    enum exynos4x12_phy_id {
    EXYNOS5250_DEVICE,
    EXYNOS5250_HOST,
    EXYNOS5250_HSIC0,
    EXYNOS5250_HSIC1,
    };
//
// exynos5250_rate_to_clk() converts the supplied clock rate to the value that
// can be written to the phy register.
//
#[no_mangle]
unsafe extern "C" fn exynos5250_rate_to_clk(rate: c_ulong, reg: *mut u32) -> c_int {
    static int exynos5250_rate_to_clk(unsigned long rate, u32 *reg)
    {
// EXYNOS_5250_FSEL_MASK
    switch (rate) {
    case 9600 * KHZ:
// reg = EXYNOS_5250_FSEL_9MHZ6;
    break;
    case 10 * MHZ:
// reg = EXYNOS_5250_FSEL_10MHZ;
    break;
    case 12 * MHZ:
// reg = EXYNOS_5250_FSEL_12MHZ;
    break;
    case 19200 * KHZ:
// reg = EXYNOS_5250_FSEL_19MHZ2;
    break;
    case 20 * MHZ:
// reg = EXYNOS_5250_FSEL_20MHZ;
    break;
    case 24 * MHZ:
// reg = EXYNOS_5250_FSEL_24MHZ;
    break;
    case 50 * MHZ:
// reg = EXYNOS_5250_FSEL_50MHZ;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos5250_isol(inst: *mut samsung_usb2_phy_instance, on: bool) {
    static void exynos5250_isol(struct samsung_usb2_phy_instance *inst, bool on)
    {
    struct samsung_usb2_phy_driver *drv = inst.drv;
    u32 offset;
    let mut mask: u32 = EXYNOS_5250_USB_ISOL_ENABLE;
    if (drv.cfg == &exynos5250_usb2_phy_config &&
    inst.cfg.id == EXYNOS5250_DEVICE)
    offset = EXYNOS_5250_USB_ISOL_OTG_OFFSET;
    else if (drv.cfg == &exynos5250_usb2_phy_config &&
    inst.cfg.id == EXYNOS5250_HOST)
    offset = EXYNOS_5250_USB_ISOL_HOST_OFFSET;
    else if (drv.cfg == &exynos5420_usb2_phy_config &&
    inst.cfg.id == EXYNOS5250_HOST)
    offset = EXYNOS_5420_USB_ISOL_HOST_OFFSET;
    else
    return;
    regmap_update_bits(drv.reg_pmu, offset, mask, on ? 0 : mask);
    }
#[no_mangle]
unsafe extern "C" fn exynos5250_power_on(inst: *mut samsung_usb2_phy_instance) -> c_int {
    static int exynos5250_power_on(struct samsung_usb2_phy_instance *inst)
    {
    struct samsung_usb2_phy_driver *drv = inst.drv;
    u32 ctrl0;
    u32 otg;
    u32 ehci;
    u32 ohci;
    u32 hsic;
    switch (inst.cfg.id) {
    case EXYNOS5250_DEVICE:
    regmap_update_bits(drv.reg_sys,
    EXYNOS_5250_MODE_SWITCH_OFFSET,
    EXYNOS_5250_MODE_SWITCH_MASK,
    EXYNOS_5250_MODE_SWITCH_DEVICE);
// OTG configuration
    otg = readl(drv.reg_phy + EXYNOS_5250_USBOTGSYS);
// The clock
    otg &= ~EXYNOS_5250_USBOTGSYS_FSEL_MASK;
    otg |= drv.ref_reg_val << EXYNOS_5250_USBOTGSYS_FSEL_SHIFT;
// Reset
    otg &= ~(EXYNOS_5250_USBOTGSYS_FORCE_SUSPEND |
    EXYNOS_5250_USBOTGSYS_FORCE_SLEEP |
    EXYNOS_5250_USBOTGSYS_SIDDQ_UOTG);
    otg |=	EXYNOS_5250_USBOTGSYS_PHY_SW_RST |
    EXYNOS_5250_USBOTGSYS_PHYLINK_SW_RESET |
    EXYNOS_5250_USBOTGSYS_LINK_SW_RST_UOTG |
    EXYNOS_5250_USBOTGSYS_OTGDISABLE;
// Ref clock
    otg &=	~EXYNOS_5250_USBOTGSYS_REFCLKSEL_MASK;
    otg |=  EXYNOS_5250_REFCLKSEL_CLKCORE <<
    EXYNOS_5250_USBOTGSYS_REFCLKSEL_SHIFT;
    writel(otg, drv.reg_phy + EXYNOS_5250_USBOTGSYS);
    udelay(100);
    otg &= ~(EXYNOS_5250_USBOTGSYS_PHY_SW_RST |
    EXYNOS_5250_USBOTGSYS_LINK_SW_RST_UOTG |
    EXYNOS_5250_USBOTGSYS_PHYLINK_SW_RESET |
    EXYNOS_5250_USBOTGSYS_OTGDISABLE);
    writel(otg, drv.reg_phy + EXYNOS_5250_USBOTGSYS);
    break;
    case EXYNOS5250_HOST:
    case EXYNOS5250_HSIC0:
    case EXYNOS5250_HSIC1:
// Host registers configuration
    ctrl0 = readl(drv.reg_phy + EXYNOS_5250_HOSTPHYCTRL0);
// The clock
    ctrl0 &= ~EXYNOS_5250_HOSTPHYCTRL0_FSEL_MASK;
    ctrl0 |= drv.ref_reg_val <<
    EXYNOS_5250_HOSTPHYCTRL0_FSEL_SHIFT;
// Reset
    ctrl0 &=	~(EXYNOS_5250_HOSTPHYCTRL0_PHYSWRST |
    EXYNOS_5250_HOSTPHYCTRL0_PHYSWRSTALL |
    EXYNOS_5250_HOSTPHYCTRL0_SIDDQ |
    EXYNOS_5250_HOSTPHYCTRL0_FORCESUSPEND |
    EXYNOS_5250_HOSTPHYCTRL0_FORCESLEEP);
    ctrl0 |=	EXYNOS_5250_HOSTPHYCTRL0_LINKSWRST |
    EXYNOS_5250_HOSTPHYCTRL0_UTMISWRST |
    EXYNOS_5250_HOSTPHYCTRL0_COMMON_ON_N;
    writel(ctrl0, drv.reg_phy + EXYNOS_5250_HOSTPHYCTRL0);
    udelay(10);
    ctrl0 &=	~(EXYNOS_5250_HOSTPHYCTRL0_LINKSWRST |
    EXYNOS_5250_HOSTPHYCTRL0_UTMISWRST);
    writel(ctrl0, drv.reg_phy + EXYNOS_5250_HOSTPHYCTRL0);
// OTG configuration
    otg = readl(drv.reg_phy + EXYNOS_5250_USBOTGSYS);
// The clock
    otg &= ~EXYNOS_5250_USBOTGSYS_FSEL_MASK;
    otg |= drv.ref_reg_val << EXYNOS_5250_USBOTGSYS_FSEL_SHIFT;
// Reset
    otg &= ~(EXYNOS_5250_USBOTGSYS_FORCE_SUSPEND |
    EXYNOS_5250_USBOTGSYS_FORCE_SLEEP |
    EXYNOS_5250_USBOTGSYS_SIDDQ_UOTG);
    otg |=	EXYNOS_5250_USBOTGSYS_PHY_SW_RST |
    EXYNOS_5250_USBOTGSYS_PHYLINK_SW_RESET |
    EXYNOS_5250_USBOTGSYS_LINK_SW_RST_UOTG |
    EXYNOS_5250_USBOTGSYS_OTGDISABLE;
// Ref clock
    otg &=	~EXYNOS_5250_USBOTGSYS_REFCLKSEL_MASK;
    otg |=  EXYNOS_5250_REFCLKSEL_CLKCORE <<
    EXYNOS_5250_USBOTGSYS_REFCLKSEL_SHIFT;
    writel(otg, drv.reg_phy + EXYNOS_5250_USBOTGSYS);
    udelay(10);
    otg &= ~(EXYNOS_5250_USBOTGSYS_PHY_SW_RST |
    EXYNOS_5250_USBOTGSYS_LINK_SW_RST_UOTG |
    EXYNOS_5250_USBOTGSYS_PHYLINK_SW_RESET);
// HSIC phy configuration
    hsic = (EXYNOS_5250_HSICPHYCTRLX_REFCLKDIV_12 |
    EXYNOS_5250_HSICPHYCTRLX_REFCLKSEL_DEFAULT |
    EXYNOS_5250_HSICPHYCTRLX_PHYSWRST);
    writel(hsic, drv.reg_phy + EXYNOS_5250_HSICPHYCTRL1);
    writel(hsic, drv.reg_phy + EXYNOS_5250_HSICPHYCTRL2);
    udelay(10);
    hsic &= ~EXYNOS_5250_HSICPHYCTRLX_PHYSWRST;
    writel(hsic, drv.reg_phy + EXYNOS_5250_HSICPHYCTRL1);
    writel(hsic, drv.reg_phy + EXYNOS_5250_HSICPHYCTRL2);
// The following delay is necessary for the reset sequence to be
// completed
    udelay(80);
// Enable EHCI DMA burst
    ehci = readl(drv.reg_phy + EXYNOS_5250_HOSTEHCICTRL);
    ehci |=	EXYNOS_5250_HOSTEHCICTRL_ENAINCRXALIGN |
    EXYNOS_5250_HOSTEHCICTRL_ENAINCR4 |
    EXYNOS_5250_HOSTEHCICTRL_ENAINCR8 |
    EXYNOS_5250_HOSTEHCICTRL_ENAINCR16;
    writel(ehci, drv.reg_phy + EXYNOS_5250_HOSTEHCICTRL);
// OHCI settings
    ohci = readl(drv.reg_phy + EXYNOS_5250_HOSTOHCICTRL);
// Following code is based on the old driver
    ohci |=	0x1 << 3;
    writel(ohci, drv.reg_phy + EXYNOS_5250_HOSTOHCICTRL);
    break;
    }
    exynos5250_isol(inst, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos5250_power_off(inst: *mut samsung_usb2_phy_instance) -> c_int {
    static int exynos5250_power_off(struct samsung_usb2_phy_instance *inst)
    {
    struct samsung_usb2_phy_driver *drv = inst.drv;
    u32 ctrl0;
    u32 otg;
    u32 hsic;
    exynos5250_isol(inst, 1);
    switch (inst.cfg.id) {
    case EXYNOS5250_DEVICE:
    otg = readl(drv.reg_phy + EXYNOS_5250_USBOTGSYS);
    otg |= (EXYNOS_5250_USBOTGSYS_FORCE_SUSPEND |
    EXYNOS_5250_USBOTGSYS_SIDDQ_UOTG |
    EXYNOS_5250_USBOTGSYS_FORCE_SLEEP);
    writel(otg, drv.reg_phy + EXYNOS_5250_USBOTGSYS);
    break;
    case EXYNOS5250_HOST:
    ctrl0 = readl(drv.reg_phy + EXYNOS_5250_HOSTPHYCTRL0);
    ctrl0 |= (EXYNOS_5250_HOSTPHYCTRL0_SIDDQ |
    EXYNOS_5250_HOSTPHYCTRL0_FORCESUSPEND |
    EXYNOS_5250_HOSTPHYCTRL0_FORCESLEEP |
    EXYNOS_5250_HOSTPHYCTRL0_PHYSWRST |
    EXYNOS_5250_HOSTPHYCTRL0_PHYSWRSTALL);
    writel(ctrl0, drv.reg_phy + EXYNOS_5250_HOSTPHYCTRL0);
    break;
    case EXYNOS5250_HSIC0:
    case EXYNOS5250_HSIC1:
    hsic = (EXYNOS_5250_HSICPHYCTRLX_REFCLKDIV_12 |
    EXYNOS_5250_HSICPHYCTRLX_REFCLKSEL_DEFAULT |
    EXYNOS_5250_HSICPHYCTRLX_SIDDQ |
    EXYNOS_5250_HSICPHYCTRLX_FORCESLEEP |
    EXYNOS_5250_HSICPHYCTRLX_FORCESUSPEND
    );
    writel(hsic, drv.reg_phy + EXYNOS_5250_HSICPHYCTRL1);
    writel(hsic, drv.reg_phy + EXYNOS_5250_HSICPHYCTRL2);
    break;
    }
    return 0;
    }
    static const struct samsung_usb2_common_phy exynos5250_phys[] = {
    {
    .label		= "device",
    .id		= EXYNOS5250_DEVICE,
    .power_on	= exynos5250_power_on,
    .power_off	= exynos5250_power_off,
    },
    {
    .label		= "host",
    .id		= EXYNOS5250_HOST,
    .power_on	= exynos5250_power_on,
    .power_off	= exynos5250_power_off,
    },
    {
    .label		= "hsic0",
    .id		= EXYNOS5250_HSIC0,
    .power_on	= exynos5250_power_on,
    .power_off	= exynos5250_power_off,
    },
    {
    .label		= "hsic1",
    .id		= EXYNOS5250_HSIC1,
    .power_on	= exynos5250_power_on,
    .power_off	= exynos5250_power_off,
    },
    };
    static const struct samsung_usb2_common_phy exynos5420_phys[] = {
    {
    .label		= "host",
    .id		= EXYNOS5250_HOST,
    .power_on	= exynos5250_power_on,
    .power_off	= exynos5250_power_off,
    },
    {
    .label		= "hsic",
    .id		= EXYNOS5250_HSIC0,
    .power_on	= exynos5250_power_on,
    .power_off	= exynos5250_power_off,
    },
    };
    const struct samsung_usb2_phy_config exynos5250_usb2_phy_config = {
    .has_mode_switch	= 1,
    .num_phys		= ARRAY_SIZE(exynos5250_phys),
    .phys			= exynos5250_phys,
    .rate_to_clk		= exynos5250_rate_to_clk,
    };
    const struct samsung_usb2_phy_config exynos5420_usb2_phy_config = {
    .has_mode_switch	= 1,
    .num_phys		= ARRAY_SIZE(exynos5420_phys),
    .phys			= exynos5420_phys,
    .rate_to_clk		= exynos5250_rate_to_clk,
    };
