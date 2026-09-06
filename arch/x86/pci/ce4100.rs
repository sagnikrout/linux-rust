//! Automatically rewritten from C to Rust
//! Source: arch/x86/pci/ce4100.c
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
// Copyright(c) 2010 Intel Corporation. All rights reserved.
//
// Contact Information:
// Intel Corporation
// 2200 Mission College Blvd.
// Santa Clara, CA  97052
//
// This provides access methods for PCI registers that mis-behave on
// the CE4100. Each register can be assigned a private init, read and
// write routine. The exception to this is the bridge device.  The
// bridge device is the only device on bus zero (0) that requires any
// fixup so it is a special case ATM
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sim_reg {
    pub value: u32,
    pub mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sim_dev_reg {
    pub dev_func: c_int,
    pub reg: c_int,
    pub reg): *mut *mut void (init)(struct sim_dev_reg,
    pub value): *mut *mut *mut void (read)(struct sim_dev_reg reg, u32,
    pub value): *mut *mut *mut void (write)(struct sim_dev_reg reg, u32,
    pub sim_reg: sim_reg,
}

    { PCI_DEVFN(device, func), offset, init_op, read_op, write_op,\
    {0, SIZE_TO_MASK(size)} },
//
// All read/write functions are called with pci_config_lock held.
//
#[no_mangle]
unsafe extern "C" fn reg_init(reg: *mut sim_dev_reg) {
    static void reg_init(struct sim_dev_reg *reg)
    {
    pci_direct_conf1.read(0, 1, reg.dev_func, reg.reg, 4,
    &reg.sim_reg.value);
    }
#[no_mangle]
unsafe extern "C" fn reg_read(reg: *mut sim_dev_reg, value: *mut u32) {
    static void reg_read(struct sim_dev_reg *reg, u32 *value)
    {
// value = reg->sim_reg.value;
    }
#[no_mangle]
unsafe extern "C" fn reg_write(reg: *mut sim_dev_reg, value: u32) {
    static void reg_write(struct sim_dev_reg *reg, u32 value)
    {
    reg.sim_reg.value = (value & reg.sim_reg.mask) |
    (reg.sim_reg.value & ~reg.sim_reg.mask);
    }
#[no_mangle]
unsafe extern "C" fn sata_reg_init(reg: *mut sim_dev_reg) {
    static void sata_reg_init(struct sim_dev_reg *reg)
    {
    pci_direct_conf1.read(0, 1, PCI_DEVFN(14, 0), 0x10, 4,
    &reg.sim_reg.value);
    reg.sim_reg.value += 0x400;
    }
#[no_mangle]
unsafe extern "C" fn ehci_reg_read(reg: *mut sim_dev_reg, value: *mut u32) {
    static void ehci_reg_read(struct sim_dev_reg *reg, u32 *value)
    {
    reg_read(reg, value);
    if (*value != reg.sim_reg.mask)
// value |= 0x100;
    }
#[no_mangle]
unsafe extern "C" fn sata_revid_init(reg: *mut sim_dev_reg) {
    static void sata_revid_init(struct sim_dev_reg *reg)
    {
    reg.sim_reg.value = 0x01060100;
    reg.sim_reg.mask = 0;
    }
#[no_mangle]
unsafe extern "C" fn sata_revid_read(reg: *mut sim_dev_reg, value: *mut u32) {
    static void sata_revid_read(struct sim_dev_reg *reg, u32 *value)
    {
    reg_read(reg, value);
    }
#[no_mangle]
unsafe extern "C" fn reg_noirq_read(reg: *mut sim_dev_reg, value: *mut u32) {
    static void reg_noirq_read(struct sim_dev_reg *reg, u32 *value)
    {
// force interrupt pin value to 0
// value = reg->sim_reg.value & 0xfff00ff;
    }
    static struct sim_dev_reg bus1_fixups[] = {
    DEFINE_REG(2, 0, 0x10, (16*MB), reg_init, reg_read, reg_write)
    DEFINE_REG(2, 0, 0x14, (256), reg_init, reg_read, reg_write)
    DEFINE_REG(2, 1, 0x10, (64*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(3, 0, 0x10, (64*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(4, 0, 0x10, (128*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(4, 1, 0x10, (128*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(6, 0, 0x10, (512*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(6, 1, 0x10, (512*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(6, 2, 0x10, (64*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(8, 0, 0x10, (1*MB), reg_init, reg_read, reg_write)
    DEFINE_REG(8, 1, 0x10, (64*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(8, 2, 0x10, (64*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(9, 0, 0x10 , (1*MB), reg_init, reg_read, reg_write)
    DEFINE_REG(9, 0, 0x14, (64*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(10, 0, 0x10, (256), reg_init, reg_read, reg_write)
    DEFINE_REG(10, 0, 0x14, (256*MB), reg_init, reg_read, reg_write)
    DEFINE_REG(11, 0, 0x10, (256), reg_init, reg_read, reg_write)
    DEFINE_REG(11, 0, 0x14, (256), reg_init, reg_read, reg_write)
    DEFINE_REG(11, 1, 0x10, (256), reg_init, reg_read, reg_write)
    DEFINE_REG(11, 2, 0x10, (256), reg_init, reg_read, reg_write)
    DEFINE_REG(11, 2, 0x14, (256), reg_init, reg_read, reg_write)
    DEFINE_REG(11, 2, 0x18, (256), reg_init, reg_read, reg_write)
    DEFINE_REG(11, 3, 0x10, (256), reg_init, reg_read, reg_write)
    DEFINE_REG(11, 3, 0x14, (256), reg_init, reg_read, reg_write)
    DEFINE_REG(11, 4, 0x10, (256), reg_init, reg_read, reg_write)
    DEFINE_REG(11, 5, 0x10, (64*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(11, 6, 0x10, (256), reg_init, reg_read, reg_write)
    DEFINE_REG(11, 7, 0x10, (64*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(11, 7, 0x3c, 256, reg_init, reg_noirq_read, reg_write)
    DEFINE_REG(12, 0, 0x10, (128*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(12, 0, 0x14, (256), reg_init, reg_read, reg_write)
    DEFINE_REG(12, 1, 0x10, (1024), reg_init, reg_read, reg_write)
    DEFINE_REG(13, 0, 0x10, (32*KB), reg_init, ehci_reg_read, reg_write)
    DEFINE_REG(13, 1, 0x10, (32*KB), reg_init, ehci_reg_read, reg_write)
    DEFINE_REG(14, 0, 0x8,  0, sata_revid_init, sata_revid_read, 0)
    DEFINE_REG(14, 0, 0x10, 0, reg_init, reg_read, reg_write)
    DEFINE_REG(14, 0, 0x14, 0, reg_init, reg_read, reg_write)
    DEFINE_REG(14, 0, 0x18, 0, reg_init, reg_read, reg_write)
    DEFINE_REG(14, 0, 0x1C, 0, reg_init, reg_read, reg_write)
    DEFINE_REG(14, 0, 0x20, 0, reg_init, reg_read, reg_write)
    DEFINE_REG(14, 0, 0x24, (0x200), sata_reg_init, reg_read, reg_write)
    DEFINE_REG(15, 0, 0x10, (64*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(15, 0, 0x14, (64*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(16, 0, 0x10, (64*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(16, 0, 0x14, (64*MB), reg_init, reg_read, reg_write)
    DEFINE_REG(16, 0, 0x18, (64*MB), reg_init, reg_read, reg_write)
    DEFINE_REG(16, 0, 0x3c, 256, reg_init, reg_noirq_read, reg_write)
    DEFINE_REG(17, 0, 0x10, (128*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(18, 0, 0x10, (1*KB), reg_init, reg_read, reg_write)
    DEFINE_REG(18, 0, 0x3c, 256, reg_init, reg_noirq_read, reg_write)
    };
#[no_mangle]
unsafe extern "C" fn init_sim_regs() -> void __init {
    static void __init init_sim_regs(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(bus1_fixups); i++) {
    if (bus1_fixups[i].init)
    bus1_fixups[i].init(&bus1_fixups[i]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn extract_bytes(value: *mut u32, reg: c_int, len: c_int) {
    static inline void extract_bytes(u32 *value, int reg, int len)
    {
    uint32_t mask;
// value >>= ((reg & 3) * 8);
    mask = 0xFFFFFFFF >> ((4 - len) * 8);
// value &= mask;
    }
#[no_mangle]
unsafe extern "C" fn bridge_read(devfn: c_uint, reg: c_int, len: c_int, value: *mut u32) -> c_int {
    static int bridge_read(unsigned int devfn, int reg, int len, u32 *value)
    {
    u32 av_bridge_base, av_bridge_limit;
    let mut retval: c_int = 0;
    switch (reg) {
// Make BARs appear to not request any memory.
    case PCI_BASE_ADDRESS_0:
    case PCI_BASE_ADDRESS_0 + 1:
    case PCI_BASE_ADDRESS_0 + 2:
    case PCI_BASE_ADDRESS_0 + 3:
// value = 0;
    break;
// Since subordinate bus number register is hardwired
// to zero and read only, so do the simulation.
//
    case PCI_PRIMARY_BUS:
    if (len == 4)
// value = 0x00010100;
    break;
    case PCI_SUBORDINATE_BUS:
// value = 1;
    break;
    case PCI_MEMORY_BASE:
    case PCI_MEMORY_LIMIT:
// Get the A/V bridge base address.
    pci_direct_conf1.read(0, 0, devfn,
    PCI_BASE_ADDRESS_0, 4, &av_bridge_base);
    av_bridge_limit = av_bridge_base + (512*MB - 1);
    av_bridge_limit >>= 16;
    av_bridge_limit &= 0xFFF0;
    av_bridge_base >>= 16;
    av_bridge_base &= 0xFFF0;
    if (reg == PCI_MEMORY_LIMIT)
// value = av_bridge_limit;
#[no_mangle]
pub unsafe extern "C" fn if(2: len ==) -> else {
    else if (len == 2)
// value = av_bridge_base;
    else
// value = (av_bridge_limit << 16) | av_bridge_base;
    break;
// Make prefetchable memory limit smaller than prefetchable
// memory base, so not claim prefetchable memory space.
//
    case PCI_PREF_MEMORY_BASE:
// value = 0xFFF0;
    break;
    case PCI_PREF_MEMORY_LIMIT:
// value = 0x0;
    break;
// Make IO limit smaller than IO base, so not claim IO space.
    case PCI_IO_BASE:
// value = 0xF0;
    break;
    case PCI_IO_LIMIT:
// value = 0;
    break;
    default:
    retval = 1;
    }
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn ce4100_bus1_read(devfn: c_uint, reg: c_int, len: c_int, value: *mut u32) -> c_int {
    static int ce4100_bus1_read(unsigned int devfn, int reg, int len, u32 *value)
    {
    unsigned long flags;
    int i;
    for (i = 0; i < ARRAY_SIZE(bus1_fixups); i++) {
    if (bus1_fixups[i].dev_func == devfn &&
    bus1_fixups[i].reg == (reg & ~3) &&
    bus1_fixups[i].read) {
    raw_spin_lock_irqsave(&pci_config_lock, flags);
    bus1_fixups[i].read(&(bus1_fixups[i]), value);
    raw_spin_unlock_irqrestore(&pci_config_lock, flags);
    extract_bytes(value, reg, len);
    return 0;
    }
    }
    return -1;
    }
    static int ce4100_conf_read(unsigned int seg, unsigned int bus,
    unsigned int devfn, int reg, int len, u32 *value)
    {
    WARN_ON(seg);
    if (bus == 1 && !ce4100_bus1_read(devfn, reg, len, value))
    return 0;
    if (bus == 0 && (PCI_DEVFN(1, 0) == devfn) &&
    !bridge_read(devfn, reg, len, value))
    return 0;
    return pci_direct_conf1.read(seg, bus, devfn, reg, len, value);
    }
#[no_mangle]
unsafe extern "C" fn ce4100_bus1_write(devfn: c_uint, reg: c_int, len: c_int, value: u32) -> c_int {
    static int ce4100_bus1_write(unsigned int devfn, int reg, int len, u32 value)
    {
    unsigned long flags;
    int i;
    for (i = 0; i < ARRAY_SIZE(bus1_fixups); i++) {
    if (bus1_fixups[i].dev_func == devfn &&
    bus1_fixups[i].reg == (reg & ~3) &&
    bus1_fixups[i].write) {
    raw_spin_lock_irqsave(&pci_config_lock, flags);
    bus1_fixups[i].write(&(bus1_fixups[i]), value);
    raw_spin_unlock_irqrestore(&pci_config_lock, flags);
    return 0;
    }
    }
    return -1;
    }
    static int ce4100_conf_write(unsigned int seg, unsigned int bus,
    unsigned int devfn, int reg, int len, u32 value)
    {
    WARN_ON(seg);
    if (bus == 1 && !ce4100_bus1_write(devfn, reg, len, value))
    return 0;
// Discard writes to A/V bridge BAR.
    if (bus == 0 && PCI_DEVFN(1, 0) == devfn &&
    ((reg & ~3) == PCI_BASE_ADDRESS_0))
    return 0;
    return pci_direct_conf1.write(seg, bus, devfn, reg, len, value);
    }
    static const struct pci_raw_ops ce4100_pci_conf = {
    .read	= ce4100_conf_read,
    .write	= ce4100_conf_write,
    };
#[no_mangle]
pub unsafe extern "C" fn ce4100_pci_init() -> int __init {
    int __init ce4100_pci_init(void)
    {
    init_sim_regs();
    raw_pci_ops = &ce4100_pci_conf;
// Indicate caller that it should invoke pci_legacy_init()
    return 1;
    }
