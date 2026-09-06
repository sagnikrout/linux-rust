//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/sym53c8xx_2/sym53c8xx.h
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
// DMA addressing mode.
//
// 0 : 32 bit addressing for all chips.
// 1 : 40 bit addressing when supported by chip.
// 2 : 64 bit addressing when supported by chip,
// limited to 16 segments of 4 GB -> 64 GB max.
//

//
// NVRAM support.
//

//
// These options are not tunable from 'make config'
//

// Macro flag: #define	SYM_LINUX_PROC_INFO_SUPPORT
// Macro flag: #define SYM_LINUX_USER_COMMAND_SUPPORT
// Macro flag: #define SYM_LINUX_USER_INFO_SUPPORT
// Macro flag: #define SYM_LINUX_DEBUG_CONTROL_SUPPORT

//
// Also handle old NCR chips if not (0).
//

//
// Allow tags from 2 to 256, default 8
//

//
// Anyway, we configure the driver for at least 64 tags per LUN. :)
//

//
// Max number of SG entries.
//

//
// Driver setup structure.
//
// This structure is initialized from linux config options.
// It can be overridden at boot-up by the boot command line.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_driver_setup {
    pub max_tag: u_short,
    pub burst_order: u_char,
    pub scsi_led: u_char,
    pub scsi_diff: u_char,
    pub irq_mode: u_char,
    pub scsi_bus_check: u_char,
    pub host_id: u_char,
    pub verbose: u_char,
    pub settle_delay: u_char,
    pub use_nvram: u_char,
    pub excludes: [u_long; 8],
}

//
// Initial setup.
//
// Can be overriden at startup by a command line.
//

//
// Max number of targets.
// Maximum is 16 and you are advised not to change this value.
//

//
// Max number of logical units.
// SPI-2 allows up to 64 logical units, but in real life, target
// that implements more that 7 logical units are pretty rare.
// Anyway, the cost of accepting up to 64 logical unit is low in
// this driver, thus going with the maximum is acceptable.
//

//
// Max number of IO control blocks queued to the controller.
// Each entry needs 8 bytes and the queues are allocated contiguously.
// Since we donnot want to allocate more than a page, the theorical
// maximum is PAGE_SIZE/8. For safety, we announce a bit less to the
// access method. :)
// When not supplied, as it is suggested, the driver compute some
// good value for this parameter.
//
// #define SYM_CONF_MAX_START	(PAGE_SIZE/8 - 16)
//
// Support for Immediate Arbitration.
// Not advised.
//
// #define SYM_CONF_IARB_SUPPORT
//
// Only relevant if IARB support configured.
// - Max number of successive settings of IARB hints.
// - Set IARB on arbitration lost.
//
pub const SYM_CONF_IARB_MAX: c_int = 3;
pub const SYM_CONF_SET_IARB_ON_ARB_LOST: c_int = 1;
//
// Returning wrong residuals may make problems.
// When zero, this define tells the driver to
// always return 0 as transfer residual.
// Btw, all my testings of residuals have succeeded.
//
pub const SYM_SETUP_RESIDUAL_SUPPORT: c_int = 1;
