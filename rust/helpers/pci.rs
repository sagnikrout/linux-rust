//! Automatically rewritten from C to Rust
//! Source: rust/helpers/pci.c
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

#[no_mangle]
pub unsafe extern "C" fn rust_helper_pci_dev_id(dev: *mut pci_dev) -> __rust_helper u16 {
    __rust_helper u16 rust_helper_pci_dev_id(struct pci_dev *dev)
    {
    return PCI_DEVID(dev.bus.number, dev.devfn);
    }
    __rust_helper resource_size_t
    rust_helper_pci_resource_start(struct pci_dev *pdev, int bar)
    {
    return pci_resource_start(pdev, bar);
    }
    __rust_helper resource_size_t rust_helper_pci_resource_len(struct pci_dev *pdev,
    int bar)
    {
    return pci_resource_len(pdev, bar);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_dev_is_pci(dev: *const device) -> __rust_helper bool {
    __rust_helper bool rust_helper_dev_is_pci(const struct device *dev)
    {
    return dev_is_pci(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_pci_irq_type(pdev: *mut pci_dev) -> __rust_helper unsigned int {
    __rust_helper unsigned int rust_helper_pci_irq_type(struct pci_dev *pdev)
    {
    return pci_irq_type(pdev);
    }

    __rust_helper unsigned int
    rust_helper_pci_sriov_get_totalvfs(struct pci_dev *pdev)
    {
    return pci_sriov_get_totalvfs(pdev);
    }

    __rust_helper int rust_helper_pci_alloc_irq_vectors(struct pci_dev *dev,
    unsigned int min_vecs,
    unsigned int max_vecs,
    unsigned int flags)
    {
    return pci_alloc_irq_vectors(dev, min_vecs, max_vecs, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_pci_free_irq_vectors(dev: *mut pci_dev) -> __rust_helper void {
    __rust_helper void rust_helper_pci_free_irq_vectors(struct pci_dev *dev)
    {
    pci_free_irq_vectors(dev);
    }
    __rust_helper int rust_helper_pci_irq_vector(struct pci_dev *pdev,
    unsigned int nvec)
    {
    return pci_irq_vector(pdev, nvec);
    }
