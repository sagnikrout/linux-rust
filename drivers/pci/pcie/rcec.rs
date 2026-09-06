//! Automatically rewritten from C to Rust
//! Source: drivers/pci/pcie/rcec.c
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
// Root Complex Event Collector Support
//
// Authors:
// Sean V Kelley <sean.v.kelley@intel.com>
// Qiuxu Zhuo <qiuxu.zhuo@intel.com>
//
// Copyright (C) 2020 Intel Corp.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct walk_rcec_data {
    pub rcec: *mut pci_dev,
    pub data): *mut *mut *mut int (user_callback)(struct pci_dev dev, void,
    pub user_data: *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn rcec_assoc_rciep(rcec: *mut pci_dev, rciep: *mut pci_dev) -> bool {
    static bool rcec_assoc_rciep(struct pci_dev *rcec, struct pci_dev *rciep)
    {
    let mut bitmap: c_ulong = rcec.rcec_ea.bitmap;
    unsigned int devn;
// An RCiEP found on a different bus in range
    if (rcec.bus.number != rciep.bus.number)
    return true;
// Same bus, so check bitmap
    for_each_set_bit(devn, &bitmap, 32)
    if (devn == PCI_SLOT(rciep.devfn))
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn link_rcec_helper(dev: *mut pci_dev, data: *mut c_void) -> c_int {
    static int link_rcec_helper(struct pci_dev *dev, void *data)
    {
    struct walk_rcec_data *rcec_data = data;
    struct pci_dev *rcec = rcec_data.rcec;
    if ((pci_pcie_type(dev) == PCI_EXP_TYPE_RC_END) &&
    rcec_assoc_rciep(rcec, dev)) {
    dev.rcec = rcec;
    pci_dbg(dev, "PME & error events signaled via %s\n",
    pci_name(rcec));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn walk_rcec_helper(dev: *mut pci_dev, data: *mut c_void) -> c_int {
    static int walk_rcec_helper(struct pci_dev *dev, void *data)
    {
    struct walk_rcec_data *rcec_data = data;
    struct pci_dev *rcec = rcec_data.rcec;
    if ((pci_pcie_type(dev) == PCI_EXP_TYPE_RC_END) &&
    rcec_assoc_rciep(rcec, dev))
    rcec_data.user_callback(dev, rcec_data.user_data);
    return 0;
    }
    static void walk_rcec(int (*cb)(struct pci_dev *dev, void *data),
    void *userdata)
    {
    struct walk_rcec_data *rcec_data = userdata;
    struct pci_dev *rcec = rcec_data.rcec;
    u8 nextbusn, lastbusn;
    struct pci_bus *bus;
    unsigned int bnr;
    if (!rcec.rcec_ea)
    return;
// Walk own bus for bitmap based association
    pci_walk_bus(rcec.bus, cb, rcec_data);
    nextbusn = rcec.rcec_ea.nextbusn;
    lastbusn = rcec.rcec_ea.lastbusn;
// All RCiEP devices are on the same bus as the RCEC
    if (nextbusn == 0xff && lastbusn == 0x00)
    return;
    for (bnr = nextbusn; bnr <= lastbusn; bnr++) {
// No association indicated (PCIe 5.0-1, 7.9.10.3)
    if (bnr == rcec.bus.number)
    continue;
    bus = pci_find_bus(pci_domain_nr(rcec.bus), bnr);
    if (!bus)
    continue;
// Find RCiEP devices on the given bus ranges
    pci_walk_bus(bus, cb, rcec_data);
    }
    }
//
// pcie_link_rcec - Link RCiEP devices associated with RCEC.
// @rcec: RCEC whose RCiEP devices should be linked.
//
// Link the given RCEC to each RCiEP device found.
//
#[no_mangle]
pub unsafe extern "C" fn pcie_link_rcec(rcec: *mut pci_dev) {
    void pcie_link_rcec(struct pci_dev *rcec)
    {
    struct walk_rcec_data rcec_data;
    if (!rcec.rcec_ea)
    return;
    rcec_data.rcec = rcec;
    rcec_data.user_callback = core::ptr::null_mut();
    rcec_data.user_data = core::ptr::null_mut();
    walk_rcec(link_rcec_helper, &rcec_data);
    }
//
// pcie_walk_rcec - Walk RCiEP devices associating with RCEC and call callback.
// @rcec:	RCEC whose RCiEP devices should be walked
// @cb:		Callback to be called for each RCiEP device found
// @userdata:	Arbitrary pointer to be passed to callback
//
// Walk the given RCEC. Call the callback on each RCiEP found.
//
// If @cb returns anything other than 0, break out.
//
    void pcie_walk_rcec(struct pci_dev *rcec, int (*cb)(struct pci_dev *, void *),
    void *userdata)
    {
    struct walk_rcec_data rcec_data;
    if (!rcec.rcec_ea)
    return;
    rcec_data.rcec = rcec;
    rcec_data.user_callback = cb;
    rcec_data.user_data = userdata;
    walk_rcec(walk_rcec_helper, &rcec_data);
    }
#[no_mangle]
pub unsafe extern "C" fn pci_rcec_init(dev: *mut pci_dev) {
    void pci_rcec_init(struct pci_dev *dev)
    {
    struct rcec_ea *rcec_ea;
    u32 rcec, hdr, busn;
    u8 ver;
// Only for Root Complex Event Collectors
    if (pci_pcie_type(dev) != PCI_EXP_TYPE_RC_EC)
    return;
    rcec = pci_find_ext_capability(dev, PCI_EXT_CAP_ID_RCEC);
    if (!rcec)
    return;
    rcec_ea = kzalloc_obj(*rcec_ea);
    if (!rcec_ea)
    return;
    pci_read_config_dword(dev, rcec + PCI_RCEC_RCIEP_BITMAP,
    &rcec_ea.bitmap);
// Check whether RCEC BUSN register is present
    pci_read_config_dword(dev, rcec, &hdr);
    ver = PCI_EXT_CAP_VER(hdr);
    if (ver >= PCI_RCEC_BUSN_REG_VER) {
    pci_read_config_dword(dev, rcec + PCI_RCEC_BUSN, &busn);
    rcec_ea.nextbusn = PCI_RCEC_BUSN_NEXT(busn);
    rcec_ea.lastbusn = PCI_RCEC_BUSN_LAST(busn);
    } else {
// Avoid later ver check by setting nextbusn
    rcec_ea.nextbusn = 0xff;
    rcec_ea.lastbusn = 0x00;
    }
    dev.rcec_ea = rcec_ea;
    }
#[no_mangle]
pub unsafe extern "C" fn pci_rcec_exit(dev: *mut pci_dev) {
    void pci_rcec_exit(struct pci_dev *dev)
    {
    kfree(dev.rcec_ea);
    dev.rcec_ea = core::ptr::null_mut();
    }
