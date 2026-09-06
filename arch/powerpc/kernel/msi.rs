//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/msi.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2006-2007, Michael Ellerman, IBM Corporation.
//

#[no_mangle]
pub unsafe extern "C" fn arch_setup_msi_irqs(dev: *mut pci_dev, nvec: c_int, type: c_int) -> c_int {
    int arch_setup_msi_irqs(struct pci_dev *dev, int nvec, int type)
    {
    struct pci_controller *phb = pci_bus_to_host(dev.bus);
    if (!phb.controller_ops.setup_msi_irqs ||
    !phb.controller_ops.teardown_msi_irqs) {
    pr_debug("msi: Platform doesn't provide MSI callbacks.\n");
    return -ENOSYS;
    }
// PowerPC doesn't support multiple MSI yet
    if (type == PCI_CAP_ID_MSI && nvec > 1)
    return 1;
    return phb.controller_ops.setup_msi_irqs(dev, nvec, type);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_teardown_msi_irqs(dev: *mut pci_dev) {
    void arch_teardown_msi_irqs(struct pci_dev *dev)
    {
    struct pci_controller *phb = pci_bus_to_host(dev.bus);
//
// We can be called even when arch_setup_msi_irqs() returns -ENOSYS,
// so check the pointer again.
//
    if (phb.controller_ops.teardown_msi_irqs)
    phb.controller_ops.teardown_msi_irqs(dev);
    }
