//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/mthca/mthca_reset.c
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
// Copyright (c) 2004 Topspin Communications.  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#[no_mangle]
pub unsafe extern "C" fn mthca_reset(mdev: *mut mthca_dev) -> c_int {
    int mthca_reset(struct mthca_dev *mdev)
    {
    int i;
    let mut err: c_int = 0;
    u32 *hca_header    = core::ptr::null_mut();
    u32 *bridge_header = core::ptr::null_mut();
    struct pci_dev *bridge = core::ptr::null_mut();
    let mut bridge_pcix_cap: c_int = 0;
    let mut hca_pcie_cap: c_int = 0;
    let mut hca_pcix_cap: c_int = 0;
    u16 devctl;
    u16 linkctl;
pub const MTHCA_RESET_OFFSET: c_uint = 0xf0010;

//
// Reset the chip.  This is somewhat ugly because we have to
// save off the PCI header before reset and then restore it
// after the chip reboots.  We skip config space offsets 22
// and 23 since those have a special meaning.
//
// To make matters worse, for Tavor (PCI-X HCA) we have to
// find the associated bridge device and save off its PCI
// header as well.
//
    if (!(mdev.mthca_flags & MTHCA_FLAG_PCIE)) {
// Look for the bridge -- its device ID will be 2 more
    than HCA's device ID. */
    while ((bridge = pci_get_device(mdev.pdev.vendor,
    mdev.pdev.device + 2,
    bridge)) != core::ptr::null_mut()) {
    if (bridge.hdr_type    == PCI_HEADER_TYPE_BRIDGE &&
    bridge.subordinate == mdev.pdev.bus) {
    mthca_dbg(mdev, "Found bridge: %s\n",
    pci_name(bridge));
    break;
    }
    }
    if (!bridge) {
//
// Didn't find a bridge for a Tavor device --
// assume we're in no-bridge mode and hope for
// the best.
//
    mthca_warn(mdev, "No bridge found for %s\n",
    pci_name(mdev.pdev));
    }
    }
// For Arbel do we need to save off the full 4K PCI Express header??
    hca_header = kmalloc(256, GFP_KERNEL);
    if (!hca_header) {
    err = -ENOMEM;
    goto put_dev;
    }
    for (i = 0; i < 64; ++i) {
    if (i == 22 || i == 23)
    continue;
    if (pci_read_config_dword(mdev.pdev, i * 4, hca_header + i)) {
    err = -ENODEV;
    mthca_err(mdev, "Couldn't save HCA "
    "PCI header, aborting.\n");
    goto free_hca;
    }
    }
    hca_pcix_cap = pci_find_capability(mdev.pdev, PCI_CAP_ID_PCIX);
    hca_pcie_cap = pci_pcie_cap(mdev.pdev);
    if (bridge) {
    bridge_header = kmalloc(256, GFP_KERNEL);
    if (!bridge_header) {
    err = -ENOMEM;
    goto free_hca;
    }
    for (i = 0; i < 64; ++i) {
    if (i == 22 || i == 23)
    continue;
    if (pci_read_config_dword(bridge, i * 4, bridge_header + i)) {
    err = -ENODEV;
    mthca_err(mdev, "Couldn't save HCA bridge "
    "PCI header, aborting.\n");
    goto free_bh;
    }
    }
    bridge_pcix_cap = pci_find_capability(bridge, PCI_CAP_ID_PCIX);
    if (!bridge_pcix_cap) {
    err = -ENODEV;
    mthca_err(mdev, "Couldn't locate HCA bridge "
    "PCI-X capability, aborting.\n");
    goto free_bh;
    }
    }
// actually hit reset
    {
    void __iomem *reset = ioremap(pci_resource_start(mdev.pdev, 0) +
    MTHCA_RESET_OFFSET, 4);
    if (!reset) {
    err = -ENOMEM;
    mthca_err(mdev, "Couldn't map HCA reset register, "
    "aborting.\n");
    goto free_bh;
    }
    writel(MTHCA_RESET_VALUE, reset);
    iounmap(reset);
    }
// Docs say to wait one second before accessing device
    msleep(1000);
// Now wait for PCI device to start responding again
    {
    u32 v;
    let mut c: c_int = 0;
    for (c = 0; c < 100; ++c) {
    if (pci_read_config_dword(bridge ? bridge : mdev.pdev, 0, &v)) {
    err = -ENODEV;
    mthca_err(mdev, "Couldn't access HCA after reset, "
    "aborting.\n");
    goto free_bh;
    }
    if (v != 0xffffffff)
    goto good;
    msleep(100);
    }
    err = -ENODEV;
    mthca_err(mdev, "PCI device did not come back after reset, "
    "aborting.\n");
    goto free_bh;
    }
    good:
// Now restore the PCI headers
    if (bridge) {
    if (pci_write_config_dword(bridge, bridge_pcix_cap + 0x8,
    bridge_header[(bridge_pcix_cap + 0x8) / 4])) {
    err = -ENODEV;
    mthca_err(mdev, "Couldn't restore HCA bridge Upstream "
    "split transaction control, aborting.\n");
    goto free_bh;
    }
    if (pci_write_config_dword(bridge, bridge_pcix_cap + 0xc,
    bridge_header[(bridge_pcix_cap + 0xc) / 4])) {
    err = -ENODEV;
    mthca_err(mdev, "Couldn't restore HCA bridge Downstream "
    "split transaction control, aborting.\n");
    goto free_bh;
    }
//
// Bridge control register is at 0x3e, so we'll
// naturally restore it last in this loop.
//
    for (i = 0; i < 16; ++i) {
    if (i * 4 == PCI_COMMAND)
    continue;
    if (pci_write_config_dword(bridge, i * 4, bridge_header[i])) {
    err = -ENODEV;
    mthca_err(mdev, "Couldn't restore HCA bridge reg %x, "
    "aborting.\n", i);
    goto free_bh;
    }
    }
    if (pci_write_config_dword(bridge, PCI_COMMAND,
    bridge_header[PCI_COMMAND / 4])) {
    err = -ENODEV;
    mthca_err(mdev, "Couldn't restore HCA bridge COMMAND, "
    "aborting.\n");
    goto free_bh;
    }
    }
    if (hca_pcix_cap) {
    if (pci_write_config_dword(mdev.pdev, hca_pcix_cap,
    hca_header[hca_pcix_cap / 4])) {
    err = -ENODEV;
    mthca_err(mdev, "Couldn't restore HCA PCI-X "
    "command register, aborting.\n");
    goto free_bh;
    }
    }
    if (hca_pcie_cap) {
    devctl = hca_header[(hca_pcie_cap + PCI_EXP_DEVCTL) / 4];
    if (pcie_capability_write_word(mdev.pdev, PCI_EXP_DEVCTL,
    devctl)) {
    err = -ENODEV;
    mthca_err(mdev, "Couldn't restore HCA PCI Express "
    "Device Control register, aborting.\n");
    goto free_bh;
    }
    linkctl = hca_header[(hca_pcie_cap + PCI_EXP_LNKCTL) / 4];
    if (pcie_capability_write_word(mdev.pdev, PCI_EXP_LNKCTL,
    linkctl)) {
    err = -ENODEV;
    mthca_err(mdev, "Couldn't restore HCA PCI Express "
    "Link control register, aborting.\n");
    goto free_bh;
    }
    }
    for (i = 0; i < 16; ++i) {
    if (i * 4 == PCI_COMMAND)
    continue;
    if (pci_write_config_dword(mdev.pdev, i * 4, hca_header[i])) {
    err = -ENODEV;
    mthca_err(mdev, "Couldn't restore HCA reg %x, "
    "aborting.\n", i);
    goto free_bh;
    }
    }
    if (pci_write_config_dword(mdev.pdev, PCI_COMMAND,
    hca_header[PCI_COMMAND / 4])) {
    err = -ENODEV;
    mthca_err(mdev, "Couldn't restore HCA COMMAND, "
    "aborting.\n");
    }
    free_bh:
    kfree(bridge_header);
    free_hca:
    kfree(hca_header);
    put_dev:
    pci_dev_put(bridge);
    return err;
    }
