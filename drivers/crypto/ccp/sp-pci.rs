//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/ccp/sp-pci.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// AMD Secure Processor device driver
//
// Copyright (C) 2013,2019 Advanced Micro Devices, Inc.
//
// Author: Tom Lendacky <thomas.lendacky@amd.com>
// Author: Gary R Hook <gary.hook@amd.com>
//

// used for version string AA.BB.CC.DD

pub const MSIX_VECTORS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp_pci {
    pub msix_count: c_int,
    pub msix_entry: [msix_entry; MSIX_VECTORS],
}

    static struct sp_device *sp_dev_master;

    static ssize_t name##_show(struct device *d, struct device_attribute *attr,	\
    char *buf)						\
    {										\
    struct sp_device *sp = dev_get_drvdata(d);				\
    struct psp_device *psp = sp.psp_data;					\
    unsigned int val = ioread32(psp.io_regs + _offset);			\
    return sysfs_emit(buf, "%02lx.%02lx.%02lx.%02lx\n",			\
    FIELD_GET(AA, val),			\
    FIELD_GET(BB, val),			\
    FIELD_GET(CC, val),			\
    FIELD_GET(DD, val));			\
    }
    version_attribute_show(bootloader_version, psp.vdata.bootloader_info_reg)
    static DEVICE_ATTR_RO(bootloader_version);
    version_attribute_show(tee_version, psp.vdata.tee.info_reg)
    static DEVICE_ATTR_RO(tee_version);
    static struct attribute *psp_firmware_attrs[] = {
    &dev_attr_bootloader_version.attr,
    &dev_attr_tee_version.attr,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn psp_firmware_is_visible(kobj: *mut kobject, attr: *mut attribute, idx: c_int) -> umode_t {
    static umode_t psp_firmware_is_visible(struct kobject *kobj, struct attribute *attr, int idx)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct sp_device *sp = dev_get_drvdata(dev);
    struct psp_device *psp = sp.psp_data;
    let mut val: c_uint = 0xffffffff;
    if (!psp)
    return 0;
    if (attr == &dev_attr_bootloader_version.attr &&
    psp.vdata.bootloader_info_reg)
    val = ioread32(psp.io_regs + psp.vdata.bootloader_info_reg);
    if (attr == &dev_attr_tee_version.attr && psp.capability.tee &&
    psp.vdata.tee.info_reg)
    val = ioread32(psp.io_regs + psp.vdata.tee.info_reg);
// If platform disallows accessing this register it will be all f's
    if (val != 0xffffffff)
    return 0444;
    return 0;
    }
    static struct attribute_group psp_firmware_attr_group = {
    .attrs = psp_firmware_attrs,
    .is_visible = psp_firmware_is_visible,
    };
    static const struct attribute_group *psp_groups[] = {

    &psp_security_attr_group,

    &psp_firmware_attr_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn sp_get_msix_irqs(sp: *mut sp_device) -> c_int {
    static int sp_get_msix_irqs(struct sp_device *sp)
    {
    struct sp_pci *sp_pci = sp.dev_specific;
    struct device *dev = sp.dev;
    struct pci_dev *pdev = to_pci_dev(dev);
    int v, ret;
    for (v = 0; v < ARRAY_SIZE(sp_pci.msix_entry); v++)
    sp_pci.msix_entry[v].entry = v;
    ret = pci_enable_msix_range(pdev, sp_pci.msix_entry, 1, v);
    if (ret < 0)
    return ret;
    sp_pci.msix_count = ret;
    sp.use_tasklet = true;
    sp.psp_irq = sp_pci.msix_entry[0].vector;
    sp.ccp_irq = (sp_pci.msix_count > 1) ? sp_pci.msix_entry[1].vector
    : sp_pci.msix_entry[0].vector;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sp_get_msi_irq(sp: *mut sp_device) -> c_int {
    static int sp_get_msi_irq(struct sp_device *sp)
    {
    struct device *dev = sp.dev;
    struct pci_dev *pdev = to_pci_dev(dev);
    int ret;
    ret = pci_enable_msi(pdev);
    if (ret)
    return ret;
    sp.ccp_irq = pdev.irq;
    sp.psp_irq = pdev.irq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sp_get_irqs(sp: *mut sp_device) -> c_int {
    static int sp_get_irqs(struct sp_device *sp)
    {
    struct device *dev = sp.dev;
    int ret;
    ret = sp_get_msix_irqs(sp);
    if (!ret)
    return 0;
// Couldn't get MSI-X vectors, try MSI
    dev_notice(dev, "could not enable MSI-X (%d), trying MSI\n", ret);
    ret = sp_get_msi_irq(sp);
    if (!ret)
    return 0;
// Couldn't get MSI interrupt
    dev_notice(dev, "could not enable MSI (%d)\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sp_free_irqs(sp: *mut sp_device) {
    static void sp_free_irqs(struct sp_device *sp)
    {
    struct sp_pci *sp_pci = sp.dev_specific;
    struct device *dev = sp.dev;
    struct pci_dev *pdev = to_pci_dev(dev);
    if (sp_pci.msix_count)
    pci_disable_msix(pdev);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sp->psp_irq) -> else {
    else if (sp.psp_irq)
    pci_disable_msi(pdev);
    sp.ccp_irq = 0;
    sp.psp_irq = 0;
    }
#[no_mangle]
unsafe extern "C" fn sp_pci_is_master(sp: *mut sp_device) -> bool {
    static bool sp_pci_is_master(struct sp_device *sp)
    {
    struct device *dev_cur, *dev_new;
    struct pci_dev *pdev_cur, *pdev_new;
    dev_new = sp.dev;
    dev_cur = sp_dev_master.dev;
    pdev_new = to_pci_dev(dev_new);
    pdev_cur = to_pci_dev(dev_cur);
    if (pci_domain_nr(pdev_new.bus) != pci_domain_nr(pdev_cur.bus))
    return pci_domain_nr(pdev_new.bus) < pci_domain_nr(pdev_cur.bus);
    if (pdev_new.bus.number != pdev_cur.bus.number)
    return pdev_new.bus.number < pdev_cur.bus.number;
    if (PCI_SLOT(pdev_new.devfn) != PCI_SLOT(pdev_cur.devfn))
    return PCI_SLOT(pdev_new.devfn) < PCI_SLOT(pdev_cur.devfn);
    if (PCI_FUNC(pdev_new.devfn) != PCI_FUNC(pdev_cur.devfn))
    return PCI_FUNC(pdev_new.devfn) < PCI_FUNC(pdev_cur.devfn);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn psp_set_master(sp: *mut sp_device) {
    static void psp_set_master(struct sp_device *sp)
    {
    if (!sp_dev_master) {
    sp_dev_master = sp;
    return;
    }
    if (sp_pci_is_master(sp))
    sp_dev_master = sp;
    }
    static struct sp_device *psp_get_master(void)
    {
    return sp_dev_master;
    }
#[no_mangle]
unsafe extern "C" fn psp_clear_master(sp: *mut sp_device) {
    static void psp_clear_master(struct sp_device *sp)
    {
    if (sp == sp_dev_master) {
    sp_dev_master = core::ptr::null_mut();
    dev_dbg(sp.dev, "Cleared sp_dev_master\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn sp_pci_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int sp_pci_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct sp_device *sp;
    struct sp_pci *sp_pci;
    struct device *dev = &pdev.dev;
    void __iomem * const *iomap_table;
    int bar_mask;
    int ret;
    ret = -ENOMEM;
    sp = sp_alloc_struct(dev);
    if (!sp)
    goto e_err;
    sp_pci = devm_kzalloc(dev, sizeof(*sp_pci), GFP_KERNEL);
    if (!sp_pci)
    goto e_err;
    sp.dev_specific = sp_pci;
    sp.dev_vdata = (struct sp_dev_vdata *)id.driver_data;
    if (!sp.dev_vdata) {
    ret = -ENODEV;
    dev_err(dev, "missing driver data\n");
    goto e_err;
    }
    ret = pcim_enable_device(pdev);
    if (ret) {
    dev_err(dev, "pcim_enable_device failed (%d)\n", ret);
    goto e_err;
    }
    bar_mask = pci_select_bars(pdev, IORESOURCE_MEM);
    ret = pcim_iomap_regions(pdev, bar_mask, "ccp");
    if (ret) {
    dev_err(dev, "pcim_iomap_regions failed (%d)\n", ret);
    goto e_err;
    }
    iomap_table = pcim_iomap_table(pdev);
    if (!iomap_table) {
    dev_err(dev, "pcim_iomap_table failed\n");
    ret = -ENOMEM;
    goto e_err;
    }
    sp.io_map = iomap_table[sp.dev_vdata.bar];
    if (!sp.io_map) {
    dev_err(dev, "ioremap failed\n");
    ret = -ENOMEM;
    goto e_err;
    }
    ret = sp_get_irqs(sp);
    if (ret)
    goto e_err;
    pci_set_master(pdev);
    sp.set_psp_master_device = psp_set_master;
    sp.get_psp_master_device = psp_get_master;
    sp.clear_psp_master_device = psp_clear_master;
    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(48));
    if (ret) {
    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(32));
    if (ret) {
    dev_err(dev, "dma_set_mask_and_coherent failed (%d)\n",
    ret);
    goto free_irqs;
    }
    }
    dev_set_drvdata(dev, sp);
    ret = sp_init(sp);
    if (ret)
    goto free_irqs;
    return 0;
    free_irqs:
    sp_free_irqs(sp);
    e_err:
    dev_notice(dev, "initialization failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sp_pci_shutdown(pdev: *mut pci_dev) {
    static void sp_pci_shutdown(struct pci_dev *pdev)
    {
    struct device *dev = &pdev.dev;
    struct sp_device *sp = dev_get_drvdata(dev);
    if (!sp)
    return;
    sp_destroy(sp);
    }
#[no_mangle]
unsafe extern "C" fn sp_pci_remove(pdev: *mut pci_dev) {
    static void sp_pci_remove(struct pci_dev *pdev)
    {
    struct device *dev = &pdev.dev;
    struct sp_device *sp = dev_get_drvdata(dev);
    if (!sp)
    return;
    sp_destroy(sp);
    sp_free_irqs(sp);
    }
#[no_mangle]
unsafe extern "C" fn sp_pci_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused sp_pci_suspend(struct device *dev)
    {
    struct sp_device *sp = dev_get_drvdata(dev);
    return sp_suspend(sp);
    }
#[no_mangle]
unsafe extern "C" fn sp_pci_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused sp_pci_resume(struct device *dev)
    {
    struct sp_device *sp = dev_get_drvdata(dev);
    return sp_resume(sp);
    }
#[no_mangle]
unsafe extern "C" fn sp_pci_restore(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused sp_pci_restore(struct device *dev)
    {
    struct sp_device *sp = dev_get_drvdata(dev);
    return sp_restore(sp);
    }

    static const struct sev_vdata sevv1 = {
    .cmdresp_reg		= 0x10580,	/* C2PMSG_32 */
    .cmdbuff_addr_lo_reg	= 0x105e0,	/* C2PMSG_56 */
    .cmdbuff_addr_hi_reg	= 0x105e4,	/* C2PMSG_57 */
    };
    static const struct sev_vdata sevv2 = {
    .cmdresp_reg		= 0x10980,	/* C2PMSG_32 */
    .cmdbuff_addr_lo_reg	= 0x109e0,	/* C2PMSG_56 */
    .cmdbuff_addr_hi_reg	= 0x109e4,	/* C2PMSG_57 */
    };
    static const struct tee_vdata teev1 = {
    .ring_wptr_reg          = 0x10550,	/* C2PMSG_20 */
    .ring_rptr_reg          = 0x10554,	/* C2PMSG_21 */
    .info_reg		= 0x109e8,	/* C2PMSG_58 */
    };
    static const struct tee_vdata teev2 = {
    .ring_wptr_reg		= 0x10950,	/* C2PMSG_20 */
    .ring_rptr_reg		= 0x10954,	/* C2PMSG_21 */
    .info_reg		= 0x109e8,	/* C2PMSG_58 */
    };
    static const struct platform_access_vdata pa_v1 = {
    .cmdresp_reg		= 0x10570,	/* C2PMSG_28 */
    .cmdbuff_addr_lo_reg	= 0x10574,	/* C2PMSG_29 */
    .cmdbuff_addr_hi_reg	= 0x10578,	/* C2PMSG_30 */
    .doorbell_button_reg	= 0x10a24,	/* C2PMSG_73 */
    .doorbell_cmd_reg	= 0x10a40,	/* C2PMSG_80 */
    };
    static const struct platform_access_vdata pa_v2 = {
    .doorbell_button_reg	= 0x10a24,	/* C2PMSG_73 */
    .doorbell_cmd_reg	= 0x10a40,	/* C2PMSG_80 */
    };
    static const struct psp_vdata pspv1 = {
    .sev			= &sevv1,
    .bootloader_info_reg	= 0x105ec,	/* C2PMSG_59 */
    .feature_reg		= 0x105fc,	/* C2PMSG_63 */
    .inten_reg		= 0x10610,	/* P2CMSG_INTEN */
    .intsts_reg		= 0x10614,	/* P2CMSG_INTSTS */
    };
    static const struct psp_vdata pspv2 = {
    .sev			= &sevv2,
    .platform_access	= &pa_v1,
    .bootloader_info_reg	= 0x109ec,	/* C2PMSG_59 */
    .feature_reg		= 0x109fc,	/* C2PMSG_63 */
    .inten_reg		= 0x10690,	/* P2CMSG_INTEN */
    .intsts_reg		= 0x10694,	/* P2CMSG_INTSTS */
    .platform_features	= PLATFORM_FEATURE_HSTI,
    };
    static const struct psp_vdata pspv3 = {
    .tee			= &teev1,
    .platform_access	= &pa_v1,
    .cmdresp_reg		= 0x10544,	/* C2PMSG_17 */
    .cmdbuff_addr_lo_reg	= 0x10548,	/* C2PMSG_18 */
    .cmdbuff_addr_hi_reg	= 0x1054c,	/* C2PMSG_19 */
    .bootloader_info_reg	= 0x109ec,	/* C2PMSG_59 */
    .feature_reg		= 0x109fc,	/* C2PMSG_63 */
    .inten_reg		= 0x10690,	/* P2CMSG_INTEN */
    .intsts_reg		= 0x10694,	/* P2CMSG_INTSTS */
    .platform_features	= PLATFORM_FEATURE_DBC |
    PLATFORM_FEATURE_HSTI,
    };
    static const struct psp_vdata pspv4 = {
    .sev			= &sevv2,
    .tee			= &teev1,
    .cmdresp_reg		= 0x10544,	/* C2PMSG_17 */
    .cmdbuff_addr_lo_reg	= 0x10548,	/* C2PMSG_18 */
    .cmdbuff_addr_hi_reg	= 0x1054c,	/* C2PMSG_19 */
    .bootloader_info_reg	= 0x109ec,	/* C2PMSG_59 */
    .feature_reg		= 0x109fc,	/* C2PMSG_63 */
    .inten_reg		= 0x10690,	/* P2CMSG_INTEN */
    .intsts_reg		= 0x10694,	/* P2CMSG_INTSTS */
    };
    static const struct psp_vdata pspv5 = {
    .tee			= &teev2,
    .platform_access	= &pa_v2,
    .cmdresp_reg		= 0x10944,	/* C2PMSG_17 */
    .cmdbuff_addr_lo_reg	= 0x10948,	/* C2PMSG_18 */
    .cmdbuff_addr_hi_reg	= 0x1094c,	/* C2PMSG_19 */
    .bootloader_info_reg	= 0x109ec,	/* C2PMSG_59 */
    .feature_reg		= 0x109fc,	/* C2PMSG_63 */
    .inten_reg		= 0x10510,	/* P2CMSG_INTEN */
    .intsts_reg		= 0x10514,	/* P2CMSG_INTSTS */
    };
    static const struct psp_vdata pspv6 = {
    .sev                    = &sevv2,
    .tee                    = &teev2,
    .cmdresp_reg		= 0x10944,	/* C2PMSG_17 */
    .cmdbuff_addr_lo_reg	= 0x10948,	/* C2PMSG_18 */
    .cmdbuff_addr_hi_reg	= 0x1094c,	/* C2PMSG_19 */
    .bootloader_info_reg	= 0x109ec,	/* C2PMSG_59 */
    .feature_reg            = 0x109fc,	/* C2PMSG_63 */
    .inten_reg              = 0x10510,	/* P2CMSG_INTEN */
    .intsts_reg             = 0x10514,	/* P2CMSG_INTSTS */
    };
    static const struct psp_vdata pspv7 = {
    .tee			= &teev2,
    .cmdresp_reg		= 0x10944,	/* C2PMSG_17 */
    .cmdbuff_addr_lo_reg	= 0x10948,	/* C2PMSG_18 */
    .cmdbuff_addr_hi_reg	= 0x1094c,	/* C2PMSG_19 */
    .bootloader_info_reg	= 0x109ec,	/* C2PMSG_59 */
    .feature_reg		= 0x109fc,	/* C2PMSG_63 */
    .inten_reg		= 0x10510,	/* P2CMSG_INTEN */
    .intsts_reg		= 0x10514,	/* P2CMSG_INTSTS */
    };

    static const struct sp_dev_vdata dev_vdata[] = {
    {	/* 0 */
    .bar = 2,

    .ccp_vdata = &ccpv3,

    },
    {	/* 1 */
    .bar = 2,

    .ccp_vdata = &ccpv5a,

    .psp_vdata = &pspv1,

    },
    {	/* 2 */
    .bar = 2,

    .ccp_vdata = &ccpv5b,

    },
    {	/* 3 */
    .bar = 2,

    .ccp_vdata = &ccpv5a,

    .psp_vdata = &pspv2,

    },
    {	/* 4 */
    .bar = 2,

    .ccp_vdata = &ccpv5a,

    .psp_vdata = &pspv3,

    },
    {	/* 5 */
    .bar = 2,

    .psp_vdata = &pspv4,

    },
    {	/* 6 */
    .bar = 2,

    .psp_vdata = &pspv3,

    },
    {	/* 7 */
    .bar = 2,

    .psp_vdata = &pspv5,

    },
    {	/* 8 */
    .bar = 2,

    .psp_vdata = &pspv6,

    },
    {	/* 9 */
    .bar = 2,

    .psp_vdata = &pspv7,

    },
    };
    static const struct pci_device_id sp_pci_table[] = {
    { PCI_VDEVICE(AMD, 0x1537), .driver_data = (kernel_ulong_t)&dev_vdata[0] },
    { PCI_VDEVICE(AMD, 0x1456), .driver_data = (kernel_ulong_t)&dev_vdata[1] },
    { PCI_VDEVICE(AMD, 0x1468), .driver_data = (kernel_ulong_t)&dev_vdata[2] },
    { PCI_VDEVICE(AMD, 0x1486), .driver_data = (kernel_ulong_t)&dev_vdata[3] },
    { PCI_VDEVICE(AMD, 0x15DF), .driver_data = (kernel_ulong_t)&dev_vdata[4] },
    { PCI_VDEVICE(AMD, 0x14CA), .driver_data = (kernel_ulong_t)&dev_vdata[5] },
    { PCI_VDEVICE(AMD, 0x15C7), .driver_data = (kernel_ulong_t)&dev_vdata[6] },
    { PCI_VDEVICE(AMD, 0x1649), .driver_data = (kernel_ulong_t)&dev_vdata[6] },
    { PCI_VDEVICE(AMD, 0x1134), .driver_data = (kernel_ulong_t)&dev_vdata[7] },
    { PCI_VDEVICE(AMD, 0x17E0), .driver_data = (kernel_ulong_t)&dev_vdata[7] },
    { PCI_VDEVICE(AMD, 0x156E), .driver_data = (kernel_ulong_t)&dev_vdata[8] },
    { PCI_VDEVICE(AMD, 0x17D8), .driver_data = (kernel_ulong_t)&dev_vdata[8] },
    { PCI_VDEVICE(AMD, 0x115A), .driver_data = (kernel_ulong_t)&dev_vdata[9] },
// Last entry must be zero
    { }
    };
    MODULE_DEVICE_TABLE(pci, sp_pci_table);
    static const struct dev_pm_ops sp_pci_pm_ops = {
    .suspend = pm_sleep_ptr(sp_pci_suspend),
    .resume = pm_sleep_ptr(sp_pci_resume),
    .freeze = pm_sleep_ptr(sp_pci_suspend),
    .thaw = pm_sleep_ptr(sp_pci_resume),
    .poweroff = pm_sleep_ptr(sp_pci_suspend),
    .restore_early = pm_sleep_ptr(sp_pci_restore),
    };
    static struct pci_driver sp_pci_driver = {
    .name = "ccp",
    .id_table = sp_pci_table,
    .probe = sp_pci_probe,
    .remove = sp_pci_remove,
    .shutdown = sp_pci_shutdown,
    .driver.pm = &sp_pci_pm_ops,
    .dev_groups = psp_groups,
    };
#[no_mangle]
pub unsafe extern "C" fn sp_pci_init() -> c_int {
    int sp_pci_init(void)
    {
    return pci_register_driver(&sp_pci_driver);
    }
#[no_mangle]
pub unsafe extern "C" fn sp_pci_exit() {
    void sp_pci_exit(void)
    {
    pci_unregister_driver(&sp_pci_driver);
    }
