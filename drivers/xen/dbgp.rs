//! Automatically rewritten from C to Rust
//! Source: drivers/xen/dbgp.c
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

#[no_mangle]
unsafe extern "C" fn xen_dbgp_op(hcd: *mut usb_hcd, op: c_int) -> c_int {
    static int xen_dbgp_op(struct usb_hcd *hcd, int op)
    {

    const struct device *ctrlr = hcd_to_bus(hcd).controller;

    struct physdev_dbgp_op dbgp;
    if (!xen_initial_domain())
    return 0;
    dbgp.op = op;

    if (dev_is_pci(ctrlr)) {
    const struct pci_dev *pdev = to_pci_dev(ctrlr);
    dbgp.u.pci.seg = pci_domain_nr(pdev.bus);
    dbgp.u.pci.bus = pdev.bus.number;
    dbgp.u.pci.devfn = pdev.devfn;
    dbgp.bus = PHYSDEVOP_DBGP_BUS_PCI;
    } else

    dbgp.bus = PHYSDEVOP_DBGP_BUS_UNKNOWN;
    return HYPERVISOR_physdev_op(PHYSDEVOP_dbgp_op, &dbgp);
    }
#[no_mangle]
pub unsafe extern "C" fn xen_dbgp_reset_prep(hcd: *mut usb_hcd) -> c_int {
    int xen_dbgp_reset_prep(struct usb_hcd *hcd)
    {
    return xen_dbgp_op(hcd, PHYSDEVOP_DBGP_RESET_PREPARE);
    }
#[no_mangle]
pub unsafe extern "C" fn xen_dbgp_external_startup(hcd: *mut usb_hcd) -> c_int {
    int xen_dbgp_external_startup(struct usb_hcd *hcd)
    {
    return xen_dbgp_op(hcd, PHYSDEVOP_DBGP_RESET_DONE);
    }

    EXPORT_SYMBOL_GPL(xen_dbgp_reset_prep);
    EXPORT_SYMBOL_GPL(xen_dbgp_external_startup);
