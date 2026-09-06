//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/indirect_pci.c
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
// Support for indirect PCI bridges.
//
// Copyright (C) 1998 Gabriel Paubert.
//

    int __indirect_read_config(struct pci_controller *hose,
    unsigned char bus_number, unsigned int devfn,
    int offset, int len, u32 *val)
    {
    volatile void __iomem *cfg_data;
    let mut cfg_type: u8 = 0;
    u32 bus_no, reg;
    if (hose.indirect_type & PPC_INDIRECT_TYPE_NO_PCIE_LINK) {
    if (bus_number != hose.first_busno)
    return PCIBIOS_DEVICE_NOT_FOUND;
    if (devfn != 0)
    return PCIBIOS_DEVICE_NOT_FOUND;
    }
    if (ppc_md.pci_exclude_device)
    if (ppc_md.pci_exclude_device(hose, bus_number, devfn))
    return PCIBIOS_DEVICE_NOT_FOUND;
    if (hose.indirect_type & PPC_INDIRECT_TYPE_SET_CFG_TYPE)
    if (bus_number != hose.first_busno)
    cfg_type = 1;
    bus_no = (bus_number == hose.first_busno) ?
    hose.self_busno : bus_number;
    if (hose.indirect_type & PPC_INDIRECT_TYPE_EXT_REG)
    reg = ((offset & 0xf00) << 16) | (offset & 0xfc);
    else
    reg = offset & 0xfc;
    if (hose.indirect_type & PPC_INDIRECT_TYPE_BIG_ENDIAN)
    out_be32(hose.cfg_addr, (0x80000000 | (bus_no << 16) |
    (devfn << 8) | reg | cfg_type));
    else
    out_le32(hose.cfg_addr, (0x80000000 | (bus_no << 16) |
    (devfn << 8) | reg | cfg_type));
//
// Note: the caller has already checked that offset is
// suitably aligned and that len is 1, 2 or 4.
//
    cfg_data = hose.cfg_data + (offset & 3);
    switch (len) {
    case 1:
// val = in_8(cfg_data);
    break;
    case 2:
// val = in_le16(cfg_data);
    break;
    default:
// val = in_le32(cfg_data);
    break;
    }
    return PCIBIOS_SUCCESSFUL;
    }
    int indirect_read_config(struct pci_bus *bus, unsigned int devfn,
    int offset, int len, u32 *val)
    {
    struct pci_controller *hose = pci_bus_to_host(bus);
    return __indirect_read_config(hose, bus.number, devfn, offset, len,
    val);
    }
    int indirect_write_config(struct pci_bus *bus, unsigned int devfn,
    int offset, int len, u32 val)
    {
    struct pci_controller *hose = pci_bus_to_host(bus);
    volatile void __iomem *cfg_data;
    let mut cfg_type: u8 = 0;
    u32 bus_no, reg;
    if (hose.indirect_type & PPC_INDIRECT_TYPE_NO_PCIE_LINK) {
    if (bus.number != hose.first_busno)
    return PCIBIOS_DEVICE_NOT_FOUND;
    if (devfn != 0)
    return PCIBIOS_DEVICE_NOT_FOUND;
    }
    if (ppc_md.pci_exclude_device)
    if (ppc_md.pci_exclude_device(hose, bus.number, devfn))
    return PCIBIOS_DEVICE_NOT_FOUND;
    if (hose.indirect_type & PPC_INDIRECT_TYPE_SET_CFG_TYPE)
    if (bus.number != hose.first_busno)
    cfg_type = 1;
    bus_no = (bus.number == hose.first_busno) ?
    hose.self_busno : bus.number;
    if (hose.indirect_type & PPC_INDIRECT_TYPE_EXT_REG)
    reg = ((offset & 0xf00) << 16) | (offset & 0xfc);
    else
    reg = offset & 0xfc;
    if (hose.indirect_type & PPC_INDIRECT_TYPE_BIG_ENDIAN)
    out_be32(hose.cfg_addr, (0x80000000 | (bus_no << 16) |
    (devfn << 8) | reg | cfg_type));
    else
    out_le32(hose.cfg_addr, (0x80000000 | (bus_no << 16) |
    (devfn << 8) | reg | cfg_type));
// suppress setting of PCI_PRIMARY_BUS
    if (hose.indirect_type & PPC_INDIRECT_TYPE_SURPRESS_PRIMARY_BUS)
    if ((offset == PCI_PRIMARY_BUS) &&
    (bus.number == hose.first_busno))
    val &= 0xffffff00;
// Workaround for PCI_28 Errata in 440EPx/GRx
    if ((hose.indirect_type & PPC_INDIRECT_TYPE_BROKEN_MRM) &&
    offset == PCI_CACHE_LINE_SIZE) {
    val = 0;
    }
//
// Note: the caller has already checked that offset is
// suitably aligned and that len is 1, 2 or 4.
//
    cfg_data = hose.cfg_data + (offset & 3);
    switch (len) {
    case 1:
    out_8(cfg_data, val);
    break;
    case 2:
    out_le16(cfg_data, val);
    break;
    default:
    out_le32(cfg_data, val);
    break;
    }
    return PCIBIOS_SUCCESSFUL;
    }
    static struct pci_ops indirect_pci_ops =
    {
    .read = indirect_read_config,
    .write = indirect_write_config,
    };
    void setup_indirect_pci(struct pci_controller *hose, resource_size_t cfg_addr,
    resource_size_t cfg_data, u32 flags)
    {
    let mut base: resource_size_t = cfg_addr & PAGE_MASK;
    void __iomem *mbase;
    mbase = ioremap(base, PAGE_SIZE);
    hose.cfg_addr = mbase + (cfg_addr & ~PAGE_MASK);
    if ((cfg_data & PAGE_MASK) != base)
    mbase = ioremap(cfg_data & PAGE_MASK, PAGE_SIZE);
    hose.cfg_data = mbase + (cfg_data & ~PAGE_MASK);
    hose.ops = &indirect_pci_ops;
    hose.indirect_type = flags;
    }
