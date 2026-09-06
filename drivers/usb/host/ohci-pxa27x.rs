//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/ohci-pxa27x.c
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


// SPDX-License-Identifier: GPL-1.0+
//
// OHCI HCD (Host Controller Driver) for USB.
//
// (C) Copyright 1999 Roman Weissgaerber <weissg@vienna.at>
// (C) Copyright 2000-2002 David Brownell <dbrownell@users.sourceforge.net>
// (C) Copyright 2002 Hewlett-Packard Company
//
// Bus Glue for pxa27x
//
// Written by Christopher Hoover <ch@hpl.hp.com>
// Based on fragments of previous driver by Russell King et al.
//
// Modified for LH7A404 from ohci-sa1111.c
// by Durgesh Pattamatta <pattamattad@sharpsec.com>
//
// Modified for pxa27x from ohci-lh7a404.c
// by Nick Bane <nick@cecomputing.co.uk> 26-8-2004
//
// This file is licenced under the GPL.
//

//
// UHC: USB Host Controller (OHCI-like) register definitions
//

    (((x) & 0xff) << 24) /* Power On To Power Good Time */

    Interrupt Enable*/

pub const PXA_UHC_MAX_PORTNUM: c_int = 3;
    static struct hc_driver __read_mostly ohci_pxa27x_hc_driver;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa27x_ohci {
    pub clk: *mut clk,
    pub mmio_base: *mut void __iomem,
    pub vbus: [*mut regulator; 3],
    pub vbus_enabled: [bool; 3],
}

//
    PMM_NPS_MODE -- PMM Non-power switching mode
    Ports are powered continuously.
    PMM_GLOBAL_MODE -- PMM global switching mode
    All ports are powered at the same time.
    PMM_PERPORT_MODE -- PMM per port switching mode
    Ports are powered individually.
