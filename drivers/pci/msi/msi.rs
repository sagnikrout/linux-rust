//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/msi/msi.h
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

extern "C" {
    pub fn pci_msi_setup_msi_irqs(dev: *mut pci_dev, nvec: c_int, type: c_int) -> c_int;
}
extern "C" {
    pub fn pci_msi_teardown_msi_irqs(dev: *mut pci_dev);
}
// Mask/unmask helpers
extern "C" {
    pub fn pci_msi_update_mask(desc: *mut msi_desc, clear: u32, set: u32);
}
//
// This internal function does not flush PCI writes to the device.  All
// users must ensure that they read from the device before either assuming
// that the device state is up to date, or returning out of this file.
// It does not affect the msi_desc::msix_ctrl cache either. Use with care!
//
// Flush write to device
//
// PCI 2.3 does not specify mask bits for each MSI interrupt.  Attempting to
// mask all MSI interrupts by clearing the MSI enable bit does not work
// reliably as devices without an INTx disable bit will then generate a
// level IRQ which will never be cleared.
//
// Don't shift by >= width of type
extern "C" {
    pub fn msix_prepare_msi_desc(dev: *mut pci_dev, desc: *mut msi_desc);
}
// Subsystem variables
// MSI internal functions invoked from the public APIs
extern "C" {
    pub fn pci_msi_shutdown(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_msix_shutdown(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_free_msi_irqs(dev: *mut pci_dev);
}
extern "C" {
    pub fn __pci_enable_msi_range(dev: *mut pci_dev, minvec: c_int, maxvec: c_int, affd: *mut irq_affinity) -> c_int;
}
extern "C" {
    pub fn __pci_restore_msi_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn __pci_restore_msix_state(dev: *mut pci_dev);
}
// irq_domain related functionality
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum support_mode {
    ALLOW_LEGACY,
    DENY_LEGACY,
}

extern "C" {
    pub fn pci_msi_domain_supports(dev: *mut pci_dev, feature_mask: c_uint, mode: support_mode) -> bool;
}
extern "C" {
    pub fn pci_setup_msi_device_domain(pdev: *mut pci_dev, hwsize: c_uint) -> bool;
}
extern "C" {
    pub fn pci_setup_msix_device_domain(pdev: *mut pci_dev, hwsize: c_uint) -> bool;
}
// Legacy (!IRQDOMAIN) fallbacks

extern "C" {
    pub fn pci_msi_legacy_setup_msi_irqs(dev: *mut pci_dev, nvec: c_int, type: c_int) -> c_int;
}
extern "C" {
    pub fn pci_msi_legacy_teardown_msi_irqs(dev: *mut pci_dev);
}

