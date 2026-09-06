//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/aacraid/rkt.c
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
// (c) Copyright 2001 Red Hat Inc.
//
// based on the old aacraid driver that is..
// Adaptec aacraid device driver for Linux.
//
// Copyright (c) 2000-2010 Adaptec, Inc.
// 2010-2015 PMC-Sierra, Inc. (aacraid@pmc-sierra.com)
// 2016-2017 Microsemi Corp. (aacraid@microsemi.com)
//
// Module Name:
// rkt.c
//
// Abstract: Hardware miniport for Drawbridge specific hardware functions.
//

//
// aac_rkt_select_comm	-	Select communications method
// @dev: Adapter
// @comm: communications method
//
#[no_mangle]
unsafe extern "C" fn aac_rkt_select_comm(dev: *mut aac_dev, comm: c_int) -> c_int {
    static int aac_rkt_select_comm(struct aac_dev *dev, int comm)
    {
    int retval;
    retval = aac_rx_select_comm(dev, comm);
    if (comm == AAC_COMM_MESSAGE) {
//
// FIB Setup has already been done, but we can minimize the
// damage by at least ensuring the OS never issues more
// commands than we can handle. The Rocket adapters currently
// can only handle 246 commands and 8 AIFs at the same time,
// and in fact do notify us accordingly if we negotiate the
// FIB size. The problem that causes us to add this check is
// to ensure that we do not overdo it with the adapter when a
// hard coded FIB override is being utilized. This special
// case warrants this half baked, but convenient, check here.
//
    if (dev.scsi_host_ptr.can_queue > AAC_NUM_IO_FIB_RKT) {
    dev.init.r7.max_io_commands =
    cpu_to_le32(AAC_NUM_IO_FIB_RKT + AAC_NUM_MGT_FIB);
    dev.scsi_host_ptr.can_queue = AAC_NUM_IO_FIB_RKT;
    }
    }
    return retval;
    }
//
// aac_rkt_ioremap
// @dev: device to ioremap
// @size: mapping resize request
//
#[no_mangle]
unsafe extern "C" fn aac_rkt_ioremap(dev: *mut *mut aac_dev, size: u32) -> c_int {
    static int aac_rkt_ioremap(struct aac_dev * dev, u32 size)
    {
    if (!size) {
    iounmap(dev.regs.rkt);
    return 0;
    }
    dev.base = dev.regs.rkt = ioremap(dev.base_start, size);
    if (dev.base == core::ptr::null_mut())
    return -1;
    dev.IndexRegs = &dev.regs.rkt.IndexRegs;
    return 0;
    }
//
// aac_rkt_init	-	initialize an i960 based AAC card
// @dev: device to configure
//
// Allocate and set up resources for the i960 based AAC variants. The
// device_interface in the commregion will be allocated and linked
// to the comm region.
//
#[no_mangle]
pub unsafe extern "C" fn aac_rkt_init(dev: *mut aac_dev) -> c_int {
    int aac_rkt_init(struct aac_dev *dev)
    {
//
// Fill in the function dispatch table.
//
    dev.a_ops.adapter_ioremap = aac_rkt_ioremap;
    dev.a_ops.adapter_comm = aac_rkt_select_comm;
    return _aac_rx_init(dev);
    }
