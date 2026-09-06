//! Automatically rewritten from C to Rust
//! Source: drivers/usb/dwc3/ulpi.c
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
// ulpi.c - DesignWare USB3 Controller's ULPI PHY interface
//
// Copyright (C) 2015 Intel Corporation
//
// Author: Heikki Krogerus <heikki.krogerus@linux.intel.com>
//

pub const USB_VENDOR_MICROCHIP: c_uint = 0x0424;

    ((a >= ULPI_EXT_VENDOR_SPECIFIC) ? \
    DWC3_GUSB2PHYACC_ADDR(ULPI_ACCESS_EXTENDED) | \
    DWC3_GUSB2PHYACC_EXTEND_ADDR(a) : DWC3_GUSB2PHYACC_ADDR(a))

#[no_mangle]
unsafe extern "C" fn dwc3_ulpi_busyloop(dwc: *mut dwc3, addr: u8, read: bool) -> c_int {
    static int dwc3_ulpi_busyloop(struct dwc3 *dwc, u8 addr, bool read)
    {
    let mut ns: c_ulong = 5L * DWC3_ULPI_BASE_DELAY;
    let mut count: c_uint = 10000;
    u32 reg;
    if (addr >= ULPI_EXT_VENDOR_SPECIFIC)
    ns += DWC3_ULPI_BASE_DELAY;
    if (read)
    ns += DWC3_ULPI_BASE_DELAY;
    reg = dwc3_readl(dwc, DWC3_GUSB2PHYCFG(0));
    if (reg & DWC3_GUSB2PHYCFG_SUSPHY)
    usleep_range(1000, 1200);
    while (count--) {
    ndelay(ns);
    reg = dwc3_readl(dwc, DWC3_GUSB2PHYACC(0));
    if (reg & DWC3_GUSB2PHYACC_DONE)
    return 0;
    cpu_relax();
    }
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ulpi_read(dev: *mut device, addr: u8) -> c_int {
    static int dwc3_ulpi_read(struct device *dev, u8 addr)
    {
    struct dwc3 *dwc = dev_get_drvdata(dev);
    u32 reg;
    int ret;
    reg = DWC3_GUSB2PHYACC_NEWREGREQ | DWC3_ULPI_ADDR(addr);
    dwc3_writel(dwc, DWC3_GUSB2PHYACC(0), reg);
    ret = dwc3_ulpi_busyloop(dwc, addr, true);
    if (ret)
    return ret;
    reg = dwc3_readl(dwc, DWC3_GUSB2PHYACC(0));
    return DWC3_GUSB2PHYACC_DATA(reg);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ulpi_write(dev: *mut device, addr: u8, val: u8) -> c_int {
    static int dwc3_ulpi_write(struct device *dev, u8 addr, u8 val)
    {
    struct dwc3 *dwc = dev_get_drvdata(dev);
    u32 reg;
    reg = DWC3_GUSB2PHYACC_NEWREGREQ | DWC3_ULPI_ADDR(addr);
    reg |= DWC3_GUSB2PHYACC_WRITE | val;
    dwc3_writel(dwc, DWC3_GUSB2PHYACC(0), reg);
    return dwc3_ulpi_busyloop(dwc, addr, false);
    }
    static const struct ulpi_ops dwc3_ulpi_ops = {
    .read = dwc3_ulpi_read,
    .write = dwc3_ulpi_write,
    };
#[no_mangle]
unsafe extern "C" fn dwc3_ulpi_detect_config(dwc: *mut dwc3) {
    static void dwc3_ulpi_detect_config(struct dwc3 *dwc)
    {
    struct ulpi *ulpi = dwc.ulpi;
    switch (ulpi.id.vendor) {
    case USB_VENDOR_MICROCHIP:
    switch (ulpi.id.product) {
    case 0x0009:
// Microchip USB3340 ULPI PHY
    dwc.enable_usb2_transceiver_delay = true;
    break;
    default:
    break;
    }
    break;
    default:
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dwc3_ulpi_init(dwc: *mut dwc3) -> c_int {
    int dwc3_ulpi_init(struct dwc3 *dwc)
    {
// Register the interface
    dwc.ulpi = ulpi_register_interface(dwc.dev, &dwc3_ulpi_ops);
    if (IS_ERR(dwc.ulpi)) {
    dev_err(dwc.dev, "failed to register ULPI interface");
    return PTR_ERR(dwc.ulpi);
    }
    dwc3_ulpi_detect_config(dwc);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dwc3_ulpi_exit(dwc: *mut dwc3) {
    void dwc3_ulpi_exit(struct dwc3 *dwc)
    {
    if (dwc.ulpi) {
    ulpi_unregister_interface(dwc.ulpi);
    dwc.ulpi = core::ptr::null_mut();
    }
    }
