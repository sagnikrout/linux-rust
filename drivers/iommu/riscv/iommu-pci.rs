//! Automatically rewritten from C to Rust
//! Source: drivers/iommu/riscv/iommu-pci.c
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
// Copyright © 2022-2024 Rivos Inc.
// Copyright © 2023 FORTH-ICS/CARV
//
// RISCV IOMMU as a PCIe device
//
// Authors
// Tomasz Jeznach <tjeznach@rivosinc.com>
// Nick Kossifidis <mick@ics.forth.gr>
//

// QEMU RISC-V IOMMU implementation
pub const PCI_DEVICE_ID_REDHAT_RISCV_IOMMU: c_uint = 0x0014;
// Rivos Inc. assigned PCI Vendor and Device IDs

pub const PCI_VENDOR_ID_RIVOS: c_uint = 0x1efd;

pub const PCI_DEVICE_ID_RIVOS_RISCV_IOMMU_GA: c_uint = 0x0008;
#[no_mangle]
unsafe extern "C" fn riscv_iommu_pci_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int riscv_iommu_pci_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    struct device *dev = &pdev.dev;
    struct riscv_iommu_device *iommu;
    int rc, vec;
    rc = pcim_enable_device(pdev);
    if (rc)
    return rc;
    if (!(pci_resource_flags(pdev, 0) & IORESOURCE_MEM))
    return -ENODEV;
    if (pci_resource_len(pdev, 0) < RISCV_IOMMU_REG_SIZE)
    return -ENODEV;
    rc = pcim_iomap_regions(pdev, BIT(0), pci_name(pdev));
    if (rc)
    return dev_err_probe(dev, rc, "pcim_iomap_regions failed\n");
    iommu = devm_kzalloc(dev, sizeof(*iommu), GFP_KERNEL);
    if (!iommu)
    return -ENOMEM;
    iommu.dev = dev;
    iommu.reg = pcim_iomap_table(pdev)[0];
    pci_set_master(pdev);
    dev_set_drvdata(dev, iommu);
// Check device reported capabilities / features.
    iommu.caps = riscv_iommu_readq(iommu, RISCV_IOMMU_REG_CAPABILITIES);
    iommu.fctl = riscv_iommu_readl(iommu, RISCV_IOMMU_REG_FCTL);
// The PCI driver only uses MSIs, make sure the IOMMU supports this
    switch (FIELD_GET(RISCV_IOMMU_CAPABILITIES_IGS, iommu.caps)) {
    case RISCV_IOMMU_CAPABILITIES_IGS_MSI:
    case RISCV_IOMMU_CAPABILITIES_IGS_BOTH:
    break;
    default:
    return dev_err_probe(dev, -ENODEV,
    "unable to use message-signaled interrupts\n");
    }
// Allocate and assign IRQ vectors for the various events
    rc = pci_alloc_irq_vectors(pdev, 1, RISCV_IOMMU_INTR_COUNT,
    PCI_IRQ_MSIX | PCI_IRQ_MSI);
    if (rc <= 0)
    return dev_err_probe(dev, -ENODEV,
    "unable to allocate irq vectors\n");
    iommu.irqs_count = rc;
    for (vec = 0; vec < iommu.irqs_count; vec++)
    iommu.irqs[vec] = msi_get_virq(dev, vec);
// Enable message-signaled interrupts, fctl.WSI
    if (iommu.fctl & RISCV_IOMMU_FCTL_WSI) {
    iommu.fctl ^= RISCV_IOMMU_FCTL_WSI;
    riscv_iommu_writel(iommu, RISCV_IOMMU_REG_FCTL, iommu.fctl);
    }
    return riscv_iommu_init(iommu);
    }
#[no_mangle]
unsafe extern "C" fn riscv_iommu_pci_remove(pdev: *mut pci_dev) {
    static void riscv_iommu_pci_remove(struct pci_dev *pdev)
    {
    struct riscv_iommu_device *iommu = dev_get_drvdata(&pdev.dev);
    riscv_iommu_remove(iommu);
    }
#[no_mangle]
unsafe extern "C" fn riscv_iommu_pci_shutdown(pdev: *mut pci_dev) {
    static void riscv_iommu_pci_shutdown(struct pci_dev *pdev)
    {
    struct riscv_iommu_device *iommu = dev_get_drvdata(&pdev.dev);
    riscv_iommu_disable(iommu);
    }
    static const struct pci_device_id riscv_iommu_pci_tbl[] = {
    { PCI_VDEVICE(REDHAT, PCI_DEVICE_ID_REDHAT_RISCV_IOMMU) },
    { PCI_VDEVICE(RIVOS, PCI_DEVICE_ID_RIVOS_RISCV_IOMMU_GA) },
    { }
    };
    static struct pci_driver riscv_iommu_pci_driver = {
    .name = KBUILD_MODNAME,
    .id_table = riscv_iommu_pci_tbl,
    .probe = riscv_iommu_pci_probe,
    .remove = riscv_iommu_pci_remove,
    .shutdown = riscv_iommu_pci_shutdown,
    .driver = {
    .suppress_bind_attrs = true,
    },
    };
    builtin_pci_driver(riscv_iommu_pci_driver);
