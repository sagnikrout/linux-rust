//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/fdomain_pci.c
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

    static int fdomain_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *d)
    {
    int err;
    struct Scsi_Host *sh;
    err = pci_enable_device(pdev);
    if (err)
    goto fail;
    err = pci_request_regions(pdev, "fdomain_pci");
    if (err)
    goto disable_device;
    err = -ENODEV;
    if (pci_resource_len(pdev, 0) == 0)
    goto release_region;
    sh = fdomain_create(pci_resource_start(pdev, 0), pdev.irq, 7,
    &pdev.dev);
    if (!sh)
    goto release_region;
    pci_set_drvdata(pdev, sh);
    return 0;
    release_region:
    pci_release_regions(pdev);
    disable_device:
    pci_disable_device(pdev);
    fail:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn fdomain_pci_remove(pdev: *mut pci_dev) {
    static void fdomain_pci_remove(struct pci_dev *pdev)
    {
    struct Scsi_Host *sh = pci_get_drvdata(pdev);
    fdomain_destroy(sh);
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    }
    static const struct pci_device_id fdomain_pci_table[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_FD, PCI_DEVICE_ID_FD_36C70) },
    {}
    };
    MODULE_DEVICE_TABLE(pci, fdomain_pci_table);
    static struct pci_driver fdomain_pci_driver = {
    .name		= "fdomain_pci",
    .id_table	= fdomain_pci_table,
    .probe		= fdomain_pci_probe,
    .remove		= fdomain_pci_remove,
    .driver.pm	= FDOMAIN_PM_OPS,
    };
    module_pci_driver(fdomain_pci_driver);
    MODULE_AUTHOR("Ondrej Zary, Rickard E. Faith");
    MODULE_DESCRIPTION("Future Domain TMC-3260 PCI SCSI driver");
    MODULE_LICENSE("GPL");
