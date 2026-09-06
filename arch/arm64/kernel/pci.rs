//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/pci.c
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
// Code borrowed from powerpc/kernel/pci-common.c
//
// Copyright (C) 2003 Anton Blanchard <anton@au.ibm.com>, IBM
// Copyright (C) 2014 ARM Ltd.
//

//
// raw_pci_read/write - Platform-specific PCI config space access.
//
    int raw_pci_read(unsigned int domain, unsigned int bus,
    unsigned int devfn, int reg, int len, u32 *val)
    {
    struct pci_bus *b = pci_find_bus(domain, bus);
    if (!b)
    return PCIBIOS_DEVICE_NOT_FOUND;
    return b.ops.read(b, devfn, reg, len, val);
    }
    int raw_pci_write(unsigned int domain, unsigned int bus,
    unsigned int devfn, int reg, int len, u32 val)
    {
    struct pci_bus *b = pci_find_bus(domain, bus);
    if (!b)
    return PCIBIOS_DEVICE_NOT_FOUND;
    return b.ops.write(b, devfn, reg, len, val);
    }

#[no_mangle]
pub unsafe extern "C" fn pcibus_to_node(bus: *mut pci_bus) -> c_int {
    int pcibus_to_node(struct pci_bus *bus)
    {
    return dev_to_node(&bus.dev);
    }
    EXPORT_SYMBOL(pcibus_to_node);
