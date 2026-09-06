//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mellanox/mlx4/reset.c
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
// Copyright (c) 2006, 2007 Cisco Systems, Inc.  All rights reserved.
// Copyright (c) 2007, 2008 Mellanox Technologies. All rights reserved.
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
pub unsafe extern "C" fn mlx4_reset(dev: *mut mlx4_dev) -> c_int {
    int mlx4_reset(struct mlx4_dev *dev)
    {
    void __iomem *reset;
    u32 *hca_header = core::ptr::null_mut();
    int pcie_cap;
    u16 devctl;
    u16 linkctl;
    u16 vendor;
    unsigned long end;
    u32 sem;
    int i;
    let mut err: c_int = 0;
pub const MLX4_RESET_BASE: c_uint = 0xf0000;
pub const MLX4_RESET_SIZE: c_uint = 0x400;
pub const MLX4_SEM_OFFSET: c_uint = 0x3fc;
pub const MLX4_RESET_OFFSET: c_uint = 0x10;

//
// Reset the chip.  This is somewhat ugly because we have to
// save off the PCI header before reset and then restore it
// after the chip reboots.  We skip config space offsets 22
// and 23 since those have a special meaning.
//
// Do we need to save off the full 4K PCI Express header??
    hca_header = kmalloc(256, GFP_KERNEL);
    if (!hca_header) {
    err = -ENOMEM;
    mlx4_err(dev, "Couldn't allocate memory to save HCA PCI header, aborting\n");
    goto out;
    }
    pcie_cap = pci_pcie_cap(dev.persist.pdev);
    for (i = 0; i < 64; ++i) {
    if (i == 22 || i == 23)
    continue;
    if (pci_read_config_dword(dev.persist.pdev, i * 4,
    hca_header + i)) {
    err = -ENODEV;
    mlx4_err(dev, "Couldn't save HCA PCI header, aborting\n");
    goto out;
    }
    }
    reset = ioremap(pci_resource_start(dev.persist.pdev, 0) +
    MLX4_RESET_BASE,
    MLX4_RESET_SIZE);
    if (!reset) {
    err = -ENOMEM;
    mlx4_err(dev, "Couldn't map HCA reset register, aborting\n");
    goto out;
    }
// grab HW semaphore to lock out flash updates
    end = jiffies + MLX4_SEM_TIMEOUT_JIFFIES;
    do {
    sem = readl(reset + MLX4_SEM_OFFSET);
    if (!sem)
    break;
    msleep(1);
    } while (time_before(jiffies, end));
    if (sem) {
    mlx4_err(dev, "Failed to obtain HW semaphore, aborting\n");
    err = -EAGAIN;
    iounmap(reset);
    goto out;
    }
// actually hit reset
    writel(MLX4_RESET_VALUE, reset + MLX4_RESET_OFFSET);
    iounmap(reset);
// Docs say to wait one second before accessing device
    msleep(1000);
    end = jiffies + MLX4_RESET_TIMEOUT_JIFFIES;
    do {
    if (!pci_read_config_word(dev.persist.pdev, PCI_VENDOR_ID,
    &vendor) && vendor != 0xffff)
    break;
    msleep(1);
    } while (time_before(jiffies, end));
    if (vendor == 0xffff) {
    err = -ENODEV;
    mlx4_err(dev, "PCI device did not come back after reset, aborting\n");
    goto out;
    }
// Now restore the PCI headers
    if (pcie_cap) {
    devctl = hca_header[(pcie_cap + PCI_EXP_DEVCTL) / 4];
    if (pcie_capability_write_word(dev.persist.pdev,
    PCI_EXP_DEVCTL,
    devctl)) {
    err = -ENODEV;
    mlx4_err(dev, "Couldn't restore HCA PCI Express Device Control register, aborting\n");
    goto out;
    }
    linkctl = hca_header[(pcie_cap + PCI_EXP_LNKCTL) / 4];
    if (pcie_capability_write_word(dev.persist.pdev,
    PCI_EXP_LNKCTL,
    linkctl)) {
    err = -ENODEV;
    mlx4_err(dev, "Couldn't restore HCA PCI Express Link control register, aborting\n");
    goto out;
    }
    }
    for (i = 0; i < 16; ++i) {
    if (i * 4 == PCI_COMMAND)
    continue;
    if (pci_write_config_dword(dev.persist.pdev, i * 4,
    hca_header[i])) {
    err = -ENODEV;
    mlx4_err(dev, "Couldn't restore HCA reg %x, aborting\n",
    i);
    goto out;
    }
    }
    if (pci_write_config_dword(dev.persist.pdev, PCI_COMMAND,
    hca_header[PCI_COMMAND / 4])) {
    err = -ENODEV;
    mlx4_err(dev, "Couldn't restore HCA COMMAND, aborting\n");
    goto out;
    }
    out:
    kfree(hca_header);
    return err;
    }
