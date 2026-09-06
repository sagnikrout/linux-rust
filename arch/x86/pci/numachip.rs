//! Automatically rewritten from C to Rust
//! Source: arch/x86/pci/numachip.c
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
//
// Numascale NumaConnect-specific PCI code
//
// Copyright (C) 2012 Numascale AS. All rights reserved.
//
// Send feedback to <support@numascale.com>
//
// PCI accessor functions derived from mmconfig_64.c
//

    static u8 limit __read_mostly;
    static inline char __iomem *pci_dev_base(unsigned int seg, unsigned int bus, unsigned int devfn)
    {
    struct pci_mmcfg_region *cfg = pci_mmconfig_lookup(seg, bus);
    if (cfg && cfg.virt)
    return cfg.virt + (PCI_MMCFG_BUS_OFFSET(bus) | (devfn << 12));
    return core::ptr::null_mut();
    }
    static int pci_mmcfg_read_numachip(unsigned int seg, unsigned int bus,
    unsigned int devfn, int reg, int len, u32 *value)
    {
    char __iomem *addr;
// Why do we have this when nobody checks it. How about a BUG()!? -AK
    if (unlikely((bus > 255) || (devfn > 255) || (reg > 4095))) {
    err:		*value = -1;
    return -EINVAL;
    }
// Ensure AMD Northbridges don't decode reads to other devices
    if (unlikely(bus == 0 && devfn >= limit)) {
// value = -1;
    return 0;
    }
    rcu_read_lock();
    addr = pci_dev_base(seg, bus, devfn);
    if (!addr) {
    rcu_read_unlock();
    goto err;
    }
    switch (len) {
    case 1:
// value = mmio_config_readb(addr + reg);
    break;
    case 2:
// value = mmio_config_readw(addr + reg);
    break;
    case 4:
// value = mmio_config_readl(addr + reg);
    break;
    }
    rcu_read_unlock();
    return 0;
    }
    static int pci_mmcfg_write_numachip(unsigned int seg, unsigned int bus,
    unsigned int devfn, int reg, int len, u32 value)
    {
    char __iomem *addr;
// Why do we have this when nobody checks it. How about a BUG()!? -AK
    if (unlikely((bus > 255) || (devfn > 255) || (reg > 4095)))
    return -EINVAL;
// Ensure AMD Northbridges don't decode writes to other devices
    if (unlikely(bus == 0 && devfn >= limit))
    return 0;
    rcu_read_lock();
    addr = pci_dev_base(seg, bus, devfn);
    if (!addr) {
    rcu_read_unlock();
    return -EINVAL;
    }
    switch (len) {
    case 1:
    mmio_config_writeb(addr + reg, value);
    break;
    case 2:
    mmio_config_writew(addr + reg, value);
    break;
    case 4:
    mmio_config_writel(addr + reg, value);
    break;
    }
    rcu_read_unlock();
    return 0;
    }
    static const struct pci_raw_ops pci_mmcfg_numachip = {
    .read = pci_mmcfg_read_numachip,
    .write = pci_mmcfg_write_numachip,
    };
#[no_mangle]
pub unsafe extern "C" fn pci_numachip_init() -> int __init {
    int __init pci_numachip_init(void)
    {
    let mut ret: c_int = 0;
    u32 val;
// For remote I/O, restrict bus 0 access to the actual number of AMD
    Northbridges, which starts at device number 0x18 */
    ret = raw_pci_read(0, 0, PCI_DEVFN(0x18, 0), 0x60, sizeof(val), &val);
    if (ret)
    goto out;
// HyperTransport fabric size in bits 6:4
    limit = PCI_DEVFN(0x18 + ((val >> 4) & 7) + 1, 0);
// Use NumaChip PCI accessors for non-extended and extended access
    raw_pci_ops = raw_pci_ext_ops = &pci_mmcfg_numachip;
    out:
    return ret;
    }
