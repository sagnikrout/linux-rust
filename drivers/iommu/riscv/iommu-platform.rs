//! Automatically rewritten from C to Rust
//! Source: drivers/iommu/riscv/iommu-platform.c
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
// RISC-V IOMMU as a platform device
//
// Copyright © 2023 FORTH-ICS/CARV
// Copyright © 2023-2024 Rivos Inc.
//
// Authors
// Nick Kossifidis <mick@ics.forth.gr>
// Tomasz Jeznach <tjeznach@rivosinc.com>
//

#[no_mangle]
unsafe extern "C" fn riscv_iommu_write_msi_msg(desc: *mut msi_desc, msg: *mut msi_msg) {
    static void riscv_iommu_write_msi_msg(struct msi_desc *desc, struct msi_msg *msg)
    {
    struct device *dev = msi_desc_to_dev(desc);
    struct riscv_iommu_device *iommu = dev_get_drvdata(dev);
    let mut idx: u16 = desc.msi_index;
    u64 addr;
    addr = ((u64)msg.address_hi << 32) | msg.address_lo;
    if (addr != (addr & RISCV_IOMMU_MSI_CFG_TBL_ADDR)) {
    dev_err_once(dev,
    "uh oh, the IOMMU can't send MSIs to 0x%llx, sending to 0x%llx instead\n",
    addr, addr & RISCV_IOMMU_MSI_CFG_TBL_ADDR);
    }
    addr &= RISCV_IOMMU_MSI_CFG_TBL_ADDR;
    riscv_iommu_writeq(iommu, RISCV_IOMMU_REG_MSI_CFG_TBL_ADDR(idx), addr);
    riscv_iommu_writel(iommu, RISCV_IOMMU_REG_MSI_CFG_TBL_DATA(idx), msg.data);
    riscv_iommu_writel(iommu, RISCV_IOMMU_REG_MSI_CFG_TBL_CTRL(idx), 0);
    }
#[no_mangle]
unsafe extern "C" fn riscv_iommu_platform_probe(pdev: *mut platform_device) -> c_int {
    static int riscv_iommu_platform_probe(struct platform_device *pdev)
    {
    enum riscv_iommu_igs_settings igs;
    struct device *dev = &pdev.dev;
    struct riscv_iommu_device *iommu = core::ptr::null_mut();
    struct irq_domain *msi_domain;
    struct resource *res = core::ptr::null_mut();
    int vec, ret;
    iommu = devm_kzalloc(dev, sizeof(*iommu), GFP_KERNEL);
    if (!iommu)
    return -ENOMEM;
    iommu.dev = dev;
    iommu.reg = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(iommu.reg))
    return dev_err_probe(dev, PTR_ERR(iommu.reg),
    "could not map register region\n");
    dev_set_drvdata(dev, iommu);
// Check device reported capabilities / features.
    iommu.caps = riscv_iommu_readq(iommu, RISCV_IOMMU_REG_CAPABILITIES);
    iommu.fctl = riscv_iommu_readl(iommu, RISCV_IOMMU_REG_FCTL);
    iommu.irqs_count = RISCV_IOMMU_INTR_COUNT;
    igs = FIELD_GET(RISCV_IOMMU_CAPABILITIES_IGS, iommu.caps);
    switch (igs) {
    case RISCV_IOMMU_CAPABILITIES_IGS_BOTH:
    case RISCV_IOMMU_CAPABILITIES_IGS_MSI:
    if (is_of_node(dev_fwnode(dev))) {
    of_msi_configure(dev, to_of_node(dev.fwnode));
    } else {
    msi_domain = irq_find_matching_fwnode(imsic_acpi_get_fwnode(dev),
    DOMAIN_BUS_PLATFORM_MSI);
    dev_set_msi_domain(dev, msi_domain);
    }
    if (!dev_get_msi_domain(dev)) {
    dev_warn(dev, "failed to find an MSI domain\n");
    goto msi_fail;
    }
    ret = platform_device_msi_init_and_alloc_irqs(dev, iommu.irqs_count,
    riscv_iommu_write_msi_msg);
    if (ret) {
    dev_warn(dev, "failed to allocate MSIs\n");
    goto msi_fail;
    }
    for (vec = 0; vec < iommu.irqs_count; vec++)
    iommu.irqs[vec] = msi_get_virq(dev, vec);
// Enable message-signaled interrupts, fctl.WSI
    if (iommu.fctl & RISCV_IOMMU_FCTL_WSI) {
    iommu.fctl ^= RISCV_IOMMU_FCTL_WSI;
    riscv_iommu_writel(iommu, RISCV_IOMMU_REG_FCTL, iommu.fctl);
    }
    dev_info(dev, "using MSIs\n");
    break;
    msi_fail:
    if (igs != RISCV_IOMMU_CAPABILITIES_IGS_BOTH) {
    return dev_err_probe(dev, -ENODEV,
    "unable to use wire-signaled interrupts\n");
    }
    fallthrough;
    case RISCV_IOMMU_CAPABILITIES_IGS_WSI:
    ret = platform_irq_count(pdev);
    if (ret <= 0)
    return dev_err_probe(dev, -ENODEV,
    "no IRQ resources provided\n");
    iommu.irqs_count = ret;
    if (iommu.irqs_count > RISCV_IOMMU_INTR_COUNT)
    iommu.irqs_count = RISCV_IOMMU_INTR_COUNT;
    for (vec = 0; vec < iommu.irqs_count; vec++)
    iommu.irqs[vec] = platform_get_irq(pdev, vec);
// Enable wire-signaled interrupts, fctl.WSI
    if (!(iommu.fctl & RISCV_IOMMU_FCTL_WSI)) {
    iommu.fctl |= RISCV_IOMMU_FCTL_WSI;
    riscv_iommu_writel(iommu, RISCV_IOMMU_REG_FCTL, iommu.fctl);
    }
    dev_info(dev, "using wire-signaled interrupts\n");
    break;
    default:
    return dev_err_probe(dev, -ENODEV, "invalid IGS\n");
    }
    return riscv_iommu_init(iommu);
    };
#[no_mangle]
unsafe extern "C" fn riscv_iommu_platform_remove(pdev: *mut platform_device) {
    static void riscv_iommu_platform_remove(struct platform_device *pdev)
    {
    struct riscv_iommu_device *iommu = dev_get_drvdata(&pdev.dev);
    let mut msi: bool = !(iommu.fctl & RISCV_IOMMU_FCTL_WSI);
    riscv_iommu_remove(iommu);
    if (msi)
    platform_device_msi_free_irqs_all(&pdev.dev);
    };
#[no_mangle]
unsafe extern "C" fn riscv_iommu_platform_shutdown(pdev: *mut platform_device) {
    static void riscv_iommu_platform_shutdown(struct platform_device *pdev)
    {
    riscv_iommu_disable(dev_get_drvdata(&pdev.dev));
    };
    static const struct of_device_id riscv_iommu_of_match[] = {
    {.compatible = "riscv,iommu",},
    {},
    };
    static const struct acpi_device_id riscv_iommu_acpi_match[] = {
    { "RSCV0004", 0 },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, riscv_iommu_acpi_match);
    static struct platform_driver riscv_iommu_platform_driver = {
    .probe = riscv_iommu_platform_probe,
    .remove = riscv_iommu_platform_remove,
    .shutdown = riscv_iommu_platform_shutdown,
    .driver = {
    .name = "riscv,iommu",
    .of_match_table = riscv_iommu_of_match,
    .suppress_bind_attrs = true,
    .acpi_match_table = riscv_iommu_acpi_match,
    },
    };
    builtin_platform_driver(riscv_iommu_platform_driver);
