//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/83xx/usb_831x.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Freescale 83xx USB SOC setup code
//
// Copyright (C) 2007 Freescale Semiconductor, Inc.
// Author: Li Yang
//

#[no_mangle]
pub unsafe extern "C" fn mpc831x_usb_cfg() -> int __init {
    int __init mpc831x_usb_cfg(void)
    {
    u32 temp;
    void __iomem *immap, *usb_regs;
    struct device_node *np = core::ptr::null_mut();
    struct device_node *immr_node = core::ptr::null_mut();
    const void *prop;
    struct resource res;
    let mut ret: c_int = 0;

    const void *dr_mode;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl-usb2-dr");
    if (!np)
    return -ENODEV;
    prop = of_get_property(np, "phy_type", core::ptr::null_mut());
// Map IMMR space for pin and clock settings
    immap = ioremap(get_immrbase(), 0x1000);
    if (!immap) {
    of_node_put(np);
    return -ENOMEM;
    }
// Configure clock
    immr_node = of_get_parent(np);
    if (immr_node && (of_device_is_compatible(immr_node, "fsl,mpc8315-immr") ||
    of_device_is_compatible(immr_node, "fsl,mpc8308-immr")))
    clrsetbits_be32(immap + MPC83XX_SCCR_OFFS,
    MPC8315_SCCR_USB_MASK,
    MPC8315_SCCR_USB_DRCM_01);
    else
    clrsetbits_be32(immap + MPC83XX_SCCR_OFFS,
    MPC83XX_SCCR_USB_MASK,
    MPC83XX_SCCR_USB_DRCM_11);
// Configure pin mux for ULPI.  There is no pin mux for UTMI
    if (prop && !strcmp(prop, "ulpi")) {
    if (of_device_is_compatible(immr_node, "fsl,mpc8308-immr")) {
    clrsetbits_be32(immap + MPC83XX_SICRH_OFFS,
    MPC8308_SICRH_USB_MASK,
    MPC8308_SICRH_USB_ULPI);
    } else if (of_device_is_compatible(immr_node, "fsl,mpc8315-immr")) {
    clrsetbits_be32(immap + MPC83XX_SICRL_OFFS,
    MPC8315_SICRL_USB_MASK,
    MPC8315_SICRL_USB_ULPI);
    clrsetbits_be32(immap + MPC83XX_SICRH_OFFS,
    MPC8315_SICRH_USB_MASK,
    MPC8315_SICRH_USB_ULPI);
    } else {
    clrsetbits_be32(immap + MPC83XX_SICRL_OFFS,
    MPC831X_SICRL_USB_MASK,
    MPC831X_SICRL_USB_ULPI);
    clrsetbits_be32(immap + MPC83XX_SICRH_OFFS,
    MPC831X_SICRH_USB_MASK,
    MPC831X_SICRH_USB_ULPI);
    }
    }
    iounmap(immap);
    of_node_put(immr_node);
// Map USB SOC space
    ret = of_address_to_resource(np, 0, &res);
    if (ret) {
    of_node_put(np);
    return ret;
    }
    usb_regs = ioremap(res.start, resource_size(&res));
// Using on-chip PHY
    if (prop && (!strcmp(prop, "utmi_wide") || !strcmp(prop, "utmi"))) {
    u32 refsel;
    if (of_device_is_compatible(immr_node, "fsl,mpc8308-immr"))
    goto out;
    if (of_device_is_compatible(immr_node, "fsl,mpc8315-immr"))
    refsel = CONTROL_REFSEL_24MHZ;
    else
    refsel = CONTROL_REFSEL_48MHZ;
// Set UTMI_PHY_EN and REFSEL
    out_be32(usb_regs + FSL_USB2_CONTROL_OFFS,
    CONTROL_UTMI_PHY_EN | refsel);
// Using external UPLI PHY
    } else if (prop && !strcmp(prop, "ulpi")) {
// Set PHY_CLK_SEL to ULPI
    temp = CONTROL_PHY_CLK_SEL_ULPI;

// Set OTG_PORT
    if (!of_device_is_compatible(immr_node, "fsl,mpc8308-immr")) {
    dr_mode = of_get_property(np, "dr_mode", core::ptr::null_mut());
    if (dr_mode && !strcmp(dr_mode, "otg"))
    temp |= CONTROL_OTG_PORT;
    }

    out_be32(usb_regs + FSL_USB2_CONTROL_OFFS, temp);
    } else {
    pr_warn("831x USB PHY type not supported\n");
    ret = -EINVAL;
    }
    out:
    iounmap(usb_regs);
    of_node_put(np);
    return ret;
    }
