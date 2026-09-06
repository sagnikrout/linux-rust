//! Automatically rewritten from C to Rust
//! Source: drivers/phy/samsung/phy-exynos4x12-usb2.c
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
// Samsung SoC USB 1.1/2.0 PHY driver - Exynos 4x12 support
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// Author: Kamil Debski <k.debski@samsung.com>
//

// Exynos USB PHY registers
// PHY power control
pub const EXYNOS_4x12_UPHYPWR: c_uint = 0x0;

    EXYNOS_4x12_UPHYPWR_PHY0_SUSPEND | \
    EXYNOS_4x12_UPHYPWR_PHY0_PWR | \
    EXYNOS_4x12_UPHYPWR_PHY0_OTG_PWR | \
    EXYNOS_4x12_UPHYPWR_PHY0_SLEEP)

    EXYNOS_4x12_UPHYPWR_PHY1_SUSPEND | \
    EXYNOS_4x12_UPHYPWR_PHY1_PWR | \
    EXYNOS_4x12_UPHYPWR_PHY1_SLEEP)

    EXYNOS_4x12_UPHYPWR_HSIC0_SUSPEND | \
    EXYNOS_4x12_UPHYPWR_HSIC0_PWR | \
    EXYNOS_4x12_UPHYPWR_HSIC0_SLEEP)

    EXYNOS_4x12_UPHYPWR_HSIC1_SUSPEND | \
    EXYNOS_4x12_UPHYPWR_HSIC1_PWR | \
    EXYNOS_4x12_UPHYPWR_HSIC1_SLEEP)
// PHY clock control
pub const EXYNOS_4x12_UPHYCLK: c_uint = 0x4;

pub const EXYNOS_4x12_UPHYCLK_PHYFSEL_OFFSET: c_int = 0;

pub const EXYNOS_4x12_UPHYCLK_HSIC_REFCLK_OFFSET: c_int = 10;

// PHY reset control
pub const EXYNOS_4x12_UPHYRST: c_uint = 0x8;

// The following bit defines are presented in the
// order taken from the Exynos4412 reference manual.
//
// During experiments with the hardware and debugging
// it was determined that the hardware behaves contrary
// to the manual.
//
// The following bit values were chaned accordingly to the
// results of real hardware experiments.
//

// Isolation, configured in the power management unit
pub const EXYNOS_4x12_USB_ISOL_OFFSET: c_uint = 0x704;

pub const EXYNOS_4x12_USB_ISOL_HSIC0_OFFSET: c_uint = 0x708;

pub const EXYNOS_4x12_USB_ISOL_HSIC1_OFFSET: c_uint = 0x70c;

// Mode switching SUB Device <-> Host
pub const EXYNOS_4x12_MODE_SWITCH_OFFSET: c_uint = 0x21c;
pub const EXYNOS_4x12_MODE_SWITCH_MASK: c_int = 1;
pub const EXYNOS_4x12_MODE_SWITCH_DEVICE: c_int = 0;
pub const EXYNOS_4x12_MODE_SWITCH_HOST: c_int = 1;
    enum exynos4x12_phy_id {
    EXYNOS4x12_DEVICE,
    EXYNOS4x12_HOST,
    EXYNOS4x12_HSIC0,
    EXYNOS4x12_HSIC1,
    EXYNOS4x12_NUM_PHYS,
    };
