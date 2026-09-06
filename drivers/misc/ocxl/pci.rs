//! Automatically rewritten from C to Rust
//! Source: drivers/misc/ocxl/pci.c
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright 2019 IBM Corp.

//
// Any opencapi device which wants to use this 'generic' driver should
// use the 0x062B device ID. Vendors should define the subsystem
// vendor/device ID to help differentiate devices.
//
    static const struct pci_device_id ocxl_pci_tbl[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_IBM, 0x062B) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, ocxl_pci_tbl);
#[no_mangle]
unsafe extern "C" fn ocxl_probe(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int ocxl_probe(struct pci_dev *dev, const struct pci_device_id *id)
    {
    int rc;
    struct ocxl_afu *afu, *tmp;
    struct ocxl_fn *fn;
    struct list_head *afu_list;
    fn = ocxl_function_open(dev);
    if (IS_ERR(fn))
    return PTR_ERR(fn);
    pci_set_drvdata(dev, fn);
    afu_list = ocxl_function_afu_list(fn);
    list_for_each_entry_safe(afu, tmp, afu_list, list) {
// Cleanup handled within ocxl_file_register_afu()
    rc = ocxl_file_register_afu(afu);
    if (rc) {
    dev_err(&dev.dev, "Failed to register AFU '%s' index %d",
    afu.config.name, afu.config.idx);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ocxl_remove(dev: *mut pci_dev) {
    static void ocxl_remove(struct pci_dev *dev)
    {
    struct ocxl_fn *fn;
    struct ocxl_afu *afu;
    struct list_head *afu_list;
    fn = pci_get_drvdata(dev);
    afu_list = ocxl_function_afu_list(fn);
    list_for_each_entry(afu, afu_list, list) {
    ocxl_file_unregister_afu(afu);
    }
    ocxl_function_close(fn);
    }
    struct pci_driver ocxl_pci_driver = {
    .name = "ocxl",
    .id_table = ocxl_pci_tbl,
    .probe = ocxl_probe,
    .remove = ocxl_remove,
    .shutdown = ocxl_remove,
    };
