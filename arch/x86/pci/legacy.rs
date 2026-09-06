//! Automatically rewritten from C to Rust
//! Source: arch/x86/pci/legacy.c
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
//
// legacy.c - traditional, old school PCI bus probing
//

//
// Discover remaining PCI buses in case there are peer host bridges.
// We use the number of last PCI bus provided by the PCI BIOS.
//
#[no_mangle]
unsafe extern "C" fn pcibios_fixup_peer_bridges() {
    static void pcibios_fixup_peer_bridges(void)
    {
    int n;
    if (pcibios_last_bus <= 0 || pcibios_last_bus > 0xff)
    return;
    DBG("PCI: Peer bridge fixup\n");
    for (n=0; n <= pcibios_last_bus; n++)
    pcibios_scan_specific_bus(n);
    }
#[no_mangle]
pub unsafe extern "C" fn pci_legacy_init() -> int __init {
    int __init pci_legacy_init(void)
    {
    if (!raw_pci_ops)
    return 1;
    pr_info("PCI: Probing PCI hardware\n");
    pcibios_scan_root(0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pcibios_scan_specific_bus(busn: c_int) {
    void pcibios_scan_specific_bus(int busn)
    {
    let mut stride: c_int = jailhouse_paravirt() ? 1 : 8;
    int devfn;
    u32 l;
    if (pci_find_bus(0, busn))
    return;
    for (devfn = 0; devfn < 256; devfn += stride) {
    if (!raw_pci_read(0, busn, devfn, PCI_VENDOR_ID, 2, &l) &&
    l != 0x0000 && l != 0xffff) {
    DBG("Found device at %02x:%02x [%04x]\n", busn, devfn, l);
    pr_info("PCI: Discovered peer bus %02x\n", busn);
    pcibios_scan_root(busn);
    return;
    }
    }
    }
    EXPORT_SYMBOL_GPL(pcibios_scan_specific_bus);
#[no_mangle]
unsafe extern "C" fn pci_subsys_init() -> int __init {
    static int __init pci_subsys_init(void)
    {
//
// The init function returns an non zero value when
// pci_legacy_init should be invoked.
//
    if (x86_init.pci.init()) {
    if (pci_legacy_init()) {
    pr_info("PCI: System does not support PCI\n");
    return -ENODEV;
    }
    }
    pcibios_fixup_peer_bridges();
    x86_init.pci.init_irq();
    pcibios_init();
    return 0;
    }
    subsys_initcall(pci_subsys_init);