//
// exynos4x12_rate_to_clk() converts the supplied clock rate to the value that
// can be written to the phy register.
//
#[no_mangle]
unsafe extern "C" fn exynos4x12_rate_to_clk(rate: c_ulong, reg: *mut u32) -> c_int {
    static int exynos4x12_rate_to_clk(unsigned long rate, u32 *reg)
    {
// EXYNOS_4x12_UPHYCLK_PHYFSEL_MASK
    switch (rate) {
    case 9600 * KHZ:
// reg = EXYNOS_4x12_UPHYCLK_PHYFSEL_9MHZ6;
    break;
    case 10 * MHZ:
// reg = EXYNOS_4x12_UPHYCLK_PHYFSEL_10MHZ;
    break;
    case 12 * MHZ:
// reg = EXYNOS_4x12_UPHYCLK_PHYFSEL_12MHZ;
    break;
    case 19200 * KHZ:
// reg = EXYNOS_4x12_UPHYCLK_PHYFSEL_19MHZ2;
    break;
    case 20 * MHZ:
// reg = EXYNOS_4x12_UPHYCLK_PHYFSEL_20MHZ;
    break;
    case 24 * MHZ:
// reg = EXYNOS_4x12_UPHYCLK_PHYFSEL_24MHZ;
    break;
    case 50 * MHZ:
// reg = EXYNOS_4x12_UPHYCLK_PHYFSEL_50MHZ;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos4x12_isol(inst: *mut samsung_usb2_phy_instance, on: bool) {
    static void exynos4x12_isol(struct samsung_usb2_phy_instance *inst, bool on)
    {
    struct samsung_usb2_phy_driver *drv = inst.drv;
    u32 offset;
    u32 mask;
    switch (inst.cfg.id) {
    case EXYNOS4x12_DEVICE:
    case EXYNOS4x12_HOST:
    offset = EXYNOS_4x12_USB_ISOL_OFFSET;
    mask = EXYNOS_4x12_USB_ISOL_OTG;
    break;
    case EXYNOS4x12_HSIC0:
    offset = EXYNOS_4x12_USB_ISOL_HSIC0_OFFSET;
    mask = EXYNOS_4x12_USB_ISOL_HSIC0;
    break;
    case EXYNOS4x12_HSIC1:
    offset = EXYNOS_4x12_USB_ISOL_HSIC1_OFFSET;
    mask = EXYNOS_4x12_USB_ISOL_HSIC1;
    break;
    default:
    return;
    }
    regmap_update_bits(drv.reg_pmu, offset, mask, on ? 0 : mask);
    }
#[no_mangle]
unsafe extern "C" fn exynos4x12_setup_clk(inst: *mut samsung_usb2_phy_instance) {
    static void exynos4x12_setup_clk(struct samsung_usb2_phy_instance *inst)
    {
    struct samsung_usb2_phy_driver *drv = inst.drv;
    u32 clk;
    clk = readl(drv.reg_phy + EXYNOS_4x12_UPHYCLK);
    clk &= ~EXYNOS_4x12_UPHYCLK_PHYFSEL_MASK;
    if (drv.cfg.has_refclk_sel)
    clk = EXYNOS_3250_UPHYCLK_REFCLKSEL;
    clk |= drv.ref_reg_val << EXYNOS_4x12_UPHYCLK_PHYFSEL_OFFSET;
    clk |= EXYNOS_4x12_UPHYCLK_PHY1_COMMON_ON;
    writel(clk, drv.reg_phy + EXYNOS_4x12_UPHYCLK);
    }
#[no_mangle]
unsafe extern "C" fn exynos4x12_phy_pwr(inst: *mut samsung_usb2_phy_instance, on: bool) {
    static void exynos4x12_phy_pwr(struct samsung_usb2_phy_instance *inst, bool on)
    {
    struct samsung_usb2_phy_driver *drv = inst.drv;
    let mut rstbits: u32 = 0;
    let mut phypwr: u32 = 0;
    u32 rst;
    u32 pwr;
    switch (inst.cfg.id) {
    case EXYNOS4x12_DEVICE:
    phypwr =	EXYNOS_4x12_UPHYPWR_PHY0;
    rstbits =	EXYNOS_4x12_URSTCON_PHY0;
    break;
    case EXYNOS4x12_HOST:
    phypwr =	EXYNOS_4x12_UPHYPWR_PHY1;
    rstbits =	EXYNOS_4x12_URSTCON_HOST_PHY |
    EXYNOS_4x12_URSTCON_PHY1 |
    EXYNOS_4x12_URSTCON_HOST_LINK_P0;
    break;
    case EXYNOS4x12_HSIC0:
    phypwr =	EXYNOS_4x12_UPHYPWR_HSIC0;
    rstbits =	EXYNOS_4x12_URSTCON_HSIC0 |
    EXYNOS_4x12_URSTCON_HOST_LINK_P1;
    break;
    case EXYNOS4x12_HSIC1:
    phypwr =	EXYNOS_4x12_UPHYPWR_HSIC1;
    rstbits =	EXYNOS_4x12_URSTCON_HSIC1 |
    EXYNOS_4x12_URSTCON_HOST_LINK_P1;
    break;
    }
    if (on) {
    pwr = readl(drv.reg_phy + EXYNOS_4x12_UPHYPWR);
    pwr &= ~phypwr;
    writel(pwr, drv.reg_phy + EXYNOS_4x12_UPHYPWR);
    rst = readl(drv.reg_phy + EXYNOS_4x12_UPHYRST);
    rst |= rstbits;
    writel(rst, drv.reg_phy + EXYNOS_4x12_UPHYRST);
    udelay(10);
    rst &= ~rstbits;
    writel(rst, drv.reg_phy + EXYNOS_4x12_UPHYRST);
// The following delay is necessary for the reset sequence to be
// completed
    udelay(80);
    } else {
    pwr = readl(drv.reg_phy + EXYNOS_4x12_UPHYPWR);
    pwr |= phypwr;
    writel(pwr, drv.reg_phy + EXYNOS_4x12_UPHYPWR);
    }
    }
#[no_mangle]
unsafe extern "C" fn exynos4x12_power_on_int(inst: *mut samsung_usb2_phy_instance) {
    static void exynos4x12_power_on_int(struct samsung_usb2_phy_instance *inst)
    {
    if (inst.int_cnt++ > 0)
    return;
    exynos4x12_setup_clk(inst);
    exynos4x12_isol(inst, 0);
    exynos4x12_phy_pwr(inst, 1);
    }
#[no_mangle]
unsafe extern "C" fn exynos4x12_power_on(inst: *mut samsung_usb2_phy_instance) -> c_int {
    static int exynos4x12_power_on(struct samsung_usb2_phy_instance *inst)
    {
    struct samsung_usb2_phy_driver *drv = inst.drv;
    if (inst.ext_cnt++ > 0)
    return 0;
    if (inst.cfg.id == EXYNOS4x12_HOST) {
    regmap_update_bits(drv.reg_sys, EXYNOS_4x12_MODE_SWITCH_OFFSET,
    EXYNOS_4x12_MODE_SWITCH_MASK,
    EXYNOS_4x12_MODE_SWITCH_HOST);
    exynos4x12_power_on_int(&drv.instances[EXYNOS4x12_DEVICE]);
    }
    if (inst.cfg.id == EXYNOS4x12_DEVICE && drv.cfg.has_mode_switch)
    regmap_update_bits(drv.reg_sys, EXYNOS_4x12_MODE_SWITCH_OFFSET,
    EXYNOS_4x12_MODE_SWITCH_MASK,
    EXYNOS_4x12_MODE_SWITCH_DEVICE);
    if (inst.cfg.id == EXYNOS4x12_HSIC0 ||
    inst.cfg.id == EXYNOS4x12_HSIC1) {
    exynos4x12_power_on_int(&drv.instances[EXYNOS4x12_DEVICE]);
    exynos4x12_power_on_int(&drv.instances[EXYNOS4x12_HOST]);
    }
    exynos4x12_power_on_int(inst);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos4x12_power_off_int(inst: *mut samsung_usb2_phy_instance) {
    static void exynos4x12_power_off_int(struct samsung_usb2_phy_instance *inst)
    {
    if (inst.int_cnt-- > 1)
    return;
    exynos4x12_isol(inst, 1);
    exynos4x12_phy_pwr(inst, 0);
    }
#[no_mangle]
unsafe extern "C" fn exynos4x12_power_off(inst: *mut samsung_usb2_phy_instance) -> c_int {
    static int exynos4x12_power_off(struct samsung_usb2_phy_instance *inst)
    {
    struct samsung_usb2_phy_driver *drv = inst.drv;
    if (inst.ext_cnt-- > 1)
    return 0;
    if (inst.cfg.id == EXYNOS4x12_DEVICE && drv.cfg.has_mode_switch)
    regmap_update_bits(drv.reg_sys, EXYNOS_4x12_MODE_SWITCH_OFFSET,
    EXYNOS_4x12_MODE_SWITCH_MASK,
    EXYNOS_4x12_MODE_SWITCH_HOST);
    if (inst.cfg.id == EXYNOS4x12_HOST)
    exynos4x12_power_off_int(&drv.instances[EXYNOS4x12_DEVICE]);
    if (inst.cfg.id == EXYNOS4x12_HSIC0 ||
    inst.cfg.id == EXYNOS4x12_HSIC1) {
    exynos4x12_power_off_int(&drv.instances[EXYNOS4x12_DEVICE]);
    exynos4x12_power_off_int(&drv.instances[EXYNOS4x12_HOST]);
    }
    exynos4x12_power_off_int(inst);
    return 0;
    }
    static const struct samsung_usb2_common_phy exynos4x12_phys[] = {
    {
    .label		= "device",
    .id		= EXYNOS4x12_DEVICE,
    .power_on	= exynos4x12_power_on,
    .power_off	= exynos4x12_power_off,
    },
    {
    .label		= "host",
    .id		= EXYNOS4x12_HOST,
    .power_on	= exynos4x12_power_on,
    .power_off	= exynos4x12_power_off,
    },
    {
    .label		= "hsic0",
    .id		= EXYNOS4x12_HSIC0,
    .power_on	= exynos4x12_power_on,
    .power_off	= exynos4x12_power_off,
    },
    {
    .label		= "hsic1",
    .id		= EXYNOS4x12_HSIC1,
    .power_on	= exynos4x12_power_on,
    .power_off	= exynos4x12_power_off,
    },
    };
    const struct samsung_usb2_phy_config exynos3250_usb2_phy_config = {
    .has_refclk_sel		= 1,
    .num_phys		= 1,
    .phys			= exynos4x12_phys,
    .rate_to_clk		= exynos4x12_rate_to_clk,
    };
    const struct samsung_usb2_phy_config exynos4x12_usb2_phy_config = {
    .has_mode_switch	= 1,
    .num_phys		= EXYNOS4x12_NUM_PHYS,
    .phys			= exynos4x12_phys,
    .rate_to_clk		= exynos4x12_rate_to_clk,
    };
