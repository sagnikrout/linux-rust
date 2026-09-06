//! Automatically rewritten from C to Rust
//! Source: arch/x86/pci/broadcom_bus.c
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
// Read address ranges from a Broadcom CNB20LE Host Bridge
//
// Copyright (c) 2010 Ira W. Snyder <iws@ovro.caltech.edu>
//

#[no_mangle]
unsafe extern "C" fn cnb20le_res(bus: u8, slot: u8, func: u8) -> void __init {
    static void __init cnb20le_res(u8 bus, u8 slot, u8 func)
    {
    struct pci_root_info *info;
    struct pci_root_res *root_res;
    struct resource res;
    u16 word1, word2;
    u8 fbus, lbus;
// read the PCI bus numbers
    fbus = read_pci_config_byte(bus, slot, func, 0x44);
    lbus = read_pci_config_byte(bus, slot, func, 0x45);
    info = alloc_pci_root_info(fbus, lbus, 0, 0);
//
// Add the legacy IDE ports on bus 0
//
// These do not exist anywhere in the bridge registers, AFAICT. I do
// not have the datasheet, so this is the best I can do.
//
    if (fbus == 0) {
    update_res(info, 0x01f0, 0x01f7, IORESOURCE_IO, 0);
    update_res(info, 0x03f6, 0x03f6, IORESOURCE_IO, 0);
    update_res(info, 0x0170, 0x0177, IORESOURCE_IO, 0);
    update_res(info, 0x0376, 0x0376, IORESOURCE_IO, 0);
    update_res(info, 0xffa0, 0xffaf, IORESOURCE_IO, 0);
    }
// read the non-prefetchable memory window
    word1 = read_pci_config_16(bus, slot, func, 0xc0);
    word2 = read_pci_config_16(bus, slot, func, 0xc2);
    if (word1 != word2) {
    res.start = ((resource_size_t) word1 << 16) | 0x0000;
    res.end   = ((resource_size_t) word2 << 16) | 0xffff;
    res.flags = IORESOURCE_MEM;
    update_res(info, res.start, res.end, res.flags, 0);
    }
// read the prefetchable memory window
    word1 = read_pci_config_16(bus, slot, func, 0xc4);
    word2 = read_pci_config_16(bus, slot, func, 0xc6);
    if (word1 != word2) {
    res.start = ((resource_size_t) word1 << 16) | 0x0000;
    res.end   = ((resource_size_t) word2 << 16) | 0xffff;
    res.flags = IORESOURCE_MEM | IORESOURCE_PREFETCH;
    update_res(info, res.start, res.end, res.flags, 0);
    }
// read the IO port window
    word1 = read_pci_config_16(bus, slot, func, 0xd0);
    word2 = read_pci_config_16(bus, slot, func, 0xd2);
    if (word1 != word2) {
    res.start = word1;
    res.end   = word2;
    res.flags = IORESOURCE_IO;
    update_res(info, res.start, res.end, res.flags, 0);
    }
// print information about this host bridge
    res.start = fbus;
    res.end   = lbus;
    res.flags = IORESOURCE_BUS;
    printk(KERN_INFO "CNB20LE PCI Host Bridge (domain 0000 %pR)\n", &res);
    list_for_each_entry(root_res, &info.resources, list)
    printk(KERN_INFO "host bridge window %pR\n", &root_res.res);
    }
#[no_mangle]
unsafe extern "C" fn broadcom_postcore_init() -> int __init {
    static int __init broadcom_postcore_init(void)
    {
    let mut bus: u8 = 0, slot = 0;
    u32 id;
    u16 vendor, device;

//
// We should get host bridge information from ACPI unless the BIOS
// doesn't support it.
//
    if (!acpi_disabled && acpi_os_get_root_pointer())
    return 0;

    id = read_pci_config(bus, slot, 0, PCI_VENDOR_ID);
    vendor = id & 0xffff;
    device = (id >> 16) & 0xffff;
    if (vendor == PCI_VENDOR_ID_SERVERWORKS &&
    device == PCI_DEVICE_ID_SERVERWORKS_LE) {
    cnb20le_res(bus, slot, 0);
    cnb20le_res(bus, slot, 1);
    }
    return 0;
    }
    postcore_initcall(broadcom_postcore_init);
