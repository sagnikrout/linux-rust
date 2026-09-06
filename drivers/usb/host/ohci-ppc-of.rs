//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/ohci-ppc-of.c
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
// (C) Copyright 2006 Sylvain Munaut <tnt@246tNt.com>
//
// Bus glue for OHCI HC on the of_platform bus
//
// Modified for of_platform bus from ohci-sa1111.c
//
// This file is licenced under the GPL.
//

    static int
    ohci_ppc_of_start(struct usb_hcd *hcd)
    {
    struct ohci_hcd	*ohci = hcd_to_ohci(hcd);
    int		ret;
    if ((ret = ohci_init(ohci)) < 0)
    return ret;
    if ((ret = ohci_run(ohci)) < 0) {
    dev_err(hcd.self.controller, "can't start %s\n",
    hcd.self.bus_name);
    ohci_stop(hcd);
    return ret;
    }
    return 0;
    }
    static const struct hc_driver ohci_ppc_of_hc_driver = {
    .description =		hcd_name,
    .product_desc =		"OF OHCI",
    .hcd_priv_size =	sizeof(struct ohci_hcd),
//
// generic hardware linkage
//
    .irq =			ohci_irq,
    .flags =		HCD_USB11 | HCD_DMA | HCD_MEMORY,
//
// basic lifecycle operations
//
    .start =		ohci_ppc_of_start,
    .stop =			ohci_stop,
    .shutdown = 		ohci_shutdown,
//
// managing i/o requests and associated device resources
//
    .urb_enqueue =		ohci_urb_enqueue,
    .urb_dequeue =		ohci_urb_dequeue,
    .endpoint_disable =	ohci_endpoint_disable,
//
// scheduling support
//
    .get_frame_number =	ohci_get_frame,
//
// root hub support
//
    .hub_status_data =	ohci_hub_status_data,
    .hub_control =		ohci_hub_control,

    .bus_suspend =		ohci_bus_suspend,
    .bus_resume =		ohci_bus_resume,

    .start_port_reset =	ohci_start_port_reset,
    };
#[no_mangle]
unsafe extern "C" fn ohci_hcd_ppc_of_probe(op: *mut platform_device) -> c_int {
    static int ohci_hcd_ppc_of_probe(struct platform_device *op)
    {
    struct device_node *dn = op.dev.of_node;
    struct usb_hcd *hcd;
    struct ohci_hcd	*ohci;
    struct resource res;
    int irq;
    int rv;
    int is_bigendian;
    struct device_node *np;
    if (usb_disabled())
    return -ENODEV;
    is_bigendian =
    of_device_is_compatible(dn, "ohci-bigendian") ||
    of_device_is_compatible(dn, "ohci-be");
    dev_dbg(&op.dev, "initializing PPC-OF USB Controller\n");
    rv = of_address_to_resource(dn, 0, &res);
    if (rv)
    return rv;
    hcd = usb_create_hcd(&ohci_ppc_of_hc_driver, &op.dev, "PPC-OF USB");
    if (!hcd)
    return -ENOMEM;
    hcd.rsrc_start = res.start;
    hcd.rsrc_len = resource_size(&res);
    hcd.regs = devm_ioremap_resource(&op.dev, &res);
    if (IS_ERR(hcd.regs)) {
    rv = PTR_ERR(hcd.regs);
    goto err_rmr;
    }
    irq = irq_of_parse_and_map(dn, 0);
    if (!irq) {
    dev_err(&op.dev, "%s: irq_of_parse_and_map failed\n",
    __FILE__);
    rv = -EBUSY;
    goto err_rmr;
    }
    ohci = hcd_to_ohci(hcd);
    if (is_bigendian) {
    ohci.flags |= OHCI_QUIRK_BE_MMIO | OHCI_QUIRK_BE_DESC;
    if (of_device_is_compatible(dn, "fsl,mpc5200-ohci"))
    ohci.flags |= OHCI_QUIRK_FRAME_NO;
    if (of_device_is_compatible(dn, "mpc5200-ohci"))
    ohci.flags |= OHCI_QUIRK_FRAME_NO;
    }
    ohci_hcd_init(ohci);
    rv = usb_add_hcd(hcd, irq, 0);
    if (rv == 0) {
    device_wakeup_enable(hcd.self.controller);
    return 0;
    }
// by now, 440epx is known to show usb_23 erratum
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,usb-ehci-440epx");
// Work around - At this point ohci_run has executed, the
// controller is running, everything, the root ports, etc., is
// set up.  If the ehci driver is loaded, put the ohci core in
// the suspended state.  The ehci driver will bring it out of
// suspended state when / if a non-high speed USB device is
// attached to the USB Host port.  If the ehci driver is not
// loaded, do nothing. request_mem_region is used to test if
// the ehci driver is loaded.
//
    if (np !=  core::ptr::null_mut()) {
    if (!of_address_to_resource(np, 0, &res)) {
    if (!request_mem_region(res.start, 0x4, hcd_name)) {
    writel_be((readl_be(&ohci.regs.control) |
    OHCI_USB_SUSPEND), &ohci.regs.control);
    (void) readl_be(&ohci.regs.control);
    } else
    release_mem_region(res.start, 0x4);
    } else
    pr_debug("%s: cannot get ehci offset from fdt\n", __FILE__);
    of_node_put(np);
    }
    irq_dispose_mapping(irq);
    err_rmr:
    usb_put_hcd(hcd);
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn ohci_hcd_ppc_of_remove(op: *mut platform_device) {
    static void ohci_hcd_ppc_of_remove(struct platform_device *op)
    {
    struct usb_hcd *hcd = platform_get_drvdata(op);
    dev_dbg(&op.dev, "stopping PPC-OF USB Controller\n");
    usb_remove_hcd(hcd);
    irq_dispose_mapping(hcd.irq);
    usb_put_hcd(hcd);
    }
    static const struct of_device_id ohci_hcd_ppc_of_match[] = {

    {
    .name = "usb",
    .compatible = "ohci-bigendian",
    },
    {
    .name = "usb",
    .compatible = "ohci-be",
    },

    {
    .name = "usb",
    .compatible = "ohci-le",
    },

    {},
    };
    MODULE_DEVICE_TABLE(of, ohci_hcd_ppc_of_match);

    !defined(CONFIG_USB_OHCI_HCD_PPC_OF_LE)

    static struct platform_driver ohci_hcd_ppc_of_driver = {
    .probe		= ohci_hcd_ppc_of_probe,
    .remove		= ohci_hcd_ppc_of_remove,
    .shutdown	= usb_hcd_platform_shutdown,
    .driver = {
    .name = "ppc-of-ohci",
    .of_match_table = ohci_hcd_ppc_of_match,
    },
    };
