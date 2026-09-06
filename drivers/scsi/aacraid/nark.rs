//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/aacraid/nark.c
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
// Adaptec AAC series RAID controller driver
//
// based on the old aacraid driver that is..
// Adaptec aacraid device driver for Linux.
//
// Copyright (c) 2000-2010 Adaptec, Inc.
// 2010-2015 PMC-Sierra, Inc. (aacraid@pmc-sierra.com)
// 2016-2017 Microsemi Corp. (aacraid@microsemi.com)
//
// Module Name:
// nark.c
//
// Abstract: Hardware Device Interface for NEMER/ARK
//

//
// aac_nark_ioremap
// @dev: device to ioremap
// @size: mapping resize request
//
#[no_mangle]
unsafe extern "C" fn aac_nark_ioremap(dev: *mut *mut aac_dev, size: u32) -> c_int {
    static int aac_nark_ioremap(struct aac_dev * dev, u32 size)
    {
    if (!size) {
    iounmap(dev.regs.rx);
    dev.regs.rx = core::ptr::null_mut();
    iounmap(dev.base);
    dev.base = core::ptr::null_mut();
    return 0;
    }
    dev.base_start = pci_resource_start(dev.pdev, 2);
    dev.regs.rx = ioremap((u64)pci_resource_start(dev.pdev, 0) |
    ((u64)pci_resource_start(dev.pdev, 1) << 32),
    sizeof(struct rx_registers) - sizeof(struct rx_inbound));
    dev.base = core::ptr::null_mut();
    if (dev.regs.rx == core::ptr::null_mut())
    return -1;
    dev.base = ioremap(dev.base_start, size);
    if (dev.base == core::ptr::null_mut()) {
    iounmap(dev.regs.rx);
    dev.regs.rx = core::ptr::null_mut();
    return -1;
    }
    dev.IndexRegs = &((struct rx_registers __iomem *)dev.base).IndexRegs;
    return 0;
    }
//
// aac_nark_init	-	initialize an NEMER/ARK Split Bar card
// @dev: device to configure
//
#[no_mangle]
pub unsafe extern "C" fn aac_nark_init(dev: *mut *mut aac_dev) -> c_int {
    int aac_nark_init(struct aac_dev * dev)
    {
//
// Fill in the function dispatch table.
//
    dev.a_ops.adapter_ioremap = aac_nark_ioremap;
    dev.a_ops.adapter_comm = aac_rx_select_comm;
    return _aac_rx_init(dev);
    }
