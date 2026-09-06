//! Automatically rewritten from C to Rust
//! Source: drivers/pci/msi/pcidev_msi.c
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
// MSI[X} related functions which are available unconditionally.
//

//
// Disable the MSI[X] hardware to avoid screaming interrupts during boot.
// This is the power on reset default so usually this should be a noop.
//
#[no_mangle]
pub unsafe extern "C" fn pci_msi_init(dev: *mut pci_dev) {
    void pci_msi_init(struct pci_dev *dev)
    {
    u16 ctrl;
    dev.msi_cap = pci_find_capability(dev, PCI_CAP_ID_MSI);
    if (!dev.msi_cap)
    return;
    pci_read_config_word(dev, dev.msi_cap + PCI_MSI_FLAGS, &ctrl);
    if (ctrl & PCI_MSI_FLAGS_ENABLE) {
    pci_write_config_word(dev, dev.msi_cap + PCI_MSI_FLAGS,
    ctrl & ~PCI_MSI_FLAGS_ENABLE);
    }
    if (!(ctrl & PCI_MSI_FLAGS_64BIT))
    dev.msi_addr_mask = DMA_BIT_MASK(32);
    }
#[no_mangle]
pub unsafe extern "C" fn pci_msix_init(dev: *mut pci_dev) {
    void pci_msix_init(struct pci_dev *dev)
    {
    u16 ctrl;
    dev.msix_cap = pci_find_capability(dev, PCI_CAP_ID_MSIX);
    if (!dev.msix_cap)
    return;
    pci_read_config_word(dev, dev.msix_cap + PCI_MSIX_FLAGS, &ctrl);
    if (ctrl & PCI_MSIX_FLAGS_ENABLE) {
    pci_write_config_word(dev, dev.msix_cap + PCI_MSIX_FLAGS,
    ctrl & ~PCI_MSIX_FLAGS_ENABLE);
    }
    }
