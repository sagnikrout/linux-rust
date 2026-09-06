//! Automatically rewritten from C to Rust
//! Source: arch/x86/pci/init.c
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

// arch_initcall has too random ordering, so call the initializers
    in the right sequence from here. */
#[no_mangle]
unsafe extern "C" fn pci_arch_init() -> __init int {
    static __init int pci_arch_init(void)
    {
    int type, pcbios = 1;
    type = pci_direct_probe();
    if (!(pci_probe & PCI_PROBE_NOEARLY))
    pci_mmcfg_early_init();
    if (x86_init.pci.arch_init)
    pcbios = x86_init.pci.arch_init();
//
// Must happen after x86_init.pci.arch_init(). Xen sets up the
// x86_init.irqs.create_pci_msi_domain there.
//
    x86_create_pci_msi_domain();
    if (!pcbios)
    return 0;
    pci_pcbios_init();
//
// don't check for raw_pci_ops here because we want pcbios as last
// fallback, yet it's needed to run first to set pcibios_last_bus
// in case legacy PCI probing is used. otherwise detecting peer busses
// fails.
//
    pci_direct_init(type);
    if (!raw_pci_ops && !raw_pci_ext_ops)
    printk(KERN_ERR
    "PCI: Fatal: No config space access function found\n");
    dmi_check_pciprobe();
    dmi_check_skip_isa_align();
    return 0;
    }
    arch_initcall(pci_arch_init);