//
#[no_mangle]
unsafe extern "C" fn pxa27x_ohci_select_pmm(pxa_ohci: *mut pxa27x_ohci, mode: c_int) -> c_int {
    static int pxa27x_ohci_select_pmm(struct pxa27x_ohci *pxa_ohci, int mode)
    {
    let mut uhcrhda: u32 = __raw_readl(pxa_ohci.mmio_base + UHCRHDA);
    let mut uhcrhdb: u32 = __raw_readl(pxa_ohci.mmio_base + UHCRHDB);
    switch (mode) {
    case PMM_NPS_MODE:
    uhcrhda |= RH_A_NPS;
    break;
    case PMM_GLOBAL_MODE:
    uhcrhda &= ~(RH_A_NPS | RH_A_PSM);
    break;
    case PMM_PERPORT_MODE:
    uhcrhda &= ~(RH_A_NPS);
    uhcrhda |= RH_A_PSM;
// Set port power control mask bits, only 3 ports.
    uhcrhdb |= (0x7<<17);
    break;
    default:
    printk( KERN_ERR
    "Invalid mode %d, set to non-power switch mode.\n",
    mode );
    uhcrhda |= RH_A_NPS;
    }
    __raw_writel(uhcrhda, pxa_ohci.mmio_base + UHCRHDA);
    __raw_writel(uhcrhdb, pxa_ohci.mmio_base + UHCRHDB);
    return 0;
    }
    static int pxa27x_ohci_set_vbus_power(struct pxa27x_ohci *pxa_ohci,
    unsigned int port, bool enable)
    {
    struct regulator *vbus = pxa_ohci.vbus[port];
    let mut ret: c_int = 0;
    if (IS_ERR_OR_NULL(vbus))
    return 0;
    if (enable && !pxa_ohci.vbus_enabled[port])
    ret = regulator_enable(vbus);
#[no_mangle]
pub unsafe extern "C" fn if(pxa_ohci->vbus_enabled[port]: !enable &&) -> else {
    else if (!enable && pxa_ohci.vbus_enabled[port])
    ret = regulator_disable(vbus);
    if (ret < 0)
    return ret;
    pxa_ohci.vbus_enabled[port] = enable;
    return 0;
    }
    static int pxa27x_ohci_hub_control(struct usb_hcd *hcd, u16 typeReq, u16 wValue,
    u16 wIndex, char *buf, u16 wLength)
    {
    struct pxa27x_ohci *pxa_ohci = to_pxa27x_ohci(hcd);
    int ret;
    switch (typeReq) {
    case SetPortFeature:
    case ClearPortFeature:
    if (!wIndex || wIndex > 3)
    return -EPIPE;
    if (wValue != USB_PORT_FEAT_POWER)
    break;
    ret = pxa27x_ohci_set_vbus_power(pxa_ohci, wIndex - 1,
    typeReq == SetPortFeature);
    if (ret)
    return ret;
    break;
    }
    return ohci_hub_control(hcd, typeReq, wValue, wIndex, buf, wLength);
    }
// -------------------------------------------------------------------------
    static inline void pxa27x_setup_hc(struct pxa27x_ohci *pxa_ohci,
    struct pxaohci_platform_data *inf)
    {
    let mut uhchr: u32 = __raw_readl(pxa_ohci.mmio_base + UHCHR);
    let mut uhcrhda: u32 = __raw_readl(pxa_ohci.mmio_base + UHCRHDA);
    if (inf.flags & ENABLE_PORT1)
    uhchr &= ~UHCHR_SSEP1;
    if (inf.flags & ENABLE_PORT2)
    uhchr &= ~UHCHR_SSEP2;
    if (inf.flags & ENABLE_PORT3)
    uhchr &= ~UHCHR_SSEP3;
    if (inf.flags & POWER_CONTROL_LOW)
    uhchr |= UHCHR_PCPL;
    if (inf.flags & POWER_SENSE_LOW)
    uhchr |= UHCHR_PSPL;
    if (inf.flags & NO_OC_PROTECTION)
    uhcrhda |= UHCRHDA_NOCP;
    else
    uhcrhda &= ~UHCRHDA_NOCP;
    if (inf.flags & OC_MODE_PERPORT)
    uhcrhda |= UHCRHDA_OCPM;
    else
    uhcrhda &= ~UHCRHDA_OCPM;
    if (inf.power_on_delay) {
    uhcrhda &= ~UHCRHDA_POTPGT(0xff);
    uhcrhda |= UHCRHDA_POTPGT(inf.power_on_delay / 2);
    }
    __raw_writel(uhchr, pxa_ohci.mmio_base + UHCHR);
    __raw_writel(uhcrhda, pxa_ohci.mmio_base + UHCRHDA);
    }
#[no_mangle]
pub unsafe extern "C" fn pxa27x_reset_hc(pxa_ohci: *mut pxa27x_ohci) {
    static inline void pxa27x_reset_hc(struct pxa27x_ohci *pxa_ohci)
    {
    let mut uhchr: u32 = __raw_readl(pxa_ohci.mmio_base + UHCHR);
    __raw_writel(uhchr | UHCHR_FHR, pxa_ohci.mmio_base + UHCHR);
    udelay(11);
    __raw_writel(uhchr & ~UHCHR_FHR, pxa_ohci.mmio_base + UHCHR);
    }
#[no_mangle]
unsafe extern "C" fn pxa27x_start_hc(pxa_ohci: *mut pxa27x_ohci, dev: *mut device) -> c_int {
    static int pxa27x_start_hc(struct pxa27x_ohci *pxa_ohci, struct device *dev)
    {
    int retval;
    struct pxaohci_platform_data *inf;
    uint32_t uhchr;
    inf = dev_get_platdata(dev);
    retval = clk_prepare_enable(pxa_ohci.clk);
    if (retval)
    return retval;
    pxa27x_reset_hc(pxa_ohci);
    uhchr = __raw_readl(pxa_ohci.mmio_base + UHCHR) | UHCHR_FSBIR;
    __raw_writel(uhchr, pxa_ohci.mmio_base + UHCHR);
    while (__raw_readl(pxa_ohci.mmio_base + UHCHR) & UHCHR_FSBIR)
    cpu_relax();
    pxa27x_setup_hc(pxa_ohci, inf);
    if (inf.init)
    retval = inf.init(dev);
    if (retval < 0) {
    clk_disable_unprepare(pxa_ohci.clk);
    return retval;
    }
    uhchr = __raw_readl(pxa_ohci.mmio_base + UHCHR) & ~UHCHR_SSE;
    __raw_writel(uhchr, pxa_ohci.mmio_base + UHCHR);
    __raw_writel(UHCHIE_UPRIE | UHCHIE_RWIE, pxa_ohci.mmio_base + UHCHIE);
// Clear any OTG Pin Hold
    pxa27x_clear_otgph();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa27x_stop_hc(pxa_ohci: *mut pxa27x_ohci, dev: *mut device) {
    static void pxa27x_stop_hc(struct pxa27x_ohci *pxa_ohci, struct device *dev)
    {
    struct pxaohci_platform_data *inf;
    uint32_t uhccoms;
    inf = dev_get_platdata(dev);
    if (inf.exit)
    inf.exit(dev);
    pxa27x_reset_hc(pxa_ohci);
// Host Controller Reset
    uhccoms = __raw_readl(pxa_ohci.mmio_base + UHCCOMS) | 0x01;
    __raw_writel(uhccoms, pxa_ohci.mmio_base + UHCCOMS);
    udelay(10);
    clk_disable_unprepare(pxa_ohci.clk);
    }

    static const struct of_device_id pxa_ohci_dt_ids[] = {
    { .compatible = "marvell,pxa-ohci" },
    { }
    };
    MODULE_DEVICE_TABLE(of, pxa_ohci_dt_ids);
#[no_mangle]
unsafe extern "C" fn ohci_pxa_of_init(pdev: *mut platform_device) -> c_int {
    static int ohci_pxa_of_init(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct pxaohci_platform_data *pdata;
    u32 tmp;
    int ret;
    if (!np)
    return 0;
// Right now device-tree probed devices don't get dma_mask set.
// Since shared usb code relies on it, set it here for now.
// Once we have dma capability bindings this can go away.
//
    ret = dma_coerce_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(32));
    if (ret)
    return ret;
    pdata = devm_kzalloc(&pdev.dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    if (of_property_read_bool(np, "marvell,enable-port1"))
    pdata.flags |= ENABLE_PORT1;
    if (of_property_read_bool(np, "marvell,enable-port2"))
    pdata.flags |= ENABLE_PORT2;
    if (of_property_read_bool(np, "marvell,enable-port3"))
    pdata.flags |= ENABLE_PORT3;
    if (of_property_read_bool(np, "marvell,port-sense-low"))
    pdata.flags |= POWER_SENSE_LOW;
    if (of_property_read_bool(np, "marvell,power-control-low"))
    pdata.flags |= POWER_CONTROL_LOW;
    if (of_property_read_bool(np, "marvell,no-oc-protection"))
    pdata.flags |= NO_OC_PROTECTION;
    if (of_property_read_bool(np, "marvell,oc-mode-perport"))
    pdata.flags |= OC_MODE_PERPORT;
    if (!of_property_read_u32(np, "marvell,power-on-delay", &tmp))
    pdata.power_on_delay = tmp;
    if (!of_property_read_u32(np, "marvell,port-mode", &tmp))
    pdata.port_mode = tmp;
    if (!of_property_read_u32(np, "marvell,power-budget", &tmp))
    pdata.power_budget = tmp;
    pdev.dev.platform_data = pdata;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ohci_pxa_of_init(pdev: *mut platform_device) -> c_int {
    static int ohci_pxa_of_init(struct platform_device *pdev)
    {
    return 0;
    }

// -------------------------------------------------------------------------
// configure so an HC device and id are always provided
// always called with process context; sleeping is OK
//
// ohci_hcd_pxa27x_probe - initialize pxa27x-based HCDs
// @pdev:	USB Host controller to probe
//
// Context: task context, might sleep
//
// Allocates basic resources for this USB host controller, and
// then invokes the start() method for the HCD associated with it
// through the hotplug entry's driver_data.
//
#[no_mangle]
unsafe extern "C" fn ohci_hcd_pxa27x_probe(pdev: *mut platform_device) -> c_int {
    static int ohci_hcd_pxa27x_probe(struct platform_device *pdev)
    {
    int retval, irq;
    struct usb_hcd *hcd;
    struct pxaohci_platform_data *inf;
    struct pxa27x_ohci *pxa_ohci;
    struct ohci_hcd *ohci;
    struct resource *r;
    struct clk *usb_clk;
    unsigned int i;
    retval = ohci_pxa_of_init(pdev);
    if (retval)
    return retval;
    inf = dev_get_platdata(&pdev.dev);
    if (!inf)
    return -ENODEV;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    pr_err("no resource of IORESOURCE_IRQ");
    return irq;
    }
    usb_clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(usb_clk))
    return PTR_ERR(usb_clk);
    hcd = usb_create_hcd(&ohci_pxa27x_hc_driver, &pdev.dev, "pxa27x");
    if (!hcd)
    return -ENOMEM;
    hcd.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &r);
    if (IS_ERR(hcd.regs)) {
    retval = PTR_ERR(hcd.regs);
    goto err;
    }
    hcd.rsrc_start = r.start;
    hcd.rsrc_len = resource_size(r);
// initialize "struct pxa27x_ohci"
    pxa_ohci = to_pxa27x_ohci(hcd);
    pxa_ohci.clk = usb_clk;
    pxa_ohci.mmio_base = (void __iomem *)hcd.regs;
    for (i = 0; i < 3; ++i) {
    char name[6];
    if (!(inf.flags & (ENABLE_PORT1 << i)))
    continue;
    sprintf(name, "vbus%u", i + 1);
    pxa_ohci.vbus[i] = devm_regulator_get(&pdev.dev, name);
    }
    retval = pxa27x_start_hc(pxa_ohci, &pdev.dev);
    if (retval < 0) {
    pr_debug("pxa27x_start_hc failed");
    goto err;
    }
// Select Power Management Mode
    pxa27x_ohci_select_pmm(pxa_ohci, inf.port_mode);
    if (inf.power_budget)
    hcd.power_budget = inf.power_budget;
// The value of NDP in roothub_a is incorrect on this hardware
    ohci = hcd_to_ohci(hcd);
    ohci.num_ports = 3;
    retval = usb_add_hcd(hcd, irq, 0);
    if (retval == 0) {
    device_wakeup_enable(hcd.self.controller);
    return retval;
    }
    pxa27x_stop_hc(pxa_ohci, &pdev.dev);
    err:
    usb_put_hcd(hcd);
    return retval;
    }
// may be called without controller electrically present
// may be called with controller, bus, and devices active
//
// ohci_hcd_pxa27x_remove - shutdown processing for pxa27x-based HCDs
// @pdev: USB Host Controller being removed
//
// Context: task context, might sleep
//
// Reverses the effect of ohci_hcd_pxa27x_probe(), first invoking
// the HCD's stop() method.  It is always called from a thread
// context, normally "rmmod", "apmd", or something similar.
//
#[no_mangle]
unsafe extern "C" fn ohci_hcd_pxa27x_remove(pdev: *mut platform_device) {
    static void ohci_hcd_pxa27x_remove(struct platform_device *pdev)
    {
    struct usb_hcd *hcd = platform_get_drvdata(pdev);
    struct pxa27x_ohci *pxa_ohci = to_pxa27x_ohci(hcd);
    unsigned int i;
    usb_remove_hcd(hcd);
    pxa27x_stop_hc(pxa_ohci, &pdev.dev);
    for (i = 0; i < 3; ++i)
    pxa27x_ohci_set_vbus_power(pxa_ohci, i, false);
    usb_put_hcd(hcd);
    }
// -------------------------------------------------------------------------

#[no_mangle]
unsafe extern "C" fn ohci_hcd_pxa27x_drv_suspend(dev: *mut device) -> c_int {
    static int ohci_hcd_pxa27x_drv_suspend(struct device *dev)
    {
    struct usb_hcd *hcd = dev_get_drvdata(dev);
    struct pxa27x_ohci *pxa_ohci = to_pxa27x_ohci(hcd);
    struct ohci_hcd *ohci = hcd_to_ohci(hcd);
    let mut do_wakeup: bool = device_may_wakeup(dev);
    int ret;
    if (time_before(jiffies, ohci.next_statechange))
    msleep(5);
    ohci.next_statechange = jiffies;
    ret = ohci_suspend(hcd, do_wakeup);
    if (ret)
    return ret;
    pxa27x_stop_hc(pxa_ohci, dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ohci_hcd_pxa27x_drv_resume(dev: *mut device) -> c_int {
    static int ohci_hcd_pxa27x_drv_resume(struct device *dev)
    {
    struct usb_hcd *hcd = dev_get_drvdata(dev);
    struct pxa27x_ohci *pxa_ohci = to_pxa27x_ohci(hcd);
    struct pxaohci_platform_data *inf = dev_get_platdata(dev);
    struct ohci_hcd *ohci = hcd_to_ohci(hcd);
    int status;
    if (time_before(jiffies, ohci.next_statechange))
    msleep(5);
    ohci.next_statechange = jiffies;
    status = pxa27x_start_hc(pxa_ohci, dev);
    if (status < 0)
    return status;
// Select Power Management Mode
    pxa27x_ohci_select_pmm(pxa_ohci, inf.port_mode);
    ohci_resume(hcd, false);
    return 0;
    }
    static const struct dev_pm_ops ohci_hcd_pxa27x_pm_ops = {
    .suspend	= ohci_hcd_pxa27x_drv_suspend,
    .resume		= ohci_hcd_pxa27x_drv_resume,
    };

    static struct platform_driver ohci_hcd_pxa27x_driver = {
    .probe		= ohci_hcd_pxa27x_probe,
    .remove		= ohci_hcd_pxa27x_remove,
    .shutdown	= usb_hcd_platform_shutdown,
    .driver		= {
    .name	= "pxa27x-ohci",
    .of_match_table = of_match_ptr(pxa_ohci_dt_ids),

    .pm	= &ohci_hcd_pxa27x_pm_ops,

    },
    };
    static const struct ohci_driver_overrides pxa27x_overrides __initconst = {
    .extra_priv_size =      sizeof(struct pxa27x_ohci),
    };
#[no_mangle]
unsafe extern "C" fn ohci_pxa27x_init() -> int __init {
    static int __init ohci_pxa27x_init(void)
    {
    if (usb_disabled())
    return -ENODEV;
    ohci_init_driver(&ohci_pxa27x_hc_driver, &pxa27x_overrides);
    ohci_pxa27x_hc_driver.hub_control = pxa27x_ohci_hub_control;
    return platform_driver_register(&ohci_hcd_pxa27x_driver);
    }
    module_init(ohci_pxa27x_init);
#[no_mangle]
unsafe extern "C" fn ohci_pxa27x_cleanup() -> void __exit {
    static void __exit ohci_pxa27x_cleanup(void)
    {
    platform_driver_unregister(&ohci_hcd_pxa27x_driver);
    }
    module_exit(ohci_pxa27x_cleanup);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:pxa27x-ohci");
