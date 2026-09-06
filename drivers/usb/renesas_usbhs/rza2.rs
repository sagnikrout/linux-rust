//! Automatically rewritten from C to Rust
//! Source: drivers/usb/renesas_usbhs/rza2.c
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
// Renesas USB driver RZ/A2 initialization and power control
//
// Copyright (C) 2019 Chris Brandt
// Copyright (C) 2019 Renesas Electronics Corporation
//

#[no_mangle]
unsafe extern "C" fn usbhs_rza2_hardware_init(pdev: *mut platform_device) -> c_int {
    static int usbhs_rza2_hardware_init(struct platform_device *pdev)
    {
    struct usbhs_priv *priv = usbhs_pdev_to_priv(pdev);
    struct phy *phy = phy_get(&pdev.dev, "usb");
    if (IS_ERR(phy))
    return PTR_ERR(phy);
    priv.phy = phy;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usbhs_rza2_hardware_exit(pdev: *mut platform_device) -> c_int {
    static int usbhs_rza2_hardware_exit(struct platform_device *pdev)
    {
    struct usbhs_priv *priv = usbhs_pdev_to_priv(pdev);
    phy_put(&pdev.dev, priv.phy);
    priv.phy = core::ptr::null_mut();
    return 0;
    }
    static int usbhs_rza2_power_ctrl(struct platform_device *pdev,
    void __iomem *base, int enable)
    {
    struct usbhs_priv *priv = usbhs_pdev_to_priv(pdev);
    let mut retval: c_int = 0;
    if (!priv.phy)
    return -ENODEV;
    if (enable) {
    retval = phy_init(priv.phy);
    usbhs_bset(priv, SUSPMODE, SUSPM, SUSPM);
    udelay(100);	/* Wait for PLL to become stable */
    if (!retval)
    retval = phy_power_on(priv.phy);
    } else {
    usbhs_bset(priv, SUSPMODE, SUSPM, 0);
    phy_power_off(priv.phy);
    phy_exit(priv.phy);
    }
    return retval;
    }
    const struct renesas_usbhs_platform_info usbhs_rza2_plat_info = {
    .platform_callback = {
    .hardware_init = usbhs_rza2_hardware_init,
    .hardware_exit = usbhs_rza2_hardware_exit,
    .power_ctrl = usbhs_rza2_power_ctrl,
    .get_id = usbhs_get_id_as_gadget,
    },
    .driver_param = {
    .has_cnen = 1,
    .cfifo_byte_addr = 1,
    .has_new_pipe_configs = 1,
    },
    };
    const struct renesas_usbhs_platform_info usbhs_rzg2l_plat_info = {
    .platform_callback = {
    .hardware_init = usbhs_rza2_hardware_init,
    .hardware_exit = usbhs_rza2_hardware_exit,
    .power_ctrl = usbhs_rza2_power_ctrl,
    .get_id = usbhs_get_id_as_gadget,
    },
    .driver_param = {
    .has_cnen = 1,
    .cfifo_byte_addr = 1,
    },
    };
