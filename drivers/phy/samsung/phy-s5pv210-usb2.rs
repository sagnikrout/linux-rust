//! Automatically rewritten from C to Rust
//! Source: drivers/phy/samsung/phy-s5pv210-usb2.c
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
// Samsung SoC USB 1.1/2.0 PHY driver - S5PV210 support
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// Authors: Kamil Debski <k.debski@samsung.com>
//

// Exynos USB PHY registers
// PHY power control
pub const S5PV210_UPHYPWR: c_uint = 0x0;

    S5PV210_UPHYPWR_PHY0_SUSPEND | \
    S5PV210_UPHYPWR_PHY0_PWR | \
    S5PV210_UPHYPWR_PHY0_OTG_PWR)

    S5PV210_UPHYPWR_PHY1_SUSPEND | \
    S5PV210_UPHYPWR_PHY1_PWR)
// PHY clock control
pub const S5PV210_UPHYCLK: c_uint = 0x4;

// PHY reset control
pub const S5PV210_UPHYRST: c_uint = 0x8;

// Isolation, configured in the power management unit
pub const S5PV210_USB_ISOL_OFFSET: c_uint = 0x680c;

    enum s5pv210_phy_id {
    S5PV210_DEVICE,
    S5PV210_HOST,
    S5PV210_NUM_PHYS,
    };
//
// s5pv210_rate_to_clk() converts the supplied clock rate to the value that
// can be written to the phy register.
//
#[no_mangle]
unsafe extern "C" fn s5pv210_rate_to_clk(rate: c_ulong, reg: *mut u32) -> c_int {
    static int s5pv210_rate_to_clk(unsigned long rate, u32 *reg)
    {
    switch (rate) {
    case 12 * MHZ:
// reg = S5PV210_UPHYCLK_PHYFSEL_12MHZ;
    break;
    case 24 * MHZ:
// reg = S5PV210_UPHYCLK_PHYFSEL_24MHZ;
    break;
    case 48 * MHZ:
// reg = S5PV210_UPHYCLK_PHYFSEL_48MHZ;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s5pv210_isol(inst: *mut samsung_usb2_phy_instance, on: bool) {
    static void s5pv210_isol(struct samsung_usb2_phy_instance *inst, bool on)
    {
    struct samsung_usb2_phy_driver *drv = inst.drv;
    u32 mask;
    switch (inst.cfg.id) {
    case S5PV210_DEVICE:
    mask = S5PV210_USB_ISOL_DEVICE;
    break;
    case S5PV210_HOST:
    mask = S5PV210_USB_ISOL_HOST;
    break;
    default:
    return;
    }
    regmap_update_bits(drv.reg_pmu, S5PV210_USB_ISOL_OFFSET,
    mask, on ? 0 : mask);
    }
#[no_mangle]
unsafe extern "C" fn s5pv210_phy_pwr(inst: *mut samsung_usb2_phy_instance, on: bool) {
    static void s5pv210_phy_pwr(struct samsung_usb2_phy_instance *inst, bool on)
    {
    struct samsung_usb2_phy_driver *drv = inst.drv;
    let mut rstbits: u32 = 0;
    let mut phypwr: u32 = 0;
    u32 rst;
    u32 pwr;
    switch (inst.cfg.id) {
    case S5PV210_DEVICE:
    phypwr =	S5PV210_UPHYPWR_PHY0;
    rstbits =	S5PV210_URSTCON_PHY0;
    break;
    case S5PV210_HOST:
    phypwr =	S5PV210_UPHYPWR_PHY1;
    rstbits =	S5PV210_URSTCON_PHY1_ALL |
    S5PV210_URSTCON_HOST_LINK_ALL;
    break;
    }
    if (on) {
    writel(drv.ref_reg_val, drv.reg_phy + S5PV210_UPHYCLK);
    pwr = readl(drv.reg_phy + S5PV210_UPHYPWR);
    pwr &= ~phypwr;
    writel(pwr, drv.reg_phy + S5PV210_UPHYPWR);
    rst = readl(drv.reg_phy + S5PV210_UPHYRST);
    rst |= rstbits;
    writel(rst, drv.reg_phy + S5PV210_UPHYRST);
    udelay(10);
    rst &= ~rstbits;
    writel(rst, drv.reg_phy + S5PV210_UPHYRST);
// The following delay is necessary for the reset sequence to be
// completed
//
    udelay(80);
    } else {
    pwr = readl(drv.reg_phy + S5PV210_UPHYPWR);
    pwr |= phypwr;
    writel(pwr, drv.reg_phy + S5PV210_UPHYPWR);
    }
    }
#[no_mangle]
unsafe extern "C" fn s5pv210_power_on(inst: *mut samsung_usb2_phy_instance) -> c_int {
    static int s5pv210_power_on(struct samsung_usb2_phy_instance *inst)
    {
    s5pv210_isol(inst, 0);
    s5pv210_phy_pwr(inst, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s5pv210_power_off(inst: *mut samsung_usb2_phy_instance) -> c_int {
    static int s5pv210_power_off(struct samsung_usb2_phy_instance *inst)
    {
    s5pv210_phy_pwr(inst, 0);
    s5pv210_isol(inst, 1);
    return 0;
    }
    static const struct samsung_usb2_common_phy s5pv210_phys[S5PV210_NUM_PHYS] = {
    [S5PV210_DEVICE] = {
    .label		= "device",
    .id		= S5PV210_DEVICE,
    .power_on	= s5pv210_power_on,
    .power_off	= s5pv210_power_off,
    },
    [S5PV210_HOST] = {
    .label		= "host",
    .id		= S5PV210_HOST,
    .power_on	= s5pv210_power_on,
    .power_off	= s5pv210_power_off,
    },
    };
    const struct samsung_usb2_phy_config s5pv210_usb2_phy_config = {
    .num_phys	= ARRAY_SIZE(s5pv210_phys),
    .phys		= s5pv210_phys,
    .rate_to_clk	= s5pv210_rate_to_clk,
    };
