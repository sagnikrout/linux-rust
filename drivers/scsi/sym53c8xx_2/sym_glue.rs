//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/sym53c8xx_2/sym_glue.h
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
// Device driver for the SYMBIOS/LSILOGIC 53C8XX and 53C1010 family
// of PCI-SCSI IO processors.
//
// Copyright (C) 1999-2001  Gerard Roudier <groudier@free.fr>
//
// This driver is derived from the Linux sym53c8xx driver.
// Copyright (C) 1998-2000  Gerard Roudier
//
// The sym53c8xx driver is derived from the ncr53c8xx driver that had been
// a port of the FreeBSD ncr driver to Linux-1.2.13.
//
// The original ncr driver has been written for 386bsd and FreeBSD by
// Wolfgang Stanglmeier        <wolf@cologne.de>
// Stefan Esser                <se@mi.Uni-Koeln.de>
// Copyright (C) 1994  Wolfgang Stanglmeier
//
// Other major contributions:
//
// NVRAM detection and reading.
// Copyright (C) 1997 Richard Waltham <dormouse@farsrobt.demon.co.uk>
//
// -----------------------------------------------------------------------------
//

//
// Configuration addendum for Linux.
//

// Macro flag: #define SYM_OPT_LIMIT_COMMAND_REORDERING
//
// Print a message with severity.
//

//
// A 'read barrier' flushes any data that have been prefetched
// by the processor due to out of order execution. Such a barrier
// must notably be inserted prior to looking at data that have
// been DMAed, assuming that program does memory READs in proper
// order and that the device ensured proper ordering of WRITEs.
//
// A 'write barrier' prevents any previous WRITEs to pass further
// WRITEs. Such barriers must be inserted each time another agent
// relies on ordering of WRITEs.
//
// Note that, due to posting of PCI memory writes, we also must
// insert dummy PCI read transactions when some ordering involving
// both directions over the PCI does matter. PCI transactions are
// fully ordered in each direction.
//

//
// IO functions definition for big/little endian CPU support.
// For now, PCI chips are only supported in little endian addressing mode,
//

//
// If the CPU and the chip use same endian-ness addressing,
// no byte reordering is needed for script patching.
// Macro cpu_to_scr() is to be used for script patching.
// Macro scr_to_cpu() is to be used for getting a DWORD
// from the script.
//

//
// These ones are used as return code from
// error recovery handlers under Linux.
//

//
// System specific target data structure.
// None for now, under Linux.
//
// #define SYM_HAVE_STCB
//
// System specific lun data structure.
//
// Macro flag: #define SYM_HAVE_SLCB
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_slcb {
    pub /: *mut *mut u_short reqtags; / Number of tags requested by user,
    pub /: *mut *mut u_short scdev_depth; / Queue depth set in select_queue_depth(),
}

//
// System specific command data structure.
// Not needed under Linux.
//
// struct sym_sccb
//
// System specific host data structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_shcb {
//
// Chip and controller identification.
//
    pub unit: c_int,
    pub inst_name: [c_char; 16],
    pub chip_name: [c_char; 8],
    pub host: *mut Scsi_Host,
    pub /: *mut *mut *mut void __iomem  ioaddr; / MMIO kernel io address,
    pub /: *mut *mut *mut void __iomem  ramaddr; / RAM kernel io address,
    pub /: *mut *mut timer_list timer; / Timer handler link header,
    pub lasttime: u_long,
    pub /: *mut *mut u_long settle_time; / Resetting the SCSI BUS,
    pub settle_time_valid: u_char,
}

//
// Return the name of the controller.
//

//
// The IO macros require a struct called 's' and are abused in sym_nvram.c
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_device {
    pub pdev: *mut pci_dev,
    pub mmio_base: c_ulong,
    pub ram_base: c_ulong,
    pub ioaddr: *mut void __iomem,
    pub ramaddr: *mut void __iomem,
    pub s: },
    pub chip: sym_chip,
    pub nvram: *mut sym_nvram,
    pub host_id: u_char,
}

//
// Driver host data structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_data {
    pub ncb: *mut sym_hcb,
    pub /: *mut *mut *mut completion io_reset; / PCI error handling,
    pub pdev: *mut pci_dev,
}

//
// Set the status field of a CAM CCB.
//
// Get the status field of a CAM CCB.
//
extern "C" {
    pub fn host_byte(_arg: cmd->result) -> return;
}
//
// Build CAM result for a successful IO and for a failed IO.
//
extern "C" {
    pub fn sym_set_cam_result_error(np: *mut sym_hcb, cp: *mut sym_ccb, resid: c_int);
}
extern "C" {
    pub fn sym_xpt_done(np: *mut sym_hcb, ccb: *mut scsi_cmnd);
}

extern "C" {
    pub fn sym_xpt_async_bus_reset(np: *mut sym_hcb);
}
extern "C" {
    pub fn sym_setup_data_and_start(np: *mut sym_hcb, csio: *mut scsi_cmnd, cp: *mut sym_ccb) -> c_int;
}
extern "C" {
    pub fn sym_log_bus_error(: *mut Scsi_Host);
}
extern "C" {
    pub fn sym_dump_registers(: *mut Scsi_Host);
}
