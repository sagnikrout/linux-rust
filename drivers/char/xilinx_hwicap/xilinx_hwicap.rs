//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/xilinx_hwicap/xilinx_hwicap.h
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
// Author: Xilinx, Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation; either version 2 of the License, or (at your
// option) any later version.
//
// XILINX IS PROVIDING THIS DESIGN, CODE, OR INFORMATION "AS IS"
// AS A COURTESY TO YOU, SOLELY FOR USE IN DEVELOPING PROGRAMS AND
// SOLUTIONS FOR XILINX DEVICES.  BY PROVIDING THIS DESIGN, CODE,
// OR INFORMATION AS ONE POSSIBLE IMPLEMENTATION OF THIS FEATURE,
// APPLICATION OR STANDARD, XILINX IS MAKING NO REPRESENTATION
// THAT THIS IMPLEMENTATION IS FREE FROM ANY CLAIMS OF INFRINGEMENT,
// AND YOU ARE RESPONSIBLE FOR OBTAINING ANY RIGHTS YOU MAY REQUIRE
// FOR YOUR IMPLEMENTATION.  XILINX EXPRESSLY DISCLAIMS ANY
// WARRANTY WHATSOEVER WITH RESPECT TO THE ADEQUACY OF THE
// IMPLEMENTATION, INCLUDING BUT NOT LIMITED TO ANY WARRANTIES OR
// REPRESENTATIONS THAT THIS IMPLEMENTATION IS FREE FROM CLAIMS OF
// INFRINGEMENT, IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
// FOR A PARTICULAR PURPOSE.
//
// (c) Copyright 2003-2007 Xilinx Inc.
// All rights reserved.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 675 Mass Ave, Cambridge, MA 02139, USA.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwicap_drvdata {
    pub /: *mut *mut u32 write_buffer_in_use; / Always in [0,3],
    pub write_buffer: [u8; 4],
    pub /: *mut *mut u32 read_buffer_in_use; / Always in [0,3],
    pub read_buffer: [u8; 4],
    pub /: *mut *mut resource_size_t mem_start;/ phys. address of the control registers,
    pub /: *mut *mut resource_size_t mem_end; / phys. address of the control registers,
    pub mem_size: resource_size_t,
    pub /: *mut *mut *mut void __iomem base_address;/ virt. address of the control registers,
    pub dev: *mut device,
    pub /: *mut *mut cdev cdev; / Char device structure,
    pub devt: dev_t,
    pub config: *const hwicap_driver_config,
    pub config_regs: *const config_registers,
    pub private_data: *mut c_void,
    pub is_open: bool,
    pub sem: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwicap_driver_config {
// Read configuration data given by size into the data buffer.
// Return 0 if successful.
//
    pub size): u32,
// Write configuration data given by size from the data buffer.
// Return 0 if successful.
//
    pub size): u32,
// Get the status register, bit pattern given by:
// D8 - 0 = configuration error
// D7 - 1 = alignment found
// D6 - 1 = readback in progress
// D5 - 0 = abort in progress
// D4 - Always 1
// D3 - Always 1
// D2 - Always 1
// D1 - Always 1
// D0 - 1 = operation completed
//
    pub drvdata): *mut *mut u32 (get_status)(struct hwicap_drvdata,
// Reset the hw
    pub drvdata): *mut *mut void (reset)(struct hwicap_drvdata,
}

