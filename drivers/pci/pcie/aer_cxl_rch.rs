//! Automatically rewritten from C to Rust
//! Source: drivers/pci/pcie/aer_cxl_rch.c
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
// Copyright(c) 2023 AMD Corporation. All rights reserved.

#[no_mangle]
unsafe extern "C" fn is_cxl_mem_dev(dev: *mut pci_dev) -> bool {
    static bool is_cxl_mem_dev(struct pci_dev *dev)
    {
//
// The capability, status, and control fields in Device 0,
// Function 0 DVSEC control the CXL functionality of the
// entire device (CXL 3.0, 8.1.3).
//
    if (dev.devfn != PCI_DEVFN(0, 0))
    return false;
//
// CXL Memory Devices must have the 502h class code set (CXL
// 3.0, 8.1.12.1).
//
    if ((dev.class >> 8) != PCI_CLASS_MEMORY_CXL)
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn cxl_error_is_native(dev: *mut pci_dev) -> bool {
    static bool cxl_error_is_native(struct pci_dev *dev)
    {
    struct pci_host_bridge *host = pci_find_host_bridge(dev.bus);
    return (pcie_ports_native || host.native_aer);
    }
#[no_mangle]
unsafe extern "C" fn cxl_rch_handle_error_iter(dev: *mut pci_dev, data: *mut c_void) -> c_int {
    static int cxl_rch_handle_error_iter(struct pci_dev *dev, void *data)
    {
    struct aer_err_info *info = (struct aer_err_info *)data;
    const struct pci_error_handlers *err_handler;
    if (!is_cxl_mem_dev(dev) || !cxl_error_is_native(dev))
    return 0;
    guard(device)(&dev.dev);
    err_handler = dev.driver ? dev.driver.err_handler : core::ptr::null_mut();
    if (!err_handler)
    return 0;
    if (info.severity == AER_CORRECTABLE) {
    if (err_handler.cor_error_detected)
    err_handler.cor_error_detected(dev);
    } else if (err_handler.error_detected) {
    if (info.severity == AER_NONFATAL)
    err_handler.error_detected(dev, pci_channel_io_normal);
#[no_mangle]
pub unsafe extern "C" fn if(AER_FATAL: info->severity ==) -> else {
    else if (info.severity == AER_FATAL)
    err_handler.error_detected(dev, pci_channel_io_frozen);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cxl_rch_handle_error(dev: *mut pci_dev, info: *mut aer_err_info) {
    void cxl_rch_handle_error(struct pci_dev *dev, struct aer_err_info *info)
    {
//
// Internal errors of an RCEC indicate an AER error in an
// RCH's downstream port. Check and handle them in the CXL.mem
// device driver.
//
    if (pci_pcie_type(dev) == PCI_EXP_TYPE_RC_EC &&
    is_aer_internal_error(info))
    pcie_walk_rcec(dev, cxl_rch_handle_error_iter, info);
    }
#[no_mangle]
unsafe extern "C" fn handles_cxl_error_iter(dev: *mut pci_dev, data: *mut c_void) -> c_int {
    static int handles_cxl_error_iter(struct pci_dev *dev, void *data)
    {
    bool *handles_cxl = data;
    if (!*handles_cxl)
// handles_cxl = is_cxl_mem_dev(dev) && cxl_error_is_native(dev);
// Non-zero terminates iteration
    return *handles_cxl;
    }
#[no_mangle]
unsafe extern "C" fn handles_cxl_errors(rcec: *mut pci_dev) -> bool {
    static bool handles_cxl_errors(struct pci_dev *rcec)
    {
    let mut handles_cxl: bool = false;
    if (pci_pcie_type(rcec) == PCI_EXP_TYPE_RC_EC &&
    pcie_aer_is_native(rcec))
    pcie_walk_rcec(rcec, handles_cxl_error_iter, &handles_cxl);
    return handles_cxl;
    }
#[no_mangle]
pub unsafe extern "C" fn cxl_rch_enable_rcec(rcec: *mut pci_dev) {
    void cxl_rch_enable_rcec(struct pci_dev *rcec)
    {
    if (!handles_cxl_errors(rcec))
    return;
    pci_aer_unmask_internal_errors(rcec);
    pci_info(rcec, "CXL: Internal errors unmasked");
    }
