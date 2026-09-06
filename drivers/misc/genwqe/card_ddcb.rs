//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/genwqe/card_ddcb.h
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
// IBM Accelerator Family 'GenWQE'
//
// (C) Copyright IBM Corp. 2013
//
// Author: Frank Haverkamp <haver@linux.vnet.ibm.com>
// Author: Joerg-Stephan Vogt <jsvogt@de.ibm.com>
// Author: Michael Jung <mijung@gmx.net>
// Author: Michael Ruettger <michael@ibmra.de>
//

//
// struct ddcb - Device Driver Control Block DDCB
// @hsi:        Hardware software interlock
// @shi:        Software hardware interlock. Hsi and shi are used to interlock
// software and hardware activities. We are using a compare and
// swap operation to ensure that there are no races when
// activating new DDCBs on the queue, or when we need to
// purge a DDCB from a running queue.
// @acfunc:     Accelerator function addresses a unit within the chip
// @cmd:        Command to work on
// @cmdopts_16: Options for the command
// @asiv:       Input data
// @asv:        Output data
//
// The DDCB data format is big endian. Multiple consequtive DDBCs form
// a DDCB queue.
//

pub const ASV_LENGTH: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddcb {
    pub /: *mut *mut __be32 icrc_hsi_shi_32; / iCRC, Hardware/SW interlock,
    pub icrc_16: __be16,
    pub hsi: u8,
    pub shi: u8,
}

// The following layout matches the new service layer format
// CRC polynomials for DDCB
pub const CRC16_POLYNOMIAL: c_uint = 0x1021;
//
// SHI: Software to Hardware Interlock
// This 1 byte field is written by software to interlock the
// movement of one queue entry to another with the hardware in the
// chip.
//
pub const DDCB_SHI_INTR: c_uint = 0x04 /* Bit 2 */;
pub const DDCB_SHI_PURGE: c_uint = 0x02 /* Bit 1 */;
pub const DDCB_SHI_NEXT: c_uint = 0x01 /* Bit 0 */;
//
// HSI: Hardware to Software interlock
// This 1 byte field is written by hardware to interlock the movement
// of one queue entry to another with the software in the chip.
//
pub const DDCB_HSI_COMPLETED: c_uint = 0x40 /* Bit 6 */;
pub const DDCB_HSI_FETCHED: c_uint = 0x04 /* Bit 2 */;
//
// Accessing HSI/SHI is done 32-bit wide
// Normally 16-bit access would work too, but on some platforms the
// 16 compare and swap operation is not supported. Therefore
// switching to 32-bit such that those platforms will work too.
//
// iCRC HSI/SHI
//

// Definitions of DDCB presets
pub const DDCB_PRESET_PRE: c_uint = 0x80;

//
// Genwqe Scatter Gather list
// Each element has up to 8 entries.
// The chaining element is element 0 cause of prefetching needs.
//
// 0b0110 Chained descriptor. The descriptor is describing the next
// descriptor list.
//

//
// 0b0010 First entry of a descriptor list. Start from a Buffer-Empty
// condition.
//

//
// 0b0000 Early terminator. This is the last entry on the list
// irregardless of the length indicated.
//

//
// struct sglist - Scatter gather list
// @target_addr:       Either a dma addr of memory to work on or a
// dma addr or a subsequent sglist block.
// @len:               Length of the data block.
// @flags:             See above.
//
// Depending on the command the GenWQE card can use a scatter gather
// list to describe the memory it works on. Always 8 sg_entry's form
// a block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_entry {
    pub target_addr: __be64,
    pub len: __be32,
    pub flags: __be32,
}
