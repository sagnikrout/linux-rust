//! Automatically rewritten from C to Rust
//! Source: drivers/pci/msi/legacy.c
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
// PCI Message Signaled Interrupt (MSI).
//
// Legacy architecture specific setup and teardown mechanism.
//

// Arch hooks
#[no_mangle]
pub unsafe extern "C" fn arch_setup_msi_irq(dev: *mut pci_dev, desc: *mut msi_desc) -> int __weak {
    int __weak arch_setup_msi_irq(struct pci_dev *dev, struct msi_desc *desc)
    {
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_teardown_msi_irq(irq: c_uint) -> void __weak {
    void __weak arch_teardown_msi_irq(unsigned int irq)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn arch_setup_msi_irqs(dev: *mut pci_dev, nvec: c_int, type: c_int) -> int __weak {
    int __weak arch_setup_msi_irqs(struct pci_dev *dev, int nvec, int type)
    {
    struct msi_desc *desc;
    int ret;
//
// If an architecture wants to support multiple MSI, it needs to
// override arch_setup_msi_irqs()
//
    if (type == PCI_CAP_ID_MSI && nvec > 1)
    return 1;
    msi_for_each_desc(desc, &dev.dev, MSI_DESC_NOTASSOCIATED) {
    ret = arch_setup_msi_irq(dev, desc);
    if (ret)
    return ret < 0 ? ret : -ENOSPC;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_teardown_msi_irqs(dev: *mut pci_dev) -> void __weak {
    void __weak arch_teardown_msi_irqs(struct pci_dev *dev)
    {
    struct msi_desc *desc;
    int i;
    msi_for_each_desc(desc, &dev.dev, MSI_DESC_ASSOCIATED) {
    for (i = 0; i < desc.nvec_used; i++)
    arch_teardown_msi_irq(desc.irq + i);
    }
    }
#[no_mangle]
unsafe extern "C" fn pci_msi_setup_check_result(dev: *mut pci_dev, type: c_int, ret: c_int) -> c_int {
    static int pci_msi_setup_check_result(struct pci_dev *dev, int type, int ret)
    {
    struct msi_desc *desc;
    let mut avail: c_int = 0;
    if (type != PCI_CAP_ID_MSIX || ret >= 0)
    return ret;
// Scan the MSI descriptors for successfully allocated ones.
    msi_for_each_desc(desc, &dev.dev, MSI_DESC_ASSOCIATED)
    avail++;
    return avail ? avail : ret;
    }
#[no_mangle]
pub unsafe extern "C" fn pci_msi_legacy_setup_msi_irqs(dev: *mut pci_dev, nvec: c_int, type: c_int) -> c_int {
    int pci_msi_legacy_setup_msi_irqs(struct pci_dev *dev, int nvec, int type)
    {
    let mut ret: c_int = arch_setup_msi_irqs(dev, nvec, type);
    ret = pci_msi_setup_check_result(dev, type, ret);
    if (!ret)
    ret = msi_device_populate_sysfs(&dev.dev);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn pci_msi_legacy_teardown_msi_irqs(dev: *mut pci_dev) {
    void pci_msi_legacy_teardown_msi_irqs(struct pci_dev *dev)
    {
    msi_device_destroy_sysfs(&dev.dev);
    arch_teardown_msi_irqs(dev);
    }