// Number of times to poll the done register. This has to be large
// enough to allow an entire configuration to complete. If an entire
// page (4kb) is configured at once, that could take up to 4k cycles
// with a byte-wide icap interface. In most cases, this driver is
// used with a much smaller fifo, but this should be sufficient in the
// worst case.
//
pub const XHI_MAX_RETRIES: c_int = 5000;
// Constant Definitions
pub const XHI_PAD_FRAMES: c_uint = 0x1;
// Mask for calculating configuration packet headers
pub const XHI_WORD_COUNT_MASK_TYPE_1: c_uint = 0x7FFUL;
pub const XHI_WORD_COUNT_MASK_TYPE_2: c_uint = 0x1FFFFFUL;
pub const XHI_TYPE_MASK: c_uint = 0x7;
pub const XHI_REGISTER_MASK: c_uint = 0xF;
pub const XHI_OP_MASK: c_uint = 0x3;
pub const XHI_TYPE_SHIFT: c_int = 29;
pub const XHI_REGISTER_SHIFT: c_int = 13;
pub const XHI_OP_SHIFT: c_int = 27;
pub const XHI_TYPE_1: c_int = 1;
pub const XHI_TYPE_2: c_int = 2;
pub const XHI_OP_WRITE: c_int = 2;
pub const XHI_OP_READ: c_int = 1;
// Address Block Types
pub const XHI_FAR_CLB_BLOCK: c_int = 0;
pub const XHI_FAR_BRAM_BLOCK: c_int = 1;
pub const XHI_FAR_BRAM_INT_BLOCK: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct config_registers {
    pub CRC: u32,
    pub FAR: u32,
    pub FDRI: u32,
    pub FDRO: u32,
    pub CMD: u32,
    pub CTL: u32,
    pub MASK: u32,
    pub STAT: u32,
    pub LOUT: u32,
    pub COR: u32,
    pub MFWR: u32,
    pub FLR: u32,
    pub KEY: u32,
    pub CBC: u32,
    pub IDCODE: u32,
    pub AXSS: u32,
    pub C0R_1: u32,
    pub CSOB: u32,
    pub WBSTAR: u32,
    pub TIMER: u32,
    pub BOOTSTS: u32,
    pub CTL_1: u32,
}

// Configuration Commands
pub const XHI_CMD_NULL: c_int = 0;
pub const XHI_CMD_WCFG: c_int = 1;
pub const XHI_CMD_MFW: c_int = 2;
pub const XHI_CMD_DGHIGH: c_int = 3;
pub const XHI_CMD_RCFG: c_int = 4;
pub const XHI_CMD_START: c_int = 5;
pub const XHI_CMD_RCAP: c_int = 6;
pub const XHI_CMD_RCRC: c_int = 7;
pub const XHI_CMD_AGHIGH: c_int = 8;
pub const XHI_CMD_SWITCH: c_int = 9;
pub const XHI_CMD_GRESTORE: c_int = 10;
pub const XHI_CMD_SHUTDOWN: c_int = 11;
pub const XHI_CMD_GCAPTURE: c_int = 12;
pub const XHI_CMD_DESYNCH: c_int = 13;

// Packet constants
pub const XHI_SYNC_PACKET: c_uint = 0xAA995566UL;
pub const XHI_DUMMY_PACKET: c_uint = 0xFFFFFFFFUL;

pub const XHI_TYPE2_CNT_MASK: c_uint = 0x07FFFFFF;

pub const XHI_TYPE_1_HEADER_BYTES: c_int = 4;
pub const XHI_TYPE_2_HEADER_BYTES: c_int = 8;
// Constant to use for CRC check when CRC has been disabled
pub const XHI_DISABLED_AUTO_CRC: c_uint = 0x0000DEFCUL;
// Meanings of the bits returned by get_status
pub const XHI_SR_CFGERR_N_MASK: c_uint = 0x00000100 /* Config Error Mask */;
pub const XHI_SR_DALIGN_MASK: c_uint = 0x00000080 /* Data Alignment Mask */;
pub const XHI_SR_RIP_MASK: c_uint = 0x00000040 /* Read back Mask */;
pub const XHI_SR_IN_ABORT_N_MASK: c_uint = 0x00000020 /* Select Map Abort Mask */;
pub const XHI_SR_DONE_MASK: c_uint = 0x00000001 /* Done bit Mask  */;
//
// hwicap_type_1_read - Generates a Type 1 read packet header.
// @reg: is the address of the register to be read back.
//
// Return:
// Generates a Type 1 read packet header, which is used to indirectly
// read registers in the configuration logic.  This packet must then
// be sent through the icap device, and a return packet received with
// the information.
//
// hwicap_type_1_write - Generates a Type 1 write packet header
// @reg: is the address of the register to be read back.
//
// Return: Type 1 write packet header
//
