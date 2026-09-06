//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/83xx/usb_837x.c
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
pub unsafe extern "C" fn mpc837x_usb_cfg() -> int __init {
    int __init mpc837x_usb_cfg(void)
    {
    void __iomem *immap;
    struct device_node *np = core::ptr::null_mut();
    const void *prop;
    let mut ret: c_int = 0;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl-usb2-dr");
    if (!np || !of_device_is_available(np)) {
    of_node_put(np);
    return -ENODEV;
    }
    prop = of_get_property(np, "phy_type", core::ptr::null_mut());
    if (!prop || (strcmp(prop, "ulpi") && strcmp(prop, "serial"))) {
    pr_warn("837x USB PHY type not supported\n");
    of_node_put(np);
    return -EINVAL;
    }
// Map IMMR space for pin and clock settings
    immap = ioremap(get_immrbase(), 0x1000);
    if (!immap) {
    of_node_put(np);
    return -ENOMEM;
    }
// Configure clock
    clrsetbits_be32(immap + MPC83XX_SCCR_OFFS, MPC837X_SCCR_USB_DRCM_11,
    MPC837X_SCCR_USB_DRCM_11);
// Configure pin mux for ULPI/serial
    clrsetbits_be32(immap + MPC83XX_SICRL_OFFS, MPC837X_SICRL_USB_MASK,
    MPC837X_SICRL_USB_ULPI);
    iounmap(immap);
    of_node_put(np);
    return ret;
    }
