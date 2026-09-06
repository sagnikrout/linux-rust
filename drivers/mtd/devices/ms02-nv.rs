//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/devices/ms02-nv.h
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
// Copyright (c) 2001, 2003  Maciej W. Rozycki
//
// DEC MS02-NV (54-20948-01) battery backed-up NVRAM module for
// DECstation/DECsystem 5000/2x0 and DECsystem 5900 and 5900/260
// systems.
//

//
// Addresses are decoded as follows:
//
// 0x000000 - 0x3fffff	SRAM
// 0x400000 - 0x7fffff	CSR
//
// Within the SRAM area the following ranges are forced by the system
// firmware:
//
// 0x000000 - 0x0003ff	diagnostic area, destroyed upon a reboot
// 0x000400 - ENDofRAM	storage area, available to operating systems
//
// but we can't really use the available area right from 0x000400 as
// the first word is used by the firmware as a status flag passed
// from an operating system.  If anything but the valid data magic
// ID value is found, the firmware considers the SRAM clean, i.e.
// containing no valid data, and disables the battery resulting in
// data being erased as soon as power is switched off.  So the choice
// for the start address of the user-available is 0x001000 which is
// nicely page aligned.  The area between 0x000404 and 0x000fff may
// be used by the driver for own needs.
//
// The diagnostic area defines two status words to be read by an
// operating system, a magic ID to distinguish a MS02-NV board from
// anything else and a status information providing results of tests
// as well as the size of SRAM available, which can be 1MiB or 2MiB
// (that's what the firmware handles; no idea if 2MiB modules ever
// existed).
//
// The firmware only handles the MS02-NV board if installed in the
// last (15th) slot, so for any other location the status information
// stored in the SRAM cannot be relied upon.  But from the hardware
// point of view there is no problem using up to 14 such boards in a
// system -- only the 1st slot needs to be filled with a DRAM module.
// The MS02-NV board is ECC-protected, like other MS02 memory boards.
//
// The state of the battery as provided by the CSR is reflected on
// the two onboard LEDs.  When facing the battery side of the board,
// with the LEDs at the top left and the battery at the bottom right
// (i.e. looking from the back side of the system box), their meaning
// is as follows (the system has to be powered on):
//
// left LED		battery disable status: lit = enabled
// right LED		battery condition status: lit = OK
//
// MS02-NV iomem register offsets.
pub const MS02NV_CSR: c_uint = 0x400000	/* control & status register */;
// MS02-NV CSR status bits.
pub const MS02NV_CSR_BATT_OK: c_uint = 0x01		/* battery OK */;
pub const MS02NV_CSR_BATT_OFF: c_uint = 0x02		/* battery disabled */;
// MS02-NV memory offsets.
pub const MS02NV_DIAG: c_uint = 0x0003f8	/* diagnostic status */;
pub const MS02NV_MAGIC: c_uint = 0x0003fc	/* MS02-NV magic ID */;
pub const MS02NV_VALID: c_uint = 0x000400	/* valid data magic ID */;
pub const MS02NV_RAM: c_uint = 0x001000	/* user-exposed RAM start */;
// MS02-NV diagnostic status bits.
pub const MS02NV_DIAG_TEST: c_uint = 0x01		/* SRAM test done (?) */;
pub const MS02NV_DIAG_RO: c_uint = 0x02		/* SRAM r/o test done */;
pub const MS02NV_DIAG_RW: c_uint = 0x04		/* SRAM r/w test done */;
pub const MS02NV_DIAG_FAIL: c_uint = 0x08		/* SRAM test failed */;
pub const MS02NV_DIAG_SIZE_MASK: c_uint = 0xf0		/* SRAM size mask */;
pub const MS02NV_DIAG_SIZE_SHIFT: c_uint = 0x10		/* SRAM size shift (left) */;
// MS02-NV general constants.
pub const MS02NV_ID: c_uint = 0x03021966	/* MS02-NV magic ID value */;
pub const MS02NV_VALID_ID: c_uint = 0xbd100248	/* valid data magic ID value */;
pub const MS02NV_SLOT_SIZE: c_uint = 0x800000	/* size of the address space;
pub type ms02nv_uint = volatile u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms02nv_private {
    pub next: *mut mtd_info,
    pub module: *mut resource,
    pub diag_ram: *mut resource,
    pub user_ram: *mut resource,
    pub csr: *mut resource,
    pub resource: },
    pub addr: *mut u_char,
    pub size: usize,
    pub uaddr: *mut u_char,
}
