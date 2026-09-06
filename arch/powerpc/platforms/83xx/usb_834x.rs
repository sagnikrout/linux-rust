//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/83xx/usb_834x.c
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
pub unsafe extern "C" fn mpc834x_usb_cfg() -> int __init {
    int __init mpc834x_usb_cfg(void)
    {
    unsigned long sccr, sicrl, sicrh;
    void __iomem *immap;
    struct device_node *np = core::ptr::null_mut();
    let mut port0_is_dr: c_int = 0, port1_is_dr = 0;
    const void *prop, *dr_mode;
    immap = ioremap(get_immrbase(), 0x1000);
    if (!immap)
    return -ENOMEM;
// Read registers
// Note: DR and MPH must use the same clock setting in SCCR
    sccr = in_be32(immap + MPC83XX_SCCR_OFFS) & ~MPC83XX_SCCR_USB_MASK;
    sicrl = in_be32(immap + MPC83XX_SICRL_OFFS) & ~MPC834X_SICRL_USB_MASK;
    sicrh = in_be32(immap + MPC83XX_SICRH_OFFS) & ~MPC834X_SICRH_USB_UTMI;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl-usb2-dr");
    if (np) {
    sccr |= MPC83XX_SCCR_USB_DRCM_11;  /* 1:3 */
    prop = of_get_property(np, "phy_type", core::ptr::null_mut());
    port1_is_dr = 1;
    if (prop &&
    (!strcmp(prop, "utmi") || !strcmp(prop, "utmi_wide"))) {
    sicrl |= MPC834X_SICRL_USB0 | MPC834X_SICRL_USB1;
    sicrh |= MPC834X_SICRH_USB_UTMI;
    port0_is_dr = 1;
    } else if (prop && !strcmp(prop, "serial")) {
    dr_mode = of_get_property(np, "dr_mode", core::ptr::null_mut());
    if (dr_mode && !strcmp(dr_mode, "otg")) {
    sicrl |= MPC834X_SICRL_USB0 | MPC834X_SICRL_USB1;
    port0_is_dr = 1;
    } else {
    sicrl |= MPC834X_SICRL_USB1;
    }
    } else if (prop && !strcmp(prop, "ulpi")) {
    sicrl |= MPC834X_SICRL_USB1;
    } else {
    pr_warn("834x USB PHY type not supported\n");
    }
    of_node_put(np);
    }
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl-usb2-mph");
    if (np) {
    sccr |= MPC83XX_SCCR_USB_MPHCM_11; /* 1:3 */
    prop = of_get_property(np, "port0", core::ptr::null_mut());
    if (prop) {
    if (port0_is_dr)
    pr_warn("834x USB port0 can't be used by both DR and MPH!\n");
    sicrl &= ~MPC834X_SICRL_USB0;
    }
    prop = of_get_property(np, "port1", core::ptr::null_mut());
    if (prop) {
    if (port1_is_dr)
    pr_warn("834x USB port1 can't be used by both DR and MPH!\n");
    sicrl &= ~MPC834X_SICRL_USB1;
    }
    of_node_put(np);
    }
// Write back
    out_be32(immap + MPC83XX_SCCR_OFFS, sccr);
    out_be32(immap + MPC83XX_SICRL_OFFS, sicrl);
    out_be32(immap + MPC83XX_SICRH_OFFS, sicrh);
    iounmap(immap);
    return 0;
    }
