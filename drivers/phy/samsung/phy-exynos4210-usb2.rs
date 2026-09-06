//! Automatically rewritten from C to Rust
//! Source: drivers/phy/samsung/phy-exynos4210-usb2.c
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
// Samsung SoC USB 1.1/2.0 PHY driver - Exynos 4210 support
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// Author: Kamil Debski <k.debski@samsung.com>
//

// Exynos USB PHY registers
// PHY power control
pub const EXYNOS_4210_UPHYPWR: c_uint = 0x0;

    EXYNOS_4210_UPHYPWR_PHY0_SUSPEND | \
    EXYNOS_4210_UPHYPWR_PHY0_PWR | \
    EXYNOS_4210_UPHYPWR_PHY0_OTG_PWR | \
    EXYNOS_4210_UPHYPWR_PHY0_SLEEP)

    EXYNOS_4210_UPHYPWR_PHY1_SUSPEND | \
    EXYNOS_4210_UPHYPWR_PHY1_PWR | \
    EXYNOS_4210_UPHYPWR_PHY1_SLEEP)

    EXYNOS_4210_UPHYPWR_HSIC0_SUSPEND | \
    EXYNOS_4210_UPHYPWR_HSIC0_SLEEP)

    EXYNOS_4210_UPHYPWR_HSIC1_SUSPEND | \
    EXYNOS_4210_UPHYPWR_HSIC1_SLEEP)
// PHY clock control
pub const EXYNOS_4210_UPHYCLK: c_uint = 0x4;

pub const EXYNOS_4210_UPHYCLK_PHYFSEL_OFFSET: c_int = 0;

// PHY reset control
pub const EXYNOS_4210_UPHYRST: c_uint = 0x8;

// Isolation, configured in the power management unit
pub const EXYNOS_4210_USB_ISOL_DEVICE_OFFSET: c_uint = 0x704;

pub const EXYNOS_4210_USB_ISOL_HOST_OFFSET: c_uint = 0x708;

// USBYPHY1 Floating prevention
pub const EXYNOS_4210_UPHY1CON: c_uint = 0x34;
pub const EXYNOS_4210_UPHY1CON_FLOAT_PREVENTION: c_uint = 0x1;
// Mode switching SUB Device <-> Host
pub const EXYNOS_4210_MODE_SWITCH_OFFSET: c_uint = 0x21c;
pub const EXYNOS_4210_MODE_SWITCH_MASK: c_int = 1;
pub const EXYNOS_4210_MODE_SWITCH_DEVICE: c_int = 0;
pub const EXYNOS_4210_MODE_SWITCH_HOST: c_int = 1;
    enum exynos4210_phy_id {
    EXYNOS4210_DEVICE,
    EXYNOS4210_HOST,
    EXYNOS4210_HSIC0,
    EXYNOS4210_HSIC1,
    EXYNOS4210_NUM_PHYS,
    };
