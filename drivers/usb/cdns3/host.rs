//! Automatically rewritten from C to Rust
//! Source: drivers/usb/cdns3/host.c
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
// Cadence USBSS and USBSSP DRD Driver - host side
//
// Copyright (C) 2018-2019 Cadence Design Systems.
// Copyright (C) 2017-2018 NXP
//
// Authors: Peter Chen <peter.chen@nxp.com>
// Pawel Laszczak <pawell@cadence.com>
//

//
// The XECP_PORT_CAP_REG and XECP_AUX_CTRL_REG1 exist only
// in Cadence USB3 dual-role controller, so it can't be used
// with Cadence CDNSP dual-role controller.
//
pub const XECP_PORT_CAP_REG: c_uint = 0x8000;
pub const XECP_AUX_CTRL_REG1: c_uint = 0x8120;

#[no_mangle]
unsafe extern "C" fn xhci_cdns3_plat_start(hcd: *mut usb_hcd) {
    static void xhci_cdns3_plat_start(struct usb_hcd *hcd)
    {
    struct xhci_hcd *xhci = hcd_to_xhci(hcd);
    u32 value;
// set usbcmd.EU3S
    value = readl(&xhci.op_regs.command);
    value |= CMD_PM_INDEX;
    writel(value, &xhci.op_regs.command);
    if (hcd.regs) {
    value = readl(hcd.regs + XECP_AUX_CTRL_REG1);
    value |= CFG_RXDET_P3_EN;
    writel(value, hcd.regs + XECP_AUX_CTRL_REG1);
    value = readl(hcd.regs + XECP_PORT_CAP_REG);
    value |= LPM_2_STB_SWITCH_EN;
    writel(value, hcd.regs + XECP_PORT_CAP_REG);
    }
    }
#[no_mangle]
unsafe extern "C" fn xhci_cdns3_resume_quirk(hcd: *mut usb_hcd) -> c_int {
    static int xhci_cdns3_resume_quirk(struct usb_hcd *hcd)
    {
    xhci_cdns3_plat_start(hcd);
    return 0;
    }
    static const struct xhci_plat_priv xhci_plat_cdns3_xhci = {
    .quirks = XHCI_SKIP_PHY_INIT | XHCI_AVOID_BEI,
    .plat_start = xhci_cdns3_plat_start,
    .resume_quirk = xhci_cdns3_resume_quirk,
    };
    static const struct xhci_plat_priv xhci_plat_cdnsp_xhci = {
    .quirks = XHCI_CDNS_SCTX_QUIRK,
    };
#[no_mangle]
unsafe extern "C" fn __cdns_host_init(cdns: *mut cdns) -> c_int {
    static int __cdns_host_init(struct cdns *cdns)
    {
    struct platform_device *xhci;
    int ret;
    struct usb_hcd *hcd;
    cdns_drd_host_on(cdns);
    xhci = platform_device_alloc("xhci-hcd", PLATFORM_DEVID_AUTO);
    if (!xhci) {
    dev_err(cdns.dev, "couldn't allocate xHCI device\n");
    return -ENOMEM;
    }
    xhci.dev.parent = cdns.dev;
    cdns.host_dev = xhci;
    ret = platform_device_add_resources(xhci, cdns.xhci_res,
    CDNS_XHCI_RESOURCES_NUM);
    if (ret) {
    dev_err(cdns.dev, "couldn't add resources to xHCI device\n");
    goto err1;
    }
    if (cdns.version < CDNSP_CONTROLLER_V2)
    cdns.xhci_plat_data = kmemdup(&xhci_plat_cdns3_xhci,
    sizeof(struct xhci_plat_priv), GFP_KERNEL);
    else
    cdns.xhci_plat_data = kmemdup(&xhci_plat_cdnsp_xhci,
    sizeof(struct xhci_plat_priv), GFP_KERNEL);
    if (!cdns.xhci_plat_data) {
    ret = -ENOMEM;
    goto err1;
    }
    if (cdns.pdata && (cdns.pdata.quirks & CDNS3_DEFAULT_PM_RUNTIME_ALLOW))
    cdns.xhci_plat_data.quirks |= XHCI_DEFAULT_PM_RUNTIME_ALLOW;
    ret = platform_device_add_data(xhci, cdns.xhci_plat_data,
    sizeof(struct xhci_plat_priv));
    if (ret)
    goto free_memory;
    ret = platform_device_add(xhci);
    if (ret) {
    dev_err(cdns.dev, "failed to register xHCI device\n");
    goto free_memory;
    }
// Glue needs to access xHCI region register for Power management
    hcd = platform_get_drvdata(xhci);
    if (hcd)
    cdns.xhci_regs = hcd.regs;
    return 0;
    free_memory:
    kfree(cdns.xhci_plat_data);
    err1:
    platform_device_put(xhci);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdns_host_exit(cdns: *mut cdns) {
    static void cdns_host_exit(struct cdns *cdns)
    {
    kfree(cdns.xhci_plat_data);
    platform_device_unregister(cdns.host_dev);
    cdns.host_dev = core::ptr::null_mut();
    cdns_drd_host_off(cdns);
    }
#[no_mangle]
unsafe extern "C" fn cdns_host_resume(cdns: *mut cdns, power_lost: bool) -> c_int {
    static int cdns_host_resume(struct cdns *cdns, bool power_lost)
    {
    struct usb_hcd *hcd = platform_get_drvdata(cdns.host_dev);
    struct xhci_plat_priv *priv = hcd_to_xhci_priv(hcd);
    priv.power_lost = power_lost;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cdns_host_init(cdns: *mut cdns) -> c_int {
    int cdns_host_init(struct cdns *cdns)
    {
    struct cdns_role_driver *rdrv;
    rdrv = devm_kzalloc(cdns.dev, sizeof(*rdrv), GFP_KERNEL);
    if (!rdrv)
    return -ENOMEM;
    rdrv.start	= __cdns_host_init;
    rdrv.stop	= cdns_host_exit;
    rdrv.resume	= cdns_host_resume;
    rdrv.state	= CDNS_ROLE_STATE_INACTIVE;
    rdrv.name	= "host";
    cdns.roles[USB_ROLE_HOST] = rdrv;
    return 0;
    }
