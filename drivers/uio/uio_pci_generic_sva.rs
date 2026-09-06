//! Automatically rewritten from C to Rust
//! Source: drivers/uio/uio_pci_generic_sva.c
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
// UIO PCI Express sva driver
//
// Copyright (c) 2025 Beijing Institute of Open Source Chip (BOSC)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uio_pci_sva_dev {
    pub pdev: *mut pci_dev,
    pub info: uio_info,
    pub sva_handle: *mut iommu_sva,
    pub pasid: c_int,
}

#[no_mangle]
unsafe extern "C" fn irq_handler(irq: c_int, dev_info: *mut uio_info) -> irqreturn_t {
    static irqreturn_t irq_handler(int irq, struct uio_info *dev_info)
    {
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn uio_pci_sva_open(info: *mut uio_info, inode: *mut inode) -> c_int {
    static int uio_pci_sva_open(struct uio_info *info, struct inode *inode)
    {
    struct iommu_sva *handle;
    struct uio_pci_sva_dev *udev = info.priv;
    struct iommu_domain *domain;
    if (!udev || !udev.pdev)
    return -ENODEV;
    domain = iommu_get_domain_for_dev(&udev.pdev.dev);
    if (domain)
    iommu_detach_device(domain, &udev.pdev.dev);
    handle = iommu_sva_bind_device(&udev.pdev.dev, current.mm);
    if (IS_ERR(handle))
    return -EINVAL;
    udev.pasid = iommu_sva_get_pasid(handle);
    udev.sva_handle = handle;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uio_pci_sva_release(info: *mut uio_info, inode: *mut inode) -> c_int {
    static int uio_pci_sva_release(struct uio_info *info, struct inode *inode)
    {
    struct uio_pci_sva_dev *udev = info.priv;
    if (!udev || !udev.pdev)
    return -ENODEV;
    iommu_sva_unbind_device(udev.sva_handle);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct uio_pci_sva_dev *udev;
    int ret, i, irq = 0;
    ret = pci_enable_device(pdev);
    if (ret) {
    dev_err(&pdev.dev, "pci_enable_device failed: %d\n", ret);
    return ret;
    }
    ret = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(64));
    if (ret)
    goto out_disable;
    pci_set_master(pdev);
    ret = pci_alloc_irq_vectors(pdev, 1, 1, PCI_IRQ_MSIX | PCI_IRQ_MSI);
    if (ret > 0) {
    irq = pci_irq_vector(pdev, 0);
    if (irq < 0) {
    dev_err(&pdev.dev, "Failed to get MSI vector\n");
    ret = irq;
    goto out_disable;
    }
    } else
    dev_warn(&pdev.dev,
    "No IRQ vectors available (%d), using polling\n", ret);
    udev = devm_kzalloc(&pdev.dev, sizeof(struct uio_pci_sva_dev),
    GFP_KERNEL);
    if (!udev) {
    ret =  -ENOMEM;
    goto out_disable;
    }
    udev.pdev = pdev;
    udev.info.name = "uio_pci_sva";
    udev.info.version = "0.0.1";
    udev.info.open = uio_pci_sva_open;
    udev.info.release = uio_pci_sva_release;
    udev.info.irq = irq;
    udev.info.handler = irq_handler;
    udev.info.priv = udev;
    for (i = 0; i < MAX_UIO_MAPS; i++) {
    struct resource *r = &pdev.resource[i];
    struct uio_mem *uiomem = &udev.info.mem[i];
    if (r.flags != (IORESOURCE_SIZEALIGN | IORESOURCE_MEM))
    continue;
    if (uiomem >= &udev.info.mem[MAX_UIO_MAPS]) {
    dev_warn(&pdev.dev, "Do not support more than %d iomem\n",
    MAX_UIO_MAPS);
    break;
    }
    uiomem.memtype = UIO_MEM_PHYS;
    uiomem.addr = r.start & PAGE_MASK;
    uiomem.offs = r.start & ~PAGE_MASK;
    uiomem.size =
    (uiomem.offs + resource_size(r) + PAGE_SIZE - 1) &
    PAGE_MASK;
    uiomem.name = r.name;
    }
    ret = devm_uio_register_device(&pdev.dev, &udev.info);
    if (ret) {
    dev_err(&pdev.dev, "Failed to register uio device\n");
    goto out_disable;
    }
    pci_set_drvdata(pdev, udev);
    return 0;
    out_disable:
    pci_disable_device(pdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn remove(pdev: *mut pci_dev) {
    static void remove(struct pci_dev *pdev)
    {
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    }
    static ssize_t pasid_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    struct uio_pci_sva_dev *udev = pci_get_drvdata(pdev);
    return sysfs_emit(buf, "%d\n", udev.pasid);
    }
    static DEVICE_ATTR_RO(pasid);
    static struct attribute *uio_pci_sva_attrs[] = {
    &dev_attr_pasid.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group uio_pci_sva_attr_group = {
    .attrs = uio_pci_sva_attrs,
    };
    static const struct attribute_group *uio_pci_sva_attr_groups[] = {
    &uio_pci_sva_attr_group,
    core::ptr::null_mut()
    };
    static struct pci_driver uio_pci_generic_sva_driver = {
    .name = "uio_pci_sva",
    .dev_groups = uio_pci_sva_attr_groups,
    .id_table = core::ptr::null_mut(),
    .probe = probe,
    .remove = remove,
    };
    module_pci_driver(uio_pci_generic_sva_driver);
    MODULE_VERSION("0.0.01");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Yaxing Guo <guoyaxing@bosc.ac.cn>");
    MODULE_DESCRIPTION("Generic UIO sva driver for PCI");