//
// exynos4210_rate_to_clk() converts the supplied clock rate to the value that
// can be written to the phy register.
//
#[no_mangle]
unsafe extern "C" fn exynos4210_rate_to_clk(rate: c_ulong, reg: *mut u32) -> c_int {
    static int exynos4210_rate_to_clk(unsigned long rate, u32 *reg)
    {
    switch (rate) {
    case 12 * MHZ:
// reg = EXYNOS_4210_UPHYCLK_PHYFSEL_12MHZ;
    break;
    case 24 * MHZ:
// reg = EXYNOS_4210_UPHYCLK_PHYFSEL_24MHZ;
    break;
    case 48 * MHZ:
// reg = EXYNOS_4210_UPHYCLK_PHYFSEL_48MHZ;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos4210_isol(inst: *mut samsung_usb2_phy_instance, on: bool) {
    static void exynos4210_isol(struct samsung_usb2_phy_instance *inst, bool on)
    {
    struct samsung_usb2_phy_driver *drv = inst.drv;
    u32 offset;
    u32 mask;
    switch (inst.cfg.id) {
    case EXYNOS4210_DEVICE:
    offset = EXYNOS_4210_USB_ISOL_DEVICE_OFFSET;
    mask = EXYNOS_4210_USB_ISOL_DEVICE;
    break;
    case EXYNOS4210_HOST:
    offset = EXYNOS_4210_USB_ISOL_HOST_OFFSET;
    mask = EXYNOS_4210_USB_ISOL_HOST;
    break;
    default:
    return;
    }
    regmap_update_bits(drv.reg_pmu, offset, mask, on ? 0 : mask);
    }
#[no_mangle]
unsafe extern "C" fn exynos4210_phy_pwr(inst: *mut samsung_usb2_phy_instance, on: bool) {
    static void exynos4210_phy_pwr(struct samsung_usb2_phy_instance *inst, bool on)
    {
    struct samsung_usb2_phy_driver *drv = inst.drv;
    let mut rstbits: u32 = 0;
    let mut phypwr: u32 = 0;
    u32 rst;
    u32 pwr;
    u32 clk;
    switch (inst.cfg.id) {
    case EXYNOS4210_DEVICE:
    phypwr =	EXYNOS_4210_UPHYPWR_PHY0;
    rstbits =	EXYNOS_4210_URSTCON_PHY0;
    break;
    case EXYNOS4210_HOST:
    phypwr =	EXYNOS_4210_UPHYPWR_PHY1;
    rstbits =	EXYNOS_4210_URSTCON_PHY1_ALL |
    EXYNOS_4210_URSTCON_PHY1_P0 |
    EXYNOS_4210_URSTCON_PHY1_P1P2 |
    EXYNOS_4210_URSTCON_HOST_LINK_ALL |
    EXYNOS_4210_URSTCON_HOST_LINK_P0;
    writel(on, drv.reg_phy + EXYNOS_4210_UPHY1CON);
    break;
    case EXYNOS4210_HSIC0:
    phypwr =	EXYNOS_4210_UPHYPWR_HSIC0;
    rstbits =	EXYNOS_4210_URSTCON_PHY1_P1P2 |
    EXYNOS_4210_URSTCON_HOST_LINK_P1;
    break;
    case EXYNOS4210_HSIC1:
    phypwr =	EXYNOS_4210_UPHYPWR_HSIC1;
    rstbits =	EXYNOS_4210_URSTCON_PHY1_P1P2 |
    EXYNOS_4210_URSTCON_HOST_LINK_P2;
    break;
    }
    if (on) {
    clk = readl(drv.reg_phy + EXYNOS_4210_UPHYCLK);
    clk &= ~EXYNOS_4210_UPHYCLK_PHYFSEL_MASK;
    clk |= drv.ref_reg_val << EXYNOS_4210_UPHYCLK_PHYFSEL_OFFSET;
    writel(clk, drv.reg_phy + EXYNOS_4210_UPHYCLK);
    pwr = readl(drv.reg_phy + EXYNOS_4210_UPHYPWR);
    pwr &= ~phypwr;
    writel(pwr, drv.reg_phy + EXYNOS_4210_UPHYPWR);
    rst = readl(drv.reg_phy + EXYNOS_4210_UPHYRST);
    rst |= rstbits;
    writel(rst, drv.reg_phy + EXYNOS_4210_UPHYRST);
    udelay(10);
    rst &= ~rstbits;
    writel(rst, drv.reg_phy + EXYNOS_4210_UPHYRST);
// The following delay is necessary for the reset sequence to be
// completed
    udelay(80);
    } else {
    pwr = readl(drv.reg_phy + EXYNOS_4210_UPHYPWR);
    pwr |= phypwr;
    writel(pwr, drv.reg_phy + EXYNOS_4210_UPHYPWR);
    }
    }
#[no_mangle]
unsafe extern "C" fn exynos4210_power_on(inst: *mut samsung_usb2_phy_instance) -> c_int {
    static int exynos4210_power_on(struct samsung_usb2_phy_instance *inst)
    {
// Order of initialisation is important - first power then isolation
    exynos4210_phy_pwr(inst, 1);
    exynos4210_isol(inst, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos4210_power_off(inst: *mut samsung_usb2_phy_instance) -> c_int {
    static int exynos4210_power_off(struct samsung_usb2_phy_instance *inst)
    {
    exynos4210_isol(inst, 1);
    exynos4210_phy_pwr(inst, 0);
    return 0;
    }
    static const struct samsung_usb2_common_phy exynos4210_phys[] = {
    {
    .label		= "device",
    .id		= EXYNOS4210_DEVICE,
    .power_on	= exynos4210_power_on,
    .power_off	= exynos4210_power_off,
    },
    {
    .label		= "host",
    .id		= EXYNOS4210_HOST,
    .power_on	= exynos4210_power_on,
    .power_off	= exynos4210_power_off,
    },
    {
    .label		= "hsic0",
    .id		= EXYNOS4210_HSIC0,
    .power_on	= exynos4210_power_on,
    .power_off	= exynos4210_power_off,
    },
    {
    .label		= "hsic1",
    .id		= EXYNOS4210_HSIC1,
    .power_on	= exynos4210_power_on,
    .power_off	= exynos4210_power_off,
    },
    };
    const struct samsung_usb2_phy_config exynos4210_usb2_phy_config = {
    .has_mode_switch	= 0,
    .num_phys		= EXYNOS4210_NUM_PHYS,
    .phys			= exynos4210_phys,
    .rate_to_clk		= exynos4210_rate_to_clk,
    };
