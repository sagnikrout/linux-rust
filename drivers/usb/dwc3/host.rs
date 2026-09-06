//! Automatically rewritten from C to Rust
//! Source: drivers/usb/dwc3/host.c
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
// host.c - DesignWare USB3 DRD Controller Host Glue
//
// Copyright (C) 2011 Texas Instruments Incorporated - https://www.ti.com
//
// Authors: Felipe Balbi <balbi@ti.com>,
//

pub const XHCI_HCSPARAMS1: c_uint = 0x4;
pub const XHCI_PORTSC_BASE: c_uint = 0x400;
//
// dwc3_power_off_all_roothub_ports - Power off all Root hub ports
// @dwc: Pointer to our controller context structure
//
#[no_mangle]
unsafe extern "C" fn dwc3_power_off_all_roothub_ports(dwc: *mut dwc3) {
    static void dwc3_power_off_all_roothub_ports(struct dwc3 *dwc)
    {
    void __iomem *xhci_regs;
    u32 op_regs_base;
    int port_num;
    u32 offset;
    u32 reg;
    int i;
// xhci regs are not mapped yet, do it temporarily here
    if (dwc.xhci_resources[0].start) {
    if (dwc.xhci_resources[0].flags & IORESOURCE_MEM_NONPOSTED)
    xhci_regs = ioremap_np(dwc.xhci_resources[0].start, DWC3_XHCI_REGS_END);
    else
    xhci_regs = ioremap(dwc.xhci_resources[0].start, DWC3_XHCI_REGS_END);
    if (!xhci_regs) {
    dev_err(dwc.dev, "Failed to ioremap xhci_regs\n");
    return;
    }
    op_regs_base = FIELD_GET(HC_LENGTH, readl(xhci_regs));
    reg = readl(xhci_regs + XHCI_HCSPARAMS1);
    port_num = FIELD_GET(HCS_MAX_PORTS, reg);
    for (i = 1; i <= port_num; i++) {
    offset = op_regs_base + XHCI_PORTSC_BASE + 0x10 * (i - 1);
    reg = readl(xhci_regs + offset);
    reg &= ~PORT_POWER;
    writel(reg, xhci_regs + offset);
    }
    iounmap(xhci_regs);
    } else {
    dev_err(dwc.dev, "xhci base reg invalid\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn dwc3_xhci_plat_start(hcd: *mut usb_hcd) {
    static void dwc3_xhci_plat_start(struct usb_hcd *hcd)
    {
    struct platform_device *pdev;
    struct dwc3 *dwc;
    if (!usb_hcd_is_primary_hcd(hcd))
    return;
    pdev = to_platform_device(hcd.self.controller);
    dwc = dev_get_drvdata(pdev.dev.parent);
    dwc3_enable_susphy(dwc, true);
    }
    static const struct xhci_plat_priv dwc3_xhci_plat_quirk = {
    .plat_start = dwc3_xhci_plat_start,
    };
    static void dwc3_host_fill_xhci_irq_res(struct dwc3 *dwc,
    int irq, char *name)
    {
    struct platform_device *pdev = to_platform_device(dwc.dev);
    struct device_node *np = dev_of_node(&pdev.dev);
    dwc.xhci_resources[1].start = irq;
    dwc.xhci_resources[1].end = irq;
    dwc.xhci_resources[1].flags = IORESOURCE_IRQ | irq_get_trigger_type(irq);
    if (!name && np)
    dwc.xhci_resources[1].name = of_node_full_name(pdev.dev.of_node);
    else
    dwc.xhci_resources[1].name = name;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_host_get_irq(dwc: *mut dwc3) -> c_int {
    static int dwc3_host_get_irq(struct dwc3 *dwc)
    {
    struct platform_device	*dwc3_pdev = to_platform_device(dwc.dev);
    int irq;
    irq = platform_get_irq_byname_optional(dwc3_pdev, "host");
    if (irq > 0) {
    dwc3_host_fill_xhci_irq_res(dwc, irq, "host");
    goto out;
    }
    if (irq == -EPROBE_DEFER)
    goto out;
    irq = platform_get_irq_byname_optional(dwc3_pdev, "dwc_usb3");
    if (irq > 0) {
    dwc3_host_fill_xhci_irq_res(dwc, irq, "dwc_usb3");
    goto out;
    }
    if (irq == -EPROBE_DEFER)
    goto out;
    irq = platform_get_irq(dwc3_pdev, 0);
    if (irq > 0)
    dwc3_host_fill_xhci_irq_res(dwc, irq, core::ptr::null_mut());
    out:
    return irq;
    }
#[no_mangle]
pub unsafe extern "C" fn dwc3_host_init(dwc: *mut dwc3) -> c_int {
    int dwc3_host_init(struct dwc3 *dwc)
    {
    struct property_entry	props[6];
    struct platform_device	*xhci;
    int			ret, irq;
    let mut prop_idx: c_int = 0;
//
// Some platforms need to power off all Root hub ports immediately after DWC3 set to host
// mode to avoid VBUS glitch happen when xhci get reset later.
//
    dwc3_power_off_all_roothub_ports(dwc);
    irq = dwc3_host_get_irq(dwc);
    if (irq < 0)
    return irq;
    xhci = platform_device_alloc("xhci-hcd", PLATFORM_DEVID_AUTO);
    if (!xhci) {
    dev_err(dwc.dev, "couldn't allocate xHCI device\n");
    return -ENOMEM;
    }
    xhci.dev.parent	= dwc.dev;
    dwc.xhci = xhci;
    ret = platform_device_add_resources(xhci, dwc.xhci_resources,
    DWC3_XHCI_RESOURCES_NUM);
    if (ret) {
    dev_err(dwc.dev, "couldn't add resources to xHCI device\n");
    goto err;
    }
    memset(props, 0, sizeof(struct property_entry) * ARRAY_SIZE(props));
    props[prop_idx++] = PROPERTY_ENTRY_BOOL("xhci-sg-trb-cache-size-quirk");
    props[prop_idx++] = PROPERTY_ENTRY_BOOL("write-64-hi-lo-quirk");
    if (dwc.usb3_lpm_capable)
    props[prop_idx++] = PROPERTY_ENTRY_BOOL("usb3-lpm-capable");
    if (dwc.usb2_lpm_disable)
    props[prop_idx++] = PROPERTY_ENTRY_BOOL("usb2-lpm-disable");
//
// WORKAROUND: dwc3 revisions <=3.00a have a limitation
// where Port Disable command doesn't work.
//
// The suggested workaround is that we avoid Port Disable
// completely.
//
// This following flag tells XHCI to do just that.
//
    if (DWC3_VER_IS_WITHIN(DWC3, ANY, 300A))
    props[prop_idx++] = PROPERTY_ENTRY_BOOL("quirk-broken-port-ped");
    props[prop_idx++] = PROPERTY_ENTRY_U16("num-hc-interrupters",
    dwc.num_hc_interrupters);
    if (prop_idx) {
    ret = device_create_managed_software_node(&xhci.dev, props, core::ptr::null_mut());
    if (ret) {
    dev_err(dwc.dev, "failed to add properties to xHCI\n");
    goto err;
    }
    }
    ret = platform_device_add_data(xhci, &dwc3_xhci_plat_quirk,
    sizeof(struct xhci_plat_priv));
    if (ret)
    goto err;
    ret = platform_device_add(xhci);
    if (ret) {
    dev_err(dwc.dev, "failed to register xHCI device\n");
    goto err;
    }
    if (dwc.sys_wakeup) {
// Restore wakeup setting if switched from device
    device_wakeup_enable(dwc.sysdev);
// Pass on wakeup setting to the new xhci platform device
    device_init_wakeup(&xhci.dev, true);
    }
    return 0;
    err:
    platform_device_put(xhci);
    return ret;
    }
    EXPORT_SYMBOL_GPL(dwc3_host_init);
#[no_mangle]
pub unsafe extern "C" fn dwc3_host_exit(dwc: *mut dwc3) {
    void dwc3_host_exit(struct dwc3 *dwc)
    {
    if (dwc.sys_wakeup)
    device_init_wakeup(&dwc.xhci.dev, false);
    dwc3_enable_susphy(dwc, true);
    platform_device_unregister(dwc.xhci);
    dwc.xhci = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(dwc3_host_exit);
