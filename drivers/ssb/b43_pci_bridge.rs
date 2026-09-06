//! Automatically rewritten from C to Rust
//! Source: drivers/ssb/b43_pci_bridge.c
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


//
// Broadcom 43xx PCI-SSB bridge module
//
// This technically is a separate PCI driver module, but
// because of its small size we include it in the SSB core
// instead of creating a standalone module.
//
// Copyright 2007  Michael Buesch <m@bues.ch>
//
// Licensed under the GNU/GPL. See COPYING for details.
//

    static const struct pci_device_id b43_pci_bridge_tbl[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4301) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4306) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4307) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4311) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4312) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4315) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4318) },
    { PCI_DEVICE(PCI_VENDOR_ID_BCM_GVC,  0x4318) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4319) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4320) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4321) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4322) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 43222) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4324) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4325) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4328) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4329) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x432b) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x432c) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4350) },
    { PCI_DEVICE(PCI_VENDOR_ID_BROADCOM, 0x4351) },
    { 0, },
    };
    MODULE_DEVICE_TABLE(pci, b43_pci_bridge_tbl);
    static struct pci_driver b43_pci_bridge_driver = {
    .name = "b43-pci-bridge",
    .id_table = b43_pci_bridge_tbl,
    };
#[no_mangle]
pub unsafe extern "C" fn b43_pci_ssb_bridge_init() -> int __init {
    int __init b43_pci_ssb_bridge_init(void)
    {
    return ssb_pcihost_register(&b43_pci_bridge_driver);
    }
#[no_mangle]
pub unsafe extern "C" fn b43_pci_ssb_bridge_exit() -> void __exit {
    void __exit b43_pci_ssb_bridge_exit(void)
    {
    ssb_pcihost_unregister(&b43_pci_bridge_driver);
    }
