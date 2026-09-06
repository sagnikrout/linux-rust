//! Automatically rewritten from C to Rust
//! Source: drivers/bcma/driver_pci.c
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
// Broadcom specific AMBA
// PCI Core
//
// Copyright 2005, 2011, Broadcom Corporation
// Copyright 2006, 2007, Michael Buesch <m@bues.ch>
// Copyright 2011, 2012, Hauke Mehrtens <hauke@hauke-m.de>
//
// Licensed under the GNU/GPL. See COPYING for details.
//

//
// R/W ops.
//
#[no_mangle]
pub unsafe extern "C" fn bcma_pcie_read(pc: *mut bcma_drv_pci, address: u32) -> u32 {
    u32 bcma_pcie_read(struct bcma_drv_pci *pc, u32 address)
    {
    pcicore_write32(pc, BCMA_CORE_PCI_PCIEIND_ADDR, address);
    pcicore_read32(pc, BCMA_CORE_PCI_PCIEIND_ADDR);
    return pcicore_read32(pc, BCMA_CORE_PCI_PCIEIND_DATA);
    }
#[no_mangle]
unsafe extern "C" fn bcma_pcie_write(pc: *mut bcma_drv_pci, address: u32, data: u32) {
    static void bcma_pcie_write(struct bcma_drv_pci *pc, u32 address, u32 data)
    {
    pcicore_write32(pc, BCMA_CORE_PCI_PCIEIND_ADDR, address);
    pcicore_read32(pc, BCMA_CORE_PCI_PCIEIND_ADDR);
    pcicore_write32(pc, BCMA_CORE_PCI_PCIEIND_DATA, data);
    }
#[no_mangle]
unsafe extern "C" fn bcma_pcie_mdio_set_phy(pc: *mut bcma_drv_pci, phy: u16) {
    static void bcma_pcie_mdio_set_phy(struct bcma_drv_pci *pc, u16 phy)
    {
    u32 v;
    int i;
    v = BCMA_CORE_PCI_MDIODATA_START;
    v |= BCMA_CORE_PCI_MDIODATA_WRITE;
    v |= (BCMA_CORE_PCI_MDIODATA_DEV_ADDR <<
    BCMA_CORE_PCI_MDIODATA_DEVADDR_SHF);
    v |= (BCMA_CORE_PCI_MDIODATA_BLK_ADDR <<
    BCMA_CORE_PCI_MDIODATA_REGADDR_SHF);
    v |= BCMA_CORE_PCI_MDIODATA_TA;
    v |= (phy << 4);
    pcicore_write32(pc, BCMA_CORE_PCI_MDIO_DATA, v);
    udelay(10);
    for (i = 0; i < 200; i++) {
    v = pcicore_read32(pc, BCMA_CORE_PCI_MDIO_CONTROL);
    if (v & BCMA_CORE_PCI_MDIOCTL_ACCESS_DONE)
    break;
    usleep_range(1000, 2000);
    }
    }
#[no_mangle]
unsafe extern "C" fn bcma_pcie_mdio_read(pc: *mut bcma_drv_pci, device: u16, address: u8) -> u16 {
    static u16 bcma_pcie_mdio_read(struct bcma_drv_pci *pc, u16 device, u8 address)
    {
    let mut max_retries: c_int = 10;
    let mut ret: u16 = 0;
    u32 v;
    int i;
// enable mdio access to SERDES
    v = BCMA_CORE_PCI_MDIOCTL_PREAM_EN;
    v |= BCMA_CORE_PCI_MDIOCTL_DIVISOR_VAL;
    pcicore_write32(pc, BCMA_CORE_PCI_MDIO_CONTROL, v);
    if (pc.core.id.rev >= 10) {
    max_retries = 200;
    bcma_pcie_mdio_set_phy(pc, device);
    v = (BCMA_CORE_PCI_MDIODATA_DEV_ADDR <<
    BCMA_CORE_PCI_MDIODATA_DEVADDR_SHF);
    v |= (address << BCMA_CORE_PCI_MDIODATA_REGADDR_SHF);
    } else {
    v = (device << BCMA_CORE_PCI_MDIODATA_DEVADDR_SHF_OLD);
    v |= (address << BCMA_CORE_PCI_MDIODATA_REGADDR_SHF_OLD);
    }
    v |= BCMA_CORE_PCI_MDIODATA_START;
    v |= BCMA_CORE_PCI_MDIODATA_READ;
    v |= BCMA_CORE_PCI_MDIODATA_TA;
    pcicore_write32(pc, BCMA_CORE_PCI_MDIO_DATA, v);
// Wait for the device to complete the transaction
    udelay(10);
    for (i = 0; i < max_retries; i++) {
    v = pcicore_read32(pc, BCMA_CORE_PCI_MDIO_CONTROL);
    if (v & BCMA_CORE_PCI_MDIOCTL_ACCESS_DONE) {
    udelay(10);
    ret = pcicore_read32(pc, BCMA_CORE_PCI_MDIO_DATA);
    break;
    }
    usleep_range(1000, 2000);
    }
    pcicore_write32(pc, BCMA_CORE_PCI_MDIO_CONTROL, 0);
    return ret;
    }
    static void bcma_pcie_mdio_write(struct bcma_drv_pci *pc, u16 device,
    u8 address, u16 data)
    {
    let mut max_retries: c_int = 10;
    u32 v;
    int i;
// enable mdio access to SERDES
    v = BCMA_CORE_PCI_MDIOCTL_PREAM_EN;
    v |= BCMA_CORE_PCI_MDIOCTL_DIVISOR_VAL;
    pcicore_write32(pc, BCMA_CORE_PCI_MDIO_CONTROL, v);
    if (pc.core.id.rev >= 10) {
    max_retries = 200;
    bcma_pcie_mdio_set_phy(pc, device);
    v = (BCMA_CORE_PCI_MDIODATA_DEV_ADDR <<
    BCMA_CORE_PCI_MDIODATA_DEVADDR_SHF);
    v |= (address << BCMA_CORE_PCI_MDIODATA_REGADDR_SHF);
    } else {
    v = (device << BCMA_CORE_PCI_MDIODATA_DEVADDR_SHF_OLD);
    v |= (address << BCMA_CORE_PCI_MDIODATA_REGADDR_SHF_OLD);
    }
    v |= BCMA_CORE_PCI_MDIODATA_START;
    v |= BCMA_CORE_PCI_MDIODATA_WRITE;
    v |= BCMA_CORE_PCI_MDIODATA_TA;
    v |= data;
    pcicore_write32(pc, BCMA_CORE_PCI_MDIO_DATA, v);
// Wait for the device to complete the transaction
    udelay(10);
    for (i = 0; i < max_retries; i++) {
    v = pcicore_read32(pc, BCMA_CORE_PCI_MDIO_CONTROL);
    if (v & BCMA_CORE_PCI_MDIOCTL_ACCESS_DONE)
    break;
    usleep_range(1000, 2000);
    }
    pcicore_write32(pc, BCMA_CORE_PCI_MDIO_CONTROL, 0);
    }
    static u16 bcma_pcie_mdio_writeread(struct bcma_drv_pci *pc, u16 device,
    u8 address, u16 data)
    {
    bcma_pcie_mdio_write(pc, device, address, data);
    return bcma_pcie_mdio_read(pc, device, address);
    }
//
// Early init.
//
#[no_mangle]
unsafe extern "C" fn bcma_core_pci_fixcfg(pc: *mut bcma_drv_pci) {
    static void bcma_core_pci_fixcfg(struct bcma_drv_pci *pc)
    {
    struct bcma_device *core = pc.core;
    u16 val16, core_index;
    uint regoff;
    regoff = BCMA_CORE_PCI_SPROM(BCMA_CORE_PCI_SPROM_PI_OFFSET);
    core_index = (u16)core.core_index;
    val16 = pcicore_read16(pc, regoff);
    if (((val16 & BCMA_CORE_PCI_SPROM_PI_MASK) >> BCMA_CORE_PCI_SPROM_PI_SHIFT)
    != core_index) {
    val16 = (core_index << BCMA_CORE_PCI_SPROM_PI_SHIFT) |
    (val16 & ~BCMA_CORE_PCI_SPROM_PI_MASK);
    pcicore_write16(pc, regoff, val16);
    }
    }
//
// Apply some early fixes required before accessing SPROM.
// See also si_pci_fixcfg.
//
#[no_mangle]
pub unsafe extern "C" fn bcma_core_pci_early_init(pc: *mut bcma_drv_pci) {
    void bcma_core_pci_early_init(struct bcma_drv_pci *pc)
    {
    if (pc.early_setup_done)
    return;
    pc.hostmode = bcma_core_pci_is_in_hostmode(pc);
    if (pc.hostmode)
    goto out;
    bcma_core_pci_fixcfg(pc);
    out:
    pc.early_setup_done = true;
    }
//
// Workarounds.
//
#[no_mangle]
unsafe extern "C" fn bcma_pcicore_polarity_workaround(pc: *mut bcma_drv_pci) -> u8 {
    static u8 bcma_pcicore_polarity_workaround(struct bcma_drv_pci *pc)
    {
    u32 tmp;
    tmp = bcma_pcie_read(pc, BCMA_CORE_PCI_PLP_STATUSREG);
    if (tmp & BCMA_CORE_PCI_PLP_POLARITYINV_STAT)
    return BCMA_CORE_PCI_SERDES_RX_CTRL_FORCE |
    BCMA_CORE_PCI_SERDES_RX_CTRL_POLARITY;
    else
    return BCMA_CORE_PCI_SERDES_RX_CTRL_FORCE;
    }
#[no_mangle]
unsafe extern "C" fn bcma_pcicore_serdes_workaround(pc: *mut bcma_drv_pci) {
    static void bcma_pcicore_serdes_workaround(struct bcma_drv_pci *pc)
    {
    u16 tmp;
    bcma_pcie_mdio_write(pc, BCMA_CORE_PCI_MDIODATA_DEV_RX,
    BCMA_CORE_PCI_SERDES_RX_CTRL,
    bcma_pcicore_polarity_workaround(pc));
    tmp = bcma_pcie_mdio_read(pc, BCMA_CORE_PCI_MDIODATA_DEV_PLL,
    BCMA_CORE_PCI_SERDES_PLL_CTRL);
    if (tmp & BCMA_CORE_PCI_PLL_CTRL_FREQDET_EN)
    bcma_pcie_mdio_write(pc, BCMA_CORE_PCI_MDIODATA_DEV_PLL,
    BCMA_CORE_PCI_SERDES_PLL_CTRL,
    tmp & ~BCMA_CORE_PCI_PLL_CTRL_FREQDET_EN);
    }
// Fix MISC config to allow coming out of L2/L3-Ready state w/o PRST
// Needs to happen when coming out of 'standby'/'hibernate'
#[no_mangle]
unsafe extern "C" fn bcma_core_pci_config_fixup(pc: *mut bcma_drv_pci) {
    static void bcma_core_pci_config_fixup(struct bcma_drv_pci *pc)
    {
    u16 val16;
    uint regoff;
    regoff = BCMA_CORE_PCI_SPROM(BCMA_CORE_PCI_SPROM_MISC_CONFIG);
    val16 = pcicore_read16(pc, regoff);
    if (!(val16 & BCMA_CORE_PCI_SPROM_L23READY_EXIT_NOPERST)) {
    val16 |= BCMA_CORE_PCI_SPROM_L23READY_EXIT_NOPERST;
    pcicore_write16(pc, regoff, val16);
    }
    }
//
// Init.
//
#[no_mangle]
unsafe extern "C" fn bcma_core_pci_clientmode_init(pc: *mut bcma_drv_pci) {
    static void bcma_core_pci_clientmode_init(struct bcma_drv_pci *pc)
    {
    bcma_pcicore_serdes_workaround(pc);
    bcma_core_pci_config_fixup(pc);
    }
#[no_mangle]
pub unsafe extern "C" fn bcma_core_pci_init(pc: *mut bcma_drv_pci) {
    void bcma_core_pci_init(struct bcma_drv_pci *pc)
    {
    if (pc.setup_done)
    return;
    bcma_core_pci_early_init(pc);
    if (pc.hostmode)
    bcma_core_pci_hostmode_init(pc);
    else
    bcma_core_pci_clientmode_init(pc);
    }
#[no_mangle]
pub unsafe extern "C" fn bcma_core_pci_power_save(bus: *mut bcma_bus, up: bool) {
    void bcma_core_pci_power_save(struct bcma_bus *bus, bool up)
    {
    struct bcma_drv_pci *pc;
    u16 data;
    if (bus.hosttype != BCMA_HOSTTYPE_PCI)
    return;
    pc = &bus.drv_pci[0];
    if (pc.core.id.rev >= 15 && pc.core.id.rev <= 20) {
    data = up ? 0x74 : 0x7C;
    bcma_pcie_mdio_writeread(pc, BCMA_CORE_PCI_MDIO_BLK1,
    BCMA_CORE_PCI_MDIO_BLK1_MGMT1, 0x7F64);
    bcma_pcie_mdio_writeread(pc, BCMA_CORE_PCI_MDIO_BLK1,
    BCMA_CORE_PCI_MDIO_BLK1_MGMT3, data);
    } else if (pc.core.id.rev >= 21 && pc.core.id.rev <= 22) {
    data = up ? 0x75 : 0x7D;
    bcma_pcie_mdio_writeread(pc, BCMA_CORE_PCI_MDIO_BLK1,
    BCMA_CORE_PCI_MDIO_BLK1_MGMT1, 0x7E65);
    bcma_pcie_mdio_writeread(pc, BCMA_CORE_PCI_MDIO_BLK1,
    BCMA_CORE_PCI_MDIO_BLK1_MGMT3, data);
    }
    }
    EXPORT_SYMBOL_GPL(bcma_core_pci_power_save);
#[no_mangle]
unsafe extern "C" fn bcma_core_pci_extend_L1timer(pc: *mut bcma_drv_pci, extend: bool) {
    static void bcma_core_pci_extend_L1timer(struct bcma_drv_pci *pc, bool extend)
    {
    u32 w;
    w = bcma_pcie_read(pc, BCMA_CORE_PCI_DLLP_PMTHRESHREG);
    if (extend)
    w |= BCMA_CORE_PCI_ASPMTIMER_EXTEND;
    else
    w &= ~BCMA_CORE_PCI_ASPMTIMER_EXTEND;
    bcma_pcie_write(pc, BCMA_CORE_PCI_DLLP_PMTHRESHREG, w);
    bcma_pcie_read(pc, BCMA_CORE_PCI_DLLP_PMTHRESHREG);
    }
#[no_mangle]
pub unsafe extern "C" fn bcma_core_pci_up(pc: *mut bcma_drv_pci) {
    void bcma_core_pci_up(struct bcma_drv_pci *pc)
    {
    bcma_core_pci_extend_L1timer(pc, true);
    }
#[no_mangle]
pub unsafe extern "C" fn bcma_core_pci_down(pc: *mut bcma_drv_pci) {
    void bcma_core_pci_down(struct bcma_drv_pci *pc)
    {
    bcma_core_pci_extend_L1timer(pc, false);
    }
