//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/ohci-pci.c
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
//
// [ Initialisation is based on Linus'  ]
// [ uhci code and gregs ohci fragments ]
// [ (C) Copyright 1999 Linus Torvalds  ]
// [ (C) Copyright 1999 Gregory P. Smith]
//
// PCI Bus Glue
//
// This file is licenced under the GPL.
//

    static const char hcd_name[] = "ohci-pci";
// -------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn broken_suspend(hcd: *mut usb_hcd) -> c_int {
    static int broken_suspend(struct usb_hcd *hcd)
    {
    device_init_wakeup(&hcd.self.root_hub.dev, 0);
    return 0;
    }
// AMD 756, for most chips (early revs), corrupts register
// values on read ... so enable the vendor workaround.
//
#[no_mangle]
unsafe extern "C" fn ohci_quirk_amd756(hcd: *mut usb_hcd) -> c_int {
    static int ohci_quirk_amd756(struct usb_hcd *hcd)
    {
    struct ohci_hcd	*ohci = hcd_to_ohci (hcd);
    ohci.flags = OHCI_QUIRK_AMD756;
    ohci_dbg (ohci, "AMD756 erratum 4 workaround\n");
// also erratum 10 (suspend/resume issues)
    return broken_suspend(hcd);
    }
// Apple's OHCI driver has a lot of bizarre workarounds
// for this chip.  Evidently control and bulk lists
// can get confused.  (B&W G3 models, and ...)
//
#[no_mangle]
unsafe extern "C" fn ohci_quirk_opti(hcd: *mut usb_hcd) -> c_int {
    static int ohci_quirk_opti(struct usb_hcd *hcd)
    {
    struct ohci_hcd	*ohci = hcd_to_ohci (hcd);
    ohci_dbg (ohci, "WARNING: OPTi workarounds unavailable\n");
    return 0;
    }
// Check for NSC87560. We have to look at the bridge (fn1) to
// identify the USB (fn2). This quirk might apply to more or
// even all NSC stuff.
//
#[no_mangle]
unsafe extern "C" fn ohci_quirk_ns(hcd: *mut usb_hcd) -> c_int {
    static int ohci_quirk_ns(struct usb_hcd *hcd)
    {
    struct pci_dev *pdev = to_pci_dev(hcd.self.controller);
    struct pci_dev	*b;
    b  = pci_get_slot (pdev.bus, PCI_DEVFN (PCI_SLOT (pdev.devfn), 1));
    if (b && b.device == PCI_DEVICE_ID_NS_87560_LIO
    && b.vendor == PCI_VENDOR_ID_NS) {
    struct ohci_hcd	*ohci = hcd_to_ohci (hcd);
    ohci.flags |= OHCI_QUIRK_SUPERIO;
    ohci_dbg (ohci, "Using NSC SuperIO setup\n");
    }
    pci_dev_put(b);
    return 0;
    }
// Check for Compaq's ZFMicro chipset, which needs short
// delays before control or bulk queues get re-activated
// in finish_unlinks()
//
#[no_mangle]
unsafe extern "C" fn ohci_quirk_zfmicro(hcd: *mut usb_hcd) -> c_int {
    static int ohci_quirk_zfmicro(struct usb_hcd *hcd)
    {
    struct ohci_hcd	*ohci = hcd_to_ohci (hcd);
    ohci.flags |= OHCI_QUIRK_ZFMICRO;
    ohci_dbg(ohci, "enabled Compaq ZFMicro chipset quirks\n");
    return 0;
    }
// Check for Toshiba SCC OHCI which has big endian registers
// and little endian in memory data structures
//
#[no_mangle]
unsafe extern "C" fn ohci_quirk_toshiba_scc(hcd: *mut usb_hcd) -> c_int {
    static int ohci_quirk_toshiba_scc(struct usb_hcd *hcd)
    {
    struct ohci_hcd	*ohci = hcd_to_ohci (hcd);
// That chip is only present in the southbridge of some
// cell based platforms which are supposed to select
// CONFIG_USB_OHCI_BIG_ENDIAN_MMIO. We verify here if
// that was the case though.
//

    ohci.flags |= OHCI_QUIRK_BE_MMIO;
    ohci_dbg (ohci, "enabled big endian Toshiba quirk\n");
    return 0;

    ohci_err (ohci, "unsupported big endian Toshiba quirk\n");
    return -ENXIO;

    }
// Check for NEC chip and apply quirk for allegedly lost interrupts.
//
#[no_mangle]
unsafe extern "C" fn ohci_quirk_nec_worker(work: *mut work_struct) {
    static void ohci_quirk_nec_worker(struct work_struct *work)
    {
    struct ohci_hcd *ohci = container_of(work, struct ohci_hcd, nec_work);
    int status;
    status = ohci_restart(ohci);
    if (status != 0)
    ohci_err(ohci, "Restarting NEC controller failed in %s, %d\n",
    "ohci_restart", status);
    }
#[no_mangle]
unsafe extern "C" fn ohci_quirk_nec(hcd: *mut usb_hcd) -> c_int {
    static int ohci_quirk_nec(struct usb_hcd *hcd)
    {
    struct ohci_hcd	*ohci = hcd_to_ohci (hcd);
    ohci.flags |= OHCI_QUIRK_NEC;
    INIT_WORK(&ohci.nec_work, ohci_quirk_nec_worker);
    ohci_dbg (ohci, "enabled NEC chipset lost interrupt quirk\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ohci_quirk_amd700(hcd: *mut usb_hcd) -> c_int {
    static int ohci_quirk_amd700(struct usb_hcd *hcd)
    {
    struct ohci_hcd *ohci = hcd_to_ohci(hcd);
    if (usb_amd_quirk_pll_check())
    ohci.flags |= OHCI_QUIRK_AMD_PLL;
// SB800 needs pre-fetch fix
    if (usb_amd_prefetch_quirk()) {
    ohci.flags |= OHCI_QUIRK_AMD_PREFETCH;
    ohci_dbg(ohci, "enabled AMD prefetch quirk\n");
    }
    ohci.flags |= OHCI_QUIRK_GLOBAL_SUSPEND;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ohci_quirk_loongson(hcd: *mut usb_hcd) -> c_int {
    static int ohci_quirk_loongson(struct usb_hcd *hcd)
    {
    struct pci_dev *pdev = to_pci_dev(hcd.self.controller);
//
// Loongson's LS7A OHCI controller (rev 0x02) has a
// flaw. MMIO register with offset 0x60/64 is treated
// as legacy PS2-compatible keyboard/mouse interface.
// Since OHCI only use 4KB BAR resource, LS7A OHCI's
// 32KB BAR is wrapped around (the 2nd 4KB BAR space
// is the same as the 1st 4KB internally). So add 4KB
// offset (0x1000) to the OHCI registers as a quirk.
//
    if (pdev.revision == 0x2)
    hcd.regs += SZ_4K;	/* SZ_4K = 0x1000 */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ohci_quirk_qemu(hcd: *mut usb_hcd) -> c_int {
    static int ohci_quirk_qemu(struct usb_hcd *hcd)
    {
    struct ohci_hcd *ohci = hcd_to_ohci(hcd);
    ohci.flags |= OHCI_QUIRK_QEMU;
    ohci_dbg(ohci, "enabled qemu quirk\n");
    return 0;
    }
// List of quirks for OHCI
    static const struct pci_device_id ohci_pci_quirks[] = {
    {
    PCI_DEVICE(PCI_VENDOR_ID_AMD, 0x740c),
    .driver_data = (unsigned long)ohci_quirk_amd756,
    },
    {
    PCI_DEVICE(PCI_VENDOR_ID_OPTI, 0xc861),
    .driver_data = (unsigned long)ohci_quirk_opti,
    },
    {
    PCI_DEVICE(PCI_VENDOR_ID_NS, PCI_ANY_ID),
    .driver_data = (unsigned long)ohci_quirk_ns,
    },
    {
    PCI_DEVICE(PCI_VENDOR_ID_COMPAQ, 0xa0f8),
    .driver_data = (unsigned long)ohci_quirk_zfmicro,
    },
    {
    PCI_DEVICE(PCI_VENDOR_ID_TOSHIBA_2, 0x01b6),
    .driver_data = (unsigned long)ohci_quirk_toshiba_scc,
    },
    {
    PCI_DEVICE(PCI_VENDOR_ID_NEC, PCI_DEVICE_ID_NEC_USB),
    .driver_data = (unsigned long)ohci_quirk_nec,
    },
    {
// Toshiba portege 4000
    .vendor		= PCI_VENDOR_ID_AL,
    .device		= 0x5237,
    .subvendor	= PCI_VENDOR_ID_TOSHIBA,
    .subdevice	= 0x0004,
    .driver_data	= (unsigned long) broken_suspend,
    },
    {
    PCI_DEVICE(PCI_VENDOR_ID_ITE, 0x8152),
    .driver_data = (unsigned long) broken_suspend,
    },
    {
    PCI_DEVICE(PCI_VENDOR_ID_ATI, 0x4397),
    .driver_data = (unsigned long)ohci_quirk_amd700,
    },
    {
    PCI_DEVICE(PCI_VENDOR_ID_ATI, 0x4398),
    .driver_data = (unsigned long)ohci_quirk_amd700,
    },
    {
    PCI_DEVICE(PCI_VENDOR_ID_ATI, 0x4399),
    .driver_data = (unsigned long)ohci_quirk_amd700,
    },
    {
    PCI_DEVICE(PCI_VENDOR_ID_LOONGSON, 0x7a24),
    .driver_data = (unsigned long)ohci_quirk_loongson,
    },
    {
    .vendor		= PCI_VENDOR_ID_APPLE,
    .device		= 0x003f,
    .subvendor	= PCI_SUBVENDOR_ID_REDHAT_QUMRANET,
    .subdevice	= PCI_SUBDEVICE_ID_QEMU,
    .driver_data	= (unsigned long)ohci_quirk_qemu,
    },
    {},
    };
#[no_mangle]
unsafe extern "C" fn ohci_pci_reset(hcd: *mut usb_hcd) -> c_int {
    static int ohci_pci_reset (struct usb_hcd *hcd)
    {
    struct ohci_hcd	*ohci = hcd_to_ohci (hcd);
    struct pci_dev *pdev = to_pci_dev(hcd.self.controller);
    let mut ret: c_int = 0;
    if (hcd.self.controller) {
    const struct pci_device_id *quirk_id;
    quirk_id = pci_match_id(ohci_pci_quirks, pdev);
    if (quirk_id != core::ptr::null_mut()) {
    int (*quirk)(struct usb_hcd *ohci);
    quirk = (void *)quirk_id.driver_data;
    ret = quirk(hcd);
    }
    }
    if (ret == 0)
    ret = ohci_setup(hcd);
//
// After ohci setup RWC may not be set for add-in PCI cards.
// This transfers PCI PM wakeup capabilities.
//
    if (device_can_wakeup(&pdev.dev))
    ohci.hc_control |= OHCI_CTRL_RWC;
    return ret;
    }
    static struct hc_driver __read_mostly ohci_pci_hc_driver;
    static const struct ohci_driver_overrides pci_overrides __initconst = {
    .product_desc =		"OHCI PCI host controller",
    .reset =		ohci_pci_reset,
    };
    static const struct pci_device_id pci_ids[] = { {
// handle any USB OHCI controller
    PCI_DEVICE_CLASS(PCI_CLASS_SERIAL_USB_OHCI, ~0),
    }, {
// The device in the ConneXT I/O hub has no class reg
    PCI_VDEVICE(STMICRO, PCI_DEVICE_ID_STMICRO_USB_OHCI),
    }, { /* end: all zeroes */ }
    };
    MODULE_DEVICE_TABLE (pci, pci_ids);
#[no_mangle]
unsafe extern "C" fn ohci_pci_probe(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int ohci_pci_probe(struct pci_dev *dev, const struct pci_device_id *id)
    {
    return usb_hcd_pci_probe(dev, &ohci_pci_hc_driver);
    }
// pci driver glue; this is a "new style" PCI driver module
    static struct pci_driver ohci_pci_driver = {
    .name =		hcd_name,
    .id_table =	pci_ids,
    .probe =	ohci_pci_probe,
    .remove =	usb_hcd_pci_remove,
    .shutdown =	usb_hcd_pci_shutdown,

    .driver =	{
    .pm =	&usb_hcd_pci_pm_ops
    },

    };

#[no_mangle]
unsafe extern "C" fn ohci_pci_resume(hcd: *mut usb_hcd, msg: pm_message_t) -> c_int {
    static int ohci_pci_resume(struct usb_hcd *hcd, pm_message_t msg)
    {
    return ohci_resume(hcd, msg.event == PM_EVENT_RESTORE);
    }

#[no_mangle]
unsafe extern "C" fn ohci_pci_init() -> int __init {
    static int __init ohci_pci_init(void)
    {
    if (usb_disabled())
    return -ENODEV;
    ohci_init_driver(&ohci_pci_hc_driver, &pci_overrides);

// Entries for the PCI suspend/resume callbacks are special
    ohci_pci_hc_driver.pci_suspend = ohci_suspend;
    ohci_pci_hc_driver.pci_resume = ohci_pci_resume;

    return pci_register_driver(&ohci_pci_driver);
    }
    module_init(ohci_pci_init);
#[no_mangle]
unsafe extern "C" fn ohci_pci_cleanup() -> void __exit {
    static void __exit ohci_pci_cleanup(void)
    {
    pci_unregister_driver(&ohci_pci_driver);
    }
    module_exit(ohci_pci_cleanup);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
    MODULE_SOFTDEP("pre: ehci_pci");
